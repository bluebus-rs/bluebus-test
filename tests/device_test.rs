#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_device_info() {
        let conn = std::sync::Arc::new(bluebus::get_system_connection().await.unwrap());

        let adapter = bluebus::AdapterProxy::builder(&conn)
            .path(bluebus::ADAPTER_PATH)
            .unwrap()
            .build()
            .await;
        assert!(adapter.is_ok());
        let adapter = adapter.unwrap();

        if !adapter.powered().await.unwrap() {
            adapter.set_powered(true).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            assert!(adapter.powered().await.unwrap());
        }

        adapter.start_discovery().await.unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
        adapter.stop_discovery().await.unwrap();
        let devices = bluebus::list_devices();
        for device in devices {
            assert!(device.address.len() > 0);
            assert!(device.alias.len() > 0);
        }
    }
}
