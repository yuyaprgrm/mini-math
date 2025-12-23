#[derive(Debug, PartialEq)]
pub enum VectorError {
    InvalidDimension,
    OutOfBounds,
}

pub type Result<T> = std::result::Result<T, VectorError>;

#[derive(PartialEq, Debug)]
pub struct Vector {
    data: Vec<f32>,
}

impl Vector {
    pub fn try_new(data: Vec<f32>) -> Result<Vector> {
        Self::validate(data.as_slice())?;

        Ok(Vector { data })
    }

    fn validate(data: &[f32]) -> Result<()> {
        if data.len() == 0 {
            return Err(VectorError::InvalidDimension);
        }

        if data.iter().any(|x| x.is_nan() || x.is_infinite()) {
            return Err(VectorError::OutOfBounds)
        }

        Ok(())
    }

    pub fn dimensions(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &[f32] {
        &self.data
    }

    pub fn try_add(&self, other: &Vector) -> Result<Vector> {
        if self.dimensions() != other.dimensions() {
            return Err(VectorError::InvalidDimension)
        }

        let data: Vec<f32> = self.data.iter()
            .zip(other.data.iter()).map(|(a, b)| a + b)
            .collect();

        Self::validate(&data)?;

        Ok(Vector { data })
    }

    pub fn try_scale(&self, scalar: f32) -> Result<Vector> {
        let data: Vec<f32> = self.data.iter()
            .map(|x| x * scalar)
            .collect();

        Self::validate(&data)?;

        Ok(Vector { data })
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::{Vector, VectorError};

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

    #[test]
    fn test_equals_on_same_vector() {
        let one = Vector::try_new(vec![0.1]).expect("must be valid");
        let another = Vector::try_new(vec![0.1]).expect("must be valid");

        assert_eq!(one, another)
    }

    #[test]
    fn test_equals_on_different_vectors() {
        let one = Vector::try_new(vec![0.1]).expect("must be valid");
        let another = Vector::try_new(vec![0.2]).expect("must be valid");

        assert_ne!(one, another)
    }

    #[test]
    fn test_equals_on_different_dimensions() {
        let one = Vector::try_new(vec![0.1]).expect("must be valid");
        let another = Vector::try_new(vec![0.1, 0.1]).expect("must be valid");

        assert_ne!(one, another)
    }

    #[test]
    fn test_dimensions() {
        let one = Vector::try_new(vec![0.1]).expect("must be valid");

        assert_eq!(one.dimensions(), 1);
    }

    #[test]
    fn test_add() {
        let one = Vector::try_new(vec![0.1]).expect("must be valid");
        let another = Vector::try_new(vec![0.2]).expect("must be valid");

        let result = one.try_add(&another);

        assert_eq!(result.ok(), Some(Vector::try_new(vec![0.3]).unwrap()))
    }

    #[test]
    fn test_add_failure_on_different_dimensions() {
        let one = Vector::try_new(vec![0.1]).expect("must be valid");
        let another = Vector::try_new(vec![0.1, 0.1]).expect("must be valid");
        let result = one.try_add(&another);

        assert_eq!(result.err(), Some(VectorError::InvalidDimension))
    }

    #[test]
    fn test_add_failure_on_overflow() {
        let one = Vector::try_new(vec![f32::MAX]).expect("must be valid");
        let another = Vector::try_new(vec![f32::MAX]).expect("must be valid");
        let result = one.try_add(&another);

        assert_eq!(result.err(), Some(VectorError::OutOfBounds))
    }

    #[test]
    fn test_scale() {
        let one = Vector::try_new(vec![2.0]).expect("must be valid");
        let result = one.try_scale(0.5);

        assert_eq!(result.ok(), Some(Vector::try_new(vec![1.0]).unwrap()))
    }

    #[test]
    fn test_scale_failure_on_overflow() {
        let one = Vector::try_new(vec![f32::MAX]).expect("must be valid");
        let result = one.try_scale(2.0);

        assert_eq!(result.err(), Some(VectorError::OutOfBounds))
    }
}