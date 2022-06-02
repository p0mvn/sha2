
// The resulting size of the message digest in bits
const MESSAGE_DIGEST_BITS: u16 = 256;
// A group of 4 byte chunks
const WORD_SIZE_BITS: u8 = 32;

const BLOCK_SIZE_BITS: u16 = 512;

// The rotate left (circular left shift) operation, where x is a w-bit word and n
// is an integer with 0 <= n < w, is defined by ROTL (n, x) = (x << n) or (x >> w - n
fn rotl() {

}

// The rotate right (circular right shift) operation, where x is a w-bit word
// and n is an integer with 0 <= n < w, is defined by ROTR (n, x) =  (x >> n) or(x << w - n).
fn rotr() {
    
}

// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf
// iterative, one-way hash function that can process a message to produce a
// condensed representation called a message digest
fn sha2() -> String {

    // preprocessing
       // * padding the message
       // * parsing the padded message into m-bit blocks
       // * setting initialization values to be used in the hash computation
    // hash computation - generates a message schedule



    return String::from("");
}

#[cfg(test)]
mod tests {

    use sha2::{Sha256, Digest};
    use hex_literal::hex;

    #[test]
    fn sha2_256_works() {
        let mut hasher = Sha256::new();
        // write input message
        hasher.update(b"hello world");
        // read hash digest and consume hasher
        let result = hasher.finalize();

        assert_eq!(result[..], hex!("
        b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
    ")[..]);
    }
}
