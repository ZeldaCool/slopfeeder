use rocket::response::Redirect;


#[macro_use] extern crate rocket;


#[get("/")]
fn index() -> Redirect {
    let mut sus_counter = 0;
    if sus_counter >= 0{
        Redirect::to(uri!("/feedertime"))
    } else{
        Redirect::to(uri!("/feedertime"))
    }
}
#[get("/feedertime")]
fn feeder_time() -> &'static str{
    "Let's feed 'em slop :D"
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, feeder_time])

    }
