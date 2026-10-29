use crate::models::{SessionMetaPayload, SessionSummary, TokenUsage, TokenUsageInfo};
use crate::pricing::PricingRegistry;
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct RolloutScanner {
    codex_home: PathBuf,
    pricing: PricingRegistry,
}

impl RolloutScanner {
    pub fn new() -> Option<Self> {
        let home = dirs::home_dir()?;
        let codex_home = home.join(".codex");
        if codex_home.exists() {
            Some(Self {
                codex_home,
                pricing: PricingRegistry::default(),
            })
        } else {
            None
        }
    }

    pub fn with_custom_path(codex_home: PathBuf) -> Self {
        Self {
            codex_home,
            pricing: PricingRegistry::default(),
        }
    }

    pub fn sessions_dir(&self) -> PathBuf {
        self.codex_home.join("sessions")
    }

    /// Encuentra todos los archivos rollout-*.jsonl ordenados por fecha de modificación descendente
    pub fn list_rollout_files(&self) -> Vec<PathBuf> {
        let sessions_dir = self.sessions_dir();
        if !sessions_dir.exists() {
            return Vec::new();
        }

        let mut files: Vec<PathBuf> = WalkDir::new(sessions_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                let name = e.file_name().to_string_lossy();
                name.starts_with("rollout-") && name.ends_with(".jsonl")
            })
            .map(|e| e.into_path())
            .collect();

        // Ordenar por tiempo de modificación (el más reciente primero)
        files.sort_by(|a, b| {
            let meta_a = a.metadata().and_then(|m| m.modified()).ok();
            let meta_b = b.metadata().and_then(|m| m.modified()).ok();
            meta_b.cmp(&meta_a)
        });

        files
    }

    /// Parsea un archivo de rollout individual y extrae el resumen de la sesión
    pub fn parse_rollout_file(&self, path: &Path) -> Option<SessionSummary> {
        let file = File::open(path).ok()?;
        let reader = BufReader::new(file);

        let mut session_id = String::new();
        let mut timestamp = None;
        let mut cwd = None;
        let mut model = "gpt-4o".to_string();
        let mut latest_usage = TokenUsage::default();

        for line in reader.lines().flatten() {
            if line.trim().is_empty() {
                continue;
            }

            if let Ok(val) = serde_json::from_str::<Value>(&line) {
                let event_type = val.get("type").and_then(|v| v.as_str()).unwrap_or("");

                if event_type == "session_meta" {
                    if let Some(payload) = val.get("payload") {
                        if let Ok(meta) = serde_json::from_value::<SessionMetaPayload>(payload.clone()) {
                            if let Some(sid) = meta.session_id.or(meta.id) {
                                session_id = sid;
                            }
                            timestamp = meta.timestamp;
                            cwd = meta.cwd;
                        }
                    }
                } else if event_type == "event_msg" {
                    if let Some(payload) = val.get("payload") {
                        let msg_type = payload.get("type").and_then(|v| v.as_str()).unwrap_or("");
                        if msg_type == "token_count" {
                            if let Some(info_val) = payload.get("info") {
                                if let Ok(info) = serde_json::from_value::<TokenUsageInfo>(info_val.clone()) {
                                    latest_usage = info.total_token_usage;
                                }
                            }
                        }
                    }
                }

                // Intenta detectar el modelo si aparece en configuraciones del turno
                if let Some(m) = val.pointer("/payload/model").and_then(|v| v.as_str()) {
                    model = m.to_string();
                } else if let Some(m) = val.pointer("/payload/settings/model").and_then(|v| v.as_str()) {
                    model = m.to_string();
                }
            }
        }

        if session_id.is_empty() {
            session_id = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown-session".to_string());
        }

        let cost = self.pricing.calculate_cost(&model, &latest_usage);

        Some(SessionSummary {
            session_id,
            path: path.to_path_buf(),
            timestamp,
            cwd,
            model,
            token_usage: latest_usage,
            estimated_cost_usd: cost,
        })
    }

    /// Obtiene la sesión activa o más reciente
    pub fn get_latest_session(&self) -> Option<SessionSummary> {
        let files = self.list_rollout_files();
        for file in files {
            if let Some(summary) = self.parse_rollout_file(&file) {
                return Some(summary);
            }
        }
        None
    }

    /// Obtiene el historial de todas las sesiones ordenadas de más reciente a más antigua
    pub fn get_session_history(&self, limit: usize) -> Vec<SessionSummary> {
        let files = self.list_rollout_files();
        files
            .into_iter()
            .take(limit)
            .filter_map(|file| self.parse_rollout_file(&file))
            .collect()
    }
}
