use petgraph::Graph;
use std::io::{BufRead, BufReader, Read};

/// Basic parser to convert graphs in [dimacs .col](http://lcs.ios.ac.cn/~caisw/Resource/about_DIMACS_graph_format.txt)
/// format to [petgraphs](https://github.com/petgraph/petgraph) with optional treewidth information.
///
/// The returned result contains:
/// (graph, treewidth (if contained in file), lower bound on treewidth (if contained in file), upper bound on treewidth (if contained in file)) (if no error occurred)
pub fn read_graph<N: Clone + Default, E: Clone + Default, R: Read>(
    file: R,
) -> Result<
    (
        Graph<N, E, petgraph::prelude::Undirected>,
        Option<usize>,
        Option<usize>,
        Option<usize>,
    ),
    Box<dyn std::error::Error>,
> {
    let mut graph: Graph<N, E, petgraph::prelude::Undirected> = Graph::new_undirected();

    let lines: Result<Vec<_>, _> = BufReader::new(file).lines().collect();

    let lines = lines?;

    // Remove comment lines
    let mut lines = lines
        .iter()
        .filter(|l| l.len() > 1 && l.chars().next() != Some('c'));

    // Check if there is information about the treewidth
    let mut treewidth = None;
    let mut lower_bound = None;
    let mut upper_bound = None;
    let mut header_line_option = None;

    while header_line_option == None {
        let mut current_line = lines
            .next()
            .expect("File should contain more than just comments");
        match current_line.chars().next() {
            Some('t') => {
                let treewidth_line: Vec<&str> = current_line.split_whitespace().collect();
                treewidth = Some(
                    treewidth_line
                        .get(1)
                        .expect("Treewidth line at header is not of correct format: t TREEWIDTH")
                        .parse::<usize>()
                        .expect("Treewidth line at header is not of correct format: t TREEWIDTH"),
                );
            }
            Some('l') => {
                let lower_bound_line: Vec<&str> = current_line.split_whitespace().collect();
                lower_bound = Some(
                    lower_bound_line
                        .get(1)
                        .expect("Treewidth line at header is not of correct format: t TREEWIDTH")
                        .parse::<usize>()
                        .expect("Treewidth line at header is not of correct format: t TREEWIDTH"),
                );
            }
            Some('u') => {
                let upper_bound_line: Vec<&str> = current_line.split_whitespace().collect();
                upper_bound = Some(
                    upper_bound_line
                        .get(1)
                        .expect("Treewidth line at header is not of correct format: t TREEWIDTH")
                        .parse::<usize>()
                        .expect("Treewidth line at header is not of correct format: t TREEWIDTH"),
                );
            }
            a => {
                assert_eq!(a.unwrap(), 'p', "File header is not of the correct format (one p(roperties) line after c(omments), t(treewidth), u(pper bound), l(ower bound) lines)");
                header_line_option = Some(current_line);
            }
        }
    }

    let header: Vec<&str> = header_line_option.unwrap().split_whitespace().collect();

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
    Ok((graph, treewidth, lower_bound, upper_bound))
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::fs::File;

    #[test]
    fn test_read_graph_test_graph_one() {
        let test_graph_one_file =
            File::open("test_graphs/test_graph_one.col").expect("Should be able to read file");
        let (test_graph_one, treewidth, lower_bound, upper_bound): (
            Graph<i32, i32, petgraph::prelude::Undirected>,
            Option<usize>,
            Option<usize>,
            Option<usize>,
        ) = read_graph(test_graph_one_file)
            .expect("Test Graph One should be readable/in correct format");

        assert_eq!(test_graph_one.edge_count(), 45);
        assert_eq!(test_graph_one.node_count(), 10);
        assert_eq!(treewidth, None);
        assert_eq!(lower_bound, None);
        assert_eq!(upper_bound, Some(1000));
        for mut vertices in test_graph_one.node_indices().combinations(2) {
            let vertex_one = vertices.pop().expect("Graph should have vertices");
            let vertex_two = vertices.pop().expect("Graph should have vertices");
            assert!(test_graph_one.contains_edge(vertex_one, vertex_two));
        }
    }

    #[test]
    fn test_read_graph_test_graph_two() {
        let test_graph_two_file =
            File::open("test_graphs/test_graph_two.col").expect("Should be able to read file");
        let (test_graph_two, treewidth, lower_bound, upper_bound): (
            Graph<i32, i32, petgraph::prelude::Undirected>,
            Option<usize>,
            Option<usize>,
            Option<usize>,
        ) = read_graph(test_graph_two_file)
            .expect("Test Graph One should be readable/in correct format");

        assert_eq!(test_graph_two.edge_count(), 4);
        assert_eq!(test_graph_two.node_count(), 4);
        assert_eq!(treewidth, Some(2));
        assert_eq!(lower_bound, Some(2));
        assert_eq!(upper_bound, None);
        for mut vertices in test_graph_two.node_indices().combinations(2) {
            let vertex_one = vertices.pop().expect("Graph should have vertices");
            let vertex_two = vertices.pop().expect("Graph should have vertices");
            if vertex_one.index() != 3 && vertex_two.index() != 3 {
                assert!(test_graph_two.contains_edge(vertex_one, vertex_two));
            }
        }
    }
}
