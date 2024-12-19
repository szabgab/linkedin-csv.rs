use std::error::Error;
use std::fs::File;
use std::path::Path;

use serde::Deserialize;

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

pub fn read_contacts_file(path: &Path) -> Result<Vec<Contact>, Box<dyn Error>> {
    let filepath = path.join("Contacts.csv");
    let mut records: Vec<Contact> = vec![];
    let fh = File::open(&filepath)?;
    let mut rdr = csv::Reader::from_reader(fh);
    for result in rdr.deserialize() {
        let record: Contact = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn read_skills_file(path: &Path) -> Result<Vec<Skill>, Box<dyn Error>> {
    let filepath = path.join("Skills.csv");
    let mut records: Vec<Skill> = vec![];
    let fh = File::open(&filepath)?;
    let mut rdr = csv::Reader::from_reader(fh);
    for result in rdr.deserialize() {
        let record: Skill = result?;
        records.push(record);
    }
    Ok(records)
}

pub fn read_shares_file(path: &Path) -> Result<Vec<Share>, Box<dyn Error>> {
    let filepath = path.join("Shares.csv");
    let mut records: Vec<Share> = vec![];
    let fh = File::open(&filepath)?;
    let mut rdr = csv::Reader::from_reader(fh);
    for result in rdr.deserialize() {
        let record: Share = result?;
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
