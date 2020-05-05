use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn create_file(name: String) -> std::io::Result<()> {
    let extension = ".tdl".to_string();
    let mut _file_name = String::from(name + &extension);

    println!("New file initialized: {}", _file_name);

    let mut _file = File::create(_file_name)?;

    Ok(())
}

pub fn write_to_file(content: String, name: &String) -> std::io::Result<()> {
    let path = std::path::Path::new(name);
    let mut _target_file = OpenOptions::new().append(true).open(path).unwrap();

    //Convert String type to stir (str)
    let str_content = str::Str::from(content);

    write!(&mut _target_file, "{}\n", str_content)?;

    Ok(())
}

pub fn remove_file(name: &String) -> std::io::Result<()> {
    let filename = format!("{}{}", name, ".tdl");
    let path = std::path::Path::new(&filename);

    fs::remove_file(path).expect("Failed to remove file.");
    Ok(())
}

pub fn rename_file(old_name: &String, new_name: &String) -> std::io::Result<()> {
    let old_filename = format!("{}{}", old_name, ".tdl");
    let new_filename = format!("{}{}", new_name, ".tdl");

    let path = std::path::Path::new(&old_filename);

    fs::rename(path, new_filename).expect("Failed to rename file.");

    Ok(())
}

pub fn check_if_exists(name: &String) -> bool {
    let filename = format!("{}{}", name, ".tdl");
    let _target_file = std::path::Path::new(&filename);

    _target_file.exists()
}

pub fn read_from_file<R: Read>(name: R) -> Result<Vec<String>, Error> {
    let _target_file = BufReader::new(name);

    //We collect our data
    //It's best to look at this, study, and see what it's doing
    _target_file
        .lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn in_file_mark_item_as(mark_status: &String, name: String, item: String) -> std::io::Result<()> {
    let filename = &format!("{}{}", name, ".tdl");
    let path = File::open(filename)?;
    let _target_file = BufReader::new(path);
    
    let mut info: Vec<String>;

    #[allow(unused_assignments)]
    let mut mark: char = ' ';

    if mark_status == &"done" || mark_status == &"complete" {
        println!("Marked as done");
        mark = 'x';
    } else if mark_status == &"incomplete" {
        println!("Marked as incomplete");
        mark = ' ';
    } else {
        panic!()
    }
    //We search through file and see if a line contains it
    info = read_from_file(_target_file)?;

    for (index, list_item) in info.iter().enumerate() {
        if list_item.contains(&item) {
            println!("index {}", index);
            let item_status = format!("[{}] {}", mark, item);

            info[index] = item_status.to_owned();

            let target_path = std::path::Path::new(filename);

            let mut _target_file = OpenOptions::new().write(true).open(target_path)?;

            for refresh in info.iter(){
                write!(&mut _target_file, "{}\n", refresh)?;
            }

            break;
        }
    }

    Ok(())
}
