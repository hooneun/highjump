use clap::Parser;
use dialoguer::FuzzySelect;
use std::env;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Directory bookmarking and navigation CLI")]
struct Cli {
    #[arg(short, long)]
    save: bool,
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
        let current_dir = env::current_dir()
            .expect("Failed to get current directory")
            .to_string_lossy()
            .to_string();

        if !paths.contains(&current_dir) {
            paths.push(current_dir.clone());
            save_paths(paths);
            eprintln!("Saved: {current_dir}");
        } else {
            eprintln!("Already saved: {current_dir}");
        }
    } else {
        if paths.is_empty() {
            eprintln!("No saved paths found. Add a path first using the 'hj --save' command.");
            return;
        }

        let mut display_items: Vec<String> = paths
            .iter()
            .enumerate()
            .map(|(index, path)| format!("{}: {}", index + 1, path))
            .collect();

        display_items.push("0: Exit".to_string());

        let selection = FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .with_prompt("Select a path to jump to:")
            .default(0)
            .items(&display_items)
            .interact_opt()
            .unwrap();

        if let Some(index) = selection {
            if index < paths.len() {
                print!("{}", paths[index]);
            }
        }
    }
}