use linkedin_csv::{
    read_ad_targeting_file, read_ads_clicked_file, read_connections_file, read_contacts_file,
    read_invitations_file, read_messages_file, read_recommendations_received_file,
    read_saved_items_file, read_search_queries_file, read_shares_file, read_skills_file,
    read_votes_file,
};

const TABLES: [&str; 12] = [
    "ads_clicked",
    "ad_targeting",
    "connections",
    "contacts",
    "invitations",
    "messages",
    "recommendations_received",
    "saved_items",
    "search_queries",
    "shares",
    "skills",
    "votes",
];

macro_rules! call_function {
    ($name:ident, $path:ident, $file:expr) => {
        match $name($path) {
            Ok(entries) => {
                for entry in entries {
                    println!("{:?}", entry);
                }
            }
            Err(err) => {
                eprintln!("Could not read '{}': {err}", $file);
                std::process::exit(1);
            }
        }
    };
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        let params = TABLES.join("|");
        eprintln!("Usage: {} [{params}] PATH_TO_LINKEDIN_FOLDER", args[0]);
        std::process::exit(1);
    }
    let table = &args[1];
    let path = std::path::Path::new(&args[2]);
    if !path.exists() {
        eprintln!("The provided path: {path:?} does not exist");
        std::process::exit(1);
    }

    read_and_print_table(table, path);
}

fn read_and_print_table(table: &str, path: &std::path::Path) {
    match table {
        "ads_clicked" => call_function!(read_ads_clicked_file, path, "Ads Clicked.csv"),
        "ad_targeting" => call_function!(read_ad_targeting_file, path, "Ad_Targeting.csv"),
        "connections" => call_function!(read_connections_file, path, "Connections.csv"),
        "contacts" => call_function!(read_contacts_file, path, "Contacts.csv"),
        "invitations" => call_function!(read_invitations_file, path, "Invitations.csv"),
        "messages" => call_function!(read_messages_file, path, "messages.csv"),
        "recommendations_received" => call_function!(
            read_recommendations_received_file,
            path,
            "Recommendations_Received.csv"
        ),
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

#[cfg(test)]
mod tests {
    use crate::{read_and_print_table, TABLES};

    // In order to run the tests, you need to have the data folder in the root of the project
    // or at least you need a symbolic link called "data" pointing to the folder where the data is stored

    #[test]
    fn it_works() {
        let path = std::path::Path::new("data");

        if path.exists() {
            for table in TABLES {
                if table == "ad_targeting" {
                    continue;
                }
                if table == "search_queries" {
                    continue;
                }
                read_and_print_table(table, path);
            }
        }
    }
}
