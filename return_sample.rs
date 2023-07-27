fn divide_by_5(num: u32) -> u32 {
    if num == 0 {
        // Return early
        return 0;
    }
    num / 5
}

fn main() {
    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));
}
