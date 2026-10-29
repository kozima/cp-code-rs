use std::collections::HashMap;

/// Construct Eulerian circuit from edge list
/// Returns Some(circuit) if Eulerian circuit exists, None otherwise
/// The circuit is a sequence of vertices
pub fn construct_eulerian_circuit(
    n: usize,
    es: &[(usize, usize)],
) -> Option<Vec<usize>> {
    let mut adj = (0..n)
        .map(|_| HashMap::new())
        .collect::<Vec<_>>();
    for &(u, v) in es {
        adj[u].entry(v).and_modify(|e| *e += 1).or_insert(1);
        adj[v].entry(u).and_modify(|e| *e += 1).or_insert(1);
    }

    // Check connectivity
    {
        let mut visited = vec![false; n];
        let mut stack = vec![0];
        visited[0] = true;
        while let Some(u) = stack.pop() {
            for &v in adj[u].keys() {
                if !visited[v] {
                    visited[v] = true;
                    stack.push(v);
                }
            }
        }
        if visited.contains(&false) {
            return None;
        }
    }

    // Check degree of each vertex is even
    for u in 0..n {
        if adj[u].values().sum::<usize>() % 2 != 0 {
            return None;
        }
    }

    // Hierholzer's algorithm
    let mut stack = vec![0];
    let mut circuit = vec![];
    while let Some(&u) = stack.last() {
        if let Some(&v) = adj[u].keys().next() {
            adj[u].entry(v).and_modify(|e| *e -= 1).or_insert(1);
            adj[v].entry(u).and_modify(|e| *e -= 1).or_insert(1);
            if adj[u].get(&v) == Some(&0) {
                adj[u].remove(&v);
            }
            if adj[v].get(&u) == Some(&0) {
                adj[v].remove(&u);
            }
            stack.push(v);
        } else {
            circuit.push(u);
            stack.pop();
        }
    }

    adj.iter().all(|a| a.is_empty()).then(|| circuit)
}
