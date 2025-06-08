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

#[derive(Clone)]
pub struct WhichTreeNode {
    pub key: Option<char>,
    pub label: String,
    pub kind: WhichTreeKind,
}

#[derive(Clone)]
pub enum WhichTreeKind {
    Command(String),
    Children(Vec<WhichTreeNode>),
}

pub fn search_which_tree(tree: &WhichTreeNode, s: &String) -> Option<WhichTreeNode> {
    assert!(
        matches!(tree.kind, WhichTreeKind::Children(_)),
        "This function only takes head node"
    );
    assert!(tree.key.is_none(), "This function only takes head node");

    if s.is_empty() {
        return Some(tree.clone());
    }

    let chars: Vec<char> = s.chars().collect();
    search_recursive(tree, &chars, 0)
}

fn search_recursive(node: &WhichTreeNode, chars: &[char], index: usize) -> Option<WhichTreeNode> {
    if index >= chars.len() {
        return Some(node.clone());
    }

    let current_char = chars[index];

    // Get children from current node
    if let WhichTreeKind::Children(children) = &node.kind {
        if let Some(child) = children
            .iter()
            .find(|child| child.key == Some(current_char))
        {
            match &child.kind {
                WhichTreeKind::Children(_) => search_recursive(child, chars, index + 1),
                WhichTreeKind::Command(_) => {
                    if index == chars.len() - 1 {
                        Some(child.clone())
                    } else {
                        // More characters left, but no children to continue
                        None
                    }
                }
            }
        } else {
            None
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
