#[cfg(target_os = "windows")]
use houdini;

fn main() {
    #[cfg(target_os = "windows")]
    match houdini::disappear_with_placeholder("temporary") {
        Ok(_) => println!("Pulled a Houdini!!"),
        Err(e) => println!("Nope! => {}", e),
    };
}
