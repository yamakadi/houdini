# Houdini
`Houdini` is a rust library that allows you to delete your executable while it's running.  

This is fairly straightforward for unix systems, since the executable is released after getting mapped to the memory. 
We just need to find where it is and unlink it.

On Windows, we use a method discovered by [@jonasLyk](https://twitter.com/jonasLyk/status/1350401461985955840). 
My implementation heavily references [@byt3bl33d3r](https://twitter.com/byt3bl33d3r)'s Nim implementation in [OffensiveNim](https://github.com/byt3bl33d3r/OffensiveNim/blob/master/src/self_delete_bin.nim)
and in turn LloydLabs' initial [`C` PoC](https://github.com/LloydLabs/delete-self-poc).

## Usage

```rust
// With a default placeholder value on windows (`svcmsrpc`)
use houdini;

fn main() {
    match houdini::disappear() {
        Ok(_) => println!("Pulled a Houdini!!"),
        Err(e) => println!("Nope! => {}", e),
    };
}
```

```rust
// With a placeholder you provide
use houdini::disappear;

fn main() {
    #[cfg(target_os = "windows")]
    match houdini::disappear_with_placeholder("temporary") {
        Ok(_) => println!("Pulled a Houdini!!"),
        Err(e) => println!("Nope! => {}", e),
    };
}
```