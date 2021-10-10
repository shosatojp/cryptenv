use std::{io::Read, process::exit};

use clap::crate_name;

#[macro_use]
mod cli;
mod crypto;
mod util;

extern crate aes;
extern crate base64;
extern crate block_modes;
extern crate clap;
extern crate openssl;
extern crate rand;
extern crate regex;
extern crate rpassword;
extern crate sha2;

const PREFIX: &str = "cryptenv://";

fn print_minimal_usage() {
    eprintln!("run \"{} -h\" for more help", crate_name!())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = cli::build_cli();
    let matches = app.get_matches();

    match matches.value_of("data") {
        Some(path) => {
            // encryption
            let mut file = std::fs::File::open(path)?;
            let mut data_buf: Vec<u8> = Vec::new();
            file.read_to_end(&mut data_buf)?;

            let orig_key = rpassword::prompt_password_stderr("password? ")?;
            let enc = crypto::encrypt(orig_key.as_bytes(), &data_buf)?;
            println!("{}{}", PREFIX, base64::encode(enc));
        }
        None => {
            // decription
            let mut command = matches.values_of("command").unwrap_or_else(|| {
                eprintln!("command is required");
                print_minimal_usage();
                exit(1);
            });
            let mut proc = std::process::Command::new(command.nth(0).unwrap());
            proc.env_clear();

            let orig_key = rpassword::prompt_password_stderr("password? ")?;

            for (ref key, ref value) in std::env::vars() {
                if value.starts_with(PREFIX) {
                    // decrypt
                    let (_, enc) = value.split_at(PREFIX.len());
                    let dec = crypto::decrypt(orig_key.as_bytes(), &base64::decode(enc)?)?;
                    let data = String::from_utf8(dec).unwrap();
                    proc.env(key, &data);
                } else {
                    proc.env(key, value);
                }
            }

            proc.args(command.skip(0));
            proc.spawn()?.wait()?;
        }
    }
    Ok(())
}
