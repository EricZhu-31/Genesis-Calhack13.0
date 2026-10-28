//! Durable specialist runtime contract.
//!
//! A [`SpecialistRuntime`] creates provider-side agents for roles and runs
//! jobs on them through split, durable operations. The kernel authorizes
//! dispatch and proves readiness/domain results independently; progress is
//! provider evidence only. No concrete runtime ships in this crate today —
//! the CREAO implementation was removed — so specialist roles fall back to
//! [`crate::DevStub`] until a new runtime is registered.

use crate::{ProviderError, ProvisionRequest};
use async_trait::async_trait;
use genesis_core::ids::{
    AssignmentId, AttemptId, JobId, MissionId, OrgId, ResponsibilityId, RoleId,
};
use genesis_core::jobs::Job;
use genesis_core::model::{AgentBinding, Provider};
use serde::Serialize;
use serde_json::Value;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Backend authority check run immediately before each provider network
/// effect; returning `false` stops the effect.
pub type PreEffectGuard = Arc<dyn Fn() -> Pin<Box<dyn Future<Output = bool> + Send>> + Send + Sync>;

/// Historical, backend-collected execution provenance, not domain verification
/// or current authority. Obtain this only from a trusted runtime with the frozen
/// job and execution binding; serialized receipts are not importable proofs.
/// Callers must separately authorize any new action against current kernel state.
///
/// Execution binds the provider, agent ID and configuration digest. A binding's
/// `conversation_ref` is provisioning metadata, not an execution input: every
/// execution starts a fresh conversation. Mutable job status, result, provider
/// references and timestamps likewise are not part of the submitted request.
#[derive(Clone, PartialEq, Serialize)]
pub struct SpecialistExecutionReceipt {
    pub schema_version: u32,
    pub provider: Provider,
    pub org_id: OrgId,
    pub mission_id: MissionId,
    pub role_id: RoleId,
    pub responsibility_id: ResponsibilityId,
    pub job_id: JobId,
    pub attempt_id: AttemptId,
    pub assignment_id: AssignmentId,
    pub attempt_no: u32,
    pub epoch: u64,
    pub operation_id: String,
    pub account_namespace: String,
    pub provider_agent_id: String,
    pub binding_config_digest: String,
    pub run_id: String,
    pub conversation_id: Option<String>,
    pub workspace_id: Option<String>,
    /// Canonical hash of the submitted provider request, excluding operation
    /// ownership metadata; distinct from the full operation fingerprint below.
    pub request_sha256: String,
    /// Fingerprint of the full immutable, organization/account-bound operation.
    pub operation_fingerprint: String,
    /// Domain-separated canonical hash of the complete validated checkpoint,
    /// including its native observation and raw response. Not authentication.
    pub checkpoint_sha256: String,
    pub checkpoint_revision: u64,
    /// Native completion may have no result. Missing or malformed semantic
    /// output must be rejected by the domain consumer, not silently promoted to
    /// task success. Output retains the runtime's validated response bounds.
    pub output: Option<Value>,
    /// Canonical hash of serialized `output`; absent output hashes JSON null.
    /// The full checkpoint hash also binds the original response shape.
    pub output_sha256: String,
}

impl fmt::Debug for SpecialistExecutionReceipt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpecialistExecutionReceipt")
            .field("provider", &self.provider)
            .field("org_id", &self.org_id)
            .field("job_id", &self.job_id)
            .field("attempt_id", &self.attempt_id)
            .field("operation_id", &self.operation_id)
            .field("run_id", &self.run_id)
            .field("operation_fingerprint", &self.operation_fingerprint)
            .field("checkpoint_sha256", &self.checkpoint_sha256)
            .field("checkpoint_revision", &self.checkpoint_revision)
            .field("output", &"[redacted]")
            .field("output_sha256", &self.output_sha256)
            .finish_non_exhaustive()
    }
}

/// Progress is provider evidence only. In particular, Created requires a separate
/// readiness check before the kernel can activate a role. For API Trigger it
/// means an existing published app completed the role-bound bootstrap run, not
/// that CREAO created or modified an agent.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum SpecialistProgress {
    Pending {
        operation_id: String,
        run_id: Option<String>,
        native_status: Option<String>,
    },
    Created {
        operation_id: String,
        run_id: String,
        provider_agent_id: String,
        config_digest: String,
        readiness_verified: bool,
    },
    Collected {
        operation_id: String,
        run_id: String,
        provider_agent_id: String,
        output: Option<Value>,
        domain_verified: bool,
    },
    Failed {
        operation_id: String,
        run_id: Option<String>,
    },
    NeedsReconciliation {
        operation_id: String,
        run_id: Option<String>,
    },
}
impl SpecialistProgress {
    pub fn operation_id(&self) -> &str {
        match self {
            Self::Pending { operation_id, .. }
            | Self::Created { operation_id, .. }
            | Self::Collected { operation_id, .. }
            | Self::Failed { operation_id, .. }
            | Self::NeedsReconciliation { operation_id, .. } => operation_id,
        }
    }
}

/// A split, durable provider contract. Preparing saves an immutable local intent;
/// advancing performs at most one native operation. The existing synchronous
/// provisioning saga must not pretend Pending is a ready agent binding.
#[async_trait]
pub trait SpecialistRuntime: Send + Sync {
    fn provider(&self) -> Provider;
    fn org_id(&self) -> &str;
    fn prepare_role(&self, request: &ProvisionRequest)
        -> Result<SpecialistProgress, ProviderError>;
    fn prepare_job(
        &self,
        job: &Job,
        binding: &AgentBinding,
    ) -> Result<SpecialistProgress, ProviderError>;
    fn get(&self, operation_id: &str) -> Result<SpecialistProgress, ProviderError>;
    /// Read a completed execution bound to the authoritative frozen job and
    /// execution binding. Does not prepare, submit, poll, or infer fresh kernel
    /// authority. Implementations without bound receipts reject by default.
    fn execution_receipt(
        &self,
        _job: &Job,
        _binding: &AgentBinding,
    ) -> Result<SpecialistExecutionReceipt, ProviderError> {
        Err(rejected("specialist execution receipts are not supported"))
    }
    async fn advance(
        &self,
        operation_id: &str,
        retry_read: bool,
    ) -> Result<SpecialistProgress, ProviderError>;
    /// Check backend authority after provider queue/quota admission and before
    /// each network effect. Implementations without this guarantee reject;
    /// silently delegating to unguarded advance would drop the authority fence.
    async fn advance_guarded(
        &self,
        _operation_id: &str,
        _retry_read: bool,
        _guard: PreEffectGuard,
    ) -> Result<SpecialistProgress, ProviderError> {
        Err(rejected("guarded specialist advancement is not supported"))
    }
}

fn rejected(message: &str) -> ProviderError {
    ProviderError::Rejected(message.into())
}
