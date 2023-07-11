use actix_session::Session;
use actix_web::{
    error,
    web::{self},
    Error, HttpResponse, Result,
};
use serde::*;
use sqlx::SqlitePool;
use tera::{Context, Tera};
use validator::Validate;
use crate::apps::*;

#[derive(Debug, Deserialize, Validate, sqlx::FromRow)]
pub struct LoginUser {
    #[validate(email)]
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    id: i32,
    email: String,
    username: String,
    password: String,
    avatar: String,
}

// login.html render
pub async fn login(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    if let Some(_) = session.get::<String>("user")? {
        return Ok(redirct("/profile"));
    }
    let ctx = Context::new();
    let a = tmpl
        .render("login.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}

// login verilerini vri tabanında kontrol ettirip girişe izin veya red vermek
pub async fn post_login(
    _tmpl: web::Data<Tera>,
    form: web::Form<LoginUser>,
    session: Session,
    conn: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    //let ctx = Context::new();

    let login_form = form.into_inner();

    if let Ok(_) = login_form.validate() {
        let result: Result<User, sqlx::Error> =
            sqlx::query_as("select * from users where email = $1")
                .bind(&login_form.email)
                .fetch_one(&**conn)
                .await;
        match result {
            Ok(user) => {
                if bcrypt::verify(&login_form.password, &user.password).unwrap() {
                    session.insert("user", &user.username)?;
                    session.insert("email", &user.email)?;
                    session.insert("user_id", &user.id)?;
                    session.insert("avatarimg", &user.avatar)?;
                    //session.set("user_id", &user.id);
                    return Ok(redirct("/profile"));
                } else {
                    let mut ctx = tera::Context::new();
                    ctx.insert("error", "wrong password!");

                    let rendered = _tmpl
                        .render("login.html", &ctx)
                        .map_err(error::ErrorInternalServerError)?;

                    return Ok(HttpResponse::Ok().content_type("text/html").body(rendered));
                }
            }
            Err(_) => {
                let mut ctx = tera::Context::new();
                ctx.insert("errormail", "wrong Email!");

                let rendered = _tmpl
                    .render("login.html", &ctx)
                    .map_err(error::ErrorInternalServerError)?;

                return Ok(HttpResponse::Ok().content_type("text/html").body(rendered));
            }
        }
    }
    return Ok(redirct("/login"));

    // let a = tmpl
    //     .render("login.html", &ctx)
    //     .map_err(error::ErrorInternalServerError)?;
}

