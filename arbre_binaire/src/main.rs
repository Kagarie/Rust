// Définition d'une structure pour un noeud de l'arbre
struct Node<T: PartialOrd + std::fmt::Display> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

// Implémentation de méthodes pour la structure Node
impl<T: PartialOrd + std::fmt::Display> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        if value < self.value {
            if let Some(left) = &mut self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(Node::new(value)));
            }
        } else {
            if let Some(right) = &mut self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(Node::new(value)));
            }
        }
    }

    fn search(&self, value: &T) -> bool {
        if value == &self.value {
            return true;
        }

        if value < &self.value {
            if let Some(left) = &self.left {
                return left.search(value);
            }
        } else {
            if let Some(right) = &self.right {
                return right.search(value);
            }
        }

        false
    }

    fn display(&self) {
        if let Some(left) = &self.left {
            left.display();
        }

        println!("{}", self.value);

        if let Some(right) = &self.right {
            right.display();
        }
    }
}

// Exemple d'utilisation de l'arbre binaire
fn main() {
    let mut root = Node::new(5);
    root.insert(3);
    root.insert(7);
    root.insert(1);
    root.insert(9);

    println!("{}", root.search(&1)); // true
    println!("{}", root.search(&4)); // false

    root.display();
}
