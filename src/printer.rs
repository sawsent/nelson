use bat::PrettyPrinter;

pub fn print_with_bat(text: &str) {
    let _ = PrettyPrinter::new()
        .input_from_bytes(text.as_bytes())
        .language("md")
        .grid(true)
        .paging_mode(bat::PagingMode::Never)
        .print();
}
