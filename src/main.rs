use std::fmt::Debug;

#[derive(Debug, Clone)]
struct List<T>
where
    T: Debug,
{
    data: T,
    node: Option<Box<List<T>>>,
}

impl<T> List<T>
where
    T: Debug,
{
    pub fn add_last_node(&mut self, last_data: T) -> &mut List<T> {
        let mut shadow_node = &mut self.node;

        while shadow_node.is_some() {
            match shadow_node {
                None => println!("Date end"),
                Some(i) => {
                    // println!("Node reached => {:?}", i);

                    if i.node.is_none() {
                        i.node = Some(Box::new(Self {
                            data: last_data,
                            node: None,
                        }));

                        break;
                    }

                    shadow_node = &mut i.node;
                }
            };
        }

        // println!(" Last node self {:?}", self);

        self
    }

    pub fn add_first_node(self, int_data: T) -> Self {
        Self {
            data: int_data,
            node: Some(Box::new(self)),
        }
    }

    pub fn visit_nodes(self) {
        let mut shadow_node = &self.node;

        while shadow_node.is_some() {
            match shadow_node {
                None => println!("The end"),
                Some(i) => {
                    println!("Data is {:?}", i.data);
                    shadow_node = &i.node
                }
            }
        }
    }
}

fn main() {
    let mut my_list = List {
        data: "Glenn",
        node: Some(Box::new(List {
            data: "Vaud",
            node: None,
        })),
    };

    let mut my_list = my_list.add_first_node("Val");

    println!("Add a node before list -> {:#?}", my_list);

    let my_list = my_list.add_last_node("Sherline");

    println!("Add node on last empty node {:#?}", my_list);

    my_list.clone().visit_nodes();
}
