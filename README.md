# MPVSS - A Simply Publicly Verifiable Secret Sharing Library

![Rust](https://github.com/AlexiaChen/mpvss-rs/workflows/Rust/badge.svg?branch=master)

The library implements a simple PVSS scheme in Rust.

## What is PVSS?

Secret sharing means a dealer can break a secret into secret shares among a group of participants which can reconstruct the secret only by collaboratively joining their parts of the secret. The library also implements threshold cryptography so that the dealer can decide whether all of the receiving participants need to collaborate or if a smaller subgroup of participants is sufficient to reconstruct the secret.

In addition to the plain secret sharing scheme PVSS adds verifiability in the following way: All the parts the secret is split into are encrypted with the receivers' public keys respectively. The dealer publishes all the encrypted shares along with a non-interactive zero-knowledge proof that allows everbody (not only the receiving participants) to verify that the decrypted shares indeed can be used to reconstruct the secret. The participants then decrypt all their shares and exchange them along with another non-interactive zero-knowledge proof that allows the receiving participant to verify that the share is actually the result of the decryption.

Thus PVSS can be used to share a secret among a group of participants so that either the secret can be reconstructed by the participants who all play fair or a participant that received a faked share can identify the malicious party.

## Build

```bash
cargo build
```

## Test

```bash
cargo test
```

### Usage

#### Initialization

At first we convert our secret message into a numeric value if necessary. When creating the dealer a PVSS instance is created as well which holds all the global parameters that every participant needs to know.

```rust

```

#### Distribution & Verification

The dealer splits the secret into shares, encrypts them and creates a proof so that everybody can verify that the shares (once decrypted) can be used to reconstruct the secret. The threshold determines how many shares are necessary for the reconstruction. The encrypted shares and the proof are then bundled together.

```rust
/
```

#### Exchange & Verification

The participants extract their shares from the distribution bundle and decrypt them. They bundle them together with a proof that allows the receiver to verify that the share is indeed the result of the decryption.

```rust

```

#### Reconstruction

Once a participant collected at least `threshold` shares the secret can be reconstructed.

```rust

```

## References:

- Berry Schoenmakers. [A Simple Publicly Verifiable Secret Sharing Scheme and its Application to Electronic Voting](https://www.win.tue.nl/~berry/papers/crypto99.pdf)

- Adi Shamir. [How to share a secret](http://users.cms.caltech.edu/~vidick/teaching/101_crypto/Shamir1979.pdf)

- Tal Rabin. [Verifiable Secret Sharing and Multiparty Protocols with Honest Majority](https://www.cs.umd.edu/users/gasarch/TOPICS/secretsharing/rabinVSS.pdf)

- Markus Stadler. [Publicly Verifiable Secret Sharing](https://link.springer.com/content/pdf/10.1007%2F3-540-68339-9_17.pdf)

- bitcoinwiki-org. [Publicly Verifiable Secret Sharing](https://en.bitcoinwiki.org/wiki/Publicly_Verifiable_Secret_Sharing)

- crypto-stackexchange. [What is a Pedersen commitment?](https://crypto.stackexchange.com/questions/64437/what-is-a-pedersen-commitment)

- Torben Pryds Pedersen. [Non-Interactive and Information-Theoretic Secure Verifiable Secret Sharing](https://link.springer.com/content/pdf/10.1007%2F3-540-46766-1_9.pdf)
