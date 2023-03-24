use houdini;

fn main() {
    match houdini::disappear() {
        Ok(_) => println!("Pulled a Houdini!!"),
        Err(e) => println!("Nope! => {}", e),
    };
}
