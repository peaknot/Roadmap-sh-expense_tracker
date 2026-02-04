https://roadmap.sh/projects/expense-tracker

# Expense Tracker CLI

A lightweight command-line interface (CLI) tool for managing personal finances. It supports adding, updating, deleting, and viewing expenses with filtering capabilities.

## Features

* **CRUD Operations:** Create, Read, Update, and Delete expenses.
* **Filtering:** View expenses by specific month (automatically filters for the current year).
* **Summary:** Calculate total expenses for all time or a specific month.
* **Case Insensitive:** Handles month names regardless of capitalization (e.g., "January", "january").

## Prerequisites

* **Rust & Cargo:** Ensure you have Rust installed.
    * Install via: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Installation

1.  Clone the repository.
2.  Build the project:
    
        cargo build --release
    

## Usage

### 1. Add an Expense
Adds a new expense with a description and amount. The ID and Date are auto-generated.

    cargo run -- add --description "Groceries" --amount 50.0

## 2. View an Expense
View All: Displays a table of all expenses sorted by ID.

    cargo run -- view

  - ### Filter by Month.

        cargo run -- view --month January

## 3. Summary

    cargo run -- summary

  - ### Filter by Month.

        cargo run -- summary --month January

## 4 Update an Expense
Modifies the amount of an existing expense using its ID.

    cargo run -- update --id 1 --amount 75.0

## 5. Delete and Expense
Permanently removes an expense by its ID.

    cargo run -- delete --id 1
