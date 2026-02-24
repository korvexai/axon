fn convert(event: AxonEvent) -> Option<WsEvent> {

    match event {

        AxonEvent::AiResponse { output, model, .. } => {
            Some(WsEvent::ChatResponse {
                text: output,
                model,
            })
        }

        AxonEvent::BuildFinished { success, duration_ms, .. } => {
            Some(WsEvent::BuildFinished {
                success,
                duration_ms: duration_ms as u64,
            })
        }

        AxonEvent::WorkerStatus { name, health } => {
            Some(WsEvent::WorkerStatusUpdate {
                name,
                health: format!("{:?}", health),
            })
        }

        AxonEvent::ChatToken { token } => {
            Some(WsEvent::ChatToken { token })
        }

        _ => None,
    }
}
