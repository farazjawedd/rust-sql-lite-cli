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

### To read all persons:
``````
   cargo run read
``````
### To update a person's age:
``````
   cargo run update <name> <new-age>
``````
### To delete a person's entry:
``````
   cargo run delete <name>
``````
##### Replace `<name>` with the name of the person and `<new-age>` with the actual name and age you want to put. 




