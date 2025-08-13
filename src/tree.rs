use std::collections::VecDeque;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Stable handle to a node inside the tree arena.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NodeId(pub usize);

/// Internal node representation.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct Node<T> {
    data: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

/// A safe, generic rooted tree.
#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tree<T> {
    nodes: Vec<Option<Node<T>>>,
    root: Option<NodeId>,
}

impl<T> Tree<T> {
    /// Create an empty tree.
    pub fn new() -> Self {
        Self { nodes: Vec::new(), root: None }
    }

    /// Create root node.
    pub fn set_root(&mut self, data: T) -> NodeId {
        assert!(self.root.is_none(), "root already exists");
        let id = self.alloc(Node { data, parent: None, children: vec![] });
        self.root = Some(id);
        id
    }

    /// Add a child to a parent.
    pub fn add_child(&mut self, parent: NodeId, data: T) -> NodeId {
        self.assert_exists(parent);
        let child = self.alloc(Node { data, parent: Some(parent), children: vec![] });
        self.node_mut(parent).children.push(child);
        child
    }

    /// Get immutable reference to node data.
    pub fn get(&self, id: NodeId) -> &T {
        &self.node(id).data
    }

    /// Get mutable reference to node data.
    pub fn get_mut(&mut self, id: NodeId) -> &mut T {
        &mut self.node_mut(id).data
    }

    /// Get parent of a node.
    pub fn parent(&self, id: NodeId) -> Option<NodeId> {
        self.node(id).parent
    }

    /// Get children of a node.
    pub fn children(&self, id: NodeId) -> impl Iterator<Item = NodeId> + '_ {
        self.node(id).children.iter().copied()
    }

    /// Depth-first search from root.
    pub fn dfs(&self) -> Vec<NodeId> {
        let mut result = Vec::new();
        if let Some(root) = self.root {
            self.dfs_rec(root, &mut result);
        }
        result
    }

    /// Breadth-first search from root.
    pub fn bfs(&self) -> Vec<NodeId> {
        let mut result = Vec::new();
        if let Some(root) = self.root {
            let mut queue = VecDeque::new();
            queue.push_back(root);
            while let Some(id) = queue.pop_front() {
                result.push(id);
                for child in self.node(id).children.iter().copied() {
                    queue.push_back(child);
                }
            }
        }
        result
    }

    /// Pretty print tree like `tree` command.
    pub fn fmt_tree<F>(&self, mut label: F) -> String
    where
        F: FnMut(&T) -> String,
    {
        let mut out = String::new();
        if let Some(root) = self.root {
            self.fmt_rec(root, "", true, &mut out, &mut label);
        }
        out
    }

    // ===== Internals =====

    fn alloc(&mut self, node: Node<T>) -> NodeId {
        let id = NodeId(self.nodes.len());
        self.nodes.push(Some(node));
        id
    }

    fn node(&self, id: NodeId) -> &Node<T> {
        self.nodes[id.0].as_ref().expect("invalid NodeId")
    }

    fn node_mut(&mut self, id: NodeId) -> &mut Node<T> {
        self.nodes[id.0].as_mut().expect("invalid NodeId")
    }

    fn assert_exists(&self, id: NodeId) {
        assert!(id.0 < self.nodes.len() && self.nodes[id.0].is_some(), "invalid NodeId");
    }

    fn dfs_rec(&self, id: NodeId, out: &mut Vec<NodeId>) {
        out.push(id);
        for &child in &self.node(id).children {
            self.dfs_rec(child, out);
        }
    }

    fn fmt_rec<F>(&self, id: NodeId, prefix: &str, last: bool, out: &mut String, label: &mut F)
    where
        F: FnMut(&T) -> String,
    {
        let connector = if prefix.is_empty() { "" }
                        else if last { "└── " } else { "├── " };
        out.push_str(prefix);
        out.push_str(connector);
        out.push_str(&label(&self.node(id).data));
        out.push('\n');

        let new_prefix = if prefix.is_empty() {
            String::new()
        } else if last {
            format!("{prefix}    ")
        } else {
            format!("{prefix}│   ")
        };

        let ch = &self.node(id).children;
        for (i, &c) in ch.iter().enumerate() {
            let is_last = i + 1 == ch.len();
            self.fmt_rec(c, &new_prefix, is_last, out, label);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_and_print() {
        let mut t = Tree::new();
        let root = t.set_root("root");
        let a = t.add_child(root, "a");
        t.add_child(a, "a1");
        t.add_child(a, "a2");
        let b = t.add_child(root, "b");
        t.add_child(b, "b1");

        let dfs_labels: Vec<_> = t.dfs().into_iter().map(|id| t.get(id)).cloned().collect();
        assert_eq!(dfs_labels, vec!["root", "a", "a1", "a2", "b", "b1"]);

        println!("{}", t.fmt_tree(|s| s.to_string()));
    }
}

