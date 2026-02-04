use chrono::{DateTime, Datelike, Utc};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_writer_pretty};
use std::{
    error,
    fs::{File, read_to_string},
    io::ErrorKind,
};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Expense {
    description: String,
    amount: f64,
    id: usize,
    date: DateTime<Utc>,
}
#[derive(Subcommand, Debug)]
enum Commands {
    Add { description: String, amount: f64 },
    Update { id: usize, amount: f64 },
    Delete { id: usize },
    View { month: Option<String> },
    Summary { month: Option<String> },
}
impl Commands {
    fn execute(self, expense: &mut Vec<Expense>) {
        match self {
            Commands::Add {
                description,
                amount,
            } => {
                let new_id = expense
                    .iter()
                    .max_by_key(|x| x.id)
                    .map(|x| x.id + 1)
                    .unwrap_or(1);
                expense.push(Expense {
                    description: description,
                    amount: amount,
                    id: new_id,
                    date: Utc::now(),
                });
                if let Err(e) = save_to_file(expense) {
                    eprintln!("There is an error in saving file: {}", e);
                } else {
                    println!("Expense added successfully");
                }
            }
            Commands::Delete { id } => {
                if let Some(indx) = expense.iter().position(|x| x.id == id) {
                    expense.remove(indx);
                    println!("Expense {} deleted successfully.", id);

                    if let Err(e) = save_to_file(expense) {
                        eprint!("There is an error in saving the file: {}", e)
                    } else {
                        println!("Expense deleted successfully");
                    }
                } else {
                    println!("ID not found.")
                }
            }
            Commands::Update { id, amount } => {
                let update_amount = expense.iter_mut().find(|acc| acc.id == id);
                if let Some(n) = update_amount {
                    n.amount = amount;

                    if let Err(e) = save_to_file(expense) {
                        eprint!("There is an error in saving the file: {}", e)
                    } else {
                        println!("Expense updated successfully");
                    }
                } else {
                    println!("ID not found");
                }
            }

            Commands::View { month } => {
                let filter_expenses_to_display = filter(expense, month.as_deref());
                if filter_expenses_to_display.is_empty() {
                    println!("No expenses found");
                } else {
                    println!(
                    "| {:<5} | {:<15} | {:<20} | {:>10} |", 
                    "ID", "Date", "Description", "Amount"
                    );
                    println!("|{:-<7}|{:-<17}|{:-<22}|{:-<12}|", "", "", "", "");

                    for account in filter_expenses_to_display {
                    println!(
                        "| {:<5} | {:<15} | {:<20} | ${:>9.2} |",
                        account.id,
                        account.date.format("%B, %d"),
                        account.description,
                        account.amount
                    )}
                }
            }
            Commands::Summary { month } => {
                let filter_expenses_to_display = filter(expense, month.as_deref());

                if filter_expenses_to_display.is_empty() {
                    println!("No expenses found");
                } else {
                    let total_expense: f64 = filter_expenses_to_display.iter().map(|n| n.amount).sum();
                    println!("Total expenses: {}", total_expense);
                }
                
            }
        }
    }
}

fn main() {
    let args = Cli::parse();
    let mut expense_list: Vec<Expense> = load_expenses().expect("Failed to load the file");
    match args.command {
        Some(cmd) => cmd.execute(&mut expense_list),
        None => println!("Please input a valid argument"),
    }
}

fn load_expenses() -> Result<Vec<Expense>, Box<dyn error::Error>> {
    let file = read_to_string("./expenses.json");
    match file {
        Ok(content) => {
            let expenses = from_str::<Vec<Expense>>(&content)?;
            Ok(expenses)
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Ok(Vec::new()),
            _ => Err(Box::new(e)),
        },
    }
}
fn save_to_file(expense: &[Expense]) -> Result<(), Box<dyn error::Error>> {
    let file = File::create("./expenses.json")?;
    to_writer_pretty(file, &expense)?;
    Ok(())
}

fn filter<'a>(expense: &'a [Expense], month: Option<&str>) -> Vec<&'a Expense> {
    let current_date = Utc::now().year();
    if let Some(text) = month {
        expense
            .iter()
            .filter(|n| {
                n.date.format("%B").to_string().to_lowercase() == text.to_lowercase()
                    && n.date.year() == current_date
            })
            .collect()
    } else {
        expense.iter().collect()
    }
}
