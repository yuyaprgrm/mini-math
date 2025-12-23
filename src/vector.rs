pub enum VectorError {
    InvalidDimension,
    OutOfBounds,
}

pub type Result<T> = std::result::Result<T, VectorError>;

pub struct Vector {
    data: Vec<f32>,
}

impl Vector {
    pub fn try_new(data: Vec<f32>) -> Result<Vector> {
        if data.len() == 0 {
            return Err(VectorError::InvalidDimension);
        }

        if data.iter().any(|x| x.is_nan() || x.is_infinite()) {
            return Err(VectorError::OutOfBounds)
        }

        Ok(Vector { data })
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::Vector;

    #[test]
    fn test_try_new_success_on_finite() {
        assert!(Vector::try_new(vec![0.0, 1.0, f32::MAX]).is_ok())
    }

    #[test]
    fn test_try_new_failure_on_infinite() {
        assert!(Vector::try_new(vec![1.0, 2.0, f32::INFINITY]).is_err());
    }

    #[test]
    fn test_try_new_failure_on_zero_dimension() {
        assert!(Vector::try_new(vec![]).is_err())
    }
}