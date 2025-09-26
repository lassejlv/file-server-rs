use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
#[clap(name = "file-server", about = "A modern file server written in Rust")]
pub struct Config {
    #[clap(long, env = "DATABASE_URL", default_value = "sqlite:./files.db")]
    pub database_url: String,

    #[clap(long, env = "HOST", default_value = "0.0.0.0")]
    pub host: String,

    #[clap(long, env = "PORT", default_value = "3000")]
    pub port: u16,

    #[clap(long, env = "FILE_SERVER_MAX_FILE_SIZE", default_value = "52428800")]
    pub max_file_size: u64,

    #[clap(long, env = "FILE_SERVER_ALLOWED_FILE_TYPES", default_value = "*")]
    pub allowed_file_types: Option<String>,

    #[clap(long, env = "FILE_SERVER_STORAGE_TYPE", default_value = "local")]
    pub storage_type: StorageType,

    #[clap(long, env = "FILE_SERVER_STORAGE_PATH", default_value = "./files")]
    pub storage_path: PathBuf,

    #[clap(long, env = "FILE_SERVER_AUTH_TOKEN")]
    pub auth_token: Option<String>,

    #[clap(long, env = "FILE_SERVER_DISABLE_UPLOAD_PAGE")]
    pub disable_upload_page: bool,

    #[clap(long, env = "AWS_S3_BUCKET")]
    pub s3_bucket: Option<String>,

    #[clap(long, env = "AWS_S3_REGION")]
    pub s3_region: Option<String>,

    #[clap(long, env = "AWS_ACCESS_KEY_ID")]
    pub aws_access_key_id: Option<String>,

    #[clap(long, env = "AWS_SECRET_ACCESS_KEY")]
    pub aws_secret_access_key: Option<String>,

    #[clap(long, env = "AWS_ENDPOINT_URL")]
    pub aws_endpoint_url: Option<String>,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum StorageType {
    Local,
    S3,
}

impl std::fmt::Display for StorageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageType::Local => write!(f, "local"),
            StorageType::S3 => write!(f, "s3"),
        }
    }
}

impl Config {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if matches!(self.storage_type, StorageType::S3) {
            if self.s3_bucket.is_none() {
                anyhow::bail!("S3 bucket must be specified when using S3 storage");
            }
            if self.s3_region.is_none() {
                anyhow::bail!("S3 region must be specified when using S3 storage");
            }
        }

        if !self.storage_path.exists() && matches!(self.storage_type, StorageType::Local) {
            std::fs::create_dir_all(&self.storage_path)?;
        }

        Ok(())
    }

    pub fn allowed_file_types_vec(&self) -> Vec<String> {
        match &self.allowed_file_types {
            Some(types) => types.split(',').map(|s| s.trim().to_string()).collect(),
            None => vec!["*".to_string()],
        }
    }
}
