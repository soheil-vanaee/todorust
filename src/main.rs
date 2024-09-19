use std::io::{self, Write}; 
use std::process::exit;   

struct ToDoItem {
    title: String,
    text: String,
    is_done: bool,
    tcount: u32,
}

fn add_todo(todos: &mut Vec<ToDoItem>) {
    let title = input("Enter the title: ");
    let text = input("Enter the description: ");
    let tcount: u32 = input("Enter the ID: ").trim().parse().expect("Please enter a valid number");
    
    let new_todo = ToDoItem {
        title,
        text,
        is_done: false, 
        tcount,
    };
    todos.push(new_todo);
    
    println!("ToDo item added successfully!");
}

fn view_todos(todos: &Vec<ToDoItem>) {
    if todos.is_empty() {
        println!("No ToDo items found!");
    } else {
        for (i, todo) in todos.iter().enumerate() {
            println!("{}. {}: {} (done: {}, count: {})", i + 1, todo.title, todo.text, todo.is_done, todo.tcount);
        }
    }
}

fn remove_done_todos(todos: &mut Vec<ToDoItem>) {
    let initial_len = todos.len();
    todos.retain(|todo| !todo.is_done); 

    let removed_count = initial_len - todos.len();
    if removed_count > 0 {
        println!("{} ToDo item(s) removed.", removed_count);
    } else {
        println!("No completed ToDo items to remove.");
    }
}

fn main() {
    let mut todos: Vec<ToDoItem> = Vec::new();  

    loop {
        println!("Enter your choice:");
        println!("1. View all todos");
        println!("2. Add new todo");
        println!("3. Remove completed todos (is_done = true)");
        println!("4. Exit");
        print!("-> ");
        
        io::stdout().flush().unwrap(); 
        let choice = input("").trim().to_string();

        match choice.as_str() {
            "1" => view_todos(&todos),             
            "2" => add_todo(&mut todos),           
            "3" => remove_done_todos(&mut todos),  
            "4" => {
                println!("Exiting...");
                exit(0); 
            }
            _ => println!("Invalid choice! Please enter a valid option."),
        }

        println!(); 
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt); 
    io::stdout().flush().unwrap(); 
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
