mod test_utils;

use simple_func_timer::timer;

#[timer]
fn sum(max: i32) -> i32 {
    let mut sum = 0;
    for i in 1..max {
        sum += i;
    }
    return sum;
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    use log::Level;
    use test_utils::utils::{setup, validate};

    #[test]
    fn test_sum() {
        setup();
        sum(1000);
        validate(|logs| {
            assert_eq!(logs[0].level, Level::Debug);
            assert!(logs[0].body.contains("ms"));
            assert!(logs[0].body.contains("sum"));
        });
    }
}
