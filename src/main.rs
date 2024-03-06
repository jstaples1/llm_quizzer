//use std::env;
use reqwest::Client;
use serde_json::json;
use serde_json::Value;

//use std::error::Error;
use tokio;

#[tokio::main]
async fn main() {
    call_open_ai().await;
    //call_dictionaryapi_with_get().await;
}

async fn call_open_ai() {
    println!("Making openai call");

    //let api_key = env::var("OPENAI_API_KEY").expect("No API key");
    let _api_key = "<YOUR_GithubPersonalAccessToken_HERE>";

    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/completions")
        //.header("Authorization", api_key)
         .json(&json!({
           "prompt": "Hello",
           "max_tokens": 10
        }))
        .send()
        .await
        .expect("Request failed");

    print!("{}",response.text().await.unwrap().as_str());  
    // Get the JSON content as a serde_json Value
    //let json = response.expect::<serde_json::Value>();

    
}



async fn _call_dictionaryapi_with_get(){

  let url = "https://api.dictionaryapi.dev/api/v2/entries/en/train";
   
    let response = reqwest::get(url).await;

    match response {  
        Ok(response) => {
          let json: Value = serde_json::from_str(response.text().await.unwrap().as_str()).unwrap();
          let array = json.as_array().unwrap(); 
          for element in array {
            println!("Element is: {}", element);
          }            
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

#[test]
fn test_function_name() {
    // test code here
    print!("test ran   ----  ");
}
