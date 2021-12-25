use crate::huffman_compression::{HuffmannCode, HuffmannResult};

mod huffman_compression;

fn main() {
    let text: &str = "Wer reitet so spaet durch Nacht und Wind, es ist der Vater mit einem Kind; Er hat den Knaben wohl in dem Arm, Er fasst ihn sicher, er haelt ihn warm.";
    // let text: &str = "aababcabcd";
    let compressor: HuffmannCode = HuffmannCode::new();
    let huffman_result: HuffmannResult = compressor.huffmann_algorithm(text);

    println!("{}", huffman_result.encrypted_string);

    let clear_text: String = compressor.encrypted_string_to_text(&huffman_result);
    println!("{}", clear_text);

    println!("The original text size is: {}Bits. The compressed text is {}Bits. It's a total difference of: {}Bits", text.len() * 8, (&huffman_result.encrypted_string).len(), difference(&huffman_result.encrypted_string, text));
}

fn difference(bit_count: &str, eight_per_letter: &str) -> i32 {
    (eight_per_letter.len() as i32 * 8) - bit_count.len() as i32
}
