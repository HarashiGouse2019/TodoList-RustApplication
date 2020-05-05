//Commands enumerator
pub enum COMMANDS{
    Show,
    Add,
    Initialize,
    Remove,
    Rename,
    Mark,
    Target,
    Help,
    Nil
}


//Parse user commands for match statement
pub fn parse_command(command: String) -> COMMANDS{
    match command.as_ref() {
        "show" => return COMMANDS::Show,
        "add" => return COMMANDS::Add,
        "init" => return COMMANDS::Initialize,
        "remove" => return COMMANDS::Remove,
        "rename" => return COMMANDS::Rename,
        "mark" => return COMMANDS::Mark,
        "target" => return COMMANDS::Target,
        "help!" => return COMMANDS::Help,
        _ => return COMMANDS::Nil
    }
}