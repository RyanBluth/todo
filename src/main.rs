#[macro_use]
extern crate failure;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate clap;
extern crate chrono;
extern crate term_table;

mod todo;

use clap::{App, Arg, SubCommand, AppSettings};
use failure::Error;
use todo::{ToDo, ToDoCollection};
use term_table::{Table, TableStyle};
use term_table::cell::Cell;
use term_table::row::Row;

const CONTENT_KEY: &'static str = "Content";

fn main() -> Result<(), Error>{
    let matches = App::new("managed-alias")
        .version("1.0")
        .author("Ryan Bluth")
        .setting(AppSettings::ArgsNegateSubcommands)
        .arg(
           Arg::with_name("Content")
                        .help("Todo content")
                        .required(false)
                        .multiple(true)
                        .index(1)
                        .allow_hyphen_values(true))
        .subcommand(
            SubCommand::with_name("delete")
                        .alias("d")
                        .arg(
                            Arg::with_name("IDs")
                            .help("IDs of the items to remove")
                            .index(1)
                            .required(true)
                            .multiple(true)
                        )
        )
        .get_matches();

    if let Some(values) = matches.values_of(CONTENT_KEY){
        let mut collection = ToDoCollection::load()?;
        let mut content = values.fold(String::new(), |mut acc, s| {acc.push_str(s); acc.push(' '); acc});
        content.pop();
        collection.add(ToDo::new(content));
        collection.save()?;
    }else if let Some(submatches) = matches.subcommand_matches("delete"){
        let ids = submatches.values_of("IDs").unwrap();
        let mut collection = ToDoCollection::load()?;
        for id in ids{
            let idx = id.parse::<i32>()?;
            collection.remove(idx - 1)?;
        }
        collection.save()?;
    }else{
        let mut collection = ToDoCollection::load()?;
        let mut table = Table::new();
        table.style = TableStyle::thin();
        table.add_row(Row::new(vec![
            Cell::new("ID", 1),
            Cell::new("Content", 1),
            Cell::new("Created", 1),
        ]));
        let mut id:usize = 1;
        for todo in collection.iter(){
            let row = Row::new(vec![
                Cell::new(id, 1),
                Cell::new(&todo.content, 1),
                Cell::new(&todo.created.format("%Y-%m-%d %H:%M:%S").to_string(), 1),
            ]);
            table.add_row(row);
            id += 1;
        }
        println!("{}", table.as_string());
    }
    
    Ok(())
}
