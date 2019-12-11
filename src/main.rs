mod chapter1;
mod chapter4;
mod playground;

fn main() {
    chapter1::section2::formatted_print();
    chapter1::section2::debug_printable();
    chapter1::section2::display_printable();
    chapter1::section2::vector_printable();
    chapter4::ownership::test();

    playground::calculator::eval("1 - 2 * 3 + 4 * 5");
}
