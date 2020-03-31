use serde::*;
use std::io::{stdin, Read};
use ed25519_dalek::{PublicKey, SecretKey, Keypair};
use hpos_config_core::{Config, public_key};
use std::{env, fs};

#[derive(Deserialize)]
struct Happs {
    hhaids: Vec<String>,
}

#[derive(Serialize)]
struct Payload {
    #[serde(serialize_with = "publickey_to_host")]
    host: PublicKey,
    hhaids: Vec<String>,
}

fn publickey_to_host<S>(public_key: &PublicKey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&public_key::to_base36_id(&public_key))
}

fn read_keypair() -> Keypair {
    let config_path = env::var("HPOS_CONFIG_PATH").unwrap();
    let config_json = fs::read(config_path).unwrap();
    let Config::V1 { seed, .. } = serde_json::from_slice(&config_json).unwrap();

    let private_key = SecretKey::from_bytes(&seed).unwrap();
    let public_key = PublicKey::from(&private_key);

    Keypair::from_bytes(&[private_key.to_bytes(), public_key.to_bytes()].concat()).unwrap()
}

fn read_hhaids() -> Vec<String> {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let happs: Happs = toml::from_str(&buffer).unwrap();
    happs.hhaids
}

fn main() {
    let keypair = read_keypair();

    let payload = Payload {
        host: keypair.public,
        hhaids: read_hhaids(),
    };

    let payload_json = serde_json::to_string(&payload).unwrap();

    let signature = keypair.sign(payload_json.as_bytes());
    let signature_base64 = base64::encode(&signature.to_bytes()[..]);

    println!("{:?}", signature_base64);
}

#[cfg(test)]
mod tests;