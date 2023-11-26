# Todo-Actix Project

## Overview
This project, developed on a Linux environment, is built using Rust and the Actix web framework. It focuses on implementing a basic Todo application.

## System Requirements
- **Operating System**: Linux
- **Rust Version**: `rustc 1.76.0-nightly (f5dc2653f 2023-11-25)`
- **Cargo Version**: `cargo 1.76.0-nightly (9b13310ca 2023-11-24)`

## Setting Up the Environment
In case you encounter environment-related issues in Rust, set up your environment with:
```bash
source "$HOME/.cargo/env"
```

## Creating the Project
1. Initialize the project with Cargo:
   ```bash
   cargo init --bin todo-actix
   ```
2. Change directory to your project:
   ```bash
   cd todo-actix
   ```
3. Start coding your application in the provided files.

## Running the Application
Execute these steps to run your application:
1. Start the application using Cargo:
   ```bash
   cargo run
   ```
2. Check if the application is running correctly:
   ```bash
   curl http://localhost:8080/
   ```

## Additional Information
- Enhance the Todo application with more features.
- Consider integrating a database for data persistence.
- Implement authentication and authorization for user security.

---