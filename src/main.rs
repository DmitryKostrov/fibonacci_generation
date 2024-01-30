use std::io;

// Генерирование n-го числа Фибоначчи.
// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946,….   ;)
fn main() {
    println!("Enter Fibonacci number");
    let mut input_user: String = String::new();
    io::stdin()
        .read_line(&mut input_user)
        .expect("Failed to read line");
    let mut input_user: u16 = input_user.trim().parse().expect("Please type a number!\n");

    let mut _fibo1 = 0;
    let mut _fibo2 = 1;
    for num in 0..input_user {

        let next: u32 = _fibo1 + _fibo2;
        _fibo1 = _fibo2;
        _fibo2 = next;
        println!("{}", _fibo2);
    }
}
