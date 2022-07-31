fn main() {
    if let Err(e) = cat_r::get_args().and_then(cat_r::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
