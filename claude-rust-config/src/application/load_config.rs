use std::collections::HashMap;
use std::path::Path;

use crate::domain::config::Settings;

pub fn load_config() -> Settings {
    let global = load_file(&global_settings_path());
    let project = load_file(&project_settings_path());
    merge(global, project)
}

pub fn save_model(model: &str) -> std::io::Result<()> {
    let path = global_settings_path();
    if let Some(parent) = path.parent() { std::fs::create_dir_all(parent)?; }

    // Use serde_json::Value as intermediate to preserve unknown fields
    let mut json: serde_json::Value = match std::fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content)
            .unwrap_or_else(|_| serde_json::Value::Object(Default::default())),
        Err(_) => serde_json::Value::Object(Default::default()),
    };

    if let Some(obj) = json.as_object_mut() {
        obj.insert("model".to_string(), serde_json::Value::String(model.to_string()));
    }

    let pretty = serde_json::to_string_pretty(&json).unwrap();
    std::fs::write(path, pretty)
}

/// Resolve a model tier name ("sonnet", "opus", "haiku") to an actual model ID.
/// Checks ANTHROPIC_DEFAULT_SONNET_MODEL, ANTHROPIC_DEFAULT_OPUS_MODEL,
/// ANTHROPIC_DEFAULT_HAIKU_MODEL from settings.json env. Falls back to Claude defaults.
pub fn resolve_model_tier(tier: &str, env: &HashMap<String, String>) -> String {
    match tier {
        "sonnet" => env.get("ANTHROPIC_DEFAULT_SONNET_MODEL")
            .cloned()
            .unwrap_or_else(|| "claude-sonnet-4-6".to_string()),
        "opus" => env.get("ANTHROPIC_DEFAULT_OPUS_MODEL")
            .cloned()
            .unwrap_or_else(|| "claude-opus-4-6".to_string()),
        "haiku" => env.get("ANTHROPIC_DEFAULT_HAIKU_MODEL")
            .cloned()
            .unwrap_or_else(|| "claude-haiku-4-5-20251001".to_string()),
        other => other.to_string(),
    }
}

/// Get the default model for this configuration (resolves "sonnet" tier from env).
pub fn default_model(env: &HashMap<String, String>) -> String {
    resolve_model_tier("sonnet", env)
}

fn global_settings_path() -> std::path::PathBuf {
    let home = super::platform::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    home.join(".claude").join("settings.json")
}

fn project_settings_path() -> std::path::PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join(".claude")
        .join("settings.json")
}

fn load_file(path: &Path) -> Settings {
    match std::fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Settings::default(),
    }
}

fn merge(global: Settings, project: Settings) -> Settings {
    let mut allow = global.permissions.allow;
    allow.extend(project.permissions.allow);

    let mut deny = global.permissions.deny;
    deny.extend(project.permissions.deny);

    let mut pre_tool_use = global.hooks.pre_tool_use;
    pre_tool_use.extend(project.hooks.pre_tool_use);
    let mut post_tool_use = global.hooks.post_tool_use;
    post_tool_use.extend(project.hooks.post_tool_use);
    let mut stop = global.hooks.stop;
    stop.extend(project.hooks.stop);
    let mut session_start = global.hooks.session_start;
    session_start.extend(project.hooks.session_start);

    let mut env = global.env.clone();
    env.extend(project.env.clone());

    Settings {
        permissions: crate::domain::config::PermissionSettings { allow, deny },
        model: project.model.or(global.model),
        env,
        hooks: crate::domain::config::HooksConfig { pre_tool_use, post_tool_use, stop, session_start },
        max_turns: project.max_turns.or(global.max_turns),
        max_tokens: project.max_tokens.or(global.max_tokens),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::config::{PermissionSettings, Settings};

    #[test]
    fn test_merge_concatenates_permissions() {
        let global = Settings {
            permissions: PermissionSettings {
                allow: vec!["read".into(), "glob".into()],
                deny: vec!["bash(rm -rf *)".into()],
            },
            model: Some("global-model".into()),
            ..Default::default()
        };
        let project = Settings {
            permissions: PermissionSettings {
                allow: vec!["grep".into()],
                deny: vec![],
            },
            model: None,
            ..Default::default()
        };
        let merged = merge(global, project);
        assert_eq!(merged.permissions.allow, vec!["read", "glob", "grep"]);
        assert_eq!(merged.permissions.deny, vec!["bash(rm -rf *)"]);
        assert_eq!(merged.model, Some("global-model".into()));
    }

    #[test]
    fn test_merge_project_model_overrides() {
        let global = Settings {
            model: Some("global-model".into()),
            ..Default::default()
        };
        let project = Settings {
            model: Some("project-model".into()),
            ..Default::default()
        };
        let merged = merge(global, project);
        assert_eq!(merged.model, Some("project-model".into()));
    }

    #[test]
    fn test_save_model_preserves_unknown_fields() {
        let dir = std::env::temp_dir().join("claude-rust-test-save-model");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("settings.json");

        // Write a settings.json with a field not in the Settings struct
        let original = serde_json::json!({
            "model": "old-model",
            "permissions": { "allow": ["read"], "deny": [] },
            "customUnknownField": true,
            "someNested": { "key": "value" }
        });
        std::fs::write(&path, serde_json::to_string_pretty(&original).unwrap()).unwrap();

        // Temporarily override the global path
        let content = std::fs::read_to_string(&path).unwrap();
        let mut json: serde_json::Value = serde_json::from_str(&content).unwrap();
        if let Some(obj) = json.as_object_mut() {
            obj.insert("model".to_string(), serde_json::Value::String("new-model".to_string()));
        }
        std::fs::write(&path, serde_json::to_string_pretty(&json).unwrap()).unwrap();

        // Verify unknown fields are preserved
        let result: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(result["model"], "new-model");
        assert_eq!(result["customUnknownField"], true);
        assert_eq!(result["someNested"]["key"], "value");
        assert_eq!(result["permissions"]["allow"], serde_json::json!(["read"]));

        std::fs::remove_dir_all(&dir).unwrap();
    }
}
