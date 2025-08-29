pub use nightfall::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod nightfall {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("UPGRADE_INTERFACE_VERSION"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UPGRADE_INTERFACE_VERSION",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("descrow_funds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("descrow_funds"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct WithdrawData"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token_type"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum TokenType"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("escrow_funds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("escrow_funds"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ercAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("secretHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token_type"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum TokenType"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTokenInfo"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getTokenInfo"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nfTokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ercAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hash_transaction"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hash_transaction"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("txn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                4usize,
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                4usize,
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                4usize,
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct OnChainTransaction",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "initialNullifierRoot",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr_verifier"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract INFVerifier"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x509_address"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "sanctionsListAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("layer2_block_number"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "layer2_block_number",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC1155BatchReceived"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "onERC1155BatchReceived",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC1155Received"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC1155Received"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC3525Received"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC3525Received"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC721Received"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("onERC721Received"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("propose_block"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("propose_block"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blk"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ),
                                                                4usize,
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ),
                                                                4usize,
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ),
                                                                4usize,
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Block"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proxiableUUID"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setAuthorities"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setAuthorities"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "sanctionsListAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x509Address"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("set_proposer_manager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "set_proposer_manager",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "proposer_manager_address",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ProposerManager"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("set_sanctions_list"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("set_sanctions_list"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "sanctionsListAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("set_x509_address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("set_x509_address"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x509_address"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sha256_and_shift"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sha256_and_shift"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("inputs"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgradeToAndCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verify_rollup_proof"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "verify_rollup_proof",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("blk"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ),
                                                                4usize,
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ),
                                                                4usize,
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ),
                                                                4usize,
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Block"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("public_hash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw_processed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw_processed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct WithdrawData"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BlockProposed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("BlockProposed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "layer2_block_number",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DepositEscrowed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("DepositEscrowed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("nfSlotId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("AddressEmptyCode"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("target"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1967InvalidImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC1967InvalidImplementation",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1967NonPayable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ERC1967NonPayable"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FailedCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("FailedCall"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidInitialization"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidInitialization",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotInitializing"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotInitializing"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UUPSUnauthorizedCallContext"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UUPSUnauthorizedCallContext",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UUPSUnsupportedProxiableUUID"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "UUPSUnsupportedProxiableUUID",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("slot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("escrowFundsError"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("escrowFundsError"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static NIGHTFALL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R0`\x80R_`\x03\x81\x90U`\x07\x81\x90U\x7F\x0Cp5\xBFJ:#zL\x08\x13\x07\xAC\xE7\xB8\x0BX0\x97N+G8\xD8\x8F\x15\xC3\xEA\x19\xC3\x8Cq`\x08U`\tU4\x80\x15a\0EW_\x80\xFD[P_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x01\x80T\x82\x16\x90U`\x02\x80T\x90\x91\x163\x17\x90U`\x80Qa3\xC8a\0\x91_9_\x81\x81a$\x0B\x01R\x81\x81a$4\x01Ra%\x9F\x01Ra3\xC8_\xF3\xFE`\x80`@R`\x046\x10a\x01;W_5`\xE0\x1C\x80c\x8Czc\xAE\x11a\0\xB3W\x80c\xD8k\xF9w\x11a\0mW\x80c\xD8k\xF9w\x14a\x04!W\x80c\xE3\x17\x8C\x86\x14a\x04@W\x80c\xE6\xD5\xAB\xE5\x14a\x04_W\x80c\xE7\xD8:\x88\x14a\x04rW\x80c\xF2:na\x14a\x04\x91W\x80c\xF3\xB8_\xC2\x14a\x04\xBDW_\x80\xFD[\x80c\x8Czc\xAE\x14a\x02\xE0W\x80c\x8D\xA5\xCB[\x14a\x03PW\x80c\x97\xE2\r\x0E\x14a\x03\x87W\x80c\xAD<\xB1\xCC\x14a\x03\xA6W\x80c\xBC\x19|\x81\x14a\x03\xE3W\x80c\xD5\xE6\xC6\xF9\x14a\x04\x02W_\x80\xFD[\x80c(\xC3\xD7\xE6\x11a\x01\x04W\x80c(\xC3\xD7\xE6\x14a\x027W\x80cJ\x8A@s\x14a\x02ZW\x80cO\x1E\xF2\x86\x14a\x02{W\x80cR\xD1\x90-\x14a\x02\x8EW\x80cu\x13xu\x14a\x02\xA2W\x80cuM\x1DT\x14a\x02\xC1W_\x80\xFD[\x80b\x9C\xE2\x0B\x14a\x01?W\x80c\x01\xFF\xC9\xA7\x14a\x01\x88W\x80c\x05\xCD\x0E\x98\x14a\x01\xB7W\x80c\x15\x0Bz\x02\x14a\x01\xEDW\x80c\x18jM\x08\x14a\x02\x18W[_\x80\xFD[4\x80\x15a\x01JW_\x80\xFD[Pa\x01ja\x01Y6`\x04a(lV[b\x9C\xE2\x0B`\xE0\x1B\x96\x95PPPPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x93W_\x80\xFD[Pa\x01\xA7a\x01\xA26`\x04a(\xD9V[a\x04\xD0V[`@Q\x90\x15\x15\x81R` \x01a\x01\x7FV[4\x80\x15a\x01\xC2W_\x80\xFD[Pa\x01\xD6a\x01\xD16`\x04a)\x16V[a\x05;V[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x7FV[4\x80\x15a\x01\xF8W_\x80\xFD[Pa\x01ja\x02\x076`\x04a)WV[c\n\x85\xBD\x01`\xE1\x1B\x95\x94PPPPPV[4\x80\x15a\x02#W_\x80\xFD[Pa\x01\xA7a\x0226`\x04a)\xD4V[a\n\x1BV[4\x80\x15a\x02BW_\x80\xFD[Pa\x02L`\x03T\x81V[`@Q\x90\x81R` \x01a\x01\x7FV[4\x80\x15a\x02eW_\x80\xFD[Pa\x02ya\x02t6`\x04a)\xEEV[a\n_V[\0[a\x02ya\x02\x896`\x04a*\xE9V[a\n\xC2V[4\x80\x15a\x02\x99W_\x80\xFD[Pa\x02La\n\xE1V[4\x80\x15a\x02\xADW_\x80\xFD[Pa\x02ya\x02\xBC6`\x04a+5V[a\n\xFCV[4\x80\x15a\x02\xCCW_\x80\xFD[Pa\x02ya\x02\xDB6`\x04a+nV[a\x13\xABV[4\x80\x15a\x02\xEBW_\x80\xFD[Pa\x031a\x02\xFA6`\x04a+\xBEV[_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x01\x90\x91\x01T\x92\x90\x91\x01\x82\x90R\x91V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x7FV[4\x80\x15a\x03[W_\x80\xFD[P`\x02Ta\x03o\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x7FV[4\x80\x15a\x03\x92W_\x80\xFD[Pa\x02ya\x03\xA16`\x04a+\xD5V[a\x15OV[4\x80\x15a\x03\xB1W_\x80\xFD[Pa\x03\xD6`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x01\x7F\x91\x90a,=V[4\x80\x15a\x03\xEEW_\x80\xFD[Pa\x01ja\x03\xFD6`\x04a,\x8FV[a\x15\x9BV[4\x80\x15a\x04\rW_\x80\xFD[Pa\x02ya\x04\x1C6`\x04a+\xD5V[a\x15\xE5V[4\x80\x15a\x04,W_\x80\xFD[Pa\x02La\x04;6`\x04a-\x92V[a\x160V[4\x80\x15a\x04KW_\x80\xFD[Pa\x02ya\x04Z6`\x04a+\xD5V[a\x16\xECV[a\x02ya\x04m6`\x04a.\0V[a\x178V[4\x80\x15a\x04}W_\x80\xFD[Pa\x02La\x04\x8C6`\x04a.VV[a\x1E\xDCV[4\x80\x15a\x04\x9CW_\x80\xFD[Pa\x01ja\x04\xAB6`\x04a.\x87V[c\xF2:na`\xE0\x1B\x96\x95PPPPPPV[a\x02ya\x04\xCB6`\x04a.\xDFV[a\x1E\xFFV[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x80a\x04\xFFWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16b\x9C\xE2\x0B`\xE0\x1B\x14[\x80a\x05\x1AWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\n\x85\xBD\x01`\xE1\x1B\x14[\x80a\x055WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x02q\x18\x97`\xE5\x1B\x14[\x92\x91PPV[_\x80\x80a\x05K`\x80\x86\x01\x86a/\x11V[a\x05Y\x91` \x91_\x91a/SV[\x81\x01\x90a\x05f\x91\x90a+\xBEV[`@\x80Q`\x18\x80\x82Ra\x03 \x82\x01\x90\x92R\x91\x92P\x82\x91_\x91` \x82\x01a\x03\0\x806\x837\x01\x90PP\x90P\x82\x81_\x81Q\x81\x10a\x05\xA2Wa\x05\xA2a/zV[` \x02` \x01\x01\x81\x81RPP\x85_\x1B\x81`\x01\x81Q\x81\x10a\x05\xC4Wa\x05\xC4a/zV[` \x02` \x01\x01\x81\x81RPP`\x07T_\x1B\x81`\x02\x81Q\x81\x10a\x05\xE8Wa\x05\xE8a/zV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80Q\x875\x90\x82\x90`\x03\x90\x81\x10a\x06\x0BWa\x06\x0Ba/zV[` \x02` \x01\x01\x81\x81RPP`\x08T_\x1B\x81`\x04\x81Q\x81\x10a\x06/Wa\x06/a/zV[` \x02` \x01\x01\x81\x81RPP\x86` \x015_\x1B\x81`\x05\x81Q\x81\x10a\x06UWa\x06Ua/zV[` \x02` \x01\x01\x81\x81RPP`\tT_\x1B\x81`\x06\x81Q\x81\x10a\x06yWa\x06ya/zV[` \x02` \x01\x01\x81\x81RPP\x86`@\x015_\x1B\x81`\x07\x81Q\x81\x10a\x06\x9FWa\x06\x9Fa/zV[` \x02` \x01\x01\x81\x81RPPa\x06\xB3a'\xFCV[a\x06\xBBa'\xFCV[a\x06\xFBa\x06\xCB`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91`@\x91` \x91a/SV[\x81\x01\x90a\x06\xE7\x91\x90a+\xBEV[`\x01`\x01`\xF8\x1B\x03\x81\x16\x91`\xF8\x91\x90\x91\x1C\x90V[\x82R\x82Ra\x07\x1Ea\x07\x0F`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91``\x91`@\x91a/SV[` \x83\x81\x01\x91\x90\x91R\x83\x01Ra\x07Ia\x07:`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91`\x80\x91``\x91a/SV[`@\x83\x81\x01\x91\x90\x91R\x83\x01Ra\x07ta\x07e`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91`\xA0\x91`\x80\x91a/SV[``\x83\x81\x01\x91\x90\x91R\x83\x01Ra\x07\x9Fa\x07\x90`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91`\xC0\x91`\xA0\x91a/SV[\x83`\x04` \x02\x01\x83`\x04` \x02\x01\x91\x90\x91RRa\x07\xD1a\x07\xC2`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91`\xE0\x91`\xC0\x91a/SV[`\xA0\x83\x81\x01\x91\x90\x91R\x83\x01Ra\x07\xFDa\x07\xED`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91a\x01\0\x91`\xE0\x91a/SV[`\xC0\x83\x81\x01\x91\x90\x91R\x83\x01Ra\x08*a\x08\x19`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91a\x01 \x91a\x01\0\x91a/SV[`\xE0\x83\x81\x01\x91\x90\x91R\x83\x01R_[`\x08\x81\x10\x15a\x08\xDCW\x82\x81`\x08\x81\x10a\x08SWa\x08Sa/zV[` \x02\x01Q\x84a\x08d\x83`\x02a/\xA2V[a\x08o\x90`\x08a/\xB9V[\x81Q\x81\x10a\x08\x7FWa\x08\x7Fa/zV[` \x02` \x01\x01\x81\x81RPP\x81\x81`\x08\x81\x10a\x08\x9DWa\x08\x9Da/zV[` \x02\x01Q\x84a\x08\xAE\x83`\x02a/\xA2V[a\x08\xB9\x90`\ta/\xB9V[\x81Q\x81\x10a\x08\xC9Wa\x08\xC9a/zV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x088V[P_a\t\x06\x84`@Q` \x01a\x08\xF2\x91\x90a/\xCCV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1E\xDCV[\x90Pa\t2\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x82a0\x01V[\x90P_\x81`@Q` \x01a\tH\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R`\x0BT\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16c\xDE\x8FP\xA1a\ty`\x80\x8E\x01\x8Ea/\x11V[a\t\x89\x91a\x01 \x91` \x91a/SV[\x8E\x80`\x80\x01\x90a\t\x99\x91\x90a/\x11V[a\t\xA8\x91a\x01 \x90\x82\x90a/SV[\x86`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xC9\x95\x94\x93\x92\x91\x90a0HV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xE4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x08\x91\x90a0\x8EV[\x98P\x94\x96PPPPPPP[\x92P\x92\x90PV[_\x80\x82`@Q` \x01a\n.\x91\x90a0\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x90\x81R`\x05\x90\x92R\x90 T`\xFF\x16`\x01\x14\x93\x92PPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a0\xECV[`@Q\x80\x91\x03\x90\xFD[_\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x93\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[a\n\xCAa$\0V[a\n\xD3\x82a$\xA6V[a\n\xDD\x82\x82a$\xD3V[PPV[_a\n\xEAa%\x94V[P_\x80Q` a3s\x839\x81Q\x91R\x90V[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Be\x91\x90a0\x8EV[a\x0B\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a1#V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xC7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xEB\x91\x90a0\x8EV[\x15a\x0C\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a1\x80V[`\nT`@\x80Qc\x1DC\xC9}`\xE2\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cu\x0F%\xF4\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0COW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cs\x91\x90a1\xD4V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FOnly the current proposer can pr`D\x82\x01Rlopose a block`\x98\x1B`d\x82\x01R`\x84\x01a\n\x89V[``\x81\x015\x81\x015_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\0Wa\r\0a*%V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r)W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a\r\x92Wa\rma\rF``\x86\x01\x86a1\xEFV[\x83\x81\x81\x10a\rVWa\rVa/zV[\x90Pa\x01\xA0\x02\x01\x806\x03\x81\x01\x90a\x04;\x91\x90a-\x92V[\x82\x82\x81Q\x81\x10a\r\x7FWa\r\x7Fa/zV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\r.V[P\x80\x82[`\x01\x81\x11\x15a\x0E@W_[`\x01\x82\x90\x1C\x81\x10\x15a\x0E7Wa\x0E\x12\x83`\x01\x83\x90\x1B\x81Q\x81\x10a\r\xC6Wa\r\xC6a/zV[` \x02` \x01\x01Q\x84`\x01\x84\x90\x1B`\x01a\r\xE0\x91\x90a/\xB9V[\x81Q\x81\x10a\r\xF0Wa\r\xF0a/zV[` \x02` \x01\x01Q`@Q` \x01a\x08\xF2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[\x83\x82\x81Q\x81\x10a\x0E$Wa\x0E$a/zV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\r\xA1V[P`\x01\x1Ca\r\x96V[P_\x80a\x0Ef\x86\x85_\x81Q\x81\x10a\x0EYWa\x0EYa/zV[` \x02` \x01\x01Qa\x05;V[\x91P\x91P\x81a\x0E\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FRollup proof verification failed`D\x82\x01R`d\x01a\n\x89V[_[\x85\x81\x10\x15a\x122W``\x87\x015a\x01\xA0\x82\x02\x01\x87\x01`\xC0\x015\x15\x80\x15a\x10*W``\x88\x015a\x01\xA0\x83\x02\x01\x88\x01a\x01@\x015\x15\x80\x15a\x0E\xF9WPPa\x12*V[\x83_\x80[`\x04\x81\x10\x15a\x10\x1DW``\x8C\x015a\x01\xA0\x87\x02\x01` \x82\x02\x01\x8C\x01a\x01@\x015\x91P\x81\x15a\x10\x15W_\x82\x81R`\x04` R`@\x90 Ta\x0F=\x90\x84a/\xB9V[_\x83\x81R`\x04` R`@\x90 `\x01\x90\x81\x01T\x91\x94P`\xFF\x90\x91\x16\x14\x80\x15a\x0F{WP_\x82\x81R`\x04` R`@\x90 `\x01\x01Ta\x01\0\x90\x04`\xFF\x16\x15[a\x0F\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDeposit either has not been escr`D\x82\x01R\x7Fowed or has already been redeeme`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\n\x89V[_\x82\x81R`\x04` R`@\x90 `\x01\x01\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01\x01a\x0E\xFDV[P\x81\x95PPPPPa\x12*V[`@``\x89\x015a\x01\xA0\x84\x02\x01\x89\x01\x90\x81\x015\x15`\xC0\x91\x90\x91\x015\x15\x15\x16\x80\x15a\x12'W_a\x10\\``\x8B\x01\x8Ba1\xEFV[\x85\x81\x81\x10a\x10lWa\x10la/zV[\x90Pa\x01\xA0\x02\x01`\xA0\x01_`\x04\x81\x10a\x10\x87Wa\x10\x87a/zV[` \x02\x015\x90P_`@Q\x80`\x80\x01`@R\x80\x8C\x80``\x01\x90a\x10\xAA\x91\x90a1\xEFV[\x88\x81\x81\x10a\x10\xBAWa\x10\xBAa/zV[\x90Pa\x01\xA0\x02\x01a\x01 \x01_`\x04\x81\x10a\x10\xD6Wa\x10\xD6a/zV[` \x02\x015\x81R` \x01\x8C\x80``\x01\x90a\x10\xF0\x91\x90a1\xEFV[\x88\x81\x81\x10a\x11\0Wa\x11\0a/zV[\x90Pa\x01\xA0\x02\x01a\x01 \x01`\x01`\x04\x81\x10a\x11\x1DWa\x11\x1Da/zV[` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x80``\x01\x90a\x11@\x91\x90a1\xEFV[\x88\x81\x81\x10a\x11PWa\x11Pa/zV[\x90Pa\x01\xA0\x02\x01a\x01 \x01`\x02`\x04\x81\x10a\x11mWa\x11ma/zV[` \x90\x81\x02\x91\x90\x91\x015\x82R\x90\x81\x01\x84\x90R`@\x80Q\x83Q\x81R\x83\x83\x01Q\x81\x84\x01R\x81\x84\x01Q\x81\x83\x01R``\x80\x85\x01Q\x90\x82\x01R`\x80\x90 _\x81\x81R`\x05\x90\x93R\x91 T\x91\x92P\x90`\xFF\x16\x15a\x12\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FFunds have already withdrawn\0\0\0\0`D\x82\x01R`d\x01a\n\x89V[_\x90\x81R`\x05` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UPa\x12*\x92PPPV[PP[`\x01\x01a\x0E\xB9V[P\x855`\x07U` \x80\x87\x015`\x08U`@\x80\x88\x015`\tU`\nT\x81Qc\x1DC\xC9}`\xE2\x1B\x81R\x91Q_\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92cu\x0F%\xF4\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x12\x8EW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xB2\x91\x90a1\xD4V[\x90P_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x12\xFDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x13\x02V[``\x91P[PP\x90P\x80a\x13fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FFailed to transfer the fee to th`D\x82\x01Ri2\x90897\xB87\xB9\xB2\xB9`\xB1\x1B`d\x82\x01R`\x84\x01a\n\x89V[`\x03\x80T\x90_a\x13u\x83a24V[\x90\x91UP`@Q\x7FF\xBF\x14\x88'\xA7N\xB5\xC7\xCA\x85\xB1\x05\x131\xF7\xE0|kQ#\0X\x0CG\xBF\x1D\xD6F\xC1\xD8\x95\x90_\x90\xA2PPPPPPPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16_\x81\x15\x80\x15a\x13\xEFWP\x82[\x90P_\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x14\nWP0;\x15[\x90P\x81\x15\x80\x15a\x14\x18WP\x80\x15[\x15a\x146W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x14`W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x14ha%\xDDV[`\x08\x89\x90U`\x02\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U_\x80T\x82\x16`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x91\x90\x91\x17\x82U`\x01\x80T\x84\x16\x8A\x83\x16\x17\x81U`\x0B\x80T\x85\x16\x8D\x84\x16\x17\x90U`@\x80Q0\x80\x82R` \x80\x83\x01\x87\x90R\x91\x83\x90 `\x04\x1C`\x0C\x81\x90U\x83Q\x80\x85\x01\x85R\x91\x82R\x81\x83\x01\x87\x81R\x90\x87R`\x06\x90\x92R\x91\x90\x94 \x90Q\x81T\x90\x95\x16\x94\x90\x92\x16\x93\x90\x93\x17\x81U\x90Q\x91\x01U\x83\x15a\x15DW\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x15yW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a0\xECV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FUnsupported by Nightfall\0\0\0\0\0\0\0\0`D\x82\x01R_\x90`d\x01a\n\x89V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x16\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a0\xECV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[` \x81\x81\x01Q\x80Q\x81\x83\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82\x88\x01Q\x80Q\x81\x89\x01Q\x82\x86\x01Q\x92\x89\x01Q\x89\x8C\x01Q\x80Q\x81\x8D\x01Q\x82\x8A\x01Q\x92\x8D\x01Q\x8AQ\x9E\x8F\x01\x9C\x90\x9CR\x8D\x8A\x01\x9A\x90\x9AR\x9A\x8C\x01\x96\x90\x96R`\x80\x8B\x01\x94\x90\x94R`\xA0\x8A\x01\x91\x90\x91R`\xC0\x89\x01R`\xE0\x88\x01Ra\x01\0\x87\x01Ra\x01 \x86\x01\x94\x90\x94Ra\x01@\x85\x01\x91\x90\x91Ra\x01`\x84\x01\x92\x90\x92Ra\x01\x80\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81Ra\x01\xA0\x90\x92\x01\x90R_\x90a\x16\xE5\x81a\x1E\xDCV[\x93\x92PPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a0\xECV[`\n\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17}W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xA1\x91\x90a0\x8EV[a\x17\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a1#V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x03W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18'\x91\x90a0\x8EV[\x15a\x18DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a1\x80V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R\x90\x81\x01\x85\x90R_\x90a\x18k\x90``\x01a\x08\xF2V[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82R` \x80\x83\x01\x8A\x81R_\x86\x81R`\x06\x90\x92R\x93\x81 \x92Q\x83T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x16\x91\x90\x91\x17\x82U\x91Q`\x01\x90\x91\x01U\x90\x91P`\x03\x83`\x03\x81\x11\x15a\x18\xCAWa\x18\xCAa2RV[\x14a\x18\xD5W\x81a\x19uV[`@Qc\x13\x1F\x9F?`\xE1\x1B\x81R`\x04\x81\x81\x01\x88\x90R\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c&?>~\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x1CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19@\x91\x90a2fV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x84\x01R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1C\x90\x1C[`@\x80Q`\x80\x80\x82\x01\x83R\x85\x82R` \x80\x83\x01\x85\x81R\x83\x85\x01\x8B\x81R``\x80\x86\x01\x8C\x81R\x87Q\x94\x85\x01\x8B\x90R\x92Q\x96\x84\x01\x96\x90\x96RQ\x94\x82\x01\x94\x90\x94R\x92Q\x90\x83\x01R\x91\x92P_\x90a\x19\xC9\x90`\xA0\x01a\x08\xF2V[_\x81\x81R`\x04` R`@\x90 `\x01\x01T\x90\x91P`\xFF\x16\x15a\x1AGW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FFunds have already been escrowed`D\x82\x01Rp\x08\x19\x9B\xDC\x88\x1D\x1A\x1A\\\xC8\x11\x19\\\x1B\xDC\xDA]`z\x1B`d\x82\x01R`\x84\x01a\n\x89V[`\x03\x85`\x03\x81\x11\x15a\x1A[Wa\x1A[a2RV[\x03a\x1A\xC7W`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c#\xB8r\xDD\x90`d\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xACW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\xBEW=_\x80>=_\xFD[PPPPa\x1C\x9BV[`\x01\x85`\x03\x81\x11\x15a\x1A\xDBWa\x1A\xDBa2RV[\x03a\x1B\x12W`@Qcy!!\x95`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xF2BC*\x90a\x1A\x95\x903\x900\x90\x8D\x90\x8D\x90`\x04\x01a2}V[`\x02\x85`\x03\x81\x11\x15a\x1B&Wa\x1B&a2RV[\x03a\x1B\x8FW\x86\x15a\x1BIW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a2\xB4V[`@Qc\\F\xA7\xEF`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x89\x90R`\x80`d\x82\x01R_`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xB8\x8DO\xDE\x90`\xA4\x01a\x1A\x95V[_\x85`\x03\x81\x11\x15a\x1B\xA2Wa\x1B\xA2a2RV[\x03a\x1C\x82W\x87\x15a\x1B\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a2\xFDV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C\x15W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C9\x91\x90a0\x8EV[a\x1C}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11T\x90\xCC\x8C\x08\x1D\x1C\x98[\x9C\xD9\x99\\\x88\x19\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\n\x89V[a\x1C\x9BV[`@Qc\xAF\xA9\x05\xDD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q``\x81\x01\x82R\x8B\x81R`\x01` \x80\x83\x01\x82\x81R_\x84\x86\x01\x81\x81R\x87\x82R`\x04\x90\x93R\x85\x90 \x93Q\x84UQ\x92\x90\x91\x01\x80T\x91Q`\xFF\x90\x81\x16a\x01\0\x02a\xFF\xFF\x19\x90\x93\x16\x93\x16\x92\x90\x92\x17\x17\x90UQ\x7F5G)\xB3XZ\xC9Q\x8AvY\xE8\xB323\x89\xF1H\xA7S\xE8\xC3\xC5\xD8\xC0\x1B\xFE\t\xBC\0\x93@\x90a\x1D#\x90\x85\x90\x8A\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1a\x1D6\x8A`\x02a/\xA2V[4\x11\x15a\x1E\xD0W_a\x1DI\x8B`\x02a/\xA2V[a\x1DS\x904a3DV[\x90P_`@Q\x80`\x80\x01`@R\x80`\x0CT\x81R` \x01`\x0CT\x81R` \x01\x83\x81R` \x01\x89\x81RP\x90P_a\x1D\xB8\x82`@Q` \x01a\x08\xF2\x91\x90\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[_\x81\x81R`\x04` R`@\x90 `\x01\x01T\x90\x91P`\xFF\x16\x15a\x1E:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FFunds have already been escrowed`D\x82\x01Rt\x08\x19\x9B\xDC\x88\x1D\x1A\x1A\\\xC8\x19\x99YH\x11\x19\\\x1B\xDC\xDA]`Z\x1B`d\x82\x01R`\x84\x01a\n\x89V[`@\x80Q``\x81\x01\x82R\x8E\x81R`\x01` \x80\x83\x01\x82\x81R_\x84\x86\x01\x81\x81R\x87\x82R`\x04\x90\x93R\x85\x90 \x93Q\x84UQ\x92\x90\x91\x01\x80T\x91Q`\xFF\x90\x81\x16a\x01\0\x02a\xFF\xFF\x19\x90\x93\x16\x93\x16\x92\x90\x92\x17\x17\x90U`\x0CT\x90Q\x7F5G)\xB3XZ\xC9Q\x8AvY\xE8\xB323\x89\xF1H\xA7S\xE8\xC3\xC5\xD8\xC0\x1B\xFE\t\xBC\0\x93@\x91a\x1E\xC4\x91\x86\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPP[PPPPPPPPPPV[_`@Q` \x81\x84Q` \x86\x01`\x02Z\xFAa\x1E\xF5W_\x80\xFD[Q`\x04\x1C\x92\x91PPV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1FDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Fh\x91\x90a0\x8EV[a\x1F\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a1#V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xCAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xEE\x91\x90a0\x8EV[\x15a \x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a1\x80V[_\x82`@Q` \x01a \x1D\x91\x90a0\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R`\x05\x90\x93R\x91 T\x90\x91P`\xFF\x16`\x01\x14a \xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FEither no funds are available to`D\x82\x01R\x7F withdraw, or they are already w`d\x82\x01Rg4\xBA4290\xBB\xB7`\xC1\x1B`\x84\x82\x01R`\xA4\x01a\n\x89V[\x825_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x01\x90\x91\x01T\x91\x83\x01\x91\x90\x91Ra!\xB9W_a!\x18`@\x86\x01` \x87\x01a+\xD5V[`\x01`\x01`\xA0\x1B\x03\x16\x85`@\x015`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a!cW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a!hV[``\x91P[PP\x90P\x80a!\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuCould not withdraw fee`P\x1B`D\x82\x01R`d\x01a\n\x89V[PPPPPV[_\x82\x81R`\x05` R`@\x81 \x80T`\xFF\x19\x16\x90U`\x01\x84`\x03\x81\x11\x15a!\xE2Wa!\xE2a2RV[\x03a\"gW\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xF2BC*0a\"\x08`@\x89\x01` \x8A\x01a+\xD5V[\x85` \x01Q\x89`@\x015`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\"1\x94\x93\x92\x91\x90a2}V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"HW_\x80\xFD[PZ\xF1\x15\x80\x15a\"ZW=_\x80>=_\xFD[PPPP`\x01\x90Pa#\xDCV[`\x02\x84`\x03\x81\x11\x15a\"{Wa\"{a2RV[\x03a#\x0FW`@\x85\x015\x15a\"\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a2\xB4V[\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xB8\x8DO\xDE0a\"\xC3`@\x89\x01` \x8A\x01a+\xD5V[` \x86\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`\x80`d\x82\x01R_`\x84\x82\x01R`\xA4\x01a\"1V[_\x84`\x03\x81\x11\x15a#\"Wa#\"a2RV[\x03a#\xDCW` \x82\x01Q\x15a#IW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a2\xFDV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa#i`@\x88\x01` \x89\x01a+\xD5V[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R\x88\x015`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\xB5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xD9\x91\x90a0\x8EV[\x90P[\x80a!\xB2W_\x83\x81R`\x05` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UPPPPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a$\x86WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a$z_\x80Q` a3s\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a$\xA4W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a$\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a0\xECV[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a%-WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra%*\x91\x81\x01\x90a2fV[`\x01[a%UW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\n\x89V[_\x80Q` a3s\x839\x81Q\x91R\x81\x14a%\x85W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\n\x89V[a%\x8F\x83\x83a%\xE5V[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a$\xA4W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$\xA4a&:V[a%\xEE\x82a&\x83V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a&2Wa%\x8F\x82\x82a&\xE6V[a\n\xDDa'XV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a$\xA4W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a&\xB8W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\n\x89V[_\x80Q` a3s\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa'\x02\x91\x90a3WV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a':W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a'?V[``\x91P[P\x91P\x91Pa'O\x85\x83\x83a'wV[\x95\x94PPPPPV[4\x15a$\xA4W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a'\x8CWa'\x87\x82a'\xD3V[a\x16\xE5V[\x81Q\x15\x80\x15a'\xA3WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a'\xCCW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\n\x89V[P\x92\x91PPV[\x80Q\x15a'\xE3W\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80a\x01\0\x01`@R\x80`\x08\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$\xD0W_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a(?W_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a(UW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\n\x14W_\x80\xFD[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a(\x81W_\x80\xFD[\x865a(\x8C\x81a(\x1BV[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xBBW_\x80\xFD[a(\xC7\x89\x82\x8A\x01a(/V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_` \x82\x84\x03\x12\x15a(\xE9W_\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x16\xE5W_\x80\xFD[_`\xA0\x82\x84\x03\x12\x15a)\x10W_\x80\xFD[P\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a)'W_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a)<W_\x80\xFD[a)H\x85\x82\x86\x01a)\0V[\x95` \x94\x90\x94\x015\x94PPPPV[_\x80_\x80_`\x80\x86\x88\x03\x12\x15a)kW_\x80\xFD[\x855a)v\x81a(\x1BV[\x94P` \x86\x015a)\x86\x81a(\x1BV[\x93P`@\x86\x015\x92P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xA7W_\x80\xFD[a)\xB3\x88\x82\x89\x01a(/V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_`\x80\x82\x84\x03\x12\x15a)\x10W_\x80\xFD[_`\x80\x82\x84\x03\x12\x15a)\xE4W_\x80\xFD[a\x16\xE5\x83\x83a)\xC4V[_\x80`@\x83\x85\x03\x12\x15a)\xFFW_\x80\xFD[\x825a*\n\x81a(\x1BV[\x91P` \x83\x015a*\x1A\x81a(\x1BV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a*[Wa*[a*%V[`@R\x90V[_\x82`\x1F\x83\x01\x12a*pW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a*\x8AWa*\x8Aa*%V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a*\xB2Wa*\xB2a*%V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a*\xCAW_\x80\xFD[\x83` \x87\x01` \x83\x017_` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a*\xFAW_\x80\xFD[\x825a+\x05\x81a(\x1BV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a+\x1FW_\x80\xFD[a++\x85\x82\x86\x01a*aV[\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a+EW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a+ZW_\x80\xFD[a+f\x84\x82\x85\x01a)\0V[\x94\x93PPPPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a+\x81W_\x80\xFD[\x845\x93P` \x85\x015a+\x93\x81a(\x1BV[\x92P`@\x85\x015a+\xA3\x81a(\x1BV[\x91P``\x85\x015a+\xB3\x81a(\x1BV[\x93\x96\x92\x95P\x90\x93PPV[_` \x82\x84\x03\x12\x15a+\xCEW_\x80\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15a+\xE5W_\x80\xFD[\x815a\x16\xE5\x81a(\x1BV[_[\x83\x81\x10\x15a,\nW\x81\x81\x01Q\x83\x82\x01R` \x01a+\xF2V[PP_\x91\x01RV[_\x81Q\x80\x84Ra,)\x81` \x86\x01` \x86\x01a+\xF0V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a\x16\xE5` \x83\x01\x84a,\x12V[_\x80\x83`\x1F\x84\x01\x12a,_W_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a,uW_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\n\x14W_\x80\xFD[_\x80_\x80_\x80_\x80`\xA0\x89\x8B\x03\x12\x15a,\xA6W_\x80\xFD[\x885a,\xB1\x81a(\x1BV[\x97P` \x89\x015a,\xC1\x81a(\x1BV[\x96P`@\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a,\xDCW_\x80\xFD[a,\xE8\x8C\x83\x8D\x01a,OV[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15a-\0W_\x80\xFD[a-\x0C\x8C\x83\x8D\x01a,OV[\x90\x96P\x94P`\x80\x8B\x015\x91P\x80\x82\x11\x15a-$W_\x80\xFD[Pa-1\x8B\x82\x8C\x01a(/V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[_\x82`\x1F\x83\x01\x12a-TW_\x80\xFD[a-\\a*9V[\x80`\x80\x84\x01\x85\x81\x11\x15a-mW_\x80\xFD[\x84[\x81\x81\x10\x15a-\x87W\x805\x84R` \x93\x84\x01\x93\x01a-oV[P\x90\x95\x94PPPPPV[_a\x01\xA0\x82\x84\x03\x12\x15a-\xA3W_\x80\xFD[a-\xABa*9V[\x825\x81Ra-\xBC\x84` \x85\x01a-EV[` \x82\x01Ra-\xCE\x84`\xA0\x85\x01a-EV[`@\x82\x01Ra-\xE1\x84a\x01 \x85\x01a-EV[``\x82\x01R\x93\x92PPPV[\x805`\x04\x81\x10a-\xFBW_\x80\xFD[\x91\x90PV[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a.\x15W_\x80\xFD[\x865\x95P` \x87\x015a.'\x81a(\x1BV[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015\x91Pa.J`\xA0\x88\x01a-\xEDV[\x90P\x92\x95P\x92\x95P\x92\x95V[_` \x82\x84\x03\x12\x15a.fW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a.{W_\x80\xFD[a+f\x84\x82\x85\x01a*aV[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a.\x9CW_\x80\xFD[\x865a.\xA7\x81a(\x1BV[\x95P` \x87\x015a.\xB7\x81a(\x1BV[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xBBW_\x80\xFD[_\x80`\xA0\x83\x85\x03\x12\x15a.\xF0W_\x80\xFD[a.\xFA\x84\x84a)\xC4V[\x91Pa/\x08`\x80\x84\x01a-\xEDV[\x90P\x92P\x92\x90PV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a/&W_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a/?W_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\n\x14W_\x80\xFD[_\x80\x85\x85\x11\x15a/aW_\x80\xFD[\x83\x86\x11\x15a/mW_\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x055Wa\x055a/\x8EV[\x80\x82\x01\x80\x82\x11\x15a\x055Wa\x055a/\x8EV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a/\xF5W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a/\xD9V[P\x92\x96\x95PPPPPPV[_\x82a0\x1BWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[``\x81R_a0[``\x83\x01\x87\x89a0 V[\x82\x81\x03` \x84\x01Ra0n\x81\x86\x88a0 V[\x90P\x82\x81\x03`@\x84\x01Ra0\x82\x81\x85a,\x12V[\x98\x97PPPPPPPPV[_` \x82\x84\x03\x12\x15a0\x9EW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x16\xE5W_\x80\xFD[\x815\x81R`\x80\x81\x01` \x83\x015a0\xC3\x81a(\x1BV[`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R`@\x83\x81\x015\x90\x83\x01R``\x92\x83\x015\x92\x90\x91\x01\x91\x90\x91R\x90V[` \x80\x82R`\x17\x90\x82\x01R\x7FCaller is not the owner\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[` \x80\x82R`=\x90\x82\x01R\x7FCertified: You are not authorise`@\x82\x01R\x7Fd to transact using Nightfall\0\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`4\x90\x82\x01R\x7FCertified: You are on the Chaina`@\x82\x01Rs\x1B\x1E\\\xDA\\\xC8\x1C\xD8[\x98\xDD\x1A[\xDB\x9C\xC8\x1B\x1A\\\xDD`b\x1B``\x82\x01R`\x80\x01\x90V[_` \x82\x84\x03\x12\x15a1\xE4W_\x80\xFD[\x81Qa\x16\xE5\x81a(\x1BV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a2\x04W_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a2\x1DW_\x80\xFD[` \x01\x91Pa\x01\xA0\x81\x026\x03\x82\x13\x15a\n\x14W_\x80\xFD[_`\x01`\x01`\xFF\x1B\x01\x82\x01a2KWa2Ka/\x8EV[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a2vW_\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01R`@\x82\x01R``\x81\x01\x91\x90\x91R`\xA0`\x80\x82\x01\x81\x90R_\x90\x82\x01R`\xC0\x01\x90V[` \x80\x82R`)\x90\x82\x01R\x7FERC721 tokens should have a valu`@\x82\x01Rhe of zero`\xB8\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`'\x90\x82\x01R\x7FERC20 tokens should have a token`@\x82\x01Rf\x04\x96B\x06\xF6b\x03`\xCC\x1B``\x82\x01R`\x80\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x055Wa\x055a/\x8EV[_\x82Qa3h\x81\x84` \x87\x01a+\xF0V[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA2dipfsX\"\x12 \xC1Y\xC4\x82\xE1\x9C{\xCF;\xDB\x7FJU\x98FT6*\xEE(K6\x9D\x0C\x83y\x12\x1C\x8B\xF3+\x99dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static NIGHTFALL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01;W_5`\xE0\x1C\x80c\x8Czc\xAE\x11a\0\xB3W\x80c\xD8k\xF9w\x11a\0mW\x80c\xD8k\xF9w\x14a\x04!W\x80c\xE3\x17\x8C\x86\x14a\x04@W\x80c\xE6\xD5\xAB\xE5\x14a\x04_W\x80c\xE7\xD8:\x88\x14a\x04rW\x80c\xF2:na\x14a\x04\x91W\x80c\xF3\xB8_\xC2\x14a\x04\xBDW_\x80\xFD[\x80c\x8Czc\xAE\x14a\x02\xE0W\x80c\x8D\xA5\xCB[\x14a\x03PW\x80c\x97\xE2\r\x0E\x14a\x03\x87W\x80c\xAD<\xB1\xCC\x14a\x03\xA6W\x80c\xBC\x19|\x81\x14a\x03\xE3W\x80c\xD5\xE6\xC6\xF9\x14a\x04\x02W_\x80\xFD[\x80c(\xC3\xD7\xE6\x11a\x01\x04W\x80c(\xC3\xD7\xE6\x14a\x027W\x80cJ\x8A@s\x14a\x02ZW\x80cO\x1E\xF2\x86\x14a\x02{W\x80cR\xD1\x90-\x14a\x02\x8EW\x80cu\x13xu\x14a\x02\xA2W\x80cuM\x1DT\x14a\x02\xC1W_\x80\xFD[\x80b\x9C\xE2\x0B\x14a\x01?W\x80c\x01\xFF\xC9\xA7\x14a\x01\x88W\x80c\x05\xCD\x0E\x98\x14a\x01\xB7W\x80c\x15\x0Bz\x02\x14a\x01\xEDW\x80c\x18jM\x08\x14a\x02\x18W[_\x80\xFD[4\x80\x15a\x01JW_\x80\xFD[Pa\x01ja\x01Y6`\x04a(lV[b\x9C\xE2\x0B`\xE0\x1B\x96\x95PPPPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\x93W_\x80\xFD[Pa\x01\xA7a\x01\xA26`\x04a(\xD9V[a\x04\xD0V[`@Q\x90\x15\x15\x81R` \x01a\x01\x7FV[4\x80\x15a\x01\xC2W_\x80\xFD[Pa\x01\xD6a\x01\xD16`\x04a)\x16V[a\x05;V[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x7FV[4\x80\x15a\x01\xF8W_\x80\xFD[Pa\x01ja\x02\x076`\x04a)WV[c\n\x85\xBD\x01`\xE1\x1B\x95\x94PPPPPV[4\x80\x15a\x02#W_\x80\xFD[Pa\x01\xA7a\x0226`\x04a)\xD4V[a\n\x1BV[4\x80\x15a\x02BW_\x80\xFD[Pa\x02L`\x03T\x81V[`@Q\x90\x81R` \x01a\x01\x7FV[4\x80\x15a\x02eW_\x80\xFD[Pa\x02ya\x02t6`\x04a)\xEEV[a\n_V[\0[a\x02ya\x02\x896`\x04a*\xE9V[a\n\xC2V[4\x80\x15a\x02\x99W_\x80\xFD[Pa\x02La\n\xE1V[4\x80\x15a\x02\xADW_\x80\xFD[Pa\x02ya\x02\xBC6`\x04a+5V[a\n\xFCV[4\x80\x15a\x02\xCCW_\x80\xFD[Pa\x02ya\x02\xDB6`\x04a+nV[a\x13\xABV[4\x80\x15a\x02\xEBW_\x80\xFD[Pa\x031a\x02\xFA6`\x04a+\xBEV[_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x01\x90\x91\x01T\x92\x90\x91\x01\x82\x90R\x91V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x7FV[4\x80\x15a\x03[W_\x80\xFD[P`\x02Ta\x03o\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x7FV[4\x80\x15a\x03\x92W_\x80\xFD[Pa\x02ya\x03\xA16`\x04a+\xD5V[a\x15OV[4\x80\x15a\x03\xB1W_\x80\xFD[Pa\x03\xD6`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\x01\x7F\x91\x90a,=V[4\x80\x15a\x03\xEEW_\x80\xFD[Pa\x01ja\x03\xFD6`\x04a,\x8FV[a\x15\x9BV[4\x80\x15a\x04\rW_\x80\xFD[Pa\x02ya\x04\x1C6`\x04a+\xD5V[a\x15\xE5V[4\x80\x15a\x04,W_\x80\xFD[Pa\x02La\x04;6`\x04a-\x92V[a\x160V[4\x80\x15a\x04KW_\x80\xFD[Pa\x02ya\x04Z6`\x04a+\xD5V[a\x16\xECV[a\x02ya\x04m6`\x04a.\0V[a\x178V[4\x80\x15a\x04}W_\x80\xFD[Pa\x02La\x04\x8C6`\x04a.VV[a\x1E\xDCV[4\x80\x15a\x04\x9CW_\x80\xFD[Pa\x01ja\x04\xAB6`\x04a.\x87V[c\xF2:na`\xE0\x1B\x96\x95PPPPPPV[a\x02ya\x04\xCB6`\x04a.\xDFV[a\x1E\xFFV[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x80a\x04\xFFWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16b\x9C\xE2\x0B`\xE0\x1B\x14[\x80a\x05\x1AWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\n\x85\xBD\x01`\xE1\x1B\x14[\x80a\x055WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x02q\x18\x97`\xE5\x1B\x14[\x92\x91PPV[_\x80\x80a\x05K`\x80\x86\x01\x86a/\x11V[a\x05Y\x91` \x91_\x91a/SV[\x81\x01\x90a\x05f\x91\x90a+\xBEV[`@\x80Q`\x18\x80\x82Ra\x03 \x82\x01\x90\x92R\x91\x92P\x82\x91_\x91` \x82\x01a\x03\0\x806\x837\x01\x90PP\x90P\x82\x81_\x81Q\x81\x10a\x05\xA2Wa\x05\xA2a/zV[` \x02` \x01\x01\x81\x81RPP\x85_\x1B\x81`\x01\x81Q\x81\x10a\x05\xC4Wa\x05\xC4a/zV[` \x02` \x01\x01\x81\x81RPP`\x07T_\x1B\x81`\x02\x81Q\x81\x10a\x05\xE8Wa\x05\xE8a/zV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80Q\x875\x90\x82\x90`\x03\x90\x81\x10a\x06\x0BWa\x06\x0Ba/zV[` \x02` \x01\x01\x81\x81RPP`\x08T_\x1B\x81`\x04\x81Q\x81\x10a\x06/Wa\x06/a/zV[` \x02` \x01\x01\x81\x81RPP\x86` \x015_\x1B\x81`\x05\x81Q\x81\x10a\x06UWa\x06Ua/zV[` \x02` \x01\x01\x81\x81RPP`\tT_\x1B\x81`\x06\x81Q\x81\x10a\x06yWa\x06ya/zV[` \x02` \x01\x01\x81\x81RPP\x86`@\x015_\x1B\x81`\x07\x81Q\x81\x10a\x06\x9FWa\x06\x9Fa/zV[` \x02` \x01\x01\x81\x81RPPa\x06\xB3a'\xFCV[a\x06\xBBa'\xFCV[a\x06\xFBa\x06\xCB`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91`@\x91` \x91a/SV[\x81\x01\x90a\x06\xE7\x91\x90a+\xBEV[`\x01`\x01`\xF8\x1B\x03\x81\x16\x91`\xF8\x91\x90\x91\x1C\x90V[\x82R\x82Ra\x07\x1Ea\x07\x0F`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91``\x91`@\x91a/SV[` \x83\x81\x01\x91\x90\x91R\x83\x01Ra\x07Ia\x07:`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91`\x80\x91``\x91a/SV[`@\x83\x81\x01\x91\x90\x91R\x83\x01Ra\x07ta\x07e`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91`\xA0\x91`\x80\x91a/SV[``\x83\x81\x01\x91\x90\x91R\x83\x01Ra\x07\x9Fa\x07\x90`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91`\xC0\x91`\xA0\x91a/SV[\x83`\x04` \x02\x01\x83`\x04` \x02\x01\x91\x90\x91RRa\x07\xD1a\x07\xC2`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91`\xE0\x91`\xC0\x91a/SV[`\xA0\x83\x81\x01\x91\x90\x91R\x83\x01Ra\x07\xFDa\x07\xED`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91a\x01\0\x91`\xE0\x91a/SV[`\xC0\x83\x81\x01\x91\x90\x91R\x83\x01Ra\x08*a\x08\x19`\x80\x8B\x01\x8Ba/\x11V[a\x06\xDA\x91a\x01 \x91a\x01\0\x91a/SV[`\xE0\x83\x81\x01\x91\x90\x91R\x83\x01R_[`\x08\x81\x10\x15a\x08\xDCW\x82\x81`\x08\x81\x10a\x08SWa\x08Sa/zV[` \x02\x01Q\x84a\x08d\x83`\x02a/\xA2V[a\x08o\x90`\x08a/\xB9V[\x81Q\x81\x10a\x08\x7FWa\x08\x7Fa/zV[` \x02` \x01\x01\x81\x81RPP\x81\x81`\x08\x81\x10a\x08\x9DWa\x08\x9Da/zV[` \x02\x01Q\x84a\x08\xAE\x83`\x02a/\xA2V[a\x08\xB9\x90`\ta/\xB9V[\x81Q\x81\x10a\x08\xC9Wa\x08\xC9a/zV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x088V[P_a\t\x06\x84`@Q` \x01a\x08\xF2\x91\x90a/\xCCV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1E\xDCV[\x90Pa\t2\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x82a0\x01V[\x90P_\x81`@Q` \x01a\tH\x91\x81R` \x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R`\x0BT\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16c\xDE\x8FP\xA1a\ty`\x80\x8E\x01\x8Ea/\x11V[a\t\x89\x91a\x01 \x91` \x91a/SV[\x8E\x80`\x80\x01\x90a\t\x99\x91\x90a/\x11V[a\t\xA8\x91a\x01 \x90\x82\x90a/SV[\x86`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xC9\x95\x94\x93\x92\x91\x90a0HV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xE4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x08\x91\x90a0\x8EV[\x98P\x94\x96PPPPPPP[\x92P\x92\x90PV[_\x80\x82`@Q` \x01a\n.\x91\x90a0\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x90\x81R`\x05\x90\x92R\x90 T`\xFF\x16`\x01\x14\x93\x92PPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a0\xECV[`@Q\x80\x91\x03\x90\xFD[_\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x93\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[a\n\xCAa$\0V[a\n\xD3\x82a$\xA6V[a\n\xDD\x82\x82a$\xD3V[PPV[_a\n\xEAa%\x94V[P_\x80Q` a3s\x839\x81Q\x91R\x90V[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Be\x91\x90a0\x8EV[a\x0B\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a1#V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xC7W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xEB\x91\x90a0\x8EV[\x15a\x0C\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a1\x80V[`\nT`@\x80Qc\x1DC\xC9}`\xE2\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cu\x0F%\xF4\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0COW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Cs\x91\x90a1\xD4V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FOnly the current proposer can pr`D\x82\x01Rlopose a block`\x98\x1B`d\x82\x01R`\x84\x01a\n\x89V[``\x81\x015\x81\x015_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\0Wa\r\0a*%V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r)W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a\r\x92Wa\rma\rF``\x86\x01\x86a1\xEFV[\x83\x81\x81\x10a\rVWa\rVa/zV[\x90Pa\x01\xA0\x02\x01\x806\x03\x81\x01\x90a\x04;\x91\x90a-\x92V[\x82\x82\x81Q\x81\x10a\r\x7FWa\r\x7Fa/zV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\r.V[P\x80\x82[`\x01\x81\x11\x15a\x0E@W_[`\x01\x82\x90\x1C\x81\x10\x15a\x0E7Wa\x0E\x12\x83`\x01\x83\x90\x1B\x81Q\x81\x10a\r\xC6Wa\r\xC6a/zV[` \x02` \x01\x01Q\x84`\x01\x84\x90\x1B`\x01a\r\xE0\x91\x90a/\xB9V[\x81Q\x81\x10a\r\xF0Wa\r\xF0a/zV[` \x02` \x01\x01Q`@Q` \x01a\x08\xF2\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[\x83\x82\x81Q\x81\x10a\x0E$Wa\x0E$a/zV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\r\xA1V[P`\x01\x1Ca\r\x96V[P_\x80a\x0Ef\x86\x85_\x81Q\x81\x10a\x0EYWa\x0EYa/zV[` \x02` \x01\x01Qa\x05;V[\x91P\x91P\x81a\x0E\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FRollup proof verification failed`D\x82\x01R`d\x01a\n\x89V[_[\x85\x81\x10\x15a\x122W``\x87\x015a\x01\xA0\x82\x02\x01\x87\x01`\xC0\x015\x15\x80\x15a\x10*W``\x88\x015a\x01\xA0\x83\x02\x01\x88\x01a\x01@\x015\x15\x80\x15a\x0E\xF9WPPa\x12*V[\x83_\x80[`\x04\x81\x10\x15a\x10\x1DW``\x8C\x015a\x01\xA0\x87\x02\x01` \x82\x02\x01\x8C\x01a\x01@\x015\x91P\x81\x15a\x10\x15W_\x82\x81R`\x04` R`@\x90 Ta\x0F=\x90\x84a/\xB9V[_\x83\x81R`\x04` R`@\x90 `\x01\x90\x81\x01T\x91\x94P`\xFF\x90\x91\x16\x14\x80\x15a\x0F{WP_\x82\x81R`\x04` R`@\x90 `\x01\x01Ta\x01\0\x90\x04`\xFF\x16\x15[a\x0F\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDeposit either has not been escr`D\x82\x01R\x7Fowed or has already been redeeme`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\n\x89V[_\x82\x81R`\x04` R`@\x90 `\x01\x01\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01\x01a\x0E\xFDV[P\x81\x95PPPPPa\x12*V[`@``\x89\x015a\x01\xA0\x84\x02\x01\x89\x01\x90\x81\x015\x15`\xC0\x91\x90\x91\x015\x15\x15\x16\x80\x15a\x12'W_a\x10\\``\x8B\x01\x8Ba1\xEFV[\x85\x81\x81\x10a\x10lWa\x10la/zV[\x90Pa\x01\xA0\x02\x01`\xA0\x01_`\x04\x81\x10a\x10\x87Wa\x10\x87a/zV[` \x02\x015\x90P_`@Q\x80`\x80\x01`@R\x80\x8C\x80``\x01\x90a\x10\xAA\x91\x90a1\xEFV[\x88\x81\x81\x10a\x10\xBAWa\x10\xBAa/zV[\x90Pa\x01\xA0\x02\x01a\x01 \x01_`\x04\x81\x10a\x10\xD6Wa\x10\xD6a/zV[` \x02\x015\x81R` \x01\x8C\x80``\x01\x90a\x10\xF0\x91\x90a1\xEFV[\x88\x81\x81\x10a\x11\0Wa\x11\0a/zV[\x90Pa\x01\xA0\x02\x01a\x01 \x01`\x01`\x04\x81\x10a\x11\x1DWa\x11\x1Da/zV[` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x80``\x01\x90a\x11@\x91\x90a1\xEFV[\x88\x81\x81\x10a\x11PWa\x11Pa/zV[\x90Pa\x01\xA0\x02\x01a\x01 \x01`\x02`\x04\x81\x10a\x11mWa\x11ma/zV[` \x90\x81\x02\x91\x90\x91\x015\x82R\x90\x81\x01\x84\x90R`@\x80Q\x83Q\x81R\x83\x83\x01Q\x81\x84\x01R\x81\x84\x01Q\x81\x83\x01R``\x80\x85\x01Q\x90\x82\x01R`\x80\x90 _\x81\x81R`\x05\x90\x93R\x91 T\x91\x92P\x90`\xFF\x16\x15a\x12\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FFunds have already withdrawn\0\0\0\0`D\x82\x01R`d\x01a\n\x89V[_\x90\x81R`\x05` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UPa\x12*\x92PPPV[PP[`\x01\x01a\x0E\xB9V[P\x855`\x07U` \x80\x87\x015`\x08U`@\x80\x88\x015`\tU`\nT\x81Qc\x1DC\xC9}`\xE2\x1B\x81R\x91Q_\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92cu\x0F%\xF4\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x12\x8EW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xB2\x91\x90a1\xD4V[\x90P_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x12\xFDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x13\x02V[``\x91P[PP\x90P\x80a\x13fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FFailed to transfer the fee to th`D\x82\x01Ri2\x90897\xB87\xB9\xB2\xB9`\xB1\x1B`d\x82\x01R`\x84\x01a\n\x89V[`\x03\x80T\x90_a\x13u\x83a24V[\x90\x91UP`@Q\x7FF\xBF\x14\x88'\xA7N\xB5\xC7\xCA\x85\xB1\x05\x131\xF7\xE0|kQ#\0X\x0CG\xBF\x1D\xD6F\xC1\xD8\x95\x90_\x90\xA2PPPPPPPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16_\x81\x15\x80\x15a\x13\xEFWP\x82[\x90P_\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x14\nWP0;\x15[\x90P\x81\x15\x80\x15a\x14\x18WP\x80\x15[\x15a\x146W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x14`W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x14ha%\xDDV[`\x08\x89\x90U`\x02\x80T3`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U_\x80T\x82\x16`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16\x91\x90\x91\x17\x82U`\x01\x80T\x84\x16\x8A\x83\x16\x17\x81U`\x0B\x80T\x85\x16\x8D\x84\x16\x17\x90U`@\x80Q0\x80\x82R` \x80\x83\x01\x87\x90R\x91\x83\x90 `\x04\x1C`\x0C\x81\x90U\x83Q\x80\x85\x01\x85R\x91\x82R\x81\x83\x01\x87\x81R\x90\x87R`\x06\x90\x92R\x91\x90\x94 \x90Q\x81T\x90\x95\x16\x94\x90\x92\x16\x93\x90\x93\x17\x81U\x90Q\x91\x01U\x83\x15a\x15DW\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x15yW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a0\xECV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FUnsupported by Nightfall\0\0\0\0\0\0\0\0`D\x82\x01R_\x90`d\x01a\n\x89V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x16\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a0\xECV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[` \x81\x81\x01Q\x80Q\x81\x83\x01Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82\x88\x01Q\x80Q\x81\x89\x01Q\x82\x86\x01Q\x92\x89\x01Q\x89\x8C\x01Q\x80Q\x81\x8D\x01Q\x82\x8A\x01Q\x92\x8D\x01Q\x8AQ\x9E\x8F\x01\x9C\x90\x9CR\x8D\x8A\x01\x9A\x90\x9AR\x9A\x8C\x01\x96\x90\x96R`\x80\x8B\x01\x94\x90\x94R`\xA0\x8A\x01\x91\x90\x91R`\xC0\x89\x01R`\xE0\x88\x01Ra\x01\0\x87\x01Ra\x01 \x86\x01\x94\x90\x94Ra\x01@\x85\x01\x91\x90\x91Ra\x01`\x84\x01\x92\x90\x92Ra\x01\x80\x80\x84\x01\x91\x90\x91R\x81Q\x80\x84\x03\x90\x91\x01\x81Ra\x01\xA0\x90\x92\x01\x90R_\x90a\x16\xE5\x81a\x1E\xDCV[\x93\x92PPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a0\xECV[`\n\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17}W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\xA1\x91\x90a0\x8EV[a\x17\xBDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a1#V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x03W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18'\x91\x90a0\x8EV[\x15a\x18DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a1\x80V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R\x90\x81\x01\x85\x90R_\x90a\x18k\x90``\x01a\x08\xF2V[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82R` \x80\x83\x01\x8A\x81R_\x86\x81R`\x06\x90\x92R\x93\x81 \x92Q\x83T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x16\x91\x90\x91\x17\x82U\x91Q`\x01\x90\x91\x01U\x90\x91P`\x03\x83`\x03\x81\x11\x15a\x18\xCAWa\x18\xCAa2RV[\x14a\x18\xD5W\x81a\x19uV[`@Qc\x13\x1F\x9F?`\xE1\x1B\x81R`\x04\x81\x81\x01\x88\x90R\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c&?>~\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x1CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19@\x91\x90a2fV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x84\x01R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1C\x90\x1C[`@\x80Q`\x80\x80\x82\x01\x83R\x85\x82R` \x80\x83\x01\x85\x81R\x83\x85\x01\x8B\x81R``\x80\x86\x01\x8C\x81R\x87Q\x94\x85\x01\x8B\x90R\x92Q\x96\x84\x01\x96\x90\x96RQ\x94\x82\x01\x94\x90\x94R\x92Q\x90\x83\x01R\x91\x92P_\x90a\x19\xC9\x90`\xA0\x01a\x08\xF2V[_\x81\x81R`\x04` R`@\x90 `\x01\x01T\x90\x91P`\xFF\x16\x15a\x1AGW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FFunds have already been escrowed`D\x82\x01Rp\x08\x19\x9B\xDC\x88\x1D\x1A\x1A\\\xC8\x11\x19\\\x1B\xDC\xDA]`z\x1B`d\x82\x01R`\x84\x01a\n\x89V[`\x03\x85`\x03\x81\x11\x15a\x1A[Wa\x1A[a2RV[\x03a\x1A\xC7W`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c#\xB8r\xDD\x90`d\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1A\xACW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\xBEW=_\x80>=_\xFD[PPPPa\x1C\x9BV[`\x01\x85`\x03\x81\x11\x15a\x1A\xDBWa\x1A\xDBa2RV[\x03a\x1B\x12W`@Qcy!!\x95`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xF2BC*\x90a\x1A\x95\x903\x900\x90\x8D\x90\x8D\x90`\x04\x01a2}V[`\x02\x85`\x03\x81\x11\x15a\x1B&Wa\x1B&a2RV[\x03a\x1B\x8FW\x86\x15a\x1BIW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a2\xB4V[`@Qc\\F\xA7\xEF`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x89\x90R`\x80`d\x82\x01R_`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xB8\x8DO\xDE\x90`\xA4\x01a\x1A\x95V[_\x85`\x03\x81\x11\x15a\x1B\xA2Wa\x1B\xA2a2RV[\x03a\x1C\x82W\x87\x15a\x1B\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a2\xFDV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C\x15W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C9\x91\x90a0\x8EV[a\x1C}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11T\x90\xCC\x8C\x08\x1D\x1C\x98[\x9C\xD9\x99\\\x88\x19\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\n\x89V[a\x1C\x9BV[`@Qc\xAF\xA9\x05\xDD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q``\x81\x01\x82R\x8B\x81R`\x01` \x80\x83\x01\x82\x81R_\x84\x86\x01\x81\x81R\x87\x82R`\x04\x90\x93R\x85\x90 \x93Q\x84UQ\x92\x90\x91\x01\x80T\x91Q`\xFF\x90\x81\x16a\x01\0\x02a\xFF\xFF\x19\x90\x93\x16\x93\x16\x92\x90\x92\x17\x17\x90UQ\x7F5G)\xB3XZ\xC9Q\x8AvY\xE8\xB323\x89\xF1H\xA7S\xE8\xC3\xC5\xD8\xC0\x1B\xFE\t\xBC\0\x93@\x90a\x1D#\x90\x85\x90\x8A\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1a\x1D6\x8A`\x02a/\xA2V[4\x11\x15a\x1E\xD0W_a\x1DI\x8B`\x02a/\xA2V[a\x1DS\x904a3DV[\x90P_`@Q\x80`\x80\x01`@R\x80`\x0CT\x81R` \x01`\x0CT\x81R` \x01\x83\x81R` \x01\x89\x81RP\x90P_a\x1D\xB8\x82`@Q` \x01a\x08\xF2\x91\x90\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[_\x81\x81R`\x04` R`@\x90 `\x01\x01T\x90\x91P`\xFF\x16\x15a\x1E:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FFunds have already been escrowed`D\x82\x01Rt\x08\x19\x9B\xDC\x88\x1D\x1A\x1A\\\xC8\x19\x99YH\x11\x19\\\x1B\xDC\xDA]`Z\x1B`d\x82\x01R`\x84\x01a\n\x89V[`@\x80Q``\x81\x01\x82R\x8E\x81R`\x01` \x80\x83\x01\x82\x81R_\x84\x86\x01\x81\x81R\x87\x82R`\x04\x90\x93R\x85\x90 \x93Q\x84UQ\x92\x90\x91\x01\x80T\x91Q`\xFF\x90\x81\x16a\x01\0\x02a\xFF\xFF\x19\x90\x93\x16\x93\x16\x92\x90\x92\x17\x17\x90U`\x0CT\x90Q\x7F5G)\xB3XZ\xC9Q\x8AvY\xE8\xB323\x89\xF1H\xA7S\xE8\xC3\xC5\xD8\xC0\x1B\xFE\t\xBC\0\x93@\x91a\x1E\xC4\x91\x86\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPP[PPPPPPPPPPV[_`@Q` \x81\x84Q` \x86\x01`\x02Z\xFAa\x1E\xF5W_\x80\xFD[Q`\x04\x1C\x92\x91PPV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1FDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Fh\x91\x90a0\x8EV[a\x1F\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a1#V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F\xCAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xEE\x91\x90a0\x8EV[\x15a \x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a1\x80V[_\x82`@Q` \x01a \x1D\x91\x90a0\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R`\x05\x90\x93R\x91 T\x90\x91P`\xFF\x16`\x01\x14a \xCDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FEither no funds are available to`D\x82\x01R\x7F withdraw, or they are already w`d\x82\x01Rg4\xBA4290\xBB\xB7`\xC1\x1B`\x84\x82\x01R`\xA4\x01a\n\x89V[\x825_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x01\x90\x91\x01T\x91\x83\x01\x91\x90\x91Ra!\xB9W_a!\x18`@\x86\x01` \x87\x01a+\xD5V[`\x01`\x01`\xA0\x1B\x03\x16\x85`@\x015`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a!cW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a!hV[``\x91P[PP\x90P\x80a!\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuCould not withdraw fee`P\x1B`D\x82\x01R`d\x01a\n\x89V[PPPPPV[_\x82\x81R`\x05` R`@\x81 \x80T`\xFF\x19\x16\x90U`\x01\x84`\x03\x81\x11\x15a!\xE2Wa!\xE2a2RV[\x03a\"gW\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xF2BC*0a\"\x08`@\x89\x01` \x8A\x01a+\xD5V[\x85` \x01Q\x89`@\x015`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\"1\x94\x93\x92\x91\x90a2}V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"HW_\x80\xFD[PZ\xF1\x15\x80\x15a\"ZW=_\x80>=_\xFD[PPPP`\x01\x90Pa#\xDCV[`\x02\x84`\x03\x81\x11\x15a\"{Wa\"{a2RV[\x03a#\x0FW`@\x85\x015\x15a\"\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a2\xB4V[\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xB8\x8DO\xDE0a\"\xC3`@\x89\x01` \x8A\x01a+\xD5V[` \x86\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`\x80`d\x82\x01R_`\x84\x82\x01R`\xA4\x01a\"1V[_\x84`\x03\x81\x11\x15a#\"Wa#\"a2RV[\x03a#\xDCW` \x82\x01Q\x15a#IW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a2\xFDV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa#i`@\x88\x01` \x89\x01a+\xD5V[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R\x88\x015`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\xB5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xD9\x91\x90a0\x8EV[\x90P[\x80a!\xB2W_\x83\x81R`\x05` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UPPPPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a$\x86WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a$z_\x80Q` a3s\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a$\xA4W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a$\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\n\x89\x90a0\xECV[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a%-WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra%*\x91\x81\x01\x90a2fV[`\x01[a%UW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\n\x89V[_\x80Q` a3s\x839\x81Q\x91R\x81\x14a%\x85W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\n\x89V[a%\x8F\x83\x83a%\xE5V[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a$\xA4W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a$\xA4a&:V[a%\xEE\x82a&\x83V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a&2Wa%\x8F\x82\x82a&\xE6V[a\n\xDDa'XV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a$\xA4W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a&\xB8W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\n\x89V[_\x80Q` a3s\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa'\x02\x91\x90a3WV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a':W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a'?V[``\x91P[P\x91P\x91Pa'O\x85\x83\x83a'wV[\x95\x94PPPPPV[4\x15a$\xA4W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a'\x8CWa'\x87\x82a'\xD3V[a\x16\xE5V[\x81Q\x15\x80\x15a'\xA3WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a'\xCCW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\n\x89V[P\x92\x91PPV[\x80Q\x15a'\xE3W\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80a\x01\0\x01`@R\x80`\x08\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$\xD0W_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a(?W_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a(UW_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\n\x14W_\x80\xFD[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a(\x81W_\x80\xFD[\x865a(\x8C\x81a(\x1BV[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xBBW_\x80\xFD[a(\xC7\x89\x82\x8A\x01a(/V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_` \x82\x84\x03\x12\x15a(\xE9W_\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x16\xE5W_\x80\xFD[_`\xA0\x82\x84\x03\x12\x15a)\x10W_\x80\xFD[P\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a)'W_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a)<W_\x80\xFD[a)H\x85\x82\x86\x01a)\0V[\x95` \x94\x90\x94\x015\x94PPPPV[_\x80_\x80_`\x80\x86\x88\x03\x12\x15a)kW_\x80\xFD[\x855a)v\x81a(\x1BV[\x94P` \x86\x015a)\x86\x81a(\x1BV[\x93P`@\x86\x015\x92P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xA7W_\x80\xFD[a)\xB3\x88\x82\x89\x01a(/V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_`\x80\x82\x84\x03\x12\x15a)\x10W_\x80\xFD[_`\x80\x82\x84\x03\x12\x15a)\xE4W_\x80\xFD[a\x16\xE5\x83\x83a)\xC4V[_\x80`@\x83\x85\x03\x12\x15a)\xFFW_\x80\xFD[\x825a*\n\x81a(\x1BV[\x91P` \x83\x015a*\x1A\x81a(\x1BV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a*[Wa*[a*%V[`@R\x90V[_\x82`\x1F\x83\x01\x12a*pW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a*\x8AWa*\x8Aa*%V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a*\xB2Wa*\xB2a*%V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a*\xCAW_\x80\xFD[\x83` \x87\x01` \x83\x017_` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[_\x80`@\x83\x85\x03\x12\x15a*\xFAW_\x80\xFD[\x825a+\x05\x81a(\x1BV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a+\x1FW_\x80\xFD[a++\x85\x82\x86\x01a*aV[\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a+EW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a+ZW_\x80\xFD[a+f\x84\x82\x85\x01a)\0V[\x94\x93PPPPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a+\x81W_\x80\xFD[\x845\x93P` \x85\x015a+\x93\x81a(\x1BV[\x92P`@\x85\x015a+\xA3\x81a(\x1BV[\x91P``\x85\x015a+\xB3\x81a(\x1BV[\x93\x96\x92\x95P\x90\x93PPV[_` \x82\x84\x03\x12\x15a+\xCEW_\x80\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15a+\xE5W_\x80\xFD[\x815a\x16\xE5\x81a(\x1BV[_[\x83\x81\x10\x15a,\nW\x81\x81\x01Q\x83\x82\x01R` \x01a+\xF2V[PP_\x91\x01RV[_\x81Q\x80\x84Ra,)\x81` \x86\x01` \x86\x01a+\xF0V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R_a\x16\xE5` \x83\x01\x84a,\x12V[_\x80\x83`\x1F\x84\x01\x12a,_W_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a,uW_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\n\x14W_\x80\xFD[_\x80_\x80_\x80_\x80`\xA0\x89\x8B\x03\x12\x15a,\xA6W_\x80\xFD[\x885a,\xB1\x81a(\x1BV[\x97P` \x89\x015a,\xC1\x81a(\x1BV[\x96P`@\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a,\xDCW_\x80\xFD[a,\xE8\x8C\x83\x8D\x01a,OV[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15a-\0W_\x80\xFD[a-\x0C\x8C\x83\x8D\x01a,OV[\x90\x96P\x94P`\x80\x8B\x015\x91P\x80\x82\x11\x15a-$W_\x80\xFD[Pa-1\x8B\x82\x8C\x01a(/V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[_\x82`\x1F\x83\x01\x12a-TW_\x80\xFD[a-\\a*9V[\x80`\x80\x84\x01\x85\x81\x11\x15a-mW_\x80\xFD[\x84[\x81\x81\x10\x15a-\x87W\x805\x84R` \x93\x84\x01\x93\x01a-oV[P\x90\x95\x94PPPPPV[_a\x01\xA0\x82\x84\x03\x12\x15a-\xA3W_\x80\xFD[a-\xABa*9V[\x825\x81Ra-\xBC\x84` \x85\x01a-EV[` \x82\x01Ra-\xCE\x84`\xA0\x85\x01a-EV[`@\x82\x01Ra-\xE1\x84a\x01 \x85\x01a-EV[``\x82\x01R\x93\x92PPPV[\x805`\x04\x81\x10a-\xFBW_\x80\xFD[\x91\x90PV[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a.\x15W_\x80\xFD[\x865\x95P` \x87\x015a.'\x81a(\x1BV[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015\x91Pa.J`\xA0\x88\x01a-\xEDV[\x90P\x92\x95P\x92\x95P\x92\x95V[_` \x82\x84\x03\x12\x15a.fW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a.{W_\x80\xFD[a+f\x84\x82\x85\x01a*aV[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a.\x9CW_\x80\xFD[\x865a.\xA7\x81a(\x1BV[\x95P` \x87\x015a.\xB7\x81a(\x1BV[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xBBW_\x80\xFD[_\x80`\xA0\x83\x85\x03\x12\x15a.\xF0W_\x80\xFD[a.\xFA\x84\x84a)\xC4V[\x91Pa/\x08`\x80\x84\x01a-\xEDV[\x90P\x92P\x92\x90PV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a/&W_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a/?W_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\n\x14W_\x80\xFD[_\x80\x85\x85\x11\x15a/aW_\x80\xFD[\x83\x86\x11\x15a/mW_\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x055Wa\x055a/\x8EV[\x80\x82\x01\x80\x82\x11\x15a\x055Wa\x055a/\x8EV[\x81Q_\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a/\xF5W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a/\xD9V[P\x92\x96\x95PPPPPPV[_\x82a0\x1BWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x06\x90V[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[``\x81R_a0[``\x83\x01\x87\x89a0 V[\x82\x81\x03` \x84\x01Ra0n\x81\x86\x88a0 V[\x90P\x82\x81\x03`@\x84\x01Ra0\x82\x81\x85a,\x12V[\x98\x97PPPPPPPPV[_` \x82\x84\x03\x12\x15a0\x9EW_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x16\xE5W_\x80\xFD[\x815\x81R`\x80\x81\x01` \x83\x015a0\xC3\x81a(\x1BV[`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R`@\x83\x81\x015\x90\x83\x01R``\x92\x83\x015\x92\x90\x91\x01\x91\x90\x91R\x90V[` \x80\x82R`\x17\x90\x82\x01R\x7FCaller is not the owner\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[` \x80\x82R`=\x90\x82\x01R\x7FCertified: You are not authorise`@\x82\x01R\x7Fd to transact using Nightfall\0\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`4\x90\x82\x01R\x7FCertified: You are on the Chaina`@\x82\x01Rs\x1B\x1E\\\xDA\\\xC8\x1C\xD8[\x98\xDD\x1A[\xDB\x9C\xC8\x1B\x1A\\\xDD`b\x1B``\x82\x01R`\x80\x01\x90V[_` \x82\x84\x03\x12\x15a1\xE4W_\x80\xFD[\x81Qa\x16\xE5\x81a(\x1BV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a2\x04W_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a2\x1DW_\x80\xFD[` \x01\x91Pa\x01\xA0\x81\x026\x03\x82\x13\x15a\n\x14W_\x80\xFD[_`\x01`\x01`\xFF\x1B\x01\x82\x01a2KWa2Ka/\x8EV[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a2vW_\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01R`@\x82\x01R``\x81\x01\x91\x90\x91R`\xA0`\x80\x82\x01\x81\x90R_\x90\x82\x01R`\xC0\x01\x90V[` \x80\x82R`)\x90\x82\x01R\x7FERC721 tokens should have a valu`@\x82\x01Rhe of zero`\xB8\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`'\x90\x82\x01R\x7FERC20 tokens should have a token`@\x82\x01Rf\x04\x96B\x06\xF6b\x03`\xCC\x1B``\x82\x01R`\x80\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x055Wa\x055a/\x8EV[_\x82Qa3h\x81\x84` \x87\x01a+\xF0V[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA2dipfsX\"\x12 \xC1Y\xC4\x82\xE1\x9C{\xCF;\xDB\x7FJU\x98FT6*\xEE(K6\x9D\x0C\x83y\x12\x1C\x8B\xF3+\x99dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static NIGHTFALL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Nightfall<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Nightfall<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Nightfall<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Nightfall<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Nightfall<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Nightfall)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Nightfall<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    NIGHTFALL_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                NIGHTFALL_ABI.clone(),
                NIGHTFALL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `UPGRADE_INTERFACE_VERSION` (0xad3cb1cc) function
        pub fn upgrade_interface_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([173, 60, 177, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `descrow_funds` (0xf3b85fc2) function
        pub fn descrow_funds(
            &self,
            data: WithdrawData,
            token_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([243, 184, 95, 194], (data, token_type))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `escrow_funds` (0xe6d5abe5) function
        pub fn escrow_funds(
            &self,
            fee: ::ethers::core::types::U256,
            erc_address: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            value: ::ethers::core::types::U256,
            secret_hash: ::ethers::core::types::U256,
            token_type: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [230, 213, 171, 229],
                    (fee, erc_address, token_id, value, secret_hash, token_type),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTokenInfo` (0x8c7a63ae) function
        pub fn get_token_info(
            &self,
            nf_token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::Address, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([140, 122, 99, 174], nf_token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hash_transaction` (0xd86bf977) function
        pub fn hash_transaction(
            &self,
            txn: OnChainTransaction,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([216, 107, 249, 119], (txn,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x754d1d54) function
        pub fn initialize(
            &self,
            initial_nullifier_root: ::ethers::core::types::U256,
            addr_verifier: ::ethers::core::types::Address,
            x_509_address: ::ethers::core::types::Address,
            sanctions_list_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [117, 77, 29, 84],
                    (
                        initial_nullifier_root,
                        addr_verifier,
                        x_509_address,
                        sanctions_list_address,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `layer2_block_number` (0x28c3d7e6) function
        pub fn layer_2_block_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([40, 195, 215, 230], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155BatchReceived` (0xbc197c81) function
        pub fn on_erc1155_batch_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::std::vec::Vec<::ethers::core::types::U256>,
            p3: ::std::vec::Vec<::ethers::core::types::U256>,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([188, 25, 124, 129], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155Received` (0xf23a6e61) function
        pub fn on_erc1155_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::U256,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([242, 58, 110, 97], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC3525Received` (0x009ce20b) function
        pub fn on_erc3525_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::U256,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([0, 156, 226, 11], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC721Received` (0x150b7a02) function
        pub fn on_erc721_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([21, 11, 122, 2], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `propose_block` (0x75137875) function
        pub fn propose_block(
            &self,
            blk: Block,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([117, 19, 120, 117], (blk,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAuthorities` (0x4a8a4073) function
        pub fn set_authorities(
            &self,
            sanctions_list_address: ::ethers::core::types::Address,
            x_509_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 138, 64, 115], (sanctions_list_address, x_509_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `set_proposer_manager` (0xe3178c86) function
        pub fn set_proposer_manager(
            &self,
            proposer_manager_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 23, 140, 134], proposer_manager_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `set_sanctions_list` (0x97e20d0e) function
        pub fn set_sanctions_list(
            &self,
            sanctions_list_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 226, 13, 14], sanctions_list_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `set_x509_address` (0xd5e6c6f9) function
        pub fn set_x_509_address(
            &self,
            x_509_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 230, 198, 249], x_509_address)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sha256_and_shift` (0xe7d83a88) function
        pub fn sha_256_and_shift(
            &self,
            inputs: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([231, 216, 58, 136], inputs)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgradeToAndCall` (0x4f1ef286) function
        pub fn upgrade_to_and_call(
            &self,
            new_implementation: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 30, 242, 134], (new_implementation, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verify_rollup_proof` (0x05cd0e98) function
        pub fn verify_rollup_proof(
            &self,
            blk: Block,
            public_hash: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([5, 205, 14, 152], (blk, public_hash))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw_processed` (0x186a4d08) function
        pub fn withdraw_processed(
            &self,
            data: WithdrawData,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([24, 106, 77, 8], (data,))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `BlockProposed` event
        pub fn block_proposed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BlockProposedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DepositEscrowed` event
        pub fn deposit_escrowed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DepositEscrowedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpgradedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NightfallEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Nightfall<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AddressEmptyCode` with signature `AddressEmptyCode(address)` and selector `0x9996b315`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "AddressEmptyCode", abi = "AddressEmptyCode(address)")]
    pub struct AddressEmptyCode {
        pub target: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1967InvalidImplementation` with signature `ERC1967InvalidImplementation(address)` and selector `0x4c9c8ce3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ERC1967InvalidImplementation",
        abi = "ERC1967InvalidImplementation(address)"
    )]
    pub struct ERC1967InvalidImplementation {
        pub implementation: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1967NonPayable` with signature `ERC1967NonPayable()` and selector `0xb398979f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "ERC1967NonPayable", abi = "ERC1967NonPayable()")]
    pub struct ERC1967NonPayable;
    ///Custom Error type `FailedCall` with signature `FailedCall()` and selector `0xd6bda275`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "FailedCall", abi = "FailedCall()")]
    pub struct FailedCall;
    ///Custom Error type `InvalidInitialization` with signature `InvalidInitialization()` and selector `0xf92ee8a9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidInitialization", abi = "InvalidInitialization()")]
    pub struct InvalidInitialization;
    ///Custom Error type `NotInitializing` with signature `NotInitializing()` and selector `0xd7e6bcf8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotInitializing", abi = "NotInitializing()")]
    pub struct NotInitializing;
    ///Custom Error type `UUPSUnauthorizedCallContext` with signature `UUPSUnauthorizedCallContext()` and selector `0xe07c8dba`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "UUPSUnauthorizedCallContext",
        abi = "UUPSUnauthorizedCallContext()"
    )]
    pub struct UUPSUnauthorizedCallContext;
    ///Custom Error type `UUPSUnsupportedProxiableUUID` with signature `UUPSUnsupportedProxiableUUID(bytes32)` and selector `0xaa1d49a4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "UUPSUnsupportedProxiableUUID",
        abi = "UUPSUnsupportedProxiableUUID(bytes32)"
    )]
    pub struct UUPSUnsupportedProxiableUUID {
        pub slot: [u8; 32],
    }
    ///Custom Error type `escrowFundsError` with signature `escrowFundsError()` and selector `0xafa905dd`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "escrowFundsError", abi = "escrowFundsError()")]
    pub struct escrowFundsError;
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum NightfallErrors {
        AddressEmptyCode(AddressEmptyCode),
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        ERC1967NonPayable(ERC1967NonPayable),
        FailedCall(FailedCall),
        InvalidInitialization(InvalidInitialization),
        NotInitializing(NotInitializing),
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        escrowFundsError(escrowFundsError),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for NightfallErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AddressEmptyCode as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddressEmptyCode(decoded));
            }
            if let Ok(decoded) = <ERC1967InvalidImplementation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC1967InvalidImplementation(decoded));
            }
            if let Ok(decoded) = <ERC1967NonPayable as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC1967NonPayable(decoded));
            }
            if let Ok(decoded) = <FailedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FailedCall(decoded));
            }
            if let Ok(decoded) = <InvalidInitialization as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidInitialization(decoded));
            }
            if let Ok(decoded) = <NotInitializing as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotInitializing(decoded));
            }
            if let Ok(decoded) = <UUPSUnauthorizedCallContext as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UUPSUnauthorizedCallContext(decoded));
            }
            if let Ok(decoded) = <UUPSUnsupportedProxiableUUID as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UUPSUnsupportedProxiableUUID(decoded));
            }
            if let Ok(decoded) = <escrowFundsError as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::escrowFundsError(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NightfallErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1967InvalidImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1967NonPayable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FailedCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInitialization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotInitializing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UUPSUnauthorizedCallContext(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UUPSUnsupportedProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::escrowFundsError(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for NightfallErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1967InvalidImplementation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1967NonPayable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <FailedCall as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidInitialization as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotInitializing as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UUPSUnauthorizedCallContext as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UUPSUnsupportedProxiableUUID as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <escrowFundsError as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for NightfallErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1967InvalidImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967NonPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidInitialization(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NotInitializing(element) => ::core::fmt::Display::fmt(element, f),
                Self::UUPSUnauthorizedCallContext(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UUPSUnsupportedProxiableUUID(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::escrowFundsError(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for NightfallErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for NightfallErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation> for NightfallErrors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for NightfallErrors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedCall> for NightfallErrors {
        fn from(value: FailedCall) -> Self {
            Self::FailedCall(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for NightfallErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for NightfallErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<UUPSUnauthorizedCallContext> for NightfallErrors {
        fn from(value: UUPSUnauthorizedCallContext) -> Self {
            Self::UUPSUnauthorizedCallContext(value)
        }
    }
    impl ::core::convert::From<UUPSUnsupportedProxiableUUID> for NightfallErrors {
        fn from(value: UUPSUnsupportedProxiableUUID) -> Self {
            Self::UUPSUnsupportedProxiableUUID(value)
        }
    }
    impl ::core::convert::From<escrowFundsError> for NightfallErrors {
        fn from(value: escrowFundsError) -> Self {
            Self::escrowFundsError(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "BlockProposed", abi = "BlockProposed(int256)")]
    pub struct BlockProposedFilter {
        #[ethevent(indexed)]
        pub layer_2_block_number: ::ethers::core::types::I256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "DepositEscrowed", abi = "DepositEscrowed(uint256,uint256)")]
    pub struct DepositEscrowedFilter {
        pub nf_slot_id: ::ethers::core::types::U256,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint64)")]
    pub struct InitializedFilter {
        pub version: u64,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum NightfallEvents {
        BlockProposedFilter(BlockProposedFilter),
        DepositEscrowedFilter(DepositEscrowedFilter),
        InitializedFilter(InitializedFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for NightfallEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = BlockProposedFilter::decode_log(log) {
                return Ok(NightfallEvents::BlockProposedFilter(decoded));
            }
            if let Ok(decoded) = DepositEscrowedFilter::decode_log(log) {
                return Ok(NightfallEvents::DepositEscrowedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(NightfallEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(NightfallEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for NightfallEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BlockProposedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositEscrowedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BlockProposedFilter> for NightfallEvents {
        fn from(value: BlockProposedFilter) -> Self {
            Self::BlockProposedFilter(value)
        }
    }
    impl ::core::convert::From<DepositEscrowedFilter> for NightfallEvents {
        fn from(value: DepositEscrowedFilter) -> Self {
            Self::DepositEscrowedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for NightfallEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for NightfallEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `UPGRADE_INTERFACE_VERSION` function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "UPGRADE_INTERFACE_VERSION", abi = "UPGRADE_INTERFACE_VERSION()")]
    pub struct UpgradeInterfaceVersionCall;
    ///Container type for all input parameters for the `descrow_funds` function with signature `descrow_funds((uint256,address,uint256,uint256),uint8)` and selector `0xf3b85fc2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "descrow_funds",
        abi = "descrow_funds((uint256,address,uint256,uint256),uint8)"
    )]
    pub struct DescrowFundsCall {
        pub data: WithdrawData,
        pub token_type: u8,
    }
    ///Container type for all input parameters for the `escrow_funds` function with signature `escrow_funds(uint256,address,uint256,uint256,uint256,uint8)` and selector `0xe6d5abe5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "escrow_funds",
        abi = "escrow_funds(uint256,address,uint256,uint256,uint256,uint8)"
    )]
    pub struct EscrowFundsCall {
        pub fee: ::ethers::core::types::U256,
        pub erc_address: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub value: ::ethers::core::types::U256,
        pub secret_hash: ::ethers::core::types::U256,
        pub token_type: u8,
    }
    ///Container type for all input parameters for the `getTokenInfo` function with signature `getTokenInfo(uint256)` and selector `0x8c7a63ae`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getTokenInfo", abi = "getTokenInfo(uint256)")]
    pub struct GetTokenInfoCall {
        pub nf_token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `hash_transaction` function with signature `hash_transaction((uint256,uint256[4],uint256[4],uint256[4]))` and selector `0xd86bf977`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "hash_transaction",
        abi = "hash_transaction((uint256,uint256[4],uint256[4],uint256[4]))"
    )]
    pub struct HashTransactionCall {
        pub txn: OnChainTransaction,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(uint256,address,address,address)` and selector `0x754d1d54`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(uint256,address,address,address)")]
    pub struct InitializeCall {
        pub initial_nullifier_root: ::ethers::core::types::U256,
        pub addr_verifier: ::ethers::core::types::Address,
        pub x_509_address: ::ethers::core::types::Address,
        pub sanctions_list_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `layer2_block_number` function with signature `layer2_block_number()` and selector `0x28c3d7e6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "layer2_block_number", abi = "layer2_block_number()")]
    pub struct Layer2BlockNumberCall;
    ///Container type for all input parameters for the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "onERC1155BatchReceived",
        abi = "onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct OnERC1155BatchReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "onERC1155Received",
        abi = "onERC1155Received(address,address,uint256,uint256,bytes)"
    )]
    pub struct OnERC1155ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC3525Received` function with signature `onERC3525Received(address,uint256,uint256,uint256,bytes)` and selector `0x009ce20b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "onERC3525Received",
        abi = "onERC3525Received(address,uint256,uint256,uint256,bytes)"
    )]
    pub struct OnERC3525ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "onERC721Received",
        abi = "onERC721Received(address,address,uint256,bytes)"
    )]
    pub struct OnERC721ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `propose_block` function with signature `propose_block((uint256,uint256,uint256,(uint256,uint256[4],uint256[4],uint256[4])[],bytes))` and selector `0x75137875`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "propose_block",
        abi = "propose_block((uint256,uint256,uint256,(uint256,uint256[4],uint256[4],uint256[4])[],bytes))"
    )]
    pub struct ProposeBlockCall {
        pub blk: Block,
    }
    ///Container type for all input parameters for the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proxiableUUID", abi = "proxiableUUID()")]
    pub struct ProxiableUUIDCall;
    ///Container type for all input parameters for the `setAuthorities` function with signature `setAuthorities(address,address)` and selector `0x4a8a4073`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setAuthorities", abi = "setAuthorities(address,address)")]
    pub struct SetAuthoritiesCall {
        pub sanctions_list_address: ::ethers::core::types::Address,
        pub x_509_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `set_proposer_manager` function with signature `set_proposer_manager(address)` and selector `0xe3178c86`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "set_proposer_manager", abi = "set_proposer_manager(address)")]
    pub struct SetProposerManagerCall {
        pub proposer_manager_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `set_sanctions_list` function with signature `set_sanctions_list(address)` and selector `0x97e20d0e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "set_sanctions_list", abi = "set_sanctions_list(address)")]
    pub struct SetSanctionsListCall {
        pub sanctions_list_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `set_x509_address` function with signature `set_x509_address(address)` and selector `0xd5e6c6f9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "set_x509_address", abi = "set_x509_address(address)")]
    pub struct SetX509AddressCall {
        pub x_509_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `sha256_and_shift` function with signature `sha256_and_shift(bytes)` and selector `0xe7d83a88`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sha256_and_shift", abi = "sha256_and_shift(bytes)")]
    pub struct Sha256AndShiftCall {
        pub inputs: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `upgradeToAndCall` function with signature `upgradeToAndCall(address,bytes)` and selector `0x4f1ef286`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "upgradeToAndCall", abi = "upgradeToAndCall(address,bytes)")]
    pub struct UpgradeToAndCallCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `verify_rollup_proof` function with signature `verify_rollup_proof((uint256,uint256,uint256,(uint256,uint256[4],uint256[4],uint256[4])[],bytes),uint256)` and selector `0x05cd0e98`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "verify_rollup_proof",
        abi = "verify_rollup_proof((uint256,uint256,uint256,(uint256,uint256[4],uint256[4],uint256[4])[],bytes),uint256)"
    )]
    pub struct VerifyRollupProofCall {
        pub blk: Block,
        pub public_hash: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdraw_processed` function with signature `withdraw_processed((uint256,address,uint256,uint256))` and selector `0x186a4d08`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "withdraw_processed",
        abi = "withdraw_processed((uint256,address,uint256,uint256))"
    )]
    pub struct WithdrawProcessedCall {
        pub data: WithdrawData,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum NightfallCalls {
        UpgradeInterfaceVersion(UpgradeInterfaceVersionCall),
        DescrowFunds(DescrowFundsCall),
        EscrowFunds(EscrowFundsCall),
        GetTokenInfo(GetTokenInfoCall),
        HashTransaction(HashTransactionCall),
        Initialize(InitializeCall),
        Layer2BlockNumber(Layer2BlockNumberCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        OnERC3525Received(OnERC3525ReceivedCall),
        OnERC721Received(OnERC721ReceivedCall),
        Owner(OwnerCall),
        ProposeBlock(ProposeBlockCall),
        ProxiableUUID(ProxiableUUIDCall),
        SetAuthorities(SetAuthoritiesCall),
        SetProposerManager(SetProposerManagerCall),
        SetSanctionsList(SetSanctionsListCall),
        SetX509Address(SetX509AddressCall),
        Sha256AndShift(Sha256AndShiftCall),
        SupportsInterface(SupportsInterfaceCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        VerifyRollupProof(VerifyRollupProofCall),
        WithdrawProcessed(WithdrawProcessedCall),
    }
    impl ::ethers::core::abi::AbiDecode for NightfallCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <UpgradeInterfaceVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeInterfaceVersion(decoded));
            }
            if let Ok(decoded) = <DescrowFundsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DescrowFunds(decoded));
            }
            if let Ok(decoded) = <EscrowFundsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EscrowFunds(decoded));
            }
            if let Ok(decoded) = <GetTokenInfoCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetTokenInfo(decoded));
            }
            if let Ok(decoded) = <HashTransactionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HashTransaction(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <Layer2BlockNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Layer2BlockNumber(decoded));
            }
            if let Ok(decoded) = <OnERC1155BatchReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded) = <OnERC1155ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnERC1155Received(decoded));
            }
            if let Ok(decoded) = <OnERC3525ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnERC3525Received(decoded));
            }
            if let Ok(decoded) = <OnERC721ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnERC721Received(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <ProposeBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposeBlock(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded) = <SetAuthoritiesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAuthorities(decoded));
            }
            if let Ok(decoded) = <SetProposerManagerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetProposerManager(decoded));
            }
            if let Ok(decoded) = <SetSanctionsListCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetSanctionsList(decoded));
            }
            if let Ok(decoded) = <SetX509AddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetX509Address(decoded));
            }
            if let Ok(decoded) = <Sha256AndShiftCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sha256AndShift(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) = <VerifyRollupProofCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VerifyRollupProof(decoded));
            }
            if let Ok(decoded) = <WithdrawProcessedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WithdrawProcessed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NightfallCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::UpgradeInterfaceVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DescrowFunds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EscrowFunds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HashTransaction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Layer2BlockNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155BatchReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC3525Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC721Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProposeBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAuthorities(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetProposerManager(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSanctionsList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetX509Address(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Sha256AndShift(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeToAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VerifyRollupProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawProcessed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for NightfallCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::UpgradeInterfaceVersion(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DescrowFunds(element) => ::core::fmt::Display::fmt(element, f),
                Self::EscrowFunds(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTokenInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashTransaction(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Layer2BlockNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC1155BatchReceived(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC1155Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC3525Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposeBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAuthorities(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProposerManager(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetSanctionsList(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetX509Address(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sha256AndShift(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyRollupProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawProcessed(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<UpgradeInterfaceVersionCall> for NightfallCalls {
        fn from(value: UpgradeInterfaceVersionCall) -> Self {
            Self::UpgradeInterfaceVersion(value)
        }
    }
    impl ::core::convert::From<DescrowFundsCall> for NightfallCalls {
        fn from(value: DescrowFundsCall) -> Self {
            Self::DescrowFunds(value)
        }
    }
    impl ::core::convert::From<EscrowFundsCall> for NightfallCalls {
        fn from(value: EscrowFundsCall) -> Self {
            Self::EscrowFunds(value)
        }
    }
    impl ::core::convert::From<GetTokenInfoCall> for NightfallCalls {
        fn from(value: GetTokenInfoCall) -> Self {
            Self::GetTokenInfo(value)
        }
    }
    impl ::core::convert::From<HashTransactionCall> for NightfallCalls {
        fn from(value: HashTransactionCall) -> Self {
            Self::HashTransaction(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for NightfallCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<Layer2BlockNumberCall> for NightfallCalls {
        fn from(value: Layer2BlockNumberCall) -> Self {
            Self::Layer2BlockNumber(value)
        }
    }
    impl ::core::convert::From<OnERC1155BatchReceivedCall> for NightfallCalls {
        fn from(value: OnERC1155BatchReceivedCall) -> Self {
            Self::OnERC1155BatchReceived(value)
        }
    }
    impl ::core::convert::From<OnERC1155ReceivedCall> for NightfallCalls {
        fn from(value: OnERC1155ReceivedCall) -> Self {
            Self::OnERC1155Received(value)
        }
    }
    impl ::core::convert::From<OnERC3525ReceivedCall> for NightfallCalls {
        fn from(value: OnERC3525ReceivedCall) -> Self {
            Self::OnERC3525Received(value)
        }
    }
    impl ::core::convert::From<OnERC721ReceivedCall> for NightfallCalls {
        fn from(value: OnERC721ReceivedCall) -> Self {
            Self::OnERC721Received(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for NightfallCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ProposeBlockCall> for NightfallCalls {
        fn from(value: ProposeBlockCall) -> Self {
            Self::ProposeBlock(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for NightfallCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<SetAuthoritiesCall> for NightfallCalls {
        fn from(value: SetAuthoritiesCall) -> Self {
            Self::SetAuthorities(value)
        }
    }
    impl ::core::convert::From<SetProposerManagerCall> for NightfallCalls {
        fn from(value: SetProposerManagerCall) -> Self {
            Self::SetProposerManager(value)
        }
    }
    impl ::core::convert::From<SetSanctionsListCall> for NightfallCalls {
        fn from(value: SetSanctionsListCall) -> Self {
            Self::SetSanctionsList(value)
        }
    }
    impl ::core::convert::From<SetX509AddressCall> for NightfallCalls {
        fn from(value: SetX509AddressCall) -> Self {
            Self::SetX509Address(value)
        }
    }
    impl ::core::convert::From<Sha256AndShiftCall> for NightfallCalls {
        fn from(value: Sha256AndShiftCall) -> Self {
            Self::Sha256AndShift(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for NightfallCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for NightfallCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<VerifyRollupProofCall> for NightfallCalls {
        fn from(value: VerifyRollupProofCall) -> Self {
            Self::VerifyRollupProof(value)
        }
    }
    impl ::core::convert::From<WithdrawProcessedCall> for NightfallCalls {
        fn from(value: WithdrawProcessedCall) -> Self {
            Self::WithdrawProcessed(value)
        }
    }
    ///Container type for all return fields from the `UPGRADE_INTERFACE_VERSION` function with signature `UPGRADE_INTERFACE_VERSION()` and selector `0xad3cb1cc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UpgradeInterfaceVersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getTokenInfo` function with signature `getTokenInfo(uint256)` and selector `0x8c7a63ae`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetTokenInfoReturn {
        pub erc_address: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `hash_transaction` function with signature `hash_transaction((uint256,uint256[4],uint256[4],uint256[4]))` and selector `0xd86bf977`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HashTransactionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `layer2_block_number` function with signature `layer2_block_number()` and selector `0x28c3d7e6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Layer2BlockNumberReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OnERC1155BatchReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OnERC1155ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC3525Received` function with signature `onERC3525Received(address,uint256,uint256,uint256,bytes)` and selector `0x009ce20b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OnERC3525ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC721Received` function with signature `onERC721Received(address,address,uint256,bytes)` and selector `0x150b7a02`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OnERC721ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `proxiableUUID` function with signature `proxiableUUID()` and selector `0x52d1902d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProxiableUUIDReturn(pub [u8; 32]);
    ///Container type for all return fields from the `sha256_and_shift` function with signature `sha256_and_shift(bytes)` and selector `0xe7d83a88`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Sha256AndShiftReturn {
        pub result: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `verify_rollup_proof` function with signature `verify_rollup_proof((uint256,uint256,uint256,(uint256,uint256[4],uint256[4],uint256[4])[],bytes),uint256)` and selector `0x05cd0e98`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VerifyRollupProofReturn(pub bool, pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `withdraw_processed` function with signature `withdraw_processed((uint256,address,uint256,uint256))` and selector `0x186a4d08`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct WithdrawProcessedReturn(pub bool);
    ///`Block(uint256,uint256,uint256,(uint256,uint256[4],uint256[4],uint256[4])[],bytes)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Block {
        pub commitments_root: ::ethers::core::types::U256,
        pub nullifier_root: ::ethers::core::types::U256,
        pub commitments_root_root: ::ethers::core::types::U256,
        pub transactions: ::std::vec::Vec<OnChainTransaction>,
        pub rollup_proof: ::ethers::core::types::Bytes,
    }
    ///`OnChainTransaction(uint256,uint256[4],uint256[4],uint256[4])`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OnChainTransaction {
        pub fee: ::ethers::core::types::U256,
        pub commitments: [::ethers::core::types::U256; 4],
        pub nullifiers: [::ethers::core::types::U256; 4],
        pub public_data: [::ethers::core::types::U256; 4],
    }
    ///`WithdrawData(uint256,address,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct WithdrawData {
        pub nf_token_id: ::ethers::core::types::U256,
        pub recipient_address: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub withdraw_fund_salt: ::ethers::core::types::U256,
    }
}
