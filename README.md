# Rust text converter

Rust port of the JavaTextConverter from @Ad-Blokker accessible under [github.com/Ad-Blokker/JavaTextConverter](https://github.com/Ad-Blokker/JavaTextConverter).

This port originated as a dare from Mark on my comment for when he would add Emoji-case to his text converter. To that question he replied "make a PR", to which I foolishly replied that I would make it happen, but only if I could port it to Rust instead of Java. To this Mark agreed and said to go make it, so I did.

The Rust port consists of two main components: My semi-self written Rust back-end that handles the conversions, and the front-end served via [rocket.rs](https://rocket.rs) where the text input will be sent to and from.

The Rust binary acts as a stand-alone service supplying the web-server that displays the text and handles the text conversion, because that's how the original was written. The server is normally accessible under 127.0.0.1:8000 in this default configuration, but can be changed to any arbitrary port and domain if so desired in the source. The dependencies are listed in Cargo.toml, but are as follows:

> This converter utilises Rocket **v0.5-rc2** and Rust **nightly** for its functionalities

- [convert_case](https://crates.io/crates/convert_case)
- [emojis](https://crates.io/crates/emojis)
- [rocket](https://crates.io/crates/rocket)
- [gh_emoji](https://crates.io/crates/gh-emoji)
- [unicode-segmentation](https://crates.io/crates/unicode-segmentation)


For anyone that finds this and found it useful/funny, feel free to make an issue or PR for things you'd wish changed/improved upon, this is one of the first actual Rust apps I've written with Rust.
