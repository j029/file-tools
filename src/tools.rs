use std::fs::File;
use std::{fs};
use std::path::Path;
use serde_json::{json, Value};
use rand::distributions::{Distribution, Uniform};


pub fn create_dir() -> std::io::Result<()> {
  fs::create_dir("./output_files")?;
  Ok(())
}

pub fn read_dir() -> std::io::Result<()> {
    println!("\nCurrent files:");
    for entry in fs::read_dir("./output_files")? {
        let dir = entry?;
        println!("{:?}", dir.path());
    }

    println!("\n");

    Ok(())
}
pub fn clear_output() -> std::io::Result<()> {
    fs::remove_dir_all("./output_files")?;
    Ok(())
}

#[allow(unused)]
pub fn json_builder(file_number: u16, pass_type: String) -> std::io::Result<()> {

    let png: String = ".png".to_string();
    let gif = ".gif".to_string();
    let mp4 = ".mp4".to_string();
    let file_number_str = &file_number.to_string();
    let image = file_number_str.to_owned() + &gif;
    let image_png = file_number_str.to_owned() + &png;
    let video = file_number_str.to_owned() + &mp4;

    let mut input_content = json!({});

      if pass_type == "basic" {
        input_content = json!({
          "name": "MoonPass Basic",
            "symbol": "MGMP",
            "description": "A basic pass to the MoonGhost ecosystem. Allowing free mints, whitelists and more.",
            "image": image,
            "animation_url": video,
            "external_url": "https://moonghostofficial.com",
            "attributes": [
              {
                "trait_type": "Pass-type",
                "value": "Basic"
              },
              {
                "trait_type": "GuardianGhost-whitelist",
                "value": "True"
              },
              {
                "trait_type": "GuardianGhost-free-mint",
                "value": "False"
              },
              {
                "trait_type": "GenesisGhost-whitelist",
                "value": "True"
              },
              {
                "trait_type": "GenesisGhost-free-mint",
                "value": "False"
              },
              {
                "trait_type": "MoonGhost-accounts-whitelist",
                "value": "False"
              },
              {
                "trait_type": "MoonGhost-accounts-free-mint",
                "value": "False"
              }
            ],
            "properties": {
              "files": [
                {
                  "uri": image,
                  "type": "image/gif"
                },
                {
                  "uri": video,
                  "type": "video/mp4"
                }
              ],
              "category": "video"
            }
          })
        }
      else if pass_type == "mythical" {
        input_content = json!({
          "name": "MoonPass Mythical",
            "symbol": "MGMP",
            "description": "A mythical pass to the MoonGhost ecosystem. Allowing free mints, whitelists and more.",
            "image": image,
            "animation_url": video,
            "external_url": "https://moonghostofficial.com",
            "attributes": [
              {
                "trait_type": "Pass-type",
                "value": "Mythical"
              },
              {
                "trait_type": "GuardianGhost-whitelist",
                "value": "True"
              },
              {
                "trait_type": "GuardianGhost-free-mint",
                "value": "True"
              },
              {
                "trait_type": "GenesisGhost-whitelist",
                "value": "True"
              },
              {
                "trait_type": "GenesisGhost-free-mint",
                "value": "False"
              },
              {
                "trait_type": "MoonGhost-accounts-whitelist",
                "value": "False"
              },
              {
                "trait_type": "MoonGhost-accounts-free-mint",
                "value": "False"
              }
            ],
            "properties": {
              "files": [
                {
                  "uri": image,
                  "type": "image/gif"
                },
                {
                  "uri": video,
                  "type": "video/mp4"
                }
              ],
              "category": "video"
            }
          });
        }
        else if pass_type == "ultimate" {
          input_content = json!({
            "name": "MoonPass Ultimate",
              "symbol": "MGMP",
              "description": "An ultimate pass to the MoonGhost ecosystem. Allowing free mints, whitelists and more.",
              "image": image,
              "animation_url": video,
              "external_url": "https://moonghostofficial.com",
              "attributes": [
                {
                  "trait_type": "Pass-type",
                  "value": "Ultimate"
                },
                {
                  "trait_type": "GuardianGhost-whitelist",
                  "value": "True"
                },
                {
                  "trait_type": "GuardianGhost-free-mint",
                  "value": "True"
                },
                {
                  "trait_type": "GenesisGhost-whitelist",
                  "value": "True"
                },
                {
                  "trait_type": "GenesisGhost-free-mint",
                  "value": "True"
                },
                {
                  "trait_type": "MoonGhost-accounts-whitelist",
                  "value": "True"
                },
                {
                  "trait_type": "MoonGhost-accounts-free-mint",
                  "value": "True"
                }
              ],
              "properties": {
                "files": [
                  {
                    "uri": image,
                    "type": "image/gif"
                  },
                  {
                    "uri": video,
                    "type": "video/mp4"
                  }
                ],
                "category": "video"
              }
            });
          }
        else if pass_type == "og" {
          input_content = json!({
            "name": "MoonPass OG",
              "symbol": "MGMP",
              "description": "An OG pass to the MoonGhost ecosystem. Allowing free mints, whitelists and more.",
              "image": image,
              "animation_url": video,
              "external_url": "https://moonghostofficial.com",
              "attributes": [
                {
                  "trait_type": "Pass-type",
                  "value": "OG"
                },
                {
                  "trait_type": "GuardianGhost-whitelist",
                  "value": "True"
                },
                {
                  "trait_type": "GuardianGhost-free-mint",
                  "value": "True"
                },
                {
                  "trait_type": "GenesisGhost-whitelist",
                  "value": "True"
                },
                {
                  "trait_type": "GenesisGhost-free-mint",
                  "value": "True"
                },
                {
                  "trait_type": "MoonGhost-accounts-whitelist",
                  "value": "True"
                },
                {
                  "trait_type": "MoonGhost-accounts-free-mint",
                  "value": "True"
                }
              ],
              "properties": {
                "files": [
                  {
                    "uri": image,
                    "type": "image/gif"
                  },
                  {
                    "uri": video,
                    "type": "video/mp4"
                  }
                ],
                "category": "video"
              }
            });
        }
        else if pass_type == "hidden" {
          input_content = json!({
            "name": "Hidden MoonPass",
              "symbol": "MGMP",
              "description": "Your pass to the MoonGhost ecosystem. Allowing free mints, whitelists and more. Currently hidden.",
              "image": image_png,
              "external_url": "https://moonghostofficial.com",
              "attributes": [
                {
                  "trait_type": "Pass-type",
                  "value": "Hidden"
                }
              ],
              "properties": {
                "files": [
                  {
                    "uri": image_png,
                    "type": "image/png"
                  }
                ]
              }
            });
        }
        else {
          panic!("Incorrect pass type!")
        };

    let file_path_string = format!("./output_files/{}.json", file_number.to_string());
    let file_path = Path::new(&file_path_string);
    File::create(file_path)?;
    println!("Created {}.json", file_number);
    fs::write(file_path, serde_json::to_string_pretty(&input_content).unwrap())?;
    
    Ok(())
}

pub fn png_builder(file_number: u16, pass_type: String) -> std::io::Result<()> {

  let file_path_string = format!("./output_files/{}.png", file_number.to_string());
  let file_path = Path::new(&file_path_string);

  File::create(file_path)?;

  let mut from_file_path = Path::new("");
  if pass_type == "basic" {
    from_file_path = Path::new("./input_files/Ba.png");
  }
  else if pass_type == "mythical" {
    from_file_path = Path::new("./input_files/My.png");
  }
  else if pass_type == "ultimate" {
    from_file_path = Path::new("./input_files/Ul.png");
  }
  else if pass_type == "og" {
    from_file_path = Path::new("./input_files/OG.png");
  }
  else if pass_type == "hidden" {
    from_file_path = Path::new("./input_files/Hidden.png");
  }

  fs::copy(from_file_path, file_path_string)?;
  
  Ok(())
}

pub fn mp4_builder(file_number: u16, pass_type: String) -> std::io::Result<()> {

  let file_path_string = format!("./output_files/{}.mp4", file_number.to_string());
  let file_path = Path::new(&file_path_string);

  File::create(file_path)?;

  let mut from_file_path = Path::new("");
  if pass_type == "basic" {
    from_file_path = Path::new("./input_files/Ba.mp4");
  }
  else if pass_type == "mythical" {
    from_file_path = Path::new("./input_files/My.mp4");
  }
  else if pass_type == "ultimate" {
    from_file_path = Path::new("./input_files/Ul.mp4");
  }
  else if pass_type == "og" {
    from_file_path = Path::new("./input_files/OG.mp4");
  }

  fs::copy(from_file_path, file_path_string)?;
  
  Ok(())
}

pub fn gif_builder(file_number: u16, pass_type: String) -> std::io::Result<()> {

  let file_path_string = format!("./output_files/{}.gif", file_number.to_string());
  let file_path = Path::new(&file_path_string);

  File::create(file_path)?;

  let mut from_file_path = Path::new("");
  if pass_type == "basic" {
    from_file_path = Path::new("./input_files/Ba.gif");
  }
  else if pass_type == "mythical" {
    from_file_path = Path::new("./input_files/My.gif");
  }
  else if pass_type == "ultimate" {
    from_file_path = Path::new("./input_files/Ul.gif");
  }
  else if pass_type == "og" {
    from_file_path = Path::new("./input_files/OG.gif");
  }

  fs::copy(from_file_path, file_path_string)?;
  
  Ok(())
}

pub fn generate_jgm(i: u16, pass_type: String) {
  json_builder(i, pass_type.to_string()).map_err(|err| println!("{:?}", err)).ok();
  gif_builder(i, pass_type.to_string()).map_err(|err| println!("{:?}", err)).ok();
  mp4_builder(i, pass_type.to_string()).map_err(|err| println!("{:?}", err)).ok();
}

#[allow(unused)]
pub fn generate_jpm(i: u16, pass_type: String) {
  json_builder(i, pass_type.to_string()).map_err(|err| println!("{:?}", err)).ok();
  png_builder(i, pass_type.to_string()).map_err(|err| println!("{:?}", err)).ok();
  mp4_builder(i, pass_type.to_string()).map_err(|err| println!("{:?}", err)).ok();
}

pub fn generate_random_jgm(i: u16) -> String {

  let mut random_number_generator = rand::thread_rng();
  let range = Uniform::from(1..101);
  let random_number: u16 = range.sample(&mut random_number_generator);

  if random_number > 0 && random_number <= 70 {
    generate_jgm(i, "basic".to_string());
    return "basic".to_string()
  }
  else if random_number > 70 && random_number <= 90 {
    generate_jgm(i, "mythical".to_string());
    return "mythical".to_string()
  }
  else if random_number > 90 && random_number <= 100 {
    generate_jgm(i, "ultimate".to_string());
    return "ultimate".to_string()
  }
  else {
    generate_jgm(i, "og".to_string());
    return "og".to_string()
  }
  
}

pub fn generate_collection_json(from: u64, to: u64) {
  let config_string = fs::read_to_string("./input_files/file_tools_config.json")
  .expect("Should have been able to read the file");

  let config: Value = serde_json::from_str(&config_string).unwrap();
      
  let new_uri_json = &config["new_uri"];
  let new_uri:String = serde_json::from_value(new_uri_json.to_owned()).unwrap();
  
  File::create("./output_files/collection_data.json")
  .expect("Should have been able to create the collection_data file");
  
  let mut output_vec: Vec<Value> = Vec::new();

  for i in from..to {
    
    let file_number_str = i.to_string();
    
    let output_item =  json!( format!("{}/{}.json", new_uri, file_number_str) );
    output_vec.push(output_item);
    
  }


  fs::write("./output_files/collection_data.json", serde_json::to_string_pretty(&output_vec).unwrap()).map_err(|err| println!("{:?}", err)).ok();

}

pub fn metaboss_generate() {

  clear_output().expect("Should have been able to clear the output directory");
  create_dir().expect("Should have been able to create the output directory");

  let config_string = fs::read_to_string("./input_files/file_tools_config.json")
  .expect("Should have been able to read the file");

  let config: Value = serde_json::from_str(&config_string).unwrap();
      
  let new_uri_json = &config["new_uri"];
  let new_uri:String = serde_json::from_value(new_uri_json.to_owned()).unwrap();

  File::create("./output_files/metaboss_data.json")
  .expect("Should have been able to create the metaboss_data file");
 

  let input_content_string = fs::read_to_string("./input_files/metaboss_snapshot.json")
  .expect("Should have been able to read the file");

  let input_content: Vec<Value> = serde_json::from_str(&input_content_string).unwrap();

  let mut output_vec: Vec<Value> = Vec::new();
 
  for i in 0..input_content.len() {
    let file_number_str = i.to_string();
    let output_item =  json!({ "mint_account": input_content[i], "new_uri": format!("{}/{}.json", new_uri, file_number_str) });
    output_vec.push(output_item);
  }


  fs::write("./output_files/metaboss_data.json", serde_json::to_string_pretty(&output_vec).unwrap()).map_err(|err| println!("{:?}", err)).ok();
  

}