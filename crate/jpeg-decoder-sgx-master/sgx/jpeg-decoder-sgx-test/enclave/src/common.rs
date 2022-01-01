use std::untrusted::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::prelude::v1::*;

pub fn crash_test_files(test_dir: &Path) -> Vec<PathBuf> {
    let mut test_files:Vec<PathBuf> = vec![
        "./tests/crashtest/images/imagetestsuite/90e46387f562ca8fa106b51dfcda1dc6.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/f6b4389c3cf0f5997b2e5a4b905aea8d.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/bd8cf05698aee36b82b4caf58edea442.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/7dbf474f80e466e9e25ee46b84166420.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/e6d9eca2c7405e13cfb850b7d0ef7476.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/ef724193653930f52acffa90e6426fd2.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/a7326ba8f3f4559991126474dd30083d.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/f6419b06a39ff09604343848658b1a41.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/fd44dc63fa7bdd12ee34fc602231ef02.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/40bb78b1ac031125a6d8466b374962a8.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/de4ae285a275bcfe2ac87c0126742552.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/897b8b6d8feb466aa6cad5f512c3fce2.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/5a43fa2cf9c1e47f0331ef71b928ee55.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/4838ece0d3900220d33528ee027289bc.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/9efd60f04cd971daa83d3131e6d6f389.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/cc4ee796d16c9fe68978166c7cd1ae1b.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/c52ffdd6a0346c4d09271f8ccbdfd5a3.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/255015e07b6f9137b53b0f97d67a8aef.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/c8bc97335529d069a753c67475b8c82c.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/865db3dd2d380626f16b6f9dc6d62dba.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/46f5d9c1b0fe352353688f736e5617b6.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/754664a12e36abff7950e796c906ae39.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/5dc71b1d868ef137394d3cc23abea65a.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/551c2656a4f6f9f5ea7e9945b9081202.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/ba60305ac83fe3d8ef01da1d9a0ecc79.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/5315c35bbcc28d8eee419028ac9f38e0.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/56d4a1bb53241f7c5ed6ab531320a542.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/32d08f4a5eb10332506ebedbb9bc7257.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/28968137f4fc75fbf56f16d7a7a8551a.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/28c74d9284d9836017fd519f6932efd8.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/2183d39878e734cf79b62428b02fafb5.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/acce3629083f0e348e94fb58f952d3de.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/b0b8914cc5f7a6eff409f16d8cc236c5.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/5482a54657765056f1a94116a8dbffe7.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/ef1f8a057bb6056674fad92f6b8c0acd.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/627c0779eb46b98f751187c5c9f43aa3.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/316be81dfdeeb942e904feb3a77f4f83.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/6903d4538fd33c8fd0ded32cb30d618e.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/f8e19feecd246156b5d7e79efc455e99.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/eddea4ef9629be031f750a8ff0b7497c.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/96b3e939852157613fa2e48d58fe35fe.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/3cc4a7fc6481ea3681138da4643f3d16.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/c1ca5583e4bfadc73e7fe9418b6e6bf4.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/59d3b529c78ac722127c41ba75b3355b.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/5bc61724b33e34a6188a817f9f2f8138.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/2c9e7a1805f8b47630bbb83d21bf8222.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/adcb34b94f4c839bdd29037419a0ee53.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/e18bb52107598f65b81b02be2c6c5124.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/138d3b9e0d9fbf641b8135981e597c3a.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/194531363df5b73f59c4c0517422f917.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/8a9cc8eeed66aeb423a91c44111d9450.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/c8c1a5675f82021d92b928a10c597bad.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/d085a42245996e5750a30ccb48791bcf.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/f6d3f522dcb693d9e731d5a0fb4e1393.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/3ef05501315073d9d4e1c6b654d99ac0.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/8e5e74dbf9b68a322fbb9512db837329.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/8417a305e3b43d5b1bda4ff06a660c54.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/cc23dd79637b606cf5ba234a037e17ba.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/3ba6af611cc5467cfdbd5566561b8478.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/d15b71b8cebe35a57cc6e996cc09218b.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/21a84b8472f6d18f5bb5c0026e97cfaa.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/7e7cdf7f4ee50b308531313bbf43e0c3.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/eecb78b937a7c5f04aae2f5b0f5b5acc.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/3976a754ef0aca80e84e2c403d714579.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/5baad44ca4702949724234e35c5bb341.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/b55977028a3a574336966b6536640fc9.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/1cbb1bb37d62c44f67374cd451643dc4.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/79f5fc6bca756e1f067c6fc83e18b32e.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/5633ed9d0eb700d0093bf85d86a95ebf.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/d22db5be7594c17a18a047ca9264ea0a.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/786b67badc535fc95a4a76c29a0e0146.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/a54f8c866cbef6e6cda858c85d72dfc8.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/ce380515a534e8226209daae00e7b4e8.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/39f43f280b31152f1d27df3f9d189317.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/5c67195f6993c9f8d0d32d4ffe0d8e62.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/a17806f32b45d63eea5230e7893e1f15.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/8546907dbe574744d7fea6ca9de1de6b.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/b5369bcbddca7135a5708c5237ad64e4.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/f012a4321f00f12af6b1eee7580ffb9c.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/21ad703b38e2c350215bb92a849486f3.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/3ea649db8e81a46ca4f92fb3238f78ff.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/c3018ebe53d0046eecb58858ca869a99.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/b4103df93880fc5677c2a081e4bfc712.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/75e4bd7544a85af6438497980b62fba5.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/8e330afbd99ba01b66570ed62fcdc6ab.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/7997b6b229f25315d33f5c7085e37500.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/f1fad47f213bb64c99f714652f30e49e.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/c4ced510f44a9bfe85c696c05a7f791d.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/6de166ee2a3a60df9017650e2a808408.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/fddcfc778ada60229380c2493fc4c243.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/46e5ac4a62d7a445a7c1fb704fafe05c.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/de5884cec093257d239f3b8be3e2f2e5.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/acb1fac4e618f636d415f62496e8b70e.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/7acc832f70b2ca62e58a953f3b90fd82.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/4085c929e00c446d3fee18b5b20a27f9.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/f006e96f3b27fdfaa075322d759ea2e8.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/d3b044a94486cae0224c002800ddd642.jpg".into(),
        "./tests/crashtest/images/imagetestsuite/72d091e08c93c9e590360130fa35221b.jpg".into(),
        "./tests/crashtest/images/derive-huffman-codes-overflow.jpg".into(),
        "./tests/crashtest/images/dc-predictor-overflow.jpg".into(),
        "./tests/crashtest/images/missing-sof.jpg".into(),
    ];

    if let Ok(file) = File::open(test_dir.join("disabled.list")) {
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();

            if line.is_empty() || line.starts_with("#") {
                continue;
            }

            let path = test_dir.join(Path::new(&line));

            if !test_files.contains(&path) {
                panic!("The file {:?} specified in {:?} could not be found among the files being tested", line, test_dir.join("disabled.txt"));
            }

            let position = test_files.iter().position(|p| p == &path).unwrap();
            test_files.remove(position);
        }
    }

    test_files
}

pub fn reftest_test_files(test_dir: &Path) -> Vec<PathBuf> {
    let mut test_files:Vec<PathBuf> = vec![
        "tests/reftest/images/mjpeg.jpg".into(),
        "tests/reftest/images/ycck.jpg".into(),
        "tests/reftest/images/restarts.jpg".into(),
        "tests/reftest/images/extraneous-data.jpg".into(),
        "tests/reftest/images/mozilla/jpg-size-1x1.jpg".into(),
        "tests/reftest/images/mozilla/jpg-size-15x15.jpg".into(),
        "tests/reftest/images/mozilla/jpg-size-5x5.jpg".into(),
        "tests/reftest/images/mozilla/jpg-size-33x33.jpg".into(),
        "tests/reftest/images/mozilla/jpg-cmyk-2.jpg".into(),
        "tests/reftest/images/mozilla/jpg-size-31x31.jpg".into(),
        "tests/reftest/images/mozilla/jpg-size-2x2.jpg".into(),
        "tests/reftest/images/mozilla/jpg-size-4x4.jpg".into(),
        "tests/reftest/images/mozilla/jpg-size-8x8.jpg".into(),
        "tests/reftest/images/mozilla/jpg-size-16x16.jpg".into(),
        "tests/reftest/images/mozilla/jpg-srgb-icc.jpg".into(),
        "tests/reftest/images/mozilla/jpg-size-3x3.jpg".into(),
        "tests/reftest/images/mozilla/jpg-size-9x9.jpg".into(),
        "tests/reftest/images/mozilla/jpg-gray.jpg".into(),
        "tests/reftest/images/mozilla/jpg-progressive.jpg".into(),
        "tests/reftest/images/mozilla/jpg-size-7x7.jpg".into(),
        "tests/reftest/images/mozilla/jpg-size-32x32.jpg".into(),
        "tests/reftest/images/mozilla/jpg-size-17x17.jpg".into(),
        "tests/reftest/images/mozilla/jpg-size-6x6.jpg".into(),
        "tests/reftest/images/mozilla/jpg-cmyk-1.jpg".into(),
        "tests/reftest/images/16bit-qtables.jpg".into(),
        "tests/reftest/images/rgb.jpg".into(),
    ];

    if let Ok(file) = File::open(test_dir.join("disabled.list")) {
        for line in BufReader::new(file).lines() {
            let line = line.unwrap();

            if line.is_empty() || line.starts_with("#") {
                continue;
            }

            let path = test_dir.join(Path::new(&line));

            if !test_files.contains(&path) {
                panic!("The file {:?} specified in {:?} could not be found among the files being tested", line, test_dir.join("disabled.txt"));
            }

            let position = test_files.iter().position(|p| p == &path).unwrap();
            test_files.remove(position);
        }
    }

    test_files
}
