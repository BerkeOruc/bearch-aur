use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlpmError {
    #[error("ALPM not available on this platform")]
    NotAvailable,
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Package not found: {0}")]
    PackageNotFound(String),
}

pub async fn check_updates() -> Result<Vec<String>, AlpmError> {
    #[cfg(not(target_os = "windows"))]
    {
        Err(AlpmError::NotAvailable)
    }
    #[cfg(target_os = "windows")]
    {
        Err(AlpmError::NotAvailable)
    }
}

pub async fn install_package(_name: &str) -> Result<(), AlpmError> {
    #[cfg(not(target_os = "windows"))]
    {
        Err(AlpmError::NotAvailable)
    }
    #[cfg(target_os = "windows")]
    {
        Err(AlpmError::NotAvailable)
    }
}

pub async fn remove_package(_name: &str) -> Result<(), AlpmError> {
    #[cfg(not(target_os = "windows"))]
    {
        Err(AlpmError::NotAvailable)
    }
    #[cfg(target_os = "windows")]
    {
        Err(AlpmError::NotAvailable)
    }
}

pub async fn sync_db() -> Result<(), AlpmError> {
    #[cfg(not(target_os = "windows"))]
    {
        Err(AlpmError::NotAvailable)
    }
    #[cfg(target_os = "windows")]
    {
        Err(AlpmError::NotAvailable)
    }
}