use serde::{Serialize,Deserialize};
use ring::signature::{Ed25519KeyPair, Signature, KeyPair, VerificationAlgorithm, EdDSAParameters, UnparsedPublicKey};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RawTransaction {
    pub input: u32,
    pub output: u32,
}

pub struct SignedTransaction {
    pub raw: RawTransaction,
    pub signature: Signature,
}

/// Create digital signature of a transaction
pub fn sign(t: &RawTransaction, key: &Ed25519KeyPair) -> Signature {
    let content = bincode::serialize(t).unwrap();
    key.sign(&content)
}

/// Verify digital signature of a transaction, using public key instead of secret key
pub fn verify(t: &RawTransaction, public_key: &<Ed25519KeyPair as KeyPair>::PublicKey, signature: &Signature) -> bool {
    // public_key.verify(&[t.input.clone(), t.output.clone()].concat(), signautre.as_ref()).is_ok()
    let _public_key = UnparsedPublicKey::new(&EdDSAParameters, public_key);
    _public_key.verify(bincode::serialize(t).unwrap().as_ref(), signature.as_ref()).is_ok()
}

#[cfg(any(test, test_utilities))]
mod tests {
    use super::*;
    use crate::crypto::key_pair;
    use log::{info, debug, error};
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            env_logger::builder().is_test(true).try_init().unwrap();
        });
    }
    pub fn generate_random_transaction() -> RawTransaction {
        info!("Generating random transaction!!!!!!!!!!!\n");
        RawTransaction {
            input: rand::random::<u32>(),
            output: rand::random::<u32>(),
        }
    }

    #[test]
    fn sign_verify() {
        setup();
        info!("sign_verify test");
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);

        // casting then printing
        // get length of signature
        let length = signature.as_ref().to_vec().len();
        println!("Signature Length: {:?}", length);
        println!("Signature: {:?}", &signature.as_ref().to_vec());
        assert!(verify(&t, &(key.public_key()), &signature));
    }

    #[test]
    fn another_sign_verify() {
        setup();
        info!("another_sign_verify test");
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);
        assert!(verify(&t, &(key.public_key()), &signature));
    }

    #[test]
    fn sign_verify_fail() {
        setup();
        info!("sign_verify_fail test");
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);
        let mut t2 = t.clone();
        t2.output += 1;
        assert!(!verify(&t2, &(key.public_key()), &signature));
    }
}
