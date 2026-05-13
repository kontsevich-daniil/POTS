use std::fmt;

#[derive(Debug, PartialEq)]
enum CalcError {
    DivisionByZero,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::DivisionByZero => write!(f, "Ошибка: деление на ноль недопустимо"),
        }
    }
}

fn calculate(a: f64, b: f64, op: &str) -> Result<f64, CalcError> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err(CalcError::DivisionByZero)
            } else {
                Ok(a / b)
            }
        },
        _ => panic!("Неподдерживаемая операция"),
    }
}

fn main() {
    let cases = vec![
        (10.0, 2.0, "/"),
        (10.0, 0.0, "/"),
        (5.0, 3.0, "+"),
    ];

    for (a, b, op) in cases {
        match calculate(a, b, op) {
            Ok(res) => println!("{} {} {} = {}", a, op, b, res),
            Err(e) => println!("При вычислении {} {} {} произошла ошибка: {}", a, op, b, e),
        }
    }
}
