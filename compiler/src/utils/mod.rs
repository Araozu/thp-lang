pub enum Result3<T> {
    Ok(T),
    Err(T),
    None,
}

impl<T> Result3<T> {
    pub fn unwrap(&self) -> &T {
        match self {
            Result3::Ok(t) => t,
            _ => panic!(""),
        }
    }
}
