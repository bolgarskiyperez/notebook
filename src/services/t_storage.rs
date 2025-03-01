use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde_json::{self};
use crate::models::task::Task;
use std::error::Error; // Для работы с ошибками типа Box<dyn Error>

pub struct TaskStorage;

impl TaskStorage {
    // Функция для чтения задач из файла
    pub fn read_tasks(&self, file_path: &str) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => return Err(Box::new(e)), // Ошибка открытия файла оборачиваем в Box
        };

        let mut data = String::new();
        file.read_to_string(&mut data)?; // Используем ? для работы с ошибками

        let tasks: Vec<Task> = serde_json::from_str(&data)?; // Сериализация JSON
        Ok(tasks)
    }

    // Функция для записи задач в файл
    pub fn write_tasks(&mut self, file_path: &str, tasks: &Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = serde_json::to_string_pretty(tasks)?; // Сериализуем задачи в формат JSON

        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path) {
            Ok(file) => file,
            Err(e) => return Err(Box::new(e)), // Ошибка открытия файла оборачиваем в Box
        };

        file.write_all(serialized.as_bytes())?; // Записываем данные в файл

        Ok(())
    }
}
