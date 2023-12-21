mod cli;
mod crate_info;

fn main() {
    let matches = cli::new().get_matches();

    match matches.subcommand() {
        Some(("convert", m)) => cli::run_convert(m),
        _ => cli::new().print_help().unwrap(),
    }
}
