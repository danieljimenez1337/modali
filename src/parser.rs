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

fn parse_char(s: &str) -> char {
    if s.chars().count() != 1 {
        panic!("This key is not a single char")
    }
    s.chars().next().unwrap()
}

#[derive(Clone)]
pub struct WhichTreeNode {
    pub key: char,
    pub label: String,
    pub kind: WhichTreeKind,
}

#[derive(Clone)]
pub enum WhichTreeKind {
    Command(String),
    Children(Vec<WhichTreeNode>),
}

pub fn search_which_tree(tree: &[WhichTreeNode], s: String) -> Option<WhichTreeNode> {
    let chars: Vec<char> = s.chars().collect();
    search_recursive(tree, &chars, 0)
}

fn search_recursive(
    nodes: &[WhichTreeNode],
    chars: &[char],
    index: usize,
) -> Option<WhichTreeNode> {
    if index >= chars.len() {
        return nodes
            .iter()
            .find(|node| matches!(node.kind, WhichTreeKind::Command(_)))
            .cloned();
    }

    let current_char = chars[index];

    if let Some(node) = nodes.iter().find(|node| node.key == current_char) {
        match &node.kind {
            WhichTreeKind::Command(_) => {
                if index == chars.len() - 1 {
                    Some(node.clone())
                } else {
                    None
                }
            }
            WhichTreeKind::Children(children) => search_recursive(children, chars, index + 1),
        }
    } else {
        None
    }
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
