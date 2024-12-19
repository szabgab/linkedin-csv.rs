use linkedin_csv::read_contacts_file;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} PATH_TO_LINKEDIN_FOLDER", args[0]);
        std::process::exit(1);
    }
    let path = std::path::Path::new(&args[1]);
    if !path.exists() {
        eprintln!("The provided path: {path:?} does not exist");
        std::process::exit(1);
    }

    match read_contacts_file(path) {
        Ok(contacts) => {
            for contact in contacts {
                println!("{:?}", contact);
            }
        }
        Err(err) => {
            eprintln!("Could not read contacts: {err}");
            std::process::exit(1);
        }
    }
}
