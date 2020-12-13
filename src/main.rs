#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

// use std::time::SystemTime;
use rocket::request::{Form, FormError, FormDataError};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};

fn main() {
    rocket::ignite().mount("/api", routes![response]).launch();
}

#[post("/api", data = "<input>")]
fn response(input: Result<Form<Parameters>, FormError>) -> Json<Message> {
    // let parameters = input.into_inner();

    match input {
        Ok(form) => {
            return Json(Message {
                response_type: "in_channel".to_string(),
                text: format!("{:?}", &*form),
            });
        },
        Err(FormDataError::Io(_)) => {
            return Json(Message {
                response_type: "ephemeral".to_string(),
                text: format!("Form input was inalid UTF-8."),
            });
        },
        Err(FormDataError::Malformed(f)) | Err(FormDataError::Parse(_, f)) => {
            return Json(Message {
                response_type: "ephemeral".to_string(),
                text: format!("Invalid form input: {}", f),
            });
        }
    };

    // Json(Message {
    //     response_type: "in_channel".to_string(),
    //     text: format!("Hello {}, the date and time is {:#?}.", parameters.user_id, SystemTime::now()),
    // })
}

#[derive(Debug, FromForm)]
pub struct Parameters {
    pub token: String,
    pub team_id: String,
    pub team_domain: String,
    pub channel_id: String,
    pub channel_name: String,
    pub user_id: String,
    pub user_name: String,
    pub command: String,
    pub text: String,
    pub response_url: String,
    pub trigger_id: String,
    pub api_app_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub response_type: String,
    pub text: String,
}
