use std::{io,env, process::exit};
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

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

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
            "-r" | "--random" => println!("Printing to ranDOM CAsE"),
            "-s" | "--leet" => println!("Printing to l337$p34k"),
            "-g" | "--angry" => println!("Printing to Angry. Case."),
            "-t" | "--trueangry" => println!("Printing to REALLY. ANGRY. CASE."),
            _ => println!("Invalid case type: {}", set_case)
        }
    
        println!("Case setting: {}", set_case);
        println!("Text to transform: {}", src_test);
        Ok(())
    }
}
