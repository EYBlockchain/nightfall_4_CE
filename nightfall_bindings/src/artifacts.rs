use alloy::sol;

sol!(
    #[sol(rpc)]
    #[derive(Debug)]
    Nightfall,
    "../blockchain_assets/dummy_artifacts/Nightfall.sol/Nightfall.json"
);
sol!(
    #[sol(rpc)]
    X509,
    "../blockchain_assets/dummy_artifacts/X509.sol/X509.json"
);
sol!(
    #[sol(rpc)]
    #[derive(Debug)]
    #[allow(clippy::too_many_arguments)]
    RoundRobin,
    "../blockchain_assets/dummy_artifacts/RoundRobin.sol/RoundRobin.json"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug)]
    IERC1155,
    "../blockchain_assets/dummy_artifacts/IERC1155.sol/IERC1155.json"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug)]
    IERC20,
    "../blockchain_assets/dummy_artifacts/IERC20.sol/IERC20.json"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug)]
    IERC3525,
    "../blockchain_assets/dummy_artifacts/IERC3525.sol/IERC3525.json"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug)]
    IERC721,
    "../blockchain_assets/dummy_artifacts/IERC721.sol/IERC721.json"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug)]
    ERC1155Mock,
    "../blockchain_assets/dummy_artifacts/ERC1155Mock.sol/ERC1155Mock.json"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug)]
    ERC20Mock,
    "../blockchain_assets/dummy_artifacts/ERC20Mock.sol/ERC20Mock.json"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug)]
    ERC3525Mock,
    "../blockchain_assets/dummy_artifacts/ERC3525Mock.sol/ERC3525Mock.json"
);

sol!(
    #[sol(rpc)]
    #[derive(Debug)]
    ERC721Mock,
    "../blockchain_assets/dummy_artifacts/ERC721Mock.sol/ERC721Mock.json"
);
