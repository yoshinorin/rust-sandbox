use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("yoshinorin")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..999),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .num_args(0)
                .required(false),
        )
        .get_matches();

    let text = matches
        .get_many::<String>("text")
        .unwrap()
        .map(|s| s.as_str());
    let ommit_newline = matches.contains_id("omit_newline");

    let ending = if ommit_newline { "" } else { "\n" };
    print!("{}{}", text.collect::<Vec<_>>().join(" "), ending);
}
