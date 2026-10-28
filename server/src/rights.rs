//! Catalogue des droits. Un droit est un booléen, accordé par département.
//! Un compte administrateur (`is_admin`) les a tous, dans tous les départements.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Right {
    /// Publier des tâches dans le département.
    PublishTasks,
    /// Publier le résultat des tâches effectuées dans le département.
    PublishResults,
    /// Accorder ou retirer des droits aux membres du département.
    ManageMembers,
}

impl Right {
    pub const ALL: [Right; 3] = [
        Right::PublishTasks,
        Right::PublishResults,
        Right::ManageMembers,
    ];

    /// Nom stocké en base (`account_rights.right_name`).
    pub fn as_str(self) -> &'static str {
        match self {
            Right::PublishTasks => "publish_tasks",
            Right::PublishResults => "publish_results",
            Right::ManageMembers => "manage_members",
        }
    }

    pub fn parse(name: &str) -> Option<Right> {
        Right::ALL.into_iter().find(|right| right.as_str() == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_round_trip() {
        for right in Right::ALL {
            assert_eq!(Right::parse(right.as_str()), Some(right));
        }
        assert_eq!(Right::parse("nope"), None);
    }

    #[test]
    fn names_match_serde() {
        for right in Right::ALL {
            let json = serde_json::to_string(&right).unwrap();
            assert_eq!(json, format!("\"{}\"", right.as_str()));
        }
    }
}
