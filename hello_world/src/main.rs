mod fizbuzz;
use fizbuzz::fizbuzz;

mod color_token;
use color_token::color_name_to_jp;

mod loop_func;
use loop_func::add_until;

// mod result_type;
// use result_type::fail;

fn main() {
    // When you run the command `cargo clippy`, you will get the following warning:
    // warning: use of a disallowed/placeholder name `foo`
    // foo is a placeholder name and should be replaced by a more descriptive name
    // let foo = "Foo";
    // for _i in 0..10 {
    //     println!("Hello, world! {}", foo);
    // }

    // // Using `Result` type
    // let some: Result<&str, &str> = Ok("Hello, world!");
    // println!("{:?}", some);
    // let err: Result<&str, &str> = Err("Hello, world!");
    // print!("{:?} ", err);
    // result_type::main();

    // Tryp optionals
    // let message: String = match fail() {
    //     Ok(_) => "Success".to_string(),
    //     Err(err) => err,
    // };
    // println!("{}", message);

    // FizzBuzz
    println!("{}", fizbuzz(15));
    println!("{}", fizbuzz(10));
    println!("{}", fizbuzz(9));
    println!("{}", fizbuzz(1));
    println!("{:?}", color_name_to_jp("red"));

    // Loop
    let sum = add_until(1, 100);
    println!("{}", sum)
}
