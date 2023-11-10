## SQLite Lab

### Lab:

# Rust CLI Binary with SQLite

## Overview

Welcome to my Rust CLI project with SQLite integration. This project showcases a command-line interface written in Rust, providing seamless interaction with an SQLite database. The application supports basic CRUD operations and is optimized for performance. Below, you'll find everything you need to know to understand and run this project.

## Table of Contents

1. [Project Structure](#project-structure)
2. [Features](#features)
3. [Usage](#usage)
4. [GitHub Copilot Integration](#github-copilot-integration)
5. [GitHub Actions Workflow](#github-actions-workflow)
6. [Demo Video](#demo-video)
7. [Contributing](#contributing)
8. [License](#license)


## Project Structure
The project is organized as follows:

- **`cli_tool/`**: Contains the Cargo.toml file used to build, compile and run the rust tool.
- **`cli_tool/src/`**: Contains the Rust source code and the database we create
- **`.github/workflows/`**: Includes the GitHub Actions workflow for testing, building, and linting this project to ensure everything works

## Features

### To insert a person:
``````
   cargo run insert <name> <age>
``````
<img width="843" alt="Screenshot 2023-11-09 at 7 57 43 PM" src="https://github.com/farazjawedd/rust-sql-lite-cli/assets/101464414/ca476569-a6c6-4176-b4e8-0f6066f517dc">

### To read all persons:
``````
   cargo run read
``````
<img width="698" alt="Screenshot 2023-11-09 at 7 57 36 PM" src="https://github.com/farazjawedd/rust-sql-lite-cli/assets/101464414/25a59901-cc73-4ce3-a4f2-1225b148f059">

### To update a person's age:
``````
   cargo run update <name> <new-age>
``````
<img width="794" alt="Screenshot 2023-11-09 at 8 05 00 PM" src="https://github.com/farazjawedd/rust-sql-lite-cli/assets/101464414/2ada1b9f-a9b3-43cc-8777-a0d11a095150">

### To delete a person's entry:
``````
   cargo run delete <name>
``````
<img width="792" alt="Screenshot 2023-11-09 at 8 05 35 PM" src="https://github.com/farazjawedd/rust-sql-lite-cli/assets/101464414/97300b55-2532-43b9-8e2e-a6686e4575ab">
We deleted the entry we added above!

##### Replace `<name>` with the name of the person and `<new-age>` with the actual name and age you want to put. 




