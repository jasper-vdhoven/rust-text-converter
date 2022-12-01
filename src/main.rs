use std::{io,env, process::exit};

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
