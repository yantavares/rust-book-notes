pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn divide(left: usize, right: usize) -> Result<usize, String> {
    if right == 0 {
        return Err(String::from("Cannot divide by zero"));
    }
    Ok(left / right)
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    #[allow(dead_code)]
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[allow(dead_code)]
fn greetings(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    // This line brings everything from the outer module into scope.
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn greetings_contains_name() {
        let result = greetings("Carol");
        // Custom error message
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        let result = add(100, 1);
        assert_eq!(result, 1);
    }

    // We can also make sure that a program panics with a specific message.
    // If any other panic occurs, the test will fail.
    #[test]
    #[should_panic(expected = "assertion `left == right` failed\n  left: 101\n right: 1")]
    fn should_panic_with_message() {
        let result = add(100, 1);
        assert_eq!(result, 1);
    }

    // Tests can return Result<T, E> to handle recoverable errors.
    #[test]
    fn it_works_with_result() -> Result<(), String> {
        if add(2, 2) == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus two does not equal four"))
        }
    }

    #[test]
    fn divide_by_zero() -> Result<(), String> {
        let result = divide(10, 0);
        if result.is_err() {
            return Ok(());
        }
        Err(String::from("Test failed"))
    }
}
