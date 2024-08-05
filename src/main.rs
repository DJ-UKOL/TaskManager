// Менеджер заданий

use chrono::{DateTime, Local};

// Приритетность задания
enum Priority {
    Low,
    Medium,
    High
}

// Имплементация для структуры Priority
impl Priority {

    // Функция для преобразования данных в строку
    fn to_string(&self) -> String {
        match self {
            Priority::Low => "Low".to_owned(),
            Priority::Medium => "Medium".to_owned(),
            Priority::High => "High".to_owned()
        }
    }
}

struct Task {
    name: String,               // Имя задания
    description: String,        // Описание
    priority: Priority,         // приоритет
    add_time: DateTime<Local>   // время добавления
}

// Имплементация для структуры Task
impl Task {
    // метод для создания новой задачи
    fn new(name: String, description: String, priority: Priority) -> Self {
        Self { name, description, priority, add_time: Local::now() }
    }

    // Метод для печати информации о задаче
    fn print_task(&self) {
        println!(
            "{} | {} | {}\n\"{}\"\n",
            self.name,
            self.priority.to_string(),
            self.add_time.format("%d-%m-%Y %H:%M:%S"),
            self.description
        );
    }
}

fn main() {
    // Создаем новую задачу
    let task = Task::new(
        "To learn Rust".to_owned(),
        "I need it...".to_owned(),
        Priority::High,
    );

    task.print_task();
}
