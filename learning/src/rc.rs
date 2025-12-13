use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    neighbors: Vec<Rc<Node>>,
}

struct TreeNode {
    value: i32,
    children: Vec<Rc<TreeNode>>,
}

fn main() {
    let data = Rc::new(String::from("shared data"));

    let owner1 = Rc::clone(&data);
    let owner2 = Rc::clone(&data); // ✅ Funciona: ambas referencias son

    // Todos apuntan a los mismos datos en memoria
    println!("Data: {}", data);
    println!("Owner1: {}", owner1);
    println!("Owner2: {}", owner2);

    // Create nodes
    let node_a = Rc::new(Node { value: 1, neighbors: vec![] });

    let node_b = Rc::new(Node { value: 2, neighbors: vec![] });

    let node_c = Rc::new(Node { value: 3, 
        neighbors: vec![
            Rc::clone(&node_a), 
            Rc::clone(&node_b)
        ] 
    });

    println!("node_a strong count: {}", Rc::strong_count(&node_a)); // 2
    println!("node_c: {:?}", node_c);
    
    
    // Create tree nodes
    let leaf1 = Rc::new(TreeNode { value: 3, children: vec![] });
    let leaf2 = Rc::new(TreeNode { value: 4, children: vec![] });

    let branch = Rc::new(TreeNode { 
        value: 2, 
        children: vec![
            Rc::clone(&leaf1), 
            Rc::clone(&leaf2)
        ] 
    });
    let root = Rc::new(TreeNode { 
        value: 1, 
        children: vec![
            Rc::clone(&branch)
        ] 
    });

    // branch puede ser accedido tanto desde root como directamente
    println!("branch count: {}", Rc::strong_count(&branch)); // 2
}