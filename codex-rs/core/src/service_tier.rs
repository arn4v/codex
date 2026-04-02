use crate::codex::Session;
use crate::codex::TurnContext;
use codex_protocol::config_types::FAST_MODE_AUTO_TOKEN_LIMIT;
use codex_protocol::config_types::ServiceTier;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RequestKind {
    Turn,
    Compaction,
}

pub(crate) async fn resolve(
    session: &Session,
    turn_context: &TurnContext,
    request_kind: RequestKind,
) -> Option<ServiceTier> {
    if let Some(service_tier) = turn_context.config.service_tier {
        return Some(service_tier);
    }

    match request_kind {
        RequestKind::Compaction => Some(ServiceTier::Fast),
        RequestKind::Turn => {
            let token_count = match session.get_estimated_token_count(turn_context).await {
                Some(token_count) => token_count,
                None => session.get_total_token_usage().await,
            };
            (token_count < FAST_MODE_AUTO_TOKEN_LIMIT).then_some(ServiceTier::Fast)
        }
    }
}
