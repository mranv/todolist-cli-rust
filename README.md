# todolist-cli-rust
A simple command line todo app in rust!

Here is a README description for the Rust command line todo list code:

# Rust Todo CLI
A simple command line program to manage your tasks and todo lists. 

## Introduction
This is a command line todo list manager written in Rust. It allows you to add tasks, mark them as completed, and print the list. The purpose is to provide a basic todo list that runs in a terminal for productivity.

The project demonstrates building small command line tools and text interfaces in Rust. It uses Rust structs to store data, enums for managing logic flow, and implements input/output via the standard library.

## Usage
These are the supported commands:

- `add` - Add a new todo item 
- `complete` - Mark an item as completed by specifying its index
- `print` - Print all the todos
- `quit` - Exit the program

An example session may look like:

```
> add 
Enter task: Submit project report
> add
Enter task: Call mom  
> print
1. Submit project report
2. Call mom
> complete 
Enter index to complete: 2
> print
1. Submit project report
> quit
```

The code handles input validation and gracefully handles incorrect commands or index values.

## Implementation
It uses a `Todo` struct to store a vector of string values for tasks. Methods are added on this struct to `add()`, `complete()` and `print()` todos.

The main `loop` implements a basic read-evaluate-print style REPL interface that reads a command, calls appropriate struct methods, and outputs relevant messages.

Error handling is done via Rust `Result` and `unwrap()` for simplicity but this can be improved.

Overall, it provides a basic but useful command line todo list manager to showcase Rust CLI development.

## Future Work
Possible improvements:

- Persist todos to file for durability
- Support editing existing items
- Improve input validation  
- Color coded output
- Progress tracking features
- Subcommands

This covers the high level working of this Rust todo list CLI project and how it can be used. Feel free to enhance it further!
