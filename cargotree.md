# 待移植树，从最低下开始移植


**如果已经移植或者为build-dependencies等，则该枝会被删除**

rustpython v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave)
├── rustpython-vm v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/vm)
│   ├── blake2 v0.8.1
│   │   ├── byte-tools v0.3.1
│   │   ├── crypto-mac v0.7.0
│   │   │   ├── generic-array v0.12.3 (*)
│   │   │   └── subtle v1.0.0
│   │   ├── digest v0.8.1 (*)
│   │   └── opaque-debug v0.2.3
│   ├── caseless v0.2.1
│   │   └── unicode-normalization v0.1.12
│   │       └── smallvec v1.3.0
│   │   [build-dependencies]
│   │   └── regex v1.3.6 (*)
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
│   ├── lexical v4.0.0
│   ├── libz-sys v1.0.25 (*)
│   ├── md-5 v0.8.0
│   │   ├── block-buffer v0.7.3 (*)
│   │   ├── digest v0.8.1 (*)
│   │   └── opaque-debug v0.2.3
│   ├── nix v0.17.0
│   │   ├── bitflags v1.2.1
│   │   ├── cfg-if v0.1.10
│   │   ├── libc v0.2.71
│   │   └── void v1.0.2
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
│   ├── rustc_version_runtime v0.1.5
│   │   ├── rustc_version v0.2.3 (*)
│   │   └── semver v0.9.0 (*)
│   │   [build-dependencies]
│   │   ├── rustc_version v0.2.3 (*)
│   │   └── semver v0.9.0 (*)
│   ├── rustpython-derive v0.1.2 (proc-macro) (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/derive)
│   │   ├── maplit v1.0.2
│   │   ├── once_cell v1.3.1
│   │   ├── proc-macro2 v1.0.10 (*)
│   │   ├── quote v1.0.3 (*)
│   │   ├── rustpython-bytecode v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/bytecode) (*)
│   │   ├── rustpython-compiler v0.1.2 (/home/waoa/incubator-teaclave-sgx-sdk/teerustpython/enclave/compiler) (*)
│   │   └── syn v1.0.17 (*)
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
│   ├── sha3 v0.8.2
│   │   ├── block-buffer v0.7.3 (*)
│   │   ├── byte-tools v0.3.1
│   │   ├── digest v0.8.1 (*)
│   │   ├── keccak v0.1.0
│   │   └── opaque-debug v0.2.3
│   ├── smallbox v0.8.0
│   ├── statrs v0.12.0
│   │   └── rand v0.7.3 (*)
│   ├── subprocess v0.2.4
│   │   └── libc v0.2.71
│   ├── uname v0.1.1
│   │   └── libc v0.2.71
└── rustyline v6.1.0 (*)
[dev-dependencies]
└── cpython v0.2.1
