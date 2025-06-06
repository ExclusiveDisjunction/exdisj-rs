use aes_gcm::{aead::{Aead, OsRng}, AeadCore, Aes256Gcm, Error as AesError, Key, KeyInit};
use sha2::digest::consts::U12;

/// A structure used to encrypt and decrypt binary data with AES. 
pub struct AesDuplex {
    key: Key<Aes256Gcm>,
    cipher: Aes256Gcm
}
impl Default for AesDuplex {
    fn default() -> Self {
        Self::new(&mut OsRng)
    }
}
impl AesDuplex {
    /// Creates a new instance using a specified random number generator 
    pub fn new<T>(rng: &mut T) -> Self where T: rsa_ext::rand_core::RngCore + rsa_ext::rand_core::CryptoRng {
        let key = Aes256Gcm::generate_key(rng);
        let cipher = Aes256Gcm::new(&key);

        Self {
            key, 
            cipher
        }
    }

    /// Generates a nonce to use for message verification. 
    pub fn make_nonce() -> aes_gcm::Nonce<U12> {
        Aes256Gcm::generate_nonce(&mut OsRng)
    }
    /// Generates a nonce to use for message verification, using a specified 
    pub fn make_nonce_using<T>(rng: &mut T) -> aes_gcm::Nonce<U12> where T: rsa_ext::rand_core::RngCore + rsa_ext::rand_core::CryptoRng {
        Aes256Gcm::generate_nonce(rng)
    }

    /// Obtains the internally used key. 
    pub fn key(&self) -> &Key<Aes256Gcm> {
        &self.key
    }

    /// Encrypts a data set using a specific Nonce. Use `Self::make_nonce()` to generate one for you.
    pub fn encrypt(&self, nonce: &aes_gcm::Nonce<U12>, data: &[u8]) -> Result<Vec<u8>, AesError> {
        self.cipher.encrypt(nonce, data)
    }
    /// Decrypts a data set using the internal key and a nonce. Ensure this nonce is the same as the one provided by the encryptor. 
    pub fn decrypt(&self, nonce: &aes_gcm::Nonce<U12>, data: &[u8]) -> Result<Vec<u8>, AesError> {
        self.cipher.decrypt(nonce, data)
    }
}