# 待移植树，从最低下开始移植


**如果已经移植，则该枝会被删除**

rustpython v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave)
├── cfg-if v0.1.10
├── clap v2.33.0
│   ├── ansi_term v0.11.0
│   ├── atty v0.2.14
│   │   └── libc v0.2.71
│   ├── bitflags v1.2.1
│   ├── strsim v0.8.0
│   ├── textwrap v0.11.0
│   │   └── unicode-width v0.1.7
│   ├── unicode-width v0.1.7
│   └── vec_map v0.8.1
├── dirs-next v1.0.1
│   ├── cfg-if v0.1.10
│   └── dirs-sys-next v0.1.0
│       └── libc v0.2.71
├── env_logger v0.7.1
├── log v0.4.8 (*)
├── num-traits v0.2.11
│   [build-dependencies]
│   └── autocfg v1.0.0
├── rustpython-compiler v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/compiler)
│   ├── arrayvec v0.5.1
│   ├── indexmap v1.3.2
│   │   [build-dependencies]
│   │   └── autocfg v1.0.0
│   ├── itertools v0.8.2
│   │   └── either v1.5.3
│   ├── log v0.4.8 (*)
│   ├── num-complex v0.2.4
│   │   ├── num-traits v0.2.11 (*)
│   │   └── serde v1.0.106
│   │   [build-dependencies]
│   │   └── autocfg v1.0.0
│   ├── rustpython-bytecode v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/bytecode)
│   │   ├── bincode v1.2.1
│   │   │   ├── byteorder v1.3.4
│   │   │   └── serde v1.0.106 ()
│   │   ├── bitflags v1.2.1
│   │   ├── itertools v0.8.2 (*)
│   │   ├── lz4-compress v0.1.1
│   │   │   ├── byteorder v0.5.3
│   │   │   └── quick-error v1.2.3
│   │   ├── num-bigint v0.2.6
│   │   └── serde v1.0.106 (*)
│   └── rustpython-parser v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/parser)
│       ├── lalrpop-util v0.17.2
│       ├── log v0.4.8 (*)
│       ├── num-bigint v0.2.6 (*)
│       ├── num-traits v0.2.11 (*)
│       ├── unic-emoji-char v0.9.0
│       │   ├── unic-char-property v0.9.0
│       │   │   └── unic-char-range v0.9.0
│       │   ├── unic-char-range v0.9.0
│       │   └── unic-ucd-version v0.9.0
│       │       └── unic-common v0.9.0
│       ├── unic-ucd-ident v0.9.0
│       │   ├── unic-char-property v0.9.0 (*)
│       │   ├── unic-char-range v0.9.0
│       │   └── unic-ucd-version v0.9.0 (*)
│       └── unicode_names2 v0.4.0
│       [build-dependencies]
│       └── lalrpop v0.17.2
│           ├── ascii-canvas v2.0.0
│           │   └── term v0.5.2
│           │       ├── byteorder v1.3.4
│           │       └── dirs v1.0.5
│           │           └── libc v0.2.71
│           ├── atty v0.2.14 (*)
│           ├── bit-set v0.5.1
│           │   └── bit-vec v0.5.1
│           ├── diff v0.1.12
│           ├── docopt v1.1.0
│           │   ├── lazy_static v1.4.0
│           │   ├── regex v1.3.6 (*)
│           │   ├── serde v1.0.106 (*)
│           │   └── strsim v0.9.3
│           ├── ena v0.13.1
│           │   └── log v0.4.8 (*)
│           ├── itertools v0.8.2 (*)
│           ├── lalrpop-util v0.17.2
│           ├── petgraph v0.4.13
│           │   ├── fixedbitset v0.1.9
│           │   └── ordermap v0.3.5
│           ├── regex v1.3.6 (*)
│           ├── regex-syntax v0.6.17
│           ├── serde v1.0.106 (*)
│           ├── serde_derive v1.0.106 (proc-macro) (*)
│           ├── sha2 v0.8.1
│           ├── string_cache v0.7.5
│           │   ├── lazy_static v1.4.0
│           │   ├── new_debug_unreachable v1.0.4
│           │   ├── phf_shared v0.7.24
│           │   │   └── siphasher v0.2.3
│           │   ├── precomputed-hash v0.1.1
│           │   ├── serde v1.0.106 (*)
│           │   └── string_cache_shared v0.3.0
│           │   [build-dependencies]
│           │   └── string_cache_codegen v0.4.4
│           │       ├── phf_generator v0.7.24
│           │       │   ├── phf_shared v0.7.24 (*)
│           │       │   └── rand v0.6.5
│           │       ├── proc-macro2 v1.0.10 (*)
│           │       ├── quote v1.0.3 (*)
│           │       └── string_cache_shared v0.3.0
│           ├── term v0.5.2 (*)
│           └── unicode-xid v0.1.0
├── rustpython-parser v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/parser) (*)
├── rustpython-vm v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/vm)
│   ├── adler32 v1.0.4
│   ├── arr_macro v0.1.3
│   │   ├── arr_macro_impl v0.1.3 (proc-macro)
│   │   │   ├── proc-macro-hack v0.5.15 (proc-macro)
│   │   │   ├── quote v1.0.3 (*)
│   │   │   └── syn v1.0.17 (*)
│   │   └── proc-macro-hack v0.5.15 (proc-macro)
│   ├── base64 v0.11.0
│   ├── bitflags v1.2.1
│   ├── blake2 v0.8.1
│   │   ├── byte-tools v0.3.1
│   │   ├── crypto-mac v0.7.0
│   │   │   ├── generic-array v0.12.3 (*)
│   │   │   └── subtle v1.0.0
│   │   ├── digest v0.8.1 (*)
│   │   └── opaque-debug v0.2.3
│   ├── bstr v0.2.12
│   │   ├── lazy_static v1.4.0
│   │   ├── memchr v2.3.3
│   │   ├── regex-automata v0.1.9
│   │   │   └── byteorder v1.3.4
│   │   └── serde v1.0.106 (*)
│   ├── byteorder v1.3.4
│   ├── caseless v0.2.1
│   │   └── unicode-normalization v0.1.12
│   │       └── smallvec v1.3.0
│   │   [build-dependencies]
│   │   └── regex v1.3.6 (*)
│   ├── chrono v0.4.11
│   ├── crc v1.8.1
│   │   [build-dependencies]
│   │   └── build_const v0.2.1
│   ├── crc32fast v1.2.0
│   ├── crossbeam-utils v0.7.2
│   │   ├── cfg-if v0.1.10
│   │   └── lazy_static v1.4.0
│   │   [build-dependencies]
│   │   └── autocfg v1.0.0
│   ├── csv v1.1.3
│   │   ├── bstr v0.2.12 (*)
│   │   ├── csv-core v0.1.10
│   │   │   └── memchr v2.3.3
│   │   ├── itoa v0.4.5
│   │   ├── ryu v1.0.3
│   │   └── serde v1.0.106 (*)
│   ├── digest v0.8.1 (*)
│   ├── dns-lookup v1.0.1
│   │   ├── libc v0.2.71
│   │   └── socket2 v0.3.12
│   │       ├── cfg-if v0.1.10
│   │       └── libc v0.2.71
│   ├── exitcode v1.1.2
│   ├── flate2 v1.0.14
│   │   ├── cfg-if v0.1.10
│   │   ├── crc32fast v1.2.0 (*)
│   │   ├── libc v0.2.71
│   │   └── libz-sys v1.0.25
│   │       └── libc v0.2.71
│   │       [build-dependencies]
│   │       ├── cc v1.0.50
│   │       └── pkg-config v0.3.17
│   ├── foreign-types-shared v0.1.1
│   ├── generational-arena v0.2.3
│   │   └── cfg-if v0.1.10
│   ├── gethostname v0.2.1
│   │   └── libc v0.2.71
│   ├── getrandom v0.1.14
│   │   ├── cfg-if v0.1.10
│   │   └── libc v0.2.71
│   ├── hex v0.4.2
│   ├── hexf-parse v0.1.0
│   ├── indexmap v1.3.2 (*)
│   ├── is-macro v0.1.8 (proc-macro)
│   │   ├── Inflector v0.11.4
│   │   ├── pmutil v0.5.3
│   │   │   ├── proc-macro2 v1.0.10 (*)
│   │   │   ├── quote v1.0.3 (*)
│   │   │   └── syn v1.0.17 (*)
│   │   ├── proc-macro2 v1.0.10 (*)
│   │   ├── quote v1.0.3 (*)
│   │   └── syn v1.0.17 (*)
│   ├── itertools v0.8.2 (*)
│   ├── lexical v4.0.0
│   │   ├── cfg-if v0.1.10
│   │   └── lexical-core v0.6.2
│   │       ├── arrayvec v0.4.12
│   │       │   └── nodrop v0.1.14
│   │       ├── cfg-if v0.1.10
│   │       ├── ryu v1.0.3
│   │       └── static_assertions v0.3.4
│   │       [build-dependencies]
│   │       └── rustc_version v0.2.3
│   │           └── semver v0.9.0
│   │               └── semver-parser v0.7.0
│   │   [build-dependencies]
│   │   └── rustc_version v0.2.3 (*)
│   ├── libc v0.2.71
│   ├── libz-sys v1.0.25 (*)
│   ├── log v0.4.8 (*)
│   ├── maplit v1.0.2
│   ├── md-5 v0.8.0
│   │   ├── block-buffer v0.7.3 (*)
│   │   ├── digest v0.8.1 (*)
│   │   └── opaque-debug v0.2.3
│   ├── mt19937 v1.0.1
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
├── libc v0.2.107
├── log v0.4.14 ()
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
│   │   │   └── serde v1.0.130 ()
│   │   ├── itertools v0.10.1
│   │   ├── lz4_flex v0.9.0
│   │   │   └── twox-hash v1.6.1
│   │   │       ├── cfg-if v1.0.0
│   │   │       └── static_assertions v1.1.0
│   │   ├── num-bigint v0.4.3
│   │   ├── num-complex v0.4.0
│   │   │   ├── num-traits v0.2.14 ()
│   │   │   └── serde v1.0.130 ()
│   │   └── serde v1.0.130 ()
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
│   │   ├── itertools v0.10.1 ()
│   │   ├── log v0.4.14 ()
│   │   ├── num-complex v0.4.0 ()
│   │   ├── num-traits v0.2.14 ()
│   │   ├── rustpython-ast v0.1.0 (/home/waoa/RustPython/ast)
│   │   │   ├── num-bigint v0.4.3 ()
│   │   │   └── rustpython-common v0.0.0 (/home/waoa/RustPython/common)
│   │   │       ├── ascii v1.0.0
│   │   │       ├── cfg-if v1.0.0
│   │   │       ├── hexf-parse v0.2.1
│   │   │       ├── lexical-parse-float v0.8.2
│   │   │       │   ├── lexical-parse-integer v0.8.0
│   │   │       │   │   ├── lexical-util v0.8.1
│   │   │       │   │   │   └── static_assertions v1.1.0
│   │   │       │   │   └── static_assertions v1.1.0
│   │   │       │   ├── lexical-util v0.8.1 ()
│   │   │       │   └── static_assertions v1.1.0
│   │   │       ├── libc v0.2.107
│   │   │       ├── lock_api v0.4.5
│   │   │       │   └── scopeguard v1.1.0
│   │   │       ├── num-bigint v0.4.3 ()
│   │   │       ├── num-complex v0.4.0 ()
│   │   │       ├── num-traits v0.2.14 ()
│   │   │       ├── once_cell v1.8.0
│   │   │       ├── parking_lot v0.11.2
│   │   │       │   ├── instant v0.1.12
│   │   │       │   │   └── cfg-if v1.0.0
│   │   │       │   ├── lock_api v0.4.5 ()
│   │   │       │   └── parking_lot_core v0.8.5
│   │   │       │       ├── cfg-if v1.0.0
│   │   │       │       ├── instant v0.1.12 ()
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
│   │   └── rustpython-bytecode v0.1.2 (/home/waoa/RustPython/bytecode) ()
│   ├── rustpython-parser v0.1.2 (/home/waoa/RustPython/parser)
│   │   ├── ahash v0.7.6 ()
│   │   ├── lalrpop-util v0.19.6
│   │   ├── log v0.4.14 ()
│   │   ├── num-bigint v0.4.3 ()
│   │   ├── num-traits v0.2.14 ()
│   │   ├── phf v0.10.0
│   │   │   ├── phf_macros v0.10.0 (proc-macro)
│   │   │   │   ├── phf_generator v0.10.0
│   │   │   │   │   ├── phf_shared v0.10.0
│   │   │   │   │   │   └── siphasher v0.3.7
│   │   │   │   │   └── rand v0.8.4
│   │   │   │   ├── phf_shared v0.10.0 ()
│   │   │   │   ├── proc-macro-hack v0.5.19 (proc-macro)
│   │   │   │   ├── proc-macro2 v1.0.32 ()
│   │   │   │   ├── quote v1.0.10 ()
│   │   │   │   └── syn v1.0.81 ()
│   │   │   ├── phf_shared v0.10.0 ()
│   │   │   └── proc-macro-hack v0.5.19 (proc-macro)
│   │   ├── rustpython-ast v0.1.0 (/home/waoa/RustPython/ast) ()
│   │   ├── unic-emoji-char v0.9.0
│   │   │   ├── unic-char-property v0.9.0 ()
│   │   │   ├── unic-char-range v0.9.0
│   │   │   └── unic-ucd-version v0.9.0 ()
│   │   ├── unic-ucd-ident v0.9.0
│   │   │   ├── unic-char-property v0.9.0 ()
│   │   │   ├── unic-char-range v0.9.0
│   │   │   └── unic-ucd-version v0.9.0 ()
│   │   └── unicode_names2 v0.4.0
│   │   [build-dependencies]
│   │   └── lalrpop v0.19.6
│   │       ├── ascii-canvas v3.0.0
│   │       │   └── term v0.7.0
│   │       │       └── dirs-next v2.0.0 ()
│   │       ├── atty v0.2.14 ()
│   │       ├── bit-set v0.5.2
│   │       │   └── bit-vec v0.6.3
│   │       ├── diff v0.1.12
│   │       ├── ena v0.14.0
│   │       │   └── log v0.4.14
│   │       ├── itertools v0.10.1 ()
│   │       ├── lalrpop-util v0.19.6
│   │       │   └── regex v1.5.4
│   │       ├── petgraph v0.5.1
│   │       │   ├── fixedbitset v0.2.0
│   │       │   └── indexmap v1.7.0 ()
│   │       ├── pico-args v0.4.2
│   │       ├── regex v1.5.4 ()
│   │       ├── regex-syntax v0.6.25
│   │       ├── string_cache v0.8.2
│   │       │   ├── lazy_static v1.4.0
│   │       │   ├── new_debug_unreachable v1.0.4
│   │       │   ├── parking_lot v0.11.2 ()
│   │       │   ├── phf_shared v0.8.0
│   │       │   │   └── siphasher v0.3.7
│   │       │   └── precomputed-hash v0.1.1
│   │       ├── term v0.7.0 ()
│   │       ├── tiny-keccak v2.0.2
│   │       │   └── crunchy v0.2.2
│   │       └── unicode-xid v0.2.2
│   └── thiserror v1.0.30
├── rustpython-parser v0.1.2 (/home/waoa/RustPython/parser) ()
├── rustpython-stdlib v0.1.2 (/home/waoa/RustPython/stdlib)
│   ├── adler32 v1.2.0
│   ├── ahash v0.7.6 ()
│   ├── ascii v1.0.0
│   ├── base64 v0.13.0
│   ├── blake2 v0.9.2
│   │   ├── crypto-mac v0.8.0
│   │   │   ├── generic-array v0.14.4
│   │   │   │   └── typenum v1.14.0
│   │   │   │   [build-dependencies]
│   │   ├── bitflags v1.2.1
│   │   ├── cfg-if v0.1.10
│   │   ├── libc v0.2.71
│   │   └── void v1.0.2
│   ├── num-bigint v0.2.6 (*)
│   ├── num-complex v0.2.4 (*)
│   ├── num-integer v0.1.42 (*)
│   ├── num-iter v0.1.40
│   │   ├── num-integer v0.1.42 (*)
│   │   └── num-traits v0.2.11 (*)
│   │   [build-dependencies]
│   │   └── autocfg v1.0.0
│   ├── num-rational v0.2.4
│   │   ├── num-bigint v0.2.6 (*)
│   │   ├── num-integer v0.1.42 (*)
│   │   └── num-traits v0.2.11 (*)
│   │   [build-dependencies]
│   │   └── autocfg v1.0.0
│   ├── num-traits v0.2.11 (*)
│   ├── num_cpus v1.12.0
│   │   └── libc v0.2.71
│   ├── num_enum v0.4.3
│   │   ├── derivative v2.1.1 (proc-macro)
│   │   │   ├── proc-macro2 v1.0.10 (*)
│   │   │   ├── quote v1.0.3 (*)
│   │   │   └── syn v1.0.17 (*)
│   │   └── num_enum_derive v0.4.3 (proc-macro)
│   │       ├── proc-macro-crate v0.1.4
│   │       │   └── toml v0.5.6
│   │       │       └── serde v1.0.106 (*)
│   │       ├── proc-macro2 v1.0.10 (*)
│   │       ├── quote v1.0.3 (*)
│   │       └── syn v1.0.17 (*)
│   ├── once_cell v1.3.1
│   ├── parking_lot v0.10.2 (https://github.com/Amanieu/parking_lot#99aa542b)
│   │   ├── instant v0.1.3
│   │   ├── lock_api v0.3.4 (https://github.com/Amanieu/parking_lot#99aa542b)
│   │   │   └── scopeguard v1.1.0
│   │   └── parking_lot_core v0.7.2 (https://github.com/Amanieu/parking_lot#99aa542b)
│   │       ├── cfg-if v0.1.10
│   │       ├── instant v0.1.3
│   │       ├── libc v0.2.71
│   │       └── smallvec v1.3.0
│   ├── paste v0.1.10
│   │   ├── paste-impl v0.1.10 (proc-macro)
│   │   │   ├── proc-macro-hack v0.5.15 (proc-macro)
│   │   │   ├── proc-macro2 v1.0.10 (*)
│   │   │   ├── quote v1.0.3 (*)
│   │   │   └── syn v1.0.17 (*)
│   │   └── proc-macro-hack v0.5.15 (proc-macro)
│   ├── pwd v1.3.0
│   │   ├── failure v0.1.7
│   │   │   ├── backtrace v0.3.41
│   │   │   │   ├── backtrace-sys v0.1.35
│   │   │   │   │   └── libc v0.2.71
│   │   │   │   │   [build-dependencies]
│   │   │   │   │   └── cc v1.0.50
│   │   │   │   ├── cfg-if v0.1.10
│   │   │   │   ├── libc v0.2.71
│   │   │   │   └── rustc-demangle v0.1.16
│   │   │   └── failure_derive v0.1.7 (proc-macro)
│   │   │       ├── proc-macro2 v1.0.10 (*)
│   │   │       ├── quote v1.0.3 (*)
│   │   │       ├── syn v1.0.17 (*)
│   │   │       └── synstructure v0.12.3
│   │   │           ├── proc-macro2 v1.0.10 (*)
│   │   │           ├── quote v1.0.3 (*)
│   │   │           ├── syn v1.0.17 (*)
│   │   │           └── unicode-xid v0.2.0
│   │   └── libc v0.2.71
│   ├── rand v0.7.3 (*)
│   ├── rand_core v0.5.1 (*)
│   ├── regex v1.3.6 (*)
│   ├── result-like v0.2.1
│   │   └── is-macro v0.1.8 (proc-macro) (*)
│   ├── rustc_version_runtime v0.1.5
│   │   ├── rustc_version v0.2.3 (*)
│   │   └── semver v0.9.0 (*)
│   │   [build-dependencies]
│   │   ├── rustc_version v0.2.3 (*)
│   │   └── semver v0.9.0 (*)
│   ├── rustpython-bytecode v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/bytecode) (*)
│   ├── rustpython-compiler v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/compiler) (*)
│   ├── rustpython-derive v0.1.2 (proc-macro) (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/derive)
│   │   ├── maplit v1.0.2
│   │   ├── once_cell v1.3.1
│   │   ├── proc-macro2 v1.0.10 (*)
│   │   ├── quote v1.0.3 (*)
│   │   ├── rustpython-bytecode v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/bytecode) (*)
│   │   ├── rustpython-compiler v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/compiler) (*)
│   │   └── syn v1.0.17 (*)
│   ├── rustpython-parser v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/parser) (*)
│   ├── rustyline v6.1.0
│   │   ├── cfg-if v0.1.10
│   │   ├── dirs v2.0.1
│   │   │   ├── cfg-if v0.1.10
│   │   │   └── dirs-sys v0.3.4
│   │   │       ├── cfg-if v0.1.10
│   │   │       └── libc v0.2.71
│   │   ├── libc v0.2.71
│   │   ├── log v0.4.8 (*)
│   │   ├── memchr v2.3.3
│   │   ├── nix v0.17.0 (*)
│   │   ├── unicode-segmentation v1.6.0
│   │   ├── unicode-width v0.1.7
│   │   └── utf8parse v0.2.0
│   ├── serde v1.0.106 (*)
│   ├── sha-1 v0.8.2
│   ├── sha2 v0.8.1 (*)
│   ├── sha3 v0.8.2
│   │   ├── block-buffer v0.7.3 (*)
│   │   ├── byte-tools v0.3.1
│   │   ├── digest v0.8.1 (*)
│   │   ├── keccak v0.1.0
│   │   └── opaque-debug v0.2.3
│   ├── smallbox v0.8.0
│   ├── socket2 v0.3.12 (*)
│   ├── statrs v0.12.0
│   │   └── rand v0.7.3 (*)
│   ├── subprocess v0.2.4
│   │   └── libc v0.2.71
│   ├── thread_local v1.0.1 (*)
│   ├── uname v0.1.1
│   │   └── libc v0.2.71
│   ├── unic-bidi v0.9.0
│   │   ├── matches v0.1.8
│   │   └── unic-ucd-bidi v0.9.0
│   │       ├── unic-char-property v0.9.0 (*)
│   │       ├── unic-char-range v0.9.0
│   │       └── unic-ucd-version v0.9.0 (*)
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
│   ├── unic-ucd-category v0.9.0
│   │   ├── matches v0.1.8
│   │   ├── unic-char-property v0.9.0 (*)
│   │   ├── unic-char-range v0.9.0
│   │   └── unic-ucd-version v0.9.0 (*)
│   ├── unic-ucd-ident v0.9.0 (*)
│   ├── unicode-casing v0.1.0
│   ├── unicode_names2 v0.4.0
│   └── volatile v0.2.6
└── rustyline v6.1.0 (*)
[dev-dependencies]
└── cpython v0.2.1
    ├── libc v0.2.71
    ├── num-traits v0.2.11 (*)
    └── python3-sys v0.2.1
        └── libc v0.2.71
        [build-dependencies]
        └── regex v1.3.6 (*)