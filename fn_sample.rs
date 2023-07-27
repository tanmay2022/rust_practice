fn goodbye(message: &str) {
    println!("\n{}", message);
}

fn main() {
    let formal = "Formal: Goodbye.";
    let casual = "Casual: See you later!";
    goodbye(formal);
    goodbye(casual);
}
