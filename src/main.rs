/// Basically, this prints Hello world in the terminal
fn main() {
    print("Hello, world!");
}

fn print(s: &str) {
    println!("{s}");
}

fn this_will_conflict() {
}
