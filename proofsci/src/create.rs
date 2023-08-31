
use actix_session::Session;
use actix_web::{
    error  ,
    web::{self},
    Error, HttpResponse, Result,
};
use serde::*;
use sqlx::SqlitePool;
use tera::{Context, Tera};
use crate::apps::*;



#[derive(Debug, Deserialize, sqlx::FromRow, Serialize)]
pub struct FormUser {
    nft_name: String,
    nft_description: String,
    project_name: String,
    project_description: String,
    clinical_stage: String,
    research_area: String,
    patent_status: String,
    country: String
}


// create.html render
pub async fn create(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();

    if let Some(user) = session.get::<String>("user")? {
        ctx.insert("user", &user)
    }

    let a = tmpl
        .render("create.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}


// kullanıcı form verilerini veri tabanına ilişkilendirerek gönderme
pub async fn post_form(
    _tmpl: web::Data<Tera>,
    form2: web::Form<FormUser>,
    session: Session,
    conn: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    //let ctx = Context::new();

    let user_id = session.get::<i32>("user_id").unwrap_or(Some(0));
   
    

    let user = form2.into_inner();

    

    let forms = sqlx::query("insert into forms (nft_name, nft_description, project_name, project_description, clinical_stage, research_area, patent_status, country, user_id ) values($1, $2, $3, $4, $5, $6, $7, $8, $9)")
        .bind(&user.nft_name)
        .bind(&user.nft_description)
        .bind(&user.project_name)
        .bind(&user.project_description)
        .bind(&user.clinical_stage)
        .bind(&user.research_area)
        .bind(&user.patent_status)
        .bind(&user.country)
        .bind(user_id)
        .execute(&**conn)
        .await;
    match forms {
        Ok(_) => {
            
            
            return Ok(redirct("/profile"));
            
        }
        Err(_) => {
            let mut ctx = tera::Context::new();
            ctx.insert("user_id", &user_id);

            let rendered = _tmpl
                .render("create.html", &ctx)
                .map_err(error::ErrorInternalServerError)?;

            return Ok(HttpResponse::Ok().content_type("text/html").body(rendered));
        }
    };
}