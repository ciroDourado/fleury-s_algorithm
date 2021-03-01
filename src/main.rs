// Proudly developed by: Ciro Dourado de Oliveira
// Date: 28/02/2021
// Contact: ciro dot brz at gmail dot com
// Description: Application of Fleury's Algorithm on Eulerian Graphs

use std::process::Command;
use petgraph::{
    Undirected,
    graph::{Graph, NodeIndex},
    algo::is_cyclic_undirected
};
type UnGraph<'a> = Graph<i32, &'a str, Undirected, u32>;


#[allow(unused_mut)]
fn main() {
    let mut _graph = UnGraph::new_undirected();

    let node1 = _graph.add_node(1); 
    let node2 = _graph.add_node(2);  
    let node3 = _graph.add_node(3); 
    let node4 = _graph.add_node(4);
    let node5 = _graph.add_node(5);
    let node6 = _graph.add_node(6);

    _graph.add_edge(node1, node2, "1-2");
    _graph.add_edge(node1, node3, "1-3");
    _graph.add_edge(node1, node4, "1-4");
    _graph.add_edge(node1, node5, "1-5");
    _graph.add_edge(node1, node6, "1-6");
    _graph.add_edge(node2, node3, "2-3");
    _graph.add_edge(node2, node5, "2-5");
    _graph.add_edge(node4, node6, "4-6");

    apply_fleury(&_graph);
} // end main


fn odd_degree_nodes(graph: &UnGraph) -> Vec<NodeIndex> {
    graph.node_indices()
        .filter(|&node| (degree(node, graph) % 2) != 0)
        .collect::<Vec<NodeIndex>>()
} // end odd_degree_nodes


fn degree(node: NodeIndex, graph: &UnGraph) -> usize {
    graph.neighbors(node)
        .map(|edge| if edge == node {2} else {1})
        .sum::<usize>()
} // end degree


fn count_odd_degree_nodes(_graph: &UnGraph) -> usize {
    odd_degree_nodes(_graph).len()
} // end count_odd_degree_nodes


fn lowest_odd_degree(graph: &UnGraph) -> NodeIndex {
    let nodes = odd_degree_nodes(graph);
    let degree_index = nodes.iter()
        .enumerate()
        .map(|(i, &node)| (degree(node, graph), i))
        .min();
    let index = degree_index.unwrap().1;
 
    nodes[index]
} // end lowest_degree


fn has_an_eulerian_path (graph: &UnGraph) -> bool {
    let condition_one = is_cyclic_undirected(graph);
    let condition_two = match count_odd_degree_nodes(graph) {
        0 | 2 => true,
        _ => false
    };
    condition_one && condition_two
} // end has_an_eulerian_path


fn apply_fleury(graph: &UnGraph) {
    //clear_terminal();

    match has_an_eulerian_path(graph) {
        true  => fleury_trail(&mut graph.clone()),
        false => print!("Unable to apply!\n")
    }
} // end apply_fleury


fn fleury_trail(graph: &mut UnGraph) {
    let mut node_a = fleury_start_point(graph);

    loop {
        let node_b = match edges_from(node_a, graph).get(0) {
            Some(node) => *node,
            None => break
        };

        let a_b = graph.find_edge(node_a, node_b);
        graph.remove_edge(a_b.unwrap());

        print!("{} - {}\n", 
            graph.node_weight(node_a).unwrap(), 
            graph.node_weight(node_b).unwrap());
        
        node_a = node_b;
    }
} // end fleury_trail


fn fleury_start_point(graph: &UnGraph) -> NodeIndex {
    match count_odd_degree_nodes(graph) {
        2 => lowest_odd_degree(graph),
        _ => first_node(graph)
    }
} // end fleury_start_point


fn first_node(graph: &UnGraph) -> NodeIndex {
    graph.node_indices().nth(0).unwrap()
} // end first_node


fn edges_from(node: NodeIndex, graph: &UnGraph) -> Vec<NodeIndex> {
    graph.neighbors_undirected(node).collect::<Vec<_>>()
} // end edges_from


#[allow(dead_code)]
fn clear_terminal() {
    let mut call = if cfg!(target_os = "windows") {
        Command::new("cls")
    } else { Command::new("clear") };
    call.status().expect("syscall!");
} // end clear_terminal
