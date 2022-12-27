pub mod funcs {
    use crate::user_assist::errors;
    use crate::user_assist::help;
    use std::env;
    use std::fs;
    use std::io::Read;
    use std::io::Write;
    use sysinfo::{System, SystemExt, CpuExt};

    pub fn ls(args: Vec<&str>) {
        /*  # LS

            List current or referenced directory.

            Usage:
            ls [Optional<Directory>]

            Examples: ls C:\Users\Dragon\Desktop        ls Users
        */
        let paths;

        if args.len() > 1 {
            let dir = args[1];
            paths = fs::read_dir(&dir).unwrap();
        } else {
            let dir = env::current_dir().unwrap();
            paths = fs::read_dir(&dir).unwrap();
        }

        println!("                    Name                  Path");

        for path in paths {
            let file_with_path = path.unwrap().path();
            let file = file_with_path.file_name();

            println!(
                "    {: >20}                  {}",
                file.unwrap().to_string_lossy(),
                file_with_path.display()
            )
        }
    }

    pub fn pwd() {
        /*  # pwd

            Return the absolute actual path.

            Usage:
            pwd

            Examples: pwd
        */
        let dir = env::current_dir().unwrap();
        println!("Path: {}", dir.display())
    }

    pub fn touch(args: Vec<&str>) {
        /*  # touch

            Create a file in actual or referenced path.

            Usage:
            touch [<File Name>] [Optional<File dir>]

            Examples: touch my.txt        touch my.txt C:\Users\Dragon\Desktop
        */

        let dir = env::current_dir().unwrap();

        if !(args.len() > 1) {
            help::help_funcs::touch_help();
        } else {
            let dir = dir.join(format!("{}", args[1]));

            let mut new_file = fs::File::create(dir).expect("Error on create file.");

            new_file.write_all(b"").unwrap();
        }
    }

    pub fn cd(args: Vec<&str>) {
        /*  # cd

            Enter in a path.

            Usage:
            cd [Folder]

            Examples: cd Desktop        cd C:\Users\Dragon\Desktop
        */

        if args.len() > 1 {
            let dir = env::set_current_dir(args[1]);
            errors::errors_check::cd_error_check(dir)
        } else {
            help::help_funcs::cd_help()
        }
    }

    pub fn cat(args: Vec<&str>) {
        /*  # Cat

            Return the content of readable file referenced.

            Usage:
            cat [File dir]

            Examples: cat my.txt        cat C:\Users\Dragon\Desktop\my.txt
        */

        if !(args.len() > 1) {
            panic!("Falta de argumentos")
        }
        let mut file = fs::File::open(args[1]).unwrap();

        let mut file_content = String::new();

        file.read_to_string(&mut file_content).unwrap();

        println!("{file_content}")
    }

    pub fn clear() {
        /*  # Clear

            Clean the terminal.

            Usage:
            clear

            Examples: clear
        */
        for _ in 0..100 {
            println!()
        }
        print!("{esc}c", esc = 27 as char);
    }

    pub fn help() {
        // Table
        println!(
            "  {: >10}                               {: <30}      {}",
            "Command", "Description", "Usage"
        );

        // Cd
        println!(
            "  {: >10}           {: <51}     cd [<Path>]",
            "cd", "Enter in a path."
        );

        // Clear
        println!(
            "  {: >10}           {: <51}     clear",
            "clear", "Clean the terminal."
        );

        // Ls
        println!(
            "  {: >10}           {: <51}     ls [optional<Path>]",
            "ls", "List files and folders in a dir."
        );

        // Pwd
        println!(
            "  {: >10}           {: <51}     pwd",
            "pwd", "Return the absolute actual path."
        );

        // Touch
        println!(
            "  {: >10}           {: <51}     touch [<File Name>]",
            "touch", "Create a file in actual or referenced path."
        );

        // Cat
        println!(
            "  {: >10}           {: <51}     cat [<File dir>]",
            "cat", "Return the content of readable file referenced."
        );
    }

    pub fn treck() {
        let mut sys = System::new_all();
        sys.refresh_all();

        for (mut i, cpu) in sys.cpus().iter().enumerate() {
            i += 1;
            println!("      Cpu{i}: {}%       ", cpu.cpu_usage());
        }
        println!("      total memory: {} bytes", sys.total_memory());
        println!("      used memory : {} bytes", sys.used_memory());
        println!("      System name:             {:?}", sys.name());
        println!("      System kernel version:   {:?}", sys.kernel_version());
        println!("      System OS version:       {:?}", sys.os_version());
    }
}
