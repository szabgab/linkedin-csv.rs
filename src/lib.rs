use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct Vote {
    #[serde(rename = "Date")]
    pub date: String,

    #[serde(rename = "Link")]
    pub link: String,

    #[serde(rename = "OptionText")]
    pub option_text: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct SearchQuery {
    #[serde(rename = "Time")]
    pub time: String,

    #[serde(rename = "Search Query")]
    pub search_query: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct Message {
    #[serde(rename = "CONVERSATION ID")]
    pub conversation_id: String,

    #[serde(rename = "CONVERSATION TITLE")]
    pub conversation_title: String,

    #[serde(rename = "FROM")]
    pub from: String,

    #[serde(rename = "SENDER PROFILE URL")]
    pub sender_profile_url: String,

    #[serde(rename = "TO")]
    pub to: String,

    #[serde(rename = "RECIPIENT PROFILE URLS")]
    pub recipient_profile_urls: String,

    #[serde(rename = "DATE")]
    pub date: String,

    #[serde(rename = "SUBJECT")]
    pub subject: String,

    #[serde(rename = "CONTENT")]
    pub content: String,

    #[serde(rename = "FOLDER")]
    pub folder: String,

    #[serde(rename = "IS MESSAGE DRAFT")]
    pub is_message_draft: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct Connection {
    #[serde(rename = "First Name")]
    pub first_name: String,

    #[serde(rename = "Last Name")]
    pub last_name: String,

    #[serde(rename = "URL")]
    pub url: String,

    #[serde(rename = "Email Address")]
    pub email_address: String,

    #[serde(rename = "Company")]
    pub company: String,

    #[serde(rename = "Position")]
    pub position: String,

    #[serde(rename = "Connected On")]
    pub connected_on: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct Invitation {
    #[serde(rename = "From")]
    pub from: String,

    #[serde(rename = "To")]
    pub to: String,

    #[serde(rename = "Sent At")]
    pub sent_at: String,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "Direction")]
    pub direction: String,

    #[serde(rename = "inviterProfileUrl")]
    pub inviter_profile_url: String,

    #[serde(rename = "inviteeProfileUrl")]
    pub invitee_profile_url: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct Skill {
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct Share {
    #[serde(rename = "Date")]
    pub date: String,

    #[serde(rename = "ShareLink")]
    pub share_link: String,

    #[serde(rename = "ShareCommentary")]
    pub share_commentary: String,

    #[serde(rename = "SharedUrl")]
    pub shared_url: String,

    #[serde(rename = "MediaUrl")]
    pub media_url: String,

    #[serde(rename = "Visibility")]
    pub visibility: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct Contact {
    #[serde(rename = "Source")]
    pub source: String,

    #[serde(rename = "FirstName")]
    pub first_name: String,

    #[serde(rename = "LastName")]
    pub last_name: String,

    #[serde(rename = "Companies")]
    pub companies: String,

    #[serde(rename = "Title")]
    pub title: String,

    #[serde(rename = "Emails")]
    pub emails: String,

    #[serde(rename = "PhoneNumbers")]
    pub phone_numbers: String,

    #[serde(rename = "CreatedAt")]
    pub created_at: String,

    #[serde(rename = "Addresses")]
    pub addresses: String,

    #[serde(rename = "Sites")]
    pub sites: String,

    #[serde(rename = "InstantMessageHandles")]
    pub instant_message_handles: String,

    #[serde(rename = "FullName")]
    pub full_name: String,

    #[serde(rename = "Birthday")]
    pub birthday: String,

    #[serde(rename = "Location")]
    pub location: String,

    #[serde(rename = "BookmarkedAt")]
    pub bookmarked_at: String,

    #[serde(rename = "Profiles")]
    pub profiles: String,
}

macro_rules! read_file {
    ($name:ident, $struct:ty, $file:expr) => {
        pub fn $name(path: &Path) -> Result<Vec<$struct>, Box<dyn Error>> {
            let filepath = path.join($file);
            let mut records: Vec<$struct> = vec![];
            let fh = File::open(&filepath)?;
            let mut rdr = csv::Reader::from_reader(fh);
            for result in rdr.deserialize() {
                let record: $struct = result?;
                records.push(record);
            }
            Ok(records)
        }
    };
}

read_file!(read_contacts_file, Contact, "Contacts.csv");
read_file!(read_skills_file, Skill, "Skills.csv");
read_file!(read_shares_file, Share, "Shares.csv");
read_file!(read_invitations_file, Invitation, "Invitations.csv");
read_file!(read_messages_file, Message, "messages.csv");
read_file!(read_votes_file, Vote, "Votes.csv");

pub fn read_connections_file(path: &Path) -> Result<Vec<Connection>, Box<dyn Error>> {
    let filepath = path.join("Connections.csv");
    let mut records: Vec<Connection> = vec![];
    let fh = File::open(&filepath)?;
    let mut br = std::io::BufReader::new(&fh);

    // This file has the following message at the top, we need to skip that.
    // Notes:
    // "When exporting your connection data, you may notice that some of the email addresses are missing. You will only see email addresses for connections who have allowed their connections to see or download their email address using this setting https://www.linkedin.com/psettings/privacy/email. You can learn more here https://www.linkedin.com/help/linkedin/answer/261"

    for _ in 0..3 {
        br.skip_until(b'\n')
            .expect("expecting Notes before the header");
    }

    let mut rdr = csv::Reader::from_reader(br);
    for result in rdr.deserialize() {
        let record: Connection = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn read_search_queries_file(path: &Path) -> Result<Vec<SearchQuery>, Box<dyn Error>> {
    let filepath = path.join("SearchQueries.csv");
    let mut records: Vec<SearchQuery> = vec![];
    let fh = File::open(&filepath)?;
    let mut rdr = csv::Reader::from_reader(fh);
    for result in rdr.deserialize() {
        let record: SearchQuery = result?;
        records.push(record);
    }
    Ok(records)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 42;
        assert_eq!(result, 42);
    }
}
