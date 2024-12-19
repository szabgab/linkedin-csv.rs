use std::error::Error;
use std::fs::File;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct Record {
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

pub fn read_contacts_file(dir_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let path = std::path::Path::new(dir_path);
    println!("{:?}", path);
    let filepath = path.join("Contacts.csv");
    let mut records: Vec<Record> = vec![];
    match File::open(&filepath) {
        Ok(file) => {
            let mut rdr = csv::Reader::from_reader(file);
            for result in rdr.deserialize() {
                let record: Record = result?;
                records.push(record);
            }
        }
        Err(error) => panic!("Error opening file {:?}: {}", filepath, error),
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
