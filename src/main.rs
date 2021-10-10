use std::{io::Read, process::exit};

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
extern crate sha2;

const PREFIX: &str = "cryptenv://";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = cli::build_cli();
    let matches = app.get_matches();

    if matches.is_present("enc") {
        let path = match matches.value_of("data") {
            Some(v) => v,
            None => {
                eprintln!("data is required on encryption");
                exit(1);
            }
        };

        let mut file = std::fs::File::open(path)?;
        let mut data_buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut data_buf)?;

        // encrypt
        let orig_key = util::get_key_from_stdin()?;
        let enc = crypto::encrypt(orig_key.as_bytes(), &data_buf);
        println!("{}{}", PREFIX, base64::encode(enc));
    } else {
        let mut command = matches.values_of("command").expect("command is required");
        let mut proc = std::process::Command::new(command.nth(0).unwrap());
        proc.env_clear();

        let orig_key = util::get_key_from_stdin()?;
        let enc_key = orig_key.as_bytes();

        for (ref key, ref value) in std::env::vars() {
            if value.starts_with(PREFIX) {
                // decrypt
                let (_, enc) = value.split_at(PREFIX.len());
                let dec = crypto::decrypt(enc_key, &base64::decode(enc)?);
                let data = String::from_utf8(dec).unwrap();
                proc.env(key, &data);
            } else {
                proc.env(key, value);
            }
        }

        proc.args(command.skip(0));
            proc.spawn()?.wait()?;
    }
    Ok(())
}
