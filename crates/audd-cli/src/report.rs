//! Report generation for AUDD comparison results
//!
//! This module generates comprehensive Markdown reports from comparison results,
//! including executive summaries and technical details.

use audd_compare::{ComparisonResult, Conflict, ConflictSeverity, ExclusiveSide, MatchReason};
use audd_resolution::DecisionLog;
use std::collections::HashMap;

/// Risk level for overall compatibility assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    fn emoji(&self) -> &str {
        match self {
            RiskLevel::Low => "✅",
            RiskLevel::Medium => "⚠️",
            RiskLevel::High => "🔥",
            RiskLevel::Critical => "💀",
        }
    }

    fn label(&self) -> &str {
        match self {
            RiskLevel::Low => "Low",
            RiskLevel::Medium => "Medium",
            RiskLevel::High => "High",
            RiskLevel::Critical => "Critical",
        }
    }
}

/// Metrics calculated from comparison results
#[derive(Debug, Clone)]
pub struct ReportMetrics {
    pub total_matches: usize,
    pub total_exclusives: usize,
    pub total_conflicts: usize,
    pub safe_exclusives: usize,
    pub exclusives_a: usize,
    pub exclusives_b: usize,
    pub compatibility_score: f64,
    pub safe_addition_rate: f64,
    pub conflict_rate: f64,
    pub average_match_confidence: f64,
    pub risk_level: RiskLevel,
    pub critical_conflicts: usize,
    pub high_severity_conflicts: usize,
}

impl ReportMetrics {
    /// Calculate metrics from comparison result
    pub fn from_comparison(result: &ComparisonResult) -> Self {
        let total_matches = result.matches.len();
        let total_exclusives = result.exclusives.len();
        let total_conflicts = result.conflicts.len();

        let safe_exclusives = result.exclusives.iter().filter(|e| e.safe_to_add).count();
        let exclusives_a = result.exclusives.iter().filter(|e| e.side == ExclusiveSide::A).count();
        let exclusives_b = result.exclusives.iter().filter(|e| e.side == ExclusiveSide::B).count();

        // Calculate scores
        let compatibility_score = if total_matches + total_conflicts > 0 {
            (total_matches as f64 / (total_matches + total_conflicts) as f64) * 100.0
        } else {
            100.0
        };

        let safe_addition_rate = if total_exclusives > 0 {
            (safe_exclusives as f64 / total_exclusives as f64) * 100.0
        } else {
            100.0
        };

        let conflict_rate = if total_matches + total_conflicts > 0 {
            (total_conflicts as f64 / (total_matches + total_conflicts) as f64) * 100.0
        } else {
            0.0
        };

        let average_match_confidence = if !result.matches.is_empty() {
            result.matches.iter().map(|m| m.score).sum::<f64>() / result.matches.len() as f64
        } else {
            0.0
        };

        // Count severity levels
        let critical_conflicts = result.conflicts.iter().filter(|c| c.severity == ConflictSeverity::Critical).count();
        let high_severity_conflicts = result.conflicts.iter().filter(|c| c.severity == ConflictSeverity::High).count();

        // Determine risk level
        let risk_level = if critical_conflicts > 0 {
            if critical_conflicts >= 3 {
                RiskLevel::Critical
            } else {
                RiskLevel::High
            }
        } else if conflict_rate > 50.0 {
            RiskLevel::Critical
        } else if conflict_rate > 25.0 || high_severity_conflicts > 0 {
            RiskLevel::High
        } else if conflict_rate > 10.0 {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };

        Self {
            total_matches,
            total_exclusives,
            total_conflicts,
            safe_exclusives,
            exclusives_a,
            exclusives_b,
            compatibility_score,
            safe_addition_rate,
            conflict_rate,
            average_match_confidence,
            risk_level,
            critical_conflicts,
            high_severity_conflicts,
        }
    }
}

/// Conflicts grouped by entity
#[derive(Debug)]
pub struct EntityConflicts {
    pub entity_name: String,
    pub conflict_count: usize,
    pub highest_severity: ConflictSeverity,
}

/// Generate a comprehensive Markdown report
pub fn generate_report(
    schema_a_name: &str,
    schema_b_name: &str,
    result: &ComparisonResult,
    decision_log: Option<&DecisionLog>,
) -> String {
    let mut md = String::new();

    // Calculate metrics
    let metrics = ReportMetrics::from_comparison(result);

    // Header
    generate_header(&mut md, schema_a_name, schema_b_name);

    // Executive Summary
    generate_executive_summary(&mut md, &metrics, result);

    // Detailed Breakdown
    generate_detailed_breakdown(&mut md, &metrics, result);

    // Technical Details
    generate_technical_details(&mut md, result);

    // Resolution Suggestions (if available)
    if let Some(log) = decision_log {
        generate_resolution_section(&mut md, result, log);
        generate_decision_log_section(&mut md, log);
    }

    // Recommendations
    generate_recommendations(&mut md, &metrics, result, decision_log);

    // Footer
    md.push_str("\n---\n\n");
    md.push_str("*Report generated by AUDD v0.1.0*\n");

    md
}

fn generate_header(md: &mut String, schema_a_name: &str, schema_b_name: &str) {
    md.push_str("# AUDD Comparison Report\n\n");
    
    // Get current timestamp
    let now = chrono::Utc::now();
    md.push_str(&format!("**Generated:** {} UTC  \n", now.format("%Y-%m-%d %H:%M:%S")));
    md.push_str(&format!("**Schema A:** {}  \n", schema_a_name));
    md.push_str(&format!("**Schema B:** {}  \n", schema_b_name));
    md.push_str("**Report Version:** 1.0.0\n\n");
    md.push_str("---\n\n");
}

fn generate_executive_summary(md: &mut String, metrics: &ReportMetrics, result: &ComparisonResult) {
    md.push_str("## Executive Summary\n\n");
    
    // Compatibility Overview
    md.push_str("### Compatibility Overview\n\n");
    md.push_str(&format!("- **Matches**: {}\n", metrics.total_matches));
    md.push_str(&format!("- **Exclusives**: {} ({} from A, {} from B)\n", 
        metrics.total_exclusives, metrics.exclusives_a, metrics.exclusives_b));
    md.push_str(&format!("- **Conflicts**: {}\n", metrics.total_conflicts));
    md.push_str(&format!("- **Compatibility Score**: {:.1}%\n", metrics.compatibility_score));
    md.push_str(&format!("- **Safe Addition Rate**: {:.1}%\n", metrics.safe_addition_rate));
    md.push_str(&format!("- **Conflict Rate**: {:.1}%\n\n", metrics.conflict_rate));

    // Risk Assessment
    md.push_str("### Risk Assessment\n\n");
    md.push_str(&format!("**Overall Risk Level**: {} **{}**\n\n", 
        metrics.risk_level.emoji(), metrics.risk_level.label()));

    // Explain risk level
    if metrics.critical_conflicts > 0 {
        md.push_str(&format!("- {} critical-severity conflict(s) detected\n", metrics.critical_conflicts));
    }
    if metrics.high_severity_conflicts > 0 {
        md.push_str(&format!("- {} high-severity conflict(s) detected\n", metrics.high_severity_conflicts));
    }
    if metrics.conflict_rate > 25.0 {
        md.push_str(&format!("- Conflict rate is {:.1}% (above 25% threshold)\n", metrics.conflict_rate));
    } else if metrics.conflict_rate > 10.0 {
        md.push_str(&format!("- Conflict rate is {:.1}% (above 10% threshold)\n", metrics.conflict_rate));
    }
    
    if metrics.risk_level == RiskLevel::Low {
        md.push_str("- Low conflict rate and no high-severity issues\n");
    }
    md.push_str("\n");

    // Top Conflicts by Entity
    if metrics.total_conflicts > 0 {
        generate_top_conflicts(md, result);
    }

    md.push_str("---\n\n");
}

fn generate_top_conflicts(md: &mut String, result: &ComparisonResult) {
    md.push_str("### Top Conflicts by Entity\n\n");

    // Group conflicts by entity
    let mut entity_map: HashMap<String, Vec<&Conflict>> = HashMap::new();
    for conflict in &result.conflicts {
        entity_map.entry(conflict.entity_name.clone())
            .or_insert_with(Vec::new)
            .push(conflict);
    }

    // Create entity conflict summary
    let mut entity_conflicts: Vec<EntityConflicts> = entity_map
        .iter()
        .map(|(name, conflicts)| {
            let highest_severity = conflicts.iter()
                .map(|c| c.severity)
                .max()
                .unwrap_or(ConflictSeverity::Low);
            EntityConflicts {
                entity_name: name.clone(),
                conflict_count: conflicts.len(),
                highest_severity,
            }
        })
        .collect();

    // Sort by conflict count (descending), then by severity (descending)
    entity_conflicts.sort_by(|a, b| {
        b.conflict_count.cmp(&a.conflict_count)
            .then(b.highest_severity.cmp(&a.highest_severity))
    });

    // Show top 5 or all if fewer
    let top_count = entity_conflicts.len().min(5);
    
    md.push_str("| Entity | Conflicts | Highest Severity |\n");
    md.push_str("|--------|-----------|------------------|\n");
    
    for ec in entity_conflicts.iter().take(top_count) {
        let severity_label = format!("{:?}", ec.highest_severity);
        let severity_icon = match ec.highest_severity {
            ConflictSeverity::Critical => "💀",
            ConflictSeverity::High => "🔥",
            ConflictSeverity::Medium => "⚠️",
            ConflictSeverity::Low => "ℹ️",
        };
        md.push_str(&format!("| {} | {} | {} {} |\n", 
            ec.entity_name, ec.conflict_count, severity_icon, severity_label));
    }
    
    md.push_str("\n");
}

fn generate_detailed_breakdown(md: &mut String, metrics: &ReportMetrics, result: &ComparisonResult) {
    md.push_str("## Detailed Breakdown\n\n");

    // Matches breakdown
    md.push_str("### Matches\n\n");
    md.push_str(&format!("- **Total**: {}\n", metrics.total_matches));
    
    // Count by match reason
    let exact_matches = result.matches.iter()
        .filter(|m| matches!(m.reason, MatchReason::ExactName))
        .count();
    let normalized_matches = result.matches.iter()
        .filter(|m| matches!(m.reason, MatchReason::NormalizedName { .. }))
        .count();
    let similarity_matches = result.matches.iter()
        .filter(|m| matches!(m.reason, MatchReason::Similarity { .. }))
        .count();
    
    md.push_str(&format!("- **Exact name matches**: {}\n", exact_matches));
    if normalized_matches > 0 {
        md.push_str(&format!("- **Normalized matches**: {}\n", normalized_matches));
    }
    if similarity_matches > 0 {
        md.push_str(&format!("- **Similarity matches**: {}\n", similarity_matches));
    }
    md.push_str(&format!("- **Average confidence**: {:.2}\n\n", metrics.average_match_confidence));

    // Exclusives breakdown
    md.push_str("### Exclusives\n\n");
    md.push_str(&format!("- **From Schema A**: {} ({} safe to add)\n", 
        metrics.exclusives_a, 
        result.exclusives.iter().filter(|e| e.side == ExclusiveSide::A && e.safe_to_add).count()));
    md.push_str(&format!("- **From Schema B**: {} ({} safe to add)\n", 
        metrics.exclusives_b,
        result.exclusives.iter().filter(|e| e.side == ExclusiveSide::B && e.safe_to_add).count()));
    md.push_str(&format!("- **Safe Addition Rate**: {:.1}%\n\n", metrics.safe_addition_rate));

    // Conflicts breakdown
    md.push_str("### Conflicts\n\n");
    md.push_str(&format!("- **Total**: {}\n", metrics.total_conflicts));
    
    if metrics.total_conflicts > 0 {
        // Count by type
        let mut type_counts: HashMap<String, usize> = HashMap::new();
        for conflict in &result.conflicts {
            let type_name = format!("{:?}", conflict.conflict_type);
            *type_counts.entry(type_name).or_insert(0) += 1;
        }
        
        md.push_str("- **By Type**:\n");
        for (type_name, count) in type_counts.iter() {
            md.push_str(&format!("  - {}: {}\n", type_name, count));
        }
        
        // Count by severity
        let low_severity = result.conflicts.iter().filter(|c| c.severity == ConflictSeverity::Low).count();
        let medium_severity = result.conflicts.iter().filter(|c| c.severity == ConflictSeverity::Medium).count();
        let high_severity = result.conflicts.iter().filter(|c| c.severity == ConflictSeverity::High).count();
        let critical_severity = result.conflicts.iter().filter(|c| c.severity == ConflictSeverity::Critical).count();
        
        md.push_str("- **By Severity**:\n");
        if critical_severity > 0 {
            md.push_str(&format!("  - Critical: {}\n", critical_severity));
        }
        if high_severity > 0 {
            md.push_str(&format!("  - High: {}\n", high_severity));
        }
        if medium_severity > 0 {
            md.push_str(&format!("  - Medium: {}\n", medium_severity));
        }
        if low_severity > 0 {
            md.push_str(&format!("  - Low: {}\n", low_severity));
        }
    }
    
    md.push_str("\n---\n\n");
}

fn generate_technical_details(md: &mut String, result: &ComparisonResult) {
    md.push_str("## Technical Details\n\n");

    // Matches listing
    if !result.matches.is_empty() {
        md.push_str("### Matches\n\n");
        md.push_str("| Entity | Field | Match Type | Score | Index A | Index B |\n");
        md.push_str("|--------|-------|------------|-------|---------|---------|\n");
        
        for m in &result.matches {
            let field_name = m.field_name.as_deref().unwrap_or("(entity)");
            let match_type = match &m.reason {
                MatchReason::ExactName => "exact_name".to_string(),
                MatchReason::NormalizedName { .. } => "normalized".to_string(),
                MatchReason::Similarity { .. } => "similarity".to_string(),
            };
            md.push_str(&format!("| {} | {} | {} | {:.2} | {} | {} |\n",
                m.entity_name, field_name, match_type, m.score, m.index_a, m.index_b));
        }
        md.push_str("\n");
    }

    // Exclusives listing
    if !result.exclusives.is_empty() {
        md.push_str("### Exclusives\n\n");
        md.push_str("| Entity | Field | Side | Safe to Add | Index |\n");
        md.push_str("|--------|-------|------|-------------|-------|\n");
        
        for e in &result.exclusives {
            let field_name = e.field_name.as_deref().unwrap_or("(entity)");
            let side = match e.side {
                ExclusiveSide::A => "A",
                ExclusiveSide::B => "B",
            };
            let safe = if e.safe_to_add { "✅ Yes" } else { "⚠️ Review" };
            md.push_str(&format!("| {} | {} | {} | {} | {} |\n",
                e.entity_name, field_name, side, safe, e.index));
        }
        md.push_str("\n");
    }

    // Conflicts listing
    if !result.conflicts.is_empty() {
        md.push_str("### Conflicts\n\n");
        
        for (i, conflict) in result.conflicts.iter().enumerate() {
            let conflict_num = i + 1;
            md.push_str(&format!("#### <a name=\"conflict-{}\"></a>Conflict #{}\n\n", conflict_num, conflict_num));
            md.push_str(&format!("- **Entity**: {}\n", conflict.entity_name));
            if let Some(ref field) = conflict.field_name {
                md.push_str(&format!("- **Field**: {}\n", field));
            }
            md.push_str(&format!("- **Type**: {:?}\n", conflict.conflict_type));
            
            let severity_icon = match conflict.severity {
                ConflictSeverity::Critical => "💀",
                ConflictSeverity::High => "🔥",
                ConflictSeverity::Medium => "⚠️",
                ConflictSeverity::Low => "ℹ️",
            };
            md.push_str(&format!("- **Severity**: {} {:?}\n", severity_icon, conflict.severity));
            md.push_str("- **Evidence**:\n");
            md.push_str(&format!("  - **From Schema A**: {}\n", conflict.evidence.from_a));
            md.push_str(&format!("  - **From Schema B**: {}\n", conflict.evidence.from_b));
            md.push_str(&format!("  - **Rule**: {}\n", conflict.evidence.rule));
            md.push_str(&format!("- **Indexes**: A={}, B={}\n\n", conflict.index_a, conflict.index_b));
        }
    }

    md.push_str("---\n\n");
}

fn generate_resolution_section(md: &mut String, result: &ComparisonResult, log: &DecisionLog) {
    let decisions = log.get_decisions();
    
    if decisions.is_empty() {
        return;
    }

    md.push_str("## Resolution Suggestions\n\n");

    // Summary
    let total_suggestions = decisions.len();
    let high_conf = decisions.iter().filter(|d| d.suggestion.confidence.value() >= 0.85).count();
    let medium_conf = decisions.iter().filter(|d| {
        let conf = d.suggestion.confidence.value();
        conf >= 0.60 && conf < 0.85
    }).count();
    let auto_accepted = decisions.iter().filter(|d| d.accepted).count();

    md.push_str("### Summary\n\n");
    md.push_str(&format!("- **Total Suggestions**: {}\n", total_suggestions));
    md.push_str(&format!("- **High Confidence** (≥0.85): {}\n", high_conf));
    md.push_str(&format!("- **Medium Confidence** (0.60-0.84): {}\n", medium_conf));
    md.push_str(&format!("- **Auto-accepted**: {}\n\n", auto_accepted));

    // Detailed suggestions by conflict
    for (i, conflict) in result.conflicts.iter().enumerate() {
        let conflict_num = i + 1;
        
        // Find suggestions for this conflict
        let conflict_suggestions: Vec<_> = decisions.iter()
            .filter(|d| {
                d.suggestion.entity_name == conflict.entity_name &&
                d.suggestion.field_name == conflict.field_name
            })
            .collect();

        if conflict_suggestions.is_empty() {
            continue;
        }

        md.push_str(&format!("### Conflict #{} - Suggestions\n\n", conflict_num));

        for decision in conflict_suggestions {
            let sug = &decision.suggestion;
            md.push_str(&format!("#### Suggestion: {}\n\n", sug.id));
            md.push_str(&format!("- **Kind**: {:?}\n", sug.kind));
            md.push_str(&format!("- **Confidence**: {:.2} ({})\n", 
                sug.confidence.value(),
                if sug.confidence.value() >= 0.85 { "High" } 
                else if sug.confidence.value() >= 0.60 { "Medium" } 
                else { "Low" }));
            md.push_str(&format!("- **Impact**: {:?}\n", sug.impact));
            md.push_str(&format!("- **Explanation**: {}\n", sug.explanation));
            
            if !sug.evidence.is_empty() {
                md.push_str("- **Evidence**:\n");
                for evidence in &sug.evidence {
                    md.push_str(&format!("  - {}\n", evidence));
                }
            }
            
            if decision.accepted {
                md.push_str("- **Status**: ✅ Accepted\n");
            } else {
                md.push_str("- **Status**: ❌ Rejected\n");
            }
            md.push_str("\n");
        }
    }

    md.push_str("---\n\n");
}

fn generate_decision_log_section(md: &mut String, log: &DecisionLog) {
    let decisions = log.get_decisions();
    
    if decisions.is_empty() {
        return;
    }

    md.push_str("## Decision Log\n\n");

    // Summary
    let total = decisions.len();
    let accepted = decisions.iter().filter(|d| d.accepted).count();
    let rejected = total - accepted;
    
    let system_decisions = decisions.iter()
        .filter(|d| matches!(d.source, audd_resolution::DecisionSource::System { .. }))
        .count();
    let user_decisions = total - system_decisions;

    md.push_str("### Summary\n\n");
    md.push_str(&format!("- **Total Decisions**: {}\n", total));
    md.push_str(&format!("- **Accepted**: {} ({:.1}%)\n", accepted, 
        if total > 0 { (accepted as f64 / total as f64) * 100.0 } else { 0.0 }));
    md.push_str(&format!("- **Rejected**: {} ({:.1}%)\n", rejected,
        if total > 0 { (rejected as f64 / total as f64) * 100.0 } else { 0.0 }));
    md.push_str("- **By Source**:\n");
    md.push_str(&format!("  - System (auto): {}\n", system_decisions));
    md.push_str(&format!("  - User (manual): {}\n\n", user_decisions));

    // Detailed decisions
    md.push_str("### Decisions\n\n");
    
    for decision in decisions {
        md.push_str(&format!("#### Decision: {}\n\n", decision.id));
        md.push_str(&format!("- **Suggestion**: {} ({}::{})\n", 
            decision.suggestion.id,
            decision.suggestion.entity_name,
            decision.suggestion.field_name.as_deref().unwrap_or("(entity)")));
        
        if decision.accepted {
            md.push_str("- **Status**: ✅ Accepted\n");
        } else {
            md.push_str("- **Status**: ❌ Rejected\n");
        }
        
        md.push_str(&format!("- **Source**: {}\n", match &decision.source {
            audd_resolution::DecisionSource::System { rule } => format!("System ({})", rule),
            audd_resolution::DecisionSource::User { username } => format!("User ({})", username),
        }));
        md.push_str(&format!("- **Rationale**: {}\n", decision.rationale));
        md.push_str(&format!("- **Timestamp**: {}\n\n", decision.timestamp));
    }

    md.push_str("---\n\n");
}

fn generate_recommendations(md: &mut String, metrics: &ReportMetrics, _result: &ComparisonResult, decision_log: Option<&DecisionLog>) {
    md.push_str("## Recommendations\n\n");

    let mut recommendations = Vec::new();

    // Risk-based recommendations
    match metrics.risk_level {
        RiskLevel::Critical => {
            recommendations.push("🔥 **CRITICAL**: Immediate manual review required before proceeding".to_string());
        }
        RiskLevel::High => {
            recommendations.push("⚠️ **HIGH RISK**: Careful review of all conflicts recommended".to_string());
        }
        RiskLevel::Medium => {
            recommendations.push("⚠️ Review medium and high-severity conflicts before unification".to_string());
        }
        RiskLevel::Low => {
            recommendations.push("✅ Low risk detected - schemas are mostly compatible".to_string());
        }
    }

    // Conflict-specific recommendations
    if metrics.critical_conflicts > 0 {
        recommendations.push(format!(
            "💀 Review {} critical-severity conflict(s) immediately",
            metrics.critical_conflicts
        ));
    }
    
    if metrics.high_severity_conflicts > 0 {
        recommendations.push(format!(
            "🔥 Address {} high-severity conflict(s) before auto-unification",
            metrics.high_severity_conflicts
        ));
    }

    // Unresolved conflicts
    let unresolved = if let Some(log) = decision_log {
        let resolved_count = log.get_accepted_decisions().len();
        metrics.total_conflicts.saturating_sub(resolved_count)
    } else {
        metrics.total_conflicts
    };

    if unresolved > 0 {
        recommendations.push(format!(
            "🔍 {} conflict(s) remain without accepted resolutions",
            unresolved
        ));
    }

    // Exclusives recommendations
    let unsafe_exclusives = metrics.total_exclusives - metrics.safe_exclusives;
    if unsafe_exclusives > 0 {
        recommendations.push(format!(
            "⚠️ Review {} exclusive field(s) marked for manual review",
            unsafe_exclusives
        ));
    }

    // Compatibility recommendations
    if metrics.compatibility_score >= 80.0 {
        recommendations.push("✅ High compatibility score - schemas align well".to_string());
    } else if metrics.compatibility_score < 50.0 {
        recommendations.push("⚠️ Low compatibility score - consider manual schema alignment".to_string());
    }

    // Decision log reference
    if decision_log.is_some() {
        recommendations.push("📄 See `decision_log.json` for complete decision history and programmatic access".to_string());
    }

    // Output recommendations
    for rec in recommendations {
        md.push_str(&format!("- {}\n", rec));
    }

    md.push_str("\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    use audd_compare::{ConflictEvidence, ConflictType, Match, Exclusive};

    fn create_test_result() -> ComparisonResult {
        let matches = vec![
            Match::exact("users".to_string(), Some("id".to_string()), 0, 0),
            Match::exact("users".to_string(), Some("name".to_string()), 1, 1),
        ];

        let exclusives = vec![
            Exclusive::from_a("users".to_string(), Some("password".to_string()), 2),
        ];

        let conflicts = vec![
            Conflict::type_incompatible(
                "users".to_string(),
                "age".to_string(),
                "String".to_string(),
                "Int32".to_string(),
                3,
                3,
            ),
        ];

        ComparisonResult::new(matches, exclusives, conflicts)
    }

    #[test]
    fn test_metrics_calculation() {
        let result = create_test_result();
        let metrics = ReportMetrics::from_comparison(&result);

        assert_eq!(metrics.total_matches, 2);
        assert_eq!(metrics.total_exclusives, 1);
        assert_eq!(metrics.total_conflicts, 1);
        assert!(metrics.compatibility_score > 0.0);
    }

    #[test]
    fn test_risk_level_low() {
        let matches = vec![
            Match::exact("users".to_string(), Some("id".to_string()), 0, 0),
        ];
        let result = ComparisonResult::new(matches, vec![], vec![]);
        let metrics = ReportMetrics::from_comparison(&result);

        assert_eq!(metrics.risk_level, RiskLevel::Low);
    }

    #[test]
    fn test_risk_level_critical() {
        let conflicts = vec![
            Conflict::new(
                "users".to_string(),
                Some("id".to_string()),
                ConflictType::TypeIncompatible,
                ConflictSeverity::Critical,
                ConflictEvidence::new("a".to_string(), "b".to_string(), "rule".to_string()),
                0,
                0,
            ),
        ];
        let result = ComparisonResult::new(vec![], vec![], conflicts);
        let metrics = ReportMetrics::from_comparison(&result);

        assert_eq!(metrics.risk_level, RiskLevel::High);
    }

    #[test]
    fn test_report_generation() {
        let result = create_test_result();
        let report = generate_report("schema_a", "schema_b", &result, None);

        assert!(report.contains("# AUDD Comparison Report"));
        assert!(report.contains("Executive Summary"));
        assert!(report.contains("Technical Details"));
        assert!(report.contains("Recommendations"));
    }
}
