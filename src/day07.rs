use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use super::{AppError, InputReader};
use anyhow::Result;

enum InputLine {
    CdRoot,
    CdUp,
    CdDir(String),
    Ls,
    File(usize, String),
    Directory(String),
}

enum FileDesc {
    File(usize, String),
    Directory(String),
}

enum Command {
    CdRoot,
    CdUp,
    CdDir(String),
    Ls(Vec<FileDesc>),
}

fn parse_input(s: &str) -> Result<InputLine> {
    let mut split_string = s.split_whitespace();
    match split_string.next() {
        Some("$") => match split_string.next() {
            Some("cd") => match split_string.next() {
                Some("..") => Ok(InputLine::CdUp),
                Some("/") => Ok(InputLine::CdRoot),
                Some(x) => Ok(InputLine::CdDir(x.to_string())),
                None => Err(AppError::BadInput.into()),
            },
            Some("ls") => Ok(InputLine::Ls),
            _ => Err(AppError::BadInput.into()),
        },
        Some("dir") => match split_string.next() {
            Some(x) => Ok(InputLine::Directory(x.to_string())),
            None => Err(AppError::BadInput.into()),
        },
        Some(size_str) => {
            let size = size_str.parse::<usize>()?;
            match split_string.next() {
                Some(x) => Ok(InputLine::File(size, x.to_string())),
                None => Err(AppError::BadInput.into()),
            }
        }
        None => Err(AppError::BadInput.into()),
    }
}

fn collect_input<I>(mut input_lines: I) -> Vec<Command>
where
    I: Iterator<Item = InputLine>,
{
    let mut collected = Vec::new();
    let mut output = Vec::new();
    let mut appending_to_output = false;
    let mut input_line_opt = input_lines.next();

    while input_line_opt.is_some() {
        let input_line = input_line_opt.unwrap();
        match input_line {
            InputLine::CdRoot => {
                if appending_to_output {
                    appending_to_output = false;
                    let command_to_append = Command::Ls(output);
                    collected.push(command_to_append);
                    output = Vec::new();
                }

                collected.push(Command::CdRoot);
            }
            InputLine::CdUp => {
                if appending_to_output {
                    appending_to_output = false;
                    let command_to_append = Command::Ls(output);
                    collected.push(command_to_append);
                    output = Vec::new();
                }

                collected.push(Command::CdUp);
            }
            InputLine::CdDir(x) => {
                if appending_to_output {
                    appending_to_output = false;
                    let command_to_append = Command::Ls(output);
                    collected.push(command_to_append);
                    output = Vec::new();
                }

                collected.push(Command::CdDir(x));
            }
            InputLine::Ls => {
                appending_to_output = true;
            }
            InputLine::File(size, name) => {
                if appending_to_output {
                    output.push(FileDesc::File(size, name));
                }
            }
            InputLine::Directory(name) => {
                if appending_to_output {
                    output.push(FileDesc::Directory(name));
                }
            }
        }
        input_line_opt = input_lines.next();
    }

    if appending_to_output {
        let command_to_append = Command::Ls(output);
        collected.push(command_to_append);
    }

    collected
}

#[derive(Debug)]
enum FileOrDir {
    File(usize, String),
    Dir(Rc<RefCell<Filesystem>>, String),
}

#[derive(Debug)]
pub struct Filesystem {
    children: HashMap<String, FileOrDir>,
    parent: Option<Weak<RefCell<Filesystem>>>,
}

impl Filesystem {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            parent: None,
        }
    }

    pub fn pretty_print(fs: &Rc<RefCell<Self>>) {
        Self::pretty_print_with_offset(fs, None, 2);
    }

    fn pretty_print_with_offset(
        fs: &Rc<RefCell<Self>>,
        fs_root_name: Option<String>,
        offset: usize,
    ) {
        let spacing = " ".repeat(offset);
        print!("{}", spacing);
        match fs_root_name {
            Some(x) => {
                println!("{}", x);
            }
            None => {
                println!("/");
            }
        }
        let children = &fs.borrow().children;
        for file in children.values() {
            match file {
                FileOrDir::File(size, name) => {
                    let spacing = " ".repeat(offset + 2);
                    print!("{}", spacing);
                    print!("{} ", name);
                    println!("{}", size);
                }
                FileOrDir::Dir(children, name) => Filesystem::pretty_print_with_offset(
                    children,
                    Some(name.to_string()),
                    offset + 2,
                ),
            }
        }
    }

    fn build(commands: &[Command]) -> Option<Rc<RefCell<Filesystem>>> {
        let root = Rc::new(RefCell::new(Self::new()));
        let mut pwd = Rc::clone(&root);
        for command in commands {
            match command {
                Command::CdRoot => {
                    pwd = Rc::clone(&root);
                }
                Command::CdUp => {
                    let parent = pwd.borrow().parent.as_ref()?.clone();
                    pwd = parent.upgrade().unwrap();
                }
                Command::CdDir(dir) => {
                    let new_pwd;
                    {
                        let children = &pwd.borrow().children;
                        let directory = children.get(dir);
                        match directory {
                            Some(FileOrDir::Dir(reference, _)) => {
                                new_pwd = reference.clone();
                            }
                            _ => {
                                return None;
                            }
                        }
                    }
                    pwd = new_pwd;
                }
                Command::Ls(contents) => {
                    // Needs a weak ref to pwd to act as parent
                    let parent = Rc::downgrade(&pwd);

                    // Need a mutable ref to pwd's children hashmap
                    let hashmap = &mut (pwd.borrow_mut()).children;
                    for file in contents {
                        match file {
                            FileDesc::File(size, name) => {
                                let file_obj = FileOrDir::File(*size, name.to_string());
                                hashmap.insert(name.to_string(), file_obj);
                            }
                            FileDesc::Directory(name) => {
                                let dir_filesystem = Rc::new(RefCell::new(Self {
                                    children: HashMap::new(),
                                    parent: Some(parent.clone()),
                                }));
                                let dir_obj = FileOrDir::Dir(dir_filesystem, name.to_string());
                                hashmap.insert(name.to_string(), dir_obj);
                            }
                        }
                    }
                }
            }
        }
        Some(root)
    }
}

fn add_sizes_up_to_threshold(fs: &Rc<RefCell<Filesystem>>, threshold: usize) -> (usize, usize) {
    let children = &fs.borrow().children;
    let mut size = 0;
    let mut threshold_sum = 0;

    for child in children.values() {
        match child {
            FileOrDir::File(filesize, _) => {
                size += filesize;
            }
            FileOrDir::Dir(child_fs, _) => {
                let (child_size, child_threshold_sum) =
                    add_sizes_up_to_threshold(child_fs, threshold);
                size += child_size;
                threshold_sum += child_threshold_sum;
            }
        }
    }

    if size <= threshold {
        threshold_sum += size
    }

    (size, threshold_sum)
}

fn smallest_directory_to_delete(
    fs: &Rc<RefCell<Filesystem>>,
    threshol: usize,
) -> (usize, Option<usize>) {
    unimplemented!()
}

pub fn day_7_1(filename: &str) -> Result<usize> {
    let input_reader = InputReader::new(filename, parse_input)?;
    let collected_input = collect_input(input_reader);
    let filesystem = Filesystem::build(&collected_input).unwrap();

    Ok(add_sizes_up_to_threshold(&filesystem, 100000).1)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;
    use std::path::Path;

    #[test]
    fn test_day_7_1() {
        let filename = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("inputs/day07-small")
            .into_os_string()
            .into_string()
            .unwrap();
        assert_eq!(day_7_1(&filename).unwrap(), 95437);
    }
}
