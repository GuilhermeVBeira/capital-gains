use serde::{Deserialize, Serialize, Serializer};
mod config;
pub mod serializer;

use config::{MIN_TAX_VALUE, TAX_PERCENTAGE};

#[non_exhaustive]
#[derive(Debug, Serialize, Deserialize)]
pub enum OperationType {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Operation {
    operation: OperationType,
    #[serde(rename = "unit-cost")]
    unit_cost: f64,
    quantity: f64,
}

impl Operation {
    pub fn new(operation: OperationType, unit_cost: f64, quantity: f64) -> Self {
        Operation {
            operation,
            unit_cost,
            quantity,
        }
    }
    fn op_cost(&self) -> f64 {
        self.quantity * self.unit_cost
    }
    fn get_profit(&self, weighted_average_price: f64) -> f64 {
        (self.unit_cost - weighted_average_price) * self.quantity
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tax {
    #[serde(serialize_with = "serialize_tax")]
    tax: f64,
}

fn serialize_tax<S>(tax: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let truncated_tax = tax.trunc() as u64;
    serializer.serialize_u64(truncated_tax)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Portfolio {
    quantity: f64,
    cost: f64,
    deficit: f64,
    weighted_average_price: f64,
}

impl Default for Portfolio {
    fn default() -> Self {
        Portfolio {
            quantity: 0.0,
            cost: 0.0,
            deficit: 0.0,
            weighted_average_price: 0.0,
        }
    }
}

impl Portfolio {
    pub fn new() -> Self {
        Portfolio::default()
    }

    pub fn execute(&mut self, operation: Operation) -> Result<Tax, &str> {
        match operation.operation {
            OperationType::Buy => self.buy(operation),
            OperationType::Sell => self.sell(operation),
        }
    }

    fn calculate_tax(&self, operation_cost: f64, profit: f64) -> Tax {
        if operation_cost > MIN_TAX_VALUE {
            Tax {
                tax: profit * TAX_PERCENTAGE,
            }
        } else {
            Tax { tax: 0.0 }
        }
    }

    fn buy(&mut self, operation: Operation) -> Result<Tax, &str> {
        self.quantity += operation.quantity;
        self.cost += operation.quantity * operation.unit_cost;
        self.weighted_average_price = self.cost / self.quantity;
        Ok(Tax { tax: 0.0 })
    }

    fn sell(&mut self, operation: Operation) -> Result<Tax, &str> {
        if operation.quantity > self.quantity {
            return Err("Not enough quantity");
        }
        self.quantity -= operation.quantity;
        self.cost -= operation.quantity * operation.unit_cost;

        let has_loss = self.weighted_average_price > operation.unit_cost;
        let profit = operation.get_profit(self.weighted_average_price);
        if has_loss {
            // If the operation has loss, the deficit is increased
            self.deficit += profit.abs();
            Ok(Tax { tax: 0.0 })
        } else {
            // calculate how much profit was made
            if profit > self.deficit {
                // if profit is greater than deficit, then we have to pay taxes
                let tax = self.calculate_tax(operation.op_cost(), profit - self.deficit);
                self.deficit -= profit;
                Ok(tax)
            } else {
                // if profit is less than deficit, then we don't have to pay taxes
                self.deficit -= profit;
                Ok(Tax { tax: 0.0 })
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_cost() {
        let operation = Operation::new(OperationType::Sell, 12.34, 5.0);
        assert_eq!(operation.op_cost(), 61.7);
    }
    #[test]
    fn test_operation_get_profit() {
        let operation = Operation::new(OperationType::Sell, 12.34, 5.0);
        let weighted_average_price = 10.0;
        assert_eq!(operation.get_profit(weighted_average_price), 11.7);
    }

    #[test]
    fn test_tax_struct() {
        let tax = Tax { tax: 12.34 };
        let serialized = serde_json::to_string(&tax).unwrap();
        assert_eq!(serialized, r#"{"tax":12}"#);

        let deserialized: Tax = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.tax, 12.0);
    }

    #[test]
    fn test_case_1() {
        let mut portfolio = Portfolio::new();
        let operation = Operation::new(OperationType::Buy, 10.0, 100.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);

        let operation = Operation::new(OperationType::Sell, 15.0, 50.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);

        let operation = Operation::new(OperationType::Sell, 15.0, 50.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);
    }

    #[test]
    fn test_case_2() {
        let mut portfolio = Portfolio::new();
        let operation = Operation::new(OperationType::Buy, 10.0, 10000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);

        let operation = Operation::new(OperationType::Sell, 20.0, 5000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 10_000.0);

        let operation = Operation::new(OperationType::Sell, 5.0, 5000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);
    }

    #[test]
    fn test_case_3() {
        let mut portfolio = Portfolio::new();
        let operation = Operation::new(OperationType::Buy, 10.0, 10000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);

        let operation = Operation::new(OperationType::Sell, 5.0, 5000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);

        let operation = Operation::new(OperationType::Sell, 20.0, 5000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 5000.0);
    }

    #[test]
    fn test_case_4() {
        let mut portfolio = Portfolio::new();
        let operation = Operation::new(OperationType::Buy, 10.0, 10_000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);

        let operation = Operation::new(OperationType::Buy, 25.0, 5_000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);

        let operation = Operation::new(OperationType::Sell, 15.0, 10_000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);
    }

    #[test]
    fn test_case_5() {
        let mut portfolio = Portfolio::new();
        let operation = Operation::new(OperationType::Buy, 10.0, 10_000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);

        let operation = Operation::new(OperationType::Buy, 25.0, 5_000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);

        let operation = Operation::new(OperationType::Sell, 15.0, 10_000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);

        let operation = Operation::new(OperationType::Sell, 25.0, 5_000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 10_000.0);
    }

    #[test]
    fn test_case_6() {
        let mut portfolio = Portfolio::new();
        let operation = Operation::new(OperationType::Buy, 10.0, 10_000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);

        let operation = Operation::new(OperationType::Sell, 2.0, 5_000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);

        let operation = Operation::new(OperationType::Sell, 20.0, 2_000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);

        let operation = Operation::new(OperationType::Sell, 20.0, 2_000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 0.0);

        let operation = Operation::new(OperationType::Sell, 25.0, 1_000.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(tax.tax, 3_000.0);
    }
    #[test]
    fn test_buy_operation() {
        let mut portfolio = Portfolio::new();
        let operation = Operation::new(OperationType::Buy, 10.0, 10.0);
        let tax = portfolio.execute(operation).unwrap();
        assert_eq!(portfolio.quantity, 10.0);
        assert_eq!(portfolio.cost, 100.0);
        assert_eq!(portfolio.weighted_average_price, 10.0);
        assert_eq!(tax.tax, 0.0);
    }

    #[test]
    fn test_sell_operation_with_not_enough_quantity() {
        let mut portfolio = Portfolio::new();
        let operation = Operation::new(OperationType::Sell, 15.0, 20.0);
        let result = portfolio.execute(operation);
        assert!(result.is_err());
    }
}
