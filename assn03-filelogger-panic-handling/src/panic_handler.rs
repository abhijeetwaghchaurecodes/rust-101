

pub fn simulate_abort_mode() {
    std::process::abort();
}

pub fn simulate_unwind_mode() {
    panic!("This is a simulated panic in unwind mode!");
}
