

// make it functional girly pops
// here's chain iterators: take two iterators, create one big one
// and the one we want, zip iterator, which makes a pair of them, and tucks them into a tuple
// and then we collect it into a vector!
pub mod functions {

    use base64ct::{Base64, Encoding};
    use rust_decimal::{Decimal, prelude::FromPrimitive};
    use std::collections::{BTreeMap, HashMap};

pub fn fixed_xor (buffer_one: Vec<u8>, buffer_two: Vec<u8>) -> Vec<u8> {
    assert_eq!(buffer_one.len(), buffer_two.len());
    let result: Vec<u8> = buffer_one.iter().zip(buffer_two.iter()).map(|f| f.0 ^ f.1).collect();
    return result;
}
// if i wanted to do this manually i'd still do it the way i did when i was using python
// run through the string, from the right to the left, three bytes at a time, converting each three byte chunk
// into equivalent four 6 bit (48 bits) base 64
// using a lookup table.
// note to self: generating a lookup table is easy if you understand you're looping over two ranges dingus

// it's been so long since i did anything like this
// i feel so dumb

// 1.1
// I.E., the whole fucking reason i'm doing this in the first place
// that is a good thing
pub fn hex_to_base64(hex_string: String) -> String {
    // println!("{} bytes long", hex_string.as_bytes().len());
    // let test = U64::trailing_zeros(&self);
    let decoded = hex::decode(&hex_string).expect("check");
    let _result: &mut[u8] = &mut Vec::with_capacity(decoded.capacity());
    let encoding_string = Base64::encode_string(&decoded);
    return encoding_string;
    
    // iterator adaptor
    // which is just the iterator, but we take two at a time

    // okay
    // this took to long
    // still a lot to learn!!
    // learn to think about the types before hand
    // learn to think about the mutability of resutls before hand
    // making things immutable by default foces you to think about your data is going to be used
    // there's nothing very complicated, and the means by which you can do the encoding/decoding is important
    // note: work with bytes
    }
// one of the functions for scoring
// possible: replace this function
// with an increasing n-gram to list all possible n-grams
// for however feasible a given n-gram is ?
pub fn most_common_bigram(string: &str) -> () {
    // enforce size ordering by using a btreemap
    // which stores is an ordered map based on a b-tree
    // from each 2-length substring from the string
    // create a new entry into the btree
    let mut map = BTreeMap::new();

    // Iterate over windows of 2 characters
    for window in string.as_bytes().windows(2) {
        // Convert the bytes back to a string
        let substring = std::str::from_utf8(window).unwrap().to_string();
        *map.entry(substring).or_insert(0) += 1;
    }    
    println!("{}", map.last_key_value().expect("FREE").0);
}

pub fn calculate_index_of_coincidence(string: &str) -> Decimal {
    // calculated by summation of n choose 2 for each element of the alphabet
    // divided by n choose two 2, with this n being the length of the text 
    let bytes = string.as_bytes().to_vec();
    let length = Decimal::from(bytes.len());
    let freq_counts = frequency_hash_table(bytes);
    // let mut frequency_pair = freq_counts.clone();
    // for (u8,f32) in frequency_pair.iter_mut() {
    //     *f32 = *f32 / length;

    // }

    let numerator_of_ioc: Decimal = freq_counts.into_values()
    .map(|f| (f ) * ((f) - Decimal::ONE)).sum();
    let alphabet_normalization = Decimal::from_i16(28).unwrap();
    let ioc =  numerator_of_ioc / (length * (length- Decimal::ONE)/ alphabet_normalization);
    return ioc;
}

pub fn frequency_hash_table(s: Vec<u8>) -> HashMap<u8, Decimal> {
    let mut counts: HashMap<u8, Decimal> = HashMap::new();
    let decimal_one: Decimal = Decimal::from_i16(1).unwrap();
        for i in 0..= 127 {
        counts.insert(i, decimal_one);
    }

    for i in s.iter() {
        // reference to a hash map
        // referencing in the u8 (byte)
        // make an entry, or insert a default

        *counts.entry(*i).or_insert(Decimal::ZERO) += Decimal::ONE;
    }
    
    counts

}

}