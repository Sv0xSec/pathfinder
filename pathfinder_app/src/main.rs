use std::fs;
use std::path::{Path, PathBuf};
use dir::{Tree, NodeId};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to scan
    path: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();


    let mut tree = Tree::new();
    build_tree_from_path(&mut tree, &args.path, None)?;

    println!("\nTree structure:");
    println!("{}", tree.fmt_tree(|s| s.clone()));

    Ok(())
}

/// Recursively build Tree<String> from a filesystem path
fn build_tree_from_path(tree: &mut Tree<String>, path: &Path, parent: Option<NodeId>) -> std::io::Result<NodeId> {
    let name = path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string());

    // Create node
    let node_id = match parent {
        Some(p) => tree.add_child(p, name),
        None => tree.set_root(name),
    };

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let child_path = entry.path();
            // Recursively add children
            build_tree_from_path(tree, &child_path, Some(node_id))?;
        }
    }

    Ok(node_id)
}

