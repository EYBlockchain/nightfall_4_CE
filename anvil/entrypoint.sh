  forge_command(&["install"]);
    forge_command(&[
        "script",
        "MockDeployer",
        "--fork-url",
        &settings.ethereum_client_url,
        "--broadcast",
        "--force",
    ]);

    exec "$@"
