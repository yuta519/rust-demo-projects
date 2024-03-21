pub fn always_error() -> Result<i32, String> {
    Err("This is an error".to_string())
}

pub fn fail() -> Result<(), String> {
    let _result = always_error()?;
    Ok(())
}
