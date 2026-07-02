use std::fs;
use std::path::Path;
use anyhow::Result;

pub fn generate_html_map() -> Result<String> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut id_counter = 1;

    traverse_dir(
        Path::new("."),
        &mut nodes,
        &mut edges,
        &mut id_counter,
        0,
    )?;

    let nodes_json = format!("[{}]", nodes.join(","));
    let edges_json = format!("[{}]", edges.join(","));

    let html_content = format!(
        r#"<!DOCTYPE html>
<html>
<head>
<title>AgentProxy Code Map</title>
<script src="https://unpkg.com/vis-network/standalone/umd/vis-network.min.js"></script>
<style>
body {{
    background:#0d1117;
    color:white;
    margin:0;
}}

#mynetwork {{
    width:100vw;
    height:100vh;
}}
</style>
</head>

<body>
<div id="mynetwork"></div>

<script>
var nodes = new vis.DataSet({0});
var edges = new vis.DataSet({1});

var container = document.getElementById("mynetwork");

var data = {{
    nodes: nodes,
    edges: edges
}};

var network = new vis.Network(
    container,
    data,
    {{}}
);
</script>

</body>
</html>"#,
        nodes_json,
        edges_json
    );

    fs::write("agentproxy-map.html", html_content)?;

    Ok("agentproxy-map.html".to_string())
}

fn traverse_dir(
    path: &Path,
    nodes: &mut Vec<String>,
    edges: &mut Vec<String>,
    id_counter: &mut i32,
    parent_id: i32,
) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    let current_id = *id_counter;
    *id_counter += 1;

    let label = path
        .file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new("root"))
        .to_string_lossy();

    nodes.push(format!(
        "{{id:{},label:'{}'}}",
        current_id,
        label
    ));

    if parent_id != 0 {
        edges.push(format!(
            "{{from:{},to:{}}}",
            parent_id,
            current_id
        ));
    }

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;

            traverse_dir(
                &entry.path(),
                nodes,
                edges,
                id_counter,
                current_id,
            )?;
        }
    }

    Ok(())
}
