use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs::{metadata, File, OpenOptions};
use std::io::prelude::*;
use serde_json::{from_str, to_string_pretty};
use failure::Error;
use todo::error::ToDoError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToDo{
pub content: String,
    pub created: DateTime<Local>,
}

impl ToDo{
    pub fn new(content:String)->ToDo{
        return ToDo{
            content: content,
            created: Local::now()      
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToDoCollection{
    store: Vec<ToDo>,
}

impl ToDoCollection{

    pub fn new() -> ToDoCollection{
        return ToDoCollection{
            store: Vec::new()
        }
    }

    pub fn load() -> Result<ToDoCollection, Error>{
        let file_result = File::open("todo.json");
        if file_result.is_ok(){
            let mut file = file_result.unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data)?;

            let collection:ToDoCollection = from_str(&data)?;
            return Ok(collection);
        }else{
            return Ok(ToDoCollection::new());
        }
    }

    pub fn save(&self)->Result<(), Error>{
        let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .read(true)
        .open("todo.json")?;

        let collection_str = to_string_pretty(self)?;
        file.write_all(collection_str.as_bytes())?;
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &ToDo>{
        return self.store.iter();
    }

    pub fn add(&mut self, todo:ToDo){
        self.store.push(todo);
    }

    pub fn remove(&mut self, index:i32)->Result<(), Error>{
        if index >= 0 && self.store.len() > index as usize{
            self.store.remove(index as usize);
        }else{
            return Err(ToDoError::InvalidIDError{
                id:index as usize + 1,
                id_start: 1,
                id_end: self.store.len()
            }.into());
        }
        Ok(())
    }
}
