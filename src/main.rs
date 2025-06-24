use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use std::{fs, io, path::Path};

#[derive(Serialize, Deserialize, Debug)]
struct Habit {
    name: String,
    completed_dates: Vec<String>,
}

impl Habit {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            completed_dates: vec![],
        }
    }

    fn mark_done_today(&mut self) {
        let today = Local::now().format("%Y-%m-%d").to_string();
        if !self.completed_dates.contains(&today) {
            self.completed_dates.push(today);
            println!("✅ Marked '{}' as done for today!", self.name);
        } else {
            println!("⚠️ '{}' was already marked done today!", self.name);
        }
    }
}

fn main() {
    let mut habits = load_habits("habits.json");

    loop {
        println!("\n🌟 Habit Tracker 🌟");
        println!("1. Add New Habit");
        println!("2. Mark Habit Done Today");
        println!("3. View Habits & Dates");
        println!("4. Exit and Save");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                println!("Enter habit name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                habits.push(Habit::new(name.trim()));
                println!("🎉 Habit added!");
            },
            "2" => {
                if habits.is_empty() {
                    println!("No habits found. Add one first.");
                    continue;
                }
                for (i, habit) in habits.iter().enumerate() {
                    println!("{}. {}", i + 1, habit.name);
                }
                println!("Pick a habit number:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(index) = input.trim().parse::<usize>() {
                    if let Some(habit) = habits.get_mut(index - 1) {
                        habit.mark_done_today();
                    } else {
                        println!("❌ Invalid index.");
                    }
                }
            },
            "3" => {
                for habit in &habits {
                    println!("\n📌 Habit: {}", habit.name);
                    for date in &habit.completed_dates {
                        println!("   ✅ Done on: {}", date);
                    }
                }
            },
            "4" => {
                save_habits("habits.json", &habits);
                println!("📁 Habits saved. Goodbye!");
                break;
            },
            _ => println!("❌ Invalid option. Try again."),
        }
    }
}

fn save_habits<P: AsRef<Path>>(path: P, habits: &[Habit]) {
    let data = serde_json::to_string_pretty(habits).unwrap();
    fs::write(path, data).expect("Unable to write to file");
}

fn load_habits<P: AsRef<Path>>(path: P) -> Vec<Habit> {
    if path.as_ref().exists() {
        let data = fs::read_to_string(&path).expect("Unable to read file");
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        vec![]
    }
}

