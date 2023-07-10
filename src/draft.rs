
fn main() {
    let n = 123456;
    let formatted = n.to_formatted_string(&Locale::en);
    println!("{}", formatted);
}
