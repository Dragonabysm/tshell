pub mod errors_check {
    use std::io::ErrorKind;
    use std::io::Result;

    pub fn cd_error_check(result: Result<()>) {
        match result {
            Ok(dir) => dir,
            Err(err) => match err.kind() {
                ErrorKind::NotFound => eprintln!("  \x1b[1;31mError: \x1b[0mDirectory not found."),
                ErrorKind::PermissionDenied => {
                    eprintln!("  \x1b[1;31mError: \x1b[0m Permission denied.")
                }
                _ => eprintln!("  \x1b[1;31mError: \x1b[0mIt's not a directory."),
            },
        }
    }

    pub fn unknow_command(command: &str) {
        eprintln!("  \x1b[1;31mError: \x1b[0mUnknow command ({command}), type help to see commands.")
    }

    pub fn remove_file_error_check(result: Result<()>) {
        match result {
            Ok(content) => content,
            Err(err) => match err.kind() {
                ErrorKind::PermissionDenied => eprintln!("  \x1b[1;31mError: \x1b[0mPermission denied."),
                _ => eprintln!("  \x1b[1;31mError: \x1b[0mIt's not a file."),
            }
        }
    }

    pub fn remove_dir_error_check(result: Result<()>) {
        match result {
            Ok(content) => content,
            Err(err) => match err.kind() {
                ErrorKind::PermissionDenied => eprintln!("  \x1b[1;31mError: \x1b[0mPermission denied."),
                _ => eprintln!("  \x1b[1;31mError: \x1b[0mIt's not a directory."),
            }
        }
    }

    
}
