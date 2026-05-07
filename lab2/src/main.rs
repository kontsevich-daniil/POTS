fn collatz_length(mut n: i32) -> u32 {
    let mut length = 1; // начинаем счёт с начального числа
    while n != 1 {
        if n % 2 == 0 {
            n /= 2; // чётное
        } else {
            n = 3 * n + 1; // нечётное
        }
        length += 1; // увеличиваем счётчик после каждого шага
    }
    length
}

fn main() {
    println!("Длина: {}", collatz_length(15));
}