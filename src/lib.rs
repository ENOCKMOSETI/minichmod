use std::fs;
use std::error::Error;
use std::os::unix::fs::PermissionsExt;

pub struct Config {
    pub query: String,
    pub filename: String,
}


// struct Role {
//     u: bool,
//     g: bool,
//     o: bool,
// }
 
// struct Operator {
//     s: bool,
//     a: bool,
//     e: bool,
// }

// struct Perm {
//     r: bool,
//     w: bool,
//     x: bool,
// }

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename,})
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let filemetadata = fs::metadata(&config.filename)?;

    let mut filepermissions = filemetadata.permissions();

    println!("{:#?}", filepermissions);

    filepermissions.set_mode(777);
    
    println!("{:#?}", filepermissions);

    println!("{:#?}", filemetadata);
    
    Ok(())
}


