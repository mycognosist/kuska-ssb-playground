use kuska_ssb::keystore::OwnedIdentity;

fn main() {
    // create an OwnedIdentity with public and private keys
    let identity = OwnedIdentity::create();

    // print the identity
    println!("{:#?}", identity);
}
