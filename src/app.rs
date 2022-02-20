use chrono::{Datelike, Utc};
use clap::{Parser, Subcommand};
use csv::{ReaderBuilder, Writer};
use prettytable::{
    format::{self, FormatBuilder},
    Table,
};
use std::fs::{self, File};
use std::path::Path;
#[derive(Parser)]
#[clap(
    author = "A simple CLI Application to store your expense information",
    version,
    about
)]
#[clap(name = "Record Keeper")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}
#[derive(Subcommand)]
enum Command {
    ///Adds an entry to the record table
    Add {
        ///Description of the entry
        #[clap(short = 'd', long = "description", value_name = "DESCRIPTION")]
        desc: String,
        ///Expense
        #[clap(short = 'v', long = "value", value_name = "VALUE")]
        val: f64,
    },
    ///Deletes an entry from the record table
    Delete {
        ///Index of the entry to delete
        #[clap(short = 'i', long = "index", value_name = "INDEX")]
        index: usize,
    },
    /// Displays the current record table
    Display,
    /// Prints the stats related to the current month
    Stats{
        #[clap(short = 'd', long = "date",value_name = "DATE", default_value = "")]
        date : String,
    },
}
#[allow(dead_code)]
struct App {
    table: Table,
    path: String,
}
#[derive(Default)]
struct Stats {
    max: f64,
    min: f64,
    avg: f64,
    total: f64,
}
impl std::fmt::Display for Stats{
    fn fmt(&self,fmt : &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error>{
        write!(fmt, "Minimum Expenditure : {}\nMaximum Expenditure : {}\nTotal Expenditure : {}\nAverage Expenditure : {}\n", self.min, self.max, self.total, self.avg)
    }
}
impl App {
    fn new() -> Self {
        let path = get_path();
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_path(&path)
            .expect("Create CSV Reader");
        let mut table = Table::from_csv(&mut reader);
        let format = FormatBuilder::new()
            .column_separator('│')
            .borders('│')
            .separators(
                &[format::LinePosition::Title],
                format::LineSeparator::new('─', '┼', '├', '┤'),
            )
            .padding(1, 1)
            .build();
        table.set_format(format);
        table.set_titles(row![bFg -> "Index", bFg -> "Date", bFg -> "Description", bFg ->"Value"]);
        Self { table, path }
    }
    fn save(&self) {
        let writer = Writer::from_path(&self.path).expect("Creating a CSV writer");
        let _ = self.table.to_csv_writer(writer);
    }
    fn stats_for_date(&self, date : &String) -> Stats {
        let mut min = 0.0;
        let mut max = 0.0;
        let mut avg = 0.0;
        let mut total = 0.0;
        min = self.table.get_row(0).unwrap().get_cell(3).unwrap().get_content().parse::<f64>().unwrap();
        avg = total /self.table.len() as f64;
        for row in self.table.row_iter(){
            let row_date = row.get_cell(1).unwrap().get_content();
            if row_date.eq(date) {
                let row_val = row.get_cell(3).unwrap().get_content().parse::<f64>().unwrap();
                total += row_val;
                if row_val < min {min = row_val}
                if row_val > max {max = row_val}
            }
        }
        avg = total / self.table.len() as f64;
        Stats{
            min,
            max,
            avg,
            total
        }
    }
    fn stats(&self) -> Stats {
        let mut min : f64 = 0.0;
        let mut max : f64 = 0.0;
        let avg : f64;
        let mut total = 0.0;
        min = self.table.get_row(0).unwrap().get_cell(3).unwrap().get_content().parse::<f64>().unwrap();
        for cell in self.table.column_iter(3) {
            let val = cell.get_content().parse::<f64>().unwrap();
            total +=  val;
            if  val > max { max = val}
            if val < min { min = val}
        }
        avg = total /self.table.len() as f64;
        Stats{
            min,
            max,
            avg,
            total
        }
    }
}
fn get_date() -> String {
    let now = Utc::now();
    let date = format!("{:02}-{:02}-{:04}", now.day(), now.month(), now.year());
    date
}
fn get_path() -> String {
    let now = Utc::now();
    let home = env!("HOME");
    let path = format!("{}/.rcdata/{:02}-{:04}", home, now.month(), now.year());
    path
}
pub fn start() {
    initialize();
    let mut app = App::new();
    let cli = Cli::parse();
    match cli.command {
        Some(Command::Add { desc, val }) => {
            app.table
                .add_row(row![app.table.len() + 1, get_date(), desc, val]);
        }
        Some(Command::Delete { index }) => app.table.remove_row(index - 1),
        Some(Command::Display) => {
            if app.table.is_empty() {
                println!("No records inserted yet.");
                println!("To insert a record, use the `add` command.");
            } else {
                app.table.printstd();
            }
        }
        Some(Command::Stats { date}) => {
            if app.table.is_empty() {
                println!("No records inserted yet.");
                println!("To insert a record, use the `add` command.");
            } else if date.is_empty(){
                println!("{}", app.stats());
            }
            else {
                println!("{}", app.stats_for_date(&date));
            }
        }
        _ => (),
    }
    app.save();
}
fn initialize() {
    let dir = format!("{}/.rcdata/", env!("HOME"));
    if !Path::exists(Path::new(&dir)) {
        fs::create_dir(dir).expect("Create Directory");
    }
    let path = get_path();
    if !Path::new(&path).exists() {
        File::create(&path).expect("Create Record File");
    }
}
