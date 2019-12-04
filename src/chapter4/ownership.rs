fn show(s: String) {
    println!("ownership::show(): {}", s);
}

pub fn test() {
    let s = String::from("hello");
    show(s);
    // use of moved value
    // show(s);

    // which is equivalent to:
    // auto s = std::string("hello");
    // show(std::move(s));
}
