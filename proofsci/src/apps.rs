use actix_session::Session;
use actix_web::{
    error, http::{self},
    web::{self},
    Error, HttpResponse, Result,
};
use tera::{Context, Tera};



// main render
pub async fn mainindex(tmpl: web::Data<Tera>) -> Result<HttpResponse, Error> {
    let ctx = Context::new();

    

    let a = tmpl
        .render("index.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}

// kullanıcı çıkış yapmak
pub async fn logout(session: Session) -> Result<HttpResponse, Error> {
    session.purge();

    return Ok(redirct("/market"));
}


// belirtilen lokasyona gitmek
pub fn redirct(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header((http::header::LOCATION, location))
        .finish()
}

