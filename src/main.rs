mod gpt;

use std::{env, io};
use std::io::Write;

#[tokio::main]
async fn main() {
    let question: Option<String> = env::args().nth(1);
    if question.is_none(){
        println!("please provide a question");
        return;
    };
    let question:String = question.unwrap();


    let gpt_api_token=  env::var("GPTApiToken");
    if gpt_api_token.is_err() {
        println!("please set  GPTApiToken environment variable to have your GPT access key");
        return;
    }
    let gpt_api_token=gpt_api_token.unwrap();


    let gpt_api_model: Result<String, env::VarError>=  env::var("GPTModel");
    if gpt_api_model.is_err() {
        println!("please set  GPTModel environment variable to set the model you want to use ");
        return;
    }
    let gpt_api_model= gpt_api_model.unwrap();

    
    
    let client =gpt::GPTClient{
        api_token:gpt_api_token,
        gpt_model:gpt_api_model,
        answer_hundler: write_to_console
    };
    client.get_answer(question).await;
}
fn write_to_console(output_string:&String){
        print!("{}",&output_string);
        io::stdout().flush();
}