use crate::{models::commen::*, AppState};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::{Data as ActData, Json as ActJson, Path as ActPath, Query as ActQuery},
    HttpResponse,
};

pub use super::coutries::*;

use service::*;
use uuid::Uuid;

type State = ActData<AppState>;

pub async fn create_contact(state: State) -> HttpResponse {}

pub async fn get_contact(state: State) -> HttpResponse {}

pub async fn update_contact(state: State) -> HttpResponse {}

pub async fn delete_contact(state: State) -> HttpResponse {}
