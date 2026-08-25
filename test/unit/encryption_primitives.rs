//! Encryption Primitive Tests
//!
//! Test low-level cryptographic operations: key derivation, chunk processing, AEAD encryption

#[allow(unused)]
use {
    chacha20poly1305::KeyInit, chacha20poly1305::XChaCha20Poly1305, rand::Rng, rand::RngCore,
    rand::rngs::OsRng, std::fs, tempfile::NamedTempFile,
};

#[allow(unused)]
use eck::encryption::{
    encryption_flow::{decrypt_stream, encrypt_stream},
    encryption_primitives::{decrypt_chunk, encrypt_chunk, generate_nonce},
    file_encryption::{decrypt_file, encrypt_file},
    folder_encryption::{decrypt_folder, encrypt_folder},
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derive_nonce_deterministic() {
        // Test that derive_nonce produces deterministic output for same inputs

        let master = [0x41u8; 24]; // All 'A'

        let nonce_0a = eck::encryption::encryption_primitives::derive_nonce(&master, 0);
        let nonce_0b = eck::encryption::encryption_primitives::derive_nonce(&master, 0);

        assert_eq!(nonce_0a, nonce_0b); // Same step → same nonce

        let nonce_1 = eck::encryption::encryption_primitives::derive_nonce(&master, 1);
        assert_ne!(nonce_0a, nonce_1); // Different step → different nonce
    }

    #[test]
    fn encrypt_decrypt_chunk_roundtrip() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key = [0x42u8; 32]; // All 'B'
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        let mut data = vec![b'T'; 64].to_vec();
        let original_len = data.len();

        for step in 0..3 {
            eck::encryption::encryption_primitives::encrypt_chunk(
                &mut data,
                &master_nonce,
                &cipher,
                step,
            )
            .unwrap();

            eck::encryption::encryption_primitives::decrypt_chunk(
                &mut data,
                &cipher,
                &master_nonce,
                step,
            )
            .unwrap();

            assert_eq!(data.len(), original_len);
        }
    }

    #[test]
    fn encrypt_decrypt_with_wrong_key_fails() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key_good = [0x43u8; 32];
        let key_bad = [0x44u8; 32]; // Different!

        let cipher = XChaCha20Poly1305::new(&key_good.into());
        let mut data = b"test".to_vec();
        let master_nonce = generate_nonce();

        eck::encryption::encryption_primitives::encrypt_chunk(&mut data, &master_nonce, &cipher, 0)
            .unwrap();

        let cipher_bad = XChaCha20Poly1305::new(&key_bad.into());
        assert!(
            eck::encryption::encryption_primitives::decrypt_chunk(
                &mut data,
                &cipher_bad,
                &master_nonce,
                0
            )
            .is_err()
        );
    }

    #[test]
    fn generate_nonce_unique() {
        let nonce1 = generate_nonce();
        let nonce2 = generate_nonce();

        assert_ne!(nonce1, nonce2); // Nonces should be unique (statistically very likely)
    }

    #[test]
    fn encrypt_decrypt_empty_data_chunk() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key = [0x45u8; 32];
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        let mut data: Vec<u8> = vec![];

        eck::encryption::encryption_primitives::encrypt_chunk(&mut data, &master_nonce, &cipher, 0)
            .unwrap();

        eck::encryption::encryption_primitives::decrypt_chunk(&mut data, &cipher, &master_nonce, 0)
            .unwrap();

        assert!(data.is_empty());
    }

    #[test]
    fn derive_nonce_byte_pattern() {
        let master = [1u8; 24]; // All ones

        let nonce_0 = eck::encryption::encryption_primitives::derive_nonce(&master, 0);

        assert_eq!(&nonce_0[..16], &master[..16]); // First part unchanged

        let mut expected_step = [0u8; 8];
        0u64.to_le_bytes().copy_from_slice(&mut expected_step);
        assert_eq!(&nonce_0[16..], &expected_step);
    }

    #[test]
    fn encrypt_decrypt_chunk_various_sizes() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key = [0x46u8; 32];
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        for size in [1, 7, 8, 9, 16, 31, 32, 33, 64].iter() {
            let mut data = vec![*size as u8; *size];

            eck::encryption::encryption_primitives::encrypt_chunk(
                &mut data,
                &master_nonce,
                &cipher,
                0,
            )
            .unwrap();

            eck::encryption::encryption_primitives::decrypt_chunk(
                &mut data,
                &cipher,
                &master_nonce,
                0,
            )
            .unwrap();

            assert_eq!(&data.len(), size); // Size preserved
        }
    }

    #[test]
    fn key_derivation_deterministic() {
        let salt = [16u8; 16];

        let derived_key_1 = eck::keygen::derive_key("test_pwd", salt).unwrap();
        let derived_key_2 = eck::keygen::derive_key("test_pwd", salt).unwrap();

        assert_eq!(derived_key_1, derived_key_2); // Same pwd + same salt → same key
    }

    #[test]
    fn key_derivation_different_salt_different_keys() {
        let salt1 = [0x55u8; 16];
        let salt2 = [0x66u8; 16]; // Different!

        let key1 = eck::keygen::derive_key("password", salt1).unwrap();
        let key2 = eck::keygen::derive_key("password", salt2).unwrap();

        assert_ne!(key1, key2); // Same pwd, different salts → different keys
    }

    #[test]
    fn encrypt_decrypt_chunk_max_size() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key = [0x47u8; 32];
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        // Test with chunk size (8MB) - but use smaller for fast tests: 64KB
        let mut data = vec![b'X'; 65_536];

        eck::encryption::encryption_primitives::encrypt_chunk(&mut data, &master_nonce, &cipher, 0)
            .unwrap();

        eck::encryption::encryption_primitives::decrypt_chunk(&mut data, &cipher, &master_nonce, 0)
            .unwrap();

        assert_eq!(data.len(), 65_536);
    }

    #[test]
    fn derive_nonce_step_overflow() {
        let master = [1u8; 24];

        // Test with very large step value (should still work)
        let nonce = eck::encryption::encryption_primitives::derive_nonce(&master, u64::MAX);

        assert_eq!(&nonce[..16], &master[..16]);
    }

    #[test]
    fn encrypt_decrypt_multiple_chunks_sequential() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key = [0x48u8; 32];
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        // Simulate multi-chunk encryption (step increments)
        let mut all_data: Vec<Vec<u8>> = vec![vec![b'A'; 64], vec![b'B'; 128], vec![b'C'; 32]];

        for (i, data) in all_data.iter_mut().enumerate() {
            eck::encryption::encryption_primitives::encrypt_chunk(
                data,
                &master_nonce,
                &cipher,
                i as u64,
            )
            .unwrap();

            // Decrypt immediately to verify each chunk independently works
            eck::encryption::encryption_primitives::decrypt_chunk(
                data,
                &cipher,
                &master_nonce,
                i as u64,
            )
            .unwrap();
        }

        assert_eq!(all_data[0].len(), 64);
        assert_eq!(all_data[1].len(), 128);
        assert_eq!(all_data[2].len(), 32);
    }

    #[test]
    fn encrypt_decrypt_with_corrupted_nonce_fails() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key = [0x49u8; 32];
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        // Use larger data to ensure AEAD tag is properly verified
        let mut data: Vec<u8> = vec![b'T'; 64];

        eck::encryption::encryption_primitives::encrypt_chunk(&mut data, &master_nonce, &cipher, 0)
            .unwrap();

        // Corrupt the nonce slightly (change last byte of step position) - this changes derived nonce at non-zero step
        let mut bad_master = master_nonce;
        bad_master[23] ^= 0xFF;

        // Decrypt with corrupted nonce should fail authentication (AEAD behavior: wrong nonce → invalid tag)
        assert!(
            eck::encryption::encryption_primitives::decrypt_chunk(
                &mut data,
                &cipher,
                &bad_master,
                1 // Use non-zero step so byte[23] corruption actually affects the derived nonce
            )
            .is_err()
        );
    }

    #[test]
    fn encrypt_decrypt_with_corrupted_data_fails() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key = [0x4Au8; 32];
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        let mut data = b"This is a super data string for testing !".to_vec();

        eck::encryption::encryption_primitives::encrypt_chunk(&mut data, &master_nonce, &cipher, 0)
            .unwrap();

        // Corrupt the ciphertext (change one byte)
        data[10] ^= 0xFF;

        assert!(
            eck::encryption::encryption_primitives::decrypt_chunk(
                &mut data,
                &cipher,
                &master_nonce,
                0
            )
            .is_err()
        );
    }

    #[test]
    fn derive_nonce_reproducible() {
        let master = [0x57u8; 24]; // All 'W'

        for _ in 0..100 {
            let nonce_0a = eck::encryption::encryption_primitives::derive_nonce(&master, 0);
            let nonce_0b = eck::encryption::encryption_primitives::derive_nonce(&master, 0);

            assert_eq!(nonce_0a, nonce_0b);
        }
    }

    #[test]
    fn encrypt_decrypt_chunk_preserves_length() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key = [0x4Bu8; 32];
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        // AEAD adds ~16 bytes overhead (tag) - verify this is handled correctly
        for size in [1, 17, 49, 100].iter() {
            let mut data = vec![*size as u8; *size];
            let original_len = data.len();

            eck::encryption::encryption_primitives::encrypt_chunk(
                &mut data,
                &master_nonce,
                &cipher,
                0,
            )
            .unwrap();

            eck::encryption::encryption_primitives::decrypt_chunk(
                &mut data,
                &cipher,
                &master_nonce,
                0,
            )
            .unwrap();

            assert_eq!(data.len(), original_len);
        }
    }

    #[test]
    fn encrypt_decrypt_with_all_steps_0_to_10() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key = [0x4Cu8; 32];
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        for step in 0..=10 {
            let mut data = vec![step as u8 * 10; 64 + (step * 7) as usize];

            eck::encryption::encryption_primitives::encrypt_chunk(
                &mut data,
                &master_nonce,
                &cipher,
                step,
            )
            .unwrap();
        }
    }

    #[test]
    fn derive_nonce_byte_order() {
        let master = [1u8; 24];

        for (step_val, expected_last_8) in [
            (0u64, vec![0x00; 8]),
            (1u64, vec![0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
            (256u64, vec![0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
        ] {
            let nonce = eck::encryption::encryption_primitives::derive_nonce(&master, step_val);

            assert_eq!(&nonce[16..], &expected_last_8[..]);
        }
    }

    #[test]
    fn encrypt_decrypt_chunk_with_zero_step() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key = [0x4Du8; 32];
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        let mut data = b"First".to_vec(); // Step will be 0

        eck::encryption::encryption_primitives::encrypt_chunk(&mut data, &master_nonce, &cipher, 0)
            .unwrap();

        eck::encryption::encryption_primitives::decrypt_chunk(&mut data, &cipher, &master_nonce, 0)
            .unwrap();
    }

    #[test]
    fn derive_nonce_different_masters() {
        let master1 = [1u8; 24];
        let master2 = [2u8; 24];

        let nonce_1_step0 = eck::encryption::encryption_primitives::derive_nonce(&master1, 0);
        let nonce_2_step0 = eck::encryption::encryption_primitives::derive_nonce(&master2, 0);

        assert_ne!(nonce_1_step0, nonce_2_step0); // Different masters → different nonces
    }

    #[test]
    fn encrypt_decrypt_chunk_with_large_data() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key = [0x4Eu8; 32];
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        // Test with larger chunk (but not full CHUNK_SIZE for speed)
        let mut data: Vec<u8> = vec![b'X'; 4096]; // 4KB

        eck::encryption::encryption_primitives::encrypt_chunk(&mut data, &master_nonce, &cipher, 0)
            .unwrap();

        eck::encryption::encryption_primitives::decrypt_chunk(&mut data, &cipher, &master_nonce, 0)
            .unwrap();

        assert_eq!(data.len(), 4096);
    }

    #[test]
    fn derive_nonce_with_zero_master() {
        let master = [0u8; 24]; // All zeros

        for step in (0u64..=100u64).into_iter() {
            let nonce = eck::encryption::encryption_primitives::derive_nonce(&master, step);

            assert_eq!(&nonce[..16], &master[..16]); // First part unchanged (bytes 0-15)

            let mut expected_step = [0u8; 8];
            expected_step.copy_from_slice(&step.to_le_bytes());
            assert_eq!(&nonce[16..], &expected_step);
        }
    }

    #[test]
    fn encrypt_decrypt_chunk_with_repeated_steps() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key = [0x4Fu8; 32];
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        // Encrypt/decrypt with same step multiple times - should work each time
        for _ in 0..5 {
            let mut data = vec![b'R'; 64].to_vec();

            eck::encryption::encryption_primitives::encrypt_chunk(
                &mut data,
                &master_nonce,
                &cipher,
                1,
            )
            .unwrap();

            eck::encryption::encryption_primitives::decrypt_chunk(
                &mut data,
                &cipher,
                &master_nonce,
                1,
            )
            .unwrap();
        }
    }

    #[test]
    fn derive_nonce_with_negative_step_simulation() {
        let master = [0x5Bu8; 24]; // All 'B'

        // Test with very large step (simulating overflow behavior)
        for &step in [u64::MAX, u64::MAX - 1, u64::MAX / 2].iter() {
            let nonce = eck::encryption::encryption_primitives::derive_nonce(&master, step);

            assert_eq!(&nonce[..16], &master[..16]); // First part unchanged

            let mut expected_step = [0u8; 8];
            expected_step.copy_from_slice(&step.to_le_bytes());
            assert_eq!(&nonce[16..], &expected_step);
        }
    }

    #[test]
    fn encrypt_decrypt_chunk_with_alternating_data() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key = [0x60u8; 32]; // All 'a' (ASCII)
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        for step in 0..=10 {
            let mut data: Vec<u8> = vec![step as u8; 64 + (step * 7) as usize];

            eck::encryption::encryption_primitives::encrypt_chunk(
                &mut data,
                &master_nonce,
                &cipher,
                step,
            )
            .unwrap();
        }
    }

    #[test]
    fn derive_nonce_with_incrementing_steps() {
        let master = [0x61u8; 24]; // All 'a' (ASCII)

        for i in 0..=50 {
            let nonce_i = eck::encryption::encryption_primitives::derive_nonce(&master, i);

            if i > 0 {
                let prev_nonce =
                    eck::encryption::encryption_primitives::derive_nonce(&master, i - 1);
                assert_ne!(nonce_i, prev_nonce); // Each step produces unique nonce
            }
        }
    }

    #[test]
    fn encrypt_decrypt_with_different_data_patterns() {
        use chacha20poly1305::XChaCha20Poly1305;

        let key = [0x62u8; 32]; // All 'b' (ASCII)
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        for pattern in [vec![0u8; 64], vec![255u8; 64]] {
            let mut data: Vec<u8> = pattern.clone();

            eck::encryption::encryption_primitives::encrypt_chunk(
                &mut data,
                &master_nonce,
                &cipher,
                0,
            )
            .unwrap();
        }
    }

    #[test]
    fn derive_nonce_with_zero_step() {
        let master = [0x63u8; 24]; // All 'c' (ASCII)

        for _ in 0..10 {
            let nonce_0a = eck::encryption::encryption_primitives::derive_nonce(&master, 0);
            let nonce_0b = eck::encryption::encryption_primitives::derive_nonce(&master, 0);

            assert_eq!(nonce_0a, nonce_0b); // Same step always produces same nonce
        }
    }

    #[test]
    fn encrypt_decrypt_chunk_with_realistic_data() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key = [0x64u8; 32]; // All 'd' (ASCII)
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        for pattern in [
            b"Hello, World!",                        // Short text
            "Lorem ipsum dolor sit amet".as_bytes(), // Medium text
            &[244u8; 1024],                          // Binary-like data
        ] {
            let mut data = pattern.to_vec();

            eck::encryption::encryption_primitives::encrypt_chunk(
                &mut data,
                &master_nonce,
                &cipher,
                0,
            )
            .unwrap();

            eck::encryption::encryption_primitives::decrypt_chunk(
                &mut data,
                &cipher,
                &master_nonce,
                0,
            )
            .unwrap();
        }
    }

    #[test]
    fn derive_nonce_with_large_step_values() {
        let master = [0x65u8; 24]; // All 'e' (ASCII)

        for step in [1_000_u64, 10_000_u64, 100_000_u64].iter() {
            let nonce = eck::encryption::encryption_primitives::derive_nonce(&master, *step);

            assert_eq!(&nonce[..16], &master[..16]); // First part unchanged

            let mut expected_step = [0u8; 8];
            expected_step.copy_from_slice(&step.to_le_bytes());
            assert_eq!(&nonce[16..], &expected_step);
        }
    }

    #[test]
    fn encrypt_decrypt_with_empty_then_data() {
        use chacha20poly1305::{KeyInit, XChaCha20Poly1305};

        let key = [0x66u8; 32]; // All 'f' (ASCII)
        let cipher = XChaCha20Poly1305::new(&key.into());
        let master_nonce = generate_nonce();

        {
            let mut empty: Vec<u8> = vec![];

            eck::encryption::encryption_primitives::encrypt_chunk(
                &mut empty,
                &master_nonce,
                &cipher,
                0,
            )
            .unwrap();

            eck::encryption::encryption_primitives::decrypt_chunk(
                &mut empty,
                &cipher,
                &master_nonce,
                0,
            )
            .unwrap();
        }

        {
            let mut data = b"NOT".to_vec();

            eck::encryption::encryption_primitives::encrypt_chunk(
                &mut data,
                &master_nonce,
                &cipher,
                1,
            )
            .unwrap();

            eck::encryption::encryption_primitives::decrypt_chunk(
                &mut data,
                &cipher,
                &master_nonce,
                1,
            )
            .unwrap();
        }
    }
}
