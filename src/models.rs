use serde::{Deserialize, Serialize};
use std::ops::AddAssign;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenUsage {
    #[serde(default)]
    pub input_tokens: i64,
    #[serde(default)]
    pub cached_input_tokens: i64,
    #[serde(default)]
    pub cache_write_input_tokens: i64,
    #[serde(default)]
    pub output_tokens: i64,
    #[serde(default)]
    pub reasoning_output_tokens: i64,
    #[serde(default)]
    pub total_tokens: i64,
}

impl TokenUsage {
    pub fn is_zero(&self) -> bool {
        self.total_tokens == 0 && self.input_tokens == 0 && self.output_tokens == 0
    }

    pub fn cached_input(&self) -> i64 {
        self.cached_input_tokens.max(0)
    }

    pub fn non_cached_input(&self) -> i64 {
        (self.input_tokens - self.cached_input()).max(0)
    }
}

impl AddAssign for TokenUsage {
    fn add_assign(&mut self, rhs: Self) {
        self.input_tokens += rhs.input_tokens;
        self.cached_input_tokens += rhs.cached_input_tokens;
        self.cache_write_input_tokens += rhs.cache_write_input_tokens;
        self.output_tokens += rhs.output_tokens;
        self.reasoning_output_tokens += rhs.reasoning_output_tokens;
        self.total_tokens += rhs.total_tokens;
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TokenUsageInfo {
    pub total_token_usage: TokenUsage,
    pub last_token_usage: TokenUsage,
    #[serde(default)]
    pub model_context_window: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionMetaPayload {
    pub session_id: Option<String>,
    pub id: Option<String>,
    pub timestamp: Option<String>,
    pub cwd: Option<String>,
    pub originator: Option<String>,
    pub cli_version: Option<String>,
    pub model_provider: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ModelUsageSummary {
    pub model: String,
    pub token_usage: TokenUsage,
    pub cost_usd: f64,
}

#[derive(Debug, Clone)]
pub struct SessionSummary {
    pub session_id: String,
    pub path: std::path::PathBuf,
    pub timestamp: Option<String>,
    pub cwd: Option<String>,
    pub models_used: Vec<String>,
    pub breakdown: Vec<ModelUsageSummary>,
    pub token_usage: TokenUsage,
    pub estimated_cost_usd: f64,
}
