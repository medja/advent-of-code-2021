use anyhow::Context;
use std::collections::HashMap;

const LOWER_CASE_MASK: u8 = 0x20;

const START_NODE_ID: usize = 0;
const END_NODE_ID: usize = 1;

pub fn part_a(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(Graph::parse(input)?.count_paths(false))
}

pub fn part_b(input: &[&str]) -> anyhow::Result<impl std::fmt::Display> {
    Ok(Graph::parse(input)?.count_paths(true))
}

type NodeId = usize;

struct Node {
    links: Vec<Link>,
}

impl Node {
    fn new(links: Vec<Link>) -> Self {
        Node { links }
    }

    fn links(&self) -> &[Link] {
        &self.links
    }
}

struct Link {
    id: NodeId,
    paths: usize,
}

impl Link {
    fn new(id: NodeId, paths: usize) -> Self {
        Link { id, paths }
    }

    fn id(&self) -> NodeId {
        self.id
    }

    fn path_count(&self) -> usize {
        self.paths
    }
}

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new(nodes: Vec<Node>) -> Self {
        Graph { nodes }
    }

    fn parse(input: &[&str]) -> anyhow::Result<Graph> {
        Ok(GraphBuilder::new().parse(input)?.build())
    }

    fn count_paths(&self, mut twice: bool) -> usize {
        let mut visited = vec![false; self.nodes.len()];
        self.count_paths_from(START_NODE_ID, &mut visited, &mut twice)
    }

    fn count_paths_from(&self, id: NodeId, visited: &mut [bool], twice: &mut bool) -> usize {
        if id == END_NODE_ID {
            return 1;
        }

        let second_time = visited[id];

        if second_time {
            if !*twice {
                return 0;
            } else {
                *twice = false;
            }
        }

        let node = &self.nodes[id];
        visited[id] = true;

        let count = node
            .links()
            .iter()
            .map(|link| link.path_count() * self.count_paths_from(link.id(), visited, twice))
            .sum();

        if second_time {
            *twice = true;
        } else {
            visited[id] = false;
        }

        count
    }
}

struct NodeBuilder {
    small: bool,
    links: Vec<NodeId>,
}

impl NodeBuilder {
    fn new(small: bool) -> Self {
        let links = Vec::new();
        NodeBuilder { small, links }
    }

    fn is_small(&self) -> bool {
        self.small
    }

    fn links(&self) -> &[NodeId] {
        &self.links
    }

    fn connect(&mut self, id: NodeId) {
        if id != START_NODE_ID {
            self.links.push(id)
        }
    }
}

struct GraphBuilder<'a> {
    ids: HashMap<&'a str, NodeId>,
    nodes: Vec<NodeBuilder>,
}

impl<'a> GraphBuilder<'a> {
    fn new() -> Self {
        let mut builder = GraphBuilder {
            ids: HashMap::new(),
            nodes: Vec::new(),
        };

        builder.create_node("start"); // START_NODE_ID
        builder.create_node("end"); // END_NODE_ID

        builder
    }

    fn parse(mut self, input: &[&'a str]) -> anyhow::Result<Self> {
        for line in input {
            let (left, right) = line
                .split_once('-')
                .context("Unexpected end of line, expecting `-`")?;

            let left = self.create_node(left);
            let right = self.create_node(right);

            self.nodes[left].connect(right);
            self.nodes[right].connect(left);
        }

        Ok(self)
    }

    fn build(self) -> Graph {
        let nodes = self
            .nodes
            .iter()
            .map(|node| self.build_node(node))
            .collect();

        Graph::new(nodes)
    }

    fn create_node(&mut self, name: &'a str) -> NodeId {
        *self.ids.entry(name).or_insert_with(|| {
            let id = self.nodes.len();
            let small = name.as_bytes()[0] & LOWER_CASE_MASK != 0;
            self.nodes.push(NodeBuilder::new(small));
            id
        })
    }

    fn build_node(&self, node: &NodeBuilder) -> Node {
        if !node.is_small() {
            return Node::new(Vec::new());
        }

        let mut links = HashMap::<NodeId, usize>::new();

        for &id in node.links() {
            let link = &self.nodes[id];

            if link.is_small() {
                *links.entry(id).or_default() += 1;
            } else {
                for &id in link.links() {
                    *links.entry(id).or_default() += 1;
                }
            }
        }

        let links = links
            .into_iter()
            .map(|(id, count)| Link::new(id, count))
            .collect();

        Node::new(links)
    }
}
