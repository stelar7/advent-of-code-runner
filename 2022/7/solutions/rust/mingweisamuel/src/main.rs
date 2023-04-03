use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    io::BufRead,
};

const MAX_SIZE: u64 = 70_000_000 - 30_000_000;

fn main() {
    let mut lines = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line as UTF-8."));
    assert_eq!(Some("$ cd /"), lines.next().as_deref());

    #[cfg(debug_assertions)]
    let mut path_map = HashMap::<u64, String>::new();

    let mut path_sizes = HashMap::<u64, u64>::new();
    let mut path = Vec::new();
    for line in lines {
        match line.split_once(' ') {
            Some(("$", cmd)) => match cmd {
                "ls" => continue,
                "cd /" => panic!(),
                "cd .." => {
                    path.pop().unwrap();
                }
                cd_dir => {
                    let dirname = cd_dir.strip_prefix("cd ").unwrap();
                    path.push(dirname.to_owned());
                }
            },
            Some(("dir", _dirname)) => continue,
            Some((filesize, _filename)) => {
                let filesize: u64 = filesize.parse().unwrap();
                for i in 0..=path.len() {
                    let mut hasher = DefaultHasher::new();
                    path[0..i].hash(&mut hasher);
                    let hash = hasher.finish();
                    *path_sizes.entry(hash).or_default() += filesize;

                    #[cfg(debug_assertions)]
                    path_map.entry(hash).or_insert_with(|| path.join("/"));
                }
            }
            None => panic!(),
        }
    }

    #[cfg(debug_assertions)]
    for (hash, path) in path_map.iter() {
        eprintln!("{} /{}", path_sizes[hash], path);
    }

    let p1: u64 = path_sizes
        .values()
        .copied()
        .filter(|&size| size <= 100_000)
        .sum();

    let total_size = path_sizes.values().copied().max().unwrap();
    let p2 = path_sizes
        .values()
        .copied()
        .filter(|&size| total_size - size <= MAX_SIZE)
        .min()
        .unwrap();

    println!("{}\n{}", p1, p2);
}
