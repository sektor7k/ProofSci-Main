use actix_session::Session;
use actix_web::{
    error,
    web::{self},
    Error, HttpResponse, Result,
};
use bcrypt::DEFAULT_COST;
use serde::*;
use sqlx::SqlitePool;
use tera::{Context, Tera};
use validator::Validate;
use crate::apps::*;

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
    avatar: String,
}

// signin.html render
pub async fn signin(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    if let Some(_) = session.get::<String>("user")? {
        return Ok(redirct("/"));
    }
    let ctx = Context::new();
    let a = tmpl
        .render("signin.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}

// signin verilerini veri tabanına göndermek
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
                // Kullanıcıyı tekrar sorgula
                let result: Result<User, sqlx::Error> =
                    sqlx::query_as("select * from users where email = $1")
                        .bind(&user.email)
                        .fetch_one(&**conn)
                        .await;
                match result {
                    Ok(user) => {
                        session.insert("user", &user.username)?;
                        session.insert("email", &user.email)?;
                        session.insert("user_id", &user.id)?;
                        session.insert("avatarimg", &user.avatar)?;
                        return Ok(redirct("/profile"));
                    }
                    Err(_) => {
                        // Hata durumunda ne yapılacağı
                    }
                }
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
