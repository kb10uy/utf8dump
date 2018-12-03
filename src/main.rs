use std::io::stdin;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error: stdin is empty");

    let split = UnicodeSegmentation::graphemes(input.trim(), true).collect::<Vec<&str>>();
    for grapheme in split.iter() {
        println!("{}", grapheme);
        let codepoints = grapheme.chars();

        print!("Codepoints: ");
        for codepoint in codepoints {
            print!("U+{:04X} ", codepoint as u32);
        }
        println!();

        print!("UTF-8 Byte Sequence: ");
        for byte in grapheme.as_bytes() {
            print!("0x{:02X} ", byte);
        }
        println!();
        println!();
    }
}
