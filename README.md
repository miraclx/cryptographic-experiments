<!-- markdownlint-disable MD024 -->

# cryptographic experiments

Repo hosting a bunch of cryptographic experiments.

## Experiments

### Using `ed25519` for ECDH

Benchmarks the ECDH operation on the Edwards curve `ed25519`.

One implementation with variable basepoint.
One implementation computing multiples of basepoint at each iteration.
One implementation computing multiples of basepoint in advance.

#### Results

MacBook Pro (16-inch, 2021) Apple M1 Pro 10-Core CPU 32GB RAM

```console
$ cargo run --bin ecdh-edwards-bench
bench: no table
  Time (mean):            80.87µs
  Range (min … max):      67.33µs … 161.96µs     [1000 samples]
  Total: 80.87ms
bench: one off table
  Time (mean):             1.82ms
  Range (min … max):       1.78ms … 1.97ms       [1000 samples]
  Total: 1.82s
bench: reused table
  Time (mean):            24.85µs
  Range (min … max):      23.54µs … 44.00µs      [1000 samples]
  Total: 24.85ms
Summary
  reused table ran
    3.25 times faster than no table
    73.07 times faster than one off table
```

### Converting `ed25519` to `x25519` for ECDH

`ed25519-dalek` provides methods for converting between Edwards and Montgomery points and scalars.

I made the following observations:

1. `ed25519_dalek::SigningKey::to_scalar` computes `reduce(clamp(sha512(ed25519_sk)[:32]))`. Which results in a valid `x25519` scalar.
2. `ed25519_dalek::VerifyingKey::to_montgomery` computes `(1+y)/(1-y)` where `y` is the `ed25519` point. Which results in a valid `x25519` point.
3. Writing the `x25519` scalar to a file, prepended with the X25519 [PKCS8](https://datatracker.ietf.org/doc/html/rfc5208#section-5) header in DER format ([ed25519_pkcs8_der_head]), and using `openssl pkey -inform der -outform der -pubout | tail -c 32` results in a `x25519` point that does not match the one computed by `ed25519_dalek::VerifyingKey::to_montgomery`.
4. The product of `Scalar` & `MontgometryPoint` for ECDH results in a valid key share.
5. Constructing a `x25519::StaticSecret` with the `ed25519` scalar has confusing results - the public key is not the same as any we have observed so far.
6. Looking at the logic in `curve25519_dalek` and [`libsodium`](https://github.com/jedisct1/libsodium/blob/7e3500e878ee5a3a286705ea646a535b33a29cd3/src/libsodium/crypto_sign/ed25519/ref10/keypair.c#L70), there seems to be a difference in the way the `x25519` scalar is computed.
    - `libsodium` computes `clamp(sha512(ed25519_sk)[:32])` without the `reduce` step.
7. Extracting out the logic for scalar computation, and doing it manually, we get more meaningful results.
8. The `PublicKey` computed from `x25519::StaticSecret` matches the one `MontgometryPoint` computed by `ed25519_dalek::VerifyingKey::to_montgomery` despite the `StaticSecret` being constructed with a different scalar.
9. The product of this `x25519::StaticSecret` & `x25519::PublicKey` results in the same key share as the one computed in the use of `Scalar` & `MontgometryPoint`, despite the `StaticSecret` being constructed with a different scalar.
10. Reattempting the `openssl pkey` command with our manually computed scalar results in a valid `x25519` point that matches the one computed by `ed25519_dalek::VerifyingKey::to_montgomery`.

Conclusion

1. You can't use the `Scalar` output by `ed25519_dalek::SigningKey::to_scalar` as a `x25519::StaticSecret` directly (because it's a reduced scalar).
2. There are two valid `x25519` scalars for a given `ed25519` scalar - perhaps I missed something, but things seem to work out.

#### Results

```console
alice
  ed25519 sk                : cef91d5a6da1af98d1b64b7d6c52d04eed16b69dff3684a792723e470c01c546
  ed25519 pk                : 06466a00356bb3caf16bd03a8140efd0c7a9ddaa9c8b9cc84e164d94948e1443
  x25519 sk (reduced)       : 3208a428ee01bc63a373026b0721bd40c998ce7ad11ca4663b2c9458d2856207
  x25519 sk (unreduced)     : c0ff66568c542a74a920d03c3ffcf6bdc998ce7ad11ca4663b2c9458d2856267
  x25519 pk (via ed25519)   : 4fd1df9c403a38c5db85e3c8e94f6b9d9b834ee2fbb5ed37d02cd4eb0fd1961a
  x25519 pk (via x22519)    : 4fd1df9c403a38c5db85e3c8e94f6b9d9b834ee2fbb5ed37d02cd4eb0fd1961a
bob
  ed25519 sk                : f6956818fbb50e2b09c257c2013d479f95a4c36afa40f75de6604041ec21d31f
  ed25519 pk                : bf0f623fb7939124f589dce40dab10feab0abb9d88e92645cda2a43cba408685
  x25519 sk (reduced)       : 72f1ddd16cf0460293179602eaf509341ff2a84e31c75b20d1133f530c305f05
  x25519 sk (unreduced)     : 00e9a0ff0a43b51299c463d421d143b11ff2a84e31c75b20d1133f530c305f65
  x25519 pk (via ed25519)   : a719b1780b26e362a0ae1d5fd069a19ceb9b40dc8abf68ad703bd748415a483a
  x25519 pk (via x22519)    : a719b1780b26e362a0ae1d5fd069a19ceb9b40dc8abf68ad703bd748415a483a
shares
  x25519 shared (reduced)   : 041c044e74ac20b940d4a22e08a9518f8d6c9781fa78906f26f19bfd3b275b5f
  x25519 shared (unreduced) : 041c044e74ac20b940d4a22e08a9518f8d6c9781fa78906f26f19bfd3b275b5f
  x25519 shared (reduced)   : 041c044e74ac20b940d4a22e08a9518f8d6c9781fa78906f26f19bfd3b275b5f
  x25519 shared (unreduced) : 041c044e74ac20b940d4a22e08a9518f8d6c9781fa78906f26f19bfd3b275b5f
```

- The public keys generated from different scalars are the same.
- All the shares check out, despite the `x25519` scalars being different.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as below, without any additional terms or conditions.

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
