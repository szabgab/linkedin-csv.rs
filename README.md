#  LinkedIn CSV

This crate defines structs that can be used to parse the CSV files exported from LinkedIn.

See the `main.rs` file on how to use the functions.


To export the data from Google:

```
Me => Settings & Privacy => Data Privacy => Get a copy of your data =>  => Request Archive)

https://www.linkedin.com/mypreferences/d/download-my-data

Select all

unzip ~/Downloads/Complete_LinkedInDataExport_04-04-2023.zip
```

Warning: right now each field is `pub` which might change.

This project is not endorsed by or associated with Microsoft or LinkedIn.
It is an independent project trying to deal with data from LinkedIn.