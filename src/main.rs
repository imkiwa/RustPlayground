mod chapter1;
mod chapter4;

fn main() {
    chapter1::section2::formatted_print();
    chapter1::section2::debug_printable();
    chapter1::section2::display_printable();
    chapter1::section2::vector_printable();
    chapter4::ownership::test();
}
