// the rust use command; base64::prelude
// base 64 is the library,
// A prelude is a collection of names that are automatically brought into scope of every module in a crate. (fro mthe rhe rust reference)
use base64ct::{Base64, Encoding};
use hex;
use std::collections::{BTreeMap, HashMap};
use rust_decimal::{Decimal, prelude::{FromPrimitive}};
fn main() {
   println!("Hello, world!");
}

// if i wanted to do this manually i'd still do it the way i did when i was using python
// run through the string, from the right to the left, three bytes at a time, converting each three byte chunk
// into equivalent four 6 bit (48 bits) base 64
// using a lookup table.

// it's been so long since i did anything like this
// i feel so dumb

// 1.1
// I.E., the whole fucking reason i'm doing this in the first place
// that is a good thing
fn hex_to_base64(hex_string: String) -> String {
    // println!("{} bytes long", hex_string.as_bytes().len());
    // let test = U64::trailing_zeros(&self);
    let decoded = hex::decode(&hex_string).expect("check");
    let result: &mut[u8] = &mut Vec::with_capacity(decoded.capacity());
    let encoding_string = Base64::encode_string(&decoded);
    return encoding_string;
    
    // iterator adaptor
    // which is just the iterator, but we take two at a time
    // (which is equal to )

    // okay
    // this took to long
    // still a lot to learn!!
    // learn to think about the types before hand
    // learn to think about the mutability of resutls before hand
    // making things immutable by default foces you to think about your data is going to be used
    // 
    // there's nothing very complicated, and the means by which you can do the encoding/decoding is important
    // note: work with bytes

    
}

// make it functional girly pops
// here's chain iterators: take two iterators, create one big one
// and the one we want, zip iterator, which makes a pair of them, and tucks them into a tuple
// and then we collect it into a vector!
fn fixed_xor (buffer_one: Vec<u8>, buffer_two: Vec<u8>) -> Vec<u8> {
    assert_eq!(buffer_one.len(), buffer_two.len());
    let result: Vec<u8> = buffer_one.iter().zip(buffer_two.iter()).map(|f| f.0 ^ f.1).collect();
    return result;


}

// one of the functions for scoring
// lead into: ETAOIN SHROLDU,
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
    // this is done entirely by index of coincidence
    // highest scoring entry is brute forced is how i got the solution
    // calculated by summation of n choose 2 for each element of the alphabet
    // divided by n choose two 2, with this n being the length of the text 
    let bytes = hex::decode(string).unwrap();
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

fn frequency_hash_table(s: Vec<u8>) -> HashMap<u8, Decimal> {
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
mod tests {
    use std::{ffi::os_str::Display, fs::File, io::{self, BufRead}, path::Path};

    use hex::decode;

    // rust by example idiom import names from outer scope
    use super::*;

    #[test]
    fn test_hex_to_base64_works(){
        assert_eq!(hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()),
    "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t")
    }
    #[test]
    fn test_fixed_xor_works() {
        assert_eq!(fixed_xor(hex::decode("1c0111001f010100061a024b53535009181c".to_string()).expect("failue"), 
        hex::decode("686974207468652062756c6c277320657965".to_string()).expect("fuck")), hex::decode("746865206b696420646f6e277420706c6179").expect("failed to"));
    }

    #[test]
    fn test_single_byte_xor_cipher() {
        // 1.2 key derived from brute force key search
        let input_one = hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string()).expect("decoding unsueccesful");
        let mut result_one: Vec<u8> = Vec::with_capacity(input_one.capacity());
        let mut result_two = result_one.clone();
        result_one.extend(input_one.iter().map(|x| x ^ 88));
        println!("{}", String::from_utf8(result_one).unwrap());  
        // 1.3 key derived as it should have been
        // incidence of coincidence calculated to return similarity score with english test
        // took the highest scoring string, then brute forced the key
        let input_two = hex::decode("7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f".to_string()).expect("decoding unsuccessful");
        result_two.extend(input_two.iter().map(|x| x ^ 53));
        println!("{}", String::from_utf8(result_two).unwrap());
    }

    #[test]
    fn testbed_file_opening() {
        let path = Path::new("4.txt");
        // much like when i used elixir 
        // we see the switch statement!
        // i actually really like this syntax, it was  one of the best things about elixir when i extremely briefly used it

        let mut file  = match File::open(&path)  {
            Err(why) => panic!("could not read file {} because {}", path.to_str().expect("failed to retrieve string representation of path"), why),
            Ok(file) => file,
        };

        // let file: File = File::open(path)?;
        // create a buffered reader on a file, and split it into component limnes
        let lines_iterator = io::BufReader::new(file).lines();
        // you get a result from a bunch of different std crate things, usually IO
        // several parts of the library are aware of this
        // you must use results.
        // you can generally expect to unwrap it
        // but it's worth matching on the error case explicitly
        
        //WOOPS! btreemap sorts by key order.
        // and it doesn't implement Ord for f32 

        // replace it with something which supports very small decimal numbers
        // okay i don't entirely know enough of the top of my head
        // to have an opinion on floats vs fixed-point
        // but if we're real this is one of the areas where this seriously matters
        let mut scorekeeper: BTreeMap<Decimal, String> = BTreeMap::new();
         for line in lines_iterator{
            let mut line = line.expect("failed to read line");
            println!("{}", line);
            scorekeeper.insert(calculate_index_of_coincidence(&line), line.clone(),);
    }
    scorekeeper.iter().map(|x| println!("{}, {}", x.0, x.1)).count();
    }
}

        // you can use count() as the collecting iterator to just consume an iterator 
        // lines_iterator.map(|x|  println!("{}",x.unwrap()))
        // .count();
    // decode the original string
    // xor each byte against a given character from the ascii alphabet
    // try to guess the key by character frequency

    // brute forced it first time. key is X or 88 in binary for challeng 1.1
    // from characters 64 to 89, it looks vaguely closer to cipher text in a sentence
    // and i initially did think of what character frequency purely in alphabetical
    // but of course space is a really common char in english text
    // WORKING BACKWORDS 
    // 0b0100000 is 64 in binary and is equal to space in ascii charset
    // 0b0100000 = 0b1011000 ^ 0b0011000
    // the enciphered space is byte 0b0011000,
    // and the key is 0b1011000

    // scoring plain text
    // count number of common characters in plain text 
    // frequency of vowels, apostophries, space

    // how can we tell something has been XOR'd against a single character?
    // xor is a replacement cipher, in effect.
    // like a caeshar cipher
    // so we have to figure out the actual underlying structure of the text

    // the minimal answer here is index of coincidence
    // which calcluates the chance that you'll pick two matching letters
    // mathematically, product of chance of choosing the letter at random
    // and of drawing it again without replacement from the text
    // for each letter of your alphabet
    // which is (n_i/N * (n_i - 1 / N-1))
    // by rule of products (since you're doing both at the same time)
    // and can be added together by rule of sums (because you're not choosing different letters at the same time, but you are counting them all)
    // which simplfies as the denominator remains the same
    // possible to extend this to n-grams 
    // if you extend the summuation form
    // as you're trying to pick the same letter from an increasingly smaller string
    //  which i think is calculating factorials?


    // test if it's involuted, if you have the plaintext
    // key ^ ciphertext = plaintext
    // plaintext ^ ciphertext = key
    // plaintext ^ key = ciphertext

    // ETAOIN SHRDLU 
    // take a given string, count the most common character
    // what key turns it into E,T,A? etc
    // are there repeats?
    // appearces of ER, TH, ON, to better guess?