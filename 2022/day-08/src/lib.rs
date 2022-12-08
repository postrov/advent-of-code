use std::cmp::max;

#[derive(Debug)]
struct Node {
    height: i32,
    left: i32,
    right: i32,
    up: i32,
    down: i32,
}

impl Node {
    fn new(height: i32) -> Self {
        Node {
            height,
            left: -1,
            right: -1,
            up: -1,
            down: -1,
        }
    }

    fn new2(height: i32) -> Self {
        Node {
            height,
            left: 0,
            right: 0,
            up: 0,
            down: 0,
        }
    }

    fn is_visible(&self) -> bool {
        self.height > self.left ||
        self.height > self.right ||
        self.height > self.up ||
        self.height > self.down
    }

    fn scenic_score(&self) -> i32 {
        self.left * self.right * self.up * self.down
    }
}

pub fn process_part1(input: &str) -> String {
    let size = input.lines()
        .next()
        .expect("empty input")
        .len();

    // a buffer with tree nodes, from top top left to bottom right
    let mut nodes: Vec<Node> = input.chars()
        .filter(|&c| c.is_ascii_digit())
        .map(|c| Node::new(c as i32 - '0' as i32 + 1))
        .collect();

    // skip first row and column
    (size..nodes.len())
        .filter(|idx| idx % size != 0)
        .for_each(|idx| {
            let node_left = &nodes[idx - 1];
            let node_up = &nodes[idx - size];
            let left = max(node_left.height, node_left.left);
            let up = max(node_up.height, node_up.up);
            nodes[idx].left =  left;
            nodes[idx].up = up;

            let rev_idx = nodes.len() - idx - 1;
            let node_right = &nodes[rev_idx + 1];
            let node_down = &nodes[rev_idx + size];
            let right = max(node_right.height, node_right.right);
            let down = max(node_down.height, node_down.down);
            nodes[rev_idx].right = right;
            nodes[rev_idx].down =  down;
        });

    nodes.iter()
        .filter(|&node| node.is_visible())
        .count()
        .to_string()
}

fn count_visible_trees<I: Iterator<Item=usize>>(nodes: &[Node], height: i32, iter: I) -> i32 {
   let indices: Vec<usize> = iter.collect(); 

   let counted = indices.iter()
        .take_while(|&&i| nodes[i].height < height)
        .count() as i32;
    
    if counted == indices.len() as i32 {
        // when we reach border, we don't see any further
        counted
    } else {
        // we still see one tree if immediately blocked
        counted + 1
    }
}

pub fn process_part2(input: &str) -> String {
    let size = input.lines()
        .next()
        .expect("empty input")
        .len();

    // a buffer with tree nodes, from top top left to bottom right
    let mut nodes: Vec<Node> = input.chars()
        .filter(|&c| c.is_ascii_digit())
        .map(|c| Node::new2(c as i32 - '0' as i32 + 1))
        .collect();

    // skip first and last rows and columns
    (size..nodes.len())
        .filter(|idx| {
            let x = idx % size;
            x != 0 && x != size - 1
        })
        .for_each(|idx| {
            let x = idx % size;
            let y = idx / size;

            let height = nodes[idx].height;
            nodes[idx].left = count_visible_trees(&nodes, height, ((y * size)..idx).rev());
            nodes[idx].right = count_visible_trees(&nodes, height, (idx + 1)..((y + 1) * size));
            nodes[idx].up = count_visible_trees(&nodes, height, (0..y).rev().map(|i| i * size + x));
            nodes[idx].down = count_visible_trees(&nodes, height, ((y + 1)..size).map(|i| i * size + x));
        });

    nodes.iter()
        .map(|node| node.scenic_score())
        .max()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        assert_eq!("21", process_part1(INPUT));
    }

    #[test]
    fn part2_works() {
        assert_eq!("8", process_part2(INPUT));
    }
}
