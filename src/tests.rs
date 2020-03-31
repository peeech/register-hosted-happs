use super::*;

const EXPECTED_PUBLIC_KEY_BASE36: &'static str = "5m5srup6m3b2iilrsqmxu6ydp8p8cr0rdbh4wamupk3s4sxqr5";
const EXPECTED_SIGNATURE_BASE64: &'static str = "Oenv1TK5Ke5OUsrZ/d3RzGldXlx6/w0yb6eW+iuDkjMohwfvT3Areqk18yRthJjx60/DUrCkRwJs0X8/z30EDw==";

#[test]
fn verify_public_key_base36() {
    let path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let hpos_config_path = format!("{}/resources/test/hpos-config.json", path);
    env::set_var("HPOS_CONFIG_PATH", &hpos_config_path);

    let keypair = read_keypair();
    let public_key = keypair.public;
    let public_key_base36 = public_key::to_base36_id(&public_key);

    assert_eq!(public_key_base36, EXPECTED_PUBLIC_KEY_BASE36);
}

#[test]
fn verify_signature() {
    let path = env::var("CARGO_MANIFEST_DIR").unwrap();
    let hpos_config_path = format!("{}/resources/test/hpos-config.json", path);
    env::set_var("HPOS_CONFIG_PATH", &hpos_config_path);

    let keypair = read_keypair();

    let payload = Payload {
        host: read_keypair().public,
        hhaids: vec!["QmHHAid_1".to_string(), "QmHHAid_2".to_string()],
    };
    let payload_json = serde_json::to_string(&payload).unwrap();
    
    let signature = keypair.sign(payload_json.as_bytes());
    let signature_base64 = base64::encode(&signature.to_bytes()[..]);

    assert_eq!(signature_base64, EXPECTED_SIGNATURE_BASE64);
}