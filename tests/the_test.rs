#[test]
fn this_will_run() {
    println!("use `cargo test -- --nocapture` to see this");
}

/// ```
/// compile_error!("This error message will not be printed");
/// ```
pub struct ThisStructDoctestWillNotRun;
