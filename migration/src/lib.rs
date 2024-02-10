pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240126_020426_create_users;
mod m20240126_115151_create_roles_permissions;
mod m20240127_012554_add_created_at_on_jobs;
mod m20240201_205352_seed_permissions;
mod m20240201_215115_seed_admin_role;
mod m20240206_122941_add_security_columns_to_user;
mod m20240206_131940_create_membership_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240126_020426_create_users::Migration),
            Box::new(m20240126_115151_create_roles_permissions::Migration),
            Box::new(m20240127_012554_add_created_at_on_jobs::Migration),
            Box::new(m20240201_205352_seed_permissions::Migration),
            Box::new(m20240201_215115_seed_admin_role::Migration),
            Box::new(m20240206_122941_add_security_columns_to_user::Migration),
            Box::new(m20240206_131940_create_membership_table::Migration),
        ]
    }
}
