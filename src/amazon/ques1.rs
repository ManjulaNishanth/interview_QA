use serde::{Deserialize, Serialize};
use serde_json::{json, Error};

#[derive(Debug, Deserialize, Serialize)]
struct Deplyment {
    deployment_id: String,
    status: String,
}

fn evaluate_deployments(deployments: &[String]) -> Vec<i32> {
    let mut success: i32 = 0;
    let mut fail: i32 = 0;
    let mut error: i32 = 0;
    for i in deployments {
        let deserialized_result: Result<Deplyment, Error> = serde_json::from_str(i);
        match deserialized_result {
            Ok(deserialized) => {
                let mut x = deserialized.deployment_id.split("-");
                let first = x.next().unwrap();
                let second = x.next().unwrap();

                if first != "d" {
                    error = error + 1;
                    println!("invalid deployment_id prefix");
                } else if second.len() != 10 {
                    error = error + 1;
                    println!("deployment_id suffix length is not 10");
                } else if second.chars().all(char::is_alphanumeric) == false {
                    error = error + 1;
                    println!("deployment_id suffix has special characters");
                } else if second.chars().any(char::is_uppercase) == true {
                    error = error + 1;
                    println!("deployment_id suffix has uppercase");
                } else if deserialized.status.to_string() == "Success".to_string() {
                    success = success + 1;
                } else if deserialized.status.to_string() == "Fail".to_string() {
                    fail = fail + 1;
                } else {
                    error = error + 1;
                }
            }
            Err(serde_error) => {
                error = error + 1;
                println!("serde_error {serde_error:?}");
            }
        };
        println!("success : {success:?}, fail : {fail:?},error : {error:?}");
    }

    [success, fail, error].to_vec()
}

mod tests {
    use std::{
        env,
        fs::File,
        io::{self, BufRead},
    };

    use super::*;
    use serde_json::json;

    #[test]
    pub fn evaluate_deployments_cfg() {
        let input = [
            json!({"deployment_id": "d-12345678ab", "status": "Success"}).to_string(),
            json!({"deployment_id": "d-09876543cd", "status": "Fail"}).to_string(),
            json!({"deployment_id": "c-09876543cd", "status": "Fail"}).to_string(),
            json!({"deployment_id": "d-09876543cd1", "status": "Fail"}).to_string(),
            json!({"deployment_id": "d-09876543c#", "status": "Fail"}).to_string(),
            json!({"deployment_id": "d-09876543Cd", "status": "Fail"}).to_string(),
            json!({"status": "Fail"}).to_string(),
        ];

        let result = evaluate_deployments(&input);
        println!("\n RESULT : {result:?}");

        assert_eq!(result, [1, 1, 5].to_vec())
        // let stdin = io::stdin();
        // let mut stdin_iterator = stdin.lock().lines();

        // let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

        // let deployments_count = stdin_iterator
        //     .next()
        //     .unwrap()
        //     .unwrap()
        //     .trim()
        //     .parse::<i32>()
        //     .unwrap();

        // let mut deployments: Vec<String> = Vec::with_capacity(deployments_count as usize);

        // for _ in 0..deployments_count {
        //     let deployments_item = stdin_iterator.next().unwrap().unwrap();
        //     deployments.push(deployments_item);
        // }

        // let result = evaluate_deployments(&deployments);

        // for i in 0..result.len() {
        //     write!(&mut fptr, "{}", result[i]).ok();

        //     if i != result.len() - 1 {
        //         writeln!(&mut fptr).ok();
        //     }
        // }

        // writeln!(&mut fptr).ok();
    }
}
