mod console_reader;
mod image_stacker;

fn main() {
    println!("Welcome to Fast Image Stacker!");
    println!("Version 0.1.0 (2025-04-11)");
    println!("Author: GarthTB <g-art-h@outlook.com>");
    println!("Repo: https://github.com/GarthTB/fast_image_stacker");

    fn exit_with_error<T>(message: &str) -> T {
        println!("Error: {message}\nExiting...");
        console_reader::read_line();
        std::process::exit(1);
    }

    let (image_paths, result_path) =
        console_reader::get_paths().unwrap_or_else(|e| exit_with_error(e));
    let mode = console_reader::get_mode();
    image_stacker::stack_and_save(image_paths, result_path, mode)
        .unwrap_or_else(|e| exit_with_error(e));

    println!("Done!");
    console_reader::read_line();
}
