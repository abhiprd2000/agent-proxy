use anyhow::{anyhow, Result};
use std::path::Path;
use tree_sitter::{Language, Node, Parser};

pub fn compress_code(raw_code: &str, filename: &str) -> Result<String> {
    let mut parser = Parser::new();
    
    // 1. Dynamic Grammar Routing
    let ext = Path::new(filename).extension().and_then(|e| e.to_str()).unwrap_or("");
    let language: Language = match ext {
        "rs" => tree_sitter_rust::LANGUAGE.into(),
        "py" => tree_sitter_python::LANGUAGE.into(),
        _ => return Err(anyhow!("Unsupported language extension: {}", ext)),
    };

    parser.set_language(&language)?;
    
    // 2. Parse the Tree
    let tree = parser.parse(raw_code, None).ok_or_else(|| anyhow!("Parse failed"))?;
    
    let mut compressed = String::new();
    let mut cursor = tree.walk();
    
    // 3. Recursive AST Pruning
    fn traverse(node: Node, source: &[u8], out: &mut String, cursor: &mut tree_sitter::TreeCursor) {
        let kind = node.kind();
        
        // Drop all standard comments across languages
        if kind.contains("comment") {
            return;
        }

        // If it's a leaf node, extract the raw text
        if node.child_count() == 0 {
            if let Ok(text) = std::str::from_utf8(&source[node.start_byte()..node.end_byte()]) {
                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    out.push_str(trimmed);
                    out.push(' ');
                }
            }
        } else {
            // Step down into children
            if cursor.goto_first_child() {
                loop {
                    traverse(cursor.node(), source, out, cursor);
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                cursor.goto_parent();
            }
        }
    }
    
    traverse(tree.root_node(), raw_code.as_bytes(), &mut compressed, &mut cursor);
    
    Ok(compressed)
}