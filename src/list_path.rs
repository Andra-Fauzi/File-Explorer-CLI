use std::path::Path;
use std::fs;
use colored::Colorize;

pub fn show_path_in_cwd()
{
    let path = Path::new(".");
    let display = path.display();
    let lists= fs::read_dir(path).expect("cant read the path");
    println!("list of {}",display);
    for list in lists {
        let list = list.unwrap();
        if list.path().is_dir(){
            let showpath = list.path().display().to_string();
            println!("{}",showpath.bright_blue());
        }
        else if list.path().is_file(){
            let showpath = list.path().display().to_string();
            println!("{}",showpath.bright_green());
        }
    }
}