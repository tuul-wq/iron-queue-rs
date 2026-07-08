#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("{field} cannot be empty")]
    Empty { field: &'static str },

    #[error("{field} is out of range {min} - {max}")]
    OutOfRange {
        field: &'static str,
        min: i16,
        max: i16,
    },
}

pub fn required_string(field: &'static str, value: String) -> Result<String, ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::Empty { field });
    }

    Ok(value)
}

pub fn required_range(
    field: &'static str,
    value: i16,
    min: i16,
    max: i16,
) -> Result<i16, ValidationError> {
    if value < min || value > max {
        return Err(ValidationError::OutOfRange { field, min, max });
    }

    Ok(value)
}
