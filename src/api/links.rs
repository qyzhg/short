use actix_web::{
    get,
    http::header,
    post,
    web::{self, Json, Path},
    HttpResponse, Responder,
};
use actix_web::http::StatusCode;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::{Error, MySql, Pool};
use url::{Url, Host, Position};

use crate::api::ApiResult;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
struct Link {
    tiny_code: String,
    origin_url: String,
}

#[derive(Deserialize, Clone)]
struct ApiAddLink {
    origin_url: String,
}

impl ApiAddLink {
    fn to_new_link(self) -> Link {
        Link {
            tiny_code: nanoid!(5),
            origin_url: self.origin_url,
        }
    }
}

#[post("/create")]
async fn create_link(link: Json<ApiAddLink>, data: web::Data<Pool<MySql>>) -> impl Responder {
    let new_link = link.0.to_new_link();
    // 检查url合法性
    if let Err(e) = Url::parse(&new_link.origin_url) {
        return Json(ApiResult::error(e.to_string()));
    };
    // 查询是否有历史记录
    match get_tiny_code(data.as_ref().clone(), new_link.origin_url.clone()).await{
        Ok(old_tiny_code) => {
            // 如果能查到历史记录，就把历史记录返回
            Json(ApiResult::success(Some(old_tiny_code)))
        },
        Err(e) => {
            match e {
                // 如果没有返回值的情况，就新建一条，然后把新的值返回
                Error::RowNotFound => {
                    let new_code = new_link.tiny_code.clone();
                    if let Err(e) = insert_into_tiny_link(data.as_ref().clone(), new_link).await {
                        return Json(ApiResult::error(e.to_string()));
                    }
                    Json(ApiResult::success(Some(new_code)))
                }
                // 有正经错误就抛异常
                _ => Json(ApiResult::error(e.to_string()))
            }
        }
    }
}

async fn insert_into_tiny_link(pool: Pool<MySql>, new_link: Link) -> Result<u64, sqlx::Error> {
    let insert_id = sqlx::query(r#"insert into tiny_link(tiny_code, origin_url) values (?, ?)"#)
        .bind(new_link.tiny_code)
        .bind(new_link.origin_url)
        .execute(&pool)
        .await?
        .last_insert_id();

    Ok(insert_id)
}

#[get("/{code}")]
async fn get_from_link(path: Path<String>, data: web::Data<Pool<MySql>>) -> impl Responder {
    let code = path.into_inner();
    let url = get_original_url(data.as_ref().clone(), code).await;
    let url = match url {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            return HttpResponse::NotFound().finish();
        }
    };
    HttpResponse::Found()
        .append_header((header::LOCATION, url))
        .finish()
}

#[get("/s/{code}")]
async fn get_origin_url_from_link(path: Path<String>, data: web::Data<Pool<MySql>>) -> impl Responder {
    let code = path.into_inner();
    let url = get_original_url(data.as_ref().clone(), code).await;
    let url = match url {
        Ok(x) => x,
        Err(e) => {
            print!("{}", e);
            return  Json(ApiResult::error("404"))
        }
    };
    Json(ApiResult::success(Some(url)))
}

async fn get_original_url(pool: Pool<MySql>, code: String) -> Result<String, sqlx::Error> {
    let row: (String,) = sqlx::query_as("SELECT origin_url from tiny_link where tiny_code = ?")
        .bind(code)
        .fetch_one(&pool)
        .await?;
    Ok(row.0)
}

async fn get_tiny_code(pool: Pool<MySql>, url: String) -> Result<String, sqlx::Error> {
    let row: (String,) = sqlx::query_as("SELECT tiny_code from tiny_link where origin_url = ?")
        .bind(url)
        .fetch_one(&pool)
        .await?;
    Ok(row.0)
}

#[get("/links")]
async fn get_all_links(data: web::Data<Pool<MySql>>) -> impl Responder {
    let links = get_links(data.as_ref().clone()).await;
    let links = match links {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            return Json(ApiResult::error(e.to_string()));
        }
    };
    Json(ApiResult::success(Some(links)))
}

async fn get_links(pool: Pool<MySql>) -> Result<Vec<Link>, sqlx::Error> {
    let row = sqlx::query_as::<_, Link>("SELECT tiny_code, origin_url from tiny_link")
        .fetch_all(&pool)
        .await?;
    Ok(row)
}
