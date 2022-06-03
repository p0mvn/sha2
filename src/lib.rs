
// The resulting size of the message digest in bits
const MESSAGE_DIGEST_BITS: u16 = 256;
// A group of 4 bytes chunks.
// Binary example: 1010 0001 0000 0011 1111 1110 0010 0011
// Hex example:  a103fe23
// Words are in Big Endian.
const WORD_SIZE_BITS: u8 = 32;
// 16 32-bit words
const BLOCK_SIZE_BITS: u16 = 512;

type Word = [u8; WORD_SIZE_BITS as usize / 8];

use std::slice;

// SHA-256 Constants
// sequence of sixty-four constant 32-bit words, These words represent
// the first thirty-two bits of the fractional parts of the cube roots
//  of the first sixty-four prime numbers. They surve as the "nothing up
// my sleeve values" for the choice of IV.
fn constants_256() -> [u32; 64] {
    [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ]
}

// *****************************************************************
// The following are utility functions  to perform some common
// bitwise operations.

// The rotate left (circular left shift) operation, where x is a w-bit word and n
// is an integer with 0 <= n < w, is defined by ROTL (n, x) = (x << n) or (x >> w - n
fn rotl() {

}

// The rotate right (circular right shift) operation, where x is a w-bit word
// and n is an integer with 0 <= n < w, is defined by ROTR (n, x) =  (x >> n) or(x << w - n).
fn rotr() {
    
}

// The right shift operation, where x is a w-bit word and n is an integer with 0 <= n < w,
// is defined by SHR (n, x) = x >> n.
fn shr() {
    
}

// *****************************************************************
// SHA-256 has six logical function that are defined below.
// each function operates on 32-bit words, which are represented 
// as x, y, and z. The result of each function is a new 32-bit word

// Ch(x, y, z) = (x and y) xor (not x and z)
fn Ch() {

}

// Maj(x, y, z) = (x and y) xor (x and z) xor (y and z) 
fn Maj() {

}

// CapSigmaZero{256}(x) = ROTR(2, x) xor ROTR (13, x) xor ROTR (22, x)
fn CapSigmaZero() {

}

// CapSigmaZero{256}(x) = ROTR(2, x) xor ROTR (13, x) xor ROTR (22, x)
fn CapSigmaOne() {

}

fn SigmaZero() {

}

fn SigmaOne() {
    
}

// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
// iterative, one-way hash function that can process a message to produce a
// condensed representation called a message digest
fn sha2(data: &[u8]) -> String {

    // Copy value for convenience
    let mut dataVec = vec![data];

    // preprocessing
       // * padding the message
       // * parsing the padded message into m-bit blocks
       // * setting initialization values to be used in the hash computation
    // hash computation - generates a message schedule



    return String::from("");
}

// Custom Helpers
///////////////////////////////////////////////////////////////////////////////

// pad pads the message to be a multiple of  BLOCK_SIZE_BITS.
fn pad(mut data: Vec<u8>) {
    // Suppose the length of the message M, is l bits. Append the bit "1" to
    // the end of the message, followed by k zero bits, where k is the smallest,
    // non-negative solution to the equation l + 1 + k === 448 mod 512. Then,
    // append the 64-bit block that is equal to the number l expressed using a
    // binary representation. For example, the (8-bit ASCII) message "abc" has
    // length 8 x 3 = 24, so the message is padded with a one bit, then
    // 448 - (24+1) = 423 zero bits, and then the message length, to become the
    // 512-bit padded message.
    //
    //                                   423        64
    // 01100001  01100010  01100011  1  00..00  00..011000
    //    "a"       "b"        "c"                  l = 24
    // The length of the padded message should now be a multiple of 512 bits.



}

#[cfg(test)]
mod tests {
    use super::*;

    use sha2::{Sha256, Digest};
    use hex_literal::hex;

    #[test]
    fn sha2_256_works() {
        let mut hasher = Sha256::new();

        let data = b"hello world";

        // write input message
        hasher.update(data);
        // read hash digest and consume hasher
        let result = hasher.finalize();

        assert_eq!(result[..], hex!("
        b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
    ")[..]);

        sha2(data);
    }
}
