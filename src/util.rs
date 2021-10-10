pub fn get_key_from_stdin() -> Result<String, Box<dyn std::error::Error>> {
    eprint!("input key>");
    let mut key_buf = String::new();
    std::io::stdin().read_line(&mut key_buf)?;
    Ok(key_buf)
}
