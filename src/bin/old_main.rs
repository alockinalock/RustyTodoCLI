use colored::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::{Read, Result, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    desc: String,
}

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
    println!("---------------------------------------");
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

fn rename_task(task: &mut Vec<Task>) {
    println!("Which task would you like to rename?");
    println!("---------------------------------------");
    list_names(task);
    println!("---------------------------------------");
    print!("Rename: ");
    io::stdout().flush();
    let mut changed = String::new();
    io::stdin()
        .read_line(&mut changed)
        .expect("Failed to read from the CLI");
    let changed = changed.trim();

    let mut is_changed = false;
    let mut renamed_task = Task {
        name: "".to_string(),
        desc: "".to_string(),
    };

    task.retain(|t| {
        if t.name == changed {
            println!("What would you like to rename the task to?");
            let mut new_name = String::new();
            io::stdin()
                .read_line(&mut new_name)
                .expect("Failed to read from CLI");
            renamed_task = Task {
                name: new_name.trim().to_string(),
                desc: t.desc.to_string(),
            };
            is_changed = true;
            false
        } else {
            true
        }
    });
    task.push(renamed_task);
    if is_changed {
        println!("\nTask has been renamed and readded to list.");
    } else {
        println!(
            "\nTask has failed to be renamed. Please check if it exists or if you have mistyped it."
        );
    }
}

fn list_names(task: &Vec<Task>) {
    if task.len() == 0 {
        println!();
    } else {
        for t in task {
            println!("{}", t.name);
        }
    }
}

fn view_tasks(task: &Vec<Task>) {
    if task.len() == 0 {
        println!("All tasks are completed, good job!");
    } else {
        for t in task {
            println!("{} - {}", t.name, t.desc);
        }
    }
}

fn write_to_json(task: &[Task], filename: &str) -> Result<() /*Box<dyn std::error::Error>*/> {
    let json_data = serde_json::to_string_pretty(task)?;

    println!("JSON data to write: {}", json_data);

    let mut file = File::create(filename)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

fn read_from_json(filename: &str) -> Vec<Task> {
    let mut file = File::open(filename).expect("a");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("c");

    let task: Vec<Task> = serde_json::from_str(&json_data).expect("d");
    return task;
}

// TODO: Colored version of the app
pub fn colored_vers(cache: &str, current_tasks: Vec<Task>) {
    let mut terminate = false;
    let mut choice = String::new();

    println!(
        "{}",
        "What are you trying to do today?".bold().underline().blue()
    );
    println!("{} Add Task", "[mk]".bold().red());
    println!("{} Remove Task", "[rm]".bold().red());
    println!("{} Rename Task", "[mv]".bold().red());
    println!("{} View All Task", "[vw]".bold().red());
    println!("{} Save Quit", "[wq]".bold().red());
    println!("{} Force Quit", "[qq]".bold().red());

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read from the CLI");
    let choice = choice.trim();
}

// TODO: empty strings need to be able to be seen
// TODO: add another function: update status. doesnt delete the task, just updates the status
// TODO: Currently stores information in the same directory as the one it is run in.
// TODO: Autocomplete for quality of life
fn main() {
    let cache = "../../storage.json";
    let mut current_tasks: Vec<Task> = Vec::new();
    println!("{}[2J", 27 as char);
    current_tasks = read_from_json(cache);

    let dev = false;
    if dev == true {
        colored_vers(cache, current_tasks);
        return;
    }

    loop {
        let mut choice = String::new();
        let mut terminate = false;

        println!("What are you trying to do today?");
        println!("---------------------------------------");

        println!("[mk] Add Task \n[rm] Remove Task \n[mv] Rename Task \n[vw] View All Tasks \n[wq] Save Quit \n[qq] Force Quit");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read from the CLI");
        let choice = choice.trim();

        match choice {
            "mk" => {
                println!("{}[2J", 27 as char);
                add_task(&mut current_tasks);
                println!();
            }
            "rm" => {
                println!("{}[2J", 27 as char);
                remove_task(&mut current_tasks);
                println!();
            }
            "mv" => {
                println!("{}[2J", 27 as char);
                rename_task(&mut current_tasks);
                println!();
            }
            "vw" => {
                println!("{}[2J", 27 as char);
                println!("Here are all your tasks:");
                println!("---------------------------");
                view_tasks(&mut current_tasks);
                println!("---------------------------\n");
            }
            "wq" => {
                // We don't handle the error.
                write_to_json(&current_tasks, cache);
                terminate = true;
            }
            "qq" => terminate = true,
            _ => println!("Not a valid command.\n\n\n"),
        }
        if terminate {
            println!("{}[2J", 27 as char);
            break;
        }
    }
}
