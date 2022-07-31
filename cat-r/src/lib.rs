use std::error::Error;
use clap::{App,Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cat-r")
        .version("0.1.0")
        .author("Bjoern Doebel <bjoern.doebel@gmail.com")
        .about("cat in Rust")
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .help("Number lines")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .help("Number non-blank lines as well")
                .takes_value(false)
                .conflicts_with("number_lines")
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("List of files")
                .required(false)
                .min_values(1)
                .default_value("-")
        )
        .get_matches();
    
    Ok(Config {
        files : matches.values_of_lossy("files").unwrap(),
        number_lines : matches.is_present("number_lines"),
        number_nonblank : matches.is_present("number_nonblank_lines")
    })

}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut counter = 0;
    for file in config.files {
        let fread = match open(&file) {
            Err(e) => {
                eprintln!("Failed to open {}: {}", file, e);
                continue;
            },
            Ok(rdr) => rdr,
        };

        for l in fread.lines() {
            let mut out_prefix = String::from("");
            let line = l.unwrap();

            if config.number_lines {
                counter += 1;
                out_prefix.push_str(format!("{:6}\t", counter).as_str());
            }

            if config.number_nonblank && (line.len() > 0) {
                    counter += 1;
                    out_prefix.push_str(format!("{:6}\t", counter).as_str());
            }

            println!("{}{}", out_prefix, line);
        }
    }
    Ok(())
}