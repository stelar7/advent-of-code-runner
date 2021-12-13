use std::collections::HashMap;
use std::io::BufRead;

const START: Node = Node {
    chars: [b's', b't', b'a', b'r'],
    is_small: true,
};
const END: Node = Node {
    chars: [b'e', b'n', b'd', 0],
    is_small: true,
};

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Debug)]
#[derive(Clone, Copy)]
struct Node {
    chars: [u8; 4],
    is_small: bool,
}
impl From<&str> for Node {
    fn from(string: &str) -> Self {
        let mut node = Node {
            chars: [0; 4],
            is_small: string.chars().next().unwrap().is_lowercase()
        };
        for (i, c) in string.bytes().take(4).enumerate() {
            node.chars[i] = c;
        }
        node
    }
}

fn count_paths(
    adj_list: &HashMap<Node, Vec<Node>>,
    path: &mut Vec<Node>,
    allow_double_visit: bool,
) -> usize {
    let mut paths = 0;
    if let Some(nexts) = adj_list.get(path.last().unwrap()) {
        for &next in nexts {
            if END == next {
                paths += 1;
                continue;
            }
            if START == next {
                continue;
            }
            let next_already_visited = next.is_small && path.contains(&next);
            if !allow_double_visit && next_already_visited {
                continue;
            }
            path.push(next.to_owned());
            paths += count_paths(adj_list, path, allow_double_visit && !next_already_visited);
            path.pop().unwrap();
        }
    }
    paths
}

fn main() {
    let mut adj_list: HashMap<Node, Vec<Node>> = HashMap::new();

    for (a, b) in std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."))
        .map(|line| {
            let (a, b) = line.split_once('-').expect("Line missing '-'.");
            (a.into(), b.into())
        })
    {
        adj_list.entry(a).or_default().push(b);
        adj_list.entry(b).or_default().push(a);
    }

    let mut path = vec![START];
    let paths_a = count_paths(&adj_list, &mut path, false);
    let paths_b = count_paths(&adj_list, &mut path, true);
    println!("{}\n{}", paths_a, paths_b);
}
