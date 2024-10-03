use hex_literal::hex;
use sha1::{Digest, Sha1};

fn main() {
    // create a Sha1 object
    let mut hasher = Sha1::new();

    // process input message
    hasher.update(b"hello world");

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 20]
    let result = hasher.finalize();
    assert_eq!(result[..], hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));

    println!("{result:x}");
}
