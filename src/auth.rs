// use http::StatusCode;
// use time::Duration;
// use tower::ServiceBuilder;
// use axum_login::{
//     permission_required,
//     tower_sessions::{Expiry, MemoryStore, SessionManagerLayer},
//     AuthManagerLayerBuilder,
// };
// use sqlx::PgPool;
//
// fn create_auth_service(db: PgPool) -> u32 {
//         // Session layer.
//         //
//         // This uses `tower-sessions` to establish a layer that will provide the session
//         // as a request extension.
//         let session_store = MemoryStore::default();
//         let session_layer = SessionManagerLayer::new(session_store)
//             .with_secure(false)
//             .with_expiry(Expiry::OnInactivity(Duration::days(1)));
//
//         // Auth service.
//         //
//         // This combines the session layer with our backend to establish the auth
//         // service which will provide the auth session as a request extension.
//         let backend = Backend::new(db);
//         let auth_service = ServiceBuilder::new()
//             .layer(HandleErrorLayer::new(|_: BoxError| async {
//                 StatusCode::BAD_REQUEST
//             }))
//             .layer(AuthManagerLayerBuilder::new(backend, session_layer).build())
// }
//
//
//
