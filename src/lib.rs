mod db;
mod graphql;
mod utils;

use worker::{Context, Env, Request, Response, RouteContext, Router};

#[worker::event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Hey! Here is how to use it https://github.com/VictorBulba/test-grafbase/blob/master/README.md"))
        .get_async("/get_user_bio", get_user_bio)
        .post_async("/create_user", create_user)
        .post_async("/update_user_bio", update_user_bio)
        .run(req, env)
        .await
}

#[derive(serde::Deserialize)]
struct UserReq<'a> {
    username: &'a str,
}

#[derive(serde::Deserialize)]
struct UserReqWithPassword<'a> {
    username: &'a str,
    password: &'a str,
}

async fn get_user_bio(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let url = req.url()?;

    let Ok(req_params) = serde_urlencoded::from_str::<UserReq>(url.query().unwrap_or("")) else {
        return Response::error("Bad request", 400)
    };

    let Some(user) = db::user::get_user(&req_params.username, &ctx).await else {
        return Response::error(format!("User `{}` does not exist", req_params.username), 400)
    };

    Response::ok(user.bio)
}

async fn create_user(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let url = req.url()?;

    let Ok(req_params) = serde_urlencoded::from_str::<UserReqWithPassword>(url.query().unwrap_or("")) else {
        return Response::error("Bad request", 400)
    };

    let user = db::user::get_user(&req_params.username, &ctx).await;

    if user.is_some() {
        return Response::error(
            format!(
                "User with username `{}` already exists",
                req_params.username
            ),
            400,
        );
    }

    let bio = req.text().await?;

    db::user::create_user(&req_params.username, &req_params.password, &bio, &ctx).await;

    Response::ok("")
}

async fn update_user_bio(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let url = req.url()?;

    let Ok(req_params) = serde_urlencoded::from_str::<UserReqWithPassword>(url.query().unwrap_or("")) else {
        return Response::error("Bad request", 400)
    };

    let Some(user) = db::user::get_user(&req_params.username, &ctx).await else {
        return Response::error(
            format!("User `{}` does not exist", req_params.username),
            400,
        )
    };

    if user.password != req_params.password {
        return Response::error("Used not authorized", 401);
    }

    let new_bio = req.text().await?;

    db::user::update_bio(&user.id, &new_bio, &ctx).await;

    Response::ok("")
}
