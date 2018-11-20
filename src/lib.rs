#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicBool};

    pub struct MyComplexStruct {
        pub read_write_with_mutex: bool,
        pub read_atomic_write_mutex: AtomicBool,
        pub read_atomic_write_atomic: AtomicBool,
    }

    impl MyComplexStruct {
        pub fn new() -> MyComplexStruct {
            MyComplexStruct {
                read_write_with_mutex: bool,
                read_atomic_write_mutex: AtomicBool::new(false),
                read_atomic_write_atomic: AtomicBool::new(false),
            }
        }
    }

    #[test]
    fn it_works() {
        let myComplexStruct = MyComplexStruct::new();
        writes_on_mutex(&myComplexStruct);
        writes_on_atomic(&myComplexStruct);
        reads_on_atomic(&myComplexStruct);
    }

    fn writes_on_mutex(str: &MyComplexStruct) {

    }

    fn writes_on_atomic(str: &MyComplexStruct) {

    }

    fn reads_on_atomic(str: &MyComplexStruct) {

    }
}
