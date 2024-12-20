use linkedin_csv::{
    read_connections_file, read_contacts_file, read_invitations_file, read_messages_file,
    read_search_queries_file, read_shares_file, read_skills_file,
};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} [connections|contacts|invitations|messages|search_queries|shares|skills] PATH_TO_LINKEDIN_FOLDER",
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
        "connections" => match read_connections_file(path) {
            Ok(entries) => {
                for entry in entries {
                    println!("{:?}", entry);
                }
            }
            Err(err) => {
                eprintln!("Could not read Contacts.csv: {err}");
                std::process::exit(1);
            }
        },
        "contacts" => match read_contacts_file(path) {
            Ok(entries) => {
                for entry in entries {
                    println!("{:?}", entry);
                }
            }
            Err(err) => {
                eprintln!("Could not read Contacts.csv: {err}");
                std::process::exit(1);
            }
        },
        "invitations" => match read_invitations_file(path) {
            Ok(entries) => {
                for entry in entries {
                    println!("{:?}", entry);
                }
            }
            Err(err) => {
                eprintln!("Could not read Invitations.csv: {err}");
                std::process::exit(1);
            }
        },
        "messages" => match read_messages_file(path) {
            Ok(entries) => {
                for entry in entries {
                    println!("{:?}", entry);
                }
            }
            Err(err) => {
                eprintln!("Could not read messages.csv: {err}");
                std::process::exit(1);
            }
        },
        "search_queries" => match read_search_queries_file(path) {
            Ok(entries) => {
                for entry in entries {
                    println!("{:?}", entry);
                }
            }
            Err(err) => {
                eprintln!("Could not read SearchQueries.csv: {err}");
                std::process::exit(1);
            }
        },
        "skills" => match read_skills_file(path) {
            Ok(entries) => {
                for entry in entries {
                    println!("{:?}", entry);
                }
            }
            Err(err) => {
                eprintln!("Could not read Skills.csv: {err}");
                std::process::exit(1);
            }
        },
        "shares" => match read_shares_file(path) {
            Ok(entries) => {
                for entry in entries {
                    println!("{:?}", entry);
                }
            }
            Err(err) => {
                eprintln!("Could not read Shares.csv: {err}");
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("The provided table: {table} is not supported");
            std::process::exit(1);
        }
    }
}
