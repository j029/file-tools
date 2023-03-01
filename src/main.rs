mod tools;

use std::{env, convert::TryInto};
fn main() {

    let args: Vec<String> = env::args().collect();
    let command_arg = &args[1];
    


    println!("Running main.rs\n");

    if command_arg == "build" {

    tools::clear_output().map_err(|err| println!("{:?}", err)).ok();
    
    let pass_type = &args[2];
    let from: u64 = args[3].parse::<u64>().unwrap();
    let to: u64 = args[4].parse::<u64>().unwrap();
    
        
    tools::create_dir().map_err(|err| println!("{:?}", err)).ok();
    tools::read_dir().map_err(|err| println!("{:?}", err)).ok();

    if pass_type == "random" {

        let ogs_to_generate: u64 = args[5].parse::<u64>().unwrap();

        let mut basic_amount: f64 = 0.00;
        let mut mythical_amount: f64 = 0.00;
        let mut ultimate_amount: f64 = 0.00;
        let mut og_amount: f64 = 0.00;

        for i in from..(from + ogs_to_generate) {
            
            tools::generate_jgm(i.try_into().unwrap(), "og".to_string());
            og_amount += 1.00;

        }

        for i in (from + ogs_to_generate)..to {
            
            let generated_type = tools::generate_random_jgm(i.try_into().unwrap());

            if generated_type == "basic" {
                basic_amount += 1.00;
            }
            else if generated_type == "mythical" {
                mythical_amount += 1.00;
            }
            else if generated_type == "ultimate" {
                ultimate_amount += 1.00;
            }

        }

        tools::generate_collection_json(from, to);

        let total: f64 = &basic_amount + &mythical_amount + &ultimate_amount + &og_amount;

        println!("Basic amount: {}", &basic_amount);
        println!("Mythical amount: {}", &mythical_amount);
        println!("Ultimate amount: {}", &ultimate_amount);
        println!("Og amount: {}", &og_amount);
        
        println!("Basic percentage: {}",( &basic_amount / &total * 100.00) as f64);
        println!("Mythical percentage: {}", (&mythical_amount / &total * 100.00) as f64);
        println!("Ultimate percentage: {}", (&ultimate_amount / &total * 100.00) as f64);
        println!("Og percentage: {}", (&og_amount / &total * 100.00) as f64);


    }
    else if pass_type == "hidden" {
        for i in from..to {
            
            tools::json_builder(i.try_into().unwrap(), pass_type.to_string()).map_err(|err| println!("{:?}", err)).ok();
            tools::png_builder(i.try_into().unwrap(), pass_type.to_string()).map_err(|err| println!("{:?}", err)).ok();

        }
    }

    else{

        for i in from..to {
            
            tools::generate_jgm(i.try_into().unwrap(), pass_type.to_string());

        }

    }

    }
    else if command_arg == "clear" {

        tools::clear_output().map_err(|err| println!("{:?}", err)).ok();

    }
    else if command_arg == "build_metaboss" {

        tools::metaboss_generate();


    }
    else if command_arg == "build_collection_data" {

        let from: u64 = args[2].parse::<u64>().unwrap();
        let to: u64 = args[3].parse::<u64>().unwrap();

        tools::generate_collection_json(from, to);

    }
    else {
        panic!("Incorrect command!")
    }
}
