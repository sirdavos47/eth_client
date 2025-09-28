use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::EncodePublicKey};

pub struct Wallet {
    pub private_key: RsaPrivateKey,
    pub public_key: RsaPublicKey,
    pub address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let mut rng = OsRng;
        let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Anahtar üretilemedi");
        let public_key = private_key.to_public_key();
        let address = base64::encode(public_key.to_public_key_der().unwrap());
        Wallet {
            private_key,
            public_key,
            address,
        }
    }
}
