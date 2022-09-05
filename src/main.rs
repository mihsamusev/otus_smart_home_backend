use clap::{self, App, Arg};
use smart_home_backend::cli;
use smart_home_backend::repository::room::InMemoryRepository;
use std::sync::Arc;

fn main() {
    let repo = Arc::new(InMemoryRepository::new());

    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .arg(Arg::with_name("cli").long("cli").help("Runs in CLI mode"))
        .get_matches();

    match matches.occurrences_of("cli") {
        0 => unreachable!(),
        _ => cli::run(repo),
    }
}
