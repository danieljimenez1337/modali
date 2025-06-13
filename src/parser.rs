use crate::whichtree::{WhichTreeKind, WhichTreeNode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Action {
    SubAction {
        key: String,
        description: String,
        sub_actions: Vec<Action>,
    },

    KeyAction {
        key: String,
        description: String,
        command: String,
    },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TypedAction {
    SubAction {
        key: String,
        description: String,
        sub_actions: Vec<TypedAction>,
    },

    KeyAction {
        key: String,
        description: String,
        command: String,
    },
}

impl Action {
    pub fn to_tree_node(&self) -> WhichTreeNode {
        match self {
            Action::SubAction {
                key,
                description,
                sub_actions,
            } => {
                let label = format!("{key}: {description}");
                let children = sub_actions
                    .iter()
                    .map(|action| action.to_tree_node())
                    .collect();
                WhichTreeNode {
                    key: Some(parse_char(key)),
                    label,
                    kind: WhichTreeKind::Children(children),
                }
            }
            Action::KeyAction {
                key,
                description,
                command,
            } => {
                let label = format!("{key}: {description}");
                WhichTreeNode {
                    key: Some(parse_char(key)),
                    label,
                    kind: WhichTreeKind::Command(command.clone()),
                }
            }
        }
    }
}

impl From<TypedAction> for Action {
    fn from(typed: TypedAction) -> Self {
        match typed {
            TypedAction::SubAction {
                key,
                description,
                sub_actions,
            } => Action::SubAction {
                key,
                description,
                sub_actions: sub_actions.into_iter().map(Into::into).collect(),
            },
            TypedAction::KeyAction {
                key,
                description,
                command,
            } => Action::KeyAction {
                key,
                description,
                command,
            },
        }
    }
}

pub fn actions_to_tree(actions: &[Action]) -> WhichTreeNode {
    let children = actions.iter().map(|a| a.to_tree_node()).collect();
    WhichTreeNode {
        key: None,
        label: "".to_string(),
        kind: WhichTreeKind::Children(children),
    }
}

fn parse_char(s: &str) -> char {
    if s.chars().count() != 1 {
        panic!("This key is not a single char")
    }
    s.chars().next().unwrap()
}

#[cfg(test)]
mod tests {
    use ron::ser::PrettyConfig;

    use super::*;
    use std::fs;

    #[test]
    fn test_load_config_file() {
        let contents = fs::read_to_string("bindings.json").unwrap();
        let config: Vec<TypedAction> = serde_json::from_str(&contents).unwrap();
        let json = serde_json::to_string_pretty(&config).unwrap();
        insta::assert_snapshot!(json)
    }

    #[test]
    fn test_load_ron_binding_file() {
        let contents = fs::read_to_string("bindings.ron").unwrap();
        let config: Vec<Action> = ron::from_str(&contents).unwrap();
        let ron = ron::ser::to_string_pretty(&config, PrettyConfig::default()).unwrap();
        insta::assert_snapshot!(ron)
    }
}
