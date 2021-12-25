type Identifier = usize;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Node {
    pub identifier: usize,
    pub char: Option<char>,
    pub frequency: usize,
    pub left: Option<Identifier>,
    pub right: Option<Identifier>,
}

impl Node {
    pub fn copy(&self) -> Node {
        Node {
            identifier: self.identifier,
            char: self.char,
            frequency: self.frequency,
            left: self.left,
            right: self.right,
        }
    }
}

impl Node {
    pub unsafe fn generate_identifier() -> usize {
        static mut COUNTER: usize = 0;
        let return_value: usize = COUNTER;
        COUNTER += 1;
        return_value
    }

    pub fn new(char: char, frequency: usize) -> Self {
        Node {
            identifier: unsafe { Node::generate_identifier() },
            char: Option::Some(char),
            frequency,
            left: Option::None,
            right: Option::None,
        }
    }

    pub fn collapse(first: &Node, second: &Node) -> Self {
        Node {
            identifier: unsafe { Node::generate_identifier() },
            char: Option::None,
            frequency: first.frequency + second.frequency,
            left: Option::Some(if first.frequency <= second.frequency { first.identifier } else { second.identifier }),
            right: Option::Some(if first.frequency > second.frequency { first.identifier } else { second.identifier }),
        }
    }
}