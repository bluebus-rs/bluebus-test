#[cfg(test)]
mod tests {
    use std::sync::Arc;

    #[tokio::test]
    async fn test_adapter_initialization() {
        let conn = Arc::new(bluebus::get_system_connection().await.unwrap());

        let adapter = bluebus::AdapterProxy::builder(&conn)
            .path(bluebus::ADAPTER_PATH)
            .unwrap()
            .build()
            .await;
        assert!(adapter.is_ok());
    }

    #[tokio::test]
    async fn test_set_power_off() {
        let conn = bluebus::get_system_connection().await.unwrap();
        let adapter = bluebus::AdapterProxy::builder(&conn)
            .path(bluebus::ADAPTER_PATH)
            .unwrap()
            .build()
            .await
            .unwrap();

        adapter.set_powered(false).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        assert!(!adapter.powered().await.unwrap());
    }

    #[tokio::test]
    async fn test_set_power_on() {
        let conn = bluebus::get_system_connection().await.unwrap();
        let adapter = bluebus::AdapterProxy::builder(&conn)
            .path(bluebus::ADAPTER_PATH)
            .unwrap()
            .build()
            .await
            .unwrap();

        adapter.set_powered(true).await.unwrap();
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        assert!(adapter.powered().await.unwrap());
    }

    #[tokio::test]
    async fn test_set_alias() {
        let conn = bluebus::get_system_connection().await.unwrap();
        let adapter = bluebus::AdapterProxy::builder(&conn)
            .path(bluebus::ADAPTER_PATH)
            .unwrap()
            .build()
            .await
            .unwrap();

        // adapter.set_powered(true).await.unwrap();
        let alias = adapter.alias().await.unwrap();
        if alias == "bluebus" {
            adapter.set_alias("rsdiagnify").await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            assert!(adapter.alias().await.unwrap() == "rsdiagnify");
        } else {
            adapter.set_alias("bluebus").await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            assert!(adapter.alias().await.unwrap() == "bluebus");
        }
    }

    #[tokio::test]
    async fn test_set_discoverable_on() {
        let conn = bluebus::get_system_connection().await.unwrap();
        let adapter = bluebus::AdapterProxy::builder(&conn)
            .path(bluebus::ADAPTER_PATH)
            .unwrap()
            .build()
            .await
            .unwrap();

        if !adapter.powered().await.unwrap() {
            adapter.set_powered(true).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            assert!(adapter.powered().await.unwrap());
        }
        if adapter.discoverable().await.unwrap() == false {
            adapter.set_discoverable(true).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
        assert!(adapter.discoverable().await.unwrap());
    }

    #[tokio::test]
    async fn test_set_discoverable_off() {
        let conn = bluebus::get_system_connection().await.unwrap();
        let adapter = bluebus::AdapterProxy::builder(&conn)
            .path(bluebus::ADAPTER_PATH)
            .unwrap()
            .build()
            .await
            .unwrap();
        if !adapter.powered().await.unwrap() {
            adapter.set_powered(true).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            assert!(adapter.powered().await.unwrap());
        }
        if adapter.discoverable().await.unwrap() == true {
            adapter.set_discoverable(false).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
        assert!(!adapter.discoverable().await.unwrap());
    }

    #[tokio::test]
    async fn test_start_stop_discovery() {
        let conn = bluebus::get_system_connection().await.unwrap();
        let adapter = bluebus::AdapterProxy::builder(&conn)
            .path(bluebus::ADAPTER_PATH)
            .unwrap()
            .build()
            .await
            .unwrap();
        if !adapter.powered().await.unwrap() {
            adapter.set_powered(true).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            assert!(adapter.powered().await.unwrap());
        }
        assert!(adapter.start_discovery().await.is_ok());
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        assert!(adapter.stop_discovery().await.is_ok());
    }

    #[tokio::test]
    async fn test_get_adapter_name() {
        let conn = bluebus::get_system_connection().await.unwrap();
        let adapter = bluebus::AdapterProxy::builder(&conn)
            .path(bluebus::ADAPTER_PATH)
            .unwrap()
            .build()
            .await
            .unwrap();

        if !adapter.powered().await.unwrap() {
            adapter.set_powered(true).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            assert!(adapter.powered().await.unwrap());
        }

        let name = adapter.name().await;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        assert!(name.is_ok());
        assert!(!name.unwrap().is_empty());
    }

    #[tokio::test]
    async fn test_get_adapter_address() {
        let conn = bluebus::get_system_connection().await.unwrap();
        let adapter = bluebus::AdapterProxy::builder(&conn)
            .path(bluebus::ADAPTER_PATH)
            .unwrap()
            .build()
            .await
            .unwrap();

        if !adapter.powered().await.unwrap() {
            adapter.set_powered(true).await.unwrap();
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            assert!(adapter.powered().await.unwrap());
        }
        let address = adapter.address().await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        assert!(address.contains(":"));
    }
}
