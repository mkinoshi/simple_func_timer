# simple_func_timer
![Rust](https://github.com/mkinoshi/simple_func_timer/actions/workflows/release.yml/badge.svg)
[![Crates](https://img.shields.io/crates/v/simple_func_timer.svg)](https://crates.io/crates/simple_func_timer)
[![Docs](https://docs.rs/simple_func_timer/badge.svg)](https://docs.rs/simple_func_timer)

Tiny macro to measure the function execution time in Rust. It uses `log` crate, so the consuming code can handle the log easily.

## Usage
Add these two crates to your Cargo.toml.
```
[dependencies]
simple_func_timer = "0.1"
log = "0.4"
```

## Examples
You can measure the execution time of a function in the following way. Your client code needs to consume the log.
```
#[timer(unit = "ns", log = "info")]
fn my_function() {
}

// It measures the execution time in milliseconds and generate `debug` level logs by default.
#[timer]
fn my_function() {
}
```

Here is an example of consuming the generated logs with [`simple_logger`](https://crates.io/crates/simple_logger)

```
use simple_logger::SimpleLogger;
use simple_func_timer::timer;

#[timer(unit = "ns", log = "info")]
fn my_function() {
    println!("Hello world");
}

fn main() {
    SimpleLogger::new().init().unwrap();
}
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate. Enjoy!

## License

[MIT](https://choosealicense.com/licenses/mit/)
