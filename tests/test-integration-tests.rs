// unit_test.rs
// Conditionally compile `main` only when the test-suite is *not* being run.
#[cfg(not(test))]
fn main() {
    println!("If you see this, the tests were not compiled nor ran!");
}

// Conditionally compile the module `test` only when the test-suite is run.
#[cfg(test)]
mod test {

    #[test]
    fn pass_test() {
    }

    #[test]
    #[should_panic]
    fn failing_test() {
        assert!(1i32 == 2i32);
    }
}

