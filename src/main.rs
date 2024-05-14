#[derive(Debug, Clone)]
struct List {
    data: i32,
    node: Option<Box<List>>,
}

impl List {
    pub fn add_last_node(&mut self, last_data: i32) -> &mut List {
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

    pub fn add_first_node(&self, int_data: i32) -> Self {
        Self {
            data: int_data,
            node: Some(Box::new(self.clone())),
        }
    }

    // pub fn visit_nodes() {}
}

fn main() {
    let mut my_list = List {
        data: 1,
        node: Some(Box::new(List {
            data: 2,
            node: None,
        })),
    };

    let mut my_list = my_list.add_first_node(3);

    println!("Add a node before list -> {:#?}", my_list);

    let my_list = my_list.add_last_node(4);

    println!("Add node on last empty node {:#?}", my_list);
}
