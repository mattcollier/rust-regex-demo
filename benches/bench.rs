
#![feature(test)]

extern crate test;

const VERES_ONE_DID: &str = r#"_:b0 <http://purl.org/dc/terms/creator> <https://ashburn.capybara.veres.one/consensus/continuity2017/voters/z6MkgTBtCodgNvf1SaQLRbCppkVMo7BggAP4NohtPY8ZNqic> .
_:b0 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/webledger#CreateWebLedgerRecord> .
_:b0 <https://w3id.org/security#proof> _:b1 .
_:b0 <https://w3id.org/security#proof> _:b3 .
_:b0 <https://w3id.org/webledger#record> "{\"@context\":[\"https://w3id.org/did/v0.11\",\"https://w3id.org/veres-one/v1\"],\"assertionMethod\":[{\"controller\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d\",\"id\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d#z6MkqVQKNUG994U8mK6p7CX6PMtijsgDuhQBUEXgfAPCqQEP\",\"publicKeyBase58\":\"C39GnE1hoWyfepG7RdZFYGLivJQNVp9pnDckptRBvBT1\",\"type\":\"Ed25519VerificationKey2018\"}],\"authentication\":[{\"controller\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d\",\"id\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d#z6MkjXWUNtoT9e1BDSt5zLmMuY7k9u99euxCQiduXCTwrY2u\",\"publicKeyBase58\":\"65FRneZ1p6Wi6x3PJmoX4SZkLKsJF2hqihiygvVvwKFX\",\"type\":\"Ed25519VerificationKey2018\"}],\"capabilityDelegation\":[{\"controller\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d\",\"id\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d#z6MknXoqGqfAG7vjBTPqNZjf1HWcMD2c5csTCkdkY62S8BRy\",\"publicKeyBase58\":\"95YngbQivaSG4xZ8gzmpABxcXdkkfjd6Wjiphp4RCxeb\",\"type\":\"Ed25519VerificationKey2018\"}],\"capabilityInvocation\":[{\"controller\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d\",\"id\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d#z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d\",\"publicKeyBase58\":\"3bKogxzDN4y6wv8rVncv23YEB3YW6MCWZUCVEAoGufMF\",\"type\":\"Ed25519VerificationKey2018\"}],\"id\":\"did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d\"}"^^<http://www.w3.org/1999/02/22-rdf-syntax-ns#JSON> .
_:b2 <http://purl.org/dc/terms/created> "2021-01-09T20:50:08Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> _:b1 .
_:b2 <http://purl.org/dc/terms/creator> <did:v1:test:nym:z279yHL6HsxRzCPU78DAWgZVieb8xPK1mJKJBbP8T2CezuFY#z279yHL6HsxRzCPU78DAWgZVieb8xPK1mJKJBbP8T2CezuFY> _:b1 .
_:b2 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519Signature2018> _:b1 .
_:b2 <https://w3id.org/security#capability> <did:v1:uuid:c37e914a-1e2a-4d59-9668-ee93458fd19a> _:b1 .
_:b2 <https://w3id.org/security#capabilityAction> "write" _:b1 .
_:b2 <https://w3id.org/security#jws> "MOCKPROOF" _:b1 .
_:b2 <https://w3id.org/security#proofPurpose> <https://w3id.org/security#capabilityInvocationMethod> _:b1 .
_:b4 <http://purl.org/dc/terms/created> "2021-01-09T20:50:08Z"^^<http://www.w3.org/2001/XMLSchema#dateTime> _:b3 .
_:b4 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <https://w3id.org/security#Ed25519Signature2018> _:b3 .
_:b4 <https://w3id.org/security#capability> <did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d> _:b3 .
_:b4 <https://w3id.org/security#capabilityAction> "create" _:b3 .
_:b4 <https://w3id.org/security#jws> "eyJhbGciOiJFZERTQSIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19..u_T7y7P_woiPmpxfnY0rDdA_o25A9m9BOUfXu4zc1PqfIs92Po8sJn_D2xSPI2Ijuz22T6YibLtud1NgvFO1BQ" _:b3 .
_:b4 <https://w3id.org/security#proofPurpose> <https://w3id.org/security#capabilityInvocationMethod> _:b3 .
_:b4 <https://w3id.org/security#verificationMethod> <did:v1:test:nym:z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d#z6Mkh3arHDEehcTa4QyZBMaks96DzcpMWESsFV7R4SmHpt8d> _:b3 .
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_veres_one_did_match(b: &mut Bencher) {
        b.iter(|| {
            // println!("JJJJJJ");
            let _c: Vec<_> = VERES_ONE_DID.lines().map(regex_demo::parse_quads).collect();
            // println!("LLLLL {}", c.len());
        })
    }    
}