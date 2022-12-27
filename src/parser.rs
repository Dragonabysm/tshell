pub mod searcher {
    use super::funcs;

    pub fn matcher(args: Vec<&str>) {
        match args[0] {
            "ls" => funcs::ls(args),
            "pwd" => funcs::pwd(args),
            "touch" => funcs::touch(args),
            "cd" => funcs::cd(args),
            "cat" => funcs::cat(args),
            "clear" => funcs::clear(args),
            &_ => {}
        }
    }
}

pub mod funcs {
    use std::env::current_dir;
    use std::io::Read;
    use std::io::Write;
    use std::env;
    use std::fs;

    pub fn ls(args: Vec<&str>) {
        let mut paths;
       
        if args.len() > 1 {
            let dir = args[1];
            paths = fs::read_dir(&dir).unwrap();
        } else {
            let dir = env::current_dir().unwrap();   
            paths = fs::read_dir(&dir).unwrap();      
        }
       

        println!("    Name                  Path");

        for path in paths {
            let file_with_path = path.unwrap().path();
            let file = file_with_path.file_name();
            

            println!("    {:?}         {}", file.unwrap().to_os_string(), file_with_path.display())
        }
    }

    pub fn pwd(args: Vec<&str>) {
        let dir = env::current_dir().unwrap();
        println!("Path: {}", dir.display())
    }

    pub fn touch(args: Vec<&str>) {
        let mut dir = env::current_dir().unwrap();

        if !(args.len() > 1) {
            panic!("Falta de argumentos")
        }

        if args.len() > 2 {
            dir = args[2].into();
        }

        let dir = dir.join(format!("{}", args[1]));


        let mut new_file = fs::File::create(dir).expect("Error on create file.");

        new_file.write_all(b"");
    }

    pub fn cd(args: Vec<&str>) {
        if !(args.len() > 1) {
            panic!("Falta de argumentos")
        }
        let dir = env::set_current_dir(args[1]);
    }

    pub fn cat(args: Vec<&str>) {
        if !(args.len() > 1) {
            panic!("Falta de argumentos")
        }
        let mut file = fs::File::open(args[1]).unwrap();

        let mut file_content = String::new();

        file.read_to_string(&mut file_content);

        println!("{file_content}")
    }

    pub fn clear(args: Vec<&str>) {
        for _ in 0..100 {
            println!()
        }
        print!("{esc}c", esc = 27 as char);
    }

}