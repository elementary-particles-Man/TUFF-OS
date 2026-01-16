use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};
use aes::Aes256;

pub fn encrypt_block(key: &[u8; 32], block: &mut [u8; 16]) {
    let cipher = Aes256::new(GenericArray::from_slice(key));
    let mut block_arr = GenericArray::from_mut_slice(block);
    cipher.encrypt_block(&mut block_arr);
}
