use crate::models::TokenUsage;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ModelRates {
    /// Cost per 1M non-cached input tokens in USD
    pub input_per_m: f64,
    /// Cost per 1M cached input tokens in USD
    pub cached_input_per_m: f64,
    /// Cost per 1M output tokens in USD
    pub output_per_m: f64,
}

pub struct PricingRegistry {
    rates: HashMap<String, ModelRates>,
    default_rates: ModelRates,
}

impl Default for PricingRegistry {
    fn default() -> Self {
        let mut rates = HashMap::new();

        // Precios estándar de referencia ($ / 1,000,000 tokens)
        rates.insert(
            "gpt-4o".to_string(),
            ModelRates {
                input_per_m: 2.50,
                cached_input_per_m: 1.25,
                output_per_m: 10.00,
            },
        );
        rates.insert(
            "gpt-4o-mini".to_string(),
            ModelRates {
                input_per_m: 0.15,
                cached_input_per_m: 0.075,
                output_per_m: 0.60,
            },
        );
        rates.insert(
            "o1".to_string(),
            ModelRates {
                input_per_m: 15.00,
                cached_input_per_m: 7.50,
                output_per_m: 60.00,
            },
        );
        rates.insert(
            "o3-mini".to_string(),
            ModelRates {
                input_per_m: 1.10,
                cached_input_per_m: 0.55,
                output_per_m: 4.40,
            },
        );
        rates.insert(
            "claude-3-5-sonnet".to_string(),
            ModelRates {
                input_per_m: 3.00,
                cached_input_per_m: 0.30,
                output_per_m: 15.00,
            },
        );

        let default_rates = ModelRates {
            input_per_m: 2.50,
            cached_input_per_m: 1.25,
            output_per_m: 10.00,
        };

        Self {
            rates,
            default_rates,
        }
    }
}

impl PricingRegistry {
    pub fn get_rates(&self, model: &str) -> &ModelRates {
        let normalized = model.to_lowercase();
        for (key, val) in &self.rates {
            if normalized.contains(key) {
                return val;
            }
        }
        &self.default_rates
    }

    pub fn calculate_cost(&self, model: &str, usage: &TokenUsage) -> f64 {
        let rates = self.get_rates(model);
        let non_cached = usage.non_cached_input() as f64;
        let cached = usage.cached_input() as f64;
        let output = usage.output_tokens.max(0) as f64;

        let input_cost = (non_cached * rates.input_per_m) / 1_000_000.0;
        let cached_cost = (cached * rates.cached_input_per_m) / 1_000_000.0;
        let output_cost = (output * rates.output_per_m) / 1_000_000.0;

        input_cost + cached_cost + output_cost
    }
}
