extern crate contest;
extern crate petgraph;

fn main() {
    use contest::io::scanner;
    use petgraph::graph::Graph;
    use std::cmp::max;
    let mut sc = scanner::Scanner::new();
    let t = sc.next();
    for case in 0..t {
        let n: i32 = sc.next();
        let mut graph = Graph::<(), ()>::new();
        let mut nodes = std::vec::Vec::new();
        for _ in 0..n {
            nodes.push(graph.add_node(()));
        }
        for i in 0..n {
            let bbf: usize = sc.next();
            graph.add_edge(nodes[i as usize], nodes[bbf - 1], ());
        }
        let scc = petgraph::algo::tarjan_scc(&graph);
        // Longest path reachable to the node.
        let mut dp = vec![0; n as usize];
        let mut res1 = 0;
        let mut res2 = 0;
        for nixs in scc.iter().rev() {
            let sz = nixs.len();
            match sz {
                1 => {
                    let nix = nixs[0];
                    let to = graph.neighbors(nix).next().unwrap().index();
                    dp[to] = max(dp[to], dp[nix.index()] + 1);
                }
                2 => {
                    res2 += 2 + dp[nixs[0].index()] + dp[nixs[1].index()];
                }
                _ => res1 = max(res1, sz),
            }
        }
        println!("Case #{}: {}", case + 1, max(res1, res2));
    }
}
