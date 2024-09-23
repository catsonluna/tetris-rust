use rsa::{pkcs1::DecodeRsaPrivateKey, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use pem::parse;

fn get_private_key() -> RsaPrivateKey {

    let pem_key = r#"
    -----BEGIN RSA PRIVATE KEY-----
MIICXwIBAAKBgQDZbpnEtnBqSvw1SJWTnk6+tFiZ71cDHR1uFBBM2iSQ+2hz03Ja
gdfysk49/HvHDhhVM/cPLEacOGkprULTFmiZidvbAZrzqhKo5iwFpodicXhfSTll
CMGa2Wh6w9try0+wlGNK5a0hBJ9wIqYJ+R9rl9c5M5BgXK50zW5DGm+PRQIDAQAB
AoGBANLfeTJW3d+4qbHjm213c1dxAR0KCpFyg0BnJQfj6OLTcRIkWBt/Iji2xTtI
y6LaAK9hnpKQlkqcSyGob87ZRMcaUpsmybUP7bYs/EncZNkXcR+n0RjIrF69d12P
njY3LH0Ogr4uyncteo3mENHhcaklzNMqc2ZXU55k3In1HXJhAkEA7JPxz9GhbIT/
iQ9duyacGmsA39sY+63+VJRVMFRrRT1/G5pNjajKW0IilS85sNMJXDVmkKokXu+o
YlXYDSNUxwJBAOtIRojckv/GBk/b5TO4dawbrcT4moDcKRgOdJ6pKmPXRnfZ+9K/
AYhUlusjInAsopTcl0mFysk3YXDiKXE4F5MCQQCn5as0kRLeAFmWPPiJbLJWsWAS
ftzxKR+ZiRgYT6E9p5JA1bZ6dbL/JyWB+N0Zl/xJm4Bp5MYhNjaraz8Eu/FbAkEA
2Un1gj4bMEZE3Anb4mkvm5QwZmGl5u3ssb/f6guXD3/oH5XcYKRwGOTuQBhBnNFM
ESisO4VpY21W+zaTSXGhKwJBAL9jyQ5GQ5DZ2VdcSg2wf9Is1IrQjULZqJrMeh6/
Z5f2LmZbByUzCrOg1itvw3Rlreld3S3wNkp68K3z4xppLfg=
-----END RSA PRIVATE KEY-----
    "#;

    // Parse the PEM string
    let pem = parse(pem_key).expect("Failed to parse PEM key");


    // Decode the PEM contents into an RSA Private Key
    let priv_key = RsaPrivateKey::from_pkcs1_der(&pem.contents()).expect("Failed to decode PEM key");

    priv_key
}

fn get_public_key() -> RsaPublicKey {

    let pub_key = RsaPublicKey::from(&get_private_key());

    pub_key
}

pub fn encrypt(data: &[u8]) -> Vec<u8> {
    let pub_key = get_public_key();

    use rand::rngs::OsRng;
    
    let mut rng = OsRng;
    let enc_data = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, data)
        .expect("failed to encrypt");

    enc_data
}

pub fn decrypt(data: &[u8]) -> Vec<u8> {
    let priv_key = get_private_key();

    let dec_data = priv_key
        .decrypt(Pkcs1v15Encrypt, data)
        .expect("failed to decrypt");

    dec_data
}

pub fn base64_to_bytes(data: &str) -> Vec<u8> {
    base64::decode(data).expect("failed to decode base64")
}