use std::env;
use rjvm::JavaClass;
use rjvm::class_printer::ClassPrinter;
use text_colorizer::Colorize;


fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let file_name = env::args().skip(1).next().expect("Java class should be passed as an argument");

    let class = JavaClass::load_from_file(file_name.as_str())?;
    println!("{}: {file_name}", "File".bold().blue());
    ClassPrinter::new(class).print();

    Ok(())
}
