use std::fs;
use std::path::Path;
use anyhow::Result;

pub fn generate_html_map() -> Result<String> {
let mut nodes = Vec::new();
let mut edges = Vec::new();
let mut id_counter = 1;

```
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
```

<html>
<head>
    <title>AgentProxy Code Map</title>

```
<script
    type="text/javascript"
    src="https://unpkg.com/vis-network/standalone/umd/vis-network.min.js">
</script>

<style>
    body {{
        background-color: #0d1117;
        color: #c9d1d9;
        font-family: sans-serif;
        margin: 0;
        overflow: hidden;
    }}

    #mynetwork {{
        width: 100vw;
        height: 100vh;
        border: none;
    }}

    #title {{
        position: absolute;
        top: 10px;
        left: 20px;
        z-index: 10;
        font-weight: bold;
    }}
</style>
```

</head>

<body>
    <div id="title">AgentProxy Code Map</div>

```
<div id="mynetwork"></div>

<script type="text/javascript">
    var nodes = new vis.DataSet({0});

    var edges = new vis.DataSet({1});

    var container = document.getElementById('mynetwork');

    var data = {{
        nodes: nodes,
        edges: edges
    }};

    var options = {{
        physics: {{
            stabilization: true,
            barnesHut: {{
                springLength: 200
            }}
        }}
    }};

    var network = new vis.Network(
        container,
        data,
        options
    );
</script>
```

</body>
</html>"#,
        nodes_json,
        edges_json
    );

```
fs::write("agentproxy-map.html", html_content)?;

Ok("agentproxy-map.html".to_string())
```

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

```
let current_id = *id_counter;
*id_counter += 1;

let label = path
    .file_name()
    .unwrap_or_else(|| std::ffi::OsStr::new("root"))
    .to_string_lossy();

if label.starts_with('.')
    || label == "target"
    || label == "node_modules"
{
    return Ok(());
}

let color = if path.is_dir() {
    "#8957e5"
} else {
    "#238636"
};

let shape = if path.is_dir() {
    "ellipse"
} else {
    "box"
};

nodes.push(format!(
    "{{id: {}, label: '{}', shape: '{}', color: '{}', font: {{color: 'white'}}}}",
    current_id,
    label,
    shape,
    color
));

if parent_id != 0 {
    edges.push(format!(
        "{{from: {}, to: {}, arrows: 'to', color: '#58a6ff'}}",
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
```

}
