use kuska_ssb::keystore::OwnedIdentity;

fn main() {

    // create an OwnedIdentity struct with id, public key and private key
    let identity = OwnedIdentity::create();

    /* struct OwnedIdentity {
     *     id: String,
     *     pk: ed25519::PublicKey,
     *     sk: ed25519::SecretKey,
     * }
     */
    
    // print the identity
    println!("{:#?}", identity);
}
