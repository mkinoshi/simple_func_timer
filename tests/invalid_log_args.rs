use simple_func_timer::timer;

#[timer(log = "hoge")]
fn sum(max: i32) -> i32 {
    let mut sum = 0;
    for i in 1..max {
        sum += i;
    }
    return sum;
}

fn main() {}