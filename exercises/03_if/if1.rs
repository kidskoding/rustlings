fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }

    b
}

fn main() {
    let a = 42;
    let b = 42;
    let c = 38;

    println!("the bigger number between a={} and b={} is {}", a, b, bigger(a, b));
    println!("the bigger number between b={} and c={} is {}", b, c, bigger(b, c));
    println!("the bigger number between a={} and c={} is {}", a, c, bigger(a, c));
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
