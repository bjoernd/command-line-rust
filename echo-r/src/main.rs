use clap::{App, Arg};

fn main() {
    let _matches = App::new("echo-r")
        .version("0.1.0")
        .author("Bjoern Doebel <bjoern.doebel@gmail.com")
        .about("echo in Rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Output text")
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not emit newline after TEXT")
                .takes_value(false)

        )
        .get_matches();

    let text = _matches.values_of_lossy("text").unwrap();

    let ending = if _matches.is_present("omit_newline") { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending);
}   
