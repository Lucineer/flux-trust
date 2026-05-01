#![allow(dead_code)]
#[derive(Clone,Debug,PartialEq)]
pub enum TrustLevel { Unknown, Suspicious, Neutral, Trusted, Verified }
pub struct TrustEntry { agent: u32, score: f64, evidence: u32, last_update: u64, level: TrustLevel }
pub struct TrustTable { scores: Vec<TrustEntry>, default: f64, decay: f64, max: f64 }
impl TrustTable {
    pub fn new(default: f64, decay: f64) -> Self { Self { scores: Vec::new(), default, decay, max: 1.0 } }
    pub fn get(&self, agent: u32) -> f64 { self.scores.iter().find(|e| e.agent == agent).map(|e| e.score).unwrap_or(self.default) }
    pub fn set(&mut self, agent: u32, score: f64) {
        let s = score.clamp(0.0, self.max);
        if let Some(e) = self.scores.iter_mut().find(|e| e.agent == agent) { e.score = s; e.level = Self::level(s); }
        else { self.scores.push(TrustEntry { agent, score: s, evidence: 1, last_update: 0, level: Self::level(s) }); }
    }
    pub fn update(&mut self, agent: u32, evidence: f64, weight: f64) {
        let old = self.get(agent); let w = weight.clamp(0.0, 1.0);
        self.set(agent, old * (1.0 - w) + evidence.clamp(0.0, self.max) * w);
        if let Some(e) = self.scores.iter_mut().find(|e| e.agent == agent) { e.evidence += 1; }
    }
    pub fn decay_all(&mut self, now: u64) { for e in &mut self.scores { e.score = (e.score * (1.0 - self.decay)).max(0.0); e.level = Self::level(e.score); e.last_update = now; } }
    fn level(s: f64) -> TrustLevel { if s < 0.2 { TrustLevel::Unknown } else if s < 0.4 { TrustLevel::Suspicious } else if s < 0.6 { TrustLevel::Neutral } else if s < 0.8 { TrustLevel::Trusted } else { TrustLevel::Verified } }
    pub fn level_of(&self, agent: u32) -> TrustLevel { self.scores.iter().find(|e| e.agent == agent).map(|e| e.level.clone()).unwrap_or(Self::level(self.default)) }
    pub fn is_trusted(&self, agent: u32, threshold: f64) -> bool { self.get(agent) >= threshold }
    pub fn most_trusted(&self, n: usize) -> Vec<&TrustEntry> { let mut v: Vec<&TrustEntry> = self.scores.iter().collect(); v.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap()); v.truncate(n); v }
    pub fn least_trusted(&self, n: usize) -> Vec<&TrustEntry> { let mut v: Vec<&TrustEntry> = self.scores.iter().collect(); v.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap()); v.truncate(n); v }
    pub fn revoke(&mut self, agent: u32) { self.set(agent, 0.0); }
    pub fn restore(&mut self, agent: u32, score: f64) { self.set(agent, score); }
    pub fn agent_count(&self) -> usize { self.scores.len() }
    pub fn average(&self) -> f64 { if self.scores.is_empty() { self.default } else { self.scores.iter().map(|e| e.score).sum::<f64>() / self.scores.len() as f64 } }
    pub fn above(&self, threshold: f64) -> Vec<&TrustEntry> { self.scores.iter().filter(|e| e.score >= threshold).collect() }
    pub fn below(&self, threshold: f64) -> Vec<&TrustEntry> { self.scores.iter().filter(|e| e.score < threshold).collect() }
    pub fn boost(&mut self, agent: u32, amount: f64) { self.set(agent, self.get(agent) + amount); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_new() { let t = TrustTable::new(0.5, 0.01); assert_eq!(t.get(99), 0.5); }
    #[test] fn test_set() { let mut t = TrustTable::new(0.5, 0.01); t.set(1, 0.9); assert!((t.get(1) - 0.9).abs() < 1e-6); }
    #[test] fn test_update() { let mut t = TrustTable::new(0.5, 0.01); t.update(1, 1.0, 0.5); assert!(t.get(1) > 0.5); }
    #[test] fn test_decay() { let mut t = TrustTable::new(0.5, 0.1); t.set(1, 0.9); t.decay_all(100); assert!(t.get(1) < 0.9); }
    #[test] fn test_level() { let mut t = TrustTable::new(0.5, 0.01); t.set(1, 0.1); assert_eq!(t.level_of(1), TrustLevel::Unknown); t.set(1, 0.3); assert_eq!(t.level_of(1), TrustLevel::Suspicious); t.set(1, 0.5); assert_eq!(t.level_of(1), TrustLevel::Neutral); t.set(1, 0.7); assert_eq!(t.level_of(1), TrustLevel::Trusted); t.set(1, 0.9); assert_eq!(t.level_of(1), TrustLevel::Verified); }
    #[test] fn test_revoke() { let mut t = TrustTable::new(0.5, 0.01); t.set(1, 0.9); t.revoke(1); assert!((t.get(1)).abs() < 1e-6); }
    #[test] fn test_most_trusted() { let mut t = TrustTable::new(0.0, 0.01); t.set(1, 0.3); t.set(2, 0.9); t.set(3, 0.6); assert_eq!(t.most_trusted(1)[0].agent, 2); }
    #[test] fn test_average() { let mut t = TrustTable::new(0.0, 0.01); t.set(1, 0.4); t.set(2, 0.8); assert!((t.average() - 0.6).abs() < 1e-6); }
    #[test] fn test_boost() { let mut t = TrustTable::new(0.5, 0.01); t.set(1, 0.5); t.boost(1, 0.3); assert!((t.get(1) - 0.8).abs() < 1e-6); }
    #[test] fn test_above_below() { let mut t = TrustTable::new(0.0, 0.01); t.set(1, 0.2); t.set(2, 0.8); assert_eq!(t.above(0.5).len(), 1); assert_eq!(t.below(0.5).len(), 1); }
}