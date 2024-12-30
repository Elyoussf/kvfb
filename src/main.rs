use std::{collections::HashMap, env::{self, args}, fs::read_to_string, io::{self, Error, ErrorKind}, path::PathBuf};

// Two formats : from file or accepting key_value pair  
// kvdb  file_name  (the option f means that data will be retrieved 
// from the file and saved in a database which has same name as file_name 

fn handle_arguments() -> Result<PathBuf,io::Error>{
    let mut args = args();
    let size = args.len();
    if size!=2 {
        return Err(Error::new(ErrorKind::InvalidInput, "The usage is simple bro : kvdb path_to_the_file"));
    }
    let relative_file_path = args.nth(1).unwrap(); // am sure it existe so i used unwrap
    let mut path = env::current_dir().expect("Failed to get the current directory");
    path.push(relative_file_path);
    if !path.exists(){
       return Err(Error::new(ErrorKind::InvalidInput, "The provided path does not exist in the current directory!"));
    }
    if path.is_dir(){
        return Err(Error::new(ErrorKind::IsADirectory, "The path provided leads to a directory not a file huh"));
}
    Ok(path)
}

fn open_extract_analyze_save(filename : PathBuf,db : PathBuf) -> Result<HashMap<String,String>,Error>{
    let mut hm = HashMap::new();
    for line in read_to_string(filename).unwrap().lines(){
        let key_value = line.split_whitespace().collect::<Vec<String>>();
        if key_value.size() != 2 {
            return Err(Error::new(ErrorKind::Other,"The syntax is not respected in the provided file"));
        }
        hm.insert(key_value[0], key_value[1]);
    }
    Ok(hm)
}

fn main() {
    
}
