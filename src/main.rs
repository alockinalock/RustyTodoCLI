use colored::*;
use serde::{Deserialize, Serialize};
use std::fs::{read_link, File};
use std::io;
use std::io::{Read, Result, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Tasks {
    name: String,
    desc: String,
}

fn homescreen(new: bool) -> bool {
    if new == true {
        println!("Welcome to RustyTodoCLI! A terminal based productivity app which helps you track tasks... in other words, this is a todo planner.");
        println!("--This message will disappear after you leave this page.--\n");
    }

    println!(">ADD\nTo add a new task to the list.\n");
    println!(">REMOVE\nTo remove a task from the list.\n");
    println!(">RENAME\nTo rename a task in the list.\n");
    println!(">VIEW\nTo view the list of tasks.\n");
    println!(">SAVE\nTo save your list.\n");
    // This is weird way of telling the user that this command does not save.
    println!(">QUIT\nTo quit the application. WARNING: This does not automatically save any changes made.\n");

    return false;
}

fn rusty_add(tasks: &mut Vec<Tasks>) {
    let mut name = String::new();
    let mut description = String::new();

    println!("ENTER NAME\n");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");
    if name.trim().to_lowercase() == "cancel" {
        println!("{}[2J", 27 as char);
        println!("Process terminated. Press ENTER to continue.\n");
        return;
    }
    println!("\n\n\nENTER DESCRIPTION\n");
    io::stdin()
        .read_line(&mut description)
        .expect("Failed to read description");
    if description.trim().to_lowercase() == "cancel" {
        println!("{}[2J", 27 as char);
        println!("Process terminated. Press ENTER to continue.\n");
        return;
    }

    let new_task = Tasks {
        name: name.trim().to_string(),
        desc: description.trim().to_string(),
    };

    tasks.push(new_task);

    println!("{}[2J", 27 as char);
    println!(
        "New task \"{}\" has been added to the list. Press ENTER to continue.\n",
        name.trim().to_lowercase()
    );
}

fn rusty_remove(tasks: &mut Vec<Tasks>) {
    if tasks.len() == 0 {
        println!("{}[2J", 27 as char);
        println!("List contains no tasks; nothing can be removed. Press ENTER to continue.\n");
        return;
    }

    println!("Select a task to remove from the list:\n-----------------------------------------------------");
    rusty_view_name_only(tasks);
    println!();
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read chosen task for removal");

    loop {
        // io::stdin().read_line(&mut user_input);
        if user_input.trim().to_lowercase() == "def" {
            println!("{}[2J", 27 as char);
            println!("Select a task to remove from the list:\n-----------------------------------------------------");
            rusty_view(tasks);
            println!();
            user_input.clear();
            io::stdin()
                .read_line(&mut user_input)
                .expect("Failed to read chosen task for removal");
        } else if user_input.trim().to_lowercase() == "cancel" {
            println!("{}[2J", 27 as char);
            println!("Process terminated. Press ENTER to continue.\n");
            return;
        } else {
            break;
        }
    }

    let mut removal_status = false;

    tasks.retain(|t| {
        if t.name.to_lowercase() == user_input.trim().to_lowercase() {
            removal_status = true;
            false
        } else {
            true
        }
    });

    if removal_status {
        println!("{}[2J", 27 as char);
        println!(
            "Removed \"{}\". You now have {} tasks left. Press ENTER to continue.\n",
            user_input.trim().to_lowercase(),
            tasks.len()
        );
    } else {
        println!("{}[2J", 27 as char);
        // FIXME: this error message is vague.
        println!(
            "Failed to remove \"{}\".\n",
            user_input.trim().to_lowercase()
        );
    }
}

fn rusty_rename(tasks: &mut Vec<Tasks>) {
    if tasks.len() == 0 {
        println!("{}[2J", 27 as char);
        println!("List contains no tasks; nothing can be renamed. Press ENTER to continue.\n");
        return;
    }

    println!("Select a task to rename from the list\n-----------------------------------------------------");
    rusty_view_name_only(tasks);
    println!();
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input);

    loop {
        if user_input.trim().to_lowercase() == "def" {
            println!("{}[2J", 27 as char);
            println!("Select a task to rename from the list\n-----------------------------------------------------");
            rusty_view(tasks);
            println!();
            user_input.clear();
            io::stdin().read_line(&mut user_input);
        } else if user_input.trim().to_lowercase() == "cancel" {
            println!("{}[2J", 27 as char);
            println!("Process terminated. Press ENTER to continue.\n");
            return;
        } else {
            break;
        }
    }

    let mut renamed_status = false;
    let mut renamed_task = Tasks {
        name: "".to_string(),
        desc: "".to_string(),
    };
    let mut new_name = String::new();

    tasks.retain(|t| {
        if t.name == user_input.trim() {
            println!("{}[2J", 27 as char);
            println!(
                "Enter a new name for \"{}\".\n",
                user_input.trim().to_lowercase()
            );
            io::stdin().read_line(&mut new_name);
            renamed_task = Tasks {
                name: new_name.trim().to_string(),
                desc: t.desc.to_string(),
            };
            renamed_status = true;
            false
        } else {
            true
        }
    });
    tasks.push(renamed_task);

    if renamed_status {
        println!("{}[2J", 27 as char);
        println!(
            "Renamed task \"{}\" to \"{}\". Press ENTER to continue.\n",
            user_input.trim().to_lowercase(),
            new_name.trim().to_lowercase()
        );
    } else {
        println!("{}[2J", 27 as char);
        // TODO: write a good error msg here
        println!("the shit failed. woops.\n");
    }
}

// TODO: Format the view command. Tasks with long description make this command look awful.
// This is mainly a problem for smaller windows. When a terminal window is maximized, it looks ok.
fn rusty_view(tasks: &Vec<Tasks>) {
    for t in tasks {
        println!("{} - {}", t.name, t.desc);
    }
}

fn rusty_view_name_only(tasks: &Vec<Tasks>) {
    for t in tasks {
        println!("{}", t.name);
    }
}

fn define_task(tasks: &mut Vec<Tasks>, command: String) {
    let task_name = &command[4..];
    let mut obtained_desc = String::new();
    let mut status = false;

    tasks.retain(|t| {
        if task_name == t.name {
            // This is a dumb way of moving out t.desc, oh well.
            let obtained = &t.desc;
            obtained_desc = obtained.to_string();
            status = true;
            false
        } else {
            true
        }
    });
}

fn rusty_save(tasks: &[Tasks], filename: &str) -> Result<()> {
    let json_data = serde_json::to_string_pretty(tasks)?;

    println!("JSON data to write: {}", json_data);

    let mut file = File::create(filename)?;
    file.write_all(json_data.as_bytes())?;

    Ok(())
}

fn rusty_initiate_cache(filename: &str) -> Vec<Tasks> {
    let mut file = File::open(filename).expect("a");
    let mut json_data = String::new();

    file.read_to_string(&mut json_data).expect("b");

    let tasks: Vec<Tasks> = serde_json::from_str(&json_data).expect("c");
    return tasks;
}

// This function is unused. It could possibly be used to detect unsaved changes later.
fn rusty_quit() {}

// TODO: press ENTER to continue is always mentioned, maybe actually look for it?
// TODO: help option which lists all possible commands
// TODO: write tests.
fn main() {
    // Declare global variables
    // ====================================

    let file = "../../storage.json";
    let mut new_session = true;
    let mut cache: Vec<Tasks> = rusty_initiate_cache(file);
    let mut void = String::new();

    // ====================================

    loop {
        println!("{}[2J", 27 as char);
        let mut option = String::new();

        new_session = homescreen(new_session);
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read chosen option from user.");
        let option = option.trim().to_lowercase();

        match option.as_str() {
            "add" => {
                println!("{}[2J", 27 as char);
                rusty_add(&mut cache);
                io::stdin().read_line(&mut void);
            }
            "remove" => {
                println!("{}[2J", 27 as char);
                rusty_remove(&mut cache);
                io::stdin().read_line(&mut void);
            }
            "rename" => {
                println!("{}[2J", 27 as char);
                rusty_rename(&mut cache);
                io::stdin().read_line(&mut void);
            }
            "view" => {
                println!("{}[2J", 27 as char);
                if cache.len() == 0 {
                    println!(
                        "There are no active tasks that can be listed. Press ENTER to continue."
                    );
                } else {
                    println!("All active tasks are listed below.\n-----------------------------------------------------");
                    rusty_view(&mut cache);
                }
                println!("\n\n\nPress ENTER when you are finished.\n");
                io::stdin().read_line(&mut void);
            }
            "save" => {
                rusty_save(&cache, file);
                println!("{}[2J", 27 as char);
                println!("Data has been saved. Press ENTER to continue.\n");
                io::stdin().read_line(&mut void);
            }
            "quit" => {
                println!("{}[2J", 27 as char);
                return;
            }
            "" => {
                println!("{}[2J", 27 as char);
                println!("No command entered. Please enter a command. Press ENTER to return back to the list of commands.\n");
                io::stdin().read_line(&mut void);
            }
            _ => {
                println!("{}[2J", 27 as char);
                println!(
                    "Command is invalid. Press ENTER to return back to the list of commands.\n"
                );
                io::stdin().read_line(&mut void);
            }
        }
    }
}
