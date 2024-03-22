

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use leptos::*;

    use dotenv::dotenv;
    use std::env;

    use argon2::{
        password_hash::{
            rand_core::OsRng,
            PasswordHasher, 
            SaltString
        },
        Argon2
    };

    use actix_identity::IdentityMiddleware;
    use actix_session::{storage::RedisSessionStore, SessionMiddleware};
    use lettre::{transport::smtp::authentication::Credentials, Tokio1Executor};
    use lettre::{AsyncSmtpTransport, AsyncTransport};
    use handlebars::Handlebars;

    use leptos_actix::{generate_route_list, LeptosRoutes};

    use vagasemaraxa::app::*;
    use vagasemaraxa::AppState;

    use sea_orm::{prelude::*, Database};
    use migration::{Migrator, MigratorTrait};

    dotenv().ok();

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);

    // Create database access
    let conn = Database::connect(env::var("DATABASE_URL").unwrap()).await.unwrap();
    Migrator::up(&conn, None).await.expect("Migrations failed");
    let secret_key = env::var("SECRET_KEY").unwrap(); 
    let credentials = Credentials::new(env::var("SMTP_MAIL").unwrap(), env::var("SMTP_PASSWORD").unwrap());
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay("mail.vagasemaraxa.com.br")
        .unwrap()
        .port(587)
        .credentials(credentials)
        .build();

    let mut template_engine = Handlebars::new();
    template_engine.register_template_file("base", "templates/layouts/base.hbs").expect("Houve um problema com o registro do template \"base.hbs\".");
    template_engine.register_template_file("styles", "templates/layouts/styles.hbs").expect("Houve um problema com o registro do template \"styles.hbs\".");
    template_engine.register_template_file("tpl-confirm_mail", "templates/verification_code.hbs").expect("Houve um problema com o registro do template \"verfication_code.hbs\".");

    let state = AppState { conn, secret_key, mailer, template_engine };

    // Session service for identity
    let session_key = cookie::Key::generate();

    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    // if not user, add it:

    use sea_orm::{Set, ActiveModelTrait};
    use entities::{
        user,
        role,
        user_roles,
        membership
    };
    use entities::prelude::{
        User,
        UserRoles,
        Membership,
        Role
    };

    if let Ok(None) = Membership::find().filter(membership::Column::IsLifetime.eq(true)).one(&state.conn).await {
        let salt = SaltString::generate(&mut OsRng);
        let secret_key = state.secret_key.clone().into_bytes();
        let argon2 = Argon2::new_with_secret(
            secret_key.as_slice(), 
            argon2::Algorithm::default(), 
            argon2::Version::default(), 
            argon2::Params::default())
        .unwrap();
        let password_hash = argon2.hash_password(b"admin", &salt).unwrap().to_string();
        
        let new_admin_user = user::ActiveModel {
            username: Set("admin".into()),
            email: Set("admin@localhost".into()),
            password: Set(password_hash),
            cellphone: Set(None),
            is_confirmed: Set(true),
            created_at: Set(chrono::Utc::now().with_timezone(&chrono::FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone"))),
            updated_at: Set(chrono::Utc::now().with_timezone(&chrono::FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone"))),
            ..Default::default()
        };

        let admin = new_admin_user.insert(&state.conn).await.unwrap();
        let admin_role = Role::find().filter(role::Column::Name.eq("Administrador")).one(&state.conn).await.unwrap().unwrap();
        let new_membership = membership::ActiveModel {
            user_id: Set(admin.id),
            is_lifetime: Set(true),
            expires_at: Set(chrono::Utc::now().with_timezone(&chrono::FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone"))),
            ..Default::default()
        };
        let new_role_relation = user_roles::ActiveModel {
            user_id: Set(admin.id),
            role_id: Set(admin_role.id),
            ..Default::default()
        };

        new_role_relation.insert(&state.conn).await.expect("Something went wrong when relating admin user with admin role.");
        new_membership.insert(&state.conn).await.expect("Something went wrong when adding admin user membership.");
    }

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                session_key.clone()
            ))
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .app_data(web::Data::new(state.clone()))
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use leptos::*;
    use vagasemaraxa::app::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
