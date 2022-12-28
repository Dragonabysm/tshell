pub mod searcher {
    use crate::funcs::funcs;
    use crate::user_assist::errors::errors_check::unknow_command;

    pub fn matcher(args: Vec<&str>) {
        match args[0] {
            "ls" => funcs::ls(args),
            "pwd" => funcs::pwd(),
            "touch" => funcs::touch(args),
            "cd" => funcs::cd(args),
            "cat" => funcs::cat(args),
            "clear" => funcs::clear(),
            "help" => funcs::help(),
            "treck" => funcs::treck(),
            "info" => funcs::info(),
            "rm" => funcs::rm(args),
            "" => {},
            "\n" => {},
            &_ => unknow_command(args[0]),
        }
    }
}
