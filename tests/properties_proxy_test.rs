#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_properties_proxy_get_all() {
        let conn = std::sync::Arc::new(bluebus::get_system_connection().await.unwrap());

        if let Ok(interface_name) =
            zbus::names::InterfaceName::try_from("org.bluez.LEAdvertisingManager1")
        {
            if let Ok(proxy) =
                zbus::fdo::PropertiesProxy::new(&conn, "org.bluez", bluebus::ADAPTER_PATH).await
            {
                let ad_properties: std::collections::HashMap<String, zbus::zvariant::OwnedValue> =
                    proxy.get_all(interface_name).await.unwrap();
                assert!(!ad_properties.is_empty());
            }
        }
    }
}
