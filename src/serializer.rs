use crate::{Operation, Portfolio, Tax};
use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ConverterError {
    #[error("Invalid input")]
    InvalidInput,
    #[error("Invalid operation")]
    InvalidOperation,
    #[error("Invalid tax conversion")]
    InvalidTaxConversion,
}

pub fn converter_raw_json(raw_json: &str) -> Result<String, ConverterError> {
    let operations: Result<Vec<Operation>, serde_json::Error> = serde_json::from_str(raw_json);
    let operations = match operations {
        Ok(operations) => operations,
        Err(_) => return Err(ConverterError::InvalidInput),
    };

    let mut portfolio = Portfolio::new();
    let mut taxes: Vec<Tax> = Vec::new();

    for operation in operations {
        let tax = portfolio.execute(operation);
        match tax {
            Ok(tax) => taxes.push(tax),
            Err(_) => return Err(ConverterError::InvalidOperation),
        }
    }

    let taxes_string = serde_json::to_string(&taxes);
    match taxes_string {
        Ok(taxes_string) => Ok(taxes_string),
        Err(_) => Err(ConverterError::InvalidTaxConversion),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_serializer_invalid_input() {
        let data = r#"[
            {"operation":"buy", "unitcost":10.00, "quantity": 100},
            {"operation":"sell", "unit-cost":15.00, "quantity": 50},
            {"operation":"sell", "unit-cost":15.00, "quantity": 50}
        ]"#;

        let result = converter_raw_json(data);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConverterError::InvalidInput));
    }

    #[test]
    fn test_serializer_invalid_input_order() {
        let data = r#"[
            {"operation":"sell", "unit-cost":15.00, "quantity": 50},
            {"operation":"buy", "unit-cost":10.00, "quantity": 100},
            {"operation":"sell", "unit-cost":15.00, "quantity": 50}
        ]"#;

        let result = super::converter_raw_json(data);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConverterError::InvalidOperation
        ));
    }

    #[test]
    fn test_serializer_case_1() {
        let data = r#"[
            {"operation":"buy", "unit-cost":10.00, "quantity": 100},
            {"operation":"sell", "unit-cost":15.00, "quantity": 50},
            {"operation":"sell", "unit-cost":15.00, "quantity": 50}
        ]"#;

        let expected_result = r#"[{"tax":0},{"tax":0},{"tax":0}]"#;

        let result = super::converter_raw_json(data);
        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_serializer_case_2() {
        let data = r#"[
            {"operation":"buy", "unit-cost":10, "quantity": 10000},
            {"operation":"sell", "unit-cost":20, "quantity": 5000},
            {"operation":"sell", "unit-cost":5, "quantity": 5000}
        ]"#;

        let expected_result = r#"[{"tax":0},{"tax":10000},{"tax":0}]"#;

        let result = super::converter_raw_json(data);
        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_serializer_case_3() {
        let data = r#"[
            {"operation":"buy", "unit-cost":10, "quantity": 10000},
            {"operation":"sell", "unit-cost":5, "quantity": 5000},
            {"operation":"sell", "unit-cost":20, "quantity": 5000}
        ]"#;

        let expected_result = r#"[{"tax":0},{"tax":0},{"tax":5000}]"#;

        let result = super::converter_raw_json(data);
        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_serializer_case_4() {
        let data = r#"[
            {"operation":"buy", "unit-cost":10, "quantity": 10000},
            {"operation":"buy", "unit-cost":25, "quantity": 5000},
            {"operation":"sell", "unit-cost":15, "quantity": 10000}
        ]"#;

        let expected_result = r#"[{"tax":0},{"tax":0},{"tax":0}]"#;

        let result = super::converter_raw_json(data);
        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_serializer_case_5() {
        let data = r#"[
            {"operation":"buy", "unit-cost":10, "quantity": 10000},
            {"operation":"buy", "unit-cost":25, "quantity": 5000},
            {"operation":"sell", "unit-cost":15, "quantity": 10000},
            {"operation":"sell", "unit-cost":25, "quantity": 5000}
        ]"#;

        let expected_result = r#"[{"tax":0},{"tax":0},{"tax":0},{"tax":10000}]"#;

        let result = super::converter_raw_json(data);
        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_serializer_case_6() {
        let data = r#"[
            {"operation":"buy", "unit-cost":10, "quantity": 10000},
            {"operation":"sell", "unit-cost":2, "quantity": 5000},
            {"operation":"sell", "unit-cost":20, "quantity": 2000},
            {"operation":"sell", "unit-cost":20, "quantity": 2000},
            {"operation":"sell", "unit-cost":25, "quantity": 1000}
        ]"#;

        let expected_result = r#"[{"tax":0},{"tax":0},{"tax":0},{"tax":0},{"tax":3000}]"#;

        let result = super::converter_raw_json(data);
        assert_eq!(result.unwrap(), expected_result);
    }
}
