use std::io;
use std::io::Write;

struct TodoItem {
    id:u32,
    title: String,
    completed: bool
}

impl TodoItem {
    fn new_item(id: u32, title:&str) -> Self {
        Self {
            id,
            title: title.to_string(),
            completed:false
        }
    }

    fn mark_done(&mut self) {
        self.completed = true
    }

    fn display(&self) {
        let status = if self.completed { "[x]" } else { "[ ]" };
        println!("{} {} - {}", self.id, status, self.title);
    }
}

fn main () {

    let mut todos: Vec<TodoItem> =Vec::new();
    let mut counter_id = 1;

    loop {
        println!("To Do App!");
        println!("1. Add todo");
        println!("2. List todos");
        println!("3. Mark as done");
        println!("4. Exit");
        print!("Enter choice: ");

        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();	

        match choice {
            "1" => {
                print!("Enter todo title: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim();
                let todo = TodoItem::new_item(counter_id, title);
                todos.push(todo);
                println!("Todo added!");
                counter_id += 1;
            }

            "2" => {
                println!("\n Your Todos:");
                for todo in &todos {
                    todo.display();
                }
            }

            "3" => {
                print!("Enter todo ID to mark as done: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();
                if let Ok(id) = id_str.trim().parse::<u32>() {
                    if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                        todo.mark_done();
                        println!("Marked as done!");
                    } else {
                        println!("Todo not found!");
                    }
                } else {
                    println!("Invalid ID input!");
                }
            }

            "4" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid choice. Try again.");
            }
        }
    }
}