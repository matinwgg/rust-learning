// Closure: An anonymous function that can capture its environment
// closures are also a way to access variables defined outside

fn main() {
    let clsr0 = |num| num + 1;
    println!("{}", clsr0(2));

    let my_value = 5;
    let clsr1 = |num| num + my_value;
    println!("{}", clsr1(2));

    let clsr2 = |x| println!("{}", x);
    clsr2("The quick brown fox jumped over the lazy dog!");

    let clsr3 = |x, y| x+y;
    println!("{}", clsr3(2, 3));
    println!("{}", clsr3("The quick brown fox ".to_string(), "jumped over the lazy dog!"))
}
