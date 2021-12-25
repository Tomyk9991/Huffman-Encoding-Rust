use std::collections::HashMap;
use string_builder::Builder;

pub struct HuffmannCode;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Left = 0,
    Right = 1
}

pub struct HuffmannResult {
    pub encrypted_string: String,
    pub start: Identifier,
    pub data: Vec<Node>
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        format!("{}", if *self == Direction::Left { "0" } else { "1" })
    }
}

impl Direction {
    pub fn list_to_string(directions: &[Direction]) -> String {
        let mut builder: Builder = string_builder::Builder::default();

        for direction in directions {
            builder.append(direction.to_string());
        }


        return builder.string().unwrap();
    }
}

impl HuffmannCode {
    pub fn new() -> Self {
        HuffmannCode {
            
        }
    }

    pub fn encrypted_string_to_text(&self, result: &HuffmannResult) -> String {
        let data: &Vec<Node> = &result.data;
        let mut builder = string_builder::Builder::default();

        let mut current: &Node = HuffmannCode::get_node_from_identifier(data, result.start).unwrap();


        for char in result.encrypted_string.chars() {
            if let Some(temp) = HuffmannCode::get_node_from_identifier(data, current.identifier) {
                if char == '0' {
                    if let Some(left) = temp.left {
                        if let Some(target_char) = HuffmannCode::get_node_from_identifier(data, left).unwrap().char {
                            builder.append(target_char);
                            current = HuffmannCode::get_node_from_identifier(data, result.start).unwrap();
                            continue;
                        }

                        current = HuffmannCode::get_node_from_identifier(data, left).unwrap();
                    } else {
                        builder.append(temp.char.unwrap());
                    }
                } else {
                    if let Some(right) = temp.right {
                        if let Some(target_char) = HuffmannCode::get_node_from_identifier(data, right).unwrap().char {
                            builder.append(target_char);
                            current = HuffmannCode::get_node_from_identifier(data, result.start).unwrap();
                            continue;
                        }

                        current = HuffmannCode::get_node_from_identifier(data, right).unwrap();
                    } else {
                        builder.append(temp.char.unwrap());
                    }
                }
            }
        }

        return builder.string().unwrap();
    }

    pub fn huffmann_algorithm(&self, text: &str) -> HuffmannResult {
        // 1)
        let ordering: Vec<LetterFrequencyValuePair> = HuffmannCode::relative_frequency(text);
    
        // 2)
        let mut nodes: Vec<Node> = Vec::new();
        let mut data: Vec<Node> = Vec::new();
    
        for lfvp in &ordering {
            nodes.push(Node::new(lfvp.char, lfvp.total_frequency));
            data.push(nodes.last().unwrap().copy());
        }
    
        // 3)
        while nodes.len() > 1 {
            let last1: Node = nodes.remove(nodes.len() - 1);
            let last2: Node = nodes.remove(nodes.len() - 1);
    
            let collapsing_node: Node = Node::collapse(&last1, &last2);
            nodes.push(collapsing_node.copy());
            data.push(collapsing_node.copy());
    
    
            nodes.sort_by(|a: &Node, b: &Node| b.frequency.partial_cmp(&(a.frequency)).unwrap());
        }
    
        data.sort_by(|a: &Node, b: &Node| b.identifier.partial_cmp(&(a.identifier)).unwrap());


        let mut all_directions:Vec<Direction> = Vec::new();
        for char in text.chars() {
            let mut code_directions: Vec<Direction> = HuffmannCode::code_for_target(char, nodes.get(0).unwrap().identifier, &data);
            all_directions.append(&mut code_directions);
        }

        return HuffmannResult {
            encrypted_string: Direction::list_to_string(&all_directions),
            data,
            start: nodes.get(0).unwrap().identifier
        }
    }

    fn code_for_target(target: char, start: Identifier, nodes: &[Node]) -> Vec<Direction> {
        let mut path: Vec<Direction> = Vec::new();

        let ending_node: &Node = HuffmannCode::get_node_from_char(nodes, target).unwrap();

        let tuple: Option<(Identifier, Direction)> = HuffmannCode::get_parent_from_child_direction(nodes, ending_node.identifier);
        
        let (mut parent, direction) = tuple.unwrap();

        path.push(direction);

        while parent != start {
            if let Some(temp) = HuffmannCode::get_parent_from_child_direction(nodes, parent) {
                parent = temp.0;
                path.push(temp.1);
            }
        }
        
        path.reverse();
        return path;
    }

    fn get_parent_from_child_direction(nodes: &[Node], child: Identifier) -> Option<(Identifier, Direction)> {
        for idx in 0..nodes.len() {
            if let Some(left) = nodes.get(idx).unwrap().left {
                if left == child {
                    return Option::from((HuffmannCode::get_node_from_identifier(nodes, nodes.get(idx).unwrap().identifier).unwrap().identifier, Direction::Left));
                }
            };

            if let Some(right) = nodes.get(idx).unwrap().right {
                if right == child {
                    return Option::from((HuffmannCode::get_node_from_identifier(nodes, nodes.get(idx).unwrap().identifier).unwrap().identifier, Direction::Right));
                }
            };
        }

        return Option::None;
    }

    fn get_node_from_char(nodes: &[Node], target: char) -> Option<&Node> {
        for idx in 0..nodes.len() {
            if let Some(comparing_char) = nodes.get(idx).unwrap().char {
                if comparing_char == target {
                    return Option::Some(nodes.get(idx).unwrap());
                }
            }
        }
    
        return Option::None;
    }
    
    fn get_node_from_identifier(nodes: &[Node], i: usize) -> Option<&Node> {
        let mut size = nodes.len();
        let mut left = 0;
        let mut right = size;

        while left < right {
            let mid = left + size / 2;

            let cmp:i32 = nodes.get(mid).unwrap().identifier as i32 - i as i32;

            if cmp > 0 {
                left = mid + 1;
            } else if cmp < 0 {
                right = mid;
            } else {
                return nodes.get(mid);
            }

            size = right - left;
        }

        return Option::None;
    }
    
    fn relative_frequency(text: &str) -> Vec<LetterFrequencyValuePair> {
        let mut letter_counts: HashMap<char, f32> = HashMap::new();
    
        let char_vec :Vec<char> = text.chars().collect();
    
        for c in char_vec {
            *letter_counts.entry(c).or_insert(0.0_f32) += 1.0_f32;
        }
    
        let mut vec: Vec<LetterFrequencyValuePair> = letter_counts
            .into_iter()
            .map(|(char, amount)| LetterFrequencyValuePair { char, total_frequency: amount as usize })
            .collect();
        vec.sort_by(|a: &LetterFrequencyValuePair, b: &LetterFrequencyValuePair| b.total_frequency.partial_cmp(&(a.total_frequency)).unwrap());
        return vec;
    }
}
