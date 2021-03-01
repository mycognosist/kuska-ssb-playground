use kuska_ssb::feed::Message;
use kuska_ssb::keystore::OwnedIdentity;

use serde_json::json;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    // create an OwnedIdentity with public and private keys
    let identity = OwnedIdentity::create();

    // create an example object with json encoding
    let content = json!({ "type": "example" });

    // sign the json object with the given identity (keys)
    let signed_object = Message::sign(None, &identity, content);

    // print the signed input
    println!("{:#?}", signed_object);

    let sig = signed_object.unwrap();
    println!("{:#?}", sig.content());
    println!("{:#?}", sig.signature());

    // verify message
    match Message::from_value(sig.value) {
        Ok(_) => println!("verified"),
        Err(e) => println!("{}", e)
    };
    // verified

    let object = json!({
        "author": "@3xCL4Nphuiez1r5hosboiFxMEFCx6pr17I3dq44VUI=.ed25519",
        "content": {
            "type": "example"
        },
        "signature": "3uCBNE9Ht0dG5dYy1yDTmeBrnEDTm7q9xUZ0DYoZgPYoiudbmD3wkI5g6HTPGshyDIwCJm5j9XQQ8SYtdJ1Cg==.sig.ed25519",
    });

    // verify message
    Message::from_value(object)?;
    // Error: InvalidJson

    Ok(())
}
