#[macro_use]
extern crate prettytable;
extern crate csv;
extern crate clap;
extern crate chrono;
use csv::Writer;
use clap::{Parser, Subcommand};
use chrono::{Utc, Datelike};
use prettytable::{Table, format::{self, FormatBuilder}};
#[derive(Parser)]
#[clap(author="A simple CLI Application to store your expense information", version, about, long_about = None)]
struct Cli{
    #[clap(subcommand)]
    command : Option<Command>
}
#[derive(Subcommand)]
enum Command{
    ///Adds an entry to the record table
    Add{
        ///Description of the entry
        #[clap(short='d', long="description", value_name = "DESCRIPTION")]
        desc : String, 
        ///Expense 
        #[clap(short='v', long="value", value_name = "VALUE")]
        val : f64
    }, 
    Display
}
#[allow(dead_code)]
struct App{
    table : Table,
    path : String
}
impl App {
    fn new() -> Self {
        let path = get_path();
        let mut table = Table::from_csv_file(&path).unwrap_or(Table::new());
        let format = FormatBuilder::new()
        .column_separator('│')
        .borders('│')
        .separators(&[format::LinePosition::Title], format::LineSeparator::new('─', '┼', '├', '┤'))
        .padding(1,1)
        .build();
        table.set_format(format);
        table.set_titles(row![bFg -> "Description", bFg ->"Value"]);
        Self{
            table, 
            path
        }
    }
    fn save(&self){
        let writer = Writer::from_path(&self.path).expect("Creating a CSV writer");
        let _ = self.table.to_csv_writer(writer);
    }
}
fn main(){
    let mut app = App::new();
    let cli = Cli::parse();
    match &cli.command{
        Some(Command::Add{desc, val}) => {
            app.table.add_row(row![desc, val]);
            app.save();
        },
        Some(Command::Display) => {app.table.printstd();},
        _ => ()
    }
}
fn get_path() -> String{
    let now = Utc::now();
    let home = env!("HOME");
    let path = format!("{}/.rcdata/{:02}-{:02}-{:04}", home, now.day(), now.month(), now.year());
    path
}