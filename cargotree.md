# 待移植树，从最低下开始移植


**如果已经移植或者为build-dependencies等，则该枝会被删除**

rustpython v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave)
├── cfg-if v0.1.10
├── dirs-next v1.0.1
├── env_logger v0.7.1
├── log v0.4.8 (*)
├── num-traits v0.2.11
├── rustpython-compiler v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/compiler)
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
│   │   ├── num-integer v0.1.42 (*)
│   │   ├── num-traits v0.2.11 (*)
│   │   └── time v0.1.43
│   │       └── libc v0.2.71
│   ├── crc v1.8.1
│   │   [build-dependencies]
│   │   └── build_const v0.2.1
│   ├── crc32fast v1.2.0
│   │   └── cfg-if v0.1.10
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
│   ├── nix v0.17.0
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