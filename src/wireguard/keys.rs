use rand::rngs::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};

use crate::wireguard::types::WireGuardKeyPair;

/// Generates a new WireGuard compatible X25519 keypair
pub fn generate_keypair() -> WireGuardKeyPair {
    let mut rng = OsRng;
    let secret = StaticSecret::random_from_rng(&mut rng);
    let public = PublicKey::from(&secret);

    WireGuardKeyPair {
        private_key: base64::encode(secret.as_bytes()),
        public_key: base64::encode(public.as_bytes()),
    }
}
