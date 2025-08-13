use std::fs;
use std::path::Path;
use pathfinder::{Tree, NodeId};

/// Recursively build Tree<String> from a filesystem path
fn build_tree_from_path(tree: &mut Tree<String>, path: &Path, parent: Option<NodeId>) -> std::io::Result<NodeId> {
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string());

    let node_id = match parent {
        Some(p) => tree.add_child(p, name),
        None => tree.set_root(name),
    };

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let child_path = entry.path();
            build_tree_from_path(tree, &child_path, Some(node_id))?;
        }
    }

    Ok(node_id)
}

fn main() -> std::io::Result<()> {
    let mut tree = Tree::new();
    let path = std::env::current_dir()?; // Or any path you want
    build_tree_from_path(&mut tree, &path, None)?;

    // Print tree
    println!("{}", tree.fmt_tree(|s| s.clone()));

    Ok(())
}

