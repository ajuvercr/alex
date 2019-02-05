mod dairy;
mod files;
mod login;
mod upload;

pub fn fuel(rocket: rocket::Rocket) -> rocket::Rocket {
    let rocket = login::fuel(rocket);
    let rocket = upload::fuel(rocket);
    let rocket = dairy::fuel(rocket);
    let rocket = files::fuel(rocket);

    rocket
}