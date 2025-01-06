use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use rand::rngs::OsRng;
use sha2::{Digest, Sha512};

struct SchnorrSignature {
    r: RistrettoPoint,
    s: Scalar,
}

fn hash_message(message: &[u8], r: &RistrettoPoint) -> Scalar {
    let mut hasher = Sha512::new();
    hasher.update(message);
    hasher.update(r.compress().as_bytes());
    let hash_result = hasher.finalize();

    // Convert the first 64 bytes of the hash result into a Scalar
    let hash_bytes = hash_result.as_slice();
    Scalar::from_bytes_mod_order_wide(hash_bytes.try_into().unwrap())
}

fn keygen() -> (Scalar, RistrettoPoint) {
    let mut csprng = OsRng;
    let private_key = Scalar::random(&mut csprng);
    let public_key = private_key * RISTRETTO_BASEPOINT_POINT;
    (private_key, public_key)
}

fn sign(message: &[u8], private_key: Scalar) -> SchnorrSignature {
    let mut csprng = OsRng;
    let nonce = Scalar::random(&mut csprng);
    let r = nonce * RISTRETTO_BASEPOINT_POINT;
    let e = hash_message(message, &r);
    let s = nonce + e * private_key;
    SchnorrSignature { r, s }
}

fn verify(message: &[u8], signature: &SchnorrSignature, public_key: RistrettoPoint) -> bool {
    let e = hash_message(message, &signature.r);
    let r_prime = signature.s * RISTRETTO_BASEPOINT_POINT - e * public_key;
    r_prime == signature.r
}

fn main() {
    // Generate keys
    let (private_key, public_key) = keygen();

    // Sign a message
    let message = b"Hello, Schnorr!";
    let signature = sign(message, private_key);

    // Verify the signature
    let is_valid = verify(message, &signature, public_key);
    println!("Signature valid: {}", is_valid);
}
