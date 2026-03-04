pub trait Node {
    fn token_literal(&self) -> String {
        todo!()
    }
}

pub trait Statement: Node {
    fn statement_node(&self) {
        todo!()
    }
}

pub trait Expression: Node {
    fn expression_node(&self) {
        todo!()
    }
}

// struct Program {
//     statements: Vec<Box<dyn Statement>>,
// }
