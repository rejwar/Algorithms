struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn Traverse(node: &Option<Box<Node>>) {
    if let Some(n) = node {
        Traverse(&n.left);
        println!("{}", n.value);
        Traverse(&n.right);
    }
}

fn main() {
    let tree = Some(Box::new(Node {
        value: 2,
        left: Some(Box::new(Node {
            value: 1,
            left: None,
            right: None,
        })),
        right: Some(Box::new(Node {
            value: 3,
            left: None,
            right: None,
        })),
    }));

    Traverse(&tree);
}
