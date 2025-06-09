use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use hmac::{Hmac, Mac};
use sha1::Sha1;

pub fn validate_twilio_signature(
    url: &str,
    form_params: &std::collections::HashMap<String, String>,
    signature_header: &str,
    auth_token: &str,
) -> bool {
    let mut keys: Vec<&String> = form_params.keys().collect();
    keys.sort();

    let mut data = url.to_string();
    for key in keys {
        data.push_str(key);
        data.push_str(&form_params[key]);
    }
    let mut mac = Hmac::<Sha1>::new_from_slice(auth_token.as_bytes()).expect("Invalid HMAC key");

    mac.update(data.as_bytes());

    let result = mac.finalize().into_bytes();
    let computed_signature = STANDARD.encode(result);

    computed_signature == signature_header
}
