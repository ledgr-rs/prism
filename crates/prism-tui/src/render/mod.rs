use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Gauge, List, ListItem, Paragraph, Wrap};

use crate::app::{App, Panel, Stage, capability_counts};
use crate::theme;

pub fn draw(frame: &mut Frame<'_>, app: &App) {
    let area = frame.area();
    frame.render_widget(Clear, area);

    if app.report.is_none() {
        draw_prompt_entry(frame, area, app);
    } else {
        draw_explorer(frame, area, app);
    }

    if app.help_open {
        draw_help(frame, area);
    }
}

fn draw_prompt_entry(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let shell = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(6),
            Constraint::Length(1),
        ])
        .split(area);

    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            " PRISM ",
            Style::default().fg(Color::Black).bg(theme::PURPLE),
        ),
        Span::styled("  Explainable model routing", theme::title()),
        
    ]))
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(theme::BORDER).bg(theme::BG)),
    )
    .style(theme::base());
    frame.render_widget(header, shell[0]);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)])
        .split(shell[1]);
    draw_sidebar(frame, columns[0], app);

    let input_area = Rect::new(
        columns[1].x + 2,
        columns[1].y + 1,
        columns[1].width.saturating_sub(4),
        5,
    );
    let mut lines = vec![Line::from(vec![Span::styled(
        if app.prompt.is_empty() {
            "Type or paste a prompt, then press Enter."
        } else {
            app.prompt.as_str()
        },
        Style::default()
            .fg(if app.prompt.is_empty() {
                theme::MUTED
            } else {
                theme::TEXT
            })
            .bg(theme::PANEL),
    )])];
    if let Some(error) = &app.error {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            error.as_str(),
            Style::default().fg(theme::RED).bg(theme::PANEL),
        )]));
    }
    let prompt = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Prompt ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme::ORANGE).bg(theme::PANEL))
                .style(theme::panel()),
        )
        .style(theme::panel())
        .wrap(Wrap { trim: false });
    frame.render_widget(prompt, input_area);

    let cursor_x = input_area
        .x
        .saturating_add(2)
        .saturating_add(app.cursor as u16)
        .min(
            input_area
                .x
                .saturating_add(input_area.width.saturating_sub(2)),
        );
    let cursor_y = input_area.y.saturating_add(1);
    frame.set_cursor_position((cursor_x, cursor_y));

    let status = Paragraph::new(app.status.as_str())
        .style(Style::default().fg(theme::MUTED).bg(theme::BG))
        .alignment(Alignment::Center);
    frame.render_widget(status, shell[2]);
}

fn draw_explorer(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let shell = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(1),
        ])
        .split(area);

    draw_header(frame, shell[0], app);

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(45),
            Constraint::Percentage(30),
        ])
        .split(shell[1]);

    draw_sidebar(frame, columns[0], app);
    draw_workspace(frame, columns[1], app);
    draw_details(frame, columns[2], app);
    draw_status(frame, shell[2], app);

    if app.search_open {
        draw_search(frame, area, app);
    }
}

fn draw_header(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let Some(report) = &app.report else {
        return;
    };

    let selected = &report.recommendation.model.identity;
    let title = Line::from(vec![
        Span::styled(
            " PRISM ",
            Style::default().fg(Color::Black).bg(theme::PURPLE),
        ),
        Span::styled("  ", theme::base()),
        Span::styled(
            "Explainable decisions",
            Style::default()
                .fg(theme::TEXT)
                .bg(theme::BG)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  ", theme::base()),
        Span::styled(
            format!("selected: {} ({})", selected.name, selected.id),
            Style::default().fg(theme::MUTED).bg(theme::BG),
        ),
    ]);

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(theme::BORDER).bg(theme::BG))
        .style(theme::base());
    let paragraph = Paragraph::new(title).block(block);
    frame.render_widget(paragraph, area);
}

fn draw_sidebar(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let items = Stage::ALL
        .iter()
        .enumerate()
        .map(|(index, stage)| {
            let complete =
                app.report.is_some() && (!app.replay.active || index <= app.replay.stage_index);
            let active = app.replay.active && index == app.replay.stage_index;
            let marker = if active {
                ">>"
            } else if complete {
                "ok"
            } else {
                ".."
            };
            let accent = if index == app.selected_stage {
                theme::selected()
            } else if complete {
                Style::default().fg(theme::GREEN).bg(theme::PANEL)
            } else {
                Style::default().fg(theme::MUTED).bg(theme::PANEL)
            };
            ListItem::new(Line::from(vec![
                Span::styled(marker, accent),
                Span::raw(" "),
                Span::styled(
                    stage.label(),
                    Style::default().fg(theme::TEXT).bg(theme::PANEL),
                ),
            ]))
        })
        .collect::<Vec<_>>();

    let border = active_border(app.active_panel == Panel::Pipeline);
    let list = List::new(items)
        .block(
            Block::default()
                .title(" Pipeline ")
                .borders(Borders::ALL)
                .border_style(border)
                .style(theme::panel()),
        )
        .style(theme::panel());
    frame.render_widget(list, area);
}

fn draw_workspace(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let Some(report) = &app.report else {
        return;
    };

    let lines = match app.current_stage() {
        Stage::Prompt => prompt_lines(app),
        Stage::PromptAnalysis => prompt_analysis_lines(app),
        Stage::CapabilityExtraction => capability_lines(app),
        Stage::ModelRegistry => registry_lines(app),
        Stage::PolicyEvaluation => policy_lines(app),
        Stage::CandidateScoring => scoring_lines(app),
        Stage::Recommendation => recommendation_lines(app),
        Stage::Explanation => explanation_lines(app),
    };

    let title = format!(" {} ", app.current_stage().label());
    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(active_border(app.active_panel == Panel::Workspace))
                .style(theme::panel()),
        )
        .style(theme::panel())
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);

    let score_area = Rect::new(
        area.x + 2,
        area.y + area.height.saturating_sub(3),
        area.width.saturating_sub(4),
        1,
    );
    if matches!(
        app.current_stage(),
        Stage::CandidateScoring | Stage::Recommendation
    ) {
        let ratio = report.recommendation.score.clamp(0.0, 1.0);
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(theme::ORANGE).bg(theme::PANEL))
            .ratio(ratio)
            .label(format!("score {:.2}", report.recommendation.score));
        frame.render_widget(gauge, score_area);
    }
}

fn draw_details(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let Some(report) = &app.report else {
        return;
    };

    let mut lines = Vec::new();
    lines.push(Line::from(vec![Span::styled(
        "Metadata",
        Style::default()
            .fg(theme::ORANGE)
            .bg(theme::PANEL)
            .add_modifier(Modifier::BOLD),
    )]));
    lines.push(Line::from(""));
    lines.extend(match app.current_stage() {
        Stage::Prompt => vec![
            kv("chars", report.prompt.text.chars().count().to_string()),
            kv("models", report.model_registry.len().to_string()),
            kv("engine", "deterministic".to_string()),
        ],
        Stage::PromptAnalysis => prompt_analysis_detail_lines(app),
        Stage::CapabilityExtraction => report
            .capabilities
            .requirements
            .iter()
            .flat_map(|req| {
                vec![
                    kv("capability", req.capability.to_string()),
                    kv("priority", format!("{:?}", req.priority)),
                    kv("confidence", format!("{:.2}", req.confidence)),
                    Line::from(""),
                ]
            })
            .collect(),
        Stage::ModelRegistry => report
            .model_registry
            .iter()
            .map(|model| {
                kv(
                    model.identity.id.as_str(),
                    format!("{} caps", model.capabilities.len()),
                )
            })
            .collect(),
        Stage::PolicyEvaluation => report
            .explanation
            .policy_decisions
            .iter()
            .map(|decision| kv(decision.policy.as_str(), decision.reason.clone()))
            .collect(),
        Stage::CandidateScoring => report
            .scored_candidates
            .iter()
            .map(|(model, score)| {
                kv(
                    model.identity.id.as_str(),
                    format!("{:.2} conf {:.2}", score.final_score, score.confidence),
                )
            })
            .collect(),
        Stage::Recommendation => selected_model_lines(app),
        Stage::Explanation => report
            .explanation
            .evidence
            .iter()
            .map(|evidence| kv(evidence.source.as_str(), evidence.detail.clone()))
            .collect(),
    });

    if lines.len() <= 2 {
        lines.push(Line::from(vec![Span::styled(
            "No additional structured records for this stage.",
            Style::default().fg(theme::MUTED).bg(theme::PANEL),
        )]));
    }

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Details ")
                .borders(Borders::ALL)
                .border_style(active_border(app.active_panel == Panel::Details))
                .style(theme::panel()),
        )
        .style(theme::panel())
        .wrap(Wrap { trim: false });
    frame.render_widget(paragraph, area);
}

fn draw_status(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let prompt_len = app.prompt.chars().count();
    let candidate_count = app
        .report
        .as_ref()
        .map(|report| report.candidates.len())
        .unwrap_or(0);
    let eval_ms = app
        .last_eval
        .map(|duration| format!("{:.1}ms", duration.as_secs_f64() * 1000.0))
        .unwrap_or_else(|| "-".into());
    let replay = if app.replay.active { " | Replay" } else { "" };
    let text = format!(
        "Prompt: {prompt_len} chars | Models: {} | Candidates: {candidate_count} | Eval: {eval_ms} | Workspace: {} | Engine: Deterministic{replay} | {}",
        app.report
            .as_ref()
            .map(|report| report.model_registry.len())
            .unwrap_or(0),
        app.current_stage().label(),
        app.status
    );
    let paragraph = Paragraph::new(text).style(Style::default().fg(Color::Black).bg(theme::ORANGE));
    frame.render_widget(paragraph, area);
}

fn draw_search(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let width = area.width.saturating_sub(8).min(72);
    let search_area = Rect::new(area.x + 4, area.y + area.height.saturating_sub(4), width, 3);
    let paragraph = Paragraph::new(format!("/{}", app.search_query))
        .block(
            Block::default()
                .title(" Search ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme::PURPLE).bg(theme::PANEL))
                .style(theme::panel()),
        )
        .style(theme::panel());
    frame.render_widget(Clear, search_area);
    frame.render_widget(paragraph, search_area);
}

fn draw_help(frame: &mut Frame<'_>, area: Rect) {
    let help_area = centered_rect(68, 18, area);
    let lines = vec![
        Line::from(vec![Span::styled(
            "Keyboard",
            Style::default()
                .fg(theme::ORANGE)
                .bg(theme::PANEL)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        kv("Up / Down", "move through pipeline".into()),
        kv("Left / Right", "switch focused panel".into()),
        kv("Tab / Shift+Tab", "next or previous stage".into()),
        kv("Enter", "run replay from existing DecisionReport".into()),
        kv("/", "search stage or report text".into()),
        kv("r", "re-run evaluation".into()),
        kv("e", "export DecisionReport JSON".into()),
        kv("?", "toggle help".into()),
        kv("q", "quit".into()),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Esc closes search or stops replay.",
            Style::default().fg(theme::MUTED).bg(theme::PANEL),
        )]),
    ];
    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" Help ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme::PURPLE).bg(theme::PANEL))
                .style(theme::panel()),
        )
        .style(theme::panel())
        .wrap(Wrap { trim: false });
    frame.render_widget(Clear, help_area);
    frame.render_widget(paragraph, help_area);
}

fn prompt_lines(app: &App) -> Vec<Line<'static>> {
    let report = app.report.as_ref().expect("report checked before render");
    vec![
        heading("Original request"),
        Line::from(""),
        text(report.prompt.text.clone()),
        Line::from(""),
        kv("characters", report.prompt.text.chars().count().to_string()),
        kv("evaluation", "completed".to_string()),
    ]
}

fn prompt_analysis_lines(app: &App) -> Vec<Line<'static>> {
    let report = app.report.as_ref().expect("report checked before render");
    let intrinsic = &report.prompt_profile.intrinsic;
    let derived = &report.prompt_profile.derived;
    vec![
        heading("Intrinsic Profile"),
        kv("word count", intrinsic.word_count.to_string()),
        kv("modality", intrinsic.modality.clone()),
        kv("languages", join_or_dash(&intrinsic.languages)),
        kv("frameworks", join_or_dash(&intrinsic.frameworks)),
        kv(
            "format",
            intrinsic
                .output_format
                .clone()
                .unwrap_or_else(|| "-".into()),
        ),
        kv("keywords", join_or_dash(&intrinsic.keywords)),
        Line::from(""),
        heading("Derived Profile"),
        kv("category", derived.task_category.clone()),
        kv("complexity", derived.complexity.clone()),
        kv("reasoning", derived.reasoning_depth.clone()),
        kv("ambiguity", derived.ambiguity.clone()),
    ]
}

fn prompt_analysis_detail_lines(app: &App) -> Vec<Line<'static>> {
    let report = app.report.as_ref().expect("report checked before render");
    vec![
        kv("source", "DecisionReport.prompt_profile".into()),
        kv(
            "text chars",
            report
                .prompt_profile
                .intrinsic
                .text
                .chars()
                .count()
                .to_string(),
        ),
        kv(
            "category",
            report.prompt_profile.derived.task_category.clone(),
        ),
        kv(
            "complexity",
            report.prompt_profile.derived.complexity.clone(),
        ),
    ]
}

fn join_or_dash(values: &[String]) -> String {
    if values.is_empty() {
        "-".into()
    } else {
        values.join(", ")
    }
}

fn capability_lines(app: &App) -> Vec<Line<'static>> {
    let report = app.report.as_ref().expect("report checked before render");
    let (required, preferred, optional) = capability_counts(&report.capabilities);
    let mut lines = vec![
        kv("required", required.to_string()),
        kv("preferred", preferred.to_string()),
        kv("optional", optional.to_string()),
        Line::from(""),
    ];

    for req in &report.capabilities.requirements {
        lines.push(Line::from(vec![
            Span::styled(
                format!("{:<22}", req.capability),
                Style::default().fg(theme::TEXT).bg(theme::PANEL),
            ),
            Span::styled(
                format!("{:?}", req.priority),
                Style::default().fg(theme::ORANGE).bg(theme::PANEL),
            ),
            Span::styled(
                format!(
                    "  weight {:.2} confidence {:.2}",
                    req.weight, req.confidence
                ),
                Style::default().fg(theme::MUTED).bg(theme::PANEL),
            ),
        ]));
        lines.push(text(format!("  {}", req.reason)));
    }

    lines
}

fn registry_lines(app: &App) -> Vec<Line<'static>> {
    let mut lines = vec![
        heading("Available model profiles supplied to the engine"),
        Line::from(""),
    ];

    let report = app.report.as_ref().expect("report checked before render");
    for model in &report.model_registry {
        let latency = model
            .operational_characteristics
            .latency
            .as_ref()
            .map(|latency| format!("{:.0}ms p95", latency.p95_ms))
            .unwrap_or_else(|| "latency n/a".into());
        lines.push(Line::from(vec![
            Span::styled(
                format!("{:<22}", model.identity.name),
                Style::default()
                    .fg(theme::TEXT)
                    .bg(theme::PANEL)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                model.identity.provider.clone(),
                Style::default().fg(theme::PURPLE).bg(theme::PANEL),
            ),
            Span::styled(
                format!("  {} caps  {}", model.capabilities.len(), latency),
                Style::default().fg(theme::MUTED).bg(theme::PANEL),
            ),
        ]));
    }

    lines
}

fn policy_lines(app: &App) -> Vec<Line<'static>> {
    let report = app.report.as_ref().expect("report checked before render");
    let mut lines = vec![kv("policy", report.policy.name.clone()), Line::from("")];

    if report.policy.rules.is_empty() {
        lines.push(text(
            "No configured policy rules rejected candidates.".into(),
        ));
    }

    if report.explanation.policy_decisions.is_empty() {
        lines.push(text(
            "The explanation contains no explicit policy decision records.".into(),
        ));
    } else {
        for decision in &report.explanation.policy_decisions {
            lines.push(kv(
                decision.policy.as_str(),
                format!("{} - {}", decision.applied, decision.reason),
            ));
        }
    }

    lines
}

fn scoring_lines(app: &App) -> Vec<Line<'static>> {
    let report = app.report.as_ref().expect("report checked before render");
    let mut lines = vec![heading("Candidate scores"), Line::from("")];

    for (rank, (model, score)) in report.scored_candidates.iter().enumerate() {
        lines.push(kv(
            format!("#{} {}", rank + 1, model.identity.id),
            format!(
                "score {:.3} confidence {:.2}",
                score.final_score, score.confidence
            ),
        ));
        for evidence in score.evidence.iter().take(2) {
            lines.push(text(format!("  {}", evidence)));
        }
    }

    lines
}

fn recommendation_lines(app: &App) -> Vec<Line<'static>> {
    let report = app.report.as_ref().expect("report checked before render");
    vec![
        heading("Selected model"),
        Line::from(""),
        kv("name", report.recommendation.model.identity.name.clone()),
        kv("id", report.recommendation.model.identity.id.clone()),
        kv(
            "provider",
            report.recommendation.model.identity.provider.clone(),
        ),
        kv("score", format!("{:.3}", report.recommendation.score)),
        kv(
            "confidence",
            format!("{:.2}", report.explanation.confidence),
        ),
        Line::from(""),
        text(report.explanation.summary.clone()),
    ]
}

fn explanation_lines(app: &App) -> Vec<Line<'static>> {
    let report = app.report.as_ref().expect("report checked before render");
    let mut lines = vec![
        heading("Summary"),
        text(report.explanation.summary.clone()),
        Line::from(""),
        heading("Capability matches"),
    ];

    if report.explanation.capability_matches.is_empty() {
        lines.push(text("No capability match records were included.".into()));
    } else {
        for item in &report.explanation.capability_matches {
            lines.push(kv(
                item.capability.as_str(),
                format!("{:?} - {}", item.status, item.reason),
            ));
        }
    }

    if !report.explanation.evidence.is_empty() {
        lines.push(Line::from(""));
        lines.push(heading("Evidence"));
        for evidence in &report.explanation.evidence {
            lines.push(kv(evidence.source.as_str(), evidence.detail.clone()));
        }
    }

    lines
}

fn selected_model_lines(app: &App) -> Vec<Line<'static>> {
    let report = app.report.as_ref().expect("report checked before render");
    let model = &report.recommendation.model;
    vec![
        kv(
            "family",
            model.identity.family.clone().unwrap_or_else(|| "-".into()),
        ),
        kv(
            "version",
            model.identity.version.clone().unwrap_or_else(|| "-".into()),
        ),
        kv("context", model.limits.max_context_tokens.to_string()),
        kv("output", model.limits.max_output_tokens.to_string()),
        kv(
            "privacy",
            format!("{:?}", model.operational_characteristics.privacy_level),
        ),
        kv(
            "locality",
            format!("{:?}", model.operational_characteristics.locality),
        ),
    ]
}

fn heading(value: &str) -> Line<'static> {
    Line::from(vec![Span::styled(
        value.to_string(),
        Style::default()
            .fg(theme::ORANGE)
            .bg(theme::PANEL)
            .add_modifier(Modifier::BOLD),
    )])
}

fn kv(name: impl AsRef<str>, value: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("{:<14}", name.as_ref()),
            Style::default().fg(theme::MUTED).bg(theme::PANEL),
        ),
        Span::styled(value, Style::default().fg(theme::TEXT).bg(theme::PANEL)),
    ])
}

fn text(value: String) -> Line<'static> {
    Line::from(Span::styled(
        value,
        Style::default().fg(theme::TEXT).bg(theme::PANEL),
    ))
}

fn active_border(active: bool) -> Style {
    if active {
        Style::default().fg(theme::ORANGE).bg(theme::PANEL)
    } else {
        Style::default().fg(theme::BORDER).bg(theme::PANEL)
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}
