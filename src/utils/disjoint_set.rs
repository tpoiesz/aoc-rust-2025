// https://en.wikipedia.org/wiki/Disjoint-set_data_structure

pub struct DisjointSet {
    nodes: Vec<Node>
}

struct Node {
    parent: usize,
    size: usize
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        DisjointSet { nodes: (0..size).map(|i| Node {parent: i, size: 1}).collect() }
    }

    pub fn iter(&self) -> impl Iterator<Item=(usize, usize)> {
        self.nodes.iter().enumerate().map(|(idx, node)| (idx, node.size))
    }

    pub fn find(&mut self, x: usize) -> usize {
        Self::find_internal(&mut self.nodes, x)
    }

    fn find_internal(nodes: &mut [Node], x: usize) -> usize {
        if nodes[x].parent == x { return x; }
        nodes[x].parent = Self::find_internal(nodes, nodes[x].parent);
        nodes[x].parent
    }

    pub fn union(&mut self, mut x: usize, mut y: usize) -> usize {
        x = self.find(x);
        y = self.find(y);

        if x != y {
            if self.nodes[x].size < self.nodes[y].size {
                (x, y) = (y, x);
            }

            self.nodes[y].parent = x;
            self.nodes[x].size += self.nodes[y].size;
        }

        self.nodes[x].size
    }
}