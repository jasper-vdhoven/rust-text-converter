use std::{io,env,process::exit, borrow::Cow, vec};
use convert_case::{Case, Casing};
use emojis::Emoji;
use node_emoji::Replacer;
use unicode_segmentation::UnicodeSegmentation;

fn convert_to_leetspeak(input_string:&String) -> String {
    let mut converted_string:String = String::from("");
    // iterate over the chars in the input string variable and substitute where necessary
    for chars in input_string.chars() {
        match chars {
            'a' | 'A' => converted_string.push('4'),
            'b' | 'B' => converted_string.push('8'),
            'e' | 'E' => converted_string.push('3'),
            'g' | 'G' => converted_string.push('9'),
            'i' | 'I' => converted_string.push('1'),
            'o' | 'O' => converted_string.push('0'),
            's' | 'S' => converted_string.push('5'),
            't' | 'T' => converted_string.push('7'),
            _ => converted_string.push(chars)
        }
    }
    return converted_string;
}

fn convert_shortcodes_to_emojis(input_string:&String) -> Cow<str> {
    // Takes a shortcode :smile: and converts it into the equivalent emoji
    // Use node_emoji::Replacer to search through a string for all matches in one go
    let replacer: Replacer = node_emoji::Replacer::new();
    let converted_string: Cow<str> = replacer.replace_all(input_string);

    return converted_string;
}

fn emoji_case(input_string:&String) -> Vec<String> {
    let mut emoji_array: Vec<String> = vec![];

    // takes a string and substitutes words -> emojis where possible
    // dog -> dog emoji | ocean -> ocean emoji
    for words in input_string.to_lowercase().split(" ").collect::<Vec<_>>() {
        let emoji_char = node_emoji::get(words);
        // If the entered emoji string is invalid, push the original value to the vec
        if emoji_char.is_none() {
            emoji_array.push(words.to_string());
        }
        // Else, pus the translated emoji equivalent to the vec
        else {
            emoji_array.push(emoji_char.unwrap().to_string());
        }
    }
    return emoji_array;
}

fn decode_emojis_to_shortcode(input_string:&String) -> Option<Vec<String>> {
    let mut emoji_arary: Vec<String> = vec![]; 
    // Convert emoji input into short code, this is only for a single emoji (I think)
    for emoji in input_string.graphemes(false) {
        let emoji_char: &Emoji = emojis::get(emoji)?;
        emoji_arary.push(format!(":{}:", emoji_char.name().to_string()));
    }

    return Some(emoji_arary);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Check for correct amount of arguments present
    if args.len() != 3 {
        print!("Usage: rust-text-converter [argument] [input string]");
        exit(1)
    } 
    else {
        println!("Rust Text Converter\n");
        let set_case = &args[1];
        let src_test = &args[2];
    
        match set_case.as_str() {
            "-c" | "--caps" => println!("{}", src_test.to_uppercase()),
            "-l" | "--lower" => println!("{}", src_test.to_lowercase()),
            "-a" | "--alt" => println!("{}", src_test.to_case(Case::Alternating)),
            "-i" | "--invalt" => println!("Printing to iNvErTeD aLtErNaTiVe cAsE"),
            "-r" | "--random" => println!("{}", src_test.to_case(Case::Random)),
            "-s" | "--leet" => println!("{}", convert_to_leetspeak(src_test)),
            "-g" | "--angry" => println!("{}", src_test.to_case(Case::Title).replace(' ', ".")),
            "-t" | "--trueangry" => println!("{}", src_test.to_uppercase().replace(' ', ".")),
            "-v" | "--reverse" => println!("{}", src_test.chars().rev().collect::<String>()),
            "-e" | "--emoji" => println!("{}", convert_shortcodes_to_emojis(src_test)),
            "-d" | "--decode" => {
                let emoji_vec: Option<Vec<String>> = decode_emojis_to_shortcode(src_test);
                for items in emoji_vec.iter().flatten() {
                    println!("{}", items);
                }
            }
            "-f" | "--emojicase" => {
                let emoji_vec: Vec<String> = emoji_case(src_test);
                for items in emoji_vec.iter() {
                    println!("{}", items)
                }
            }
            _ => println!("Invalid case type: {}", set_case)
        }
        Ok(())
    }
}
