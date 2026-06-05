use base64::{engine::general_purpose, Engine as _};
use rand::rngs::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};

use crate::wireguard::types::WireGuardKeyPair;

/// Generates a new WireGuard compatible X25519 keypair
pub fn generate_keypair() -> WireGuardKeyPair {
    let mut rng = OsRng;
    let secret = StaticSecret::random_from_rng(&mut rng);
    let public = PublicKey::from(&secret);

    WireGuardKeyPair {
        private_key: general_purpose::STANDARD.encode(secret.as_bytes()),
        public_key: general_purpose::STANDARD.encode(public.as_bytes()),
    }
}
