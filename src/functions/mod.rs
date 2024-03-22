use leptos::*;

#[cfg(feature = "ssr")]
use leptos_actix::extract;
#[cfg(feature = "ssr")]
use lettre::AsyncTransport;
#[cfg(feature = "ssr")]
use sea_orm::*;

#[cfg(feature = "ssr")]
pub mod embed;

#[cfg(feature = "ssr")]
use entities;

use serde::{Deserialize, Serialize};
use chrono::{prelude::*, Days};
use wasm_bindgen::UnwrapThrowExt;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct JobPaginatorInfo {
    pub current_content: Vec<JobModel>,
    pub current_page: u64,
    pub num_pages: u64,
    pub num_items: u64,
}

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct GenericTDModel {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub children: Vec<GenericTDModel>
}

pub type PermissionModel = GenericTDModel;
pub type RoleModel = GenericTDModel;

#[cfg(feature = "ssr")]
impl From<entities::role::Model> for RoleModel {
    fn from(value: entities::role::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            children: vec![]
        }
    }
}

#[cfg(feature = "ssr")]
impl From<entities::permission::Model> for PermissionModel {
    fn from(value: entities::permission::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            children: vec![]
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct MembershipModel {
    pub id: i32,
    pub user_id: i32,
    pub is_lifetime: bool,
    pub expires_at: DateTime<FixedOffset>,
}

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct JobModel {
    pub id: i32,
    pub position: String,
    pub company: String,
    pub description: Option<String>,
    pub requirements: Vec<String>,
    pub published_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>
}

impl JobModel {
    pub fn new(position: String, company: String, description: Option<String>, requirements: Vec<String>) -> Self {
        Self {
            id: -1,
            position,
            company,
            description,
            requirements,
            published_at: Utc::now().with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone")),
            updated_at: Utc::now().with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone"))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub cellphone: Option<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub is_confirmed: bool,
}

#[cfg(feature = "ssr")]
impl From<entities::user::Model> for UserModel {
    fn from(value: entities::user::Model) -> Self {
        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            password: value.password,
            cellphone: value.cellphone,
            created_at: value.created_at,
            updated_at: value.updated_at,
            is_confirmed: value.is_confirmed,
        }
    }
}

#[cfg(feature = "ssr")]
impl From<entities::membership::Model> for MembershipModel {
    fn from(value: entities::membership::Model) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            is_lifetime: value.is_lifetime,
            expires_at: value.expires_at
        }
    }
}

#[cfg(feature = "ssr")]
impl From<entities::job::Model> for JobModel {
    fn from(value: entities::job::Model) -> Self {
        Self {
            id: value.id,
            position: value.position,
            company: value.company,
            description: value.description,
            requirements: value.requirements,
            published_at: value.published_at,
            updated_at: value.updated_at
        }
    }
}

#[server(GeneratePaymentUrl, "/api")]
pub async fn generate_payment_url() -> Result<String, ServerFnError> {
    use embed::generate_payment_url;
    Ok(generate_payment_url())
}

#[server(UserHavePermission, "/api")]
pub async fn user_have_permission(user_id: i32, permission_name: String) -> Result<bool, ServerFnError> {
    use actix_web::web;
    use crate::AppState;

    use sea_orm::EntityTrait;

    use entities::{user_roles, role_permissions, permission};
    use entities::prelude::{UserRoles, RolePermissions, Permission};

    let app_state = extract::<web::Data<AppState>>().await.unwrap();
    let user_roles: Vec<user_roles::Model> = UserRoles::find()
        .filter(user_roles::Column::UserId.eq(user_id))
        .all(&app_state.conn)
        .await.unwrap_or(Vec::new());

    for user_role in user_roles.iter() {
        for role_permission in RolePermissions::find()
            .filter(role_permissions::Column::RoleId.eq(user_role.role_id))
            .all(&app_state.conn)
            .await.unwrap_or(Vec::new()).iter() {
                if let Some(_perm) = Permission::find_by_id(role_permission.permission_id).filter(permission::Column::Name.eq(permission_name.clone())).one(&app_state.conn).await.unwrap_or_default() {
                    return Ok(true)
                }
        }
    }

    Ok(false)
}

#[server(GetJobsPaginated, "/api")]
pub async fn get_jobs_paginated(page: u64) -> Result<JobPaginatorInfo, ServerFnError> {
    use actix_web::web;
    use crate::AppState;

    use entities::job;
    use entities::prelude::Job;

    let app_state = extract::<web::Data<AppState>>().await.unwrap();
    let jobs_page = Job::find().order_by_desc(job::Column::PublishedAt).paginate(&app_state.conn, 25);

    let mut jobs_list = Vec::<JobModel>::new();
    jobs_list.extend(jobs_page.fetch_page(page - 1).await?.into_iter().map(Into::into));

    let paginator_info = JobPaginatorInfo {
        current_content: jobs_list,
        current_page: jobs_page.cur_page(),
        num_pages: jobs_page.num_pages().await?,
        num_items: jobs_page.num_items().await?
    };
    Ok(paginator_info)
}

#[server(GetJob, "/api")]
pub async fn get_job(id: i32) -> Result<JobModel, ServerFnError> {
    use actix_web::web;
    use crate::AppState;

    use entities::job;
    use entities::prelude::Job;

    let state = extract::<web::Data<AppState>>().await.unwrap();
    let job: Option<job::Model> = Job::find().filter(job::Column::Id.eq(id)).one(&state.conn).await.unwrap();
    match job {
        Some(job) => Ok(job.into()),
        None => Err(ServerFnError::new("Essa vaga não foi encontrada."))
    }
}

#[server(GetJobs, "/api")]
pub async fn get_jobs() -> Result<Vec<JobModel>, ServerFnError> {
    use actix_web::web;
    use crate::AppState;

    use entities::job;
    use entities::prelude::Job;

    let app_state = extract::<web::Data<AppState>>().await.unwrap();
    let jobs: Vec<job::Model> = Job::find().order_by_desc(job::Column::UpdatedAt).all(&app_state.conn).await.unwrap_or(Vec::new());

    let mut jobs_list = Vec::<JobModel>::new();
    jobs_list.extend(jobs.into_iter().map(Into::into));
    Ok(jobs_list)
}

#[server(CreateUser, "/api")]
pub async fn create_user(new_user: UserModel) -> Result<UserModel, ServerFnError> {
    use actix_web::web;
    use crate::AppState;

    use argon2::{
        password_hash::{
            rand_core::OsRng,
            PasswordHasher, 
            SaltString
        },
        Argon2
    };

    let state = extract::<web::Data<AppState>>().await.unwrap();
    let secret = state.secret_key.clone().into_bytes();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::new_with_secret(
        secret.as_slice(), 
        argon2::Algorithm::default(), 
        argon2::Version::default(), 
        argon2::Params::default())
    .unwrap();
   
    let password_hash = argon2.hash_password(new_user.password.into_bytes().as_slice(), &salt).unwrap().to_string();

    use sea_orm::{Set, ActiveModelTrait};

    use entities::user;

    let app_state = extract::<web::Data<AppState>>().await.unwrap();
    let new_user = user::ActiveModel {
        username: Set(new_user.username),
        email: Set(new_user.email),
        password: Set(password_hash),
        cellphone: Set(new_user.cellphone),
        created_at: Set(Utc::now().with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone"))),
        updated_at: Set(Utc::now().with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone"))),
        ..Default::default()
    };

    let new_user = UserModel::from(new_user.insert(&app_state.conn).await.unwrap());
    Ok(new_user)
}

#[server(GetRoles, "/api")]
pub async fn get_roles() -> Result<Vec<RoleModel>, ServerFnError> {
    use actix_web::web;
    use crate::AppState;

    use entities::role;
    use entities::prelude::Role;

    let app_state = extract::<web::Data<AppState>>().await.unwrap();
    let roles: Vec<role::Model> = Role::find().all(&app_state.conn).await.unwrap_or(Vec::new());

    let mut roles_list = Vec::<RoleModel>::new();
    roles_list.extend(roles.into_iter().map(Into::into));
    Ok(roles_list)
}

#[server(UnsignRoleToUser)]
pub async fn unsign_role_from_user(role_id: i32, user_id: i32) -> Result<(), ServerFnError> {
    use actix_web::web;
    use actix_identity::Identity;
    use crate::AppState;

    use entities::user_roles;
    use entities::prelude::UserRoles;

    let app_state = extract::<web::Data<AppState>>().await.unwrap();
    let identity = extract::<Option<Identity>>().await.unwrap();
    if let Some(user) = identity {
        if user_have_permission(user.id().unwrap().parse::<i32>().unwrap(), "GerenciarCargos".to_owned()).await.unwrap() {
            if let Ok(Some(role_assignment)) = UserRoles::find()
                .filter(user_roles::Column::RoleId.eq(role_id))
                .filter(user_roles::Column::UserId.eq(user_id))
                .one(&app_state.conn)
                .await {
                    role_assignment.delete(&app_state.conn).await;
                    return Ok(())
                }
            else {
                return Err(ServerFnError::new("Banco de dados não encontrou essa relação."))
            }
        }
        
        return Err(ServerFnError::new("Usuário não tem permissão para realizar essa ação."))
    }

    Err(ServerFnError::new("Usuário não está conectado na sessão."))
}

#[server(AssignRoleToUser)]
pub async fn assign_role_to_user(role_id: i32, user_id: i32) -> Result<(), ServerFnError> {
    use actix_web::web;
    use actix_identity::Identity;
    use crate::AppState;

    use sea_orm::{Set, ActiveModelTrait};

    use entities::user_roles;

    let app_state = extract::<web::Data<AppState>>().await.unwrap();
    let identity = extract::<Option<Identity>>().await.unwrap();
    if let Some(user) = identity {
        if user_have_permission(user.id().unwrap().parse::<i32>().unwrap(), "GerenciarCargos".to_owned()).await.unwrap() {
            let new_role_assignment = user_roles::ActiveModel {
                user_id: Set(user_id),
                role_id: Set(role_id),
                ..Default::default()
            };
            new_role_assignment.insert(&app_state.conn).await;
            return Ok(())
        }
        
        return Err(ServerFnError::new("Usuário não tem permissão para realizar essa ação."))
    }

    Err(ServerFnError::new("Usuário não está conectado na sessão."))
}

#[server(IsUserLoggedIn, "/api")]
pub async fn is_user_logged_in() -> Result<bool, ServerFnError> {
    use actix_identity::Identity;

    let identity = extract::<Option<Identity>>().await.unwrap();
    if let Some(_user) = identity {
        return Ok(true)
    } else {
        return Ok(false)
    }
}

#[server(GetUserRoles, "/api")]
pub async fn get_user_roles(id: i32) -> Result<Vec<RoleModel>, ServerFnError> {
    use actix_web::web;
    use crate::AppState;

    use entities::{role, user_roles, role_permissions};
    use entities::prelude::{Role, UserRoles, RolePermissions, Permission};

    let state = extract::<web::Data<AppState>>().await.unwrap();

    let roles: Vec<user_roles::Model> = UserRoles::find()
        .filter(user_roles::Column::UserId.eq(id))
        .all(&state.conn)
        .await
        .unwrap_or(vec![]);

    let mut user_roles = vec![];

    for role in roles.iter() {
        let mut role = RoleModel::from(
            Role::find_by_id(role.role_id).one(&state.conn).await.unwrap().unwrap()
        );

        let mut permissions = vec![];

        for role_permission in RolePermissions::find()
            .filter(role_permissions::Column::RoleId.eq(role.id))
            .all(&state.conn)
            .await.unwrap_or(Vec::new()).iter() {
                if let Some(perm) = Permission::find_by_id(role_permission.permission_id).one(&state.conn).await.unwrap() {
                    permissions.push(PermissionModel::from(perm))
                }
        }
        role.children = permissions;
        user_roles.push(role)
    }

    Ok(user_roles)
}

#[server(ConfirmEmailChanges, "/api")]
pub async fn confirm_email_changes(code: String) -> Result<(), ServerFnError> {
    use actix_web::web;
    use actix_identity::Identity;
    use serde_json::json;

    use entities::{user, confirmation_request};
    use entities::prelude::{User, ConfirmationRequest};

    use lettre::message::{header::ContentType, MultiPart, SinglePart};
    use lettre::{Message, Address};

    use std::env;

    use crate::AppState;

    let state = extract::<web::Data<AppState>>().await.unwrap();
    let request = ConfirmationRequest::find()
        .filter(confirmation_request::Column::ConfirmCode.eq(code))
        .one(&state.conn)
        .await
        .unwrap();

    match request {
        Some(request) => {
            let now = Utc::now();

            let response = if request.expires_at < now {
                Err(ServerFnError::new("Essa requisição expirou."))
            } else {
                let user = request.find_related(User).one(&state.conn).await.unwrap().unwrap();
                let mut user: user::ActiveModel = user.into();
                user.updated_at = Set(chrono::Utc::now().with_timezone(&chrono::FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone")));
                user.is_confirmed = Set(true);
                user.update(&state.conn).await;
                Ok(())
            };

            let mut request: confirmation_request::ActiveModel = request.into();
            request.delete(&state.conn).await;
            response
        },

        None => {
            Err(ServerFnError::new("Essa requisição é inválida pois não existem registros."))
        }
    }
}

#[server(ChangeEmail, "/api")]
pub async fn change_email(new_email: String) -> Result<UserModel, ServerFnError> {
    use actix_web::web;
    use actix_identity::Identity;
    use serde_json::json;

    use entities::{user, confirmation_request};
    use entities::prelude::{User, ConfirmationRequest};

    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    use lettre::message::{header::ContentType, MultiPart, SinglePart};
    use lettre::{Message, Address};

    use std::env;

    use crate::AppState;

    let identity = extract::<Option<Identity>>().await.unwrap();
    let state = extract::<web::Data<AppState>>().await.unwrap();

    if let Some(user) = identity {
        println!("User logged in.");
        let mut user: user::ActiveModel = 
            User::find()
            .filter(
                user::Column::Id.eq(
                    user
                        .id()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap()
                )
            )
            .one(&state.conn)
            .await
            .unwrap()
            .unwrap()
            .into();
        
        let gen_confirmation_code = move |length| -> String {
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(length)
                .map(char::from)
                .collect()
        };

        println!("Generating confirm code...");
        let mut confirmation_code = gen_confirmation_code(24);

        let unique_code = loop {
            if let Some(request) = ConfirmationRequest::find().filter(confirmation_request::Column::ConfirmCode.eq(&confirmation_code)).one(&state.conn).await.unwrap() {
                confirmation_code = gen_confirmation_code(24);
            } else {
                break confirmation_code
            }
        };

        let confirm_url = format!("http://localhost:3000/perfil/confirmar-email?code={}", unique_code.clone());

        let confirmation_email = Message::builder()
            .from(env::var("SMTP_MAIL").unwrap().parse::<Address>().unwrap().into())
            .to(new_email.parse::<Address>().unwrap().into())
            .subject("Verificação do e-mail")
            .multipart(
                MultiPart::alternative_plain_html(
                    format!("Link para confirmar seu novo e-mail: {}", confirm_url),
                    state.template_engine.render("tpl-confirm_mail", &json!({"first_name": user.username.clone().unwrap(), "url":confirm_url})).unwrap()
                )
            )
            .unwrap();
        match state.mailer.send(confirmation_email).await {
            Ok(res) => {
                let request: confirmation_request::ActiveModel = confirmation_request::ActiveModel { 
                    user_id: user.id.clone(), 
                    confirm_code: Set(unique_code.clone()), 
                    expires_at: Set(chrono::Utc::now().with_timezone(&chrono::FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone")).checked_add_days(Days::new(1)).unwrap()),
                    ..Default::default()
                };
                user.email = Set(new_email);
                user.is_confirmed = Set(false);

                let request = request.insert(&state.conn).await.unwrap();
                let user = user.update(&state.conn).await.unwrap();
                Ok(user.into())
            },
            Err(e) => {
                Err(ServerFnError::new(format!("Houve um erro ao validar o e-mail: {e:?}")))
            }
        }
        
        
        
    } else {
        Err(ServerFnError::new("Nenhum usuário está conectado nessa sessão."))
    }
}

#[server(ChangeUsername, "/api")]
pub async fn change_username(new_name: String) -> Result<UserModel, ServerFnError> {
    use actix_web::web;
    use actix_identity::Identity;

    use entities::user;
    use entities::prelude::User;

    use crate::AppState;

    let identity = extract::<Option<Identity>>().await.unwrap();
    let state = extract::<web::Data<AppState>>().await.unwrap();

    if let Some(user) = identity {
        let mut user: user::ActiveModel = 
            User::find()
            .filter(
                user::Column::Id.eq(
                    user
                        .id()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap()
                )
            )
            .one(&state.conn)
            .await
            .unwrap()
            .unwrap()
            .into();
        
        user.username = Set(new_name);
        let user = user.update(&state.conn).await.unwrap();
        Ok(user.into())
    } else {
        Err(ServerFnError::new("Nenhum usuário está conectado nessa sessão."))
    }
}

#[server(GetLoggedUser, "/api")]
pub async fn get_logged_user() -> Result<UserModel, ServerFnError> {
    use actix_web::web;
    use actix_identity::Identity;

    use entities::user;
    use entities::prelude::User;

    use crate::AppState;

    let identity = extract::<Option<Identity>>().await.unwrap();
    let state = extract::<web::Data<AppState>>().await.unwrap();

    if let Some(user) = identity {
        let mut user = UserModel::from(
            User::find()
            .filter(
                user::Column::Id.eq(
                    user
                        .id()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap()
                )
            )
            .one(&state.conn)
            .await
            .unwrap()
            .unwrap()
        );
        user.password = "".to_owned();
        Ok(user)
    } else {
        Err(ServerFnError::new("Nenhum usuário está conectado nessa sessão."))
    }
}

#[server(LoginUser, "/api")]
pub async fn login_user(email: String, password: String) -> Result<(), ServerFnError> {
    use actix_web::{web, HttpRequest, HttpMessage};
    use actix_identity::Identity;
    use crate::AppState;

    use argon2::{
        password_hash::{
            PasswordHash, 
            PasswordVerifier
        },
        Argon2
    };

    use entities::user;
    use entities::prelude::User;

    let state = extract::<web::Data<AppState>>().await.unwrap();
    let request = extract::<HttpRequest>().await.unwrap();

    if let Ok(Some(logged_user)) = User::find()
        .filter(user::Column::Email.eq(email))
        .one(&state.conn)
        .await {
            let password_hashed = PasswordHash::new(&logged_user.password).unwrap();
            let secret = state.secret_key.clone().into_bytes();
            if let Ok(_) = Argon2::new_with_secret(
                    secret.as_slice(), 
                    argon2::Algorithm::default(), 
                    argon2::Version::default(), 
                    argon2::Params::default())
                .unwrap().verify_password(password.into_bytes().as_slice(), &password_hashed) {
                let _ = Identity::login(&request.extensions(), logged_user.id.to_string());
                leptos_actix::redirect("/perfil");
                return Ok(())
            }
            return Err(ServerFnError::new("Dados do usuário estão incorretos."))
        }

    Err(ServerFnError::new("Essa conta não existe!"))
}

#[server(GetMembershipDetails, "/api")]
pub async fn get_membership_details(id: i32) -> Result<Option<MembershipModel>, ServerFnError> {
    use actix_web::web;
    use actix_identity::Identity;
    use crate::AppState;

    use entities::membership;
    use entities::prelude::Membership;

    let state = extract::<web::Data<AppState>>().await.unwrap();

    let membership_details = Membership::find()
        .filter(membership::Column::UserId.eq(id))
        .one(&state.conn)
        .await
        .unwrap();

    match membership_details {
        Some(membership) => Ok(Some(MembershipModel::from(membership))),
        None => Ok(None)
    }
}

#[server(EditPassword, "/api")]
pub async fn edit_password(new_password: String) -> Result<(), ServerFnError> {
    use actix_web::web;
    use actix_identity::Identity;

    use entities::user;
    use entities::prelude::User;

    use crate::AppState;

    let identity = extract::<Option<Identity>>().await.unwrap();
    
    use argon2::{
        password_hash::{
            rand_core::OsRng,
            PasswordHasher, 
            SaltString
        },
        Argon2
    };

    let state = extract::<web::Data<AppState>>().await.unwrap();
    let secret = state.secret_key.clone().into_bytes();

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::new_with_secret(
        secret.as_slice(), 
        argon2::Algorithm::default(), 
        argon2::Version::default(), 
        argon2::Params::default())
    .unwrap();
   
    let password_hash = argon2.hash_password(new_password.into_bytes().as_slice(), &salt).unwrap().to_string();

    if let Some(user) = identity {
        let mut user: user::ActiveModel = 
            User::find()
            .filter(
                user::Column::Id.eq(
                    user
                        .id()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap()
                )
            )
            .one(&state.conn)
            .await
            .unwrap()
            .unwrap()
            .into();
        
        user.password = Set(password_hash);
        let _ = user.update(&state.conn).await.unwrap();
        Ok(())
    } else {
        Err(ServerFnError::new("Nenhum usuário está conectado nessa sessão."))
    }
}

#[server(EditJob, "/api")]
pub async fn edit_job(job_values: JobModel) -> Result<JobModel, ServerFnError> {
    use actix_web::web;
    use actix_identity::Identity;
    use crate::AppState;

    use sea_orm::{Set, ActiveModelTrait};

    use entities::job;
    use entities::prelude::Job;

    let state = extract::<web::Data<AppState>>().await.unwrap();
    let identity = extract::<Option<Identity>>().await.unwrap();
    if let Some(user) = identity {
        if user_have_permission(user.id().unwrap().parse::<i32>().unwrap(), "EditarVagas".to_owned()).await.unwrap() {
            let mut job: job::ActiveModel = 
                Job::find()
                .filter(
                    job::Column::Id.eq(
                        job_values.id
                    )
                )
                .one(&state.conn)
                .await
                .unwrap()
                .unwrap()
                .into();
            
            job.position = Set(job_values.position);
            job.company = Set(job_values.company);
            job.description = Set(job_values.description);
            job.requirements = Set(job_values.requirements);
            job.updated_at = Set(Utc::now().with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone")));
            let job = job.update(&state.conn).await.unwrap();
            return Ok(job.into())
        }
        
        return Err(ServerFnError::new("Usuário não tem permissão de acesso."))
    }

    Err(ServerFnError::new("Usuário não está conectado na sessão."))
}

#[server(DeleteJob, "/api")]
pub async fn delete_job(id: i32) -> Result<(), ServerFnError> {
    use actix_web::web;
    use actix_identity::Identity;
    use crate::AppState;

    use sea_orm::{Set, ActiveModelTrait};

    use entities::prelude::Job;
    use entities::job;

    let app_state = extract::<web::Data<AppState>>().await.unwrap();
    let identity = extract::<Option<Identity>>().await.unwrap();
    if let Some(user) = identity {
        if user_have_permission(user.id().unwrap().parse::<i32>().unwrap(), "EditarVagas".to_owned()).await.unwrap() {
            let deleted_job = Job::find().filter(job::Column::Id.eq(id)).one(&app_state.conn).await.unwrap().unwrap();
            let deleted_job = deleted_job.delete(&app_state.conn).await.unwrap_throw();
            return Ok(())
        }
        
        return Err(ServerFnError::new("Usuário não tem permissão de acesso."))
    }

    Err(ServerFnError::new("Usuário não está conectado na sessão."))
}

#[server(AddJob, "/api")]
pub async fn add_job(new_job: JobModel) -> Result<JobModel, ServerFnError> {
    use actix_web::web;
    use actix_identity::Identity;
    use crate::AppState;

    use sea_orm::{Set, ActiveModelTrait};

    use entities::job;

    let app_state = extract::<web::Data<AppState>>().await.unwrap();
    let identity = extract::<Option<Identity>>().await.unwrap();
    if let Some(user) = identity {
        if user_have_permission(user.id().unwrap().parse::<i32>().unwrap(), "EditarVagas".to_owned()).await.unwrap() {
            let new_job = job::ActiveModel {
                position: Set(new_job.position),
                company: Set(new_job.company),
                description: Set(new_job.description),
                requirements: Set(new_job.requirements),
                published_at: Set(Utc::now().with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone"))),
                updated_at: Set(Utc::now().with_timezone(&FixedOffset::west_opt(3 * 3600).expect("Invalid Timezone"))),
                ..Default::default()
            };
            let new_job = JobModel::from(new_job.insert(&app_state.conn).await.unwrap_throw());
            return Ok(new_job)
        }
        
        return Err(ServerFnError::new("Usuário não tem permissão de acesso."))
    }

    Err(ServerFnError::new("Usuário não está conectado na sessão."))
}