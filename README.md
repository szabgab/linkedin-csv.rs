#  LinkedIn CSV

This crate defines structs that can be used to parse the CSV files exported from LinkedIn.

## To export the data from LinkedIn:

```
Me => Settings & Privacy => Data Privacy => Get a copy of your data =>  => Request Archive)
```

That should lead you to [this page](https://www.linkedin.com/mypreferences/d/download-my-data)


Select all

```
unzip ~/Downloads/Complete_LinkedInDataExport_04-04-2023.zip
```

## Usage

Currently a command line tool is included to read a file and print to the screen.

```
cargo clone https://github.com/szabgab/linkedin-csv.rs.git
cd linkedin-csv
cargo run
```

See the `main.rs` file on how to use the functions.


## Warning

Right now each field is `pub` which might change. Many other things will change.

## Clarification

This project is not endorsed by or associated with Microsoft or LinkedIn.
It is an independent project trying to deal with data from LinkedIn.
