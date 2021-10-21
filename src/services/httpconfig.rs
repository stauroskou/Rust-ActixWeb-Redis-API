use actix_web::web;

use crate::services::configurations_manager;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/").route(web::get().to(configurations_manager::get_user)));
    cfg.service(
        web::resource("/delete-user/").route(web::delete().to(configurations_manager::delete_user)),
    );
    cfg.service(
        web::resource("/update-user/").route(web::patch().to(configurations_manager::put_user)),
    );
    cfg.service(
        web::resource("/create-user/").route(web::post().to(configurations_manager::post_user)),
    );
}
