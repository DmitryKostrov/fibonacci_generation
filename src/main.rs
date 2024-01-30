use std::io;

// Генерирование n-го числа Фибоначчи.
// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946,….   ;)
fn main() {
    println!("Enter Fibonacci number"); // Сообщаем пользователю что нужно ввести n-ое число фибоначчи.
    let mut input_user: String = String::new(); // создаем переменную для ввода пользователя
    io::stdin() // просим ввести что нибудь
        .read_line(&mut input_user)// записываем номер числа пользователя
        .expect("Failed to read line"); // в случае ошибки ввода выводим сообщение в консоль
    let input_user: i32 = input_user.trim().parse().expect("\n!!!Enter a positive number!!!\n\n");// конвертируем строку в число

    println!("\n"); // Создаем пустое место в консоли чтобы ввод пользователя не смешалось с результатом выполнения программы
    // проверяем ввод пользователя

    if input_user >= 1 { // Проверка числа положительное оно или нет

        println!("0");
        println!("1");

        let mut fibo_one: u64 = 0;
        let mut fibo_two: u64= 1;
        for _num in 1..input_user {

            let next: u64 = fibo_one + fibo_two; // следующее значение
            fibo_one = fibo_two;
            fibo_two = next;
            println!("{}", fibo_two);
        }
    }
    else { // если число отрицателььное выводим 0 с вопросом
        println!("0");
        println!("Are you sure you entered a positive number?");
    }
}
