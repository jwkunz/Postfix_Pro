//! Calculator operations for the statistics panel panel.

use super::*;

impl Calculator {
    /// Executes the `mean` operation.
    pub fn mean(&mut self) -> Result<(), CalcError> {
        self.apply_stat_op(Self::matrix_mean, "mean")
    }

    /// Executes the `mode` operation.
    pub fn mode(&mut self) -> Result<(), CalcError> {
        self.apply_stat_op(Self::matrix_mode, "mode")
    }

    /// Executes the `variance` operation.
    pub fn variance(&mut self) -> Result<(), CalcError> {
        self.apply_stat_op(Self::matrix_variance, "variance")
    }

    /// Executes the `std_dev` operation.
    pub fn std_dev(&mut self) -> Result<(), CalcError> {
        self.apply_stat_op(Self::matrix_std_dev, "std_dev")
    }

    /// Executes the `max_value` operation.
    pub fn max_value(&mut self) -> Result<(), CalcError> {
        self.apply_stat_op(Self::matrix_max, "max")
    }

    /// Executes the `min_value` operation.
    pub fn min_value(&mut self) -> Result<(), CalcError> {
        self.apply_stat_op(Self::matrix_min, "min")
    }

}
