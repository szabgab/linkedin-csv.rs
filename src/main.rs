use linkedin_csv::{read_contacts_file, read_skills_file};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} [contacts|skills] PATH_TO_LINKEDIN_FOLDER",
            args[0]
        );
        std::process::exit(1);
    }
    let table = &args[1];
    let path = std::path::Path::new(&args[2]);
    if !path.exists() {
        eprintln!("The provided path: {path:?} does not exist");
        std::process::exit(1);
    }

    match table.as_str() {
        "contacts" => match read_contacts_file(path) {
            Ok(contacts) => {
                for contact in contacts {
                    println!("{:?}", contact);
                }
            }
            Err(err) => {
                eprintln!("Could not read contacts: {err}");
                std::process::exit(1);
            }
        },
        "skills" => match read_skills_file(path) {
            Ok(skills) => {
                for skill in skills {
                    println!("{:?}", skill);
                }
            }
            Err(err) => {
                eprintln!("Could not read contacts: {err}");
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("The provided table: {table} is not supported");
            std::process::exit(1);
        }
    }
}
