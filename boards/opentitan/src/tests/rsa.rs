use crate::tests::run_kernel_op;
use capsules::public_key_crypto::rsa_keys::RSA2048Keys;
use kernel::debug;
use kernel::hil::public_key_crypto::keys::{PubKey, PubPrivKey, RsaKey, RsaPrivKey};
use kernel::static_init;
use kernel::utilities::mut_imut_buffer::MutImutBuffer;

static PUB_KEY: [u8; 260] = [
    // Public Exponent
    0x00, 0x01, 0x00, 0x01, // Modulus
    0xc9, 0x03, 0x2e, 0x93, 0x05, 0x1c, 0xe8, 0x6b, 0x0f, 0x41, 0x5c, 0x7e, 0x2d, 0x1e, 0x3b, 0xee,
    0x9a, 0x37, 0xfe, 0x6b, 0x1b, 0xa5, 0x9f, 0x8b, 0x51, 0x69, 0x10, 0x79, 0x1d, 0xd1, 0x2c, 0x22,
    0xf1, 0x88, 0xed, 0xf4, 0xda, 0x1c, 0x8d, 0xa0, 0xd0, 0x5e, 0x29, 0xf3, 0x36, 0x78, 0xef, 0x07,
    0x43, 0xc5, 0xd6, 0xf5, 0x8a, 0x2a, 0x69, 0x70, 0x7d, 0x21, 0x45, 0x00, 0x5b, 0x13, 0x2b, 0x8e,
    0x7a, 0x7a, 0xaf, 0xe3, 0x97, 0x26, 0x54, 0x49, 0x34, 0x90, 0x69, 0x89, 0xaf, 0xc7, 0xc2, 0xa7,
    0x2a, 0x31, 0xc3, 0x78, 0x8b, 0x6d, 0x8a, 0x4c, 0xa9, 0xea, 0xe7, 0xc4, 0x2e, 0xa4, 0xc2, 0xce,
    0x4b, 0x98, 0xae, 0xa9, 0x73, 0xf2, 0x60, 0xde, 0xcb, 0x47, 0x9c, 0x22, 0x24, 0xe7, 0x5a, 0x95,
    0x6b, 0x61, 0xd9, 0x15, 0x41, 0xb4, 0x0f, 0x27, 0x77, 0x0b, 0x7c, 0xff, 0x29, 0xc1, 0xff, 0x86,
    0xae, 0x28, 0xfd, 0x33, 0x9f, 0x7e, 0xac, 0xfc, 0x39, 0x08, 0x72, 0x28, 0x62, 0x5d, 0xc1, 0x21,
    0x27, 0xa1, 0xbb, 0x2a, 0x38, 0xe5, 0x17, 0x74, 0xbc, 0x1e, 0x76, 0x3f, 0x1c, 0xa7, 0xa8, 0x57,
    0x81, 0x3c, 0x60, 0x56, 0xed, 0xe3, 0xa9, 0x7f, 0xb4, 0x3d, 0xfb, 0xbf, 0x4f, 0x38, 0x58, 0x3d,
    0x1b, 0x23, 0x6f, 0x39, 0xde, 0x5c, 0xc3, 0xdb, 0x47, 0x33, 0x4d, 0x7d, 0xa4, 0xf8, 0xce, 0xeb,
    0xc1, 0x4a, 0x6a, 0xe2, 0xe8, 0x5f, 0xac, 0xe5, 0x19, 0x09, 0xc7, 0xe4, 0x8d, 0xd3, 0xca, 0x66,
    0xca, 0xe9, 0x76, 0x4c, 0x75, 0x1d, 0x37, 0xf2, 0xc6, 0xe5, 0x74, 0x1f, 0xee, 0x5a, 0x0b, 0x8a,
    0x67, 0x90, 0xe6, 0x5e, 0x6a, 0x77, 0xe4, 0x36, 0xd7, 0x10, 0x40, 0x74, 0xa6, 0xfb, 0xf9, 0xfc,
    0xdb, 0x73, 0x8e, 0x7a, 0x32, 0x2d, 0xf6, 0xbc, 0xb2, 0x08, 0xdb, 0x1e, 0x3c, 0x01, 0xde, 0x4d,
];
static PRIV_KEY: [u8; 256] = [
    // Private Exponent
    0xbb, 0xbb, 0x4c, 0x09, 0x24, 0xf2, 0x4d, 0xa2, 0x87, 0x39, 0xdc, 0xff, 0x3e, 0x76, 0x09, 0x35,
    0x1b, 0x35, 0x06, 0x58, 0xd4, 0x16, 0x47, 0xbe, 0x1e, 0xc7, 0x48, 0x0a, 0x45, 0xad, 0xb0, 0x51,
    0xe6, 0x50, 0xa3, 0x24, 0x55, 0x7b, 0xeb, 0x4d, 0xf0, 0xac, 0xb0, 0xf3, 0x23, 0xc1, 0xa8, 0x43,
    0x99, 0xa9, 0x30, 0xcc, 0x5b, 0x40, 0xa6, 0xfe, 0xd6, 0xee, 0x76, 0x7a, 0x11, 0x95, 0x02, 0xcd,
    0xeb, 0x57, 0x9f, 0xe3, 0xa7, 0xab, 0xb5, 0x76, 0x35, 0x30, 0x56, 0x50, 0xb0, 0x29, 0x99, 0x82,
    0xf3, 0xe6, 0x4c, 0x0f, 0xcd, 0xef, 0xfd, 0x05, 0x02, 0x80, 0x8f, 0xfa, 0x6a, 0x31, 0x98, 0x7a,
    0x80, 0xa1, 0xd5, 0x26, 0x0f, 0x52, 0xa3, 0xe4, 0x0f, 0xe8, 0x0e, 0x4a, 0xd9, 0x3a, 0x75, 0x20,
    0x2d, 0x8c, 0xd1, 0xe8, 0x87, 0x57, 0x79, 0xfb, 0xba, 0xb3, 0xb1, 0x06, 0xc1, 0xe2, 0x1c, 0x18,
    0xeb, 0xc9, 0xd2, 0x8b, 0xa2, 0xf7, 0xc4, 0xf1, 0x1c, 0x8b, 0x2c, 0xd4, 0x2d, 0x55, 0xa9, 0x1b,
    0xee, 0xe3, 0x9f, 0x6f, 0xa0, 0x42, 0xa5, 0x09, 0x2f, 0x22, 0x93, 0x35, 0x16, 0xd5, 0xa2, 0x4d,
    0x1d, 0x1f, 0x54, 0x85, 0xd9, 0xdf, 0xbd, 0x74, 0x3c, 0x84, 0x59, 0x9a, 0xa4, 0x56, 0xdb, 0xb9,
    0x92, 0x89, 0x82, 0x89, 0x0f, 0xe0, 0x84, 0xc5, 0xdc, 0xca, 0x31, 0xd7, 0xc8, 0x06, 0xdf, 0x68,
    0xff, 0x14, 0xb1, 0x65, 0x7d, 0x1b, 0xb4, 0xa2, 0x7e, 0xd5, 0x06, 0xc0, 0x72, 0x2e, 0xbb, 0xc9,
    0x87, 0xd8, 0xb2, 0xa6, 0x82, 0xdf, 0x59, 0xdb, 0xc7, 0x79, 0x29, 0xff, 0xe2, 0xd2, 0x9f, 0x65,
    0x42, 0x83, 0x78, 0x53, 0xda, 0x18, 0x63, 0xf4, 0xdd, 0xdd, 0x4a, 0xf9, 0xc3, 0x02, 0x35, 0x64,
    0x5f, 0x37, 0xe8, 0x5c, 0xe4, 0xfb, 0x95, 0xec, 0xdd, 0x48, 0x37, 0x76, 0xc5, 0xad, 0x9c, 0xa5,
];

#[test_case]
fn rsa_import_key() {
    let key = unsafe { static_init!(RSA2048Keys, RSA2048Keys::new()) };

    debug!("check rsa key import... ");
    run_kernel_op(100);

    if let Err(e) = key.import_public_key(MutImutBuffer::Immutable(&PUB_KEY)) {
        panic!("Failed to import public key: {:?}", e.0);
    }
    if let Err(e) = key.import_private_key(MutImutBuffer::Immutable(&PRIV_KEY)) {
        panic!("Failed to import private key: {:?}", e.0);
    }

    run_kernel_op(1000);

    assert_eq!(
        key.map_modulus(&|modulus| {
            assert_eq!(modulus[..], PUB_KEY[4..]);
        }),
        Some(())
    );

    assert_eq!(
        key.map_exponent(&|exponent| {
            assert_eq!(exponent, PRIV_KEY);
        }),
        Some(())
    );

    assert_eq!(key.public_exponent(), Some(0x10001));

    debug!("    [ok]");
    run_kernel_op(100);
}
