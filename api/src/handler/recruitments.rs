use axum::response::Response;

pub async fn read_recruitments() -> Result<Response, Response> {
    todo!()
    // let primitive_recruitments = get_recruitment_summary_list(&state.client).await;
    // if let Err(e) = primitive_recruitments {
    //     return Err((StatusCode::INTERNAL_SERVER_ERROR, e).into_response());
    // }
    // let primitive_recruitments = primitive_recruitments.unwrap();
    //
    // let mut result: Vec<Value> = Vec::new();
    // for car_pool in primitive_recruitments {
    //     let car_pool = car_pool.sophisticate();
    //     // 正常でない募集の場合はレスポンスに含まない
    //     if car_pool.is_err() {
    //         continue;
    //     }
    //     let car_pool = car_pool.unwrap();
    //     result.push(car_pool.to_value());
    // }
    // Ok((StatusCode::OK, json!(result).to_string()).into_response())
}
