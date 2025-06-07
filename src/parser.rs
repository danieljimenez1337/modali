use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
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
                    key: parse_char(key),
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
                    key: parse_char(key),
                    label,
                    kind: WhichTreeKind::Command(command.clone()),
                }
            }
        }
    }
}

pub fn actions_to_tree(actions: &[Action]) -> Vec<WhichTreeNode> {
    actions.iter().map(|a| a.to_tree_node()).collect()
}

// pub fn search_which_tree(tree: &[WhichTreeNode], s: String) -> WhichTreeNode

fn parse_char(s: &str) -> char {
    if s.chars().count() != 1 {
        panic!("This key is not a single char")
    }
    s.chars().next().unwrap()
}

pub struct WhichTreeNode {
    key: char,
    label: String,
    kind: WhichTreeKind,
}

pub enum WhichTreeKind {
    Command(String),
    Children(Vec<WhichTreeNode>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_load_config_file() {
        let contents = fs::read_to_string("bindings.json").unwrap();
        let config: Vec<Action> = serde_json::from_str(&contents).unwrap();
        let json = serde_json::to_string_pretty(&config).unwrap();
        insta::assert_snapshot!(json)
    }
}
