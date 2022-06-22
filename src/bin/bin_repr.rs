fn main() {
    let number: i32 = 2;
    let width = 5;
    let n = format!("{:0width$b}", number, width=width, );
    println!("{}", n)
}