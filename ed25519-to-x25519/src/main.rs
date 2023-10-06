use curve25519_dalek::scalar::clamp_integer;
use ed25519_dalek::SigningKey;
use sha2::{Digest, Sha512};
use x25519_dalek::{PublicKey, StaticSecret};

mod hex {
    pub use hex_literal::hex as decode;
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }
}

fn unreduced_scalar(sk: &SigningKey) -> [u8; 32] {
    let sk: [u8; 32] = Sha512::digest(&sk.to_bytes())[0..32].try_into().unwrap();
    clamp_integer(sk)
}

fn main() {
    println!("alice");
    let alice_ed25519_sk = SigningKey::from_bytes(&hex::decode!(
        "cef91d5a6da1af98d1b64b7d6c52d04eed16b69dff3684a792723e470c01c546"
    ));
    println!(
        "  {:<25} : {}",
        "ed25519 sk",
        hex::encode(&alice_ed25519_sk.to_bytes())
    );
    let alice_ed25519_pk = alice_ed25519_sk.verifying_key();
    println!(
        "  {:<25} : {}",
        "ed25519 pk",
        hex::encode(&alice_ed25519_pk.to_bytes())
    );
    let alice_x25519_reduced_sk = alice_ed25519_sk.to_scalar();
    println!(
        "  {:<25} : {}",
        "x25519 sk (reduced)",
        hex::encode(&alice_x25519_reduced_sk.to_bytes())
    );
    let alice_x25519_unreduced_sk = StaticSecret::from(unreduced_scalar(&alice_ed25519_sk));
    println!(
        "  {:<25} : {}",
        "x25519 sk (unreduced)",
        hex::encode(&alice_x25519_unreduced_sk.to_bytes())
    );
    let alice_x25519_pk_via_ed25519 = alice_ed25519_pk.to_montgomery();
    println!(
        "  {:<25} : {}",
        "x25519 pk (via ed25519)",
        hex::encode(&alice_x25519_pk_via_ed25519.to_bytes())
    );
    let alice_x25519_pk_via_x25519 = PublicKey::from(&alice_x25519_unreduced_sk);
    println!(
        "  {:<25} : {}",
        "x25519 pk (via x22519)",
        hex::encode(&alice_x25519_pk_via_x25519.to_bytes())
    );

    println!("bob");

    let bob_ed25519_sk = SigningKey::from_bytes(&hex::decode!(
        "f6956818fbb50e2b09c257c2013d479f95a4c36afa40f75de6604041ec21d31f"
    ));
    println!(
        "  {:<25} : {}",
        "ed25519 sk",
        hex::encode(&bob_ed25519_sk.to_bytes())
    );
    let bob_ed25519_pk = bob_ed25519_sk.verifying_key();
    println!(
        "  {:<25} : {}",
        "ed25519 pk",
        hex::encode(&bob_ed25519_pk.to_bytes())
    );
    let bob_x25519_reduced_sk = bob_ed25519_sk.to_scalar();
    println!(
        "  {:<25} : {}",
        "x25519 sk (reduced)",
        hex::encode(&bob_x25519_reduced_sk.to_bytes())
    );
    let bob_x25519_unreduced_sk = StaticSecret::from(unreduced_scalar(&bob_ed25519_sk));
    println!(
        "  {:<25} : {}",
        "x25519 sk (unreduced)",
        hex::encode(&bob_x25519_unreduced_sk.to_bytes())
    );
    let bob_x25519_pk_via_ed25519 = bob_ed25519_pk.to_montgomery();
    println!(
        "  {:<25} : {}",
        "x25519 pk (via ed25519)",
        hex::encode(&bob_x25519_pk_via_ed25519.to_bytes())
    );
    let bob_x25519_pk_via_x25519 = PublicKey::from(&bob_x25519_unreduced_sk);
    println!(
        "  {:<25} : {}",
        "x25519 pk (via x22519)",
        hex::encode(&bob_x25519_pk_via_x25519.to_bytes())
    );

    println!("shares");

    let alice_shared_reduced = alice_x25519_reduced_sk * bob_x25519_pk_via_ed25519;
    println!(
        "  {:<25} : {}",
        "x25519 shared (reduced)",
        hex::encode(&alice_shared_reduced.to_bytes())
    );
    let alice_shared_unreduced =
        alice_x25519_unreduced_sk.diffie_hellman(&bob_x25519_pk_via_x25519);
    println!(
        "  {:<25} : {}",
        "x25519 shared (unreduced)",
        hex::encode(&alice_shared_unreduced.to_bytes())
    );
    let bob_shared_reduced = bob_x25519_reduced_sk * alice_x25519_pk_via_ed25519;
    println!(
        "  {:<25} : {}",
        "x25519 shared (reduced)",
        hex::encode(&bob_shared_reduced.to_bytes())
    );
    let bob_shared_unreduced = bob_x25519_unreduced_sk.diffie_hellman(&alice_x25519_pk_via_x25519);
    println!(
        "  {:<25} : {}",
        "x25519 shared (unreduced)",
        hex::encode(&bob_shared_unreduced.to_bytes())
    );
}
