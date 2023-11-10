## SQLite Lab

### Lab:

# Rust CLI Binary with SQLite

## Overview

Welcome to my Rust CLI project with SQLite integration. This project showcases a command-line interface written in Rust, providing seamless interaction with an SQLite database. The application supports basic CRUD operations and is optimized for performance. Below, you'll find everything you need to know to understand and run this project.

## Table of Contents

1. [Project Structure](#project-structure)
2. [Features](#features)
3. [GitHub Copilot Integration](#github-copilot-integration)
4. [GitHub Actions Workflow](#github-actions-workflow)
5. [Binary Executable](#binary-executable)
6. [Demo Video](#demo-video)


## Project Structure
The project is organized as follows:

- **`cli_tool/`**: Contains the Cargo.toml file used to build, compile and run the rust tool.
- **`cli_tool/src/`**: Contains the Rust source code and the database we create
- **`.github/workflows/`**: Includes the GitHub Actions workflow for testing, building, and linting this project to ensure everything works

## Features

##### Replace `<name>` with the name of the person and `<new-age>` with the actual name and age you want to put. 
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


## Usage of Github Co-pilot

During the course of this project, GitHub Copilot played a pivotal role in easing my transition into Rust development, especially considering my relative unfamiliarity with Rust syntax. As a newcomer to the language, Copilot served as an invaluable assistant, offering real-time suggestions and generating code snippets that not only aligned with the logic I intended but also adhered to Rust's unique syntax and best practices. This proactive support significantly accelerated my learning curve, enabling me to write clear and concise Rust code with increased confidence. Copilot's ability to comprehend context and provide context-aware suggestions significantly streamlined the coding process, making it an indispensable tool for newcomers navigating the intricacies of Rust programming.

## Github Actions workflow

The GitHub Actions workflow implemented in this project streamlined the development and maintenance processes by automating essential tasks. By defining a series of workflows in the .github/workflows/ directory, the project benefits from automated testing, building, and linting of the Rust code with each push or pull request. This ensures that the codebase remains consistent and error-free, promoting a robust and reliable project structure. The workflow is meticulously crafted to run tests, build the Rust binary, and apply linting checks, guaranteeing that the codebase adheres to established coding standards. Additionally, the inclusion of GitHub Actions as part of the development pipeline enhances collaboration among contributors by providing a consistent and efficient environment for code validation, ultimately contributing to a more seamless and collaborative development experience.

Screenshots:
<img width="674" alt="Screenshot 2023-11-09 at 8 09 13 PM" src="https://github.com/farazjawedd/rust-sql-lite-cli/assets/101464414/2c4eda87-42e7-4268-bb18-334277d54e64">
<img width="524" alt="Screenshot 2023-11-09 at 8 09 30 PM" src="https://github.com/farazjawedd/rust-sql-lite-cli/assets/101464414/b1ba482e-a125-4c3d-a8d8-950ecec87ad6">

## Binary Executable
The binary executable in this Rust CLI project represents the compiled, machine-readable form of the Rust source code. Once the Rust code is compiled, it is transformed into a binary executable file that can be run on a compatible system without requiring the presence of the source code or Rust compiler. This executable encapsulates the functionality of the CLI application, allowing users to interact with the program through the command line. The optimization of the binary ensures efficient performance, and in the context of this project, the GitHub Actions workflow includes a process that generates an optimized Rust binary as a downloadable artifact. This optimized binary reflects the culmination of the development efforts, delivering a standalone and efficient application that users can easily run to perform CRUD operations on the associated SQLite database.

```
cargo build --release
```

I have added an executable file along with the database in the executable folder. In order to run the CLI tool without cargo (and directly from the binary), you can execute your commands like this:

```
./cli_tool read
```
or any other command you like.

You can also download the executable by going to the Github actions and scrolling all the way down:

<img width="668" alt="Screenshot 2023-11-09 at 8 12 13 PM" src="https://github.com/farazjawedd/rust-sql-lite-cli/assets/101464414/e5ddf33d-964a-425f-b6be-6a462a5ee71b">


## Demo Video






