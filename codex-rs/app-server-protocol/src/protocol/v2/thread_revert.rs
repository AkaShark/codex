use super::Thread;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use ts_rs::TS;

/// Replaces paginated conversation history with the prefix before a turn.
/// This does not undo changes to files.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct ThreadRevertParams {
    pub thread_id: String,
    pub before_turn_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "v2/")]
pub struct ThreadRevertResponse {
    /// Updated metadata. Hydrate retained turns separately through pagination.
    pub thread: Thread,
    pub turns_backwards_cursor: Option<String>,
    pub items_backwards_cursor: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::ThreadRevertParams;
    use crate::ClientRequest;
    use crate::RequestId;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn revert_uses_a_stable_turn_boundary_on_the_wire() {
        let request = ClientRequest::ThreadRevert {
            request_id: RequestId::Integer(7),
            params: ThreadRevertParams {
                thread_id: "thread".to_string(),
                before_turn_id: "turn".to_string(),
            },
        };
        let wire = json!({"id":7,"method":"thread/revert","params":{"threadId":"thread","beforeTurnId":"turn"}});
        assert_eq!(serde_json::to_value(&request).unwrap(), wire);
        assert_eq!(
            serde_json::from_value::<ClientRequest>(wire).unwrap(),
            request
        );
    }
}
