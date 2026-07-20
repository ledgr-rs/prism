use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use prism_core::types::{
    CapabilityProfile, DecisionReport, Explanation, ModelProfile, Policy, Prompt,
};
use prism_engine::engine::DecisionEngine;

use crate::sample_registry;

const EXPORT_FILE: &str = "prism-decision-report.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Prompt,
    PromptAnalysis,
    CapabilityExtraction,
    ModelRegistry,
    PolicyEvaluation,
    CandidateScoring,
    Recommendation,
    Explanation,
}

impl Stage {
    pub const ALL: [Stage; 8] = [
        Stage::Prompt,
        Stage::PromptAnalysis,
        Stage::CapabilityExtraction,
        Stage::ModelRegistry,
        Stage::PolicyEvaluation,
        Stage::CandidateScoring,
        Stage::Recommendation,
        Stage::Explanation,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Stage::Prompt => "Prompt",
            Stage::PromptAnalysis => "Prompt Analysis",
            Stage::CapabilityExtraction => "Capability Extraction",
            Stage::ModelRegistry => "Model Registry",
            Stage::PolicyEvaluation => "Policy Evaluation",
            Stage::CandidateScoring => "Candidate Scoring",
            Stage::Recommendation => "Recommendation",
            Stage::Explanation => "Explanation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Pipeline,
    Workspace,
    Details,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    Prompt,
    Recommendation,
    Details,
}

#[derive(Debug, Clone)]
pub struct ReplayState {
    pub active: bool,
    pub stage_index: usize,
    last_step: Instant,
}

impl ReplayState {
    fn new() -> Self {
        Self {
            active: false,
            stage_index: 0,
            last_step: Instant::now(),
        }
    }
}

pub struct App {
    pub prompt: String,
    pub cursor: usize,
    pub report: Option<DecisionReport>,
    pub models: Vec<ModelProfile>,
    pub policy: Policy,
    pub view: View,
    pub selected_stage: usize,
    pub active_panel: Panel,
    pub should_quit: bool,
    pub command_open: bool,
    pub command_query: String,
    pub help_open: bool,
    pub replay: ReplayState,
    pub status: String,
    pub last_eval: Option<Duration>,
    pub export_path: Option<PathBuf>,
    pub error: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            prompt: String::new(),
            cursor: 0,
            report: None,
            models: sample_registry::load_models(),
            policy: Policy {
                name: "Default deterministic policy".into(),
                rules: Vec::new(),
            },
            view: View::Prompt,
            selected_stage: Stage::Recommendation as usize,
            active_panel: Panel::Workspace,
            should_quit: false,
            command_open: false,
            command_query: String::new(),
            help_open: false,
            replay: ReplayState::new(),
            status: "Enter a prompt and press Enter to evaluate. Press / for commands.".into(),
            last_eval: None,
            export_path: None,
            error: None,
        }
    }

    pub fn current_stage(&self) -> Stage {
        Stage::ALL[self.selected_stage]
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.should_quit = true;
            return;
        }

        if self.help_open {
            match key.code {
                KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => self.help_open = false,
                _ => {}
            }
            return;
        }

        if self.command_open {
            self.handle_command_key(key);
            return;
        }

        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('?') => self.help_open = true,
            KeyCode::Char('/') => {
                self.command_open = true;
                self.command_query.clear();
                self.status = "Command palette open.".into();
            }
            KeyCode::Char('r') if self.report.is_some() => self.evaluate(),
            KeyCode::Char('e') if self.report.is_some() => self.export_report(),
            KeyCode::Enter if self.view == View::Prompt => self.evaluate(),
            KeyCode::Enter if self.view == View::Details && self.report.is_some() => {
                self.start_replay()
            }
            KeyCode::Up if self.view == View::Details && self.report.is_some() => {
                self.previous_stage()
            }
            KeyCode::Down if self.view == View::Details && self.report.is_some() => {
                self.next_stage()
            }
            KeyCode::Left if self.view == View::Details && self.report.is_some() => {
                self.previous_panel()
            }
            KeyCode::Right if self.view == View::Details && self.report.is_some() => {
                self.next_panel()
            }
            KeyCode::Tab if self.view == View::Details && self.report.is_some() => {
                self.next_stage()
            }
            KeyCode::BackTab if self.view == View::Details && self.report.is_some() => {
                self.previous_stage()
            }
            KeyCode::Esc if self.view == View::Details => {
                self.replay.active = false;
                self.view = View::Recommendation;
                self.status =
                    "Returned to recommendation. Press /details to inspect the decision.".into();
            }
            KeyCode::Esc if self.report.is_some() => {
                self.replay.active = false;
                self.status = "Press /details to inspect the decision.".into();
            }
            _ if self.view == View::Prompt => self.handle_prompt_key(key),
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        if !self.replay.active {
            return;
        }

        if self.replay.last_step.elapsed() >= Duration::from_millis(650) {
            if self.replay.stage_index + 1 >= Stage::ALL.len() {
                self.replay.active = false;
                self.selected_stage = Stage::ALL.len() - 1;
                self.status = "Replay complete. DecisionReport was not recomputed.".into();
            } else {
                self.replay.stage_index += 1;
                self.selected_stage = self.replay.stage_index;
                self.status = format!("Replaying: {}", self.current_stage().label());
                self.replay.last_step = Instant::now();
            }
        }
    }

    fn handle_prompt_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(ch) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    if ch == 'u' {
                        self.prompt.clear();
                        self.cursor = 0;
                    }
                } else {
                    let byte_index = byte_index(&self.prompt, self.cursor);
                    self.prompt.insert(byte_index, ch);
                    self.cursor += 1;
                }
            }
            KeyCode::Backspace if self.cursor > 0 => {
                let end = byte_index(&self.prompt, self.cursor);
                let start = byte_index(&self.prompt, self.cursor - 1);
                self.prompt.replace_range(start..end, "");
                self.cursor -= 1;
            }
            KeyCode::Delete if self.cursor < char_len(&self.prompt) => {
                let start = byte_index(&self.prompt, self.cursor);
                let end = byte_index(&self.prompt, self.cursor + 1);
                self.prompt.replace_range(start..end, "");
            }
            KeyCode::Left if self.cursor > 0 => self.cursor -= 1,
            KeyCode::Right if self.cursor < char_len(&self.prompt) => self.cursor += 1,
            KeyCode::Home => self.cursor = 0,
            KeyCode::End => self.cursor = char_len(&self.prompt),
            _ => {}
        }
    }

    fn handle_command_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.command_open = false;
                self.status = "Command palette closed.".into();
            }
            KeyCode::Enter => {
                self.apply_command();
                self.command_open = false;
            }
            KeyCode::Backspace => {
                self.command_query.pop();
            }
            KeyCode::Char(ch) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.command_query.push(ch);
            }
            _ => {}
        }
    }

    fn apply_command(&mut self) {
        let command = self
            .command_query
            .trim()
            .trim_start_matches('/')
            .to_lowercase();
        if command.is_empty() {
            return;
        }

        match command.as_str() {
            "details" | "pipeline" => self.open_details(Stage::Recommendation),
            "models" => self.open_details(Stage::ModelRegistry),
            "capabilities" => self.open_details(Stage::CapabilityExtraction),
            "policy" => self.open_details(Stage::PolicyEvaluation),
            "export" => self.export_report(),
            "settings" => {
                self.status = "Settings are not configurable in this build.".into();
            }
            "help" => {
                self.help_open = true;
                self.status = "Help open.".into();
            }
            other => {
                if let Some(stage) = Stage::ALL
                    .iter()
                    .copied()
                    .find(|stage| stage.label().to_lowercase().contains(other))
                {
                    self.open_details(stage);
                } else {
                    self.status = format!("Unknown command: /{other}");
                }
            }
        }
    }

    fn open_details(&mut self, stage: Stage) {
        if self.report.is_none() {
            self.status = "Evaluate a prompt before opening decision details.".into();
            return;
        }

        self.view = View::Details;
        self.selected_stage = stage as usize;
        self.active_panel = Panel::Pipeline;
        self.status = "Decision explorer open. Esc returns to the recommendation.".into();
    }

    fn evaluate(&mut self) {
        self.error = None;
        self.export_path = None;

        if self.prompt.trim().is_empty() {
            self.error = Some("Prompt cannot be empty.".into());
            self.status = "Enter a prompt before evaluating.".into();
            return;
        }

        let engine = DecisionEngine::default();
        let prompt = Prompt {
            text: self.prompt.trim().into(),
        };
        let start = Instant::now();

        match engine.evaluate(&prompt, &self.models, &self.policy) {
            Ok(report) => {
                self.last_eval = Some(start.elapsed());
                self.report = Some(report);
                self.view = View::Recommendation;
                self.selected_stage = Stage::Recommendation as usize;
                self.active_panel = Panel::Workspace;
                self.replay.active = false;
                self.status =
                    "Recommendation ready. Press /details to inspect the decision.".into();
            }
            Err(err) => {
                self.error = Some(err.to_string());
                self.status = "Evaluation failed.".into();
            }
        }
    }

    fn export_report(&mut self) {
        let Some(report) = &self.report else {
            self.status = "Evaluate a prompt before exporting a DecisionReport.".into();
            return;
        };

        let json = report_json(report, self.last_eval);
        let path = PathBuf::from(EXPORT_FILE);
        match fs::write(&path, json) {
            Ok(()) => {
                self.export_path = Some(path.clone());
                self.status = format!("Exported DecisionReport to {}.", path.display());
            }
            Err(err) => {
                self.error = Some(err.to_string());
                self.status = "Export failed.".into();
            }
        }
    }

    fn start_replay(&mut self) {
        self.replay.active = true;
        self.replay.stage_index = 0;
        self.replay.last_step = Instant::now();
        self.selected_stage = 0;
        self.status = "Replay started from computed DecisionReport.".into();
    }

    fn next_stage(&mut self) {
        self.selected_stage = (self.selected_stage + 1).min(Stage::ALL.len() - 1);
    }

    fn previous_stage(&mut self) {
        self.selected_stage = self.selected_stage.saturating_sub(1);
    }

    fn next_panel(&mut self) {
        self.active_panel = match self.active_panel {
            Panel::Pipeline => Panel::Workspace,
            Panel::Workspace => Panel::Details,
            Panel::Details => Panel::Details,
        };
    }

    fn previous_panel(&mut self) {
        self.active_panel = match self.active_panel {
            Panel::Pipeline => Panel::Pipeline,
            Panel::Workspace => Panel::Pipeline,
            Panel::Details => Panel::Workspace,
        };
    }
}

pub fn report_json(report: &DecisionReport, eval_duration: Option<Duration>) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    push_json_field(&mut out, 1, "prompt", &report.prompt.text, true);
    out.push_str("  \"capabilities\": [\n");
    for (index, req) in report.capabilities.requirements.iter().enumerate() {
        out.push_str("    {\n");
        push_json_field(&mut out, 3, "capability", &req.capability.to_string(), true);
        push_json_field(
            &mut out,
            3,
            "priority",
            &format!("{:?}", req.priority),
            true,
        );
        push_json_number(&mut out, 3, "weight", req.weight, true);
        push_json_number(&mut out, 3, "confidence", req.confidence, true);
        push_json_field(&mut out, 3, "reason", &req.reason, false);
        out.push_str(if index + 1 == report.capabilities.requirements.len() {
            "    }\n"
        } else {
            "    },\n"
        });
    }
    out.push_str("  ],\n");
    out.push_str("  \"prompt_profile\": {\n");
    push_json_field(
        &mut out,
        2,
        "task_category",
        &report.prompt_profile.derived.task_category,
        true,
    );
    push_json_field(
        &mut out,
        2,
        "complexity",
        &report.prompt_profile.derived.complexity,
        true,
    );
    push_json_field(
        &mut out,
        2,
        "reasoning_depth",
        &report.prompt_profile.derived.reasoning_depth,
        true,
    );
    push_json_number(
        &mut out,
        2,
        "word_count",
        report.prompt_profile.intrinsic.word_count as f64,
        false,
    );
    out.push_str("  },\n");
    out.push_str("  \"candidate_scores\": [\n");
    for (index, (model, score)) in report.scored_candidates.iter().enumerate() {
        out.push_str("    {\n");
        push_json_field(&mut out, 3, "model_id", &model.identity.id, true);
        push_json_number(&mut out, 3, "score", score.final_score, true);
        push_json_number(&mut out, 3, "confidence", score.confidence, false);
        out.push_str(if index + 1 == report.scored_candidates.len() {
            "    }\n"
        } else {
            "    },\n"
        });
    }
    out.push_str("  ],\n");
    out.push_str("  \"recommendation\": {\n");
    push_json_field(
        &mut out,
        2,
        "model_id",
        &report.recommendation.model.identity.id,
        true,
    );
    push_json_field(
        &mut out,
        2,
        "model_name",
        &report.recommendation.model.identity.name,
        true,
    );
    push_json_number(&mut out, 2, "score", report.recommendation.score, false);
    out.push_str("  },\n");
    push_explanation_json(&mut out, &report.explanation);
    out.push_str(",\n  \"run_metadata\": {\n");
    push_json_number(
        &mut out,
        2,
        "models",
        report.model_registry.len() as f64,
        true,
    );
    push_json_field(&mut out, 2, "policy", &report.policy.name, true);
    push_json_number(
        &mut out,
        2,
        "evaluation_ms",
        eval_duration
            .map(|d| d.as_secs_f64() * 1000.0)
            .unwrap_or(0.0),
        false,
    );
    out.push_str("  }\n");
    out.push_str("}\n");
    out
}

fn push_explanation_json(out: &mut String, explanation: &Explanation) {
    out.push_str("  \"explanation\": {\n");
    push_json_field(out, 2, "summary", &explanation.summary, true);
    push_json_number(out, 2, "confidence", explanation.confidence, false);
    out.push_str("  }");
}

fn push_json_field(out: &mut String, indent: usize, name: &str, value: &str, comma: bool) {
    out.push_str(&"  ".repeat(indent));
    out.push('"');
    out.push_str(name);
    out.push_str("\": \"");
    out.push_str(&json_escape(value));
    out.push('"');
    if comma {
        out.push(',');
    }
    out.push('\n');
}

fn push_json_number(out: &mut String, indent: usize, name: &str, value: f64, comma: bool) {
    out.push_str(&"  ".repeat(indent));
    out.push('"');
    out.push_str(name);
    out.push_str("\": ");
    out.push_str(&format!("{value:.3}"));
    if comma {
        out.push(',');
    }
    out.push('\n');
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn char_len(value: &str) -> usize {
    value.chars().count()
}

fn byte_index(value: &str, char_index: usize) -> usize {
    value
        .char_indices()
        .nth(char_index)
        .map(|(index, _)| index)
        .unwrap_or(value.len())
}

pub fn capability_counts(profile: &CapabilityProfile) -> (usize, usize, usize) {
    profile
        .requirements
        .iter()
        .fold(
            (0, 0, 0),
            |(required, preferred, optional), req| match format!("{:?}", req.priority).as_str() {
                "Required" => (required + 1, preferred, optional),
                "Preferred" => (required, preferred + 1, optional),
                _ => (required, preferred, optional + 1),
            },
        )
}
