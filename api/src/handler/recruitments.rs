use axum::response::{Response};

pub async fn read_recruitments(
) -> Result<Response, Response> {
    todo!()
    // let primitive_recruitments = get_recruitment_summary_list(&state.client).await;
    // if let Err(e) = primitive_recruitments {
    //     return Err((StatusCode::INTERNAL_SERVER_ERROR, e).into_response());
    // }
    // let primitive_recruitments = primitive_recruitments.unwrap();
    //
    // let mut result: Vec<Value> = Vec::new();
    // for recruitment in primitive_recruitments {
    //     let recruitment = recruitment.sophisticate();
    //     // 正常でない募集の場合はレスポンスに含まない
    //     if recruitment.is_err() {
    //         continue;
    //     }
    //     let recruitment = recruitment.unwrap();
    //     result.push(recruitment.to_value());
    // }
    // Ok((StatusCode::OK, json!(result).to_string()).into_response())
}
