//The actual application itself

//Using standard library
use ::str::Str;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Error;

//Using console crate
use console::Term;

//Use all of our created modules
mod command;            //So that we can validate the commands used in the application
mod file_management;    //To perform multiple IO tasks

//Our TodoItem structure
struct TodoItem {
    name: &'static str,
    completed: char,
}

//Implement TodoItem with a new function
impl TodoItem {
    fn new(new_name: &'static str) -> TodoItem {
        return TodoItem {
            name: new_name,
            completed: ' ',
        };
    }
}

//Impliment Copy trait for TodoItem
impl Copy for TodoItem {}

//Impliment Clone trat and function for TodoItem
impl Clone for TodoItem {
    fn clone(&self) -> TodoItem {
        *self
    }
}

//Create a TodoList Structure
struct TodoList {
    list: Vec<TodoItem>,
}

//Impliment new, add, and show function to TodoList structure
impl TodoList {
    //Create a new TodoList
    fn new() -> TodoList {
        return TodoList { list: Vec::new() };
    }

    //Add an item to the list
    fn add_to_list(&mut self, name: &'static str, target: &String) {
        println!("Doing a thing!!!");

        let todo_item = TodoItem::new(name).clone();
        let item = todo_item.name;
        let complete = todo_item.completed.to_string();

        let filename = format!("{}{}", target, ".tdl");

        file_management::write_to_file(format!("[{}] {}", complete, item), &filename)
            .expect("Failed to write to file...");

        self.list.push(todo_item);
    }

    //Show your currentl list of items based on directory
    fn show_list(&self, name: String) -> Result<(), Error> {
        let conversion_name: Str = Str::from(name);
        let owned_name = conversion_name.to_owned();

        let vec: Vec<String> =
            file_management::read_from_file(File::open(owned_name.to_string() + ".tdl")?)?;

        for item in vec {
            println!("{}", item);
        }

        Ok(())
    }
}

pub fn run() -> std::io::Result<()> {
    /*On run, 
    *tell the user that it's set to default
    *and that if they want to target outside of the default directory
    *they must use cargo run target [directory]
    */

    //Generate tar.dir file
    generate_target_directory_file()?;


    //Get whatever value is saved on the target directory file
    let target_directory = get_target_from_target_directory_file().unwrap().clone();

    //Alert user
    println!("Current target directory is {}", &target_directory);

    //Research what collect() is and why args let us use it
    let arguments: Vec<String> = env::args().collect();
    let command = arguments[1].clone();

    //To receive user input
    let terminal = Term::stdout();
    
    //Create a new TodoList
    let mut todo_list = TodoList::new();

    match command::parse_command(command.to_string()) {
        //If user command is "show", show our to-do list
        command::COMMANDS::Show => {
            terminal.clear_screen().unwrap();

            let target_args = arguments[2].clone();

            println!(
                "Showing Todo List from {target_file}...",
                target_file = format!("{}{}", &target_directory.clone(), target_args)
            );

            //Check if file exists
            if file_management::check_if_exists(&target_args) {
                todo_list
                    .show_list(target_args)
                    .expect("Something went wrong...");
            }
            //If not... I guess we'll panic
            else {
                panic!();
            }
        }
        
        //If user command is "add", check for 2nd arg, and add to list
        command::COMMANDS::Add => {
            terminal.clear_screen().unwrap();
            alert("Adding task...", true)?;

            let task_args = arguments[2].clone();

            //Check for to token
            let to_token_args = arguments[3].clone();
            let appropriate_token_used: bool =
                is_token_used_in_arg("to".to_string(), to_token_args);

            let target_args = arguments[4].clone();

            //Convert task_args to lifetime_task_args
            let lifetime_task_args: &'static str = string_to_static_str(task_args);

            //Check if our file exists
            if appropriate_token_used {
                todo_list.add_to_list(lifetime_task_args, &format!("{}{}", &target_directory, target_args));

                println!(
                    "Showing updated list from {target_file}...",
                    target_file = format!("{}{}", &target_directory, target_args)
                );

                todo_list.show_list(format!("{}{}", &target_directory, target_args).clone()).expect("Something went wrong...");
            }
            //If not, we'll create the file
            else {
                panic!();
            }
        }

        //If user command is "init", initiate a new to-do list
        command::COMMANDS::Initialize => {
            terminal.clear_screen().unwrap();
            alert("Initializing a new to-do list...", true)?;
            let new_tdl = arguments[2].clone();

            file_management::create_file(format!("{}{}", &target_directory, new_tdl)).expect("File wasn't created");
        }

        //If user command is "remove", remove a to-do list
        command::COMMANDS::Remove => {
            terminal.clear_screen().unwrap();
            alert("Removing specified to-do list...", true)?;

            let target_args = arguments[2].clone();
            file_management::remove_file(&format!("{}{}", &target_directory, target_args))?;

            println!(
                "Removed {target_list}",
                target_list = target_args.clone()
            );
        }

        //If user command is "rename", rename a to-do list
        command::COMMANDS::Rename => {
            terminal.clear_screen().unwrap();
            alert("Renaming to-do list...", true)?;

            //What the file we're targetting
            let old_name_args = arguments[2].clone();

            //Check for to token
            let to_token_args = arguments[3].clone();
            let appropriate_token_used: bool =
                is_token_used_in_arg("to".to_string(), to_token_args);

            //what will our new name be
            let new_name_args = arguments[4].clone();

            //Now we rename our file
            if appropriate_token_used {
                file_management::rename_file(&format!("{}{}", &target_directory, old_name_args), &format!("{}{}", &target_directory, new_name_args))?;
                terminal.clear_screen().unwrap();

                println!(
                    "{old_name} was renamed to {new_name} Showing updated list from {new_name}...",
                    old_name = old_name_args.clone(),
                    new_name = new_name_args.clone()
                );

                todo_list.show_list(format!("{}{}", &target_directory, new_name_args)).expect("Something went wrong...");
            } else {
                panic!();
            }
        }

        //If user command is "mark", mark a to-list
        command::COMMANDS::Mark => {
            //What the file we're targetting
            let task_args = arguments[2].clone();

            //Check for to token
            let as_token_args = arguments[3].clone();
            let in_token_args = arguments[5].clone();

            //what will our new name be
            let status_args = arguments[4].clone();

            //Target file
            let target_args = arguments[6].clone();

            //We'll mark our item complete
            //This one will be a little complicated, since we have to
            if is_token_used_in_arg("as".to_string(), as_token_args)
            && is_token_used_in_arg("in".to_string(), in_token_args)
            && status_args != "".to_string() {
                //We has a grab task function, so that we can mark it as done
                //...
                file_management::in_file_mark_item_as(&status_args, format!("{}{}", &target_directory, target_args), task_args.clone()).unwrap();

                terminal.clear_screen().unwrap();

                println!(
                    "Showing updated list from {target_file}...",
                    target_file = target_args
                );

                todo_list.show_list(format!("{}{}", &target_directory, target_args)).expect("Something went wrong...");
            } else{
                panic!();
            }
        }

        //If the user command is target, change directory location
        command::COMMANDS::Target => {
            terminal.clear_screen().unwrap();

            //Basically with this function, the path has to be specific
            //Once path is set, you access to your list with that target
            //so that you don't have to retype a long directory.

            //Get our new directory
            let new_directory = arguments[2].clone();

            #[allow(unused_assignments)]
            let mut unwrapped_target_directory = &target_directory;

            //We'll change our path with our new directory
            unwrapped_target_directory = &new_directory;

            //String
            let string_now = unwrapped_target_directory.to_string();

            //Update the target directory folder
            update_target_directory_file(string_now)?;
        }

        //If user command is help!, print out documentation
        command::COMMANDS::Help => {
            terminal.clear_screen().unwrap();
            print_out_help();
        }

        //If user command is null, have the console panic
        command::COMMANDS::Nil => panic!(),
    };

    Ok(())
}

//This shouldn't be used so lightly
//since this is causing a memory leak,
//meaning that memory is never released.
fn string_to_static_str(s: String) -> &'static str {
    let x = Box::new(s);
    let ptr = Box::into_raw(x);
    let x = unsafe { Box::from_raw(ptr) };

    Box::leak(x)
}

//Alert the user of any actions
fn alert(s: &str, c: bool) -> std::io::Result<()> {
    let terminal = Term::stdout();

    match c {
        true => {
            terminal.clear_line()?;
            terminal.write_line(s)?;
        }
        false => {
            terminal.write_line(s)?;
        }
    }

    Ok(())
}

//Check if the token was used in a given context
fn is_token_used_in_arg(tokens: String, arg: String) -> bool {
    return arg == tokens;
}

//Generate a target directory file to keep track of last directory accessed
fn generate_target_directory_file() -> std::io::Result<()> {
    //Create file
    let path = std::path::Path::new("tar.dir");
    let mut tar_dir_file = OpenOptions::new().create(true).write(true).open(path).unwrap();

    let content = str::Str::from(r"C:\Users\Tokusunei\Desktop\Rust(Console)\todo-cli\");

    //Write directory to that file specified
    write!(&mut tar_dir_file, "{}", &content)?;

    Ok(())
}

//Update the target directory
fn update_target_directory_file(content: String) -> std::io::Result<()> {
    //Create file
    let path = std::path::Path::new("tar.dir");

    //Set a file that is able to be modified, and unwrap it;
    let mut tar_dir_file = OpenOptions::new().write(true).open(path).unwrap();

    //Create a sring variable from out content
    let str_content = str::Str::from(content);

    //Write directory to that file specified
    write!(&mut tar_dir_file, "{}", &str_content)?;

    //Print out our new directory target
    println!("Target directory has been changed to {}", str_content);

    //The function went through successfully
    Ok(())
}

//Return the last accessed target directory
fn get_target_from_target_directory_file() -> std::io::Result<String> {
    //The file name, which will be of type String
    let filename = String::from(r"C:\Users\Tokusunei\Desktop\Rust(Console)\todo-cli\tar.dir");

    //Creat a new bath with our file. We'll use the borrow operator to assure that we can use it;
    let path = std::path::Path::new(&filename);

    //Create a string with a result
    let read_target: Result<String, Error>;

    //Read information from file
    read_target = std::fs::read_to_string(path);

    //Return the target
    read_target
}

//Print out help whenever the user needs it
fn print_out_help(){
    print!(
    "
    Help is on the way...

    [SHOW COMMAND]
    syntax : show...[list]
    example: 
        cargo run show \"My-Grocery-List\"
        cargo run show \"C:\\Users\\Tokusunei\\Desktop\\My-Grocery-List\"

    [ADD COMMAND <must use 'to' token>]
    syntax : add....[task] to [list]
    example:
        cargo run add \"Buy food for the dogs\" to \"My-Grocery-List\"
        cargo run add \"Buy food for the dogs\" to \"C:\\Users\\Tokusunei\\Desktop\\My-Grocery-List\"

    [RENAME COMMAND <must use 'to' token>]
    syntax : rename.[list] to [new name]
    example:
        cargo run rename \"My-Grocery-List\" to \"Chores\"
        cargo run rename \"C:\\Users\\Tokusunei\\Desktop\\My-Grocery-List\" to \"Chores\"

    [REMOVE COMMAND]
    syntax : remove.[list]
    example:
        cargo run remove \"My-Grocery-List\"    
        cargo run remove \"C:\\Users\\Tokusunei\\Desktop\\My-Grocery-List)\"   

    [MARK COMMAND <must use 'as' and 'in' tokens>]
    syntax : mark...[task] as [status] in [list]
    example: 
        cargo run mark \"Buy food for the dogs\" as \"complete\" in \"My-Grocery-List\"   
        cargo run mark \"Buy food for the dogs\" as \"incomplete\" in \"C:\\Users\\Tokusunei\\Desktop\\My-Grocery-List)\"

        *You could also type \"done\" and \"not-done\" as a status for marking and unmarking
        *status values are interchangeable to each other

    [TARGET COMMAND]
    syntax : target...[directory]
    example: 
        cargo run target \"C:\\Users\\Tokusunei\\Desktop\\My-Grocery-List)\" 
        cargo run target \"C:\\Users\\Tokusunei\\Desktop\\My-Other-Grocery-List)\"

        *The last directory that you used will be saved...
    "
); 
}