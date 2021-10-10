use std::{io::Read, process::exit};

use clap::crate_name;

#[macro_use]
mod cli;
mod crypto;

extern crate aes;
extern crate base64;
extern crate block_modes;
extern crate clap;
extern crate openssl;
extern crate rand;
extern crate rpassword;
extern crate sha2;

const PREFIX: &str = "cryptenv://";

fn print_minimal_usage() {
    eprintln!("run \"{} -h\" for more help", crate_name!())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = cli::build_cli();
    let matches = app.get_matches();

    if matches.is_present("list") {
        //list
        std::env::vars().for_each(|(ref key, ref value)| {
            if value.starts_with(PREFIX) {
                println!("{}={}", key, value);
            }
        });
        return Ok(());
    } else if let Some(path) = matches.value_of("data") {
        // encryption
        let mut file = std::fs::File::open(path)?;
        let mut data_buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut data_buf)?;

        let orig_key = match matches.value_of("password-from") {
            Some(var_name) => std::env::var(var_name)?,
            None => rpassword::prompt_password_stderr("password? ")?,
        };
        let enc = crypto::encrypt(orig_key.as_bytes(), &data_buf)?;
        println!("{}{}", PREFIX, base64::encode(enc));
        return Ok(());
    } else {
        // decription
        let mut command = matches.values_of("command").unwrap_or_else(|| {
            eprintln!("command is required");
            print_minimal_usage();
            exit(1);
        });
        let mut proc = std::process::Command::new(command.nth(0).unwrap());
        proc.env_clear();

        let orig_key = match matches.value_of("password-from") {
            Some(var_name) => std::env::var(var_name)?,
            None => rpassword::prompt_password_stderr("password? ")?,
        };

        for (ref key, ref value) in std::env::vars() {
            if value.starts_with(PREFIX) {
                // decrypt
                let (_, enc_base64) = value.split_at(PREFIX.len());
                let enc = match base64::decode(enc_base64) {
                    Ok(enc) => enc,
                    Err(_) => {
                        eprintln!("failed to decode base64: {}, skip", key);
                        continue;
                    }
                };
                let dec = match crypto::decrypt(orig_key.as_bytes(), &enc) {
                    Ok(dec) => dec,
                    Err(_) => {
                        eprintln!("failed to decrypt: {}, skip", key);
                        continue;
                    }
                };
                let data = String::from_utf8(dec).unwrap();
                proc.env(key, &data);
            } else {
                proc.env(key, value);
            }
        }

        proc.args(command.skip(0));
        proc.spawn()?.wait()?;
        return Ok(());
    }
}
