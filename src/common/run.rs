use std::collections::HashMap;
use actix_web::{middleware, web, App, HttpServer};


use super::database;
use super::http::handlers;



pub async fn server(ip: String, config_db: HashMap<String, String>) -> std::io::Result<()> {
    log::info!("starting HTTP server at http://{}:8081", &ip);

    let pool = database::create_pool(config_db);

    let server = HttpServer::new(move || {
        App::new()
            // enable logger
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            // <- limit size of the payload (global configuration)
            .service(web::resource("/signIn").route(web::post().to(handlers::sign_in)))
            .service(web::resource("/signOut").route(web::post().to(handlers::sign_out)))
            .service(web::resource("/account").route(web::post().to(handlers::account)))
            .service(web::resource("/trades").route(web::post().to(handlers::trade)))
            .service(web::resource("/position").route(web::post().to(handlers::positions)))
            .service(web::resource("/net_worth").route(web::post().to(handlers::net_worth)))
            .service(web::resource("/income").route(web::post().to(handlers::incomes)))
            .service(web::resource("/equity").route(web::post().to(handlers::pnl)))
            .service(web::resource("/newPrice").route(web::post().to(handlers::is_price)))
            .service(web::resource("/incomes").route(web::post().to(handlers::history_incomes)))
            .service(web::resource("/open_orders").route(web::post().to(handlers::open_orders)))
            .service(web::resource("/accounts").route(web::post().to(handlers::assets)))
            .service(web::resource("/date_history").route(web::post().to(handlers::date_trade)))
            .service(web::resource("/products").route(web::post().to(handlers::get_products_data)))
            .service(web::resource("/alarm_orders").route(web::post().to(handlers::get_open_orders_data)))
            .service(web::resource("/delect_orders").route(web::post().to(handlers::delect_open_orders_data)))
            .service(web::resource("/add_orders").route(web::post().to(handlers::add_open_orders_data)))
            .service(web::resource("/get_positions").route(web::post().to(handlers::get_positions_data)))
            .service(web::resource("/delect_positions").route(web::post().to(handlers::delect_positions_data)))
            .service(web::resource("/add_positions").route(web::post().to(handlers::add_positions_data)))
            .service(web::resource("/update_positions").route(web::post().to(handlers::update_positions_data)))
            .service(web::resource("/update_ori_balance").route(web::post().to(handlers::update_ori_balance_data)))
            .service(web::resource("/get_accounts").route(web::post().to(handlers::get_account)))
            .service(web::resource("/update_accounts_alarm").route(web::post().to(handlers::update_accounts_alarm)))
            .service(web::resource("/delete_accounts").route(web::post().to(handlers::delete_accounts_data)))
            .service(web::resource("/add_accounts").route(web::post().to(handlers::add_accounts_data)))
            .service(web::resource("/select_id").route(web::post().to(handlers::select_tra_id)))
            .service(web::resource("/get_net_worths").route(web::post().to(handlers::get_net_worths_data)))
            .service(web::resource("/get_equitys").route(web::post().to(handlers::get_equitys_data)))
            .service(web::resource("/get_single_account").route(web::post().to(handlers::single_account)))
            .service(web::resource("/get_bybit_account").route(web::post().to(handlers::bybit_account)))
            .service(web::resource("/get_bybit_equity").route(web::post().to(handlers::get_bybit_equity)))
            .service(web::resource("/get_bian_equity").route(web::post().to(handlers::get_bian_equity)))
            .service(web::resource("/test_xh01_one").route(web::post().to(handlers::get_bian_xh01_equity)))
            .service(web::resource("/test_xh01_two").route(web::post().to(handlers::get_bian_xh01_equity_one)))
            .service(web::resource("/test_xh01_three").route(web::post().to(handlers::get_bian_xh01_equity_two)))
            .service(web::resource("/test_xh01_four").route(web::post().to(handlers::get_bian_xh01_equity_three)))
            .service(web::resource("/test_xh01_five").route(web::post().to(handlers::get_bian_xh01_equity_four)))
            .service(web::resource("/test_xh01_six").route(web::post().to(handlers::get_bian_xh01_equity_five)))
            .service(web::resource("/test_trader02_one").route(web::post().to(handlers::get_bian_trader02_equity_one)))
            .service(web::resource("/test_trader02_two").route(web::post().to(handlers::get_bian_trader02_equity_two)))
            .service(web::resource("/test_trader02_three").route(web::post().to(handlers::get_bian_trader02_equity_three)))
            .service(web::resource("/test_trader04_one").route(web::post().to(handlers::get_bian_trader04_equity_one)))
            .service(web::resource("/test_trader04_two").route(web::post().to(handlers::get_bian_trader04_equity_two)))
            .service(web::resource("/test_trader04_three").route(web::post().to(handlers::get_bian_trader04_equity_three)))
            .service(web::resource("/test_trader04_four").route(web::post().to(handlers::get_bian_trader04_equity_four)))
            .service(web::resource("/test_xh02_one").route(web::post().to(handlers::get_bian_xh02_equity_one)))
            .service(web::resource("/test_xh02_two").route(web::post().to(handlers::get_bian_xh02_equity_two)))
            .service(web::resource("/test_xh02_three").route(web::post().to(handlers::get_bian_xh02_equity_three)))
            .service(web::resource("/test_xh02_four").route(web::post().to(handlers::get_bian_xh02_equity_four)))
            .service(web::resource("/get_traders").route(web::post().to(handlers::select_traders)))


            
    })
    .bind((ip.as_str(), 8081))?
    .run();

    return server.await;
}