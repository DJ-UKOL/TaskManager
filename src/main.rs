use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
// Менеджер заданий
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

// Приритетность задания
#[derive(Serialize, Deserialize)]
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

// Структура задание
#[derive(Serialize, Deserialize)]
struct Task {
    name: String,               // имя задания
    description: String,        // описание
    priority: Priority,         // приоритет
    add_time: DateTime<Local>   // время добавления
}

// Имплементация для структуры Task
impl Task {
    // метод для создания новой задачи
    fn new(name: String, description: String, priority: Priority) -> Self {         // Self в возвращаемом типе и в теле функции являются псевдонимами
        Self { name, description, priority, add_time: Local::now() }                // для типа, указанного после ключевого слова impl,
    }                                                                               // которым в данном случае является Task

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

    fn new_from_console() -> Self {
        let name = ConsoleManager::input("Enter new task name: ").unwrap();
        let description = ConsoleManager::input("Enter new task description: ").unwrap();
        let priority = match ConsoleManager::input("Enter new task priority: ").unwrap().to_lowercase().as_str()
        {
                "low" => Priority::Low,
                "medium" => Priority::Medium,
                "high" => Priority::High,
                _ => {
                    println!("Invalid priority, setting to low!");
                    Priority::Low
                }
        };
        Self::new(name, description, priority)
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
        Self { tasks: vec![]}       // Создаем вектор, где будем хранить задачи
    }

    // Добавление задачи
    fn add_task(&mut self, task: Task)         // в метод предаем первым параметром вектор мутабельный,
    {                                           // вторым параметром саму задачу, которую нужно добавить в вектор
        self.tasks.push(task);                  // методом push добавляем задачу в вектор
    }

    // Вывод информации о задачах которые есть в векторе
    fn print_tasks(&self) {
        for task in &self.tasks {       // проходим по вектору, по каждой заадаче
            task.print_task();                 // выводим информацию методом который определен в Task
        }
    }

    // Удаление задачи из вектора
    fn remove_task(&mut self, name: &str) -> Result<String, String> {       // принимает ссылку на вектор, вторым параметором имя,
        if let Some(index) = self.find_task(name) {                   // передаем имя в метод, чтобы найти индекс объекта
            self.tasks.remove(index);                                       // удаляем объект из вектора по индексу
            Ok(format!("Task \"{}\" removed successfully", name))
        } else {
            Err(format!("Task with name \"{}\" doesn't exist", name))
        }
    }

    // Поиск задачи по имени
    fn find_task(&self, name: &str) -> Option<usize> {                      // передаем вектор, и имя типа &str для того чтобы заимствовать, а не владеть
        self.tasks.iter().position(|task| task.name == name)         // в векторе вызываем итератор, position принимает замыкание
    }                                                                       // и возвращает Some(index) если имя найдено, или None если нет

    // Редактирование задачи
    fn edit_task(&mut self, name: &str, updated_task: Task) -> Result<String, String> {      // передаем вектор, имя и саму задачу с полями которые нужно изменить
        if let Some(index) = self.find_task(name) {                                    // если задача по имени найдена
            match self.tasks.get_mut(index) {                                                // получаем задачу по индексу
                None => Err("Error borrowing task".to_owned()),                              // если возвращается None, то это ошибка заимствования
                Some(task) => {                                                    // если возвращается Some, то меняем
                    task.name = updated_task.name;                                           // имя
                    task.description = updated_task.description;                             // описание
                    task.priority = updated_task.priority;                                   // приоритет
                    Ok(format!("Task \"{}\" updated successfully", name))                    // выводим, что задача изменилась успешно
                }
            }
        } else {
            Err(format!("Task with name \"{}\" doesn't exist", name))                        // если задача не найдена, то выводим, что имя не найдено
        }
    }

    // Сохраняем в файл
    fn store_to_fail(&self, filename: &str) -> Result<String, String> {
        // Если файл не существует
        if !Path::new(filename).exists() {
            let file = match File::create(filename) {
                Ok(file) => file,
                Err(err) => return Err(format!("Error creating file: {}", err))
            };

            // Запись в файл в формате json
            match serde_json::to_writer(&file, &self.tasks) {
                Ok(_) => Ok("Data stored successfully".to_owned()),
                Err(err) => Err(format!("Error saving data: {}", err))
            }
        } else {    // Если файл существует
            Err("File \"{filename}\" already exists".to_owned())
        }
    }

    fn read_from_file(&mut self, filename: &str) -> Result<String, String> {
        // Если файл существует
        if Path::new(filename).exists() {
            let file = match File::open(filename) {
                Ok(file) => file,
                Err(err) => return Err(format!("Error creating file: {}", err))
            };
            let reader = BufReader::new(file);
            self.tasks = match serde_json::from_reader(reader) {
                    Ok(data) => data,
                    Err(err) => return Err(format!("Error reading file: {}", err))
            };

            Ok("Data read successfully".to_owned())
        } else {
            Err("File \"(filename)\" doesn't exist".to_owned())
        }
    }
}

// Структура взаимодействия с консоль
struct ConsoleManager {
    tasks_managers: TasksManagers,          // для взаимодействия с менеджером задач
    menu_options: Vec<String>               // вектор строк для меню
}

// Методы для взаимодйствия с консолью
impl ConsoleManager {
    // Меню
    fn new() -> Self {
        Self {
            tasks_managers: TasksManagers::new(),
            menu_options: vec![
                "Add task".to_owned(),
                "Find task".to_owned(),
                "Edit task".to_owned(),
                "Remove task".to_owned(),
                "Print all tasks".to_owned(),
                "Store tasks to file".to_owned(),
                "Read tasks from file".to_owned()
            ]
        }
    }

    // Вывод меню
    fn print_menu(&self) {
        for(index, menu_option) in self.menu_options.iter().enumerate() {
            println!("{}. {}", index+1, menu_option);
        }
    }

    // Метод ввода
    fn input(query: &str) -> std::io::Result<String> {      // возвращает Result из библиотеки std::io для работы с ошбками ввода и вывода
        print!("{}", query);
        std::io::stdout().flush()?;

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;           // получить текст ввода в buffer

        Ok(buffer.trim().to_owned())                        // Убираем пробелы и возвращаем строку
    }

    // Метод для приема и обработки комманд
    fn process_command(&mut self) {
        match Self::input("\nEnter command index: ") {
            Ok(command) => {
                match command.as_str() {
                    "1" => {    // add task
                        self.tasks_managers.add_task(Task::new_from_console());
                    }

                    "2" => {    // find task
                            let name = match Self::input("Enter task name to find: ") {
                                Ok(name) => name,
                                Err(err) => {
                                    println!("Error getting user input: {}", err);
                                    return;
                                }
                            };
                        match self.tasks_managers.find_task(name.as_str()) {
                            None => println!("Task with name \"{}\" doesn't exist", name),
                            Some(index) => {
                                println!("Task found");
                                self.tasks_managers.tasks.get(index).unwrap().print_task();
                            }
                        }

                    }

                    "3" => {    // edit task
                        let name = match Self::input("Enter task name to find: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };
                        match self.tasks_managers.edit_task(name.as_str(), Task::new_from_console()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg)
                        }
                    }

                    "4" => {        // remove task
                        let name = match Self::input("Enter task name to remove: ") {
                            Ok(name) => name,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };
                        match self.tasks_managers.remove_task(name.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg)
                        }
                    }

                    // Print all tasks
                    "5" => {
                        self.tasks_managers.print_tasks();
                    }

                    // Store task to file
                    "6" => {
                        let filename = match Self::input("Enter file name to store data in: ") {
                            Ok(filename) => filename,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };
                        match self.tasks_managers.store_to_fail(filename.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg)
                        }
                    }

                    // Read tasks from file
                    "7" => {
                        let filename = match Self::input("Enter file name to read data from: ") {
                            Ok(filename) => filename,
                            Err(err) => {
                                println!("Error getting user input: {}", err);
                                return;
                            }
                        };
                        match self.tasks_managers.read_from_file(filename.as_str()) {
                            Ok(msg) => println!("{}", msg),
                            Err(msg) => println!("{}", msg)
                        }
                    }
                    _ => { println!("Wrong input command!") }
                }
            }
            Err(err) => println!("Error getting user input: {err}")
        }
    }
}

fn main() {
    let mut manager = ConsoleManager::new();
    manager.print_menu();

    loop {
        manager.process_command();
        println!();
    }
}