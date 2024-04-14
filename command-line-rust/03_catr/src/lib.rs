use clap::{Arg, Command};
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matchers = Command::new("catr")
        .version("0.1.0")
        .author("yoshinorin")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .required(true)
                .num_args(1..999),
        )
        .arg(Arg::new("number_lines").required(false).num_args(0))
        .arg(
            Arg::new("number_nonblank_lines")
                .required(false)
                .num_args(0),
        )
        .get_matches();

    let files = matchers
        .get_many::<String>("files")
        .unwrap()
        .map(|s| s.to_string());

    let number_lines = matchers.contains_id("number_lines");

    let number_nonblank_lines = matchers.contains_id("number_nonblank_lines");

    Ok(Config {
        files: files.collect::<Vec<_>>(),
        number_lines: number_lines,
        number_nonblank_lines: number_nonblank_lines,
    })
}
