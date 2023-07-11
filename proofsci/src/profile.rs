use actix_session::Session;
use actix_web::{
    error,
    web::{self
    },
    Error, HttpResponse, Result,
};
use sqlx::SqlitePool;
use tera::{Tera, Context};
use crate::create::*;


// profile.html render
pub async fn index2(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();

    if let Some(user) = session.get::<String>("user")? {
        ctx.insert("user", &user)
    }
    if let Some(email) = session.get::<String>("email")? {
        ctx.insert("email", &email)
    }

    let a = tmpl
        .render("profile.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}

// kullanıcının verilerini veri tabanından profile yansıtma
pub async fn profile(
    _tmpl: web::Data<Tera>,
    session: Session,
    conn: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    let user_id = session.get::<i32>("user_id").unwrap_or(Some(0));

    let forms:Vec<FormUser> = sqlx::query_as::<_,FormUser>("SELECT * FROM forms WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(&**conn)
        .await
        .map_err(|e| {
            error::ErrorInternalServerError(format!("Error querying database: {:?}", e))
        })?; 
    
println!("{:?}",forms);
    


    let mut ctx = tera::Context::new();
    ctx.insert("forms", &forms);
    if let Some(user) = session.get::<String>("user")? {
        ctx.insert("user", &user)
        
    }
    if let Some(email) = session.get::<String>("email")? {
        ctx.insert("email", &email)
        
    }
    if let Some(avatarimg) = session.get::<String>("avatarimg")? {
        ctx.insert("avatarimg", &avatarimg)
    }

    let rendered = _tmpl
        .render("profile.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}
