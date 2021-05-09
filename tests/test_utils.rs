#[cfg(test)]
pub mod utils {
    use log::{Level, LevelFilter, Log, Metadata, Record};
    use std::cell::RefCell;
    use std::sync::Once;

    thread_local!(static LOG_RECORDS: RefCell<Vec<TestLog>> = RefCell::new(Vec::with_capacity(1)));

    pub struct TestLog {
        pub body: String,
        pub level: Level,
        pub target: String,
    }

    struct TestingLogger {}

    impl Log for TestingLogger {
        #[allow(unused_variables)]
        fn enabled(&self, metadata: &Metadata) -> bool {
            true // capture all log levels
        }

        fn log(&self, record: &Record) {
            LOG_RECORDS.with(|records| {
                println!("records: {:?}", record.args());
                let captured_record = TestLog {
                    body: format!("{}", record.args()),
                    level: record.level(),
                    target: record.target().to_string(),
                };
                records.borrow_mut().push(captured_record);
            });
        }

        fn flush(&self) {}
    }

    static TEST_LOGGER: TestingLogger = TestingLogger {};
    static FIRST_TEST: Once = Once::new();

    pub fn setup() {
        FIRST_TEST.call_once(|| {
            log::set_logger(&TEST_LOGGER)
                .map(|()| log::set_max_level(LevelFilter::Trace))
                .unwrap();
        });
        LOG_RECORDS.with(|records| {
            records.borrow_mut().truncate(0);
        });
    }

    pub fn validate<F>(asserter: F)
    where
        F: Fn(&Vec<TestLog>),
    {
        LOG_RECORDS.with(|records| {
            asserter(&records.borrow());
            records.borrow_mut().truncate(0);
        });
    }
}
