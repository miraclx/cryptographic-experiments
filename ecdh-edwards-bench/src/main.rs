use std::env;

use curve25519_dalek::edwards::EdwardsPoint;
use curve25519_dalek::edwards::{CompressedEdwardsY, EdwardsBasepointTable};
use curve25519_dalek::traits::BasepointTable;
use curve25519_dalek::Scalar;
use ed25519_dalek::SigningKey;

mod bench;

fn main() {
    let alice_sk = SigningKey::generate(&mut rand::thread_rng());
    let alice_pk = alice_sk.verifying_key();
    let alice_sk = alice_sk.to_scalar();
    let alice_pk = CompressedEdwardsY(alice_pk.to_bytes())
        .decompress()
        .unwrap();
    let alice_pk_table = EdwardsBasepointTable::create(&alice_pk);

    let bob_sk = SigningKey::generate(&mut rand::thread_rng());
    let bob_pk = bob_sk.verifying_key();
    let bob_sk = bob_sk.to_scalar();
    let bob_pk = CompressedEdwardsY(bob_pk.to_bytes()).decompress().unwrap();
    let bob_pk_table = EdwardsBasepointTable::create(&bob_pk);

    let samples = env::args()
        .nth(1)
        .map_or(1000, |s| s.parse().expect("samples must be a number"));

    bench::run!(
        samples: samples,
        "no table": || {
            assert!(no_table(&alice_sk, &bob_pk) == no_table(&bob_sk, &alice_pk));
        },
        "one off table": || {
            assert!(one_off_table(&alice_sk, &bob_pk) == one_off_table(&bob_sk, &alice_pk));
        },
        "reused table": || {
            assert!(
                reused_table(&alice_sk, &bob_pk_table)
                    == reused_table(&bob_sk, &alice_pk_table)
            );
        }
    );
}

pub fn no_table(sk: &Scalar, pk: &EdwardsPoint) -> [u8; 32] {
    (sk * pk).compress().to_bytes()
}

fn one_off_table(sk: &Scalar, pk: &EdwardsPoint) -> [u8; 32] {
    let pk = EdwardsBasepointTable::create(pk);

    (sk * &pk).compress().to_bytes()
}

fn reused_table(sk: &Scalar, pk: &EdwardsBasepointTable) -> [u8; 32] {
    (sk * pk).compress().to_bytes()
}
