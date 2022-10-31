use worker::RouteContext;

#[derive(serde::Deserialize)]
pub(crate) struct User {
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) bio: String,
}

#[derive(serde::Deserialize)]
struct UserResponse {
    user: Option<User>,
}

pub(crate) async fn get_user(username: &str, ctx: &RouteContext<()>) -> Option<User> {
    let req = crate::graphql::QueryRequest {
        query: include_str!("get_user.graphql"),
        variables: serde_json::json!({ "by": { "username": username } }),
    };

    req.send::<UserResponse>(ctx)
        .await
        .expect("GraphQL error")
        .user
}

pub(crate) async fn create_user(username: &str, password: &str, bio: &str, ctx: &RouteContext<()>) {
    let req = crate::graphql::QueryRequest {
        query: include_str!("create_user.graphql"),
        variables: serde_json::json!(
            {
                "input": {
                    "username": username,
                    "password": password,
                    "bio": bio,
                }
            }
        ),
    };

    req.send::<UserResponse>(ctx).await.expect("GraphQL error");
}

pub(crate) async fn update_bio(id: &str, new_bio: &str, ctx: &RouteContext<()>) {
    let req = crate::graphql::QueryRequest {
        query: include_str!("update_user.graphql"),
        variables: serde_json::json!(
            {
                "id": id,
                "input": {
                    "bio": new_bio,
                }
            }
        ),
    };

    req.send::<UserResponse>(ctx).await.expect("GraphQL error");
}
