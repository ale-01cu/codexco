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

        let mut add = |name: &str, input: f64, cached: f64, output: f64| {
            rates.insert(
                name.to_string(),
                ModelRates {
                    input_per_m: input,
                    cached_input_per_m: cached,
                    output_per_m: output,
                },
            );
        };

        // GPT-6 Series
        add("gpt-6-astra", 10.00, 1.00, 50.00);
        add("gpt-6.1-sol", 2.00, 0.10, 10.00);
        add("gpt-6.1-terra", 1.00, 0.05, 5.00);
        add("gpt-6-luna", 0.10, 0.01, 0.50);
        add("gpt-6-sol", 2.00, 0.20, 10.00);

        // GPT-5.6 Series
        add("gpt-5.6-sol", 4.00, 0.40, 20.00);
        add("gpt-5.6-terra", 2.00, 0.20, 12.00);
        add("gpt-5.6-luna", 0.20, 0.02, 1.20);

        // GPT-5.5 Series
        add("gpt-5.5-pro", 30.00, 30.00, 180.00);
        add("gpt-5.5", 5.00, 0.50, 30.00);

        // GPT-5.4 Series
        add("gpt-5.4-pro", 30.00, 30.00, 180.00);
        add("gpt-5.4-mini", 0.75, 0.075, 4.50);
        add("gpt-5.4-nano", 0.20, 0.02, 1.25);
        add("gpt-5.4", 2.50, 0.25, 15.00);

        // GPT-5.2 Series
        add("gpt-5.2-pro", 21.00, 21.00, 168.00);
        add("gpt-5.2", 1.75, 0.175, 14.00);

        // GPT-5.1 Series
        add("gpt-5.1", 1.25, 0.125, 10.00);

        // GPT-5 Series
        add("gpt-5-pro", 15.00, 15.00, 120.00);
        add("gpt-5-mini", 0.25, 0.025, 2.00);
        add("gpt-5-nano", 0.05, 0.005, 0.40);
        add("gpt-5", 1.25, 0.125, 10.00);

        // GPT-4.1 Series
        add("gpt-4.1-mini", 0.40, 0.10, 1.60);
        add("gpt-4.1-nano", 0.10, 0.025, 0.40);
        add("gpt-4.1", 2.00, 0.50, 8.00);

        // GPT-4o Series
        add("gpt-4o-mini", 0.15, 0.075, 0.60);
        add("gpt-4o", 2.50, 1.25, 10.00);

        // Reasoning models
        add("o1", 15.00, 7.50, 60.00);
        add("o3-mini", 1.10, 0.55, 4.40);

        let default_rates = ModelRates {
            input_per_m: 2.00,
            cached_input_per_m: 0.20,
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
        // Búsqueda exacta primero
        if let Some(rates) = self.rates.get(&normalized) {
            return rates;
        }

        // Búsqueda por prefijo más específico a menos específico
        let mut matches: Vec<(&String, &ModelRates)> = self
            .rates
            .iter()
            .filter(|(key, _)| normalized.contains(*key))
            .collect();

        matches.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

        if let Some((_, rates)) = matches.first() {
            rates
        } else {
            &self.default_rates
        }
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
