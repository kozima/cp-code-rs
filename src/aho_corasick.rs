use std::collections::VecDeque;

#[derive(Debug)]
pub struct AhoCorasickAutomaton {
    words: Vec<String>,
    node_count: usize,
    children: Vec<Vec<(char, usize)>>,
    failure: Vec<usize>,
    output: Vec<Vec<usize>>,
}

impl AhoCorasickAutomaton {
    const ROOT: usize = 0;

    pub fn new(words: &Vec<String>) -> AhoCorasickAutomaton {
        let mut a = AhoCorasickAutomaton {
            words: words.clone(),
            node_count: 1,
            children: vec![Vec::new(); 1],
            failure: Vec::new(),
            output: Vec::new(),
        };

        for word in words {
            a.add_word(&word);
        }

        a.failure.resize(a.node_count, 0);
        a.output.resize(a.node_count, Vec::new());

        for (i, word) in words.iter().enumerate() {
            let mut node = AhoCorasickAutomaton::ROOT;
            for c in word.chars() {
                node = a.next_node(node, c).unwrap();
            }
            a.output[node].push(i);
        }

        let mut q = VecDeque::new();
        for &(label, node) in &a.children[AhoCorasickAutomaton::ROOT] {
            q.push_back((node, label, AhoCorasickAutomaton::ROOT));
        }
        while let Some((node, label, parent)) = q.pop_front() {
            assert!(node != AhoCorasickAutomaton::ROOT);
            for &(label, child) in &a.children[node] {
                q.push_back((child, label, node));
            }
            if parent == AhoCorasickAutomaton::ROOT {
                a.failure[node] = parent;
            } else {
                let mut cand = a.failure[parent];
                loop {
                    if let Some(v) = a.next_node(cand, label) {
                        a.failure[node] = v;
                        break;
                    } else if cand == AhoCorasickAutomaton::ROOT {
                        a.failure[node] = AhoCorasickAutomaton::ROOT;
                        break;
                    } else {
                        cand = a.failure[cand];
                    }
                }

                for i in a.output[a.failure[node]].clone() {
                    a.output[node].push(i);
                }
            }
        }

        a
    }

    fn next_node(&self, node: usize, c: char) -> Option<usize> {
        self.children[node]
            .iter()
            .find(|(label, _)| *label == c)
            .map(|(_, child)| *child)
    }

    fn next_node_or_add(&mut self, node: usize, c: char) -> usize {
        if let Some(child) = self.next_node(node, c) {
            child
        } else {
            let new_node = self.node_count;
            self.children.push(Vec::new());
            self.children[node].push((c, new_node));
            self.node_count += 1;
            new_node
        }
    }

    fn add_word(&mut self, word: &str) {
        let mut node = AhoCorasickAutomaton::ROOT;
        for c in word.chars() {
            node = self.next_node_or_add(node, c)
        }
    }

    fn transition(&self, node: usize, c: char) -> usize {
        let mut cur = node;
        loop {
            if let Some(child) = self.next_node(cur, c) {
                break child;
            } else if cur == AhoCorasickAutomaton::ROOT {
                break AhoCorasickAutomaton::ROOT;
            } else {
                cur = self.failure[cur];
            }
        }
    }

    pub fn run(&self, text: &String) -> Vec<(usize, String)> {
        let mut result = Vec::new();
        let mut node = AhoCorasickAutomaton::ROOT;
        for (i, c) in text.chars().enumerate() {
            node = self.transition(node, c);
            for &j in &self.output[node] {
                result.push((i, j));
            }
        }
        result
            .iter()
            .map(|(i, j)| (*i, self.words[*j].clone()))
            .collect()
    }
}
