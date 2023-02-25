use didcomm::{did::DIDResolver, secrets::SecretsResolver, Message, UnpackMetadata, UnpackOptions};
use serde_json::json;

pub struct MsgPacker<'a> {
    from_did: &'a str,
    to_did: &'a str,
    msg: &'a str,
    did_resolver: &'a dyn DIDResolver,
}

impl MsgPacker<'_> {
    pub async fn build(&self) -> String {
        let msg = Message::build(
            "example-1".to_owned(),
            "example/v1".to_owned(),
            json!({ "msg": self.msg }),
        )
        .from(self.from_did.to_owned())
        .to(self.to_did.to_owned())
        .finalize();

        msg.pack_plaintext(self.did_resolver)
            .await
            .expect("Message packed")
    }
}

pub struct MsgUnpacker<'a> {
    msg: &'a str,
    did_resolver: &'a dyn DIDResolver,
    secrets_resolver: &'a dyn SecretsResolver,
    unpack_options: &'a UnpackOptions,
}

impl MsgUnpacker<'_> {
    pub async fn build(&self) -> (Message, UnpackMetadata) {
        Message::unpack(
            self.msg,
            self.did_resolver,
            self.secrets_resolver,
            self.unpack_options,
        )
        .await
        .expect("Message unpacked")
    }
}
