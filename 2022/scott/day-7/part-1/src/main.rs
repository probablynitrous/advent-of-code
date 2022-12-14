use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Folder {
    // The total size of the directory
    pub size: i64,
}

// We could model this more accurately depending on the command itself,
// but I don't think there's any point for a simple AoC
#[derive(Debug)]
struct Instruction<'args> {
    pub command: Command,
    pub args: Vec<&'args str>,
}

#[derive(Debug, PartialEq)]
enum Command {
    ChangeDirectory,
    List,
}

impl Command {
    pub fn from_str(input: &str) -> Command {
        match input {
            "cd" => Command::ChangeDirectory,
            "ls" => Command::List,
            _ => todo!(),
        }
    }
}

fn get_file_as_string(file_name: &str) -> String {
    return fs::read_to_string(file_name).expect("Failed to read input");
}

fn add_file_size(size: &i64, pwd: &Vec<String>, directories: &mut HashMap<String, Folder>) {
    let mut working_directory = pwd.clone();
    while working_directory.len() > 0 {
        let dir_name = working_directory.join("");
        let dir_size = directories.get(&dir_name).unwrap().size + size;
        directories.insert(dir_name.to_string(), Folder { size: dir_size });
        working_directory.pop();
    }
}

fn main() {
    let file = get_file_as_string("input.txt");

    // Keep track of the current working directory
    let mut pwd: Vec<String> = Vec::new();
    let mut directories: HashMap<String, Folder> = HashMap::new();

    file.split("\n")
        .filter(|line| line != &"")
        .for_each(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();

            // If we're parsing a command
            if parts[0].eq("$") {
                let instruction = Instruction {
                    command: Command::from_str(parts[1]),
                    args: parts[2..].to_vec(),
                };

                if instruction.command == Command::List {
                    return;
                }

                if instruction.command == Command::ChangeDirectory {
                    // Move up a directory (assumption is the input won't try to go
                    // out of the root directory).
                    if instruction.args[0].eq("..") {
                        pwd.pop();
                        return;
                    }

                    let mut dir_name = instruction.args[0].to_owned();
                    dir_name.push_str("/");
                    pwd.push(dir_name);

                    let dir = pwd.join("");
                    directories.insert(dir.clone(), Folder { size: 0 });
                }

                return;
            }

            // If it's not a command, then it's either a file or folder
            if parts[0] == "dir" {
                // We don't care about directories until we move into them
                return;
            }

            // Add the file size to the directory total (all the way up the chain)
            add_file_size(&parts[0].parse::<i64>().unwrap(), &pwd, &mut directories);
        });

    println!(
        "sum: {:?}",
        directories
            .values()
            .filter(|dir| dir.size < 100000)
            .map(|dir| dir.size)
            .sum::<i64>()
    );
}
