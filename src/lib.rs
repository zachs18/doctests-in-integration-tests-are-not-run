#[test]
fn this_will_run() {
    println!("use `cargo test -- --nocapture` to see this");
}

/// ```compile_fail
/// compile_error!("This doctest will correctly fail");
/// ```
pub struct ThisStructDoctestWillNotRun;
