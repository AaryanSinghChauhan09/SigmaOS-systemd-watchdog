// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-watchdog: systemd watchdog component model.
//!
//! Real implementation: watchdogs, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Watchdog {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemdWatchdogError {
    EmptyWatchdogName,
    DuplicateWatchdog(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdWatchdogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyWatchdogName => write!(f, "watchdog name must not be empty"),
            Self::DuplicateWatchdog(id) => write!(f, "duplicate watchdog {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdWatchdogError {}

pub struct SystemdWatchdog {
    pub name: String,
    watchdogs: BTreeMap<String, Watchdog>,
    events: BTreeMap<String, Event>,
}

impl SystemdWatchdog {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            watchdogs: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_watchdog(&mut self, item: Watchdog) -> Result<(), SystemdWatchdogError> {
        if item.name.trim().is_empty() {
            return Err(SystemdWatchdogError::EmptyWatchdogName);
        }
        let id = item.id.clone();
        if self.watchdogs.contains_key(&id) {
            return Err(SystemdWatchdogError::DuplicateWatchdog(id.clone()));
        }
        self.watchdogs.insert(id, item);
        Ok(())
    }

    pub fn watchdog_count(&self) -> usize {
        self.watchdogs.len()
    }

    pub fn get_watchdog(&self, id: &str) -> Option<&Watchdog> {
        self.watchdogs.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdWatchdogError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdWatchdogError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_watchdog(id: &str, name: &str) -> Watchdog {
        Watchdog {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdWatchdog::new("SystemdWatchdog");
        assert_eq!(m.name, "SystemdWatchdog");
        assert_eq!(m.watchdog_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_watchdog() {
        let mut m = SystemdWatchdog::new("SystemdWatchdog");
        m.add_watchdog(sample_watchdog("a1", "Alpha")).unwrap();
        assert_eq!(m.watchdog_count(), 1);
        assert!(m.get_watchdog("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdWatchdog::new("SystemdWatchdog");
        assert!(m.add_watchdog(sample_watchdog("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdWatchdog::new("SystemdWatchdog");
        m.add_watchdog(sample_watchdog("x", "One")).unwrap();
        assert!(m.add_watchdog(sample_watchdog("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdWatchdog::new("SystemdWatchdog");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
