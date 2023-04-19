use std::error::Error;
use std::string::FromUtf8Error;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::Response;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use bytes::Bytes;
use serde_json::from_str;

pub struct GPTClient{
    pub api_token:String,
    pub gpt_model:String,
    pub answer_hundler :  fn(&String)
}

impl GPTClient{

   pub async fn get_answer(&self,question:String) {


        let message =  GPTRequestMessage{
            content:&question,
            role: String::from("user")
        };
       let request_content =  GPTRequst{
            model  : &self.gpt_model,
            stream : true,
            messages : [
                message
            ]
       };

        let request_content =serde_json::to_string(&request_content).unwrap();
        let client = reqwest::Client::new();
        let response  = client.post("https://api.openai.com/v1/chat/completions")
            .header(AUTHORIZATION,format!("Bearer {}",&self.api_token))
            .header(CONTENT_TYPE, "application/json")
        .body(request_content)
        .send().await;
        if response.is_err(){
           println!("error {}",response.err().unwrap().source().unwrap());
           return;
        }
        let response:Response = response.unwrap();
        let mut response = response.bytes_stream();
        while let Some(partial_answer) = response.next().await
        {
            let partial_answer:Bytes = partial_answer.unwrap();
            let partial_answer:Result<String,FromUtf8Error> = String::from_utf8(partial_answer.to_vec());
            if  partial_answer.is_err()
            {
                continue;
            }
            let mut partial_answer = partial_answer.unwrap();

            let first_brackets_index= partial_answer.find(|x| x=='{'); //the response of GPTApi Contain unnecessary word before the json so we take the indix to skip it
            if first_brackets_index.is_some() {
                partial_answer.drain(0..first_brackets_index.unwrap());
            }
            let partial_answer: GPTResponse = match  from_str(&partial_answer) {
                Ok(data)=>data,
                Err(_)=>continue
            };

            let partial_answer = partial_answer.choices.first();
            if partial_answer.is_none() {
                continue;
            }
            let partial_answer= partial_answer.unwrap();
            (&self.answer_hundler)( &partial_answer.delta.content);

       }

    

    }
}

// Request-------------------------------
#[derive(Serialize)]
struct  GPTRequst<'a>{
    pub model:& 'a str,
    pub messages:[GPTRequestMessage<'a>;1],
    pub stream:bool

}
#[derive(Serialize)]
struct GPTRequestMessage<'a>{
    pub role:String,
    pub content: & 'a str
}
//Response ----------------------------
#[derive( Deserialize)]
struct  GPTResponse{
    pub choices:[Choice;1]

}
#[derive( Deserialize)]
struct  Choice {
    pub delta: Delta
}
#[derive( Deserialize)]
struct Delta{
    pub content:String
}