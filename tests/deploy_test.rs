use assert_cmd::Command;
use httpmock::{MockServer, Then, When};
use serde_json::{json, Value}; // Import for Value
use std::fs;
use tempfile::tempdir;

#[test]
fn test_bos_components_deploy_with_mocked_rpc() {
    // Step 1: Start a mock server to simulate the NEAR RPC server
    let server = MockServer::start();

    // Step 2: Locate the existing config directory
    let config_dir = dirs::config_dir().unwrap().join("near-cli");
    let config_path = config_dir.join("config.toml");

    // Step 3: Backup and remove the original config.toml if it exists
    let backup_path = config_dir.join("config_backup.toml");
    if config_path.exists() {
        fs::rename(&config_path, &backup_path).expect("Failed to backup original config.toml");
    }

    // Step 4: Create a new config.toml with the mock server configuration
    fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    fs::write(
        &config_path,
        format!(
            r#"
            version = "2"
            credentials_home_dir = "~/.near-credentials"

            [network_connection.mainnet]
            network_name = "mainnet"
            rpc_url = "{}"
            wallet_url = "https://app.mynearwallet.com/"
            explorer_transaction_url = "https://explorer.near.org/transactions/"
            linkdrop_account_id = "near"
            near_social_db_contract_account_id = "social.near"
            fastnear_url = "https://api.fastnear.com/"
            staking_pools_factory_account_id = "poolv1.near"
            coingecko_url = "https://api.coingecko.com/"
            "#,
            server.url("/")
        ),
    )
    .expect("Failed to create test config.toml");

    // Step 5: Set up a temporary directory for components
    let temp_dir = tempdir().unwrap();
    let src_dir = temp_dir.path().join("src");
    fs::create_dir(&src_dir).unwrap();

    // Create a mock component file in the temp directory
    let component_path = src_dir.join("example_component.jsx");
    fs::write(&component_path, "console.log('Hello, world!');").unwrap();

    // Step 6: Mock the necessary RPC calls on the mock server

    // Mock for view_access_key RPC call
    server.mock(|when: When, then: Then| {
        when.method(httpmock::Method::POST)
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
    server.mock(|when: When, then: Then| {
        when.method(httpmock::Method::POST)
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

    // Step 7: Mock the `query` RPC call for getting an empty JSON object as a result
    server.mock(|when: When, then: Then| {
        when.method(httpmock::Method::POST)
            .path("/")
            .body_contains(r#""method":"query""#)
            .body_contains(r#""request_type":"call_function""#)
            .body_contains(r#""method_name":"get""#);
        then.status(200)
            .json_body(json!({
                "jsonrpc": "2.0",
                "result": {
                    "result": [123, 125], // ASCII for `{}` is 123, 125
                    "logs": [],
                    "block_height": 17817336,
                    "block_hash": "4qkA4sUUG8opjH5Q9bL5mWJTnfR4ech879Db1BZXbx6P"
                },
                "id": "dontcare"
            }));
    });

    // Step 8: Mock the `query` RPC call for `storage_balance_of`
    server.mock(|when: When, then: Then| {
        when.method(httpmock::Method::POST)
            .path("/")
            .body_contains(r#""method":"query""#)
            .body_contains(r#""request_type":"call_function""#)
            .body_contains(r#""method_name":"storage_balance_of""#);
        then.status(200)
            .json_body(json!({
                "jsonrpc": "2.0",
                "result": {
                    "result": [123, 125], // ASCII for `{}` is 123, 125
                    "logs": [],
                    "block_height": 17817337,
                    "block_hash": "6qkA4sUUG8opjH5Q9bL5mWJTnfR4ech879Db1BZXbx7Q"
                },
                "id": "dontcare"
            }));
    });

    let write_permission = false;
    server.mock(|when, then| {
        when.body_contains("is_write_permission_granted");
        let write_permission_json_str =
            serde_json::to_string(&json!(write_permission)).unwrap();
        let binary_write_permission = write_permission_json_str.as_bytes().to_vec();
        then.json_body(json!({
          "jsonrpc": "2.0",
          "result": {
            "result": binary_write_permission,
            "logs": [],
            "block_height": 17817336,
            "block_hash": "4qkA4sUUG8opjH5Q9bL5mWJTnfR4ech879Db1BZXbx6P"
          },
          "id": "dontcare"
        }));
    });

    // Step 9: Log unmatched requests and return a 500 error
    server.mock(|when: When, then: Then| {
        when.matches(|req| {
            if let Some(body_bytes) = &req.body {
                // Convert body to string
                let body_str = String::from_utf8_lossy(body_bytes);
                if let Ok(json_body) = serde_json::from_str::<Value>(&body_str) {
                    println!(
                        "No mock for request: {}",
                        serde_json::to_string_pretty(&json_body).unwrap()
                    );
                } else {
                    println!("Failed to parse JSON body");
                }
            }
            true
        });
        then.status(500);
    });

    // Step 10: Change the current directory to the temporary directory for components
    std::env::set_current_dir(&temp_dir).unwrap();

    // Step 11: Run the CLI command as a subprocess
    let mut cmd = Command::cargo_bin("bos").unwrap();

    cmd.args(&[
        "components",
        "deploy",
        "test.near",
        "sign-as",
        "test.near",
        "network-config",
        "mainnet", // Use the mock network we added
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

    // Step 12: Restore the original config.toml if it existed
    if backup_path.exists() {
        fs::rename(&backup_path, &config_path).expect("Failed to restore original config.toml");
    } else {
        // If the original config.toml didn't exist, delete the one we created
        fs::remove_file(&config_path).expect("Failed to remove generated config.toml");
    }

    // Step 13: Clean up the temp directory is handled automatically by `tempdir`
}
