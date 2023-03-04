use core::fmt;

#[derive(Clone, Debug, serde::Deserialize, PartialEq, Eq)]
pub enum ENV {
    Development,
    Production,
}

/// This function ensures that ENV enum can be serialized
///
/// # Example
/// ```
/// use zero_to_prod::common::configuration::env::ENV;
///
/// let env = ENV::Development;
/// let env_string = format!("{}", env);
/// assert_eq!(env_string, "Development");
///
/// let env = ENV::Production;
/// let env_string = format!("{}", env);
/// assert_eq!(env_string, "Production");
/// ```
impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::Development => write!(f, "Development"),
            ENV::Production => write!(f, "Production"),
        }
    }
}

/// This function ensures that an env string can be deserialized into an ENV enum
///
/// # Example
/// ```
/// use zero_to_prod::common::configuration::env::ENV;
///
/// let env_string = "Development";
/// assert_eq!(ENV::from(env_string), ENV::Development);
///
/// ```
impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Development" => ENV::Development,
            "Production" => ENV::Production,
            _ => ENV::Development,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, PartialEq, Eq)]
pub enum Env {
    Env,
    ServerPort,
    ServerHost,
    DatabaseUrl,
    DbPort,
    DbHost,
    DbUser,
    DbPassword,
    DbName,
}

/// This function ensures that ENV enum can be serialized
///
/// # Example
/// ```
/// use zero_to_prod::common::configuration::env::Env;
/// use zero_to_prod::common::configuration::env::ENV;
///
/// let env = Env::Env;
/// let env_string = format!("{}", env);
/// assert_eq!(env_string, "ENV");
///
///
/// ```
///
impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Env::Env => write!(f, "ENV"),
            Env::ServerPort => write!(f, "SERVER_PORT"),
            Env::ServerHost => write!(f, "SERVER_HOST"),
            Env::DatabaseUrl => write!(f, "DATABASE_URL"),
            Env::DbPort => write!(f, "DB_PORT"),
            Env::DbHost => write!(f, "DB_HOST"),
            Env::DbUser => write!(f, "DB_USER"),
            Env::DbPassword => write!(f, "DB_PASSWORD"),
            Env::DbName => write!(f, "DB_NAME"),
        }
    }
}
