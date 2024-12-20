use linkedin_csv::{
    read_connections_file, read_contacts_file, read_invitations_file, read_messages_file,
    read_saved_items_file, read_search_queries_file, read_shares_file, read_skills_file,
    read_votes_file,
};

macro_rules! call_function {
    ($name:ident, $path:ident, $file:expr) => {
        match $name($path) {
            Ok(entries) => {
                for entry in entries {
                    println!("{:?}", entry);
                }
            }
            Err(err) => {
                eprintln!("Could not read $file: {err}");
                std::process::exit(1);
            }
        }
    };
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} [connections|contacts|invitations|messages|search_queries|shares|skills|votes] PATH_TO_LINKEDIN_FOLDER",
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
        "connections" => call_function!(read_connections_file, path, "Connections.csv"),
        "contacts" => call_function!(read_contacts_file, path, "Contacts.csv"),
        "invitations" => call_function!(read_invitations_file, path, "Invitations.csv"),
        "messages" => call_function!(read_messages_file, path, "messages.csv"),
        "saved_items" => call_function!(read_saved_items_file, path, "Saved_Items.csv"),
        "search_queries" => call_function!(read_search_queries_file, path, "SearchQueries.csv"),
        "shares" => call_function!(read_shares_file, path, "Shares.csv"),
        "skills" => call_function!(read_skills_file, path, "Skills.csv"),
        "votes" => call_function!(read_votes_file, path, "Votes.csv"),
        _ => {
            eprintln!("The provided table: {table} is not supported");
            std::process::exit(1);
        }
    }
}
