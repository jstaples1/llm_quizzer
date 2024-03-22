//use std::env;
use reqwest::Client;
use serde_json::json;
use serde_json::Value;

//use std::error::Error;
use tokio;

const OPENAI_RESPONSE_TOKENS:i32 = 10;

#[tokio::main]
async fn main() {
    call_open_ai().await;
    //call_dictionaryapi_with_get().await;
}

async fn call_open_ai() {
    println!("Making Open AI call");

    //let api_key = env::var("OPENAI_API_KEY").expect("No API key");
    let api_key = "PROVIDE YOUR OWN OPENAI API KEY HERE";

    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        //.header(AUTHORIZATION, "Bearer" + api_key)
         .json(&json!({
           //"prompt": "what is the tallest building in the",
           "model": "gpt-3.5-turbo", 
           "messages": [
              //{"role": "system", "content": "You are a rust prog, skilled in explaining complex programming concepts with creative flair."},
              {"role": "user", "content": "what is the tallest building in the world?"}
           ],
           "max_tokens": OPENAI_RESPONSE_TOKENS
        }))
        .send()
        .await
        .expect("Request failed");

        let text = response.text().await.expect("response has no text");
        

        let json: Value = serde_json::from_str(&text).expect("json dserialization of Open AI response failed");

        let choices = json.get("choices").unwrap();
        let choice = choices.get(0).unwrap();
        let message = choice.get("message").unwrap();
        let content = message.get("content").unwrap();
        println!("{}", content);
        println!("-----------------");
        println!("{}", text);
        
        

    
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
