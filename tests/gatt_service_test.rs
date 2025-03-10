#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_gatt_service_uuid_is_ok() {
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
        if !devices.is_empty() {
            let device_address = devices.get(0).unwrap().address.replace(":", "_");
            let device_path = format!("{}/dev_{}", bluebus::ADAPTER_PATH, device_address);
            let device = bluebus::DeviceProxy::builder(&conn)
                .path(device_path.to_string())
                .unwrap()
                .build()
                .await;
            assert!(device.is_ok());
            let device = device.unwrap();

            // This assertion does not check the actual return value
            assert!(device.services_resolved().await.is_ok());
            let gatt_service_path = format!("{}/service0001", device_path);
            let gatt_service_proxy = bluebus::GattServiceProxy::new(
                &conn,
                gatt_service_path.as_str(),
                gatt_service_path.as_str(),
            )
            .await
            .unwrap();

            let uuid = gatt_service_proxy.UUID().await.unwrap();
            assert!(!uuid.is_empty());
        }
    }
}
