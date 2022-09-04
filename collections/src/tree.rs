// Immutable Tree implementation via node and edge vectors
pub fn list() {
    #[derive(Debug)]
    struct Node {
        _value: i32,
    }

    #[derive(Debug)]
    struct Edge<'a> {
        _n1: &'a Node,
        _n2: &'a Node,
    }

    let root = Node { _value: 0 };
    let child1 = Node { _value: 1 };
    let child2 = Node { _value: 2 };

    let e1 = Edge {
        _n1: &root,
        _n2: &child1,
    };
    let e2 = Edge {
        _n1: &root,
        _n2: &child2,
    };

    let nodes = vec![&root, &child1, &child2];
    let edges = vec![&e1, &e2];

    for e in edges {
        println!("{:?}", e);
    }

    for n in nodes {
        println!("{:?}", n);
    }
}
