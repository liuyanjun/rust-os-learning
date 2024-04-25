use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
// use tower_http::services;
// use std::fs::File;
use axum::{routing::{get, post},
        //    http::StatusCode,
        Json, Router,};
// use std::io::{self, prelude::*, BufReader};
use askama_axum::Template;
use axum::Form;
// use lazy_static::lazy_static;

#[derive(Serialize, Deserialize, Debug)]
struct RequestBody {
    prompt: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ResponseBody{
    id: String,
    object: String,
    created: u32,
    result: String
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Deserialize, Serialize, Debug)]
struct Question{
    query: String,
    db_schema: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    
    println!("AI-BOT starting now");
    //initialize tracing
    tracing_subscriber::fmt::init();
    
    //build our application with a route
    let app = Router::new()
    .route("/index", get(index))
    // .route("/spinner", get(spinner))
    .route("/chatllm", post(fetch_answer))
    .nest_service("/static", 
        ServeDir::new("static")
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;
    axum::serve(listener, app).await.unwrap();
    //fetch_answer().await?;

    Ok(())

}


async fn index() -> IndexTemplate{
    IndexTemplate
}

// async fn spinner() -> SpinnerTemplate{
//     SpinnerTemplate
// }

async fn get_input(reader: String)-> String{
    // let file = File::open(r"C:\Daimler\GenAI\CMS_DB.SQL").unwrap();
    // let reader = BufReader::new(file);
    let mut s = String::new();

    for line in reader.lines(){
        s.push_str(&line.trim());//.unwrap()
        s.push_str("\n");

    }
    // s = s.replace("\r", "\\n");
    // let s = reader.replace('\n', " ");
    // let s = s.replace('\r', " ");//^   
    // let s = s.replace('^', "\\n");//^ 
    s    
}

async fn fetch_answer(Form(question): Form<Question>) -> String{

    let question = format!("### Task\n
    Generate a SQL query to answer the following question:\n
    `{}`\n
    ### Database Schema\n
    This query will run on a database whose schema is represented in this string:\n
    {}\n
    ### SQL\n
    Given the database schema, here is the SQL query that answers `{}`\n", question.query, question.db_schema.unwrap_or("".to_string()), question.query);
    println!("{}", &question);
    let prompt = get_input(question).await;
    //println!("prompt:{}", &prompt);
    let request = RequestBody{
        prompt: prompt,
    };
    
    // println!("{}", serde_json::to_string(&request).unwrap());
    let proxy = reqwest::Proxy::https("http://127.0.0.1:3128").unwrap();
       // .basic_auth("liuyanj", "xxx");

    //let client = reqwest::Client::new();
    let client = reqwest::Client::builder()
        .proxy(proxy)
        .build().unwrap();
    // let res = client.post("https://www.baidu.com")
    let res = client.post("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/completions/sqlcoder_7b?access_token=xxxx.282335-58092575")
    .json(&request)
    .send()
    .await.unwrap()
    .json::<ResponseBody>()
    .await;//.unwrap();
    match res {
        Err(errstr) => errstr.to_string(),
        Ok(res) => res.result,
    }

    //println!("{}", &res.result);
    //Ok(()).ok_or("No results found".into())
    //res.result
}
