#[cfg(windows)]
fn main() {
    use std::io::Write;
    
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/icon.ico");
    res.set("InternalName", "GurkeCrypt");
    res.set("FileDescription", "Encdoing and decoding text");
    res.set("OriginalFilename", "GurkeCrypt");
    res.set("ProductName", "GurkeCrypt");
    
    match res.compile() {
        Err(e) => {
            write!(std::io::stderr(), "{}", e).unwrap();
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}

#[cfg(not(windows))]
fn main() {
    // No special handling needed for non-Windows platforms
}