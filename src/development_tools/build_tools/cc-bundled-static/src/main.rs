use std::error::Error;
use std::ffi::CString;
use std::os::raw::c_char;

fn prompt(s: &str) -> Result<String, Box<dyn Error>> {
    use std::io::Write;
    print!("{}", s);
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

extern {
    fn hello();
    fn greet(name: *const c_char);
}

fn main() -> Result<(), Box<dyn Error>> {
    unsafe { hello() }
    let name = prompt("What's your name? ")?;
    let c_name = CString::new(name)?;
    unsafe { greet(c_name.as_ptr()) }
    Ok(())
}
