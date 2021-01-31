use kuska_ssb::feed::Message;
use kuska_ssb::keystore::OwnedIdentity;

use serde_json::json;

fn main() {
    // create an OwnedIdentity with public and private keys
    let identity = OwnedIdentity::create();

    // create an example object with json encoding
    let content = json!({ "type": "example" });

    // sign the json object with the given identity (keys)
    let signed_object = Message::sign(None, &identity, content);

    // print the signed input
    println!("{:#?}", signed_object);
}
