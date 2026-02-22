fn main() {
    let x = define_x(); // Використовуємо функцію для отримання x
    println!("{}, world", x);
}

fn define_x() -> &'static str {
    let x = "hello";
    x
}