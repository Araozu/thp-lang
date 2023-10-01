

#[derive(Debug)]
pub struct FunctionCall {
    pub identifier: Box<String>
}


#[derive(Debug)]
pub struct ArgumentsList {
    pub arguments: Vec<Box<Argument>>
}

#[derive(Debug)]
pub enum Argument {}
