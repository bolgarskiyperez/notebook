use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde_json::{self, Result};
use crate::models::task::Task;

pub struct TaskStorage;

impl TaskStorage {
    pub fn read_tasks(file_path: &str) -> Result<Vec<Task>> {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => return Ok(Vec::new()), 
            // if file not found err return clear new task list, if Ok return variable file with data
        };

        let mut data = String::new();
        file.read_to_string(&mut data);
        //? for error processing !!!

        let tasks: Vec<Task> = serde_json::from_str(&data)?;
        Ok(tasks)
        // file -> data(by File::open(file_path)) -> tasks(by serde_json)
    }

    pub fn write_tasks(file_path: &str, tasks: &Vec<Task>) -> Result<()> {
        let serialized = serde_json::to_string_pretty(tasks)?;
        //serde_json Vec -> json ; to_string_pretty for data formattting
        let file = OpenOptions::new()
            .write(true) //open file
            .create(true) //create if its none
            .truncate(true) //if file exists, its be cleared before recording
            .open(file_path);//open file 

        let mut data = match file {
            Ok(data) => data,
            Err(_e) => panic!("sdfdsf")
        };

         data.write_all(serialized.as_bytes());// record to file serializeds data as bytes

        //write_all work strong as bytes
        Ok(())
    }
}
//file will be created automatically from Main if it is not