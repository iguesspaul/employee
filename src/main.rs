use std::{collections::HashMap, io};

fn main() {
    let mut company = HashMap::new();
    loop {
        println!("What do you want to do?");
        println!("add (name) to (department)");
        println!("list (department) || (company)");
        println!("quit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        
        let command: Vec<&str> = input.trim().split(" ").collect();
        
        if command[0] == "quit" {
            break;
        } else if command[0] == "add" {
            let worker = company.entry(String::from(command[3])).or_insert(vec![]);
            worker.push(String::from(command[1]));
        } else if command[0] == "list" {
            if command[1] == "company" {
                for (key, value) in &company {
                    println!("{}: ", key);
                    for j in value {
                        println!("{},", j);
                    }
                    println!("");
                }
            } else {
                for x in &company.get(command[1]) {
                    println!("{:?}", x);
                }            
            }
        }
    }
}
