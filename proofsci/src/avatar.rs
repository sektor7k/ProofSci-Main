use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{
    error,
    web::{self},
    Error, HttpResponse, Result,
};
use futures::{StreamExt, TryStreamExt};
use serde::*;
use sqlx::SqlitePool;
use std::fs;
use tera::{Context, Tera};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize)]
pub struct Avatar {
    avatar: String,
}

// updateprofile.html render
pub async fn update(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();

    if let Some(user) = session.get::<String>("user")? {
        ctx.insert("user", &user)
    }
    if let Some(avatarimg) = session.get::<String>("avatarimg")? {
        ctx.insert("avatarimg", &avatarimg)
    }

    let a = tmpl
        .render("updateprofile.html", &ctx)
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}

// kullancı avatar upload
pub async fn post_avatar(
    _tmpl: web::Data<Tera>,
    mut payload: Multipart,
    session: Session,
    conn: web::Data<SqlitePool>,
) -> Result<HttpResponse, Error> {
    let mut count = 1;

    // Kullanıcının adını al
    let user = if let Some(user) = session.get::<String>("user")? {
        user
    } else {
        return Ok(HttpResponse::Unauthorized().finish());
    };

    // Kullanıcının resim yüklemelerinin tutulduğu klasörü oluştur
    let upload_dir = format!("upload/{}/img/", user);
    fs::create_dir_all(&upload_dir)?;

    // Resmi işle
    while let Some(mut field) = payload.try_next().await? {
        // Resmin dosya adını oluştur
        let filename = generate_filename(&user, count);
        let filepath = format!("{}{}", upload_dir, filename);

        // Resmi diske kaydet
        let mut f = File::create(&filepath).await?;
        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f.write_all(&data).await?;
        }

        // Kullanıcının avatar sütununu güncelle
        update_avatar_in_database(&conn, &user, &filename).await?;

        count += 1;
    }

    Ok(HttpResponse::SeeOther()
        .header("Location", "/updateprofile")
        .finish())
}

fn generate_filename(username: &str, count: u32) -> String {
    format!("{}profilresmi{:03}.jpg", username, count)
}

async fn update_avatar_in_database(
    conn: &SqlitePool,
    username: &str,
    filename: &str,
) -> Result<(), Error> {
    // Kullanıcının avatar sütununu güncelleme işlemlerini yap
    sqlx::query!(
        "UPDATE users SET avatar = $1 WHERE username = $2",
        filename,
        username
    )
    .execute(conn)
    .await
    .map_err(|e| error::ErrorInternalServerError(format!("Failed to update avatar URL: {}", e)))?;

    Ok(())
}
