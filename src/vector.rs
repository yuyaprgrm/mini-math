pub enum VectorError {
}

pub type Result<T> = std::result::Result<T, VectorError>;

pub struct Vector {
    data: Vec<f32>,
}

impl Vector {
    pub fn new(data: Vec<f32>) -> Vector {
        Self { data }
    }
}

#[cfg(test)]
mod tests {
}