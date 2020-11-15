#[cfg(target_os="macos")]
#[macro_use]
extern crate objc;


#[cfg(test)]
mod tests;

mod os;

fn main() {
    println!("Hello, world!");
}
