pub mod user;

// See: https://actix.rs/docs/extractors
/* TLDR: The use of Request information extraction
    Path  | web::Path<T>  | Extract typed information from the request path.
    Query | web::Query<T> | Extract typed information from the query string.
    Json  | web::Json<T>  | Extract typed information from the request body.
    Data  | web::Data<T>  | Extract application data from the application state.
*/
