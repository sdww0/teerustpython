use crate::sys::Events;
use crate::{Interests, Token};

use log::error;
use std::mem::MaybeUninit;
use std::os::unix::io::{AsRawFd, RawFd};
#[cfg(debug_assertions)]
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
use std::{cmp, io, ptr, slice};

/// Unique id for use as `SelectorId`.
#[cfg(debug_assertions)]
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

// Type of the `nchanges` and `nevents` parameters in the `kevent` function.
#[cfg(not(target_os = "netbsd"))]
type Count = libc::c_int;
#[cfg(target_os = "netbsd")]
type Count = libc::size_t;

// Type of the `filter` field in the `kevent` structure.
#[cfg(any(target_os = "freebsd", target_os = "openbsd"))]
type Filter = libc::c_short;
#[cfg(any(target_os = "macos", target_os = "ios"))]
type Filter = i16;
#[cfg(target_os = "netbsd")]
type Filter = u32;

// Type of the `data` field in the `kevent` structure.
#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos"
))]
type Data = libc::intptr_t;
#[cfg(any(target_os = "bitrig", target_os = "netbsd", target_os = "openbsd",))]
type Data = i64;

// Type of the `udata` field in the `kevent` structure.
#[cfg(not(target_os = "netbsd"))]
type UData = *mut libc::c_void;
#[cfg(target_os = "netbsd")]
type UData = libc::intptr_t;

macro_rules! kevent {
    ($id: expr, $filter: expr, $flags: expr, $data: expr) => {
        libc::kevent {
            ident: $id as libc::uintptr_t,
            filter: $filter as Filter,
            flags: $flags,
            fflags: 0,
            data: 0,
            udata: $data as UData,
        }
    };
}

#[derive(Debug)]
pub struct Selector {
    #[cfg(debug_assertions)]
    id: usize,
    kq: RawFd,
}

impl Selector {
    pub fn new() -> io::Result<Selector> {
        syscall!(kqueue())
            .and_then(|kq| syscall!(fcntl(kq, libc::F_SETFD, libc::FD_CLOEXEC)).map(|_| kq))
            .map(|kq| Selector {
                #[cfg(debug_assertions)]
                id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
                kq,
            })
    }

    #[cfg(debug_assertions)]
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn try_clone(&self) -> io::Result<Selector> {
        syscall!(dup(self.kq)).map(|kq| Selector {
            // It's the same selector, so we use the same id.
            #[cfg(debug_assertions)]
            id: self.id,
            kq,
        })
    }

    pub fn select(&self, events: &mut Events, timeout: Option<Duration>) -> io::Result<()> {
        let timeout = timeout.map(|to| libc::timespec {
            tv_sec: cmp::min(to.as_secs(), libc::time_t::max_value() as u64) as libc::time_t,
            // `Duration::subsec_nanos` is guaranteed to be less than one
            // billion (the number of nanoseconds in a second), making the
            // cast to i32 safe. The cast itself is needed for platforms
            // where C's long is only 32 bits.
            tv_nsec: libc::c_long::from(to.subsec_nanos() as i32),
        });
        let timeout = timeout
            .as_ref()
            .map(|s| s as *const _)
            .unwrap_or(ptr::null_mut());

        events.clear();
        syscall!(kevent(
            self.kq,
            ptr::null(),
            0,
            events.as_mut_ptr(),
            events.capacity() as Count,
            timeout,
        ))
        .map(|n_events| {
            // This is safe because `kevent` ensures that `n_events` are
            // assigned.
            unsafe { events.set_len(n_events as usize) };
        })
    }

    pub fn register(&self, fd: RawFd, token: Token, interests: Interests) -> io::Result<()> {
        let flags = libc::EV_CLEAR | libc::EV_RECEIPT | libc::EV_ADD;
        // At most we need two changes, but maybe we only need 1.
        let mut changes: [MaybeUninit<libc::kevent>; 2] =
            [MaybeUninit::uninit(), MaybeUninit::uninit()];
        let mut n_changes = 0;

        if interests.is_writable() {
            let kevent = kevent!(fd, libc::EVFILT_WRITE, flags, token.0);
            changes[n_changes] = MaybeUninit::new(kevent);
            n_changes += 1;
        }

        if interests.is_readable() {
            let kevent = kevent!(fd, libc::EVFILT_READ, flags, token.0);
            changes[n_changes] = MaybeUninit::new(kevent);
            n_changes += 1;
        }

        // Older versions of macOS (OS X 10.11 and 10.10 have been witnessed)
        // can return EPIPE when registering a pipe file descriptor where the
        // other end has already disappeared. For example code that creates a
        // pipe, closes a file descriptor, and then registers the other end will
        // see an EPIPE returned from `register`.
        //
        // It also turns out that kevent will still report events on the file
        // descriptor, telling us that it's readable/hup at least after we've
        // done this registration. As a result we just ignore `EPIPE` here
        // instead of propagating it.
        //
        // More info can be found at tokio-rs/mio#582.
        let changes = unsafe {
            // This is safe because we ensure that at least `n_changes` are in
            // the array.
            slice::from_raw_parts_mut(changes[0].as_mut_ptr(), n_changes)
        };
        kevent_register(self.kq, changes, &[libc::EPIPE as Data])
    }

    pub fn reregister(&self, fd: RawFd, token: Token, interests: Interests) -> io::Result<()> {
        let flags = libc::EV_CLEAR | libc::EV_RECEIPT;
        let write_flags = if interests.is_writable() {
            flags | libc::EV_ADD
        } else {
            flags | libc::EV_DELETE
        };
        let read_flags = if interests.is_readable() {
            flags | libc::EV_ADD
        } else {
            flags | libc::EV_DELETE
        };

        let mut changes: [libc::kevent; 2] = [
            kevent!(fd, libc::EVFILT_WRITE, write_flags, token.0),
            kevent!(fd, libc::EVFILT_READ, read_flags, token.0),
        ];

        // Since there is no way to check with which interests the fd was
        // registered we modify both readable and write, adding it when required
        // and removing it otherwise, ignoring the ENOENT error when it comes
        // up. The ENOENT error informs us that a filter we're trying to remove
        // wasn't there in first place, but we don't really care since our goal
        // is accomplished.
        //
        // For the explanation of ignoring `EPIPE` see `register`.
        kevent_register(
            self.kq,
            &mut changes,
            &[libc::ENOENT as Data, libc::EPIPE as Data],
        )
    }

    pub fn deregister(&self, fd: RawFd) -> io::Result<()> {
        let flags = libc::EV_DELETE | libc::EV_RECEIPT;
        let mut changes: [libc::kevent; 2] = [
            kevent!(fd, libc::EVFILT_WRITE, flags, 0),
            kevent!(fd, libc::EVFILT_READ, flags, 0),
        ];

        // Since there is no way to check with which interests the fd was
        // registered we remove both filters (readable and writeable) and ignore
        // the ENOENT error when it comes up. The ENOENT error informs us that
        // the filter wasn't there in first place, but we don't really care
        // about that since our goal is to remove it.
        kevent_register(self.kq, &mut changes, &[libc::ENOENT as Data])
    }

    // Used by `Waker`.
    #[cfg(any(target_os = "freebsd", target_os = "ios", target_os = "macos"))]
    pub fn setup_waker(&self, token: Token) -> io::Result<()> {
        // First attempt to accept user space notifications.
        let mut kevent = kevent!(
            0,
            libc::EVFILT_USER,
            libc::EV_ADD | libc::EV_CLEAR | libc::EV_RECEIPT,
            token.0
        );

        syscall!(kevent(self.kq, &kevent, 1, &mut kevent, 1, ptr::null())).and_then(|_| {
            if (kevent.flags & libc::EV_ERROR) != 0 && kevent.data != 0 {
                Err(io::Error::from_raw_os_error(kevent.data as i32))
            } else {
                Ok(())
            }
        })
    }

    // Used by `Waker`.
    #[cfg(any(target_os = "freebsd", target_os = "ios", target_os = "macos"))]
    pub fn wake(&self, token: Token) -> io::Result<()> {
        let mut kevent = kevent!(
            0,
            libc::EVFILT_USER,
            libc::EV_ADD | libc::EV_RECEIPT,
            token.0
        );
        kevent.fflags = libc::NOTE_TRIGGER;

        syscall!(kevent(self.kq, &kevent, 1, &mut kevent, 1, ptr::null())).and_then(|_| {
            if (kevent.flags & libc::EV_ERROR) != 0 && kevent.data != 0 {
                Err(io::Error::from_raw_os_error(kevent.data as i32))
            } else {
                Ok(())
            }
        })
    }
}

/// Register `changes` with `kq`ueue.
fn kevent_register(
    kq: RawFd,
    changes: &mut [libc::kevent],
    ignored_errors: &[Data],
) -> io::Result<()> {
    syscall!(kevent(
        kq,
        changes.as_ptr(),
        changes.len() as Count,
        changes.as_mut_ptr(),
        changes.len() as Count,
        ptr::null(),
    ))
    .map(|_| ())
    .or_else(|err| {
        // According to the manual page of FreeBSD: "When kevent() call fails
        // with EINTR error, all changes in the changelist have been applied",
        // so we can safely ignore it.
        if err.raw_os_error() == Some(libc::EINTR) {
            Ok(())
        } else {
            Err(err)
        }
    })
    .and_then(|()| check_errors(&changes, ignored_errors))
}

/// Check all events for possible errors, it returns the first error found.
fn check_errors(events: &[libc::kevent], ignored_errors: &[Data]) -> io::Result<()> {
    for event in events {
        // We can't use references to packed structures (in checking the ignored
        // errors), so we need copy the data out before use.
        let data = event.data;
        // Check for the error flag, the actual error will be in the `data`
        // field.
        if (event.flags & libc::EV_ERROR != 0) && data != 0 && !ignored_errors.contains(&data) {
            return Err(io::Error::from_raw_os_error(data as i32));
        }
    }
    Ok(())
}

impl AsRawFd for Selector {
    fn as_raw_fd(&self) -> RawFd {
        self.kq
    }
}

impl Drop for Selector {
    fn drop(&mut self) {
        if let Err(err) = syscall!(close(self.kq)) {
            error!("error closing kqueue: {}", err);
        }
    }
}

pub type Event = libc::kevent;

pub mod event {
    use crate::sys::Event;
    use crate::Token;

    pub fn token(event: &Event) -> Token {
        Token(event.udata as usize)
    }

    pub fn is_readable(event: &Event) -> bool {
        event.filter == libc::EVFILT_READ || {
            #[cfg(any(target_os = "freebsd", target_os = "ios", target_os = "macos"))]
            // Used by the `Awakener`. On platforms that use `eventfd` or a unix
            // pipe it will emit a readable event so we'll fake that here as
            // well.
            {
                event.filter == libc::EVFILT_USER
            }
            #[cfg(not(any(target_os = "freebsd", target_os = "ios", target_os = "macos")))]
            {
                false
            }
        }
    }

    pub fn is_writable(event: &Event) -> bool {
        event.filter == libc::EVFILT_WRITE
    }

    pub fn is_error(event: &Event) -> bool {
        (event.flags & libc::EV_ERROR) != 0 ||
            // When the read end of the socket is closed, EV_EOF is set on
            // flags, and fflags contains the error if there is one.
            (event.flags & libc::EV_EOF) != 0 && event.fflags != 0
    }

    pub fn is_hup(_: &Event) -> bool {
        // Not supported.
        false
    }

    pub fn is_read_hup(event: &Event) -> bool {
        event.filter == libc::EVFILT_READ && (event.flags & libc::EV_EOF) != 0
    }

    pub fn is_priority(_: &Event) -> bool {
        // kqueue doesn't have priority indicators.
        false
    }

    #[allow(unused_variables)] // `event` is not used on some platforms.
    pub fn is_aio(event: &Event) -> bool {
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos"
        ))]
        {
            event.filter == libc::EVFILT_AIO
        }
        #[cfg(not(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos"
        )))]
        {
            false
        }
    }

    #[allow(unused_variables)] // `event` is only used on FreeBSD.
    pub fn is_lio(event: &Event) -> bool {
        #[cfg(target_os = "freebsd")]
        {
            event.filter == libc::EVFILT_LIO
        }
        #[cfg(not(target_os = "freebsd"))]
        {
            false
        }
    }
}

#[test]
fn does_not_register_rw() {
    use crate::unix::SourceFd;
    use crate::{Poll, Token};

    let kq = unsafe { libc::kqueue() };
    let kqf = SourceFd(&kq);
    let poll = Poll::new().unwrap();

    // Registering kqueue fd will fail if write is requested (On anything but
    // some versions of macOS).
    poll.registry()
        .register(&kqf, Token(1234), Interests::READABLE)
        .unwrap();
}
