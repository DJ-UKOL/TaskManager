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

// Структура менеджера задач
struct TasksManagers {
    tasks: Vec<Task>
}

// Функционал (добавление, удаление, поиск и т.д)
impl TasksManagers {

    // Создание
    fn new() -> Self {
        Self { tasks: vec![]}
    }

    // Добавление
    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    // Вывод информации о задачах
    fn print_tasks(&self) {
        for task in &self.tasks {
            task.print_task();
        }
    }

    // Удаление задачи
    fn remove_task(&mut self, name: &str) -> Result<String, String> {
        if let Some(index) = self.find_task(name) {
            self.tasks.remove(index);
            Ok(format!("Task \"{}\" removed successfully", name))
        } else {
            Err(format!("Task with name \"{}\" doesn't exist", name))
        }
    }

    // Поиск задачи
    fn find_task(&self, name: &str) -> Option<usize> {
        self.tasks.iter().position(|task| task.name == name)
    }

    // Редактирование
    fn edit_task(&mut self, name: &str, updated_task: Task) -> Result<String, String>{
        if let Some(index) = self.find_task(name) {
            match self.tasks.get_mut(index) {
                None => Err("Error borrowing task".to_owned()),
                Some(task) => {
                    task.name = updated_task.name;
                    task.description = updated_task.description;
                    task.priority = updated_task.priority;
                    Ok(format!("Task \"{}\" updated successfully", name))
                }
            }
        } else {
            Err(format!("Task with name \"{}\" doesn't exist", name))
        }
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