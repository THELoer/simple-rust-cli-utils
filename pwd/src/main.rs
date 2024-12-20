use std::env;

fn main() {
    // Получаем текущую рабочую директорию
    match env::current_dir() {
        Ok(path) => {
            // Выводим путь
            println!("{}", path.display());
        }
        Err(e) => {
            // Обработка ошибки, если не удаётся получить текущую директорию
            eprintln!("Error: {}", e);
        }
    }
}
