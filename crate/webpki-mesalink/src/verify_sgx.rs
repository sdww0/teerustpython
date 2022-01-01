// Copyright 2015 Brian Smith.
// Copyright 2018 Yiming Jing.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use crate::cert::Cert;
use crate::time;
use crate::{EndEntityCert, TLSServerTrustAnchors, TrustAnchor, RSA_PKCS1_2048_8192_SHA256};
use crate::{Error, SignatureAlgorithm};

use untrusted;

static IAS_CA_TRUST_ANCHOR: TLSServerTrustAnchors = TLSServerTrustAnchors(&[
    TrustAnchor {
        subject: b"1\x0b0\t\x06\x03U\x04\x06\x13\x02US1\x0b0\t\x06\x03U\x04\x08\x0c\x02CA1\x140\x12\x06\x03U\x04\x07\x0c\x0bSanta Clara1\x1a0\x18\x06\x03U\x04\n\x0c\x11Intel Corporation100.\x06\x03U\x04\x03\x0c\'Intel SGX Attestation Report Signing CA",
        spki: b"0\r\x06\t*\x86H\x86\xf7\r\x01\x01\x01\x05\x00\x03\x82\x01\x8f\x000\x82\x01\x8a\x02\x82\x01\x81\x00\x9f<d~\xb5w<\xbbQ-\'2\xc0\xd7A^\xbbU\xa0\xfa\x9e\xde.d\x91\x99\xe6\x82\x1d\xb9\x10\xd51w7\twFjj^G\x86\xcc\xd2\xdd\xeb\xd4\x14\x9dj/c%R\x9d\xd1\x0c\xc9\x877\xb0w\x9c\x1a\x07\xe2\x9cG\xa1\xae\x00IHGlH\x9fE\xa5\xa1]z\xc8\xec\xc6\xac\xc6E\xad\xb4=\x87g\x9d\xf5\x9c\t;\xc5\xa2\xe9ilTxT\x1b\x97\x9euKW9\x14\xbeU\xd3/\xf4\xc0\x9d\xdf\'!\x994\xcd\x99\x05\'\xb3\xf9.\xd7\x8f\xbf)$j\xbe\xcbq$\x0e\xf3\x9c-q\x07\xb4GTZ\x7f\xfb\x10\xeb\x06\nh\xa9\x85\x80!\x9e6\x91\tRh8\x92\xd6\xa5\xe2\xa8\x08\x03\x19>@u1@N6\xb3\x15b7\x99\xaa\x82Pt@\x97T\xa2\xdf\xe8\xf5\xaf\xd5\xfec\x1e\x1f\xc2\xaf8\x08\x90o(\xa7\x90\xd9\xdd\x9f\xe0`\x93\x9b\x12W\x90\xc5\x80]\x03}\xf5j\x99S\x1b\x96\xdei\xde3\xed\"l\xc1 }\x10B\xb5\xc9\xab\x7f@O\xc7\x11\xc0\xfeGi\xfb\x95x\xb1\xdc\x0e\xc4i\xea\x1a%\xe0\xff\x99\x14\x88n\xf2i\x9b#[\xb4\x84}\xd6\xff@\xb6\x06\xe6\x17\x07\x93\xc2\xfb\x98\xb3\x14X\x7f\x9c\xfd%sb\xdf\xea\xb1\x0b;\xd2\xd9vs\xa1\xa4\xbdD\xc4S\xaa\xf4\x7f\xc1\xf2\xd3\xd0\xf3\x84\xf7J\x06\xf8\x9c\x08\x9f\r\xa6\xcd\xb7\xfc\xee\xe8\xc9\x82\x1a\x8eT\xf2\\\x04\x16\xd1\x8cF\x83\x9a_\x80\x12\xfb\xdd=\xc7M%by\xad\xc2\xc0\xd5Z\xffo\x06\"B]\x1b\x02\x03\x01\x00\x01",
        name_constraints: None
    }
]);

static ISA_CA_CERT: &'static [u8] =
    include_bytes!("../third-party/AttestationReportSigningCACert.der");

pub fn verify_is_valid_attestation_report<'a, F>(
    supported_sig_algs: &[&SignatureAlgorithm],
    cert: &Cert<'a>,
    time: time::Time,
    verify: F,
) -> Result<(), Error>
where
    F: FnOnce(&Cert<'a>, untrusted::Input) -> Result<(), Error>,
{
    untrusted::read_all_optional(cert.comments, Error::BadDER, |comments| {
        let input = match comments {
            Some(input) => input,
            None => return Err(Error::ExtensionValueInvalid),
        };

        // 1. Parse and extract "report|signature|certificate"
        let report_input = read_pipe_separated_segment(input)?;
        let sig_input = read_pipe_separated_segment(input)?;
        let sig_cert_input = read_pipe_separated_segment(input)?;

        // 2. Base63 decode the signature and certfiicate
        let sig_buf = base64::decode(sig_input.as_slice_less_safe()).map_err(|_| Error::BadDER)?;
        let sig_cert_buf =
            base64::decode_config(sig_cert_input.as_slice_less_safe(), base64::STANDARD)
                .map_err(|_| Error::BadDER)?;
        let sig_cert = EndEntityCert::from(&sig_cert_buf)?;

        // 3. Verify if the report is properly signed
        sig_cert.verify_is_valid_tls_server_cert(
            supported_sig_algs,
            &IAS_CA_TRUST_ANCHOR,
            &[ISA_CA_CERT],
            time,
        )?;
        sig_cert.verify_signature(
            &RSA_PKCS1_2048_8192_SHA256,
            report_input.as_slice_less_safe(),
            &sig_buf,
        )?;

        // 4. Invoke the closure function to process the report
        verify(cert, report_input)
    })
}

fn read_pipe_separated_segment<'a>(
    input: &mut untrusted::Reader<'a>,
) -> Result<untrusted::Input<'a>, Error> {
    if input.at_end() {
        // Nothing to read from this point
        return Err(Error::BadDER);
    }
    let mark1 = input.mark();
    let mut mark2 = input.mark();
    loop {
        match input.read_byte() {
            Ok(byte) if byte == 0x7C => break,
            Err(_) => break,
            _ => mark2 = input.mark(),
        }
    }
    input
        .get_input_between_marks(mark1, mark2)
        .map_err(|_| Error::BadDER)
}
