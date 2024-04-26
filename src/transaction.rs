use serde::{Serialize,Deserialize};
use ring::signature::{Ed25519KeyPair, Signature, KeyPair, EdDSAParameters, UnparsedPublicKey};
use crate::crypto::hash::{Hashable};

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


use crate::crypto::hash::H256;
/* Please add the following code snippet into `src/transaction.rs`: */
impl Hashable for Transaction {
    fn hash(&self) -> H256 {
        let bytes = bincode::serialize(&self).unwrap();
        ring::digest::digest(&ring::digest::SHA256, &bytes).into()
    }
}


#[cfg(any(test, test_utilities))]
mod tests {
    use super::*;
    use crate::crypto::key_pair;

    pub fn generate_random_transaction() -> RawTransaction {
        RawTransaction {
            input: rand::random::<u32>(),
            output: rand::random::<u32>(),
        }
    }

    #[test]
    fn sign_verify() {
        let t = generate_random_transaction();
        let key = key_pair::random();
        let signature = sign(&t, &key);

        // casting then printing
        // get length of signature
        let _length = signature.as_ref().to_vec().len();
        assert!(verify(&t, &(key.public_key()), &signature));
    }

}
