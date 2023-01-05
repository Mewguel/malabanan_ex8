// imports
extern crate regex ;
use regex::Regex ; // Regex
// use std :: error :: Error ; // Error class
use std::fs::File ; // File class
use std::io::prelude::*; // for file reading (io)
use std::path::Path ; // Path class
use std::io;

fn print_menu(){
    println!("\n\n============== MENU ================");
    println!("[1] Show all numbers");
    println!("[2] Show all keywords");
    println!("[3] Show all strings");
    println!("[4] Show all non-keyword identifiers");
    println!("====================================");
    println!("Choice: ");
}

fn main() {
    let out_path = Path::new("files/out.txt");
    let out_display = out_path.display();
    // Open file input.arnoldc
    let path = Path::new("files/input.arnoldc");
    let display = path.display();
    let mut file = match File::open(&path){

        Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    // Reading file contents
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.to_string()),
        Ok(_) => print!("{} has been loaded.\n\n", display),
    }

    print_menu();

    // Get user's choice of input
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Error");
    let choice:u32 = choice.trim().parse().unwrap();

    if choice == 1 {
        // vector for numbers
        let mut numb_vec:Vec<String> = Vec::new(); 
        // define regex for integers
        let re = Regex::new(r"(\b-?\d*\.?\d+\b)").unwrap();

        // append all matches to vector
        for cap in re.captures_iter(&s) {
            let mut new_num = String::new();
            new_num.push_str(&"Detected integer: ");
            new_num.push_str(&cap.at(1).unwrap_or("").to_string());
            new_num.push_str(&"\n".to_string());
            numb_vec.push(new_num);
        }

        // count matches
        let mut num_v_size = "Count: ".to_string();
        num_v_size.push_str(&numb_vec.len().to_string());
        num_v_size.push_str(&"\n".to_string());

        // file creation
        let mut file = match File::create(out_path) {
            Err(why) => panic!("couldn't create {}: {}\n", out_display, why.to_string()),
            Ok(file) => file,
        };

        match file.write_all(num_v_size.as_bytes()) {
            Err(why) => panic!("couldn't write {} to {}\n", why.to_string(), out_display),
            Ok(_) => print!("successfully wrote to {}\n", out_display),
        };

        for nums in numb_vec {
            match file.write_all(nums.as_bytes()) {
                Err(why) => panic!("couldn't write {} to {}\n", why.to_string(), out_display),
                Ok(_) => print!("successfully wrote to {}\n", out_display),
            };
        }
    } else if choice == 2 {
        //vector for keywords
        let mut key_vec:Vec<String> = Vec::new(); 
        //define regex for integers
        let keywords = vec![
            // main method
            Regex::new(r"\bIT'S SHOWTIME\b").unwrap(),
            Regex::new(r"\bYOU HAVE BEEN TERMINATED\b").unwrap(),
            // variable declaration
            Regex::new(r"\bHEY CHRISTMAS TREE\b").unwrap(),
            Regex::new(r"\bYOU SET US UP\b").unwrap(),
            // variable assignment
            Regex::new(r"\bGET TO THE CHOPPER\b").unwrap(),
            Regex::new(r"\bHERE IS MY INVITATION\b").unwrap(),
            Regex::new(r"\bENOUGH TALK\b").unwrap(),
            // arithmetic operations add, sub, mult, div
            Regex::new(r"\bGET UP\b").unwrap(),
            Regex::new(r"\bGET DOWN\b").unwrap(),
            Regex::new(r"\bYOU'RE FIRED\b").unwrap(),
            Regex::new(r"\bHE HAD TO SPLIT\b").unwrap(),
            // print method
            Regex::new(r"\bTALK TO THE HAND\b").unwrap(),
            // logical operators; ( equal to,  greater than)
            Regex::new(r"\bYOU ARE NOT YOU YOU ARE ME\b").unwrap(),
            Regex::new(r"\bLET OFF SOME STEAM BENNET\b").unwrap(),
            // logical operators; ( "or", "and")
            Regex::new(r"\bCONSIDER THAT A DIVORCE\b").unwrap(),
            Regex::new(r"\bKNOCK KNOCK\b").unwrap(),
            // Conditional statements; (if, else, if else)
            Regex::new(r"\bBECAUSE I'M GOING TO SAY PLEASE\b").unwrap(),
            Regex::new(r"\bBULLSHIT\b").unwrap(),
            Regex::new(r"\bYOU HAVE NO RESPECT FOR LOGIC\b").unwrap(),
            // While loop
            Regex::new(r"\bSTICK AROUND\b").unwrap(),
            Regex::new(r"\bCHILL\b").unwrap(),
            // void method
            Regex::new(r"\bLISTEN TO ME VERY CAREFULLY\b").unwrap(),
            Regex::new(r"\bHASTA LA VISTA, BABY\b").unwrap(),
            // non-void method
            Regex::new(r"\bLISTEN TO ME VERY CAREFULLY\b").unwrap(),
            Regex::new(r"\bI NEED YOUR CLOTHES YOUR BOOTS AND YOUR MOTORCYCLE\b").unwrap(),
            Regex::new(r"\bGIVE THESE PEOPLE AIR\b").unwrap(),
        ];
        
        // search for matches in the file using each regex in the vector
        for i in 0..keywords.len() {        
            for cap in keywords[i].captures_iter(&s){
                let mut capt_vec = String::new();
                // checks for keywords
                capt_vec.push_str("Detected keyword: ");
                capt_vec.push_str(&cap[0]);
                capt_vec.push_str("\n");
                key_vec.push(capt_vec);
            }
        }

        // count matches
        let mut kv_size = "Count: ".to_string();
        kv_size.push_str(&key_vec.len().to_string());
        kv_size.push_str(&"\n".to_string());

        // file creation
        let mut file = match File::create(out_path) {
            Err(why) => panic!("couldn't create {}: {}\n", out_display, why.to_string()),
            Ok(file) => file,
        };

        match file.write_all(kv_size.as_bytes()) {
            Err(why) => panic!("couldn't write {} to {}\n", why.to_string(), out_display),
            Ok(_) => print!("successfully wrote to {}\n", out_display),
        };

        for k in key_vec {
            match file.write_all(k.as_bytes()) {
                Err(why) => panic!("couldn't write {} to {}\n", why.to_string(), out_display),
                Ok(_) => print!("successfully wrote to {}\n", out_display),
            };
        }
        
    } else if choice == 3 {
        // vector for strings
        let mut str_vec:Vec<String> = Vec::new(); 
        // define regex for integers
        let str_re = Regex::new(r#""[A-Z]*\d*.*""#).unwrap();

        // append all matches to vector
        for cap in str_re.captures_iter(&s) {
            let mut new_str = String::new();
            let mut temp_str = String::new();
            new_str.push_str(&"Detected string literal: ");

            // clean the string; remove the " "
            temp_str.push_str(&cap[0].to_string());
            for i in 1..temp_str.len()-1 {
                new_str.push(temp_str.chars().nth(i).unwrap());
            }
            new_str.push_str(&"\n".to_string());
            str_vec.push(new_str);
        }
        
        // count matches
        let mut str_v_size = "Count: ".to_string();
        str_v_size.push_str(&str_vec.len().to_string());
        str_v_size.push_str(&"\n".to_string());

        // file creation
        let mut file = match File::create(out_path) {
            Err(why) => panic!("couldn't create {}: {}\n", out_display, why.to_string()),
            Ok(file) => file,
        };

        match file.write_all(str_v_size.as_bytes()) {
            Err(why) => panic!("couldn't write {} to {}\n", why.to_string(), out_display),
            Ok(_) => print!("successfully wrote to {}\n", out_display),
        };

        for s_str in str_vec {
            match file.write_all(s_str.as_bytes()) {
                Err(why) => panic!("couldn't write {} to {}\n", why.to_string(), out_display),
                Ok(_) => print!("successfully wrote to {}\n", out_display),
            };
        }
    }else if choice == 4 {
        // vector for non keyword identifiers
        let mut nk_vec:Vec<String> = Vec::new(); 
        // define regex for integers
        let nk_re = Regex::new(r"\b([a-z]+[a-z\d_]*)\b").unwrap();

        // append all matches to vector
        for cap in nk_re.captures_iter(&s) {
            let mut new_str = String::new();
            new_str.push_str(&"Detected identifier: ");
            new_str.push_str(&cap[0].to_string());
            new_str.push_str(&"\n".to_string());
            nk_vec.push(new_str);
        }

        // count matches
        let mut nk_v_size = "Count: ".to_string();
        nk_v_size.push_str(&nk_vec.len().to_string());
        nk_v_size.push_str(&"\n".to_string());

        // file creation
        let mut file = match File::create(out_path) {
            Err(why) => panic!("couldn't create {}: {}\n", out_display, why.to_string()),
            Ok(file) => file,
        };

        match file.write_all(nk_v_size.as_bytes()) {
            Err(why) => panic!("couldn't write {} to {}\n", why.to_string(), out_display),
            Ok(_) => print!("successfully wrote to {}\n", out_display),
        };

        for nk in nk_vec {
            match file.write_all(nk.as_bytes()) {
                Err(why) => panic!("couldn't write {} to {}\n", why.to_string(), out_display),
                Ok(_) => print!("successfully wrote to {}\n", out_display),
            };
        }

    }else {
        println!("Invalid Choice");
    }
}

// Ref.:
// https://github.com/lhartikk/ArnoldC/wiki/ArnoldC
// 