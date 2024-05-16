extern crate petgraph;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};

fn main() {
    // Create a new directed graph
    let mut graph = DiGraph::new();

    // Add nodes
    let all_heads = graph.add_node("allHEADs");
    let all_tails = graph.add_node("allTAILs");
    let all_list_ordered_of_symbols = graph.add_node("AllListOrderedOfSymbols");
    let list1 = graph.add_node("list1");
    let list2 = graph.add_node("list2");
    let list3 = graph.add_node("list3");
    let list4 = graph.add_node("list4");
    let all_element_capsules = graph.add_node("allElementCapsules");
    let element_capsule1 = graph.add_node("ElementCapsule1");
    let element_capsule2 = graph.add_node("ElementCapsule2");
    let element_capsule3 = graph.add_node("ElementCapsule3");
    let element_capsule4 = graph.add_node("ElementCapsule4");
    let element_capsule5 = graph.add_node("ElementCapsule5");
    let element_capsule6 = graph.add_node("ElementCapsule6");
    let element_capsule7 = graph.add_node("ElementCapsule7");
    let element_capsule8 = graph.add_node("ElementCapsule8");
    let element_capsule9 = graph.add_node("ElementCapsule9");
    let all_prev_element_capsules = graph.add_node("allPrevElementCapsules");
    let all_elements_of_element_capsules = graph.add_node("allElements of ElementCapsules");
    let all_next_element_capsules = graph.add_node("allNextElementCapsules");
    let unique1 = graph.add_node("unique1");
    let unique2 = graph.add_node("unique2");
    let unique3 = graph.add_node("unique3");
    let unique40 = graph.add_node("unique40");
    let unique50 = graph.add_node("unique50");
    let unique60 = graph.add_node("unique60");
    let symbol1_as_first_element = graph.add_node("symbol1 as first element");
    let symbol2_as_first_and_last_element = graph.add_node("symbol2 as first and last element");

    // Add edges
    graph.add_edge(all_heads, element_capsule1, ());
    graph.add_edge(all_heads, element_capsule9, ());
    graph.add_edge(list1, element_capsule1, ());
    graph.add_edge(list1, all_element_capsules, ());
    graph.add_edge(list1, element_capsule9, ());
    graph.add_edge(all_list_ordered_of_symbols, list2, ());
    graph.add_edge(all_list_ordered_of_symbols, list4, ());
    graph.add_edge(all_list_ordered_of_symbols, all_tails, ());
    graph.add_edge(list3, element_capsule1, ());
    graph.add_edge(list3, element_capsule9, ());
    graph.add_edge(list2, element_capsule8, ());
    graph.add_edge(list4, element_capsule6, ());
    graph.add_edge(unique1, element_capsule9, ());
    graph.add_edge(unique1, unique40, ());
    graph.add_edge(unique2, unique3, ());
    graph.add_edge(unique2, element_capsule9, ());
    graph.add_edge(unique3, element_capsule2, ());
    graph.add_edge(unique3, unique60, ());
    graph.add_edge(unique50, symbol2_as_first_and_last_element, ());
    graph.add_edge(unique50, element_capsule9, ());
    graph.add_edge(symbol1_as_first_element, unique60, ());
    graph.add_edge(element_capsule1, unique2, ());
    graph.add_edge(element_capsule2, unique3, ());
    graph.add_edge(all_element_capsules, element_capsule3, ());
    graph.add_edge(all_element_capsules, element_capsule4, ());
    graph.add_edge(all_element_capsules, element_capsule5, ());
    graph.add_edge(all_element_capsules, element_capsule6, ());
    graph.add_edge(all_element_capsules, element_capsule7, ());

    // Print the graph in DOT format
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}

