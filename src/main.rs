mod amazon;

fn main() {
    println!("Hello, world!");
}

// use std::collections::HashMap;

// fn parse_configuration(config_str: &str) -> Result<Vec<String>, String> {
//     let pairs: Vec<&str> = config_str.split('|').collect();

//     if pairs.len() < 2 {
//         return Err("Invalid configuration string: too few pairs".to_string());
//     }

//     let mut config_map: HashMap<u32, String> = HashMap::new();
//     let mut count = 1;
//     for pair in pairs {
//         let index_str = &pair[0..4];
//         let config = &pair[4..];

//         if config.len() != 10 {
//             return Err(format!("Invalid configuration length"));
//         }

//         let index_result: Result<u32, std::num::ParseIntError> = index_str.parse::<u32>(); //.unwrap();

//         // println!("\n index_number : {index_number:?}, compare : {:?}", index_number == count);

//         match index_result {
//             Ok(index) => {
//                 if index != count {
//                     return Err(format!("Invalid order"));
//                 }
//             }
//             Err(_) => return Err(format!("Invalid index: {}", index_str)),
//         };

//         if config.chars().all(char::is_alphanumeric) == false {
//             return Err(format!("Config should be alphanumeric"));
//         }

//         // let index = match u32::from_str_radix(index_str, 16) {
//         //     Ok(i) => {
//         //         println!("\n here : {:?}",i);
//         //         i
//         //     },
//         //     Err(_) => return Err(format!("Invalid index: {}", index_str)),
//         // };

//         if config_map.contains_key(&count) {
//             return Err(format!("Duplicate index: {}", count));
//         }

//         config_map.insert(count, config.to_string());
//         count += 1;
//     }

//     let mut ordered_configs: Vec<String> = Vec::new();
//     for i in 1..=config_map.len() as u32 {
//         if let Some(config) = config_map.remove(&i) {
//             ordered_configs.push(config);
//         } else {
//             return Err(format!("Missing configuration for index: {}", i));
//         }
//     }

//     Ok(ordered_configs)
// }

// /*
// A barcode scanner can be configured by scanning a series of barcodes in the correct order.
// Barcode configurations are encoded into a single string and stored as a blob in the backend system.
// The client requests the configuration from the backend configuration service, and then needs to present the configurations in the correct order.
// The encoded configuration string is a series of pairs separated by |.
// The ordinal index value is a 4 digit numeric prefixed with zeros.

// For example,
//     the first configuration will be represented as 0001.

// The goals are to
// 1) validate the configuratifs string
// 2) provide the configuration client the configuration values in the order required to successfully configure the barcode scanner.
//     Validation conditions All configurations must be separated by a "|" character.
//     Configurations cannot skip a number in the ordering.
//     If there are three configuration strings, there must be a 1, 2, and 3 index.
//     Configuration values are alphanumeric and may contain no other characters.
//     Configuration value length is exactly 10 characters.
//     Ordinal indices may not repeat, for example there cannot be two occurrences of
// */
// fn main() {
//     /* CORRECT OUTPUT */
//     // let config_str = "0001f7c22e7904|000276a3a4d214|000305d29f4a4b";
//     // let result = parse_configuration(config_str);

//     /* ERROR - Configurations cannot skip a number in the ordering */
//     // let config_str = "0002f7c22e7904|000176a3a4d214|000305d29f4a4b";
//     // let result = parse_configuration(config_str);

//     /* ERROR Invalid value in ordering */
//     // let config_str = "000af7c22e7904|000276a3a4d214|000305d29f4a4b";
//     // let result = parse_configuration(config_str);

//     /* ERROR - Configuration values are alphanumeric and may contain no other characters*/
//     // let config_str = "0001f7c22e790#|000276a3a4d214|000305d29f4a4b";
//     // let result = parse_configuration(config_str);

//     /* ERROR Configuration value length is exactly 10 characters */
//     let config_str = "0001f7c22e79002|000276a3a4d214|000305d29f4a4b";
//     let result = parse_configuration(config_str);

//     match result {
//         Ok(configs) => println!("{:?}", configs),
//         Err(err) => println!("Error: {}", err),
//     }
// }
