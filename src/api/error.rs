use rocket::{get, http::Status, response::status};

#[get("/errors?<page>")]
pub fn get(page: Option<usize>) -> status::Custom<&'static str> {
    let page = page.unwrap_or(0);
    status::Custom(Status::NotImplemented, "unimplemented")
}

#[get("/error/last")]
pub fn get_last() -> status::Custom<&'static str> {
    status::Custom(Status::NotImplemented, "unimplemented")
}

#[get("/error/<id>")]
pub fn get_id(id: usize) -> status::Custom<&'static str> {
    status::Custom(Status::NotImplemented, "unimplemented")
}
