use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn organize_files(directory: &str) {
    let path = Path::new(directory);
    if !path.is_dir() {
        eprintln!("{} is not a directory", directory);
        return;
    }

    let categories = get_categories();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();
        if file_path.is_file() {
            let file_name = file_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_lowercase();
            for (category, patterns) in &categories {
                if patterns.iter().any(|pattern| {
                    let re = Regex::new(pattern).unwrap();
                    re.is_match(&file_name)
                }) {
                    let category_path = path.join(category);
                    if let Err(e) = fs::create_dir_all(&category_path) {
                        eprintln!("Failed to create directory {:?}: {}", category_path, e);
                        continue;
                    }
                    let new_path = category_path.join(file_path.file_name().unwrap());
                    if let Err(e) = fs::rename(&file_path, &new_path) {
                        eprintln!("Failed to move {:?} to {:?}: {}", file_path, new_path, e);
                    } else {
                        println!("Moved {:?} to {:?}", file_path, new_path);
                    }
                    break;
                }
            }
        }
    }
}

fn get_categories() -> HashMap<&'static str, Vec<&'static str>> {
    let mut categories = HashMap::new();
    categories.insert(
        "Foundation of Science",
        vec![
            r"science",
            r"python",
            r"physics",
            r"chemistry",
            r"biology",
            r"pytorch",
        ],
    );
    categories.insert("Mathematics and Statistics", vec![r"math", r"statistics"]);
    categories.insert(
        "Foundation of Informatics",
        vec![r"informatics", r"computerscience", r"computer"],
    );
    categories.insert(
        "Business Administration and Accounting",
        vec![
            r"business",
            r"accounting",
            r"rust",
            r"finance",
            r"economics",
            r"cli",
            r"commands",
            r"linux",
        ],
    );
    categories.insert("German Language", vec![r"german", r"deutsch"]);
    categories.insert(
        "Foundation of Medicine",
        vec![
            r"medicine",
            r"health",
            r"doctor",
            r"hospital",
            r"medical",
            r"terminology",
            r"anatomy",
        ],
    );
    categories
}
