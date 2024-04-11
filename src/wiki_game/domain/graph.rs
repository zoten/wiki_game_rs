/** Treat links as graphs
 *  A lot of ".clone()" cleansing to do probably
*/
use std::collections::HashMap;

use petgraph::algo::astar;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Directed;

use super::coordinator::RelWikiUrl;

pub struct LinksGraph {
    nodes_map: HashMap<RelWikiUrl, NodeIndex>,
    // <node, edge associated data>
    graph: Graph<RelWikiUrl, RelWikiUrl, Directed>,
}

type ShortestPathWeights = Vec<RelWikiUrl>;

impl LinksGraph {
    pub fn new() -> LinksGraph {
        let graph: Graph<RelWikiUrl, RelWikiUrl, Directed> = Graph::new();

        LinksGraph {
            nodes_map: HashMap::new(),
            graph: graph,
        }
    }

    fn node_idx(&self, node: &RelWikiUrl) -> Option<&NodeIndex> {
        self.nodes_map.get(node)
    }

    pub fn node_exists(&self, node: &RelWikiUrl) -> bool {
        self.nodes_map.contains_key(node)
    }

    fn maybe_create_node(&mut self, node: &RelWikiUrl) -> NodeIndex {
        if let Some(idx) = self.node_idx(node) {
            return *idx;
        }

        let idx = self.graph.add_node(node.clone());
        self.nodes_map.insert(node.clone(), idx);

        idx
    }

    /// Creates the nodes if needed
    pub fn add_edge(&mut self, a: &RelWikiUrl, b: &RelWikiUrl) {
        let idxa = self.maybe_create_node(a);
        let idxb = self.maybe_create_node(b);

        self.graph.add_edge(idxa, idxb, "".to_string());
    }

    pub fn shortest_path_to_target(
        &self,
        start: RelWikiUrl,
        target: RelWikiUrl,
    ) -> Result<ShortestPathWeights, String> {
        let start_idx = self
            .nodes_map
            .get(&start)
            .ok_or(String::from("Start node does not exist"))?;
        let target_idx = self
            .nodes_map
            .get(&target)
            .ok_or(String::from("Target node does not exist"))?;
        // use petgraph::algo::{astar, dijkstra};
        // let result = dijkstra(&self.graph, *start_idx, Some(*target_idx), |_| 1);
        if let Some(result) = astar(
            &self.graph,
            *start_idx,
            |node| node == *target_idx,
            |_| 1,
            |_| 0,
        ) {
            let (_, shortest_path) = result;
            let shortest_path: ShortestPathWeights = shortest_path
                .iter()
                // .iter()
                // .map(|(node_index, _weight)| self.graph.node_weight(*node_index).unwrap().clone())
                .map(|node_index| self.graph.node_weight(*node_index).unwrap().clone())
                .collect();
            Ok(shortest_path)
        } else {
            return Err(String::from("No path found"));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn shortest_path_no_start_test() {
        let g = LinksGraph::new();
        let sp = g.shortest_path_to_target(String::from("start"), String::from("end"));
        assert!(sp.is_err());
    }

    #[test]
    fn shortest_path_no_target_test() {
        let mut g = LinksGraph::new();
        let start = String::from("start");
        let target = String::from("target");
        g.add_edge(&start, &String::from("node0"));

        let sp = g.shortest_path_to_target(String::from("start"), target);
        assert!(sp.is_err());
    }

    #[test]
    fn shortest_path_one_step_test() {
        let mut g = LinksGraph::new();
        let start = String::from("start");
        let target = String::from("target");
        g.add_edge(&start, &target);

        let sp = g.shortest_path_to_target(String::from("start"), target);
        assert!(sp.is_ok());
        assert!(sp.unwrap().len() == 2);
    }

    #[test]
    fn shortest_path_multiple_step_test() {
        //        a -> b
        // start /      \ target
        //       \      /
        //           c
        let mut g = LinksGraph::new();
        let start = String::from("start");
        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");

        let target = String::from("target");
        g.add_edge(&start, &a);
        g.add_edge(&start, &c);
        g.add_edge(&a, &b);
        g.add_edge(&b, &target);
        g.add_edge(&c, &target);

        let sp = g.shortest_path_to_target(String::from("start"), target);
        assert!(sp.is_ok());
        // println!("{:?}", sp.clone().unwrap());
        assert_eq!(sp.unwrap().len(), 3);
    }
}
