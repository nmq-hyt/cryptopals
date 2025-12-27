mod tests {

use std::{f32::consts::LN_10, ffi::os_str::Display, fs::{self, File}, io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write}, ops::Index, path::Path, string, vec};
use std::collections::BTreeMap;
use std::collections::HashMap;

use base64ct::Encoding;
use rust_decimal::Decimal;

use crate::utils::functions::{calculate_index_of_coincidence, hex_to_base64};
use crate::utils::functions::fixed_xor;
// these are the solutions i have to the challenges.
// rust by example idiom import names from outer scope
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

fn brute_force_xor(text:Vec<u8>)  {
    for i in 0u8 .. 40 {
        println!("XOR KEY {}\n {}", i, &String::from_utf8(text.iter().map( |x| x ^ i).collect()).unwrap());
    }

}

fn hamming_distance(string_one: String, string_two: String) -> u32{
   assert_eq!(string_one.len(),string_two.len());

   return string_one
   .as_bytes()
   .into_iter().zip(string_two.as_bytes())
   .map(|x|(x.0 ^ x.1).count_ones())
   .sum();
// correction: it's counting bit differences 
// for which the compiler has a handy builtin
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

    // reflection: replacement ciphers preserver the underlying structure of words, while messing up the letters
    // a ciphertext looks like gibberish, but it's still the same word patterns
    // so cryptanalysis which measures frequency of letters works because you care about frequencies, things that occurr in all texts
    // note: monogram fitness (whether the vectors of a given plaintext's language match your cipher text) is an important signal here, because
    // closeness to frequencies of actual plaintext should be low; it is nothing like the english language
    let input_two = hex::decode("7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f".to_string()).expect("decoding unsuccessful");
    result_two.extend(input_two.iter().map(|x| x ^ 53));
    println!("{}", String::from_utf8(result_two).unwrap());
}

#[test]
fn test_repeating_key_xor() {
    repeating_key_xor_cipher("ICE");
}

fn repeating_key_xor_cipher(string :&str) -> () {
    // sequentially apply each byte of the key XOR against each given byte of your plaintext
    // should first find length of plaintext in bytes
    // rust has the iterator.cycle, which does what we need
    // but we need to stop somewhere
    // can track iterations and stop after the length
    let key: Vec<u8> = string.as_bytes().to_vec();
    let plaintext: &str = "Burning 'em, if you ain't quick and nimble 
        I go crazy when I hear a cymbal";
    let plaintext_length: usize = plaintext.as_bytes().len();

    // can we zip two iterators together if one is infinite?
    // docs seems to imply the zipped iterator will eventually yield None

    let xor_cipher_pass = plaintext
    .as_bytes()
    .iter()
    .zip(key
        .iter()
        .cycle()
    );

    let ciphertext:Vec<u8> = xor_cipher_pass.map(|element| element.0 ^ element.1).collect();
    ciphertext.iter().map(|x| print!(" {:x} ", x)).count();
    let base64_encoded: String = base64ct::Base64::encode_string(&ciphertext);
    println!("\n{}", base64_encoded);
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

#[test]
fn test_hamming_distance_works_as_expected() {
    assert_eq!(hamming_distance("eta".to_string(), "etb".to_string()), 2);
    assert_eq!(hamming_distance("aaa".to_string(), "etb".to_string()), 6);
    assert_eq!(hamming_distance("this is a test".to_string(), "wokka wokka!!!".to_string()), 37);
}


#[test]
fn test_break_repeating_key_xor() -> (){
    let path: &Path = Path::new("6.txt");
    let bytes: Vec<u8> = fs::read(path).unwrap();
    // let mut two_string: Vec<u8> = vec![0; string.len()];
    // base64ct::Base64::decode(&mut string, &mut two_string).expect("failed to decode");
    

    let mut keysize_guess: BTreeMap<u32, usize> = BTreeMap::new();

    for i in 2..200{
        let first_half = &bytes[0 .. (i - 1)];
        let second_half = &bytes[i  .. ((i*2) - 1)];
        let distance = hamming_distance(String::from_utf8(first_half.to_vec()).unwrap(), 
            String::from_utf8(second_half.to_vec()).unwrap()) / i as u32;
        keysize_guess.insert(distance, i);
    }
    println!("best keysize guesses {} {} {}", keysize_guess.pop_first().expect("dad").1, keysize_guess.pop_first().expect("cool").1,keysize_guess.pop_first().expect("dad").1);
    // VECTOR of vectors
    // map through chunks, adding nth byte to nth vector
    let key_size_guess = 3;
    let mut list_of_list: Vec<Vec<u8>> = Vec::new();
    for i in 0..key_size_guess {
        list_of_list.push(Vec::new());
    }
    // slightly ugly way of transposing
    bytes.chunks(key_size_guess).map(|f| for i in 0..key_size_guess {
        list_of_list[i].push(f[i]);
    }).count();
    list_of_list.iter().next();
    brute_force_xor(list_of_list.iter().next().unwrap().to_vec());

}
mod tests{
    use std::{f32::consts::LN_10, ffi::os_str::Display, fs::{self, File}, io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write}, ops::Index, path::Path, string, vec};

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

    fn brute_force_xor(text:Vec<u8>)  {
        for i in 0u8 .. 40 {
            println!("XOR KEY {}\n {}", i, &String::from_utf8(text.iter().map( |x| x ^ i).collect()).unwrap());
        }

    }

    fn hamming_distance(string_one: String, string_two: String) -> u32{
       assert_eq!(string_one.len(),string_two.len());

       return string_one
       .as_bytes()
       .into_iter().zip(string_two.as_bytes())
       .map(|x|(x.0 ^ x.1).count_ones())
       .sum();
    // correction: it's counting bit differences 
    // for which the compiler has a handy builtin
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

        // reflection: replacement ciphers preserver the underlying structure of words, while messing up the letters
        // a ciphertext looks like gibberish, but it's still the same word patterns
        // so cryptanalysis which measures frequency of letters works because you care about frequencies, things that occurr in all texts
        // note: monogram fitness (whether the vectors of a given plaintext's language match your cipher text) is an important signal here, because
        // closeness to frequencies of actual plaintext should be low; it is nothing like the english language
        let input_two = hex::decode("7b5a4215415d544115415d5015455447414c155c46155f4058455c5b523f".to_string()).expect("decoding unsuccessful");
        result_two.extend(input_two.iter().map(|x| x ^ 53));
        println!("{}", String::from_utf8(result_two).unwrap());
    }

    #[test]
    fn test_repeating_key_xor() {
        repeating_key_xor_cipher("ICE");
    }

    fn repeating_key_xor_cipher(string :&str) -> () {
        // sequentially apply each byte of the key XOR against each given byte of your plaintext
        // should first find length of plaintext in bytes
        // rust has the iterator.cycle, which does what we need
        // but we need to stop somewhere
        // can track iterations and stop after the length
        let key: Vec<u8> = string.as_bytes().to_vec();
        let plaintext: &str = "Burning 'em, if you ain't quick and nimble 
        I go crazy when I hear a cymbal";
        let plaintext_length: usize = plaintext.as_bytes().len();
    
        // can we zip two iterators together if one is infinite?
        // docs seems to imply the zipped iterator will eventually yield None
    
        let xor_cipher_pass = plaintext
        .as_bytes()
        .iter()
        .zip(key
            .iter()
            .cycle()
        );
    
        let ciphertext:Vec<u8> = xor_cipher_pass.map(|element| element.0 ^ element.1).collect();
        ciphertext.iter().map(|x| print!(" {:x} ", x)).count();
        let base64_encoded: String = base64ct::Base64::encode_string(&ciphertext);
        println!("\n{}", base64_encoded);
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

    #[test]
    fn test_hamming_distance_works_as_expected() {
        assert_eq!(hamming_distance("eta".to_string(), "etb".to_string()), 2);
        assert_eq!(hamming_distance("aaa".to_string(), "etb".to_string()), 6);
        assert_eq!(hamming_distance("this is a test".to_string(), "wokka wokka!!!".to_string()), 37);
    }


    #[test]
    fn test_break_repeating_key_xor() -> (){
        let path: &Path = Path::new("6.txt");
        let bytes: Vec<u8> = fs::read(path).unwrap();
        

        let mut keysize_guess: BTreeMap<u32, usize> = BTreeMap::new();

        for i in 2..200{
            let first_half = &bytes[0 .. (i - 1)];
            let second_half = &bytes[i  .. ((i*2) - 1)];
            let distance = hamming_distance(String::from_utf8(first_half.to_vec()).unwrap(), 
                String::from_utf8(second_half.to_vec()).unwrap()) / i as u32;
            keysize_guess.insert(distance, i);
        }
        println!("best keysize guesses {} {} {}", keysize_guess.pop_first().expect("dad").1, keysize_guess.pop_first().expect("cool").1,keysize_guess.pop_first().expect("dad").1);
        // VECTOR of vectors
        // map through chunks, adding nth byte to nth vector
        let key_size_guess = 3;
        let mut list_of_list: Vec<Vec<u8>> = Vec::new();
        for i in 0..key_size_guess {
            list_of_list.push(Vec::new());
        }
        // slightly ugly way of transposing
        bytes.chunks(key_size_guess).map(|f| for i in 0..key_size_guess {
            list_of_list[i].push(f[i]);
        }).count();
        list_of_list.iter().next();
        brute_force_xor(list_of_list.iter().next().unwrap().to_vec());

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
    // if you extend the summuation form?
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
}