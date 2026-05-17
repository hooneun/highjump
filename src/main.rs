use clap::Parser;
use dialoguer::FuzzySelect;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about = "Directory bookmarking and navigation CLI")]
struct Cli {
    #[arg(short, long)]
    save: bool,
    #[arg(short, long)]
    auto_remove: bool,
    #[arg(short, long)]
    list: bool,
    #[arg(short, long)]
    remove: bool,
}

fn get_data_path() -> PathBuf {
    dirs::home_dir()
        .expect("Failed to get home directory")
        .join(".highjump_paths.json")
}

fn load_paths() -> Vec<String> {
    let path = get_data_path();
    if path.exists() {
        let data = std::fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_paths(paths: Vec<String>) {
    let path = get_data_path();
    let data = serde_json::to_string_pretty(&paths).unwrap_or_default();
    std::fs::write(&path, data).expect("Failed to write to file");
}

fn main() {
    let cli = Cli::parse();
    let mut paths = load_paths();

    if cli.save {
        save_current_path(&mut paths);
    } else if cli.auto_remove {
        auto_remove_missing_paths(&mut paths);
    } else if cli.list {
        list_paths(&paths);
    } else if cli.remove {
        remove_path_interactively(&mut paths);
    } else {
        jump_to_path_interactively(&paths);
    }
}

fn save_current_path(paths: &mut Vec<String>) {
    let current_dir = env::current_dir()
        .expect("Failed to get current directory")
        .to_string_lossy()
        .to_string();

    if !paths.contains(&current_dir) {
        paths.push(current_dir.clone());
        save_paths(paths.clone());
        eprintln!("Saved: {current_dir}");
    } else {
        eprintln!("Already saved: {current_dir}");
    }
}

fn auto_remove_missing_paths(paths: &mut Vec<String>) {
    let items = paths.iter().filter(|path| {
        let exists = Path::new(path).exists();

        if !exists {
            println!("Removing non-existent path: {path}");
        }

        exists
    });

    save_paths(items.cloned().collect());
}

fn list_paths(paths: &Vec<String>) {
    if paths.is_empty() {
        eprintln!("No saved paths found.");
        return;
    }

    for (index, path) in paths.iter().enumerate() {
        println!("[{}] {path}", index + 1);
    }
}

fn build_display_items(paths: &[String]) -> Vec<String> {
    let mut display_items: Vec<String> = paths
        .iter()
        .enumerate()
        .map(|(index, path)| format!("{}: {}", index + 1, path))
        .collect();

    display_items.push("0: Exit".to_string());
    display_items
}

fn select_path_index(paths: &[String], prompt: &str) -> Option<usize> {
    let display_items = build_display_items(paths);

    let selection = FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt(prompt)
        .default(0)
        .items(&display_items)
        .interact_opt()
        .unwrap();

    match selection {
        Some(index) if index < paths.len() => Some(index),
        Some(_) => {
            eprintln!("Exiting.");
            None
        }
        None => None,
    }
}

fn remove_path_interactively(paths: &mut Vec<String>) {
    if paths.is_empty() {
        eprintln!("No saved paths found.");
        return;
    }

    if let Some(index) = select_path_index(paths, "Select a path to remove:") {
        println!("Removing path: {}", index + 1);
        paths.remove(index);
        save_paths(paths.clone());
        eprintln!("Path removed successfully.");
    }
}

fn jump_to_path_interactively(paths: &[String]) {
    if paths.is_empty() {
        eprintln!("No saved paths found. Add a path first using the 'hj --save' command.");
        return;
    }

    if let Some(index) = select_path_index(paths, "Select a path to jump to:") {
        print!("{}", paths[index]);
    }
}
