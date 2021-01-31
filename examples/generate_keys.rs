use async_std::fs::File;

use kuska_ssb::keystore;
use kuska_ssb::keystore::OwnedIdentity;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[async_std::main]
async fn main() -> Result<()> {

    // struct OwnedIdentity {
    //     id: String,
    //     pk: ed25519::PublicKey,
    //     sk: ed25519::SecretKey,
    // }

    // create an OwnedIdentity struct with id, public key and private key
    let identity = OwnedIdentity::create();

    // print the identity
    println!("{:#?}", identity);

    // load an identity from file (Patchwork interoperability)
    let patchwork_identity = keystore::from_patchwork_local().await.expect("read local secret");

    // print the patchwork identity from file
    println!("{:#?}", patchwork_identity);

    // create test file for storing patchwork identity
    let mut file = File::create("patchwork_id").await?;

    // write patchwork identity to file
    keystore::write_patchwork_config(&identity, &mut file).await.expect("write patchwork secret");

    // {
    //   "id": "@z+GWOeqa8SKcAwdVe79UFI0ayoTGs+ZsAM+NwuTve4A=.ed25519",
    //   "curve": "ed25519",
    //   "public": "z+GWOeqa8SKcAwdVe79UFI0ayoTGs+ZsAM+NwuTve4A=.ed25519",
    //   "private": "vzgGRxbqmPLR3Lx4eiwyJXitfeYQdmaMrkZfx0J1UbP4ZY56prxIpwDB1V7v1QUjRrKhMaz5mwAz43C5O97gA==.ed25519"
    // }

    Ok(())
}
