use std::fs;
use std::io::{self, Write};

#[derive(Debug)]
struct Contact {
    name: String,
    phone: String,
}

impl Contact {
    fn from_string(line: &str) -> Option<Contact> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            Some(Contact {
                name: parts[0].trim().to_string(),
                phone: parts[1].trim().to_string(),
            })
        } else {
            None
        }
    }

    fn to_string(&self) -> String {
        format!("{},{}", self.name, self.phone)
    }
}

fn load_contacts(filename: &str) -> Vec<Contact> {
    let mut contacts = Vec::new();

    if let Ok(contents) = fs::read_to_string(filename) {
        for line in contents.lines() {
            if let Some(contact) = Contact::from_string(line) {
                contacts.push(contact);
            }
        }
    }

    contacts
}

fn save_contacts(filename: &str, contacts: &[Contact]) -> io::Result<()> {
    let mut file = fs::File::create(filename)?;
    for contact in contacts {
        writeln!(file, "{}", contact.to_string())?;
    }
    Ok(())
}

fn main() {
    let filename = "contacts.txt";
    let mut contacts = load_contacts(filename);

    loop {
        println!("\nContact Manager");
        println!("1. List contacts");
        println!("2. Add a contact");
        println!("3. Save and exit");

        let choice = get_input("Enter your choice:");

        match choice.as_str() {
            "1" => list_contacts(&contacts),
            "2" => add_contact(&mut contacts),
            "3" => {
                if save_contacts(filename, &contacts).is_ok() {
                    println!("Contacts saved successfully.");
                } else {
                    println!("Failed to save contacts.");
                }
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn list_contacts(contacts: &[Contact]) {
    println!("\nContacts:");
    for (i, contact) in contacts.iter().enumerate() {
        println!("{}. {} - {}", i + 1, contact.name, contact.phone);
    }
}

fn add_contact(contacts: &mut Vec<Contact>) {
    let name = get_input("Enter the name:");
    let phone = get_input("Enter the phone number:");
    contacts.push(Contact { name, phone });
    println!("Contact added successfully.");
}
