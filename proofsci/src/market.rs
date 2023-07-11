
use actix_session::Session;
use actix_web::{
    error,
    web::{self},
    Error, HttpResponse, Result,
};
use tera::{Context, Tera};

// market.html render
pub async fn index(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();

    if let Some(user) = session.get::<String>("user")? {
        ctx.insert("user", &user)
    }

    let a = tmpl
        .render("market.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}