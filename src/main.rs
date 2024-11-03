#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::TempFile;

#[derive(FromForm)]
struct Upload<'r> {
    file: TempFile<'r>,
}

#[get("/")]
fn index() -> &'static str {
    "Hi, frust!"
}

#[post("/upload", data = "<upload>")]
async fn upload(mut upload: Form<Upload<'_>>) -> String {
    // get file name
    let file_name = upload.file.name().unwrap().to_string();

    // Set path name
    let path = format!("/your/path/here/{}", file_name);

    // Set the path reference with '&', because when we set this variable here, its moved
    let result = upload.file.persist_to(&path).await;

    // If there is an error when the file is persisted, generate an exception
    if result.is_err() { panic!("{:?}", result.err().unwrap()); }

    // Return success message
    format!("File named {:?} has been saved!", file_name)
}

#[launch] // defines this function as main (without this function 'rocket' should be 'main')
fn rocket() -> _ {
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", 5000)))
        .mount("/", routes![index])
        .mount("/files", routes![upload])
}


