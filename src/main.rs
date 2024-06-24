use iced::widget::{button, Button, Text};

struct Calculator {
    value: i64,
    current_input: i64,
    error_message: Option<String>,
}

enum Buttons {
    Equals,
    Line,
    Number(i64),
}

struct Numbers;

enum Functions {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Functions {
    fn as_str(&self) -> &str {
        match self {
            Functions::Plus => "+",
            Functions::Minus => "-",
            Functions::Multiply => "*",
            Functions::Divide => "/",
        }
    }

    fn apply(&self, a: f64, b: f64) -> Option<f64> {
        match self {
            Functions::Plus => Some(a + b),
            Functions::Minus => Some(a - b),
            Functions::Multiply => Some(a * b),
            Functions::Divide => {
                if b != 0.0 {
                    Some(a / b)
                } else {
                    None
                }
            }
        }
    }
}

impl Buttons {}

impl Calculator {
    fn new() -> Self {
        Self {
            value: 0,
            current_input: 0,
            error_message: None,
        }
    }

    fn update(&mut self, line: i64, menu: Buttons) {
        match menu {
            Buttons::Line => {
                self.value = line;
            }
            Buttons::Number(num) => {
                self.current_input = num;
                self.value = num;
            }
            _ => {}
        }

        if self.value != line {
            self.error();
            return;
        }
    }

    fn output(&mut self, line: i64, menu: Buttons) {
        match menu {
            Buttons::Equals => {
                println!("{}", line);
            }
            _ => {}
        }
    }

    fn error(&mut self) {
        self.error_message = Some("Error: Value did not match the expected line.".to_string());
    }

    fn get_error_message(&self) -> Option<&String> {
        self.error_message.as_ref()
    }
}

fn main() {
    println!("Hello, world!");

    let minus: Button<Functions> = button(Text::new("-")).on_press(Functions::Minus);
    let plus: Button<Functions> = button(Text::new("+")).on_press(Functions::Plus);
    let mult: Button<Functions> = button(Text::new("*")).on_press(Functions::Multiply);
    let divide: Button<Functions> = button(Text::new("/")).on_press(Functions::Divide);
    let equals: Button<Buttons> = button(Text::new("=")).on_press(Buttons::Equals);

    let number_buttons: Vec<Button<Buttons>> = (0..10)
        .map(|n| button(Text::new(format!("{}", n))).on_press(Buttons::Number(n)))
        .collect();

    let mut all_buttons: Vec<Button<Buttons>> = vec![equals];
    all_buttons.extend(number_buttons);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_properly() {
        let mut calculator = Calculator::new();

        calculator.update(1, Buttons::Line);
        assert_eq!(calculator.value, 1);

        calculator.update(2, Buttons::Line);
        assert_eq!(calculator.value, 2);

        calculator.update(3, Buttons::Line);
        assert_eq!(calculator.value, 3);

        calculator.update(4, Buttons::Line);
        assert_eq!(calculator.value, 4);

        calculator.update(5, Buttons::Line);
        assert_eq!(calculator.value, 5);

        calculator.update(0, Buttons::Number(0));
        assert_eq!(calculator.value, 0);

        calculator.update(9, Buttons::Number(9));
        assert_eq!(calculator.value, 9);
    }

    #[test]
    fn it_applies_addition() {
        let result = Functions::Plus.apply(2.0, 3.0);
        assert_eq!(result, Some(5.0));
    }

    #[test]
    fn it_applies_subtraction() {
        let result = Functions::Minus.apply(5.0, 3.0);
        assert_eq!(result, Some(2.0));
    }

    #[test]
    fn it_applies_multiplication() {
        let result = Functions::Multiply.apply(2.0, 3.0);
        assert_eq!(result, Some(6.0));
    }

    #[test]
    fn it_applies_division() {
        let result = Functions::Divide.apply(6.0, 3.0);
        assert_eq!(result, Some(2.0));
    }

    #[test]
    fn it_handles_division_by_zero() {
        let result = Functions::Divide.apply(10.0, 0.0);
        assert_eq!(result, None);
    }

    #[test]
    fn it_handles_invalid_operation() {
        let mut calculator = Calculator::new();

        calculator.update(1, Buttons::Line);
        assert_eq!(calculator.value, 1);

        // Simulate an invalid operation (e.g., non-matching value update)
        calculator.update(2, Buttons::Line);
        calculator.error();
        assert_eq!(
            calculator.get_error_message(),
            Some(&"Error: Value did not match the expected line.".to_string())
        );
    }
}
