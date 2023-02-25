use async_trait::async_trait;
use didkit::{ResolutionInputMetadata, DID_METHODS};

#[derive(Default)]
pub struct Resolver;

#[async_trait(?Send)]
impl didcomm::did::DIDResolver for Resolver {
    async fn resolve(
        &self,
        did: &str,
    ) -> Result<Option<didcomm::did::DIDDoc>, didcomm::error::Error> {
        let did_resolver = DID_METHODS.to_resolver();
        let (resolution_metadata, doc, doc_metadata) = did_resolver
            .resolve(did, &ResolutionInputMetadata::default())
            .await;

        dbg!(&resolution_metadata, &doc, &doc_metadata);

        if resolution_metadata.error.is_some() {
            panic!("Error resolving DID: {:?}", resolution_metadata.error);
        }

        let doc = doc.unwrap();
        Ok(Some(didcomm::did::DIDDoc {
            did: did.to_string(),
            key_agreements: vec![],
            authentications: vec![serde_json::to_string(&doc.authentication.unwrap()).unwrap()],
            verification_methods: vec![],
            services: vec![],
        }))
    }
}

#[cfg(test)]
mod test {
    use didcomm::did::DIDResolver;

    use crate::resolver::Resolver;

    #[tokio::test]
    async fn resolve_did() {
        let r = Resolver::default();
        let did_doc = r
            .resolve("did:key:z6Mkqf9HERbJSA1bGKyom3g4Ng1y3y9yCQVpLLWAHMvfjaJB")
            .await
            .unwrap()
            .unwrap();
        println!("{did_doc:?}");
    }

    #[test]
    fn gen_dids() {
        let jwk = didkit::JWK::generate_ed25519().unwrap();

        println!("{:?}", jwk.params);

        println!(
            "--- Private key ---\n{}\n",
            serde_json::to_string(&jwk).unwrap()
        );
        println!(
            "--- Public key ---\n{}\n",
            serde_json::to_string(&jwk.to_public()).unwrap()
        );

        let did = didkit::DID_METHODS
            .generate(&didkit::Source::KeyAndPattern(&jwk, "key"))
            .ok_or(didkit::Error::UnableToGenerateDID)
            .unwrap();

        println!("--- DID ---\n{}\n", &did);
    }
}
