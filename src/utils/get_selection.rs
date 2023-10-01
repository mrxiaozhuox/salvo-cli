use anyhow::Result;
use rust_i18n::t;
use dialoguer::{theme::ColorfulTheme, Select, console::Style};

#[derive(Debug)]
pub struct UserSelected {
    pub template_type: TemplateType,
    pub db_type: DbType,
    pub db_conn_type: DbConnectionType,
}


pub fn get_user_selected() -> Result<Option<UserSelected>> {
    let theme = ColorfulTheme {
        defaults_style: Style::new().blue(),
        prompt_style: Style::new().green().bold(),
        active_item_style: Style::new().blue().bold(),
        values_style: Style::new().blue().dim(),
        ..ColorfulTheme::default()
    };
    let selections = &[
        t!("salvo_web_api"),
        t!("salvo_web_site"),
        // "custom",
    ];
    let selection = Select::with_theme(&theme)
        .with_prompt(t!("welcome_message").replace(r"\n", "\n"))
        .default(0)
        .items(&selections[..])
        .interact()?;
    let template_type = match selection {
        0 => TemplateType::SalvoWebApi,
        1 => TemplateType::SalvoWebSite,
        _ => anyhow::bail!("Invalid selection"),
    };
    let db_conn_types = &[
        t!("db_conn_types_sqlx"),
        t!("db_conn_types_diesel"),
        t!("db_conn_types_sea_orm"),
        t!("db_conn_types_rbatis"),
        t!("db_conn_types_nothing"),
        // "custom",
    ];
    let db_conn_type_selection = Select::with_theme(&theme)
        .with_prompt(t!("select_db_conn_type").replace(r"\n", "\n"))
        .default(0)
        .items(&db_conn_types[..])
        .interact()?;

        let db_conn_type = match db_conn_type_selection {
            0 => DbConnectionType::Sqlx,
            1 => DbConnectionType::Diesel,
            2 => DbConnectionType::SeaOrm,
            3 => DbConnectionType::Rbatis,
            4 => DbConnectionType::Nothing,
            _ => anyhow::bail!("Invalid db connection type selection"),
        };


    let db_types = &[
        "sqlite",
        "mysql",
        "postgres",
        // "custom",
    ];
    let db_type_selection = Select::with_theme(&theme)
        .with_prompt(t!("select_db_type").replace(r"\n", "\n"))
        .default(0)
        .items(&db_types[..])
        .interact()?;
    let db_type = match db_type_selection {
        0 => DbType::Sqlite,
        1 => DbType::Mysql,
        2 => DbType::Postgres,
        _ => anyhow::bail!("Invalid db type selection"),
    };

        Ok(Some(UserSelected { template_type, db_type, db_conn_type }))
}
#[derive(Debug, PartialEq)]
pub enum TemplateType {
    SalvoWebSite,
    SalvoWebApi,
}

#[derive(Debug, PartialEq)]
pub enum  DbType {
    Sqlite,
    Mysql,
    Postgres,
}

#[derive(Debug,PartialEq)]
pub enum  DbConnectionType {
    Sqlx,
    Diesel,
    SeaOrm,
    Rbatis,
    Nothing,
}