#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_agent_manager() {
        let agent_path: zbus::zvariant::OwnedObjectPath =
            "/org/bluez/diagnify/agent/0".try_into().unwrap();
        let capability = String::from("NoInputNoOutput");
        let conn = std::sync::Arc::new(bluebus::get_system_connection().await.unwrap());

        let agent_proxy = bluebus::AgentManagerProxy::new(&conn, "/org/bluez")
            .await
            .unwrap();

        assert!(
            agent_proxy
                .register_agent(&agent_path, capability)
                .await
                .is_ok()
        );
        assert!(agent_proxy.request_default_agent(&agent_path).await.is_ok());
        assert!(agent_proxy.unregister_agent(&agent_path).await.is_ok());
    }
}
