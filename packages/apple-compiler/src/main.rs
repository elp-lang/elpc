extern "C" {
    fn swift_function_swift() -> u32;
    fn swift_string_swift() -> ;
}

fn main() {
    let int = unsafe { swift_function_swift() };
    println!("{}", int);

    let string = unsafe { swift_string_swift() };
    println!("{}", string);
}
