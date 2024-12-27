use chrono::{DateTime, Datelike, Local, NaiveDate};
use colored::*;
use std::fs::read_to_string;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "schedule", about = "Weekly schedule tracker")]
enum Cli {
    #[structopt(name = "thisWeek")]
    ThisWeek,
    #[structopt(name = "thisMonth")]
    ThisMonth,
    #[structopt(name = "thisQuarter")]
    ThisQuarter,
}

struct WeekEntry {
    date: NaiveDate,
    description: String,
}

fn parse_week_entries() -> Vec<WeekEntry> {
    let mut home = PathBuf::from(env!("HOME"));
    home.push(".files");
    home.push("weeks");
    
    let content = read_to_string(home).expect("Could not read weeks file");
    
    content
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(" - ").collect();
            if parts.len() != 2 {
                return None;
            }
            
            let date = NaiveDate::parse_from_str(parts[0], "%m/%d/%y")
                .ok()?;
                
            Some(WeekEntry {
                date,
                description: parts[1].to_string(),
            })
        })
        .collect()
}

fn get_current_week() -> NaiveDate {
    let now: DateTime<Local> = Local::now();
    let weekday = now.weekday().num_days_from_monday();
    now.date_naive() - chrono::Duration::days(weekday as i64)
}

fn print_week_entry(entry: &WeekEntry, current_week: NaiveDate) {
    let week_diff = (entry.date - current_week).num_weeks();
    
    let formatted = format!("{} - {}", entry.date.format("%m/%d/%y"), entry.description);
    
    match week_diff {
        1 => println!("{}", formatted.white()),
        0 => println!("{}", formatted.green()),
        -1 => println!("{}", formatted.white()),
        _ => println!("{}", formatted.normal()),
    }
}

fn print_quarter_entry(){
    let mut home = PathBuf::from(env!("HOME"));
    home.push(".files");
    home.push("plan");
    let content = read_to_string(home).expect("Could not read plan file");
    let all_quarters:Vec<_> = content.split("\n\n").collect();
    let this_quarter = all_quarters[0];
    println!("{}",this_quarter);
}

fn main() {
    let entries = parse_week_entries();
    let current_week = get_current_week();
    
    if std::env::args().len() == 1 {
        for entry in entries.iter() {
            print_week_entry(entry, current_week);
        }
    } else {
        match Cli::from_args() {
            Cli::ThisWeek => {
                for entry in entries.iter() {
                    let week_diff = (entry.date - current_week).num_weeks().abs();
                    if week_diff <= 1 {
                        print_week_entry(entry, current_week);
                    }
                }
            }
            Cli::ThisMonth => {
                let current_month = current_week.month();
                for entry in entries.iter() {
                    if entry.date.month() == current_month {
                        print_week_entry(entry, current_week);
                    }
                }
            }
            Cli::ThisQuarter => {
                print_quarter_entry();
            }
        }
    }
}