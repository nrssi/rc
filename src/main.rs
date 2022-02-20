#[macro_use]
extern crate prettytable;
extern crate csv;
extern crate clap;
extern crate chrono;
pub mod app;
fn main(){
    app::start();
}
