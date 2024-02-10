use sea_orm_migration::prelude::*;

use crate::m20240126_115151_create_roles_permissions::Permission;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(Permission::Table)
            .columns([Permission::Name, Permission::Description])
            .values_panic([
                "EditarVagas".into(),
                "Essa permissão concede ao usuário a habilidade de adicionar, editar e apagar vagas publicadas na seção de vagas do site.".into()])
            .values_panic([
                "GerenciarCargos".into(),
                "Essa permissão concede ao usuário a habilidade de adicionar, editar e apagar cargos de acesso dos usuários, além de conceder um cargo aos usuários que acessam o site.".into()])
            .values_panic([
                "ModerarUsuários".into(),
                "Essa permissão concede ao usuário o gerenciamento de inscrições de usuários, capacitando o usuário a remover e alterar o acesso de um usuário".into()])
            .values_panic([
                "EditarUsuários".into(),
                "Essa permissão concede ao usuário a habilidade de adicionar, editar e apagar usuários.".into()])
            .values_panic([
                "EditarArtigosPessoais".into(),
                "Essa permissão concede ao usuário a habilidade de escrever, editar e apagar artigos de sua própria autoria.".into()])
            .values_panic([
                "ModerarArtigos".into(),
                "Essa permissão concede ao usuário o poder de editar e apagar artigos escritos por outros usuários.".into()])
            .values_panic([
                "EditarDestaques".into(),
                "Essa permissão concede ao usuário a capacidade de editar a página inicial do site e alterar os artigos e outros itens em destaque.".into()])
            .values_panic([
                "ConfigurarSite".into(),
                "Essa permissão concede ao usuário a responsabilidade de editar configurações relacionadas a funções internas do website.".into()])
            .values_panic([
                "EditarAprovados".into(),
                "Essa permissão concede ao usuário a habilidade de adicionar, editar e apagar usuários que foram aprovados em vagas.".into()])    
            .to_owned();
        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete = Query::delete()
            .from_table(Permission::Table)
            .cond_where(Expr::col(Permission::Name).eq("EditarVagas"))
            .and_where(Expr::col(Permission::Name).eq("GerenciarCargos"))
            .and_where(Expr::col(Permission::Name).eq("ModerarUsuários"))
            .and_where(Expr::col(Permission::Name).eq("EditarUsuários"))
            .and_where(Expr::col(Permission::Name).eq("EditarArtigosPessoais"))
            .and_where(Expr::col(Permission::Name).eq("ModerarArtigos"))
            .and_where(Expr::col(Permission::Name).eq("EditarDestaques"))
            .and_where(Expr::col(Permission::Name).eq("ConfigurarSite"))
            .and_where(Expr::col(Permission::Name).eq("EditarAprovados"))
            .to_owned();

        manager.exec_stmt(delete).await?;

        Ok(())
    }
}
