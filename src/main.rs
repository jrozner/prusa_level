use lazy_static::lazy_static;
use prettytable::Table;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Read;
use std::process;
use std::vec::Vec;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(-?\d+\.\d+)").unwrap();
}

fn main() -> Result<(), std::io::Error> {
    if env::args().len() != 2 {
        println!("no input file specified");
        process::exit(1)
    }

    let mut file = File::open(env::args().nth(1).unwrap())?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let rows: Vec<Vec<f32>> = buf
        .lines()
        .map(|line| {
            RE.find_iter(line)
                .map(|col| col.as_str().parse::<f32>().unwrap())
                .collect::<Vec<f32>>()
        })
        .collect();

    let center_row = rows.len() / 2;
    let center_column = rows[center_row].len() / 2;
    let center_value = rows[center_row][center_column];

    let mut table = Table::new();
    rows.iter().for_each(|row| {
        let row = row.iter().map(|col| col - center_value).collect();
        table.add_row(row);
    });

    table.printstd();

    Ok(())
}
