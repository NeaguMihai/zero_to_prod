use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    sync::mpsc::{self},
    thread,
};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CONTROLLER_REGEX: Regex = Regex::new(r"struct\s+([A-Za-z_]\w*)").unwrap();
}

//Generate a constant vector with all http verbs as #[http_verb
struct ControllerStructure {
    name: String,
    path: String,
}

fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").expect("Failed to get current dir");
    let target_dir = PathBuf::from(crate_dir);
    let target_dir = target_dir.parent().unwrap();
    let routes_file_path = target_dir.join("target/tmp/routes.txt");
    println!("Routes file path: {:?}", routes_file_path);
    create_routes_file_if_not_exists(routes_file_path.clone());
    let last_write = get_last_write(routes_file_path.clone());
    std::fs::write(
        routes_file_path.clone(),
        format!("last_write:{}\n", last_write),
    )
    .expect("Failed to clean file");
    println!("Last write: {}", last_write);
    let base_path = std::env::current_dir().expect("Failed to get current dir");
    let (tx, rx) = mpsc::channel::<ControllerStructure>();
    let moved_routes_file_path = routes_file_path.clone();

    thread::spawn(move || {
        walk_path(
            base_path.join("src"),
            moved_routes_file_path,
            last_write,
            tx,
        );
    });

    let mut controllers: HashMap<String, String> = HashMap::new();
    for controller in rx {
        if controllers.contains_key(&controller.name) {
            panic!("Controller name {} already exists", controller.name);
        }
        controllers.insert(controller.name, controller.path);
    }
    let t = controllers
        .iter()
        .map(|c| format!("{}:{}", c.0, c.1))
        .collect::<Vec<String>>();
    OpenOptions::new()
        .append(true)
        .write(true)
        .open(routes_file_path.clone())
        .expect("Failed to write routes to file")
        .write_all(t.join("\n").as_bytes())
        .expect("Failed to write routes to file");

    update_last_write(routes_file_path);
}

fn create_routes_file_if_not_exists(routes_file: PathBuf) {
    if !routes_file.exists() {
        let mut routes = std::fs::File::create(routes_file).expect("Failed to create routes file");

        routes
            .write_all("last_write:0\n".as_bytes())
            .expect("Failed to write initial timestamp to routes file");
    }
}

fn get_last_write(routes_file: PathBuf) -> u64 {
    let routes_file = File::open(routes_file).expect("Failed to open routes file");
    let mut last_write_line = String::new();

    BufReader::new(routes_file)
        .read_line(&mut last_write_line)
        .expect("Failed to read line");

    let last_write = last_write_line
        .split("last_write:")
        .nth(1)
        .expect("Failed to get last write")
        .trim_end();
    println!("Last write: {}", last_write);
    last_write
        .parse()
        .expect("Failed to parse last write timestamp")
}

fn update_last_write(routes_file_path: PathBuf) {
    let previous_content =
        std::fs::read_to_string(routes_file_path.clone()).expect("Failed to read routes file");
    let mut previous_content = previous_content
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    println!("Previous content: {:?}", previous_content);

    previous_content.remove(0);

    let current_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to get duration since epoch")
        .as_secs();

    previous_content.insert(0, format!("last_write:{}", current_timestamp));
    println!("Previous content: {:?}", previous_content.join("\n"));

    std::fs::write(routes_file_path, previous_content.join("\n"))
        .expect("Failed to write to routes file");
}

fn walk_path(
    path: PathBuf,
    routes_path: PathBuf,
    _last_write: u64,
    tx: mpsc::Sender<ControllerStructure>,
) {
    let entries = std::fs::read_dir(path).expect("Failed to read src");
    for entry in entries.into_iter().flatten() {
        let path = entry.path();
        if path.is_dir() {
            let route_path_clone = routes_path.clone();
            let tx_clone = tx.clone();
            thread::spawn(move || {
                walk_path(path, route_path_clone, _last_write, tx_clone);
            });
        } else {
            if !is_controller(path.clone()) {
                continue;
            }

            let f = File::open(path.clone());
            if let Ok(file) = f {
                let reader = BufReader::new(file);
                let lines = reader.lines();
                match find_controller(lines, &path) {
                    Some(controller) => {
                        tx.send(controller).unwrap();
                    }
                    None => {
                        panic!(
                            "Expected to find controller in file: {}",
                            path.file_name().unwrap().to_str().unwrap()
                        );
                    }
                };
            }
        }
    }
}

fn find_controller(
    mut lines: std::io::Lines<BufReader<File>>,
    path: &Path,
) -> Option<ControllerStructure> {
    loop {
        let line = lines.next();
        println!("Line: {:?}", line);
        if line.is_none() {
            break;
        }
        let line = line.unwrap().unwrap_or_default();

        if !line.starts_with("#[controller") {
            continue;
        }
        let mut controller_structure = ControllerStructure {
            name: String::new(),
            path: String::new(),
        };
        loop {
            let next_line = lines.next().unwrap().unwrap_or_default();
            if next_line.starts_with('}') || next_line.is_empty() {
                break;
            }
            let found_controller = CONTROLLER_REGEX.captures(&next_line);
            if found_controller.is_none() {
                continue;
            }

            let found_controller = found_controller.unwrap();
            let controller_name = found_controller.get(1).unwrap().as_str();
            controller_structure.name = controller_name.to_string();
            controller_structure.path = path.to_path_buf().to_str().unwrap().to_string();
            return Some(controller_structure);
        }
    }
    Option::None
}

fn is_controller(path: PathBuf) -> bool {
    path.file_name()
        .unwrap_or_else(|| {
            panic!(
                "Failed to get file name from path: {}",
                path.to_str().unwrap()
            )
        })
        .to_str()
        .unwrap_or_else(|| {
            panic!(
                "Failed to get file name from path: {}",
                path.to_str().unwrap()
            )
        })
        .contains("controller.rs")
}
