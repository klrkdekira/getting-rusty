// TODO
// more reference at https://chrismorgan.info/blog/rust-fizzbuzz.html
fn main() {
    for i in (1..101).map(fizzbuzz) {
        println!("{}", i);
    }
}

fn fizzbuzz(i: i32) -> String {
    if i % 15 == 0 {
        "FizzBuzz".to_string()
    } else if i % 3 == 0 {
        "Fizz".to_string()
    } else if i % 5 == 0 {
        "Buzz".to_string()
    } else {
        i.to_string()
    }
}
