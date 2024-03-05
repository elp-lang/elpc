extern "C" {
    fn swift_function_swift();
}

fn main() {
    unsafe {
        swift_function_swift();
    }
}
