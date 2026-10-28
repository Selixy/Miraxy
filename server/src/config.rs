//! Configuration du serveur, lue dans les variables d'environnement.

use std::{net::SocketAddr, path::PathBuf};

const DEFAULT_BIND: &str = "127.0.0.1:7001";
const DEFAULT_UI_DIR: &str = "ui";

pub struct Config {
    pub bind: SocketAddr,
    pub ui_dir: PathBuf,
    /// Absente en développement local : le serveur démarre sans base.
    pub database_url: Option<String>,
}

impl Config {
    /// `MIRAXY_BIND` : adresse d'écoute (défaut `127.0.0.1:7001`).
    /// `MIRAXY_UI_DIR` : dossier des fichiers de l'ui (défaut `ui`).
    /// `MIRAXY_DATABASE_URL` : URL de connexion PostgreSQL (optionnelle).
    pub fn from_env() -> Result<Self, String> {
        let raw = std::env::var("MIRAXY_BIND").unwrap_or_else(|_| DEFAULT_BIND.to_string());
        let bind = raw
            .parse()
            .map_err(|e| format!("MIRAXY_BIND invalide ({raw}) : {e}"))?;
        let database_url = std::env::var("MIRAXY_DATABASE_URL")
            .ok()
            .filter(|url| !url.is_empty());
        let ui_dir = std::env::var("MIRAXY_UI_DIR")
            .unwrap_or_else(|_| DEFAULT_UI_DIR.to_string())
            .into();
        Ok(Self {
            bind,
            ui_dir,
            database_url,
        })
    }
}
