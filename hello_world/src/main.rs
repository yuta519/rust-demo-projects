fn main() {

    // When you run the command `cargo clippy`, you will get the following warning:
    // warning: use of a disallowed/placeholder name `foo`
    // foo is a placeholder name and should be replaced by a more descriptive name
    let foo = "Foo";
    for _i in 0..10 {
        println!("Hello, world! {}", foo);
    }
}
