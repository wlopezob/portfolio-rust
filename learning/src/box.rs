use std::collections::hash_map::Values;

trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode { 
            value, 
            left: None, 
            right: None
        }
    }

    fn insert(&mut self, value: i32) {
        if value < self.value {
            match &mut self.left {
                None => self.left = Some(Box::new(TreeNode::new(value))),
                Some(node) => node.insert(value),                
            }
        } else {
            match &mut self.right {
                None => self.right = Some(Box::new(TreeNode::new(value))),
                Some(node) => node.insert(value),                
            }
        }
    }
}

trait Draw{ fn draw(&self); }

struct Button { label: String }
struct Checkbox { checked: bool }

impl Draw for Button {
    fn draw(&self) {
        println!("[Button: {}]", self.label);
    }
}

impl Draw for Checkbox {
    fn draw(&self) {
        println!("[Checkbox: {}]", if self.checked { "X" } else { " " });
    }
}

fn main() {
    let a = Box::new(5);
    println!("Boxed value: {}", a);

    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog), 
        Box::new(Cat)
    ];

    for animal in animals {
        animal.speak();
    }

    let mut root = TreeNode::new(10);
    root.insert(5);
    root.insert(15);
    root.insert(3);
    root.insert(7);

    println!("Binary Tree: {:?}", root);

    let ui: Vec<Box<dyn Draw>> = vec![
        Box::new(Button { label: "OK".into() }),
        Box::new(Checkbox { checked: true }),
    ];
    
    for component in ui {
        component.draw();
    }
}