# 


**如果已经移植，则该枝会被删除**


rustpython v0.1.2 (/home/waoa/RustPython)
├── cfg-if v1.0.0
├── clap v2.33.3
│   ├── ansi_term v0.11.0
│   ├── atty v0.2.14
│   │   └── libc v0.2.107
│   ├── bitflags v1.3.2
│   ├── strsim v0.8.0
│   ├── textwrap v0.11.0
│   │   └── unicode-width v0.1.9
│   ├── unicode-width v0.1.9
│   └── vec_map v0.8.2
├── dirs-next v2.0.0
│   ├── cfg-if v1.0.0
│   └── dirs-sys-next v0.1.2
│       └── libc v0.2.107
├── env_logger v0.9.0
│   ├── atty v0.2.14 (*)
│   ├── log v0.4.14
│   │   └── cfg-if v1.0.0
│   └── termcolor v1.1.2
├── libc v0.2.107
├── log v0.4.14 (*)
├── num-traits v0.2.14
│   [build-dependencies]
│   └── autocfg v1.0.1
├── rustpython-compiler v0.1.2 (/home/waoa/RustPython/compiler/porcelain)
│   ├── rustpython-bytecode v0.1.2 (/home/waoa/RustPython/bytecode)
│   │   ├── bincode v1.3.3
│   │   ├── bitflags v1.3.2
│   │   ├── bstr v0.2.17
│   │   │   ├── lazy_static v1.4.0
│   │   │   ├── memchr v2.4.1
│   │   │   ├── regex-automata v0.1.10
│   │   │   └── serde v1.0.130 (*)
│   │   ├── itertools v0.10.1
│   │   │   └── either v1.6.1
│   │   ├── lz4_flex v0.9.0
│   │   │   └── twox-hash v1.6.1
│   │   │       ├── cfg-if v1.0.0
│   │   │       └── static_assertions v1.1.0
│   │   ├── num-bigint v0.4.3
│   │   ├── num-complex v0.4.0
│   │   │   ├── num-traits v0.2.14 (*)
│   │   │   └── serde v1.0.130 (*)
│   │   └── serde v1.0.130 (*)
│   ├── rustpython-compiler-core v0.1.2 (/home/waoa/RustPython/compiler)
│   │   ├── ahash v0.7.6
│   │   │   ├── getrandom v0.2.3
│   │   │   └── once_cell v1.8.0
│   │   │   [build-dependencies]
│   │   │   └── version_check v0.9.3
│   │   ├── indexmap v1.7.0
│   │   │   └── hashbrown v0.11.2
│   │   │   [build-dependencies]
│   │   │   └── autocfg v1.0.1
│   │   ├── itertools v0.10.1 (*)
│   │   ├── log v0.4.14 (*)
│   │   ├── num-complex v0.4.0 (*)
│   │   ├── num-traits v0.2.14 (*)
│   │   ├── rustpython-ast v0.1.0 (/home/waoa/RustPython/ast)
│   │   │   ├── num-bigint v0.4.3 (*)
│   │   │   └── rustpython-common v0.0.0 (/home/waoa/RustPython/common)
│   │   │       ├── ascii v1.0.0
│   │   │       ├── cfg-if v1.0.0
│   │   │       ├── hexf-parse v0.2.1
│   │   │       ├── lexical-parse-float v0.8.2
│   │   │       │   ├── lexical-parse-integer v0.8.0
│   │   │       │   │   ├── lexical-util v0.8.1
│   │   │       │   │   │   └── static_assertions v1.1.0
│   │   │       │   │   └── static_assertions v1.1.0
│   │   │       │   ├── lexical-util v0.8.1 (*)
│   │   │       │   └── static_assertions v1.1.0
│   │   │       ├── libc v0.2.107
│   │   │       ├── lock_api v0.4.5
│   │   │       │   └── scopeguard v1.1.0
│   │   │       ├── num-bigint v0.4.3 (*)
│   │   │       ├── num-complex v0.4.0 (*)
│   │   │       ├── num-traits v0.2.14 (*)
│   │   │       ├── once_cell v1.8.0
│   │   │       ├── parking_lot v0.11.2
│   │   │       │   ├── instant v0.1.12
│   │   │       │   │   └── cfg-if v1.0.0
│   │   │       │   ├── lock_api v0.4.5 (*)
│   │   │       │   └── parking_lot_core v0.8.5
│   │   │       │       ├── cfg-if v1.0.0
│   │   │       │       ├── instant v0.1.12 (*)
│   │   │       │       ├── libc v0.2.107
│   │   │       │       └── smallvec v1.7.0
│   │   │       ├── radium v0.6.2
│   │   │       ├── rand v0.8.4
│   │   │       ├── siphasher v0.3.7
│   │   │       ├── unic-ucd-category v0.9.0
│   │   │       │   ├── matches v0.1.9
│   │   │       │   ├── unic-char-property v0.9.0
│   │   │       │   │   └── unic-char-range v0.9.0
│   │   │       │   ├── unic-char-range v0.9.0
│   │   │       │   └── unic-ucd-version v0.9.0
│   │   │       │       └── unic-common v0.9.0
│   │   │       └── volatile v0.3.0
│   │   └── rustpython-bytecode v0.1.2 (/home/waoa/RustPython/bytecode) (*)
│   ├── rustpython-parser v0.1.2 (/home/waoa/RustPython/parser)
│   │   ├── ahash v0.7.6 (*)
│   │   ├── lalrpop-util v0.19.6
│   │   ├── log v0.4.14 (*)
│   │   ├── num-bigint v0.4.3 (*)
│   │   ├── num-traits v0.2.14 (*)
│   │   ├── phf v0.10.0
│   │   │   ├── phf_macros v0.10.0 (proc-macro)
│   │   │   │   ├── phf_generator v0.10.0
│   │   │   │   │   ├── phf_shared v0.10.0
│   │   │   │   │   │   └── siphasher v0.3.7
│   │   │   │   │   └── rand v0.8.4
│   │   │   │   ├── phf_shared v0.10.0 (*)
│   │   │   │   ├── proc-macro-hack v0.5.19 (proc-macro)
│   │   │   │   ├── proc-macro2 v1.0.32 (*)
│   │   │   │   ├── quote v1.0.10 (*)
│   │   │   │   └── syn v1.0.81 (*)
│   │   │   ├── phf_shared v0.10.0 (*)
│   │   │   └── proc-macro-hack v0.5.19 (proc-macro)
│   │   ├── rustpython-ast v0.1.0 (/home/waoa/RustPython/ast) (*)
│   │   ├── unic-emoji-char v0.9.0
│   │   │   ├── unic-char-property v0.9.0 (*)
│   │   │   ├── unic-char-range v0.9.0
│   │   │   └── unic-ucd-version v0.9.0 (*)
│   │   ├── unic-ucd-ident v0.9.0
│   │   │   ├── unic-char-property v0.9.0 (*)
│   │   │   ├── unic-char-range v0.9.0
│   │   │   └── unic-ucd-version v0.9.0 (*)
│   │   └── unicode_names2 v0.4.0
│   │   [build-dependencies]
│   │   └── lalrpop v0.19.6
│   │       ├── ascii-canvas v3.0.0
│   │       │   └── term v0.7.0
│   │       │       └── dirs-next v2.0.0 (*)
│   │       ├── atty v0.2.14 (*)
│   │       ├── bit-set v0.5.2
│   │       │   └── bit-vec v0.6.3
│   │       ├── diff v0.1.12
│   │       ├── ena v0.14.0
│   │       │   └── log v0.4.14
│   │       ├── itertools v0.10.1 (*)
│   │       ├── lalrpop-util v0.19.6
│   │       │   └── regex v1.5.4
│   │       ├── petgraph v0.5.1
│   │       │   ├── fixedbitset v0.2.0
│   │       │   └── indexmap v1.7.0 (*)
│   │       ├── pico-args v0.4.2
│   │       ├── regex v1.5.4 (*)
│   │       ├── regex-syntax v0.6.25
│   │       ├── string_cache v0.8.2
│   │       │   ├── lazy_static v1.4.0
│   │       │   ├── new_debug_unreachable v1.0.4
│   │       │   ├── parking_lot v0.11.2 (*)
│   │       │   ├── phf_shared v0.8.0
│   │       │   │   └── siphasher v0.3.7
│   │       │   └── precomputed-hash v0.1.1
│   │       ├── term v0.7.0 (*)
│   │       ├── tiny-keccak v2.0.2
│   │       │   └── crunchy v0.2.2
│   │       └── unicode-xid v0.2.2
│   └── thiserror v1.0.30
├── rustpython-parser v0.1.2 (/home/waoa/RustPython/parser) (*)
├── rustpython-stdlib v0.1.2 (/home/waoa/RustPython/stdlib)
│   ├── adler32 v1.2.0
│   ├── ahash v0.7.6 (*)
│   ├── ascii v1.0.0
│   ├── base64 v0.13.0
│   ├── blake2 v0.9.2
│   │   ├── crypto-mac v0.8.0
│   │   │   ├── generic-array v0.14.4
│   │   │   │   └── typenum v1.14.0
│   │   │   │   [build-dependencies]
│   │   │   │   └── version_check v0.9.3
│   │   │   └── subtle v2.4.1
│   │   ├── digest v0.9.0
│   │   │   └── generic-array v0.14.4 (*)
│   │   └── opaque-debug v0.3.0
│   ├── cfg-if v1.0.0
│   ├── crc32fast v1.2.1
│   │   └── cfg-if v1.0.0
│   ├── crossbeam-utils v0.8.5
│   │   ├── cfg-if v1.0.0
│   │   └── lazy_static v1.4.0
│   ├── csv-core v0.1.10
│   │   └── memchr v2.4.1
│   ├── digest v0.9.0 (*)
│   ├── dns-lookup v1.0.8
│   │   ├── cfg-if v1.0.0
│   │   ├── libc v0.2.107
│   │   └── socket2 v0.4.2
│   │       └── libc v0.2.107
│   ├── flate2 v1.0.22
│   │   ├── cfg-if v1.0.0
│   │   ├── crc32fast v1.2.1 (*)
│   │   ├── libc v0.2.107
│   │   ├── libz-sys v1.1.3
│   │   │   └── libc v0.2.107
│   │   │   [build-dependencies]
│   │   │   ├── cc v1.0.71
│   │   │   └── pkg-config v0.3.22
│   │   └── miniz_oxide v0.4.4
│   │       └── adler v1.0.2
│   │       [build-dependencies]
│   │       └── autocfg v1.0.1
│   ├── gethostname v0.2.1
│   │   └── libc v0.2.107
│   ├── hex v0.4.3
│   ├── itertools v0.10.1 (*)
│   ├── lexical-parse-float v0.8.2 (*)
│   ├── libc v0.2.107
│   ├── libz-sys v1.1.3 (*)
│   ├── md-5 v0.9.1 (感觉重合了)
│   │   ├── block-buffer v0.9.0
│   │   │   ├── block-padding v0.2.1
│   │   │   └── generic-array v0.14.4 (*)
│   │   ├── digest v0.9.0 (*)
│   │   └── opaque-debug v0.3.0
│   ├── memchr v2.4.1
│   ├── mt19937 v2.0.1
│   │   └── rand_core v0.6.3 (*)
│   ├── nix v0.23.0
│   │   ├── bitflags v1.3.2
│   │   ├── cfg-if v1.0.0
│   │   ├── libc v0.2.107
│   │   └── memoffset v0.6.4
│   │       [build-dependencies]
│   │       └── autocfg v1.0.1
│   ├── num-bigint v0.4.3 (*)
│   ├── num-complex v0.4.0 (*)
│   ├── num-integer v0.1.44 (*)
│   ├── num-traits v0.2.14 (*)
│   ├── num_enum v0.5.4
│   │   ├── derivative v2.2.0 (proc-macro)
│   │   │   ├── proc-macro2 v1.0.32 (*)
│   │   │   ├── quote v1.0.10 (*)
│   │   │   └── syn v1.0.81 (*)
│   │   └── num_enum_derive v0.5.4 (proc-macro)
│   │       ├── proc-macro-crate v1.1.0
│   │       │   ├── thiserror v1.0.30 (*)
│   │       │   └── toml v0.5.8
│   │       │       └── serde v1.0.130 (*)
│   │       ├── proc-macro2 v1.0.32 (*)
│   │       ├── quote v1.0.10 (*)
│   │       └── syn v1.0.81 (*)
│   ├── puruspe v0.1.5
│   ├── rand v0.8.4 (*)
│   ├── rand_core v0.6.3 (*)
│   ├── rustpython-common v0.0.0 (/home/waoa/RustPython/common) (*)
│   ├── rustpython-derive v0.1.2 (proc-macro) (/home/waoa/RustPython/derive)
│   │   ├── indexmap v1.7.0 (*)
│   │   ├── maplit v1.0.2
│   │   ├── once_cell v1.8.0
│   │   ├── proc-macro2 v1.0.32 (*)
│   │   ├── quote v1.0.10 (*)
│   │   ├── rustpython-bytecode v0.1.2 (/home/waoa/RustPython/bytecode) (*)
│   │   ├── rustpython-compiler v0.1.2 (/home/waoa/RustPython/compiler/porcelain) (*)
│   │   ├── serde_json v1.0.69
│   │   ├── syn v1.0.81 (*)
│   │   ├── syn-ext v0.3.1
│   │   │   └── syn v1.0.81 (*)
│   │   └── textwrap v0.14.2
│   ├── rustpython-parser v0.1.2 (/home/waoa/RustPython/parser) (*)
│   ├── rustpython-vm v0.1.2 (/home/waoa/RustPython/vm)
│   │   ├── adler32 v1.2.0
│   │   ├── ahash v0.7.6 (*)
│   │   ├── ascii v1.0.0
│   │   ├── atty v0.2.14 (*)
│   │   ├── bitflags v1.3.2
│   │   ├── bstr v0.2.17 (*)
│   │   ├── caseless v0.2.1
│   │   │   └── unicode-normalization v0.1.19
│   │   │       └── tinyvec v1.5.0
│   │   │           └── tinyvec_macros v0.1.0
│   │   │   [build-dependencies]
│   │   │   └── regex v1.5.4 (*)
│   │   ├── cfg-if v1.0.0
│   │   ├── chrono v0.4.19
│   │   ├── crossbeam-utils v0.8.5 (*)
│   │   ├── exitcode v1.1.2
│   │   ├── flate2 v1.0.22 (*)
│   │   ├── getrandom v0.2.3 (*)
│   │   ├── half v1.8.2
│   │   ├── hex v0.4.3
│   │   ├── hexf-parse v0.2.1
│   │   ├── indexmap v1.7.0 (*)
│   │   ├── is-macro v0.1.9 (proc-macro)
│   │   │   ├── Inflector v0.11.4
│   │   │   │   ├── lazy_static v1.4.0
│   │   │   │   └── regex v1.5.4 (*)
│   │   │   ├── pmutil v0.5.3
│   │   │   │   ├── proc-macro2 v1.0.32 (*)
│   │   │   │   ├── quote v1.0.10 (*)
│   │   │   │   └── syn v1.0.81 (*)
│   │   │   ├── proc-macro2 v1.0.32 (*)
│   │   │   ├── quote v1.0.10 (*)
│   │   │   └── syn v1.0.81 (*)
│   │   ├── itertools v0.10.1 (*)
│   │   ├── libc v0.2.107
│   │   ├── log v0.4.14 (*)
│   │   ├── memchr v2.4.1
│   │   ├── nix v0.23.0 (*)
│   │   ├── num-bigint v0.4.3 (*)
│   │   ├── num-complex v0.4.0 (*)
│   │   ├── num-integer v0.1.44 (*)
│   │   ├── num-rational v0.4.0
│   │   │   ├── num-bigint v0.4.3 (*)
│   │   │   ├── num-integer v0.1.44 (*)
│   │   │   └── num-traits v0.2.14 (*)
│   │   │   [build-dependencies]
│   │   │   └── autocfg v1.0.1
│   │   ├── num-traits v0.2.14 (*)
│   │   ├── num_cpus v1.13.0
│   │   │   └── libc v0.2.107
│   │   ├── num_enum v0.5.4 (*)
│   │   ├── once_cell v1.8.0
│   │   ├── parking_lot v0.11.2 (*)
│   │   ├── paste v1.0.6 (proc-macro)
│   │   ├── rand v0.8.4 (*)
│   │   ├── result-like v0.4.2
│   │   │   └── result-like-derive v0.4.2 (proc-macro)
│   │   │       ├── Inflector v0.11.4 (*)
│   │   │       ├── pmutil v0.5.3 (*)
│   │   │       ├── proc-macro2 v1.0.32 (*)
│   │   │       ├── quote v1.0.10 (*)
│   │   │       ├── syn v1.0.81 (*)
│   │   │       └── syn-ext v0.3.1 (*)
│   │   ├── rustpython-ast v0.1.0 (/home/waoa/RustPython/ast) (*)
│   │   ├── rustpython-bytecode v0.1.2 (/home/waoa/RustPython/bytecode) (*)
│   │   ├── rustpython-common v0.0.0 (/home/waoa/RustPython/common) (*)
│   │   ├── rustpython-compiler v0.1.2 (/home/waoa/RustPython/compiler/porcelain) (*)
│   │   ├── rustpython-compiler-core v0.1.2 (/home/waoa/RustPython/compiler) (*)
│   │   ├── rustpython-derive v0.1.2 (proc-macro) (/home/waoa/RustPython/derive) (*)
│   │   ├── rustpython-parser v0.1.2 (/home/waoa/RustPython/parser) (*)
│   │   ├── rustpython-pylib v0.1.0 (/home/waoa/RustPython/vm/pylib-crate)
│   │   ├── rustyline v9.0.0
│   │   │   ├── bitflags v1.3.2
│   │   │   ├── cfg-if v1.0.0
│   │   │   ├── dirs-next v2.0.0 (*)
│   │   │   ├── fd-lock v3.0.0
│   │   │   │   ├── cfg-if v1.0.0
│   │   │   │   └── libc v0.2.107
│   │   │   ├── libc v0.2.107
│   │   │   ├── log v0.4.14 (*)
│   │   │   ├── memchr v2.4.1
│   │   │   ├── nix v0.22.0
│   │   │   │   ├── bitflags v1.3.2
│   │   │   │   ├── cfg-if v1.0.0
│   │   │   │   ├── libc v0.2.107
│   │   │   │   └── memoffset v0.6.4 (*)
│   │   │   ├── radix_trie v0.2.1
│   │   │   │   ├── endian-type v0.1.2
│   │   │   │   └── nibble_vec v0.1.0
│   │   │   │       └── smallvec v1.7.0
│   │   │   ├── smallvec v1.7.0
│   │   │   ├── unicode-segmentation v1.8.0
│   │   │   ├── unicode-width v0.1.9
│   │   │   └── utf8parse v0.2.0
│   │   ├── serde v1.0.130 (*)
│   │   ├── sre-engine v0.1.2
│   │   │   ├── bitflags v1.3.2
│   │   │   └── num_enum v0.5.4 (*)
│   │   ├── static_assertions v1.1.0
│   │   ├── strum v0.21.0
│   │   ├── strum_macros v0.21.1 (proc-macro)
│   │   │   ├── heck v0.3.3
│   │   │   │   └── unicode-segmentation v1.8.0
│   │   │   ├── proc-macro2 v1.0.32 (*)
│   │   │   ├── quote v1.0.10 (*)
│   │   │   └── syn v1.0.81 (*)
│   │   ├── thiserror v1.0.30 (*)
│   │   ├── thread_local v1.1.3
│   │   │   └── once_cell v1.8.0
│   │   ├── timsort v0.1.2
│   │   ├── uname v0.1.1
│   │   │   └── libc v0.2.107
│   │   ├── unic-ucd-bidi v0.9.0
│   │   │   ├── unic-char-property v0.9.0 (*)
│   │   │   ├── unic-char-range v0.9.0
│   │   │   └── unic-ucd-version v0.9.0 (*)
│   │   ├── unic-ucd-category v0.9.0 (*)
│   │   ├── unic-ucd-ident v0.9.0 (*)
│   │   ├── unicode-casing v0.1.0
│   │   ├── unicode_names2 v0.4.0
│   │   └── which v4.2.2
│   │       ├── either v1.6.1
│   │       └── libc v0.2.107
│   │   [build-dependencies]
│   │   ├── itertools v0.10.1 (*)
│   │   └── rustc_version v0.4.0
│   │       └── semver v1.0.4
│   ├── sha-1 v0.9.8
│   ├── sha2 v0.9.8
│   ├── sha3 v0.9.1
│   │   ├── block-buffer v0.9.0 (*)
│   │   ├── digest v0.9.0 (*)
│   │   ├── keccak v0.1.0
│   │   └── opaque-debug v0.3.0
│   ├── socket2 v0.4.2 (*)
│   ├── termios v0.3.3
│   │   └── libc v0.2.107
│   ├── unic-char-property v0.9.0 (*)
│   ├── unic-normal v0.9.0
│   │   └── unic-ucd-normal v0.9.0
│   │       ├── unic-char-property v0.9.0 (*)
│   │       ├── unic-char-range v0.9.0
│   │       ├── unic-ucd-hangul v0.9.0
│   │       │   └── unic-ucd-version v0.9.0 (*)
│   │       └── unic-ucd-version v0.9.0 (*)
│   ├── unic-ucd-age v0.9.0
│   │   ├── unic-char-property v0.9.0 (*)
│   │   ├── unic-char-range v0.9.0
│   │   └── unic-ucd-version v0.9.0 (*)
│   ├── unic-ucd-bidi v0.9.0 (*)
│   ├── unic-ucd-category v0.9.0 (*)
│   ├── unic-ucd-ident v0.9.0 (*)
│   ├── unicode-casing v0.1.0
│   ├── unicode_names2 v0.4.0
│   └── xml-rs v0.8.4
├── rustpython-vm v0.1.2 (/home/waoa/RustPython/vm) (*)
└── rustyline v9.0.0 (*)
[dev-dependencies]
├── cpython v0.7.0
│   ├── libc v0.2.107
│   ├── num-traits v0.2.14 (*)
│   ├── paste v1.0.6 (proc-macro)
│   └── python3-sys v0.7.0
│       └── libc v0.2.107
│       [build-dependencies]
│       └── regex v1.5.4 (*)
├── criterion v0.3.5
│   ├── atty v0.2.14 (*)
│   ├── cast v0.2.7
│   │   [build-dependencies]
│   │   └── rustc_version v0.4.0 (*)
│   ├── clap v2.33.3 (*)
│   ├── criterion-plot v0.4.4
│   │   ├── cast v0.2.7 (*)
│   │   └── itertools v0.10.1 (*)
│   ├── csv v1.1.6
│   │   ├── bstr v0.2.17 (*)
│   │   ├── csv-core v0.1.10 (*)
│   │   ├── itoa v0.4.8
│   │   ├── ryu v1.0.5
│   │   └── serde v1.0.130 (*)
│   ├── itertools v0.10.1 (*)
│   ├── lazy_static v1.4.0
│   ├── num-traits v0.2.14 (*)
│   ├── oorandom v11.1.3
│   ├── plotters v0.3.1
│   │   ├── num-traits v0.2.14 (*)
│   │   ├── plotters-backend v0.3.2
│   │   └── plotters-svg v0.3.1
│   │       └── plotters-backend v0.3.2
│   ├── rayon v1.5.1
│   │   ├── crossbeam-deque v0.8.1
│   │   │   ├── cfg-if v1.0.0
│   │   │   ├── crossbeam-epoch v0.9.5
│   │   │   │   ├── cfg-if v1.0.0
│   │   │   │   ├── crossbeam-utils v0.8.5 (*)
│   │   │   │   ├── lazy_static v1.4.0
│   │   │   │   ├── memoffset v0.6.4 (*)
│   │   │   │   └── scopeguard v1.1.0
│   │   │   └── crossbeam-utils v0.8.5 (*)
│   │   ├── either v1.6.1
│   │   └── rayon-core v1.9.1
│   │       ├── crossbeam-channel v0.5.1
│   │       │   ├── cfg-if v1.0.0
│   │       │   └── crossbeam-utils v0.8.5 (*)
│   │       ├── crossbeam-deque v0.8.1 (*)
│   │       ├── crossbeam-utils v0.8.5 (*)
│   │       ├── lazy_static v1.4.0
│   │       └── num_cpus v1.13.0 (*)
│   │   [build-dependencies]
│   │   └── autocfg v1.0.1
│   ├── regex v1.5.4
│   │   └── regex-syntax v0.6.25
│   ├── serde v1.0.130 (*)
│   ├── serde_cbor v0.11.2
│   │   ├── half v1.8.2
│   │   └── serde v1.0.130 (*)
│   ├── serde_derive v1.0.130 (proc-macro) (*)
│   ├── serde_json v1.0.69 (*)
│   ├── tinytemplate v1.2.1
│   │   ├── serde v1.0.130 (*)
│   │   └── serde_json v1.0.69 (*)
│   └── walkdir v2.3.2
│       └── same-file v1.0.6
└── python3-sys v0.7.0 (*)