# Dimacs Petgraph Parser

Basic parser to convert graphs in [dimacs .col](http://lcs.ios.ac.cn/~caisw/Resource/about_DIMACS_graph_format.txt) format to [petgraphs](https://github.com/petgraph/petgraph).

# Usage / Example

Include this lib in your dependencies in your Cargo.toml: <br>

```toml
dimacs_petgraph_parser = { git = "https://github.com/RaoulLuque/dimacs-petgraph-parser" }
```
Use the read_graph() function. This will return a result with an error if any IO errors occur or the graph is in the wrong format:

```rust
  let test_graph_one_file =
      File::open("test_graphs/test_graph_one.col").expect("Should be able to read file");
  let test_graph_one: Graph<i32, i32, petgraph::prelude::Undirected> =
      read_graph(test_graph_one_file)
          .expect("Test Graph One should be readable/in correct format");
```
