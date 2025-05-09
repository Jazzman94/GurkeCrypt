#[cfg(windows)]
fn main() {
    use std::io::Write;
    
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/icon.ico");
    res.set("InternalName", "TextProcessor");
    res.set("FileDescription", "Text Processor Application");
    res.set("ProductName", "Text Processor");
    
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