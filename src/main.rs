#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::{relative, FileServer, NamedFile};

use convert_case::{Case, Casing};
use gh_emoji::Replacer;
use std::{borrow::Cow, vec};
use unicode_segmentation::UnicodeSegmentation;

// Struct with the form structure that will be sent by the front-end
#[derive(Debug, FromForm)]
struct InputText<'r> {
    pub input_string: &'r str,
    pub format: String,
}

fn convert_to_leetspeak(input_string: &String) -> String {
    let mut converted_string: String = String::from("");
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
            _ => converted_string.push(chars),
        }
    }
    return converted_string;
}

// TODO: check why this won't print certain emojis, such as melting_face
fn convert_shortcodes_to_emojis(input_string: &String) -> Cow<str> {
    // Takes a shortcode :smile: and converts it into the equivalent emoji
    // Use gh_emoji::Replacer to search through a string for all matches in one go
    let replacer: Replacer = gh_emoji::Replacer::new();
    let converted_string: Cow<str> = replacer.replace_all(input_string);

    return converted_string;
}

fn emoji_case(input_string: &String) -> String {
    let mut emoji_array: Vec<String> = vec![];

    // takes a string and substitutes words -> emojis where possible
    // dog -> dog emoji | ocean -> ocean emoji
    for words in input_string.to_lowercase().split(" ").collect::<Vec<_>>() {
        let emoji_char = gh_emoji::get(words);
        // If the entered emoji string is invalid, push the original value to the vec
        if emoji_char.is_none() {
            emoji_array.push(words.to_string() + " ");
        }
        // Else, pus the translated emoji equivalent to the vec
        else {
            emoji_array.push(emoji_char.unwrap().to_string() + " ");
        }
    }
    // return these shitty vecs as a string to make life easier
    return emoji_array.into_iter().collect::<String>();
}

fn decode_emojis_to_shortcode(input_string: &String) -> String {
    let mut emoji_arary: Vec<String> = vec![];
    println!("input_string <function>:  {}", input_string);
    // split the text into Unicode graphemes, then check whether they are a valid emoji
    // if the value is NOT an emoji, push the input to the vector and move on, otherwise
    // this will throw a panic, and crash the server
    for emoji in input_string.graphemes(false) {
        if emojis::get(emoji).is_none() {
            emoji_arary.push(emoji.to_string());
        } else {
            emoji_arary.push(emojis::get(emoji).unwrap().to_string())
        }
    }
    // return these shitty vecs as a string to make life easier
    return emoji_arary.into_iter().collect::<String>();
}

fn space_case(input_string: &String) -> String {
    // take the input string, then add a space between each character
    // input string -> i n p u t s t r i n g
    let mut char_array: Vec<String> = vec![];
    for characters in input_string.chars() {
        // if the current character is already a space, ignore it and add it back into the vector
        if characters == ' ' {
            char_array.push(characters.to_string());
        } else {
            char_array.push(characters.to_string() + " ");
        }
    }
    return char_array.into_iter().collect();
}

#[post("/api/convert", data = "<form>")]
async fn api(form: Form<InputText<'_>>) -> String {
    let input_text: String = form.input_string.to_string();
    let selected_case: String = form.format.to_owned().to_string();

    let converted_text: String = state_selector(&selected_case, &input_text);
    converted_text
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("static/index.html").await.ok()
}

fn state_selector(case: &String, input_text: &String) -> String {
    let mut converted_text: String = String::from("");

    match case.as_ref() {
        "uppercase" => converted_text.push_str(&input_text.to_uppercase()),
        "lowercase" => converted_text.push_str(&input_text.to_lowercase()),
        "alternative-case" => converted_text.push_str(&input_text.to_case(Case::Alternating)),
        "random-case" => converted_text.push_str(&input_text.to_case(Case::Random)),
        "leet-case" => converted_text.push_str(&convert_to_leetspeak(&input_text)),
        "angry-case" => converted_text.push_str(&input_text.to_case(Case::Title).replace(" ", ".")),
        "true-angry-case" => converted_text.push_str(&input_text.to_uppercase().replace(" ", ".")),
        "reverse-case" => converted_text.push_str(&input_text.chars().rev().collect::<String>()),
        "space-case" => converted_text.push_str(&space_case(&input_text)),
        "convert-shortcode-to-emoji" => {
            converted_text.push_str(&convert_shortcodes_to_emojis(&input_text))
        }
        "convert-emoji-to-shortcode" => {
            converted_text.push_str(&decode_emojis_to_shortcode(&input_text))
        }
        "emoji-case" => converted_text.push_str(&emoji_case(&input_text)),
        _ => converted_text.push_str("Invalid case"),
    }
    converted_text
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, api])
        .mount("/", FileServer::from(relative!("static")))
}
