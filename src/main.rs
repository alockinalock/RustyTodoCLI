use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::io::Write;

struct Task {
    name: String,
    desc: String,
}

// TODO: Add another field: status
fn add_task(task: &mut Vec<Task>) {
    let mut name = String::new();
    let mut description = String::new();
    println!("Enter the name of the new task:");
    io::stdin().read_line(&mut name);
    println!("Enter the description for this new task:");
    io::stdin().read_line(&mut description);

    let new_task = Task {
        name: name.trim().to_string(),
        desc: description.trim().to_string(),
    };

    task.push(new_task);
}

fn remove_task(task: &mut Vec<Task>) {
    println!("Which task would you like to remove?");
    view_tasks(task);
    println!("---------------------------------------");
    print!("Remove: ");
    io::stdout().flush();
    let mut destroy = String::new();
    io::stdin()
        .read_line(&mut destroy)
        .expect("Failed to read input from the CLI");
    let destroy = destroy.trim();

    let mut is_removed = false;

    task.retain(|t| {
        if t.name == destroy {
            is_removed = true;
            false
        } else {
            true
        }
    });

    if is_removed {
        println!("\n{} has been removed.", destroy);
    } else {
        println!(
            "\nAn error has been encountered. \"{}\" has not been removed from the list.",
            destroy
        );
    }
}

fn rename_task() {}

// TODO: add color to output.
fn view_tasks(task: &Vec<Task>) {
    if task.len() == 0 {
        println!("All tasks are completed, good job!");
    } else {
        for t in task {
            println!("{} - {}", t.name, t.desc);
        }
    }
}

fn write_to_json() {}

// TODO: add another function: update status. doesnt delete the task, just updates the status
// FIXME: add an expect method everytime input is taken from the console
fn main() {
    let mut current_tasks: Vec<Task> = Vec::new();
    loop {
        let mut choice = String::new();
        let mut running_status = String::new();

        println!("What are you trying to do today?");
        println!("[mk] Add Task | [rm] Remove Task | [mv] Rename Task | [vw] View All Tasks");
        io::stdin().read_line(&mut choice);
        let choice = choice.trim();

        match choice {
            "mk" => add_task(&mut current_tasks),
            "rm" => remove_task(&mut current_tasks),
            "mv" => rename_task(),
            "vw" => {
                println!("Here are all your tasks:");
                println!("---------------------------");
                view_tasks(&mut current_tasks)
            }
            _ => println!("Not a valid command."),
        }

        println!("\nAre you finished?");
        println!("[N/n] No | [Y/y] Yes");
        io::stdin().read_line(&mut running_status);
        let running_status = running_status.trim();

        match running_status {
            "Y" | "y" => {
                println!("\nTerminating process.");
                write_to_json();
                break;
            }
            _ => (),
        }
    }
}
