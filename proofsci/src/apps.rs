use actix_session::Session;
use actix_web::{error, http, web, Error, HttpResponse, Result};
use bcrypt::DEFAULT_COST;
use serde::*;
use sqlx::SqlitePool;
use tera::{Context, Tera};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, sqlx::FromRow)]
pub struct LoginUser {
    #[validate(email)]
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SigninUser {
    #[validate(email)]
    email: String,
    #[validate(length(min = 4))]
    username: String,
    #[validate(must_match = "password2", length(min = 5))]
    password: String,
    password2: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    id: i32,
    email: String,
    username: String,
    password: String,
}

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
pub async fn index2(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();

    if let Some(user) = session.get::<String>("user")? {
        ctx.insert("user", &user)
    }

    let a = tmpl
        .render("profile.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}
pub async fn index3(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();

    if let Some(user) = session.get::<String>("user")? {
        ctx.insert("user", &user)
    }

    let a = tmpl
        .render("create.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}

pub async fn login(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    if let Some(user) = session.get::<String>("user")? {
        return Ok(redirct("/"));
    }
    let ctx = Context::new();
    let a = tmpl
        .render("login.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}

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
                    return Ok(redirct("/"));
                } else {
                    let mut ctx = tera::Context::new();
                    ctx.insert("error", "wrong password!");

                    let rendered = _tmpl
                        .render("login.html", &ctx)
                        .map_err(error::ErrorInternalServerError)?;

                    return Ok(HttpResponse::Ok().content_type("text/html").body(rendered));
                }
            }
            Err(e) => {
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

pub async fn logout(session: Session) -> Result<HttpResponse, Error> {
    session.purge();

    return Ok(redirct("/"));
}

pub fn redirct(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header((http::header::LOCATION, location))
        .finish()
}

pub async fn signin(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    if let Some(user) = session.get::<String>("user")? {
        return Ok(redirct("/"));
    }
    let ctx = Context::new();
    let a = tmpl
        .render("signin.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}

pub async fn post_signin(
    _tmpl: web::Data<Tera>,
    form2: web::Form<SigninUser>,
    session: Session,
    conn: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    //let ctx = Context::new();

    let user = form2.into_inner();
    if let Ok(_) = user.validate() {
        let add_user =
            sqlx::query("insert into users (username, email, password) values($1, $2, $3)")
                .bind(&user.username)
                .bind(&user.email)
                .bind(&bcrypt::hash(&user.password, DEFAULT_COST).expect("şifreleme hatali"))
                .execute(&**conn)
                .await;

        match add_user {
            Ok(_) => {
                session.insert("user", &user.username)?;
                return Ok(redirct("/"));
            }
            Err(_) => {
                let mut ctx = tera::Context::new();
                ctx.insert("errorsignin", "wrong signin!");

                let rendered = _tmpl
                    .render("signin.html", &ctx)
                    .map_err(error::ErrorInternalServerError)?;

                return Ok(HttpResponse::Ok().content_type("text/html").body(rendered));
            }
        };
    }
    return Ok(redirct("/signin"));
}
