use petgraph::Graph;
use std::io::{BufRead, BufReader, Read};

pub fn read_graph<N: Clone + Default, E: Clone + Default, R: Read>(
    file: R,
) -> Result<Graph<N, E, petgraph::prelude::Undirected>, Box<dyn std::error::Error>> {
    let mut graph: Graph<N, E, petgraph::prelude::Undirected> = Graph::new_undirected();

    let lines: Result<Vec<_>, _> = BufReader::new(file).lines().collect();

    let lines = lines?;

    // Remove comment lines
    let mut lines = lines
        .iter()
        .filter(|l| l.len() > 1 && l.chars().next() != Some('c'));

    // Get header line
    let header_line = lines.next().unwrap();
    let header: Vec<&str> = header_line.split_whitespace().collect();

    let number_of_vertices: usize = header
        .get(2)
        .expect("Problem line at header is not of the correct format: p FORMAT #NODES #EDGES")
        .parse()?;

    let number_of_edges: usize = header
        .get(3)
        .expect("Problem line at header is not of the correct format: p FORMAT #NODES #EDGES")
        .parse()?;

    let nodes: Vec<_> = (0..number_of_vertices)
        .map(|_| graph.add_node(N::default()))
        .collect();

    for line in lines {
        let edge: Vec<&str> = line.split_whitespace().collect();
        let (vertex_one, vertex_two) = (
            edge.get(1)
                .expect(&format!(
                    "Edge line is not of the correct format. See: {:?}",
                    line
                ))
                .parse::<usize>()?,
            edge.get(2)
                .expect(&format!(
                    "Edge line is not of the correct format. See: {:?}",
                    line
                ))
                .parse::<usize>()?,
        );
        graph.add_edge(
            *nodes
                .get(vertex_one - 1)
                .expect(&format!("Node number in edge is out of bounds: {:?}", line)),
            *nodes
                .get(vertex_two - 1)
                .expect(&format!("Node number in edge is out of bounds: {:?}", line)),
            E::default(),
        );
    }

    debug_assert_eq!(graph.edge_count(), number_of_edges);
    Ok(graph)
}
