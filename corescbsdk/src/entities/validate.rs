use regex::Regex;
use validator::ValidationError;

pub fn validate_amount(amount: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^\d{1,10}(\.\d{1,2})?$").unwrap();

    // Check if length is within 13 characters
    if amount.len() > 13 {
        return Err(ValidationError::new("length"));
    }

    // Check if it matches the regex pattern
    if !re.is_match(amount) {
        return Err(ValidationError::new("format"));
    }

    Ok(())
}
pub fn validate_data_type_az09(data: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^[A-Z0-9]+$").unwrap();

    if !re.is_match(data) {
        return Err(ValidationError::new("invalid_format"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_amount() {
        assert_eq!(validate_amount("123.45"), Ok(()));
        assert_eq!(validate_amount("123"), Ok(()));
        assert_eq!(
            validate_amount("123.456"),
            Err(ValidationError::new("format"))
        );
        assert_eq!(
            validate_amount("123456789012345"),
            Err(ValidationError::new("length"))
        );
    }
    #[test]
    fn test_validate_data_type_az09() {
        assert_eq!(validate_data_type_az09("ABC123"), Ok(()));
        assert_eq!(
            validate_data_type_az09("ABC123!"),
            Err(ValidationError::new("invalid_format"))
        );
    }
}
