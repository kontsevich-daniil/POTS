pub fn is_luhn_valid(card_number: &str) -> bool {
    // Убираем пробелы и проверяем длину (минимум 2 цифры)
    let digits: Vec<u32> = card_number
        .chars()
        .filter(|c| !c.is_whitespace())
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() < 2 {
        return false;
    }

    let sum: u32 = digits
        .iter()
        .rev() // Считаем справа налево
        .enumerate()
        .map(|(idx, &digit)| {
            if idx % 2 == 1 {
                // Каждая вторая цифра удваивается
                let doubled = digit * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            } else {
                digit
            }
        })
        .sum();

    sum % 10 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_card() {
        assert!(is_luhn_valid("79927398713")); // Пример валидного номера
    }

    #[test]
    fn test_invalid_card() {
        assert!(!is_luhn_valid("79927398710"));
    }

    #[test]
    fn test_with_spaces() {
        assert!(is_luhn_valid("7992 7398 713"));
    }

    #[test]
    fn test_too_short() {
        assert!(!is_luhn_valid("1"));
        assert!(!is_luhn_valid(""));
    }
}
