use parity_wasm::io::*;

pub fn cursor() {
	let mut cursor = Cursor::new(vec![0xFFu8, 0x7Fu8]);
	assert_eq!(cursor.position(), 0);

	let mut buf = [0u8];
	assert!(cursor.read(&mut buf[..]).is_ok());
	assert_eq!(cursor.position(), 1);
	assert_eq!(buf[0], 0xFFu8);
	assert!(cursor.read(&mut buf[..]).is_ok());
	assert_eq!(buf[0], 0x7Fu8);
	assert_eq!(cursor.position(), 2);
}

pub fn overflow_in_cursor() {
	let mut cursor = Cursor::new(vec![0u8]);
	let mut buf = [0, 1, 2];
	assert!(cursor.read(&mut buf[..]).is_err());
}
