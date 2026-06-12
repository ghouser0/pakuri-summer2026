use std::io::{Write, stdout, stdin};
use std::collections::HashMap;
mod pakuri;

fn display_menu()
{   
    println!("
Welcome to Pakudex: Let's Go!

Pakudex Main Menu 
----------------- 
1. List Pakuri 
2. Show Pakuri 
3. Add Pakuri 
4. Remove Pakuri 
5. Change Pakuri Level 
6. Exit 
");
}

fn list_pakuri() -> bool
{   
    // number and list pakuri
    println!("Pakuri in Pakudex");
    if paku_vec.len() == 0 {
        return false;
    }
    
    let mut i = 1;
    for paku in &paku_vec {
        println!("{}. {} ({}, level {})", i, paku.name, paku.species, paku.level);
        i += 1;
    }
    return true;
}

fn show_pakuri(String: name) -> bool
{   
    // returns false if pakuri doesn't exist
}

fn add_pakuri(String: name) -> bool
{   
    // returns false if add successful
    
    let mut pakuri_name = String::new();
    let mut pakuri_species = String::new();
    let mut pakuri_level = String::new();
    let mut pakuri_level_casted: i32 = 0;

    print!("\nName: "); 
    stdout().flush().unwrap();
    pakuri_name.clear();
    stdin().read_line(&mut pakuri_name).expect("Read line failed");

    for paku in &paku_vec {
        if paku.name == pakuri_name {
            println!("Error: Pakudex already contains this Pakuri!");
            return false;
        }
    }

    println!("Species: "); 
    stdout().flush().unwrap();
    pakuri_species.clear();
    stdin().read_line(&mut pakuri_species).expect("Read line failed");

    loop {
        println!("Level: ");
        stdout().flush().unwrap();
        pakuri_level.clear();
        stdin().read_line(&mut pakuri_level).expect("Read line failed");
        let pakuri_level_casted: i32 = pakuri_level.trim().parse().expect("Invalid level!");

        if pakuri_level_casted >= 0 {
            break;
        }
        println!("Level cannot be negative.");
    }

    return true;
}

fn kill_pakuri(String: name) -> bool
{   
    // returns false if remove successful
}

fn change_pakuri_level(String: name, Int32: level) -> bool
{   
    // returns false if add successful
    // enter name
    // enter level

}

fn main() 
{
    let mut menu_choice = String::new(); 
    let mut menu_choice_casted: i32 = 0;  
    let mut pakuri_name = String::new(); 
    
    // let mut previous_result = 0.0;
    // let mut sum_calculations = 0.0;
    // let mut total_calculations: i32 = 0;
    
    let mut print_da_menu: bool = true;

    let mut paku_vec: Vec::new();

    loop {

        print!("\nWhat would you like to do? \n"); 
        stdout().flush().unwrap();
        menu_choice.clear();
        stdin().read_line(&mut menu_choice).expect("Read line failed");
        let menu_choice_casted: i32 = menu_choice.trim().parse().expect("Not a number");

        if menu_choice_casted == 6 {
            break;
        }
        else if menu_choice_casted == 1 { // LIST
            print_da_menu = true;
            if list_pakuri() == false {
                println!("No Pakuri in Pakudex yet!\n");
            }
        } 
        else if menu_choice_casted == 2 { // SHOW
            print_da_menu = true;
            
            print!("\nEnter the name of the Pakuri to display: "); 
            stdout().flush().unwrap();
            pakuri_name.clear();
            stdin().read_line(&mut pakuri_name).expect("Read line failed");
            
            if show_pakuri(pakuri_name) == false {
                println!("Error: No such Pakuri!");
            }
        } 
        else if menu_choice_casted == 3 { // ADD
            print_da_menu = true;

            if add_pakuri() == true {
                println!("Pakuri Added!"); // FIX THIS IDK IF IT"S RIGHT
            }
        } 
        else if menu_choice_casted == 4 { // REMOVE
            print_da_menu = true;

        } 
        else if menu_choice_casted == 5 { // CHANGE LEVEL
            print_da_menu = true;

        } 
    
    }

    println!("\nThanks for using Pakudex. Goodbye!");
}