use worker::RouteContext;

#[derive(serde::Serialize)]
pub(crate) struct QueryRequest<T> {
    pub(crate) query: &'static str,
    pub(crate) variables: T
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct GraphQLError {
    // This field is only used for debug messages
    #[allow(dead_code)]
    pub(crate) message: String,
}

#[derive(serde::Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

impl<T> Into<Result<T, String>> for GraphQLResponse<T> {
    fn into(self) -> Result<T, String> {
        match (self.data, self.errors) {
            (Some(data), None) => Ok(data),
            (_, Some(errors)) if !errors.is_empty() => Err(format!("{:?}", errors)),
            _ => Err("Unknown GraphQL error".to_string()),
        }
    }
}

impl<T: serde::Serialize> QueryRequest<T> {
    pub(crate) async fn send<R: serde::de::DeserializeOwned>(&self, ctx: &RouteContext<()>) -> Result<R, String> {
        reqwest::Client::new()
            .post("https://test-grafbase-victorbulba.grafbase.app/graphql")
            .header("x-api-key", &ctx.env.var("GRAFBASE_API_KEY").unwrap().to_string())
            .json(&self)
            .send()
            .await
            .expect("Failed to send GraphQL request")
            .json::<GraphQLResponse<R>>()
            .await
            .expect("Failed to decode GraphQL response")
            .into()
    }
}
