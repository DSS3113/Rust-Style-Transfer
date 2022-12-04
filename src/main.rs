mod style_transfer;
use bytes::BufMut;
use futures::TryStreamExt;
use std::convert::Infallible;
use uuid::Uuid;
use warp::{
    http::StatusCode,
    multipart::{FormData, Part},
    Filter, Rejection, Reply
};
use std::str;
use std::fs;
use lettre::{ message::Attachment, Message, message::MultiPart,
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport,
    Tokio1Executor
};
use std::path::Path;
use std::env;

async fn send_email(uuid: String, to_email: String) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_credentials = Credentials::new("johnrustdavid@gmail.com".to_string(), "dxmychfgprpofpdx".to_string());

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
        .credentials(smtp_credentials)
        .build();

    let from = "CS128H Project Server <johnrustdavid@gmail.com>";
    let to = format!("<{}>", to_email);
    let subject = "Image generated using style transfer";
    let body = "<h1>Here's the image</h1>".to_string();
    let image_body = Attachment::new(String::from("result.jpg")).body(
        fs::read(format!("./outputs/{}_output.jpg", uuid))?,
        "img/jpg".parse().unwrap(),
    );

    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .multipart(
            MultiPart::mixed()
            .singlepart(image_body)
        );

    mailer.send(email?).await;

    Ok(())
}



// Credits: https://github.com/zupzup/warp-upload-download-example
async fn upload_imgs_and_transfer_style(form: FormData) -> Result<impl Reply, Rejection> {

    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

    let uuid: String = Uuid::new_v4().to_string();
    let mut content_img_file_name: String = "content.jpg".to_string();
    let mut style_img_file_name: String = "style.jpg".to_string();
    let mut to_email: String = "abc@example.com".to_string();
    for p in parts {
        let p_name = p.name().to_string().clone();

        if p_name == "email" {
            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                })?;
            let value_str = match str::from_utf8(&value) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            to_email = value_str.clone().to_string();
            println!("Email to be sent to: {}", value_str);
        }

        else if p_name == "content_img" || p_name == "style_img" {
            let content_type = p.content_type();
            let file_ending;
            match content_type {
                Some(file_type) => match file_type {
                    "image/png" => {
                        file_ending = "png";
                    }
                    "image/jpeg" => {
                        file_ending = "jpeg";
                    }
                    v => {
                        eprintln!("invalid file type found: {}", v);
                        return Err(warp::reject::reject());
                    }
                },
                None => {
                    eprintln!("file type could not be determined");
                    return Err(warp::reject::reject());
                }
            }

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                }).unwrap();

            let file_name: String;

            if p_name == "content_img" {
                file_name = format!("./images/{}_content.{}", uuid.clone(), file_ending);
                content_img_file_name = file_name.clone();
            }
            else {
                file_name = format!("./images/{}_style.{}", uuid.clone(), file_ending);
                style_img_file_name = file_name.clone();
            }
            tokio::fs::write(&file_name, value).await.map_err(|e| {
                eprint!("error writing file: {}", e);
                warp::reject::reject()
            })?;
            println!("created file: {}", &file_name);
        }
    }
  let weights = "weights.ot";
  let response = style_transfer::style_transfer(uuid.to_string(), style_img_file_name.to_string(), content_img_file_name.to_string(), weights.to_string()).await;
  let email_result = send_email(uuid.clone(), to_email.clone()).await;
  if email_result.is_ok() {
      println!("Email sent successfully for UUID {}", uuid.clone());
  } else {
      eprintln!("Error: email not sent.");
  }

    Ok("success")
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}

#[tokio::main]
async fn main() {
    fs::create_dir_all("./outputs/").unwrap();
    fs::create_dir_all("./images/").unwrap();

    if !Path::new("weights.ot").exists() {
        println!("Download weights from https://drive.google.com/file/d/1KxgrUkgC3TeRWmW8GEmf9QWU4n5-KCpU/view?usp=sharing");
        return;
    }

    let upload_route = warp::path("style_transfer")
    .and(warp::post())
    .and(warp::multipart::form().max_length(5_000_000))
    .and_then(upload_imgs_and_transfer_style);


    let router = upload_route.recover(handle_rejection);

    println!("Server started.");
    warp::serve(router)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
