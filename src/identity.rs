use std::path::Path;
use std::fs;
use promptly::prompt;
use rusqlite::{Connection, params};
use rusqlite::NO_PARAMS;
use git_config::file::GitConfig;
use std::convert::TryFrom;
struct Identity {
    id: i32,
    idname: String,
    name: String,
    email: String
}

fn identity_prompt() {
    let path = Path::new(&format!("{}/gitdentity", dirs::config_dir().expect("Error reading config dir").to_str().unwrap())).to_owned();
    let pathstr = String::from(path.to_str().unwrap());
    let pathstrclone = pathstr.clone();
    let input = read_user_input_int();
    match input {
        1 => create_identity(pathstr),
        2 => delete_identity(pathstrclone),
        3 => std::process::exit(0),
        _ => {
            println!("Invalid input");
            identity_prompt();
        }
    }
}

pub fn edit_identity() {
    let path = Path::new(&format!("{}/gitdentity", dirs::config_dir().expect("Error reading config dir").to_str().unwrap())).to_owned();
    let pathstr = String::from(path.to_str().unwrap());
    let pathstrclone = pathstr.clone();
    if !path.exists() {
        create_database(pathstr).unwrap()
    }
    println!("Gitdentity");
    println!("Current Identities:");
    println!("------------------");
    get_identity(pathstrclone);
    println!("1) Add an Identity");
    println!("2) Delete an Identity");
    println!("3) Exit this program");
    identity_prompt()
}

fn get_identity(pathstr: String) {
    let conn = Connection::open(format!("{}/database.db", pathstr)).unwrap();
    let mut identity = conn.prepare("
    SELECT * from identities
    ").unwrap();
    let identities = identity.query_map(NO_PARAMS, |row| {
        Ok(Identity {
            id: row.get(0)?,
            idname: row.get(1)?,
            name: row.get(2)?,
            email: row.get(3)?
        })
    }).unwrap();
    for identity in identities {
        let var = identity.unwrap();
        println!("{} | {} | {} | {}", var.id, var.idname, var.name, var.email);
    }
}

fn create_identity(pathstr: String) {
    let idname: String = prompt("What idname do you want the identity to have").expect("Error");
    let name: String = prompt("What name do you want your identity to have").expect("Error");
    let email: String = prompt("What email do you want your identity to have").expect("Error");
    let conn = Connection::open(format!("{}/database.db", pathstr)).unwrap();
    conn.execute(
        "INSERT INTO identities (idname, name, email) VALUES (?1, ?2, ?3)", 
        &[idname, name, email]).expect("An error has occured when trying to insert into the table");
    println!("I have created the identity");
    get_identity(pathstr);
    identity_prompt();
}

fn delete_identity(pathstr: String) {
    let iden: i32 = prompt("What identity do you wanna delete").expect("Error");
    let conn = Connection::open(format!("{}/database.db", pathstr)).unwrap();
    conn.execute(
        "DELETE from identities WHERE id = (?1)
        ", 
        &[&iden]).unwrap();
    println!("I have deleted the identity with the ID {}", iden);
    get_identity(pathstr);
    identity_prompt();
}

pub fn set_identity(identity: String) {
    let homedir = dirs::home_dir().unwrap();
    let path = Path::new(&format!("{}/gitdentity", dirs::config_dir().expect("Error reading config dir").to_str().unwrap())).to_owned();
    let pathstr = String::from(path.to_str().unwrap());
    let conn = Connection::open(format!("{}/database.db", pathstr)).unwrap();
    let mut statement = conn.prepare("SELECT * FROM identities WHERE idname=(?1)").unwrap();
    let iden = statement.query_map(params![identity], |row| {
        Ok(Identity {
            id: row.get(0)?,
            idname: row.get(1)?,
            name: row.get(2)?,
            email: row.get(3)?,
        })
    }).unwrap();
    for id in iden {
        let i = id.unwrap();
        let gitconfig = fs::read_to_string(format!("{}/.gitconfig", homedir.to_str().unwrap()))
            .expect("Failed to read .gitconfig");
        let sus: &str = &gitconfig;
        let mut config = GitConfig::try_from(sus).unwrap();
        config.set_raw_value("user", None, "name", i.name.as_bytes().to_vec()).unwrap();
        config.set_raw_value("user", None, "email", i.email.as_bytes().to_vec()).unwrap();
        fs::write(format!("{}/.gitconfig", homedir.to_str().unwrap()), config.to_string()).expect("Error writing file");
        println!("I have wrote the {} identity to {}", i.idname, format!("{}/.gitconfig", homedir.to_str().unwrap()));
    }
}

fn create_database(dir: String) -> std::io::Result<()> {
    println!("Creating database");
    let dirclone = dir.clone();
    fs::create_dir(dir)?;
    let conn = Connection::open(format!("{}/database.db", dirclone)).unwrap();
    conn.execute(
        "create table if not exists identities (
            id integer primary key,
            idname text not null,
            name text not null,
            email text not null
        )",
        NO_PARAMS,
    ).unwrap();
    Ok(())
}

fn read_user_input_int() -> i32 {
    let i: i32 = prompt("> ").expect("Please enter a number");
    i
}
