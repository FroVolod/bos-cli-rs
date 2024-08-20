use assert_cmd::Command;
use httpmock::Method::POST;
use httpmock::MockServer;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;
use serde_json::json;

#[test]
fn test_bos_components_deploy_with_mocked_rpc() {
    // Step 1: Start a mock server to simulate the NEAR RPC server
    let server = MockServer::start();

    // Step 2: Locate the existing config directory
    let config_dir = dirs::config_dir().unwrap().join("near-cli");
    let config_path = config_dir.join("config.toml");

    // Step 3: Backup the original config.toml
    let backup_path = config_dir.join("config_backup.toml");
    fs::copy(&config_path, &backup_path).expect("Failed to backup config.toml");

    // Step 4: Modify the config.toml to use the mock server
    let mut config_content = fs::read_to_string(&config_path).expect("Failed to read config.toml");
    
    // Add or modify the [network_connection.mocknet] section
    config_content.push_str(&format!(
        r#"
[network_connection.mocknet]
network_name = "mocknet"
rpc_url = "{}"
wallet_url = "https://wallet.near.org/"
explorer_transaction_url = "https://explorer.near.org/transactions/"
linkdrop_account_id = "near"
fastnear_url = "https://api.fastnear.com/"
staking_pools_factory_account_id = "poolv1.near"
coingecko_url = "https://api.coingecko.com/"
"#,
        server.url("/"), // Pointing to the mock server
    ));

    fs::write(&config_path, config_content).expect("Failed to write to config.toml");

    // Step 5: Set up a temporary directory for components
    let temp_dir = tempdir().unwrap();
    let src_dir = temp_dir.path().join("src");
    fs::create_dir(&src_dir).unwrap();

    // Create a mock component file in the temp directory
    let component_path = src_dir.join("example_component.jsx");
    fs::write(&component_path, "console.log('Hello, world!');").unwrap();

    // Step 6: Mock the necessary RPC calls on the mock server
    let _mock = server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .body_contains("view_access_key");
        then.status(200)
            .json_body(json!({
                "jsonrpc": "2.0",
                "result": {
                    "nonce": 85,
                    "permission": {
                        "FunctionCall": {
                            "allowance": "18501534631167209000000000",
                            "receiver_id": "social.near",
                            "method_names": ["set"]
                        }
                    },
                    "block_height": 19884918,
                    "block_hash": "GGJQ8yjmo7aEoj8ZpAhGehnq9BSWFx4xswHYzDwwAP2n"
                },
                "id": "dontcare"
            }));
    });

    // Mock the RPC call for `view_access_key_list`
    server.mock(|when, then| {
        when.method(POST)
            .path("/")
            .body_contains(r#""request_type":"view_access_key_list""#);
        then.status(200)
            .json_body(json!({
                "jsonrpc": "2.0",
                "result": {
                    "keys": [
                        {
                            "public_key": "ed25519:7fvCiaE4NTmhexo8fDoa3CFNupL6mvJmNjL1hydN65fm",
                            "access_key": {
                                "nonce": 17,
                                "permission": {
                                    "FunctionCall": {
                                        "allowance": "9999203942481156415000",
                                        "receiver_id": "social.near",
                                        "method_names": ["set"]
                                    }
                                }
                            }
                        },
                        {
                            "public_key": "ed25519:4F9TwuSqWwvoyu7JVZDsupPhC7oYbYNsisBV2yQvyXFn",
                            "access_key": {
                                "nonce": 0,
                                "permission": "FullAccess"
                            }
                        }
                    ],
                    "block_height": 17798231,
                    "block_hash": "Gm7YSdx22wPuciW1jTTeRGP9mFqmon69ErFQvgcFyEEB"
                },
                "id": "dontcare"
            }));
    });

    // Step 7: Change the current directory to the temporary directory for components
    std::env::set_current_dir(&temp_dir).unwrap();

    // Step 8: Run the CLI command as a subprocess
    let mut cmd = Command::cargo_bin("bos").unwrap();

    cmd.args(&[
        "components",
        "deploy",
        "test.near",
        "sign-as",
        "test.near",
        "network-config",
        "mocknet", // Use the mock network we added
        "sign-with-plaintext-private-key",
        "--signer-public-key",
        "ed25519:7fvCiaE4NTmhexo8fDoa3CFNupL6mvJmNjL1hydN65fm",
        "--signer-private-key",
        "ed25519:VzeoRptTNGWeXwq3JNLdo8XqoBKvjSXyV5VxjvxPyzsiGmwo2Vu6LTiBujoQSXYjF8khQS5r3SSQK3xV8uomjv7",
        "send",
    ])
    .assert()
    .success()
    .stdout(predicates::str::contains("Deployment successful"));

    // Step 9: Restore the original config.toml
    fs::copy(&backup_path, &config_path).expect("Failed to restore original config.toml");

    // Step 10: Clean up the backup file
    fs::remove_file(backup_path).expect("Failed to remove backup config.toml");

    // Step 11: Clean up the temp directory is handled automatically by `tempdir`
}