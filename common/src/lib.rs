//! Protocole partagé entre le serveur et le daemon : une seule définition des types échangés.

/// Version du protocole, à incrémenter à chaque changement incompatible.
pub const PROTOCOL_VERSION: u32 = 1;
