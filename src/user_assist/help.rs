pub mod help_funcs {
    pub fn cd_help() {
        println!(
            "       {: ^40} \n  Example: \x1b[1;36mcd Users/   cd C:/Users/Dragon/Desktop/\x1b[0m",
            "Enter in a directory."
        )
    }

    pub fn touch_help() {
        println!(
            "       {: ^40} \n  Example: \x1b[1;36mtouch my.txt   touch C:/Users/Dragon/my.txt\x1b[0m",
            "Create a file."
        )
    }

    pub fn cat_help() {
        println!(
            "       {: ^40} \n  Example: \x1b[1;36mcat my.txt   cat C:/Users/Dragon/my.txt\x1b[0m",
            "Returns the content of a file."
        )
    }

    pub fn rm_help() {
        println!(
            "       {: ^40} \n  Example: \x1b[1;36mrm -rf my.txt   rm C:/Users/Dragon/Desktop/Folder\x1b[0m",
            "Remove a file or directory."
        )
    }
}
