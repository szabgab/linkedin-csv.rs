use linkedin_csv::read_contacts_file;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} PATH_TO_LINKEDIN_FOLDER", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];

    let contacts = read_contacts_file(path).unwrap();

    for contact in contacts {
        println!("{:?}", contact);
    }
}
