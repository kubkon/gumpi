extern crate clap;
extern crate failure;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate log;

mod args;
mod backend;
mod failure_ext;
mod session;
use crate::{backend::SessionManMPI, session::SessionMan};
use failure::Fallible;
use std::env;

fn main() {
    init_logger();
    if let Err(e) = run() {
        eprint!("error");
        for cause in e.iter_chain() {
            eprint!(": {}", cause);
        }
        eprintln!("");
        std::process::exit(1);
    }
}

fn init_logger() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init()
}

fn run() -> Fallible<()> {
    let matches = args::get_parser().get_matches();
    let progname = matches.value_of("progname").unwrap();
    let nproc: u32 = matches.value_of("numproc").unwrap().parse()?;

    let mut mgr = SessionMan::new("127.0.0.1:61621".to_owned());
    info!("Creating session");
    mgr.create()?;

    let mpimgr = SessionManMPI::new(mgr, progname.to_owned());
    mpimgr.make()?;
    mpimgr.run(nproc, args);
    Ok(())
}
