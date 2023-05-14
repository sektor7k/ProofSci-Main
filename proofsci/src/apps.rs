use actix_session::Session;
use actix_web::{error, http, web::{self, Form}, Error, HttpResponse, Result};
use bcrypt::DEFAULT_COST;
use serde::*;
use sqlx::SqlitePool;
use tera::{Context, Tera};
use validator::Validate;
use sqlx::Pool;
use sqlx::Sqlite;

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

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize)]
pub struct EditProfile{
    username: String,
    
}


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


// login.html render
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
                    //session.set("user_id", &user.id); 
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


// kullanıcı çıkış yapmak
pub async fn logout(session: Session) -> Result<HttpResponse, Error> {
    session.purge();

    return Ok(redirct("/"));
}


// belirtilen lokasyona gitmek
pub fn redirct(location: &str) -> HttpResponse {
    HttpResponse::Found()
        .append_header((http::header::LOCATION, location))
        .finish()
}


// signin.html render
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
                session.insert("user", &user.username)?;
                session.insert("email", &user.email)?;
                
                
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

    let rendered = _tmpl
        .render("profile.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}






// updateprofile.html render
pub async fn update(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();

    if let Some(user) = session.get::<String>("user")? {
        ctx.insert("user", &user)
    }

    let a = tmpl
        .render("updateprofile.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}


// user name  güncelleme
pub async fn post_edit_profile(
    _tmpl: web::Data<Tera>,
    form: web::Form<EditProfile>,
    session: Session,
    conn: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    let old_username = session.get::<String>("user").unwrap().unwrap();
    let new_username = form.username.clone();

    sqlx::query("UPDATE users SET username = $1 WHERE username = $2")
        .bind(&new_username)
        .bind(&old_username)
        .execute(&**conn)
        .await;

    // Güncellenen kullanıcının username bilgisi, oturum verilerinde de güncellenmeli
    session.insert("user", &new_username)?;

    Ok(redirct("/profile"))
}