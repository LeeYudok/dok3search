use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerationState {
    Empty,
    Building,
    CatchingUp,
    Validating,
    Ready,
    Activating,
    Active,
    Retired,
    Failed,
    Recovering,
}

impl GenerationState {
    pub fn transition(self, next: Self) -> Result<Self> {
        use GenerationState::*;
        match (self, next) {
            (Empty, Building)
            | (Building, CatchingUp | Recovering | Failed)
            | (CatchingUp, Validating | Recovering | Failed)
            | (Validating, Ready | Recovering | Failed)
            | (Ready, CatchingUp | Activating | Recovering | Failed)
            | (Activating, Active | Recovering)
            | (Active, Retired)
            | (Retired, CatchingUp)
            | (Recovering, Failed) => Ok(next),
            _ => Err(Error::Invalid(format!(
                "invalid transition {self:?} -> {next:?}"
            ))),
        }
    }
}

/// The backend publishes this manifest with each durable index commit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub generation_id: String,
    pub schema_version: u64,
    pub analyzer_version: u64,
    pub acl_policy_version: u64,
    pub applied_change_sequence: u64,
    pub doc_count: u64,
}

impl Manifest {
    pub fn checksum(&self) -> Result<String> {
        checksum(self)
    }
}

pub(crate) fn checksum(value: &impl Serialize) -> Result<String> {
    Ok(dok3_storage::content_revision(&serde_json::to_vec(value)?)
        .as_str()
        .to_owned())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRecord {
    pub state: GenerationState,
    pub created_at_unix_secs: u64,
    pub retired_at_unix_secs: Option<u64>,
    pub manifest_checksum: Option<String>,
    #[serde(flatten)]
    pub manifest: Option<Manifest>,
    pub target_sequence: u64,
    pub validated_sequence: Option<u64>,
    pub files: GenerationFiles,
}

impl GenerationRecord {
    pub(crate) fn new(sequence: u64) -> Result<Self> {
        Ok(Self {
            state: GenerationState::Empty.transition(GenerationState::Building)?,
            created_at_unix_secs: now(),
            retired_at_unix_secs: None,
            manifest_checksum: None,
            manifest: None,
            target_sequence: sequence,
            validated_sequence: None,
            files: GenerationFiles::Present,
        })
    }

    pub(crate) fn verified_at(&self, sequence: u64) -> bool {
        self.validated_sequence == Some(sequence)
            && self.target_sequence == sequence
            && self
                .manifest
                .as_ref()
                .is_some_and(|manifest| manifest.applied_change_sequence == sequence)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerationFiles {
    Present,
    Deleting,
    Deleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntentStage {
    Prepared,
    RuntimeSwapped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationIntent {
    pub from_generation: Option<String>,
    pub to_generation: String,
    pub from_epoch: u64,
    pub target_epoch: u64,
    pub target_sequence: u64,
    pub intent_stage: IntentStage,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DurableState {
    pub active_generation: Option<String>,
    pub active_epoch: u64,
    pub activating_transition: Option<ActivationIntent>,
    pub generations: BTreeMap<String, GenerationRecord>,
}

pub(crate) fn valid_id(id: &str) -> bool {
    // A generation ID may become a directory name in a backend.
    !id.is_empty()
        && id.len() <= 128
        && id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

pub(crate) fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

impl DurableState {
    pub(crate) fn verify(&self) -> Result<()> {
        let corrupt = || Error::Invalid("inconsistent generation state".into());
        if self.active_generation.is_none() != (self.active_epoch == 0)
            || self.active_generation.as_ref().is_some_and(|id| {
                self.generations.get(id).map(|record| record.state) != Some(GenerationState::Active)
            })
        {
            return Err(corrupt());
        }
        for (id, record) in &self.generations {
            match (&record.manifest, &record.manifest_checksum) {
                (Some(manifest), Some(expected))
                    if &manifest.generation_id == id && &manifest.checksum()? == expected => {}
                (None, None) => {}
                _ => return Err(corrupt()),
            }
            if !valid_id(id)
                || (record.state == GenerationState::Active
                    && self.active_generation.as_ref() != Some(id))
                || (record.state == GenerationState::Activating
                    && self
                        .activating_transition
                        .as_ref()
                        .map(|intent| &intent.to_generation)
                        != Some(id))
                || (matches!(
                    record.state,
                    GenerationState::Ready | GenerationState::Activating | GenerationState::Active
                ) && !record.verified_at(record.target_sequence))
                || (record.files != GenerationFiles::Present
                    && record.state != GenerationState::Retired)
            {
                return Err(corrupt());
            }
        }
        if let Some(intent) = &self.activating_transition
            && (intent.from_generation != self.active_generation
                || intent.from_epoch != self.active_epoch
                || Some(intent.target_epoch) != self.active_epoch.checked_add(1)
                || !self
                    .generations
                    .get(&intent.to_generation)
                    .is_some_and(|record| {
                        matches!(
                            record.state,
                            GenerationState::Activating | GenerationState::Recovering
                        ) && record.verified_at(intent.target_sequence)
                    }))
        {
            return Err(corrupt());
        }
        Ok(())
    }
}
