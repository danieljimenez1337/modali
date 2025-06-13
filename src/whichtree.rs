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

pub fn search_which_tree<'a>(tree: &'a WhichTreeNode, s: &str) -> Option<&'a WhichTreeNode> {
    assert!(
        matches!(tree.kind, WhichTreeKind::Children(_)),
        "This function only takes head node"
    );
    assert!(tree.key.is_none(), "This function only takes head node");

    if s.is_empty() {
        return Some(tree);
    }

    let chars: Vec<char> = s.chars().collect();
    search_recursive(tree, &chars, 0)
}

fn search_recursive<'a>(
    node: &'a WhichTreeNode,
    chars: &[char],
    index: usize,
) -> Option<&'a WhichTreeNode> {
    if index >= chars.len() {
        return Some(node);
    }

    let current_char = chars[index];

    if let WhichTreeKind::Children(children) = &node.kind {
        if let Some(child) = children
            .iter()
            .find(|child| child.key == Some(current_char))
        {
            match &child.kind {
                WhichTreeKind::Children(_) => search_recursive(child, chars, index + 1),
                WhichTreeKind::Command(_) => {
                    if index == chars.len() - 1 {
                        Some(child)
                    } else {
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
