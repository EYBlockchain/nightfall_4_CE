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
                inputs: ::std::vec![
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
                        name: ::std::borrow::ToOwned::to_owned("sanctionsListAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
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
                                    indexed: false,
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
            ]),
            errors: ::core::convert::From::from([
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
    const __BYTECODE: &[u8] = b"`\x80`@R_`\x03U_`\x07U_`\x08U_`\tU4\x80\x15a\0\x1FW__\xFD[P`@Qa+d8\x03\x80a+d\x839\x81\x01`@\x81\x90Ra\0>\x91a\0\xC0V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x82U`\x01\x80T\x82\x16\x93\x85\x16\x93\x90\x93\x17\x90\x92U`\x02\x80T`\x0B\x80T\x85\x16\x96\x90\x95\x16\x95\x90\x95\x17\x90\x93U`@\x80Q0\x81R` \x81\x01\x92\x90\x92R\x90 `\x04\x1C`\x0CU\x91\x82\x163\x92\x83\x16\x17\x90\x91\x17\x90Ua\x01\nV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xBDW__\xFD[PV[___``\x84\x86\x03\x12\x15a\0\xD2W__\xFD[\x83Qa\0\xDD\x81a\0\xA9V[` \x85\x01Q\x90\x93Pa\0\xEE\x81a\0\xA9V[`@\x85\x01Q\x90\x92Pa\0\xFF\x81a\0\xA9V[\x80\x91PP\x92P\x92P\x92V[a*M\x80a\x01\x17_9_\xF3\xFE`\x80`@R`\x046\x10a\0\xEEW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\x87W\x80c\xE3\x17\x8C\x86\x11a\0WW\x80c\xE3\x17\x8C\x86\x14a\x03\x1BW\x80c\xE6\xD5\xAB\xE5\x14a\x03:W\x80c\xF2:na\x14a\x03MW\x80c\xF3\xB8_\xC2\x14a\x03yW__\xFD[\x80c\x8D\xA5\xCB[\x14a\x02\x87W\x80c\x97\xE2\r\x0E\x14a\x02\xBEW\x80c\xBC\x19|\x81\x14a\x02\xDDW\x80c\xD5\xE6\xC6\xF9\x14a\x02\xFCW__\xFD[\x80c(\xC3\xD7\xE6\x11a\0\xC2W\x80c(\xC3\xD7\xE6\x14a\x01\xB4W\x80cJ\x8A@s\x14a\x01\xD7W\x80cu\x13xu\x14a\x01\xF8W\x80c\x8Czc\xAE\x14a\x02\x17W__\xFD[\x80b\x9C\xE2\x0B\x14a\0\xF2W\x80c\x01\xFF\xC9\xA7\x14a\x01;W\x80c\x15\x0Bz\x02\x14a\x01jW\x80c\x18jM\x08\x14a\x01\x95W[__\xFD[4\x80\x15a\0\xFDW__\xFD[Pa\x01\x1Da\x01\x0C6`\x04a!YV[b\x9C\xE2\x0B`\xE0\x1B\x96\x95PPPPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01FW__\xFD[Pa\x01Za\x01U6`\x04a!\xC6V[a\x03\x8CV[`@Q\x90\x15\x15\x81R` \x01a\x012V[4\x80\x15a\x01uW__\xFD[Pa\x01\x1Da\x01\x846`\x04a!\xF4V[c\n\x85\xBD\x01`\xE1\x1B\x95\x94PPPPPV[4\x80\x15a\x01\xA0W__\xFD[Pa\x01Za\x01\xAF6`\x04a\"wV[a\x03\xF7V[4\x80\x15a\x01\xBFW__\xFD[Pa\x01\xC9`\x03T\x81V[`@Q\x90\x81R` \x01a\x012V[4\x80\x15a\x01\xE2W__\xFD[Pa\x01\xF6a\x01\xF16`\x04a\"\x91V[a\x04;V[\0[4\x80\x15a\x02\x03W__\xFD[Pa\x01\xF6a\x02\x126`\x04a\"\xC8V[a\x04\x9EV[4\x80\x15a\x02\"W__\xFD[Pa\x02ha\x0216`\x04a\"\xFEV[_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x01\x90\x91\x01T\x92\x90\x91\x01\x82\x90R\x91V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x012V[4\x80\x15a\x02\x92W__\xFD[P`\x02Ta\x02\xA6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x012V[4\x80\x15a\x02\xC9W__\xFD[Pa\x01\xF6a\x02\xD86`\x04a#\x15V[a\rqV[4\x80\x15a\x02\xE8W__\xFD[Pa\x01\x1Da\x02\xF76`\x04a#pV[a\r\xBDV[4\x80\x15a\x03\x07W__\xFD[Pa\x01\xF6a\x03\x166`\x04a#\x15V[a\x0E\0V[4\x80\x15a\x03&W__\xFD[Pa\x01\xF6a\x0356`\x04a#\x15V[a\x0EKV[a\x01\xF6a\x03H6`\x04a$CV[a\x0E\xD5V[4\x80\x15a\x03XW__\xFD[Pa\x01\x1Da\x03g6`\x04a$\x99V[c\xF2:na`\xE0\x1B\x96\x95PPPPPPV[a\x01\xF6a\x03\x876`\x04a$\xF1V[a\x16yV[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x80a\x03\xBBWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16b\x9C\xE2\x0B`\xE0\x1B\x14[\x80a\x03\xD6WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\n\x85\xBD\x01`\xE1\x1B\x14[\x80a\x03\xF1WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x02q\x18\x97`\xE5\x1B\x14[\x92\x91PPV[__\x82`@Q` \x01a\x04\n\x91\x90a%#V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x90\x81R`\x05\x90\x92R\x90 T`\xFF\x16`\x01\x14\x93\x92PPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a%bV[`@Q\x80\x91\x03\x90\xFD[_\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x93\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x07\x91\x90a%\x99V[a\x05#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a%\xB8V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05iW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x8D\x91\x90a%\x99V[\x15a\x05\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a&\x15V[`\nT`@\x80Qc\x1DC\xC9}`\xE2\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cu\x0F%\xF4\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x15\x91\x90a&iV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FOnly the current proposer can pr`D\x82\x01Rlopose a block`\x98\x1B`d\x82\x01R`\x84\x01a\x04eV[``\x81\x015\x81\x015_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xA2Wa\x06\xA2a&\x84V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xCBW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a\x079Wa\x07\x14a\x06\xE8``\x86\x01\x86a&\x98V[\x83\x81\x81\x10a\x06\xF8Wa\x06\xF8a&\xDDV[\x90Pa\x01\xA0\x02\x01\x806\x03\x81\x01\x90a\x07\x0F\x91\x90a'rV[a\x1BcV[\x82\x82\x81Q\x81\x10a\x07&Wa\x07&a&\xDDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x06\xD0V[P\x80\x82[`\x01\x81\x11\x15a\x07\xFBW_[`\x01\x82\x90\x1C\x81\x10\x15a\x07\xF2Wa\x07\xCD\x83`\x01\x83\x90\x1B\x81Q\x81\x10a\x07mWa\x07ma&\xDDV[` \x02` \x01\x01Q\x84`\x01\x84\x90\x1B`\x01a\x07\x87\x91\x90a'\xE3V[\x81Q\x81\x10a\x07\x97Wa\x07\x97a&\xDDV[` \x02` \x01\x01Q`@Q` \x01a\x07\xB9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1C2V[\x83\x82\x81Q\x81\x10a\x07\xDFWa\x07\xDFa&\xDDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x07HV[P`\x01\x1Ca\x07=V[P__a\x08!\x86\x85_\x81Q\x81\x10a\x08\x14Wa\x08\x14a&\xDDV[` \x02` \x01\x01Qa\x1CUV[\x91P\x91P\x81a\x08rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FRollup proof verification failed`D\x82\x01R`d\x01a\x04eV[_[\x85\x81\x10\x15a\x0B\xEDW``\x87\x015a\x01\xA0\x82\x02\x01\x87\x01`\xC0\x015\x15\x80\x15a\t\xE5W``\x88\x015a\x01\xA0\x83\x02\x01\x88\x01a\x01@\x015\x15\x80\x15a\x08\xB4WPPa\x0B\xE5V[\x83_\x80[`\x04\x81\x10\x15a\t\xD8W``\x8C\x015a\x01\xA0\x87\x02\x01` \x82\x02\x01\x8C\x01a\x01@\x015\x91P\x81\x15a\t\xD0W_\x82\x81R`\x04` R`@\x90 Ta\x08\xF8\x90\x84a'\xE3V[_\x83\x81R`\x04` R`@\x90 `\x01\x90\x81\x01T\x91\x94P`\xFF\x90\x91\x16\x14\x80\x15a\t6WP_\x82\x81R`\x04` R`@\x90 `\x01\x01Ta\x01\0\x90\x04`\xFF\x16\x15[a\t\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDeposit either has not been escr`D\x82\x01R\x7Fowed or has already been redeeme`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x04eV[_\x82\x81R`\x04` R`@\x90 `\x01\x01\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01\x01a\x08\xB8V[P\x81\x95PPPPPa\x0B\xE5V[`@``\x89\x015a\x01\xA0\x84\x02\x01\x89\x01\x90\x81\x015\x15`\xC0\x91\x90\x91\x015\x15\x15\x16\x80\x15a\x0B\xE2W_a\n\x17``\x8B\x01\x8Ba&\x98V[\x85\x81\x81\x10a\n'Wa\n'a&\xDDV[\x90Pa\x01\xA0\x02\x01`\xA0\x01_`\x04\x81\x10a\nBWa\nBa&\xDDV[` \x02\x015\x90P_`@Q\x80`\x80\x01`@R\x80\x8C\x80``\x01\x90a\ne\x91\x90a&\x98V[\x88\x81\x81\x10a\nuWa\nua&\xDDV[\x90Pa\x01\xA0\x02\x01a\x01 \x01_`\x04\x81\x10a\n\x91Wa\n\x91a&\xDDV[` \x02\x015\x81R` \x01\x8C\x80``\x01\x90a\n\xAB\x91\x90a&\x98V[\x88\x81\x81\x10a\n\xBBWa\n\xBBa&\xDDV[\x90Pa\x01\xA0\x02\x01a\x01 \x01`\x01`\x04\x81\x10a\n\xD8Wa\n\xD8a&\xDDV[` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x80``\x01\x90a\n\xFB\x91\x90a&\x98V[\x88\x81\x81\x10a\x0B\x0BWa\x0B\x0Ba&\xDDV[\x90Pa\x01\xA0\x02\x01a\x01 \x01`\x02`\x04\x81\x10a\x0B(Wa\x0B(a&\xDDV[` \x90\x81\x02\x91\x90\x91\x015\x82R\x90\x81\x01\x84\x90R`@\x80Q\x83Q\x81R\x83\x83\x01Q\x81\x84\x01R\x81\x84\x01Q\x81\x83\x01R``\x80\x85\x01Q\x90\x82\x01R`\x80\x90 _\x81\x81R`\x05\x90\x93R\x91 T\x91\x92P\x90`\xFF\x16\x15a\x0B\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FFunds have already withdrawn\0\0\0\0`D\x82\x01R`d\x01a\x04eV[_\x90\x81R`\x05` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UPa\x0B\xE5\x92PPPV[PP[`\x01\x01a\x08tV[P`\nT`@\x80Qc\x1DC\xC9}`\xE2\x1B\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cu\x0F%\xF4\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0C5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CY\x91\x90a&iV[\x90P_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x0C\xA4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0C\xA9V[``\x91P[PP\x90P\x80a\r\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FFailed to transfer the fee to th`D\x82\x01Ri2\x90897\xB87\xB9\xB2\xB9`\xB1\x1B`d\x82\x01R`\x84\x01a\x04eV[\x875`\x07U` \x88\x015`\x08U`@\x88\x015`\tU`\x03\x80T\x7FF\xBF\x14\x88'\xA7N\xB5\xC7\xCA\x85\xB1\x05\x131\xF7\xE0|kQ#\0X\x0CG\xBF\x1D\xD6F\xC1\xD8\x95\x91_a\rR\x83a'\xF6V[\x90\x91UP`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a%bV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x15[\x9C\xDD\\\x1C\x1B\xDC\x9D\x08\x18\x9EH\x13\x9AY\xDA\x1D\x19\x98[\x1B`R\x1B`D\x82\x01R_\x90`d\x01a\x04eV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a%bV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FOnly the owner can call this fun`D\x82\x01Rd1\xBA4\xB7\xB7`\xD9\x1B`d\x82\x01R`\x84\x01a\x04eV[`\n\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F>\x91\x90a%\x99V[a\x0FZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a%\xB8V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xC4\x91\x90a%\x99V[\x15a\x0F\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a&\x15V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R\x90\x81\x01\x85\x90R_\x90a\x10\x08\x90``\x01a\x07\xB9V[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82R` \x80\x83\x01\x8A\x81R_\x86\x81R`\x06\x90\x92R\x93\x81 \x92Q\x83T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x16\x91\x90\x91\x17\x82U\x91Q`\x01\x90\x91\x01U\x90\x91P`\x03\x83`\x03\x81\x11\x15a\x10gWa\x10ga(\x14V[\x14a\x10rW\x81a\x11\x12V[`@Qc\x13\x1F\x9F?`\xE1\x1B\x81R`\x04\x81\x81\x01\x88\x90R\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c&?>~\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xB9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xDD\x91\x90a((V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x84\x01R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1C\x90\x1C[`@\x80Q`\x80\x80\x82\x01\x83R\x85\x82R` \x80\x83\x01\x85\x81R\x83\x85\x01\x8B\x81R``\x80\x86\x01\x8C\x81R\x87Q\x94\x85\x01\x8B\x90R\x92Q\x96\x84\x01\x96\x90\x96RQ\x94\x82\x01\x94\x90\x94R\x92Q\x90\x83\x01R\x91\x92P_\x90a\x11f\x90`\xA0\x01a\x07\xB9V[_\x81\x81R`\x04` R`@\x90 `\x01\x01T\x90\x91P`\xFF\x16\x15a\x11\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FFunds have already been escrowed`D\x82\x01Rp\x08\x19\x9B\xDC\x88\x1D\x1A\x1A\\\xC8\x11\x19\\\x1B\xDC\xDA]`z\x1B`d\x82\x01R`\x84\x01a\x04eV[`\x03\x85`\x03\x81\x11\x15a\x11\xF8Wa\x11\xF8a(\x14V[\x03a\x12dW`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c#\xB8r\xDD\x90`d\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x12IW__\xFD[PZ\xF1\x15\x80\x15a\x12[W=__>=_\xFD[PPPPa\x148V[`\x01\x85`\x03\x81\x11\x15a\x12xWa\x12xa(\x14V[\x03a\x12\xAFW`@Qcy!!\x95`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xF2BC*\x90a\x122\x903\x900\x90\x8D\x90\x8D\x90`\x04\x01a(?V[`\x02\x85`\x03\x81\x11\x15a\x12\xC3Wa\x12\xC3a(\x14V[\x03a\x13,W\x86\x15a\x12\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a(vV[`@Qc\\F\xA7\xEF`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x89\x90R`\x80`d\x82\x01R_`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xB8\x8DO\xDE\x90`\xA4\x01a\x122V[_\x85`\x03\x81\x11\x15a\x13?Wa\x13?a(\x14V[\x03a\x14\x1FW\x87\x15a\x13bW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a(\xBFV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xD6\x91\x90a%\x99V[a\x14\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11T\x90\xCC\x8C\x08\x1D\x1C\x98[\x9C\xD9\x99\\\x88\x19\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\x04eV[a\x148V[`@Qc\xAF\xA9\x05\xDD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q``\x81\x01\x82R\x8B\x81R`\x01` \x80\x83\x01\x82\x81R_\x84\x86\x01\x81\x81R\x87\x82R`\x04\x90\x93R\x85\x90 \x93Q\x84UQ\x92\x90\x91\x01\x80T\x91Q`\xFF\x90\x81\x16a\x01\0\x02a\xFF\xFF\x19\x90\x93\x16\x93\x16\x92\x90\x92\x17\x17\x90UQ\x7F5G)\xB3XZ\xC9Q\x8AvY\xE8\xB323\x89\xF1H\xA7S\xE8\xC3\xC5\xD8\xC0\x1B\xFE\t\xBC\0\x93@\x90a\x14\xC0\x90\x85\x90\x8A\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1a\x14\xD3\x8A`\x02a)\x06V[4\x11\x15a\x16mW_a\x14\xE6\x8B`\x02a)\x06V[a\x14\xF0\x904a)\x1DV[\x90P_`@Q\x80`\x80\x01`@R\x80`\x0CT\x81R` \x01`\x0CT\x81R` \x01\x83\x81R` \x01\x89\x81RP\x90P_a\x15U\x82`@Q` \x01a\x07\xB9\x91\x90\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[_\x81\x81R`\x04` R`@\x90 `\x01\x01T\x90\x91P`\xFF\x16\x15a\x15\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FFunds have already been escrowed`D\x82\x01Rt\x08\x19\x9B\xDC\x88\x1D\x1A\x1A\\\xC8\x19\x99YH\x11\x19\\\x1B\xDC\xDA]`Z\x1B`d\x82\x01R`\x84\x01a\x04eV[`@\x80Q``\x81\x01\x82R\x8E\x81R`\x01` \x80\x83\x01\x82\x81R_\x84\x86\x01\x81\x81R\x87\x82R`\x04\x90\x93R\x85\x90 \x93Q\x84UQ\x92\x90\x91\x01\x80T\x91Q`\xFF\x90\x81\x16a\x01\0\x02a\xFF\xFF\x19\x90\x93\x16\x93\x16\x92\x90\x92\x17\x17\x90U`\x0CT\x90Q\x7F5G)\xB3XZ\xC9Q\x8AvY\xE8\xB323\x89\xF1H\xA7S\xE8\xC3\xC5\xD8\xC0\x1B\xFE\t\xBC\0\x93@\x91a\x16a\x91\x86\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPP[PPPPPPPPPPV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xBEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE2\x91\x90a%\x99V[a\x16\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a%\xB8V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17h\x91\x90a%\x99V[\x15a\x17\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a&\x15V[_\x82`@Q` \x01a\x17\x97\x91\x90a%#V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R`\x05\x90\x93R\x91 T\x90\x91P`\xFF\x16`\x01\x14a\x18GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FEither no funds are available to`D\x82\x01R\x7F withdraw, or they are already w`d\x82\x01Rg4\xBA4290\xBB\xB7`\xC1\x1B`\x84\x82\x01R`\xA4\x01a\x04eV[\x825_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x01\x90\x91\x01T\x91\x83\x01\x91\x90\x91Ra\x193W_a\x18\x92`@\x86\x01` \x87\x01a#\x15V[`\x01`\x01`\xA0\x1B\x03\x16\x85`@\x015`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x18\xDDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x18\xE2V[``\x91P[PP\x90P\x80a\x19,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuCould not withdraw fee`P\x1B`D\x82\x01R`d\x01a\x04eV[PPPPPV[_`\x01\x84`\x03\x81\x11\x15a\x19HWa\x19Ha(\x14V[\x03a\x19\xCDW\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xF2BC*0a\x19n`@\x89\x01` \x8A\x01a#\x15V[\x85` \x01Q\x89`@\x015`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\x97\x94\x93\x92\x91\x90a(?V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x19\xAEW__\xFD[PZ\xF1\x15\x80\x15a\x19\xC0W=__>=_\xFD[PPPP`\x01\x90Pa\x1BBV[`\x02\x84`\x03\x81\x11\x15a\x19\xE1Wa\x19\xE1a(\x14V[\x03a\x1AuW`@\x85\x015\x15a\x1A\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a(vV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xB8\x8DO\xDE0a\x1A)`@\x89\x01` \x8A\x01a#\x15V[` \x86\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`\x80`d\x82\x01R_`\x84\x82\x01R`\xA4\x01a\x19\x97V[_\x84`\x03\x81\x11\x15a\x1A\x88Wa\x1A\x88a(\x14V[\x03a\x1BBW` \x82\x01Q\x15a\x1A\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a(\xBFV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa\x1A\xCF`@\x88\x01` \x89\x01a#\x15V[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R\x88\x015`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1B\x1BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B?\x91\x90a%\x99V[\x90P[\x80\x15a\x19,WPP_\x90\x81R`\x05` R`@\x90 \x80T`\xFF\x19\x16\x90UPPV[``\x81\x81\x01Q\x80\x82\x01Q` \x80\x85\x01Q\x80Q\x81\x83\x01Q`@\x80\x84\x01Q\x93\x88\x01Q\x81\x8A\x01Q\x80Q\x81\x88\x01Q\x82\x85\x01Q\x92\x8C\x01Q\x8BQ\x8A\x8D\x01Q\x9C\x87\x01Q\x87Q\x9B\x8C\x01\x99\x90\x99R\x8A\x87\x01\x97\x90\x97R\x9B\x89\x01\x97\x90\x97R`\x80\x88\x01\x92\x90\x92R`\xA0\x87\x01\x91\x90\x91R`\xC0\x86\x01\x94\x90\x94R`\xE0\x85\x01\x93\x90\x93Ra\x01\0\x84\x01\x96\x90\x96Ra\x01 \x83\x01\x95\x90\x95Ra\x01@\x82\x01\x93\x90\x93Ra\x01`\x81\x01\x93\x90\x93R`\x01`\x01`\xFF\x1B\x03\x16a\x01\x80\x80\x84\x01\x82\x90R\x82Q\x80\x85\x03\x90\x91\x01\x81Ra\x01\xA0\x90\x93\x01\x90\x91R_\x91a\x1C*\x81a\x1C2V[\x94\x93PPPPV[_`@Q` \x81\x84Q` \x86\x01`\x02Z\xFAa\x1CKW__\xFD[Q`\x04\x1C\x92\x91PPV[_\x80\x80a\x1Ce`\x80\x86\x01\x86a)0V[a\x1Cs\x91` \x91_\x91a)rV[\x81\x01\x90a\x1C\x80\x91\x90a\"\xFEV[\x90P\x80_a\x1C\x91`\x80\x88\x01\x88a)0V[a\x1C\xA0\x91`@\x91` \x91a)rV[\x81\x01\x90a\x1C\xAD\x91\x90a\"\xFEV[\x90P_a\x1C\xBD`\x80\x89\x01\x89a)0V[a\x1C\xCC\x91``\x91`@\x91a)rV[\x81\x01\x90a\x1C\xD9\x91\x90a\"\xFEV[\x90P_a\x1C\xE9`\x80\x8A\x01\x8Aa)0V[a\x1C\xF8\x91`\x80\x91``\x91a)rV[\x81\x01\x90a\x1D\x05\x91\x90a\"\xFEV[\x90P_a\x1D\x15`\x80\x8B\x01\x8Ba)0V[a\x1D$\x91`\xA0\x91`\x80\x91a)rV[\x81\x01\x90a\x1D1\x91\x90a\"\xFEV[\x90P_a\x1DA`\x80\x8C\x01\x8Ca)0V[a\x1DP\x91`\xC0\x91`\xA0\x91a)rV[\x81\x01\x90a\x1D]\x91\x90a\"\xFEV[\x90P_a\x1Dm`\x80\x8D\x01\x8Da)0V[a\x1D|\x91`\xE0\x91`\xC0\x91a)rV[\x81\x01\x90a\x1D\x89\x91\x90a\"\xFEV[\x90P_a\x1D\x99`\x80\x8E\x01\x8Ea)0V[a\x1D\xA9\x91a\x01\0\x91`\xE0\x91a)rV[\x81\x01\x90a\x1D\xB6\x91\x90a\"\xFEV[\x90P_a\x1D\xC6`\x80\x8F\x01\x8Fa)0V[a\x1D\xD7\x91a\x01 \x91a\x01\0\x91a)rV[\x81\x01\x90a\x1D\xE4\x91\x90a\"\xFEV[`@\x80Q`\x10\x80\x82Ra\x02 \x82\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01a\x02\0\x806\x837\x01\x90PP\x90P\x8A\x81_\x81Q\x81\x10a\x1E\x1FWa\x1E\x1Fa&\xDDV[` \x02` \x01\x01\x81\x81RPP\x8D_\x1B\x81`\x01\x81Q\x81\x10a\x1EAWa\x1EAa&\xDDV[` \x02` \x01\x01\x81\x81RPP`\x07T_\x1B\x81`\x02\x81Q\x81\x10a\x1EeWa\x1Eea&\xDDV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80Q\x8F5\x90\x82\x90`\x03\x90\x81\x10a\x1E\x88Wa\x1E\x88a&\xDDV[` \x02` \x01\x01\x81\x81RPP`\x08T_\x1B\x81`\x04\x81Q\x81\x10a\x1E\xACWa\x1E\xACa&\xDDV[` \x02` \x01\x01\x81\x81RPP\x8E` \x015_\x1B\x81`\x05\x81Q\x81\x10a\x1E\xD2Wa\x1E\xD2a&\xDDV[` \x02` \x01\x01\x81\x81RPP`\tT_\x1B\x81`\x06\x81Q\x81\x10a\x1E\xF6Wa\x1E\xF6a&\xDDV[` \x02` \x01\x01\x81\x81RPP\x8E`@\x015_\x1B\x81`\x07\x81Q\x81\x10a\x1F\x1CWa\x1F\x1Ca&\xDDV[` \x02` \x01\x01\x81\x81RPP\x88\x81`\x08\x81Q\x81\x10a\x1F<Wa\x1F<a&\xDDV[` \x02` \x01\x01\x81\x81RPP\x87\x81`\t\x81Q\x81\x10a\x1F\\Wa\x1F\\a&\xDDV[` \x02` \x01\x01\x81\x81RPP\x86\x81`\n\x81Q\x81\x10a\x1F|Wa\x1F|a&\xDDV[` \x02` \x01\x01\x81\x81RPP\x85\x81`\x0B\x81Q\x81\x10a\x1F\x9CWa\x1F\x9Ca&\xDDV[` \x02` \x01\x01\x81\x81RPP\x84\x81`\x0C\x81Q\x81\x10a\x1F\xBCWa\x1F\xBCa&\xDDV[` \x02` \x01\x01\x81\x81RPP\x83\x81`\r\x81Q\x81\x10a\x1F\xDCWa\x1F\xDCa&\xDDV[` \x02` \x01\x01\x81\x81RPP\x82\x81`\x0E\x81Q\x81\x10a\x1F\xFCWa\x1F\xFCa&\xDDV[` \x02` \x01\x01\x81\x81RPP\x81\x81`\x0F\x81Q\x81\x10a \x1CWa \x1Ca&\xDDV[` \x02` \x01\x01\x81\x81RPP_\x8F\x80`\x80\x01\x90a 9\x91\x90a)0V[a H\x91a\x01 \x90\x82\x90a)rV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`\x0BT`@Qc:\x9449`\xE2\x1B\x81R\x93\x94P`\x01`\x01`\xA0\x1B\x03\x16\x92c\xEAP\xD0\xE4\x92Pa \xAE\x91P\x84\x90\x86\x90`\x04\x01a)\x99V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xED\x91\x90a%\x99V[\x9DP\x99\x9BPPPPPPPPPPPP[\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\x19W__\xFD[PV[__\x83`\x1F\x84\x01\x12a!,W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a!BW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a \xFEW__\xFD[______`\xA0\x87\x89\x03\x12\x15a!nW__\xFD[\x865a!y\x81a!\x05V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xA8W__\xFD[a!\xB4\x89\x82\x8A\x01a!\x1CV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_` \x82\x84\x03\x12\x15a!\xD6W__\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a!\xEDW__\xFD[\x93\x92PPPV[_____`\x80\x86\x88\x03\x12\x15a\"\x08W__\xFD[\x855a\"\x13\x81a!\x05V[\x94P` \x86\x015a\"#\x81a!\x05V[\x93P`@\x86\x015\x92P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"DW__\xFD[a\"P\x88\x82\x89\x01a!\x1CV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_`\x80\x82\x84\x03\x12\x15a\"qW__\xFD[P\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a\"\x87W__\xFD[a!\xED\x83\x83a\"aV[__`@\x83\x85\x03\x12\x15a\"\xA2W__\xFD[\x825a\"\xAD\x81a!\x05V[\x91P` \x83\x015a\"\xBD\x81a!\x05V[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\"\xD8W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xEDW__\xFD[\x82\x01`\xA0\x81\x85\x03\x12\x15a!\xEDW__\xFD[_` \x82\x84\x03\x12\x15a#\x0EW__\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15a#%W__\xFD[\x815a!\xED\x81a!\x05V[__\x83`\x1F\x84\x01\x12a#@W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a#VW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a \xFEW__\xFD[________`\xA0\x89\x8B\x03\x12\x15a#\x87W__\xFD[\x885a#\x92\x81a!\x05V[\x97P` \x89\x015a#\xA2\x81a!\x05V[\x96P`@\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xBCW__\xFD[a#\xC8\x8B\x82\x8C\x01a#0V[\x90\x97P\x95PP``\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xE6W__\xFD[a#\xF2\x8B\x82\x8C\x01a#0V[\x90\x95P\x93PP`\x80\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$\x10W__\xFD[a$\x1C\x8B\x82\x8C\x01a!\x1CV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[\x805`\x04\x81\x10a$>W__\xFD[\x91\x90PV[______`\xC0\x87\x89\x03\x12\x15a$XW__\xFD[\x865\x95P` \x87\x015a$j\x81a!\x05V[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015\x91Pa$\x8D`\xA0\x88\x01a$0V[\x90P\x92\x95P\x92\x95P\x92\x95V[______`\xA0\x87\x89\x03\x12\x15a$\xAEW__\xFD[\x865a$\xB9\x81a!\x05V[\x95P` \x87\x015a$\xC9\x81a!\x05V[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xA8W__\xFD[__`\xA0\x83\x85\x03\x12\x15a%\x02W__\xFD[a%\x0C\x84\x84a\"aV[\x91Pa%\x1A`\x80\x84\x01a$0V[\x90P\x92P\x92\x90PV[\x815\x81R`\x80\x81\x01` \x83\x015a%9\x81a!\x05V[`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R`@\x83\x81\x015\x90\x83\x01R``\x92\x83\x015\x92\x90\x91\x01\x91\x90\x91R\x90V[` \x80\x82R`\x17\x90\x82\x01R\x7FCaller is not the owner\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a%\xA9W__\xFD[\x81Q\x80\x15\x15\x81\x14a!\xEDW__\xFD[` \x80\x82R`=\x90\x82\x01R\x7FCertified: You are not authorise`@\x82\x01R\x7Fd to transact using Nightfall\0\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`4\x90\x82\x01R\x7FCertified: You are on the Chaina`@\x82\x01Rs\x1B\x1E\\\xDA\\\xC8\x1C\xD8[\x98\xDD\x1A[\xDB\x9C\xC8\x1B\x1A\\\xDD`b\x1B``\x82\x01R`\x80\x01\x90V[_` \x82\x84\x03\x12\x15a&yW__\xFD[\x81Qa!\xED\x81a!\x05V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a&\xADW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a&\xC6W__\xFD[` \x01\x91Pa\x01\xA0\x81\x026\x03\x82\x13\x15a \xFEW__\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a'\x1FWcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@R\x90V[_\x82`\x1F\x83\x01\x12a'4W__\xFD[a'<a&\xF1V[\x80`\x80\x84\x01\x85\x81\x11\x15a'MW__\xFD[\x84[\x81\x81\x10\x15a'gW\x805\x84R` \x93\x84\x01\x93\x01a'OV[P\x90\x95\x94PPPPPV[_a\x01\xA0\x82\x84\x03\x12\x80\x15a'\x84W__\xFD[Pa'\x8Da&\xF1V[\x825\x81Ra'\x9E\x84` \x85\x01a'%V[` \x82\x01Ra'\xB0\x84`\xA0\x85\x01a'%V[`@\x82\x01Ra'\xC3\x84a\x01 \x85\x01a'%V[``\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x03\xF1Wa\x03\xF1a'\xCFV[_`\x01`\x01`\xFF\x1B\x01\x82\x01a(\rWa(\ra'\xCFV[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a(8W__\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01R`@\x82\x01R``\x81\x01\x91\x90\x91R`\xA0`\x80\x82\x01\x81\x90R_\x90\x82\x01R`\xC0\x01\x90V[` \x80\x82R`)\x90\x82\x01R\x7FERC721 tokens should have a valu`@\x82\x01Rhe of zero`\xB8\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`'\x90\x82\x01R\x7FERC20 tokens should have a token`@\x82\x01Rf\x04\x96B\x06\xF6b\x03`\xCC\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xF1Wa\x03\xF1a'\xCFV[\x81\x81\x03\x81\x81\x11\x15a\x03\xF1Wa\x03\xF1a'\xCFV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a)EW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a)^W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a \xFEW__\xFD[__\x85\x85\x11\x15a)\x80W__\xFD[\x83\x86\x11\x15a)\x8CW__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`@\x81R_\x83Q\x80`@\x84\x01R\x80` \x86\x01``\x85\x01^_``\x82\x85\x01\x01R`\x1F\x19`\x1F\x82\x01\x16\x83\x01\x90P``\x81\x01``\x84\x83\x03\x01` \x85\x01R\x80\x85Q\x80\x83R`\x80\x84\x01\x91P` \x87\x01\x93P_\x92P[\x80\x83\x10\x15a*\x0CW\x83Q\x82R` \x82\x01\x91P` \x84\x01\x93P`\x01\x83\x01\x92Pa)\xE9V[P\x96\x95PPPPPPV\xFE\xA2dipfsX\"\x12 \xF9\xF0lE\x81=s\xF0\xAC\xF1\x199\t\x97\x90!2wpj\xB1\x9D\xDB\x8F\x99\xFB\xE1\xAE\xA5\r\xD3\xC1dsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static NIGHTFALL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xEEW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\x87W\x80c\xE3\x17\x8C\x86\x11a\0WW\x80c\xE3\x17\x8C\x86\x14a\x03\x1BW\x80c\xE6\xD5\xAB\xE5\x14a\x03:W\x80c\xF2:na\x14a\x03MW\x80c\xF3\xB8_\xC2\x14a\x03yW__\xFD[\x80c\x8D\xA5\xCB[\x14a\x02\x87W\x80c\x97\xE2\r\x0E\x14a\x02\xBEW\x80c\xBC\x19|\x81\x14a\x02\xDDW\x80c\xD5\xE6\xC6\xF9\x14a\x02\xFCW__\xFD[\x80c(\xC3\xD7\xE6\x11a\0\xC2W\x80c(\xC3\xD7\xE6\x14a\x01\xB4W\x80cJ\x8A@s\x14a\x01\xD7W\x80cu\x13xu\x14a\x01\xF8W\x80c\x8Czc\xAE\x14a\x02\x17W__\xFD[\x80b\x9C\xE2\x0B\x14a\0\xF2W\x80c\x01\xFF\xC9\xA7\x14a\x01;W\x80c\x15\x0Bz\x02\x14a\x01jW\x80c\x18jM\x08\x14a\x01\x95W[__\xFD[4\x80\x15a\0\xFDW__\xFD[Pa\x01\x1Da\x01\x0C6`\x04a!YV[b\x9C\xE2\x0B`\xE0\x1B\x96\x95PPPPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01FW__\xFD[Pa\x01Za\x01U6`\x04a!\xC6V[a\x03\x8CV[`@Q\x90\x15\x15\x81R` \x01a\x012V[4\x80\x15a\x01uW__\xFD[Pa\x01\x1Da\x01\x846`\x04a!\xF4V[c\n\x85\xBD\x01`\xE1\x1B\x95\x94PPPPPV[4\x80\x15a\x01\xA0W__\xFD[Pa\x01Za\x01\xAF6`\x04a\"wV[a\x03\xF7V[4\x80\x15a\x01\xBFW__\xFD[Pa\x01\xC9`\x03T\x81V[`@Q\x90\x81R` \x01a\x012V[4\x80\x15a\x01\xE2W__\xFD[Pa\x01\xF6a\x01\xF16`\x04a\"\x91V[a\x04;V[\0[4\x80\x15a\x02\x03W__\xFD[Pa\x01\xF6a\x02\x126`\x04a\"\xC8V[a\x04\x9EV[4\x80\x15a\x02\"W__\xFD[Pa\x02ha\x0216`\x04a\"\xFEV[_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x01\x90\x91\x01T\x92\x90\x91\x01\x82\x90R\x91V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x012V[4\x80\x15a\x02\x92W__\xFD[P`\x02Ta\x02\xA6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x012V[4\x80\x15a\x02\xC9W__\xFD[Pa\x01\xF6a\x02\xD86`\x04a#\x15V[a\rqV[4\x80\x15a\x02\xE8W__\xFD[Pa\x01\x1Da\x02\xF76`\x04a#pV[a\r\xBDV[4\x80\x15a\x03\x07W__\xFD[Pa\x01\xF6a\x03\x166`\x04a#\x15V[a\x0E\0V[4\x80\x15a\x03&W__\xFD[Pa\x01\xF6a\x0356`\x04a#\x15V[a\x0EKV[a\x01\xF6a\x03H6`\x04a$CV[a\x0E\xD5V[4\x80\x15a\x03XW__\xFD[Pa\x01\x1Da\x03g6`\x04a$\x99V[c\xF2:na`\xE0\x1B\x96\x95PPPPPPV[a\x01\xF6a\x03\x876`\x04a$\xF1V[a\x16yV[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x80a\x03\xBBWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16b\x9C\xE2\x0B`\xE0\x1B\x14[\x80a\x03\xD6WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\n\x85\xBD\x01`\xE1\x1B\x14[\x80a\x03\xF1WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x02q\x18\x97`\xE5\x1B\x14[\x92\x91PPV[__\x82`@Q` \x01a\x04\n\x91\x90a%#V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x90\x81R`\x05\x90\x92R\x90 T`\xFF\x16`\x01\x14\x93\x92PPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04nW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a%bV[`@Q\x80\x91\x03\x90\xFD[_\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x93\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xE3W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x07\x91\x90a%\x99V[a\x05#W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a%\xB8V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05iW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x8D\x91\x90a%\x99V[\x15a\x05\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a&\x15V[`\nT`@\x80Qc\x1DC\xC9}`\xE2\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cu\x0F%\xF4\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\xF1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x15\x91\x90a&iV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FOnly the current proposer can pr`D\x82\x01Rlopose a block`\x98\x1B`d\x82\x01R`\x84\x01a\x04eV[``\x81\x015\x81\x015_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\xA2Wa\x06\xA2a&\x84V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06\xCBW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a\x079Wa\x07\x14a\x06\xE8``\x86\x01\x86a&\x98V[\x83\x81\x81\x10a\x06\xF8Wa\x06\xF8a&\xDDV[\x90Pa\x01\xA0\x02\x01\x806\x03\x81\x01\x90a\x07\x0F\x91\x90a'rV[a\x1BcV[\x82\x82\x81Q\x81\x10a\x07&Wa\x07&a&\xDDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x06\xD0V[P\x80\x82[`\x01\x81\x11\x15a\x07\xFBW_[`\x01\x82\x90\x1C\x81\x10\x15a\x07\xF2Wa\x07\xCD\x83`\x01\x83\x90\x1B\x81Q\x81\x10a\x07mWa\x07ma&\xDDV[` \x02` \x01\x01Q\x84`\x01\x84\x90\x1B`\x01a\x07\x87\x91\x90a'\xE3V[\x81Q\x81\x10a\x07\x97Wa\x07\x97a&\xDDV[` \x02` \x01\x01Q`@Q` \x01a\x07\xB9\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1C2V[\x83\x82\x81Q\x81\x10a\x07\xDFWa\x07\xDFa&\xDDV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x07HV[P`\x01\x1Ca\x07=V[P__a\x08!\x86\x85_\x81Q\x81\x10a\x08\x14Wa\x08\x14a&\xDDV[` \x02` \x01\x01Qa\x1CUV[\x91P\x91P\x81a\x08rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FRollup proof verification failed`D\x82\x01R`d\x01a\x04eV[_[\x85\x81\x10\x15a\x0B\xEDW``\x87\x015a\x01\xA0\x82\x02\x01\x87\x01`\xC0\x015\x15\x80\x15a\t\xE5W``\x88\x015a\x01\xA0\x83\x02\x01\x88\x01a\x01@\x015\x15\x80\x15a\x08\xB4WPPa\x0B\xE5V[\x83_\x80[`\x04\x81\x10\x15a\t\xD8W``\x8C\x015a\x01\xA0\x87\x02\x01` \x82\x02\x01\x8C\x01a\x01@\x015\x91P\x81\x15a\t\xD0W_\x82\x81R`\x04` R`@\x90 Ta\x08\xF8\x90\x84a'\xE3V[_\x83\x81R`\x04` R`@\x90 `\x01\x90\x81\x01T\x91\x94P`\xFF\x90\x91\x16\x14\x80\x15a\t6WP_\x82\x81R`\x04` R`@\x90 `\x01\x01Ta\x01\0\x90\x04`\xFF\x16\x15[a\t\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDeposit either has not been escr`D\x82\x01R\x7Fowed or has already been redeeme`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x04eV[_\x82\x81R`\x04` R`@\x90 `\x01\x01\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01\x01a\x08\xB8V[P\x81\x95PPPPPa\x0B\xE5V[`@``\x89\x015a\x01\xA0\x84\x02\x01\x89\x01\x90\x81\x015\x15`\xC0\x91\x90\x91\x015\x15\x15\x16\x80\x15a\x0B\xE2W_a\n\x17``\x8B\x01\x8Ba&\x98V[\x85\x81\x81\x10a\n'Wa\n'a&\xDDV[\x90Pa\x01\xA0\x02\x01`\xA0\x01_`\x04\x81\x10a\nBWa\nBa&\xDDV[` \x02\x015\x90P_`@Q\x80`\x80\x01`@R\x80\x8C\x80``\x01\x90a\ne\x91\x90a&\x98V[\x88\x81\x81\x10a\nuWa\nua&\xDDV[\x90Pa\x01\xA0\x02\x01a\x01 \x01_`\x04\x81\x10a\n\x91Wa\n\x91a&\xDDV[` \x02\x015\x81R` \x01\x8C\x80``\x01\x90a\n\xAB\x91\x90a&\x98V[\x88\x81\x81\x10a\n\xBBWa\n\xBBa&\xDDV[\x90Pa\x01\xA0\x02\x01a\x01 \x01`\x01`\x04\x81\x10a\n\xD8Wa\n\xD8a&\xDDV[` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x80``\x01\x90a\n\xFB\x91\x90a&\x98V[\x88\x81\x81\x10a\x0B\x0BWa\x0B\x0Ba&\xDDV[\x90Pa\x01\xA0\x02\x01a\x01 \x01`\x02`\x04\x81\x10a\x0B(Wa\x0B(a&\xDDV[` \x90\x81\x02\x91\x90\x91\x015\x82R\x90\x81\x01\x84\x90R`@\x80Q\x83Q\x81R\x83\x83\x01Q\x81\x84\x01R\x81\x84\x01Q\x81\x83\x01R``\x80\x85\x01Q\x90\x82\x01R`\x80\x90 _\x81\x81R`\x05\x90\x93R\x91 T\x91\x92P\x90`\xFF\x16\x15a\x0B\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FFunds have already withdrawn\0\0\0\0`D\x82\x01R`d\x01a\x04eV[_\x90\x81R`\x05` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UPa\x0B\xE5\x92PPPV[PP[`\x01\x01a\x08tV[P`\nT`@\x80Qc\x1DC\xC9}`\xE2\x1B\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cu\x0F%\xF4\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0C5W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0CY\x91\x90a&iV[\x90P_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x0C\xA4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0C\xA9V[``\x91P[PP\x90P\x80a\r\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FFailed to transfer the fee to th`D\x82\x01Ri2\x90897\xB87\xB9\xB2\xB9`\xB1\x1B`d\x82\x01R`\x84\x01a\x04eV[\x875`\x07U` \x88\x015`\x08U`@\x88\x015`\tU`\x03\x80T\x7FF\xBF\x14\x88'\xA7N\xB5\xC7\xCA\x85\xB1\x05\x131\xF7\xE0|kQ#\0X\x0CG\xBF\x1D\xD6F\xC1\xD8\x95\x91_a\rR\x83a'\xF6V[\x90\x91UP`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a%bV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x15[\x9C\xDD\\\x1C\x1B\xDC\x9D\x08\x18\x9EH\x13\x9AY\xDA\x1D\x19\x98[\x1B`R\x1B`D\x82\x01R_\x90`d\x01a\x04eV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E*W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a%bV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0E\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FOnly the owner can call this fun`D\x82\x01Rd1\xBA4\xB7\xB7`\xD9\x1B`d\x82\x01R`\x84\x01a\x04eV[`\n\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x1AW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F>\x91\x90a%\x99V[a\x0FZW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a%\xB8V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xA0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xC4\x91\x90a%\x99V[\x15a\x0F\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a&\x15V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R\x90\x81\x01\x85\x90R_\x90a\x10\x08\x90``\x01a\x07\xB9V[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82R` \x80\x83\x01\x8A\x81R_\x86\x81R`\x06\x90\x92R\x93\x81 \x92Q\x83T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x16\x91\x90\x91\x17\x82U\x91Q`\x01\x90\x91\x01U\x90\x91P`\x03\x83`\x03\x81\x11\x15a\x10gWa\x10ga(\x14V[\x14a\x10rW\x81a\x11\x12V[`@Qc\x13\x1F\x9F?`\xE1\x1B\x81R`\x04\x81\x81\x01\x88\x90R\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c&?>~\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xB9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10\xDD\x91\x90a((V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x84\x01R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1C\x90\x1C[`@\x80Q`\x80\x80\x82\x01\x83R\x85\x82R` \x80\x83\x01\x85\x81R\x83\x85\x01\x8B\x81R``\x80\x86\x01\x8C\x81R\x87Q\x94\x85\x01\x8B\x90R\x92Q\x96\x84\x01\x96\x90\x96RQ\x94\x82\x01\x94\x90\x94R\x92Q\x90\x83\x01R\x91\x92P_\x90a\x11f\x90`\xA0\x01a\x07\xB9V[_\x81\x81R`\x04` R`@\x90 `\x01\x01T\x90\x91P`\xFF\x16\x15a\x11\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FFunds have already been escrowed`D\x82\x01Rp\x08\x19\x9B\xDC\x88\x1D\x1A\x1A\\\xC8\x11\x19\\\x1B\xDC\xDA]`z\x1B`d\x82\x01R`\x84\x01a\x04eV[`\x03\x85`\x03\x81\x11\x15a\x11\xF8Wa\x11\xF8a(\x14V[\x03a\x12dW`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c#\xB8r\xDD\x90`d\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x12IW__\xFD[PZ\xF1\x15\x80\x15a\x12[W=__>=_\xFD[PPPPa\x148V[`\x01\x85`\x03\x81\x11\x15a\x12xWa\x12xa(\x14V[\x03a\x12\xAFW`@Qcy!!\x95`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xF2BC*\x90a\x122\x903\x900\x90\x8D\x90\x8D\x90`\x04\x01a(?V[`\x02\x85`\x03\x81\x11\x15a\x12\xC3Wa\x12\xC3a(\x14V[\x03a\x13,W\x86\x15a\x12\xE6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a(vV[`@Qc\\F\xA7\xEF`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x89\x90R`\x80`d\x82\x01R_`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xB8\x8DO\xDE\x90`\xA4\x01a\x122V[_\x85`\x03\x81\x11\x15a\x13?Wa\x13?a(\x14V[\x03a\x14\x1FW\x87\x15a\x13bW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a(\xBFV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\xB2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xD6\x91\x90a%\x99V[a\x14\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11T\x90\xCC\x8C\x08\x1D\x1C\x98[\x9C\xD9\x99\\\x88\x19\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\x04eV[a\x148V[`@Qc\xAF\xA9\x05\xDD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q``\x81\x01\x82R\x8B\x81R`\x01` \x80\x83\x01\x82\x81R_\x84\x86\x01\x81\x81R\x87\x82R`\x04\x90\x93R\x85\x90 \x93Q\x84UQ\x92\x90\x91\x01\x80T\x91Q`\xFF\x90\x81\x16a\x01\0\x02a\xFF\xFF\x19\x90\x93\x16\x93\x16\x92\x90\x92\x17\x17\x90UQ\x7F5G)\xB3XZ\xC9Q\x8AvY\xE8\xB323\x89\xF1H\xA7S\xE8\xC3\xC5\xD8\xC0\x1B\xFE\t\xBC\0\x93@\x90a\x14\xC0\x90\x85\x90\x8A\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1a\x14\xD3\x8A`\x02a)\x06V[4\x11\x15a\x16mW_a\x14\xE6\x8B`\x02a)\x06V[a\x14\xF0\x904a)\x1DV[\x90P_`@Q\x80`\x80\x01`@R\x80`\x0CT\x81R` \x01`\x0CT\x81R` \x01\x83\x81R` \x01\x89\x81RP\x90P_a\x15U\x82`@Q` \x01a\x07\xB9\x91\x90\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[_\x81\x81R`\x04` R`@\x90 `\x01\x01T\x90\x91P`\xFF\x16\x15a\x15\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FFunds have already been escrowed`D\x82\x01Rt\x08\x19\x9B\xDC\x88\x1D\x1A\x1A\\\xC8\x19\x99YH\x11\x19\\\x1B\xDC\xDA]`Z\x1B`d\x82\x01R`\x84\x01a\x04eV[`@\x80Q``\x81\x01\x82R\x8E\x81R`\x01` \x80\x83\x01\x82\x81R_\x84\x86\x01\x81\x81R\x87\x82R`\x04\x90\x93R\x85\x90 \x93Q\x84UQ\x92\x90\x91\x01\x80T\x91Q`\xFF\x90\x81\x16a\x01\0\x02a\xFF\xFF\x19\x90\x93\x16\x93\x16\x92\x90\x92\x17\x17\x90U`\x0CT\x90Q\x7F5G)\xB3XZ\xC9Q\x8AvY\xE8\xB323\x89\xF1H\xA7S\xE8\xC3\xC5\xD8\xC0\x1B\xFE\t\xBC\0\x93@\x91a\x16a\x91\x86\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPP[PPPPPPPPPPV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xBEW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xE2\x91\x90a%\x99V[a\x16\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a%\xB8V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17DW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17h\x91\x90a%\x99V[\x15a\x17\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a&\x15V[_\x82`@Q` \x01a\x17\x97\x91\x90a%#V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R`\x05\x90\x93R\x91 T\x90\x91P`\xFF\x16`\x01\x14a\x18GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FEither no funds are available to`D\x82\x01R\x7F withdraw, or they are already w`d\x82\x01Rg4\xBA4290\xBB\xB7`\xC1\x1B`\x84\x82\x01R`\xA4\x01a\x04eV[\x825_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x01\x90\x91\x01T\x91\x83\x01\x91\x90\x91Ra\x193W_a\x18\x92`@\x86\x01` \x87\x01a#\x15V[`\x01`\x01`\xA0\x1B\x03\x16\x85`@\x015`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x18\xDDW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x18\xE2V[``\x91P[PP\x90P\x80a\x19,W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuCould not withdraw fee`P\x1B`D\x82\x01R`d\x01a\x04eV[PPPPPV[_`\x01\x84`\x03\x81\x11\x15a\x19HWa\x19Ha(\x14V[\x03a\x19\xCDW\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xF2BC*0a\x19n`@\x89\x01` \x8A\x01a#\x15V[\x85` \x01Q\x89`@\x015`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19\x97\x94\x93\x92\x91\x90a(?V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x19\xAEW__\xFD[PZ\xF1\x15\x80\x15a\x19\xC0W=__>=_\xFD[PPPP`\x01\x90Pa\x1BBV[`\x02\x84`\x03\x81\x11\x15a\x19\xE1Wa\x19\xE1a(\x14V[\x03a\x1AuW`@\x85\x015\x15a\x1A\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a(vV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xB8\x8DO\xDE0a\x1A)`@\x89\x01` \x8A\x01a#\x15V[` \x86\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`\x80`d\x82\x01R_`\x84\x82\x01R`\xA4\x01a\x19\x97V[_\x84`\x03\x81\x11\x15a\x1A\x88Wa\x1A\x88a(\x14V[\x03a\x1BBW` \x82\x01Q\x15a\x1A\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04e\x90a(\xBFV[\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa\x1A\xCF`@\x88\x01` \x89\x01a#\x15V[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R\x88\x015`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1B\x1BW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B?\x91\x90a%\x99V[\x90P[\x80\x15a\x19,WPP_\x90\x81R`\x05` R`@\x90 \x80T`\xFF\x19\x16\x90UPPV[``\x81\x81\x01Q\x80\x82\x01Q` \x80\x85\x01Q\x80Q\x81\x83\x01Q`@\x80\x84\x01Q\x93\x88\x01Q\x81\x8A\x01Q\x80Q\x81\x88\x01Q\x82\x85\x01Q\x92\x8C\x01Q\x8BQ\x8A\x8D\x01Q\x9C\x87\x01Q\x87Q\x9B\x8C\x01\x99\x90\x99R\x8A\x87\x01\x97\x90\x97R\x9B\x89\x01\x97\x90\x97R`\x80\x88\x01\x92\x90\x92R`\xA0\x87\x01\x91\x90\x91R`\xC0\x86\x01\x94\x90\x94R`\xE0\x85\x01\x93\x90\x93Ra\x01\0\x84\x01\x96\x90\x96Ra\x01 \x83\x01\x95\x90\x95Ra\x01@\x82\x01\x93\x90\x93Ra\x01`\x81\x01\x93\x90\x93R`\x01`\x01`\xFF\x1B\x03\x16a\x01\x80\x80\x84\x01\x82\x90R\x82Q\x80\x85\x03\x90\x91\x01\x81Ra\x01\xA0\x90\x93\x01\x90\x91R_\x91a\x1C*\x81a\x1C2V[\x94\x93PPPPV[_`@Q` \x81\x84Q` \x86\x01`\x02Z\xFAa\x1CKW__\xFD[Q`\x04\x1C\x92\x91PPV[_\x80\x80a\x1Ce`\x80\x86\x01\x86a)0V[a\x1Cs\x91` \x91_\x91a)rV[\x81\x01\x90a\x1C\x80\x91\x90a\"\xFEV[\x90P\x80_a\x1C\x91`\x80\x88\x01\x88a)0V[a\x1C\xA0\x91`@\x91` \x91a)rV[\x81\x01\x90a\x1C\xAD\x91\x90a\"\xFEV[\x90P_a\x1C\xBD`\x80\x89\x01\x89a)0V[a\x1C\xCC\x91``\x91`@\x91a)rV[\x81\x01\x90a\x1C\xD9\x91\x90a\"\xFEV[\x90P_a\x1C\xE9`\x80\x8A\x01\x8Aa)0V[a\x1C\xF8\x91`\x80\x91``\x91a)rV[\x81\x01\x90a\x1D\x05\x91\x90a\"\xFEV[\x90P_a\x1D\x15`\x80\x8B\x01\x8Ba)0V[a\x1D$\x91`\xA0\x91`\x80\x91a)rV[\x81\x01\x90a\x1D1\x91\x90a\"\xFEV[\x90P_a\x1DA`\x80\x8C\x01\x8Ca)0V[a\x1DP\x91`\xC0\x91`\xA0\x91a)rV[\x81\x01\x90a\x1D]\x91\x90a\"\xFEV[\x90P_a\x1Dm`\x80\x8D\x01\x8Da)0V[a\x1D|\x91`\xE0\x91`\xC0\x91a)rV[\x81\x01\x90a\x1D\x89\x91\x90a\"\xFEV[\x90P_a\x1D\x99`\x80\x8E\x01\x8Ea)0V[a\x1D\xA9\x91a\x01\0\x91`\xE0\x91a)rV[\x81\x01\x90a\x1D\xB6\x91\x90a\"\xFEV[\x90P_a\x1D\xC6`\x80\x8F\x01\x8Fa)0V[a\x1D\xD7\x91a\x01 \x91a\x01\0\x91a)rV[\x81\x01\x90a\x1D\xE4\x91\x90a\"\xFEV[`@\x80Q`\x10\x80\x82Ra\x02 \x82\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01a\x02\0\x806\x837\x01\x90PP\x90P\x8A\x81_\x81Q\x81\x10a\x1E\x1FWa\x1E\x1Fa&\xDDV[` \x02` \x01\x01\x81\x81RPP\x8D_\x1B\x81`\x01\x81Q\x81\x10a\x1EAWa\x1EAa&\xDDV[` \x02` \x01\x01\x81\x81RPP`\x07T_\x1B\x81`\x02\x81Q\x81\x10a\x1EeWa\x1Eea&\xDDV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80Q\x8F5\x90\x82\x90`\x03\x90\x81\x10a\x1E\x88Wa\x1E\x88a&\xDDV[` \x02` \x01\x01\x81\x81RPP`\x08T_\x1B\x81`\x04\x81Q\x81\x10a\x1E\xACWa\x1E\xACa&\xDDV[` \x02` \x01\x01\x81\x81RPP\x8E` \x015_\x1B\x81`\x05\x81Q\x81\x10a\x1E\xD2Wa\x1E\xD2a&\xDDV[` \x02` \x01\x01\x81\x81RPP`\tT_\x1B\x81`\x06\x81Q\x81\x10a\x1E\xF6Wa\x1E\xF6a&\xDDV[` \x02` \x01\x01\x81\x81RPP\x8E`@\x015_\x1B\x81`\x07\x81Q\x81\x10a\x1F\x1CWa\x1F\x1Ca&\xDDV[` \x02` \x01\x01\x81\x81RPP\x88\x81`\x08\x81Q\x81\x10a\x1F<Wa\x1F<a&\xDDV[` \x02` \x01\x01\x81\x81RPP\x87\x81`\t\x81Q\x81\x10a\x1F\\Wa\x1F\\a&\xDDV[` \x02` \x01\x01\x81\x81RPP\x86\x81`\n\x81Q\x81\x10a\x1F|Wa\x1F|a&\xDDV[` \x02` \x01\x01\x81\x81RPP\x85\x81`\x0B\x81Q\x81\x10a\x1F\x9CWa\x1F\x9Ca&\xDDV[` \x02` \x01\x01\x81\x81RPP\x84\x81`\x0C\x81Q\x81\x10a\x1F\xBCWa\x1F\xBCa&\xDDV[` \x02` \x01\x01\x81\x81RPP\x83\x81`\r\x81Q\x81\x10a\x1F\xDCWa\x1F\xDCa&\xDDV[` \x02` \x01\x01\x81\x81RPP\x82\x81`\x0E\x81Q\x81\x10a\x1F\xFCWa\x1F\xFCa&\xDDV[` \x02` \x01\x01\x81\x81RPP\x81\x81`\x0F\x81Q\x81\x10a \x1CWa \x1Ca&\xDDV[` \x02` \x01\x01\x81\x81RPP_\x8F\x80`\x80\x01\x90a 9\x91\x90a)0V[a H\x91a\x01 \x90\x82\x90a)rV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`\x0BT`@Qc:\x9449`\xE2\x1B\x81R\x93\x94P`\x01`\x01`\xA0\x1B\x03\x16\x92c\xEAP\xD0\xE4\x92Pa \xAE\x91P\x84\x90\x86\x90`\x04\x01a)\x99V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xC9W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \xED\x91\x90a%\x99V[\x9DP\x99\x9BPPPPPPPPPPPP[\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\x19W__\xFD[PV[__\x83`\x1F\x84\x01\x12a!,W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a!BW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a \xFEW__\xFD[______`\xA0\x87\x89\x03\x12\x15a!nW__\xFD[\x865a!y\x81a!\x05V[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xA8W__\xFD[a!\xB4\x89\x82\x8A\x01a!\x1CV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_` \x82\x84\x03\x12\x15a!\xD6W__\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a!\xEDW__\xFD[\x93\x92PPPV[_____`\x80\x86\x88\x03\x12\x15a\"\x08W__\xFD[\x855a\"\x13\x81a!\x05V[\x94P` \x86\x015a\"#\x81a!\x05V[\x93P`@\x86\x015\x92P``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\"DW__\xFD[a\"P\x88\x82\x89\x01a!\x1CV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_`\x80\x82\x84\x03\x12\x15a\"qW__\xFD[P\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a\"\x87W__\xFD[a!\xED\x83\x83a\"aV[__`@\x83\x85\x03\x12\x15a\"\xA2W__\xFD[\x825a\"\xAD\x81a!\x05V[\x91P` \x83\x015a\"\xBD\x81a!\x05V[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\"\xD8W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xEDW__\xFD[\x82\x01`\xA0\x81\x85\x03\x12\x15a!\xEDW__\xFD[_` \x82\x84\x03\x12\x15a#\x0EW__\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15a#%W__\xFD[\x815a!\xED\x81a!\x05V[__\x83`\x1F\x84\x01\x12a#@W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a#VW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a \xFEW__\xFD[________`\xA0\x89\x8B\x03\x12\x15a#\x87W__\xFD[\x885a#\x92\x81a!\x05V[\x97P` \x89\x015a#\xA2\x81a!\x05V[\x96P`@\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xBCW__\xFD[a#\xC8\x8B\x82\x8C\x01a#0V[\x90\x97P\x95PP``\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xE6W__\xFD[a#\xF2\x8B\x82\x8C\x01a#0V[\x90\x95P\x93PP`\x80\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a$\x10W__\xFD[a$\x1C\x8B\x82\x8C\x01a!\x1CV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[\x805`\x04\x81\x10a$>W__\xFD[\x91\x90PV[______`\xC0\x87\x89\x03\x12\x15a$XW__\xFD[\x865\x95P` \x87\x015a$j\x81a!\x05V[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015\x91Pa$\x8D`\xA0\x88\x01a$0V[\x90P\x92\x95P\x92\x95P\x92\x95V[______`\xA0\x87\x89\x03\x12\x15a$\xAEW__\xFD[\x865a$\xB9\x81a!\x05V[\x95P` \x87\x015a$\xC9\x81a!\x05V[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xA8W__\xFD[__`\xA0\x83\x85\x03\x12\x15a%\x02W__\xFD[a%\x0C\x84\x84a\"aV[\x91Pa%\x1A`\x80\x84\x01a$0V[\x90P\x92P\x92\x90PV[\x815\x81R`\x80\x81\x01` \x83\x015a%9\x81a!\x05V[`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R`@\x83\x81\x015\x90\x83\x01R``\x92\x83\x015\x92\x90\x91\x01\x91\x90\x91R\x90V[` \x80\x82R`\x17\x90\x82\x01R\x7FCaller is not the owner\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a%\xA9W__\xFD[\x81Q\x80\x15\x15\x81\x14a!\xEDW__\xFD[` \x80\x82R`=\x90\x82\x01R\x7FCertified: You are not authorise`@\x82\x01R\x7Fd to transact using Nightfall\0\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`4\x90\x82\x01R\x7FCertified: You are on the Chaina`@\x82\x01Rs\x1B\x1E\\\xDA\\\xC8\x1C\xD8[\x98\xDD\x1A[\xDB\x9C\xC8\x1B\x1A\\\xDD`b\x1B``\x82\x01R`\x80\x01\x90V[_` \x82\x84\x03\x12\x15a&yW__\xFD[\x81Qa!\xED\x81a!\x05V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a&\xADW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a&\xC6W__\xFD[` \x01\x91Pa\x01\xA0\x81\x026\x03\x82\x13\x15a \xFEW__\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a'\x1FWcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@R\x90V[_\x82`\x1F\x83\x01\x12a'4W__\xFD[a'<a&\xF1V[\x80`\x80\x84\x01\x85\x81\x11\x15a'MW__\xFD[\x84[\x81\x81\x10\x15a'gW\x805\x84R` \x93\x84\x01\x93\x01a'OV[P\x90\x95\x94PPPPPV[_a\x01\xA0\x82\x84\x03\x12\x80\x15a'\x84W__\xFD[Pa'\x8Da&\xF1V[\x825\x81Ra'\x9E\x84` \x85\x01a'%V[` \x82\x01Ra'\xB0\x84`\xA0\x85\x01a'%V[`@\x82\x01Ra'\xC3\x84a\x01 \x85\x01a'%V[``\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x03\xF1Wa\x03\xF1a'\xCFV[_`\x01`\x01`\xFF\x1B\x01\x82\x01a(\rWa(\ra'\xCFV[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a(8W__\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01R`@\x82\x01R``\x81\x01\x91\x90\x91R`\xA0`\x80\x82\x01\x81\x90R_\x90\x82\x01R`\xC0\x01\x90V[` \x80\x82R`)\x90\x82\x01R\x7FERC721 tokens should have a valu`@\x82\x01Rhe of zero`\xB8\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`'\x90\x82\x01R\x7FERC20 tokens should have a token`@\x82\x01Rf\x04\x96B\x06\xF6b\x03`\xCC\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\xF1Wa\x03\xF1a'\xCFV[\x81\x81\x03\x81\x81\x11\x15a\x03\xF1Wa\x03\xF1a'\xCFV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a)EW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a)^W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a \xFEW__\xFD[__\x85\x85\x11\x15a)\x80W__\xFD[\x83\x86\x11\x15a)\x8CW__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[`@\x81R_\x83Q\x80`@\x84\x01R\x80` \x86\x01``\x85\x01^_``\x82\x85\x01\x01R`\x1F\x19`\x1F\x82\x01\x16\x83\x01\x90P``\x81\x01``\x84\x83\x03\x01` \x85\x01R\x80\x85Q\x80\x83R`\x80\x84\x01\x91P` \x87\x01\x93P_\x92P[\x80\x83\x10\x15a*\x0CW\x83Q\x82R` \x82\x01\x91P` \x84\x01\x93P`\x01\x83\x01\x92Pa)\xE9V[P\x96\x95PPPPPPV\xFE\xA2dipfsX\"\x12 \xF9\xF0lE\x81=s\xF0\xAC\xF1\x199\t\x97\x90!2wpj\xB1\x9D\xDB\x8F\x99\xFB\xE1\xAE\xA5\r\xD3\xC1dsolcC\0\x08\x1C\x003";
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
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
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
        DescrowFunds(DescrowFundsCall),
        EscrowFunds(EscrowFundsCall),
        GetTokenInfo(GetTokenInfoCall),
        Layer2BlockNumber(Layer2BlockNumberCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        OnERC3525Received(OnERC3525ReceivedCall),
        OnERC721Received(OnERC721ReceivedCall),
        Owner(OwnerCall),
        ProposeBlock(ProposeBlockCall),
        SetAuthorities(SetAuthoritiesCall),
        SetProposerManager(SetProposerManagerCall),
        SetSanctionsList(SetSanctionsListCall),
        SetX509Address(SetX509AddressCall),
        SupportsInterface(SupportsInterfaceCall),
        WithdrawProcessed(WithdrawProcessedCall),
    }
    impl ::ethers::core::abi::AbiDecode for NightfallCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
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
                Self::DescrowFunds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EscrowFunds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTokenInfo(element) => {
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
                Self::SupportsInterface(element) => {
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
                Self::DescrowFunds(element) => ::core::fmt::Display::fmt(element, f),
                Self::EscrowFunds(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTokenInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::Layer2BlockNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC1155BatchReceived(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC1155Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC3525Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC721Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposeBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAuthorities(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProposerManager(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetSanctionsList(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetX509Address(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawProcessed(element) => ::core::fmt::Display::fmt(element, f),
            }
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
    impl ::core::convert::From<SupportsInterfaceCall> for NightfallCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<WithdrawProcessedCall> for NightfallCalls {
        fn from(value: WithdrawProcessedCall) -> Self {
            Self::WithdrawProcessed(value)
        }
    }
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
