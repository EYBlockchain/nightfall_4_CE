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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R_`\x03U_`\x07U_`\x08U_`\tU4\x80\x15b\0\0 W_\x80\xFD[P`@Qb\0+\x1C8\x03\x80b\0+\x1C\x839\x81\x01`@\x81\x90Rb\0\0C\x91b\0\0\xC7V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x82U`\x01\x80T\x82\x16\x93\x85\x16\x93\x90\x93\x17\x90\x92U`\x02\x80T`\x0B\x80T\x85\x16\x96\x90\x95\x16\x95\x90\x95\x17\x90\x93U`@\x80Q0\x81R` \x81\x01\x92\x90\x92R\x90 `\x04\x1C`\x0CU\x91\x82\x163\x92\x83\x16\x17\x90\x91\x17\x90Ub\0\x01\x18V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xC4W_\x80\xFD[PV[_\x80_``\x84\x86\x03\x12\x15b\0\0\xDAW_\x80\xFD[\x83Qb\0\0\xE7\x81b\0\0\xAFV[` \x85\x01Q\x90\x93Pb\0\0\xFA\x81b\0\0\xAFV[`@\x85\x01Q\x90\x92Pb\0\x01\r\x81b\0\0\xAFV[\x80\x91PP\x92P\x92P\x92V[a)\xF6\x80b\0\x01&_9_\xF3\xFE`\x80`@R`\x046\x10a\0\xE3W_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\x87W\x80c\xE3\x17\x8C\x86\x11a\0WW\x80c\xE3\x17\x8C\x86\x14a\x02\xB2W\x80c\xE6\xD5\xAB\xE5\x14a\x02\xD1W\x80c\xF2:na\x14a\x02\xE4W\x80c\xF3\xB8_\xC2\x14a\x03\x10W_\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x02\x1EW\x80c\x97\xE2\r\x0E\x14a\x02UW\x80c\xBC\x19|\x81\x14a\x02tW\x80c\xD5\xE6\xC6\xF9\x14a\x02\x93W_\x80\xFD[\x80c\x18jM\x08\x11a\0\xC2W\x80c\x18jM\x08\x14a\x01\x8AW\x80c(\xC3\xD7\xE6\x14a\x01\xBBW\x80cJ\x8A@s\x14a\x01\xDEW\x80cu\x13xu\x14a\x01\xFFW_\x80\xFD[\x80b\x9C\xE2\x0B\x14a\0\xE7W\x80c\x01\xFF\xC9\xA7\x14a\x010W\x80c\x15\x0Bz\x02\x14a\x01_W[_\x80\xFD[4\x80\x15a\0\xF2W_\x80\xFD[Pa\x01\x12a\x01\x016`\x04a \xEFV[b\x9C\xE2\x0B`\xE0\x1B\x96\x95PPPPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01;W_\x80\xFD[Pa\x01Oa\x01J6`\x04a!]V[a\x03#V[`@Q\x90\x15\x15\x81R` \x01a\x01'V[4\x80\x15a\x01jW_\x80\xFD[Pa\x01\x12a\x01y6`\x04a!\x8BV[c\n\x85\xBD\x01`\xE1\x1B\x95\x94PPPPPV[4\x80\x15a\x01\x95W_\x80\xFD[Pa\x01\xA9a\x01\xA46`\x04a\"\x0FV[a\x03\x8EV[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01'V[4\x80\x15a\x01\xC6W_\x80\xFD[Pa\x01\xD0`\x03T\x81V[`@Q\x90\x81R` \x01a\x01'V[4\x80\x15a\x01\xE9W_\x80\xFD[Pa\x01\xFDa\x01\xF86`\x04a\")V[a\x03\xCFV[\0[4\x80\x15a\x02\nW_\x80\xFD[Pa\x01\xFDa\x02\x196`\x04a\"`V[a\x042V[4\x80\x15a\x02)W_\x80\xFD[P`\x02Ta\x02=\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01'V[4\x80\x15a\x02`W_\x80\xFD[Pa\x01\xFDa\x02o6`\x04a\"\x97V[a\r\x06V[4\x80\x15a\x02\x7FW_\x80\xFD[Pa\x01\x12a\x02\x8E6`\x04a\"\xF3V[a\rRV[4\x80\x15a\x02\x9EW_\x80\xFD[Pa\x01\xFDa\x02\xAD6`\x04a\"\x97V[a\r\x95V[4\x80\x15a\x02\xBDW_\x80\xFD[Pa\x01\xFDa\x02\xCC6`\x04a\"\x97V[a\r\xE0V[a\x01\xFDa\x02\xDF6`\x04a#\xBDV[a\x0EjV[4\x80\x15a\x02\xEFW_\x80\xFD[Pa\x01\x12a\x02\xFE6`\x04a$\x13V[c\xF2:na`\xE0\x1B\x96\x95PPPPPPV[a\x01\xFDa\x03\x1E6`\x04a$lV[a\x16\x0EV[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x80a\x03RWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16b\x9C\xE2\x0B`\xE0\x1B\x14[\x80a\x03mWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\n\x85\xBD\x01`\xE1\x1B\x14[\x80a\x03\x88WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x02q\x18\x97`\xE5\x1B\x14[\x92\x91PPV[_\x80\x82`@Q` \x01a\x03\xA1\x91\x90a$\x9EV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x90\x81R`\x05\x90\x92R\x90 T`\xFF\x16\x93\x92PPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a$\xDDV[`@Q\x80\x91\x03\x90\xFD[_\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x93\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04wW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x9B\x91\x90a%\x14V[a\x04\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a%3V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xFDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05!\x91\x90a%\x14V[\x15a\x05>W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a%\x90V[`\nT`@\x80Qc\x1DC\xC9}`\xE2\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cu\x0F%\xF4\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\x85W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xA9\x91\x90a%\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FOnly the current proposer can pr`D\x82\x01Rlopose a block`\x98\x1B`d\x82\x01R`\x84\x01a\x03\xF9V[``\x81\x015\x81\x015_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x067Wa\x067a%\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06`W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a\x06\xCEWa\x06\xA9a\x06}``\x86\x01\x86a&\x13V[\x83\x81\x81\x10a\x06\x8DWa\x06\x8Da&YV[\x90Pa\x01\xA0\x02\x01\x806\x03\x81\x01\x90a\x06\xA4\x91\x90a&\xEFV[a\x1A\xF8V[\x82\x82\x81Q\x81\x10a\x06\xBBWa\x06\xBBa&YV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x06eV[P\x80\x82[`\x01\x81\x11\x15a\x07\x90W_[`\x01\x82\x90\x1C\x81\x10\x15a\x07\x87Wa\x07b\x83`\x01\x83\x90\x1B\x81Q\x81\x10a\x07\x02Wa\x07\x02a&YV[` \x02` \x01\x01Q\x84`\x01\x84\x90\x1B`\x01a\x07\x1C\x91\x90a'^V[\x81Q\x81\x10a\x07,Wa\x07,a&YV[` \x02` \x01\x01Q`@Q` \x01a\x07N\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1B\xC7V[\x83\x82\x81Q\x81\x10a\x07tWa\x07ta&YV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x06\xDDV[P`\x01\x1Ca\x06\xD2V[P_\x80a\x07\xB6\x86\x85_\x81Q\x81\x10a\x07\xA9Wa\x07\xA9a&YV[` \x02` \x01\x01Qa\x1B\xEAV[\x91P\x91P\x81a\x08\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FRollup proof verification failed`D\x82\x01R`d\x01a\x03\xF9V[_[\x85\x81\x10\x15a\x0B\x82W``\x87\x015a\x01\xA0\x82\x02\x01\x87\x01`\xC0\x015\x15\x80\x15a\tzW``\x88\x015a\x01\xA0\x83\x02\x01\x88\x01a\x01@\x015\x15\x80\x15a\x08IWPPa\x0BzV[\x83_\x80[`\x04\x81\x10\x15a\tmW``\x8C\x015a\x01\xA0\x87\x02\x01` \x82\x02\x01\x8C\x01a\x01@\x015\x91P\x81\x15a\teW_\x82\x81R`\x04` R`@\x90 Ta\x08\x8D\x90\x84a'^V[_\x83\x81R`\x04` R`@\x90 `\x01\x90\x81\x01T\x91\x94P`\xFF\x90\x91\x16\x14\x80\x15a\x08\xCBWP_\x82\x81R`\x04` R`@\x90 `\x01\x01Ta\x01\0\x90\x04`\xFF\x16\x15[a\tGW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDeposit either has not been escr`D\x82\x01R\x7Fowed or has already been redeeme`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x03\xF9V[_\x82\x81R`\x04` R`@\x90 `\x01\x01\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01\x01a\x08MV[P\x81\x95PPPPPa\x0BzV[`@``\x89\x015a\x01\xA0\x84\x02\x01\x89\x01\x90\x81\x015\x15`\xC0\x91\x90\x91\x015\x15\x15\x16\x80\x15a\x0BwW_a\t\xAC``\x8B\x01\x8Ba&\x13V[\x85\x81\x81\x10a\t\xBCWa\t\xBCa&YV[\x90Pa\x01\xA0\x02\x01`\xA0\x01_`\x04\x81\x10a\t\xD7Wa\t\xD7a&YV[` \x02\x015\x90P_`@Q\x80`\x80\x01`@R\x80\x8C\x80``\x01\x90a\t\xFA\x91\x90a&\x13V[\x88\x81\x81\x10a\n\nWa\n\na&YV[\x90Pa\x01\xA0\x02\x01a\x01 \x01_`\x04\x81\x10a\n&Wa\n&a&YV[` \x02\x015\x81R` \x01\x8C\x80``\x01\x90a\n@\x91\x90a&\x13V[\x88\x81\x81\x10a\nPWa\nPa&YV[\x90Pa\x01\xA0\x02\x01a\x01 \x01`\x01`\x04\x81\x10a\nmWa\nma&YV[` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x80``\x01\x90a\n\x90\x91\x90a&\x13V[\x88\x81\x81\x10a\n\xA0Wa\n\xA0a&YV[\x90Pa\x01\xA0\x02\x01a\x01 \x01`\x02`\x04\x81\x10a\n\xBDWa\n\xBDa&YV[` \x90\x81\x02\x91\x90\x91\x015\x82R\x90\x81\x01\x84\x90R`@\x80Q\x83Q\x81R\x83\x83\x01Q\x81\x84\x01R\x81\x84\x01Q\x81\x83\x01R``\x80\x85\x01Q\x90\x82\x01R`\x80\x90 _\x81\x81R`\x05\x90\x93R\x91 T\x91\x92P\x90`\xFF\x16\x15a\x0BUW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FFunds have already withdrawn\0\0\0\0`D\x82\x01R`d\x01a\x03\xF9V[_\x90\x81R`\x05` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UPa\x0Bz\x92PPPV[PP[`\x01\x01a\x08\tV[P`\nT`@\x80Qc\x1DC\xC9}`\xE2\x1B\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cu\x0F%\xF4\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0B\xCAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xEE\x91\x90a%\xE4V[\x90P_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x0C9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0C>V[``\x91P[PP\x90P\x80a\x0C\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FFailed to transfer the fee to th`D\x82\x01Ri2\x90897\xB87\xB9\xB2\xB9`\xB1\x1B`d\x82\x01R`\x84\x01a\x03\xF9V[\x875`\x07U` \x88\x015`\x08U`@\x88\x015`\tU`\x03\x80T\x7FF\xBF\x14\x88'\xA7N\xB5\xC7\xCA\x85\xB1\x05\x131\xF7\xE0|kQ#\0X\x0CG\xBF\x1D\xD6F\xC1\xD8\x95\x91_a\x0C\xE7\x83a'qV[\x90\x91UP`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a$\xDDV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x15[\x9C\xDD\\\x1C\x1B\xDC\x9D\x08\x18\x9EH\x13\x9AY\xDA\x1D\x19\x98[\x1B`R\x1B`D\x82\x01R_\x90`d\x01a\x03\xF9V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a$\xDDV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0EHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FOnly the owner can call this fun`D\x82\x01Rd1\xBA4\xB7\xB7`\xD9\x1B`d\x82\x01R`\x84\x01a\x03\xF9V[`\n\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xAFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xD3\x91\x90a%\x14V[a\x0E\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a%3V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FY\x91\x90a%\x14V[\x15a\x0FvW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a%\x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R\x90\x81\x01\x85\x90R_\x90a\x0F\x9D\x90``\x01a\x07NV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82R` \x80\x83\x01\x8A\x81R_\x86\x81R`\x06\x90\x92R\x93\x81 \x92Q\x83T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x16\x91\x90\x91\x17\x82U\x91Q`\x01\x90\x91\x01U\x90\x91P`\x03\x83`\x03\x81\x11\x15a\x0F\xFCWa\x0F\xFCa'\x8FV[\x14a\x10\x07W\x81a\x10\xA7V[`@Qc\x13\x1F\x9F?`\xE1\x1B\x81R`\x04\x81\x81\x01\x88\x90R\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c&?>~\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10NW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10r\x91\x90a'\xA3V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x84\x01R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1C\x90\x1C[`@\x80Q`\x80\x80\x82\x01\x83R\x85\x82R` \x80\x83\x01\x85\x81R\x83\x85\x01\x8B\x81R``\x80\x86\x01\x8C\x81R\x87Q\x94\x85\x01\x8B\x90R\x92Q\x96\x84\x01\x96\x90\x96RQ\x94\x82\x01\x94\x90\x94R\x92Q\x90\x83\x01R\x91\x92P_\x90a\x10\xFB\x90`\xA0\x01a\x07NV[_\x81\x81R`\x04` R`@\x90 `\x01\x01T\x90\x91P`\xFF\x16\x15a\x11yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FFunds have already been escrowed`D\x82\x01Rp\x08\x19\x9B\xDC\x88\x1D\x1A\x1A\\\xC8\x11\x19\\\x1B\xDC\xDA]`z\x1B`d\x82\x01R`\x84\x01a\x03\xF9V[`\x03\x85`\x03\x81\x11\x15a\x11\x8DWa\x11\x8Da'\x8FV[\x03a\x11\xF9W`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c#\xB8r\xDD\x90`d\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11\xDEW_\x80\xFD[PZ\xF1\x15\x80\x15a\x11\xF0W=_\x80>=_\xFD[PPPPa\x13\xCDV[`\x01\x85`\x03\x81\x11\x15a\x12\rWa\x12\ra'\x8FV[\x03a\x12DW`@Qcy!!\x95`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xF2BC*\x90a\x11\xC7\x903\x900\x90\x8D\x90\x8D\x90`\x04\x01a'\xBAV[`\x02\x85`\x03\x81\x11\x15a\x12XWa\x12Xa'\x8FV[\x03a\x12\xC1W\x86\x15a\x12{W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a'\xF1V[`@Qc\\F\xA7\xEF`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x89\x90R`\x80`d\x82\x01R_`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xB8\x8DO\xDE\x90`\xA4\x01a\x11\xC7V[_\x85`\x03\x81\x11\x15a\x12\xD4Wa\x12\xD4a'\x8FV[\x03a\x13\xB4W\x87\x15a\x12\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a(:V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13GW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13k\x91\x90a%\x14V[a\x13\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11T\x90\xCC\x8C\x08\x1D\x1C\x98[\x9C\xD9\x99\\\x88\x19\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\x03\xF9V[a\x13\xCDV[`@Qc\xAF\xA9\x05\xDD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q``\x81\x01\x82R\x8B\x81R`\x01` \x80\x83\x01\x82\x81R_\x84\x86\x01\x81\x81R\x87\x82R`\x04\x90\x93R\x85\x90 \x93Q\x84UQ\x92\x90\x91\x01\x80T\x91Q`\xFF\x90\x81\x16a\x01\0\x02a\xFF\xFF\x19\x90\x93\x16\x93\x16\x92\x90\x92\x17\x17\x90UQ\x7F5G)\xB3XZ\xC9Q\x8AvY\xE8\xB323\x89\xF1H\xA7S\xE8\xC3\xC5\xD8\xC0\x1B\xFE\t\xBC\0\x93@\x90a\x14U\x90\x85\x90\x8A\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1a\x14h\x8A`\x02a(\x81V[4\x11\x15a\x16\x02W_a\x14{\x8B`\x02a(\x81V[a\x14\x85\x904a(\x98V[\x90P_`@Q\x80`\x80\x01`@R\x80`\x0CT\x81R` \x01`\x0CT\x81R` \x01\x83\x81R` \x01\x89\x81RP\x90P_a\x14\xEA\x82`@Q` \x01a\x07N\x91\x90\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[_\x81\x81R`\x04` R`@\x90 `\x01\x01T\x90\x91P`\xFF\x16\x15a\x15lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FFunds have already been escrowed`D\x82\x01Rt\x08\x19\x9B\xDC\x88\x1D\x1A\x1A\\\xC8\x19\x99YH\x11\x19\\\x1B\xDC\xDA]`Z\x1B`d\x82\x01R`\x84\x01a\x03\xF9V[`@\x80Q``\x81\x01\x82R\x8E\x81R`\x01` \x80\x83\x01\x82\x81R_\x84\x86\x01\x81\x81R\x87\x82R`\x04\x90\x93R\x85\x90 \x93Q\x84UQ\x92\x90\x91\x01\x80T\x91Q`\xFF\x90\x81\x16a\x01\0\x02a\xFF\xFF\x19\x90\x93\x16\x93\x16\x92\x90\x92\x17\x17\x90U`\x0CT\x90Q\x7F5G)\xB3XZ\xC9Q\x8AvY\xE8\xB323\x89\xF1H\xA7S\xE8\xC3\xC5\xD8\xC0\x1B\xFE\t\xBC\0\x93@\x91a\x15\xF6\x91\x86\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPP[PPPPPPPPPPV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16SW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16w\x91\x90a%\x14V[a\x16\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a%3V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xD9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xFD\x91\x90a%\x14V[\x15a\x17\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a%\x90V[_\x82`@Q` \x01a\x17,\x91\x90a$\x9EV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R`\x05\x90\x93R\x91 T\x90\x91P`\xFF\x16`\x01\x14a\x17\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FEither no funds are available to`D\x82\x01R\x7F withdraw, or they are already w`d\x82\x01Rg4\xBA4290\xBB\xB7`\xC1\x1B`\x84\x82\x01R`\xA4\x01a\x03\xF9V[\x825_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x01\x90\x91\x01T\x91\x83\x01\x91\x90\x91Ra\x18\xC8W_a\x18'`@\x86\x01` \x87\x01a\"\x97V[`\x01`\x01`\xA0\x1B\x03\x16\x85`@\x015`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x18rW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x18wV[``\x91P[PP\x90P\x80a\x18\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuCould not withdraw fee`P\x1B`D\x82\x01R`d\x01a\x03\xF9V[PPPPPV[_`\x01\x84`\x03\x81\x11\x15a\x18\xDDWa\x18\xDDa'\x8FV[\x03a\x19bW\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xF2BC*0a\x19\x03`@\x89\x01` \x8A\x01a\"\x97V[\x85` \x01Q\x89`@\x015`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19,\x94\x93\x92\x91\x90a'\xBAV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x19CW_\x80\xFD[PZ\xF1\x15\x80\x15a\x19UW=_\x80>=_\xFD[PPPP`\x01\x90Pa\x1A\xD7V[`\x02\x84`\x03\x81\x11\x15a\x19vWa\x19va'\x8FV[\x03a\x1A\nW`@\x85\x015\x15a\x19\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a'\xF1V[\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xB8\x8DO\xDE0a\x19\xBE`@\x89\x01` \x8A\x01a\"\x97V[` \x86\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`\x80`d\x82\x01R_`\x84\x82\x01R`\xA4\x01a\x19,V[_\x84`\x03\x81\x11\x15a\x1A\x1DWa\x1A\x1Da'\x8FV[\x03a\x1A\xD7W` \x82\x01Q\x15a\x1ADW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a(:V[\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa\x1Ad`@\x88\x01` \x89\x01a\"\x97V[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R\x88\x015`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1A\xB0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xD4\x91\x90a%\x14V[\x90P[\x80\x15a\x18\xC1WPP_\x90\x81R`\x05` R`@\x90 \x80T`\xFF\x19\x16\x90UPPV[``\x81\x81\x01Q\x80\x82\x01Q` \x80\x85\x01Q\x80Q\x81\x83\x01Q`@\x80\x84\x01Q\x93\x88\x01Q\x81\x8A\x01Q\x80Q\x81\x88\x01Q\x82\x85\x01Q\x92\x8C\x01Q\x8BQ\x8A\x8D\x01Q\x9C\x87\x01Q\x87Q\x9B\x8C\x01\x99\x90\x99R\x8A\x87\x01\x97\x90\x97R\x9B\x89\x01\x97\x90\x97R`\x80\x88\x01\x92\x90\x92R`\xA0\x87\x01\x91\x90\x91R`\xC0\x86\x01\x94\x90\x94R`\xE0\x85\x01\x93\x90\x93Ra\x01\0\x84\x01\x96\x90\x96Ra\x01 \x83\x01\x95\x90\x95Ra\x01@\x82\x01\x93\x90\x93Ra\x01`\x81\x01\x93\x90\x93R`\x01`\x01`\xFF\x1B\x03\x16a\x01\x80\x80\x84\x01\x82\x90R\x82Q\x80\x85\x03\x90\x91\x01\x81Ra\x01\xA0\x90\x93\x01\x90\x91R_\x91a\x1B\xBF\x81a\x1B\xC7V[\x94\x93PPPPV[_`@Q` \x81\x84Q` \x86\x01`\x02Z\xFAa\x1B\xE0W_\x80\xFD[Q`\x04\x1C\x92\x91PPV[_\x80\x80a\x1B\xFA`\x80\x86\x01\x86a(\xABV[a\x1C\x08\x91` \x91_\x91a(\xEEV[\x81\x01\x90a\x1C\x15\x91\x90a)\x15V[\x90P\x80_a\x1C&`\x80\x88\x01\x88a(\xABV[a\x1C5\x91`@\x91` \x91a(\xEEV[\x81\x01\x90a\x1CB\x91\x90a)\x15V[\x90P_a\x1CR`\x80\x89\x01\x89a(\xABV[a\x1Ca\x91``\x91`@\x91a(\xEEV[\x81\x01\x90a\x1Cn\x91\x90a)\x15V[\x90P_a\x1C~`\x80\x8A\x01\x8Aa(\xABV[a\x1C\x8D\x91`\x80\x91``\x91a(\xEEV[\x81\x01\x90a\x1C\x9A\x91\x90a)\x15V[\x90P_a\x1C\xAA`\x80\x8B\x01\x8Ba(\xABV[a\x1C\xB9\x91`\xA0\x91`\x80\x91a(\xEEV[\x81\x01\x90a\x1C\xC6\x91\x90a)\x15V[\x90P_a\x1C\xD6`\x80\x8C\x01\x8Ca(\xABV[a\x1C\xE5\x91`\xC0\x91`\xA0\x91a(\xEEV[\x81\x01\x90a\x1C\xF2\x91\x90a)\x15V[\x90P_a\x1D\x02`\x80\x8D\x01\x8Da(\xABV[a\x1D\x11\x91`\xE0\x91`\xC0\x91a(\xEEV[\x81\x01\x90a\x1D\x1E\x91\x90a)\x15V[\x90P_a\x1D.`\x80\x8E\x01\x8Ea(\xABV[a\x1D>\x91a\x01\0\x91`\xE0\x91a(\xEEV[\x81\x01\x90a\x1DK\x91\x90a)\x15V[\x90P_a\x1D[`\x80\x8F\x01\x8Fa(\xABV[a\x1Dl\x91a\x01 \x91a\x01\0\x91a(\xEEV[\x81\x01\x90a\x1Dy\x91\x90a)\x15V[`@\x80Q`\x10\x80\x82Ra\x02 \x82\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01a\x02\0\x806\x837\x01\x90PP\x90P\x8A\x81_\x81Q\x81\x10a\x1D\xB4Wa\x1D\xB4a&YV[` \x02` \x01\x01\x81\x81RPP\x8D_\x1B\x81`\x01\x81Q\x81\x10a\x1D\xD6Wa\x1D\xD6a&YV[` \x02` \x01\x01\x81\x81RPP`\x07T_\x1B\x81`\x02\x81Q\x81\x10a\x1D\xFAWa\x1D\xFAa&YV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80Q\x8F5\x90\x82\x90`\x03\x90\x81\x10a\x1E\x1DWa\x1E\x1Da&YV[` \x02` \x01\x01\x81\x81RPP`\x08T_\x1B\x81`\x04\x81Q\x81\x10a\x1EAWa\x1EAa&YV[` \x02` \x01\x01\x81\x81RPP\x8E` \x015_\x1B\x81`\x05\x81Q\x81\x10a\x1EgWa\x1Ega&YV[` \x02` \x01\x01\x81\x81RPP`\tT_\x1B\x81`\x06\x81Q\x81\x10a\x1E\x8BWa\x1E\x8Ba&YV[` \x02` \x01\x01\x81\x81RPP\x8E`@\x015_\x1B\x81`\x07\x81Q\x81\x10a\x1E\xB1Wa\x1E\xB1a&YV[` \x02` \x01\x01\x81\x81RPP\x88\x81`\x08\x81Q\x81\x10a\x1E\xD1Wa\x1E\xD1a&YV[` \x02` \x01\x01\x81\x81RPP\x87\x81`\t\x81Q\x81\x10a\x1E\xF1Wa\x1E\xF1a&YV[` \x02` \x01\x01\x81\x81RPP\x86\x81`\n\x81Q\x81\x10a\x1F\x11Wa\x1F\x11a&YV[` \x02` \x01\x01\x81\x81RPP\x85\x81`\x0B\x81Q\x81\x10a\x1F1Wa\x1F1a&YV[` \x02` \x01\x01\x81\x81RPP\x84\x81`\x0C\x81Q\x81\x10a\x1FQWa\x1FQa&YV[` \x02` \x01\x01\x81\x81RPP\x83\x81`\r\x81Q\x81\x10a\x1FqWa\x1Fqa&YV[` \x02` \x01\x01\x81\x81RPP\x82\x81`\x0E\x81Q\x81\x10a\x1F\x91Wa\x1F\x91a&YV[` \x02` \x01\x01\x81\x81RPP\x81\x81`\x0F\x81Q\x81\x10a\x1F\xB1Wa\x1F\xB1a&YV[` \x02` \x01\x01\x81\x81RPP_\x8F\x80`\x80\x01\x90a\x1F\xCE\x91\x90a(\xABV[a\x1F\xDD\x91a\x01 \x90\x82\x90a(\xEEV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`\x0BT`@Qc:\x9449`\xE2\x1B\x81R\x93\x94P`\x01`\x01`\xA0\x1B\x03\x16\x92c\xEAP\xD0\xE4\x92Pa C\x91P\x84\x90\x86\x90`\x04\x01a),V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a ^W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x82\x91\x90a%\x14V[\x9DP\x99\x9BPPPPPPPPPPPP[\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a \xAEW_\x80\xFD[PV[_\x80\x83`\x1F\x84\x01\x12a \xC1W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xD8W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a \x93W_\x80\xFD[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a!\x04W_\x80\xFD[\x865a!\x0F\x81a \x9AV[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!?W_\x80\xFD[a!K\x89\x82\x8A\x01a \xB1V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_` \x82\x84\x03\x12\x15a!mW_\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a!\x84W_\x80\xFD[\x93\x92PPPV[_\x80_\x80_`\x80\x86\x88\x03\x12\x15a!\x9FW_\x80\xFD[\x855a!\xAA\x81a \x9AV[\x94P` \x86\x015a!\xBA\x81a \x9AV[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xDCW_\x80\xFD[a!\xE8\x88\x82\x89\x01a \xB1V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_`\x80\x82\x84\x03\x12\x15a\"\tW_\x80\xFD[P\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a\"\x1FW_\x80\xFD[a!\x84\x83\x83a!\xF9V[_\x80`@\x83\x85\x03\x12\x15a\":W_\x80\xFD[\x825a\"E\x81a \x9AV[\x91P` \x83\x015a\"U\x81a \x9AV[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\"pW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x86W_\x80\xFD[\x82\x01`\xA0\x81\x85\x03\x12\x15a!\x84W_\x80\xFD[_` \x82\x84\x03\x12\x15a\"\xA7W_\x80\xFD[\x815a!\x84\x81a \x9AV[_\x80\x83`\x1F\x84\x01\x12a\"\xC2W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xD9W_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a \x93W_\x80\xFD[_\x80_\x80_\x80_\x80`\xA0\x89\x8B\x03\x12\x15a#\nW_\x80\xFD[\x885a#\x15\x81a \x9AV[\x97P` \x89\x015a#%\x81a \x9AV[\x96P`@\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#AW_\x80\xFD[a#M\x8C\x83\x8D\x01a\"\xB2V[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15a#eW_\x80\xFD[a#q\x8C\x83\x8D\x01a\"\xB2V[\x90\x96P\x94P`\x80\x8B\x015\x91P\x80\x82\x11\x15a#\x89W_\x80\xFD[Pa#\x96\x8B\x82\x8C\x01a \xB1V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[\x805`\x04\x81\x10a#\xB8W_\x80\xFD[\x91\x90PV[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a#\xD2W_\x80\xFD[\x865\x95P` \x87\x015a#\xE4\x81a \x9AV[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015\x91Pa$\x07`\xA0\x88\x01a#\xAAV[\x90P\x92\x95P\x92\x95P\x92\x95V[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a$(W_\x80\xFD[\x865a$3\x81a \x9AV[\x95P` \x87\x015a$C\x81a \x9AV[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!?W_\x80\xFD[_\x80`\xA0\x83\x85\x03\x12\x15a$}W_\x80\xFD[a$\x87\x84\x84a!\xF9V[\x91Pa$\x95`\x80\x84\x01a#\xAAV[\x90P\x92P\x92\x90PV[\x815\x81R`\x80\x81\x01` \x83\x015a$\xB4\x81a \x9AV[`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R`@\x83\x81\x015\x90\x83\x01R``\x92\x83\x015\x92\x90\x91\x01\x91\x90\x91R\x90V[` \x80\x82R`\x17\x90\x82\x01R\x7FCaller is not the owner\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a%$W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a!\x84W_\x80\xFD[` \x80\x82R`=\x90\x82\x01R\x7FCertified: You are not authorise`@\x82\x01R\x7Fd to transact using Nightfall\0\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`4\x90\x82\x01R\x7FCertified: You are on the Chaina`@\x82\x01Rs\x1B\x1E\\\xDA\\\xC8\x1C\xD8[\x98\xDD\x1A[\xDB\x9C\xC8\x1B\x1A\\\xDD`b\x1B``\x82\x01R`\x80\x01\x90V[_` \x82\x84\x03\x12\x15a%\xF4W_\x80\xFD[\x81Qa!\x84\x81a \x9AV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a&(W_\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&BW_\x80\xFD[` \x01\x91Pa\x01\xA0\x81\x026\x03\x82\x13\x15a \x93W_\x80\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a&\x9CWcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@R\x90V[_\x82`\x1F\x83\x01\x12a&\xB1W_\x80\xFD[a&\xB9a&mV[\x80`\x80\x84\x01\x85\x81\x11\x15a&\xCAW_\x80\xFD[\x84[\x81\x81\x10\x15a&\xE4W\x805\x84R` \x93\x84\x01\x93\x01a&\xCCV[P\x90\x95\x94PPPPPV[_a\x01\xA0\x82\x84\x03\x12\x15a'\0W_\x80\xFD[a'\x08a&mV[\x825\x81Ra'\x19\x84` \x85\x01a&\xA2V[` \x82\x01Ra'+\x84`\xA0\x85\x01a&\xA2V[`@\x82\x01Ra'>\x84a\x01 \x85\x01a&\xA2V[``\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x03\x88Wa\x03\x88a'JV[_`\x01`\x01`\xFF\x1B\x01\x82\x01a'\x88Wa'\x88a'JV[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a'\xB3W_\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01R`@\x82\x01R``\x81\x01\x91\x90\x91R`\xA0`\x80\x82\x01\x81\x90R_\x90\x82\x01R`\xC0\x01\x90V[` \x80\x82R`)\x90\x82\x01R\x7FERC721 tokens should have a valu`@\x82\x01Rhe of zero`\xB8\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`'\x90\x82\x01R\x7FERC20 tokens should have a token`@\x82\x01Rf\x04\x96B\x06\xF6b\x03`\xCC\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x88Wa\x03\x88a'JV[\x81\x81\x03\x81\x81\x11\x15a\x03\x88Wa\x03\x88a'JV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a(\xC0W_\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(\xDAW_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a \x93W_\x80\xFD[_\x80\x85\x85\x11\x15a(\xFCW_\x80\xFD[\x83\x86\x11\x15a)\x08W_\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[_` \x82\x84\x03\x12\x15a)%W_\x80\xFD[P5\x91\x90PV[`@\x81R_\x83Q\x80`@\x84\x01R_[\x81\x81\x10\x15a)XW` \x81\x87\x01\x81\x01Q``\x86\x84\x01\x01R\x01a);V[P_``\x82\x85\x01\x01R`\x1F\x19`\x1F\x82\x01\x16\x83\x01\x90P``\x81\x01` ``\x85\x84\x03\x01` \x86\x01R\x81\x86Q\x80\x84R`\x80\x85\x01\x91P` \x88\x01\x94P_\x93P[\x80\x84\x10\x15a)\xB4W\x84Q\x82R\x93\x82\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a)\x94V[P\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \xEB \xA6ql\xE9\xA9\xEB{n7\x13\x93\xE6\xE8\xEA\xB3\xCC\0<\xF1\xD3\xD9\\\xB2\xFD\xB9: \xAD`\xDEdsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static NIGHTFALL_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xE3W_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0\x87W\x80c\xE3\x17\x8C\x86\x11a\0WW\x80c\xE3\x17\x8C\x86\x14a\x02\xB2W\x80c\xE6\xD5\xAB\xE5\x14a\x02\xD1W\x80c\xF2:na\x14a\x02\xE4W\x80c\xF3\xB8_\xC2\x14a\x03\x10W_\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x02\x1EW\x80c\x97\xE2\r\x0E\x14a\x02UW\x80c\xBC\x19|\x81\x14a\x02tW\x80c\xD5\xE6\xC6\xF9\x14a\x02\x93W_\x80\xFD[\x80c\x18jM\x08\x11a\0\xC2W\x80c\x18jM\x08\x14a\x01\x8AW\x80c(\xC3\xD7\xE6\x14a\x01\xBBW\x80cJ\x8A@s\x14a\x01\xDEW\x80cu\x13xu\x14a\x01\xFFW_\x80\xFD[\x80b\x9C\xE2\x0B\x14a\0\xE7W\x80c\x01\xFF\xC9\xA7\x14a\x010W\x80c\x15\x0Bz\x02\x14a\x01_W[_\x80\xFD[4\x80\x15a\0\xF2W_\x80\xFD[Pa\x01\x12a\x01\x016`\x04a \xEFV[b\x9C\xE2\x0B`\xE0\x1B\x96\x95PPPPPPV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01;W_\x80\xFD[Pa\x01Oa\x01J6`\x04a!]V[a\x03#V[`@Q\x90\x15\x15\x81R` \x01a\x01'V[4\x80\x15a\x01jW_\x80\xFD[Pa\x01\x12a\x01y6`\x04a!\x8BV[c\n\x85\xBD\x01`\xE1\x1B\x95\x94PPPPPV[4\x80\x15a\x01\x95W_\x80\xFD[Pa\x01\xA9a\x01\xA46`\x04a\"\x0FV[a\x03\x8EV[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01'V[4\x80\x15a\x01\xC6W_\x80\xFD[Pa\x01\xD0`\x03T\x81V[`@Q\x90\x81R` \x01a\x01'V[4\x80\x15a\x01\xE9W_\x80\xFD[Pa\x01\xFDa\x01\xF86`\x04a\")V[a\x03\xCFV[\0[4\x80\x15a\x02\nW_\x80\xFD[Pa\x01\xFDa\x02\x196`\x04a\"`V[a\x042V[4\x80\x15a\x02)W_\x80\xFD[P`\x02Ta\x02=\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01'V[4\x80\x15a\x02`W_\x80\xFD[Pa\x01\xFDa\x02o6`\x04a\"\x97V[a\r\x06V[4\x80\x15a\x02\x7FW_\x80\xFD[Pa\x01\x12a\x02\x8E6`\x04a\"\xF3V[a\rRV[4\x80\x15a\x02\x9EW_\x80\xFD[Pa\x01\xFDa\x02\xAD6`\x04a\"\x97V[a\r\x95V[4\x80\x15a\x02\xBDW_\x80\xFD[Pa\x01\xFDa\x02\xCC6`\x04a\"\x97V[a\r\xE0V[a\x01\xFDa\x02\xDF6`\x04a#\xBDV[a\x0EjV[4\x80\x15a\x02\xEFW_\x80\xFD[Pa\x01\x12a\x02\xFE6`\x04a$\x13V[c\xF2:na`\xE0\x1B\x96\x95PPPPPPV[a\x01\xFDa\x03\x1E6`\x04a$lV[a\x16\x0EV[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x80a\x03RWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16b\x9C\xE2\x0B`\xE0\x1B\x14[\x80a\x03mWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\n\x85\xBD\x01`\xE1\x1B\x14[\x80a\x03\x88WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x02q\x18\x97`\xE5\x1B\x14[\x92\x91PPV[_\x80\x82`@Q` \x01a\x03\xA1\x91\x90a$\x9EV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x90\x81R`\x05\x90\x92R\x90 T`\xFF\x16\x93\x92PPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a$\xDDV[`@Q\x80\x91\x03\x90\xFD[_\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x01\x80T\x93\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04wW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x9B\x91\x90a%\x14V[a\x04\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a%3V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xFDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05!\x91\x90a%\x14V[\x15a\x05>W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a%\x90V[`\nT`@\x80Qc\x1DC\xC9}`\xE2\x1B\x81R\x90Q3\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cu\x0F%\xF4\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x05\x85W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xA9\x91\x90a%\xE4V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FOnly the current proposer can pr`D\x82\x01Rlopose a block`\x98\x1B`d\x82\x01R`\x84\x01a\x03\xF9V[``\x81\x015\x81\x015_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x067Wa\x067a%\xFFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06`W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a\x06\xCEWa\x06\xA9a\x06}``\x86\x01\x86a&\x13V[\x83\x81\x81\x10a\x06\x8DWa\x06\x8Da&YV[\x90Pa\x01\xA0\x02\x01\x806\x03\x81\x01\x90a\x06\xA4\x91\x90a&\xEFV[a\x1A\xF8V[\x82\x82\x81Q\x81\x10a\x06\xBBWa\x06\xBBa&YV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x06eV[P\x80\x82[`\x01\x81\x11\x15a\x07\x90W_[`\x01\x82\x90\x1C\x81\x10\x15a\x07\x87Wa\x07b\x83`\x01\x83\x90\x1B\x81Q\x81\x10a\x07\x02Wa\x07\x02a&YV[` \x02` \x01\x01Q\x84`\x01\x84\x90\x1B`\x01a\x07\x1C\x91\x90a'^V[\x81Q\x81\x10a\x07,Wa\x07,a&YV[` \x02` \x01\x01Q`@Q` \x01a\x07N\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1B\xC7V[\x83\x82\x81Q\x81\x10a\x07tWa\x07ta&YV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x06\xDDV[P`\x01\x1Ca\x06\xD2V[P_\x80a\x07\xB6\x86\x85_\x81Q\x81\x10a\x07\xA9Wa\x07\xA9a&YV[` \x02` \x01\x01Qa\x1B\xEAV[\x91P\x91P\x81a\x08\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FRollup proof verification failed`D\x82\x01R`d\x01a\x03\xF9V[_[\x85\x81\x10\x15a\x0B\x82W``\x87\x015a\x01\xA0\x82\x02\x01\x87\x01`\xC0\x015\x15\x80\x15a\tzW``\x88\x015a\x01\xA0\x83\x02\x01\x88\x01a\x01@\x015\x15\x80\x15a\x08IWPPa\x0BzV[\x83_\x80[`\x04\x81\x10\x15a\tmW``\x8C\x015a\x01\xA0\x87\x02\x01` \x82\x02\x01\x8C\x01a\x01@\x015\x91P\x81\x15a\teW_\x82\x81R`\x04` R`@\x90 Ta\x08\x8D\x90\x84a'^V[_\x83\x81R`\x04` R`@\x90 `\x01\x90\x81\x01T\x91\x94P`\xFF\x90\x91\x16\x14\x80\x15a\x08\xCBWP_\x82\x81R`\x04` R`@\x90 `\x01\x01Ta\x01\0\x90\x04`\xFF\x16\x15[a\tGW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FDeposit either has not been escr`D\x82\x01R\x7Fowed or has already been redeeme`d\x82\x01R`\x19`\xFA\x1B`\x84\x82\x01R`\xA4\x01a\x03\xF9V[_\x82\x81R`\x04` R`@\x90 `\x01\x01\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01\x01a\x08MV[P\x81\x95PPPPPa\x0BzV[`@``\x89\x015a\x01\xA0\x84\x02\x01\x89\x01\x90\x81\x015\x15`\xC0\x91\x90\x91\x015\x15\x15\x16\x80\x15a\x0BwW_a\t\xAC``\x8B\x01\x8Ba&\x13V[\x85\x81\x81\x10a\t\xBCWa\t\xBCa&YV[\x90Pa\x01\xA0\x02\x01`\xA0\x01_`\x04\x81\x10a\t\xD7Wa\t\xD7a&YV[` \x02\x015\x90P_`@Q\x80`\x80\x01`@R\x80\x8C\x80``\x01\x90a\t\xFA\x91\x90a&\x13V[\x88\x81\x81\x10a\n\nWa\n\na&YV[\x90Pa\x01\xA0\x02\x01a\x01 \x01_`\x04\x81\x10a\n&Wa\n&a&YV[` \x02\x015\x81R` \x01\x8C\x80``\x01\x90a\n@\x91\x90a&\x13V[\x88\x81\x81\x10a\nPWa\nPa&YV[\x90Pa\x01\xA0\x02\x01a\x01 \x01`\x01`\x04\x81\x10a\nmWa\nma&YV[` \x02\x015`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x8C\x80``\x01\x90a\n\x90\x91\x90a&\x13V[\x88\x81\x81\x10a\n\xA0Wa\n\xA0a&YV[\x90Pa\x01\xA0\x02\x01a\x01 \x01`\x02`\x04\x81\x10a\n\xBDWa\n\xBDa&YV[` \x90\x81\x02\x91\x90\x91\x015\x82R\x90\x81\x01\x84\x90R`@\x80Q\x83Q\x81R\x83\x83\x01Q\x81\x84\x01R\x81\x84\x01Q\x81\x83\x01R``\x80\x85\x01Q\x90\x82\x01R`\x80\x90 _\x81\x81R`\x05\x90\x93R\x91 T\x91\x92P\x90`\xFF\x16\x15a\x0BUW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FFunds have already withdrawn\0\0\0\0`D\x82\x01R`d\x01a\x03\xF9V[_\x90\x81R`\x05` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UPa\x0Bz\x92PPPV[PP[`\x01\x01a\x08\tV[P`\nT`@\x80Qc\x1DC\xC9}`\xE2\x1B\x81R\x90Q_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91cu\x0F%\xF4\x91`\x04\x80\x83\x01\x92` \x92\x91\x90\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0B\xCAW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xEE\x91\x90a%\xE4V[\x90P_\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x0C9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x0C>V[``\x91P[PP\x90P\x80a\x0C\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FFailed to transfer the fee to th`D\x82\x01Ri2\x90897\xB87\xB9\xB2\xB9`\xB1\x1B`d\x82\x01R`\x84\x01a\x03\xF9V[\x875`\x07U` \x88\x015`\x08U`@\x88\x015`\tU`\x03\x80T\x7FF\xBF\x14\x88'\xA7N\xB5\xC7\xCA\x85\xB1\x05\x131\xF7\xE0|kQ#\0X\x0CG\xBF\x1D\xD6F\xC1\xD8\x95\x91_a\x0C\xE7\x83a'qV[\x90\x91UP`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1PPPPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a$\xDDV[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01Ru\x15[\x9C\xDD\\\x1C\x1B\xDC\x9D\x08\x18\x9EH\x13\x9AY\xDA\x1D\x19\x98[\x1B`R\x1B`D\x82\x01R_\x90`d\x01a\x03\xF9V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\r\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a$\xDDV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0EHW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FOnly the owner can call this fun`D\x82\x01Rd1\xBA4\xB7\xB7`\xD9\x1B`d\x82\x01R`\x84\x01a\x03\xF9V[`\n\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xAFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xD3\x91\x90a%\x14V[a\x0E\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a%3V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0FY\x91\x90a%\x14V[\x15a\x0FvW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a%\x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x87\x16` \x82\x01R\x90\x81\x01\x85\x90R_\x90a\x0F\x9D\x90``\x01a\x07NV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x82R` \x80\x83\x01\x8A\x81R_\x86\x81R`\x06\x90\x92R\x93\x81 \x92Q\x83T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x16\x91\x90\x91\x17\x82U\x91Q`\x01\x90\x91\x01U\x90\x91P`\x03\x83`\x03\x81\x11\x15a\x0F\xFCWa\x0F\xFCa'\x8FV[\x14a\x10\x07W\x81a\x10\xA7V[`@Qc\x13\x1F\x9F?`\xE1\x1B\x81R`\x04\x81\x81\x01\x88\x90R\x90\x88\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c&?>~\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10NW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10r\x91\x90a'\xA3V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x93\x16` \x84\x01R\x82\x01R``\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 _\x1C\x90\x1C[`@\x80Q`\x80\x80\x82\x01\x83R\x85\x82R` \x80\x83\x01\x85\x81R\x83\x85\x01\x8B\x81R``\x80\x86\x01\x8C\x81R\x87Q\x94\x85\x01\x8B\x90R\x92Q\x96\x84\x01\x96\x90\x96RQ\x94\x82\x01\x94\x90\x94R\x92Q\x90\x83\x01R\x91\x92P_\x90a\x10\xFB\x90`\xA0\x01a\x07NV[_\x81\x81R`\x04` R`@\x90 `\x01\x01T\x90\x91P`\xFF\x16\x15a\x11yW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FFunds have already been escrowed`D\x82\x01Rp\x08\x19\x9B\xDC\x88\x1D\x1A\x1A\\\xC8\x11\x19\\\x1B\xDC\xDA]`z\x1B`d\x82\x01R`\x84\x01a\x03\xF9V[`\x03\x85`\x03\x81\x11\x15a\x11\x8DWa\x11\x8Da'\x8FV[\x03a\x11\xF9W`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x89\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c#\xB8r\xDD\x90`d\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x11\xDEW_\x80\xFD[PZ\xF1\x15\x80\x15a\x11\xF0W=_\x80>=_\xFD[PPPPa\x13\xCDV[`\x01\x85`\x03\x81\x11\x15a\x12\rWa\x12\ra'\x8FV[\x03a\x12DW`@Qcy!!\x95`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xF2BC*\x90a\x11\xC7\x903\x900\x90\x8D\x90\x8D\x90`\x04\x01a'\xBAV[`\x02\x85`\x03\x81\x11\x15a\x12XWa\x12Xa'\x8FV[\x03a\x12\xC1W\x86\x15a\x12{W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a'\xF1V[`@Qc\\F\xA7\xEF`\xE1\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x89\x90R`\x80`d\x82\x01R_`\x84\x82\x01R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c\xB8\x8DO\xDE\x90`\xA4\x01a\x11\xC7V[_\x85`\x03\x81\x11\x15a\x12\xD4Wa\x12\xD4a'\x8FV[\x03a\x13\xB4W\x87\x15a\x12\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a(:V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x88\x90R`\x01`\x01`\xA0\x1B\x03\x8A\x16\x90c#\xB8r\xDD\x90`d\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13GW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13k\x91\x90a%\x14V[a\x13\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01Rt\x11T\x90\xCC\x8C\x08\x1D\x1C\x98[\x9C\xD9\x99\\\x88\x19\x98Z[\x19Y`Z\x1B`D\x82\x01R`d\x01a\x03\xF9V[a\x13\xCDV[`@Qc\xAF\xA9\x05\xDD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q``\x81\x01\x82R\x8B\x81R`\x01` \x80\x83\x01\x82\x81R_\x84\x86\x01\x81\x81R\x87\x82R`\x04\x90\x93R\x85\x90 \x93Q\x84UQ\x92\x90\x91\x01\x80T\x91Q`\xFF\x90\x81\x16a\x01\0\x02a\xFF\xFF\x19\x90\x93\x16\x93\x16\x92\x90\x92\x17\x17\x90UQ\x7F5G)\xB3XZ\xC9Q\x8AvY\xE8\xB323\x89\xF1H\xA7S\xE8\xC3\xC5\xD8\xC0\x1B\xFE\t\xBC\0\x93@\x90a\x14U\x90\x85\x90\x8A\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1a\x14h\x8A`\x02a(\x81V[4\x11\x15a\x16\x02W_a\x14{\x8B`\x02a(\x81V[a\x14\x85\x904a(\x98V[\x90P_`@Q\x80`\x80\x01`@R\x80`\x0CT\x81R` \x01`\x0CT\x81R` \x01\x83\x81R` \x01\x89\x81RP\x90P_a\x14\xEA\x82`@Q` \x01a\x07N\x91\x90\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[_\x81\x81R`\x04` R`@\x90 `\x01\x01T\x90\x91P`\xFF\x16\x15a\x15lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FFunds have already been escrowed`D\x82\x01Rt\x08\x19\x9B\xDC\x88\x1D\x1A\x1A\\\xC8\x19\x99YH\x11\x19\\\x1B\xDC\xDA]`Z\x1B`d\x82\x01R`\x84\x01a\x03\xF9V[`@\x80Q``\x81\x01\x82R\x8E\x81R`\x01` \x80\x83\x01\x82\x81R_\x84\x86\x01\x81\x81R\x87\x82R`\x04\x90\x93R\x85\x90 \x93Q\x84UQ\x92\x90\x91\x01\x80T\x91Q`\xFF\x90\x81\x16a\x01\0\x02a\xFF\xFF\x19\x90\x93\x16\x93\x16\x92\x90\x92\x17\x17\x90U`\x0CT\x90Q\x7F5G)\xB3XZ\xC9Q\x8AvY\xE8\xB323\x89\xF1H\xA7S\xE8\xC3\xC5\xD8\xC0\x1B\xFE\t\xBC\0\x93@\x91a\x15\xF6\x91\x86\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPP[PPPPPPPPPPV[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16SW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16w\x91\x90a%\x14V[a\x16\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a%3V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xD9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xFD\x91\x90a%\x14V[\x15a\x17\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a%\x90V[_\x82`@Q` \x01a\x17,\x91\x90a$\x9EV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 _\x81\x81R`\x05\x90\x93R\x91 T\x90\x91P`\xFF\x16`\x01\x14a\x17\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R\x7FEither no funds are available to`D\x82\x01R\x7F withdraw, or they are already w`d\x82\x01Rg4\xBA4290\xBB\xB7`\xC1\x1B`\x84\x82\x01R`\xA4\x01a\x03\xF9V[\x825_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x01\x90\x91\x01T\x91\x83\x01\x91\x90\x91Ra\x18\xC8W_a\x18'`@\x86\x01` \x87\x01a\"\x97V[`\x01`\x01`\xA0\x1B\x03\x16\x85`@\x015`@Q_`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80_\x81\x14a\x18rW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x18wV[``\x91P[PP\x90P\x80a\x18\xC1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x16`$\x82\x01RuCould not withdraw fee`P\x1B`D\x82\x01R`d\x01a\x03\xF9V[PPPPPV[_`\x01\x84`\x03\x81\x11\x15a\x18\xDDWa\x18\xDDa'\x8FV[\x03a\x19bW\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xF2BC*0a\x19\x03`@\x89\x01` \x8A\x01a\"\x97V[\x85` \x01Q\x89`@\x015`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19,\x94\x93\x92\x91\x90a'\xBAV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x19CW_\x80\xFD[PZ\xF1\x15\x80\x15a\x19UW=_\x80>=_\xFD[PPPP`\x01\x90Pa\x1A\xD7V[`\x02\x84`\x03\x81\x11\x15a\x19vWa\x19va'\x8FV[\x03a\x1A\nW`@\x85\x015\x15a\x19\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a'\xF1V[\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xB8\x8DO\xDE0a\x19\xBE`@\x89\x01` \x8A\x01a\"\x97V[` \x86\x01Q`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x86\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`D\x82\x01R`\x80`d\x82\x01R_`\x84\x82\x01R`\xA4\x01a\x19,V[_\x84`\x03\x81\x11\x15a\x1A\x1DWa\x1A\x1Da'\x8FV[\x03a\x1A\xD7W` \x82\x01Q\x15a\x1ADW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x03\xF9\x90a(:V[\x81Q`\x01`\x01`\xA0\x1B\x03\x16c\xA9\x05\x9C\xBBa\x1Ad`@\x88\x01` \x89\x01a\"\x97V[`@\x80Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R\x88\x015`$\x82\x01R`D\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1A\xB0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xD4\x91\x90a%\x14V[\x90P[\x80\x15a\x18\xC1WPP_\x90\x81R`\x05` R`@\x90 \x80T`\xFF\x19\x16\x90UPPV[``\x81\x81\x01Q\x80\x82\x01Q` \x80\x85\x01Q\x80Q\x81\x83\x01Q`@\x80\x84\x01Q\x93\x88\x01Q\x81\x8A\x01Q\x80Q\x81\x88\x01Q\x82\x85\x01Q\x92\x8C\x01Q\x8BQ\x8A\x8D\x01Q\x9C\x87\x01Q\x87Q\x9B\x8C\x01\x99\x90\x99R\x8A\x87\x01\x97\x90\x97R\x9B\x89\x01\x97\x90\x97R`\x80\x88\x01\x92\x90\x92R`\xA0\x87\x01\x91\x90\x91R`\xC0\x86\x01\x94\x90\x94R`\xE0\x85\x01\x93\x90\x93Ra\x01\0\x84\x01\x96\x90\x96Ra\x01 \x83\x01\x95\x90\x95Ra\x01@\x82\x01\x93\x90\x93Ra\x01`\x81\x01\x93\x90\x93R`\x01`\x01`\xFF\x1B\x03\x16a\x01\x80\x80\x84\x01\x82\x90R\x82Q\x80\x85\x03\x90\x91\x01\x81Ra\x01\xA0\x90\x93\x01\x90\x91R_\x91a\x1B\xBF\x81a\x1B\xC7V[\x94\x93PPPPV[_`@Q` \x81\x84Q` \x86\x01`\x02Z\xFAa\x1B\xE0W_\x80\xFD[Q`\x04\x1C\x92\x91PPV[_\x80\x80a\x1B\xFA`\x80\x86\x01\x86a(\xABV[a\x1C\x08\x91` \x91_\x91a(\xEEV[\x81\x01\x90a\x1C\x15\x91\x90a)\x15V[\x90P\x80_a\x1C&`\x80\x88\x01\x88a(\xABV[a\x1C5\x91`@\x91` \x91a(\xEEV[\x81\x01\x90a\x1CB\x91\x90a)\x15V[\x90P_a\x1CR`\x80\x89\x01\x89a(\xABV[a\x1Ca\x91``\x91`@\x91a(\xEEV[\x81\x01\x90a\x1Cn\x91\x90a)\x15V[\x90P_a\x1C~`\x80\x8A\x01\x8Aa(\xABV[a\x1C\x8D\x91`\x80\x91``\x91a(\xEEV[\x81\x01\x90a\x1C\x9A\x91\x90a)\x15V[\x90P_a\x1C\xAA`\x80\x8B\x01\x8Ba(\xABV[a\x1C\xB9\x91`\xA0\x91`\x80\x91a(\xEEV[\x81\x01\x90a\x1C\xC6\x91\x90a)\x15V[\x90P_a\x1C\xD6`\x80\x8C\x01\x8Ca(\xABV[a\x1C\xE5\x91`\xC0\x91`\xA0\x91a(\xEEV[\x81\x01\x90a\x1C\xF2\x91\x90a)\x15V[\x90P_a\x1D\x02`\x80\x8D\x01\x8Da(\xABV[a\x1D\x11\x91`\xE0\x91`\xC0\x91a(\xEEV[\x81\x01\x90a\x1D\x1E\x91\x90a)\x15V[\x90P_a\x1D.`\x80\x8E\x01\x8Ea(\xABV[a\x1D>\x91a\x01\0\x91`\xE0\x91a(\xEEV[\x81\x01\x90a\x1DK\x91\x90a)\x15V[\x90P_a\x1D[`\x80\x8F\x01\x8Fa(\xABV[a\x1Dl\x91a\x01 \x91a\x01\0\x91a(\xEEV[\x81\x01\x90a\x1Dy\x91\x90a)\x15V[`@\x80Q`\x10\x80\x82Ra\x02 \x82\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01a\x02\0\x806\x837\x01\x90PP\x90P\x8A\x81_\x81Q\x81\x10a\x1D\xB4Wa\x1D\xB4a&YV[` \x02` \x01\x01\x81\x81RPP\x8D_\x1B\x81`\x01\x81Q\x81\x10a\x1D\xD6Wa\x1D\xD6a&YV[` \x02` \x01\x01\x81\x81RPP`\x07T_\x1B\x81`\x02\x81Q\x81\x10a\x1D\xFAWa\x1D\xFAa&YV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80Q\x8F5\x90\x82\x90`\x03\x90\x81\x10a\x1E\x1DWa\x1E\x1Da&YV[` \x02` \x01\x01\x81\x81RPP`\x08T_\x1B\x81`\x04\x81Q\x81\x10a\x1EAWa\x1EAa&YV[` \x02` \x01\x01\x81\x81RPP\x8E` \x015_\x1B\x81`\x05\x81Q\x81\x10a\x1EgWa\x1Ega&YV[` \x02` \x01\x01\x81\x81RPP`\tT_\x1B\x81`\x06\x81Q\x81\x10a\x1E\x8BWa\x1E\x8Ba&YV[` \x02` \x01\x01\x81\x81RPP\x8E`@\x015_\x1B\x81`\x07\x81Q\x81\x10a\x1E\xB1Wa\x1E\xB1a&YV[` \x02` \x01\x01\x81\x81RPP\x88\x81`\x08\x81Q\x81\x10a\x1E\xD1Wa\x1E\xD1a&YV[` \x02` \x01\x01\x81\x81RPP\x87\x81`\t\x81Q\x81\x10a\x1E\xF1Wa\x1E\xF1a&YV[` \x02` \x01\x01\x81\x81RPP\x86\x81`\n\x81Q\x81\x10a\x1F\x11Wa\x1F\x11a&YV[` \x02` \x01\x01\x81\x81RPP\x85\x81`\x0B\x81Q\x81\x10a\x1F1Wa\x1F1a&YV[` \x02` \x01\x01\x81\x81RPP\x84\x81`\x0C\x81Q\x81\x10a\x1FQWa\x1FQa&YV[` \x02` \x01\x01\x81\x81RPP\x83\x81`\r\x81Q\x81\x10a\x1FqWa\x1Fqa&YV[` \x02` \x01\x01\x81\x81RPP\x82\x81`\x0E\x81Q\x81\x10a\x1F\x91Wa\x1F\x91a&YV[` \x02` \x01\x01\x81\x81RPP\x81\x81`\x0F\x81Q\x81\x10a\x1F\xB1Wa\x1F\xB1a&YV[` \x02` \x01\x01\x81\x81RPP_\x8F\x80`\x80\x01\x90a\x1F\xCE\x91\x90a(\xABV[a\x1F\xDD\x91a\x01 \x90\x82\x90a(\xEEV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`\x0BT`@Qc:\x9449`\xE2\x1B\x81R\x93\x94P`\x01`\x01`\xA0\x1B\x03\x16\x92c\xEAP\xD0\xE4\x92Pa C\x91P\x84\x90\x86\x90`\x04\x01a),V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a ^W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a \x82\x91\x90a%\x14V[\x9DP\x99\x9BPPPPPPPPPPPP[\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a \xAEW_\x80\xFD[PV[_\x80\x83`\x1F\x84\x01\x12a \xC1W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xD8W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a \x93W_\x80\xFD[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a!\x04W_\x80\xFD[\x865a!\x0F\x81a \x9AV[\x95P` \x87\x015\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!?W_\x80\xFD[a!K\x89\x82\x8A\x01a \xB1V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_` \x82\x84\x03\x12\x15a!mW_\x80\xFD[\x815`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a!\x84W_\x80\xFD[\x93\x92PPPV[_\x80_\x80_`\x80\x86\x88\x03\x12\x15a!\x9FW_\x80\xFD[\x855a!\xAA\x81a \x9AV[\x94P` \x86\x015a!\xBA\x81a \x9AV[\x93P`@\x86\x015\x92P``\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!\xDCW_\x80\xFD[a!\xE8\x88\x82\x89\x01a \xB1V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_`\x80\x82\x84\x03\x12\x15a\"\tW_\x80\xFD[P\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a\"\x1FW_\x80\xFD[a!\x84\x83\x83a!\xF9V[_\x80`@\x83\x85\x03\x12\x15a\":W_\x80\xFD[\x825a\"E\x81a \x9AV[\x91P` \x83\x015a\"U\x81a \x9AV[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\"pW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x86W_\x80\xFD[\x82\x01`\xA0\x81\x85\x03\x12\x15a!\x84W_\x80\xFD[_` \x82\x84\x03\x12\x15a\"\xA7W_\x80\xFD[\x815a!\x84\x81a \x9AV[_\x80\x83`\x1F\x84\x01\x12a\"\xC2W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xD9W_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a \x93W_\x80\xFD[_\x80_\x80_\x80_\x80`\xA0\x89\x8B\x03\x12\x15a#\nW_\x80\xFD[\x885a#\x15\x81a \x9AV[\x97P` \x89\x015a#%\x81a \x9AV[\x96P`@\x89\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#AW_\x80\xFD[a#M\x8C\x83\x8D\x01a\"\xB2V[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15a#eW_\x80\xFD[a#q\x8C\x83\x8D\x01a\"\xB2V[\x90\x96P\x94P`\x80\x8B\x015\x91P\x80\x82\x11\x15a#\x89W_\x80\xFD[Pa#\x96\x8B\x82\x8C\x01a \xB1V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[\x805`\x04\x81\x10a#\xB8W_\x80\xFD[\x91\x90PV[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a#\xD2W_\x80\xFD[\x865\x95P` \x87\x015a#\xE4\x81a \x9AV[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015\x91Pa$\x07`\xA0\x88\x01a#\xAAV[\x90P\x92\x95P\x92\x95P\x92\x95V[_\x80_\x80_\x80`\xA0\x87\x89\x03\x12\x15a$(W_\x80\xFD[\x865a$3\x81a \x9AV[\x95P` \x87\x015a$C\x81a \x9AV[\x94P`@\x87\x015\x93P``\x87\x015\x92P`\x80\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!?W_\x80\xFD[_\x80`\xA0\x83\x85\x03\x12\x15a$}W_\x80\xFD[a$\x87\x84\x84a!\xF9V[\x91Pa$\x95`\x80\x84\x01a#\xAAV[\x90P\x92P\x92\x90PV[\x815\x81R`\x80\x81\x01` \x83\x015a$\xB4\x81a \x9AV[`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R`@\x83\x81\x015\x90\x83\x01R``\x92\x83\x015\x92\x90\x91\x01\x91\x90\x91R\x90V[` \x80\x82R`\x17\x90\x82\x01R\x7FCaller is not the owner\0\0\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[_` \x82\x84\x03\x12\x15a%$W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a!\x84W_\x80\xFD[` \x80\x82R`=\x90\x82\x01R\x7FCertified: You are not authorise`@\x82\x01R\x7Fd to transact using Nightfall\0\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`4\x90\x82\x01R\x7FCertified: You are on the Chaina`@\x82\x01Rs\x1B\x1E\\\xDA\\\xC8\x1C\xD8[\x98\xDD\x1A[\xDB\x9C\xC8\x1B\x1A\\\xDD`b\x1B``\x82\x01R`\x80\x01\x90V[_` \x82\x84\x03\x12\x15a%\xF4W_\x80\xFD[\x81Qa!\x84\x81a \x9AV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a&(W_\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&BW_\x80\xFD[` \x01\x91Pa\x01\xA0\x81\x026\x03\x82\x13\x15a \x93W_\x80\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a&\x9CWcNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@R\x90V[_\x82`\x1F\x83\x01\x12a&\xB1W_\x80\xFD[a&\xB9a&mV[\x80`\x80\x84\x01\x85\x81\x11\x15a&\xCAW_\x80\xFD[\x84[\x81\x81\x10\x15a&\xE4W\x805\x84R` \x93\x84\x01\x93\x01a&\xCCV[P\x90\x95\x94PPPPPV[_a\x01\xA0\x82\x84\x03\x12\x15a'\0W_\x80\xFD[a'\x08a&mV[\x825\x81Ra'\x19\x84` \x85\x01a&\xA2V[` \x82\x01Ra'+\x84`\xA0\x85\x01a&\xA2V[`@\x82\x01Ra'>\x84a\x01 \x85\x01a&\xA2V[``\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x03\x88Wa\x03\x88a'JV[_`\x01`\x01`\xFF\x1B\x01\x82\x01a'\x88Wa'\x88a'JV[P`\x01\x01\x90V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a'\xB3W_\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x92\x90\x93\x16` \x83\x01R`@\x82\x01R``\x81\x01\x91\x90\x91R`\xA0`\x80\x82\x01\x81\x90R_\x90\x82\x01R`\xC0\x01\x90V[` \x80\x82R`)\x90\x82\x01R\x7FERC721 tokens should have a valu`@\x82\x01Rhe of zero`\xB8\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`'\x90\x82\x01R\x7FERC20 tokens should have a token`@\x82\x01Rf\x04\x96B\x06\xF6b\x03`\xCC\x1B``\x82\x01R`\x80\x01\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x88Wa\x03\x88a'JV[\x81\x81\x03\x81\x81\x11\x15a\x03\x88Wa\x03\x88a'JV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a(\xC0W_\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(\xDAW_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a \x93W_\x80\xFD[_\x80\x85\x85\x11\x15a(\xFCW_\x80\xFD[\x83\x86\x11\x15a)\x08W_\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[_` \x82\x84\x03\x12\x15a)%W_\x80\xFD[P5\x91\x90PV[`@\x81R_\x83Q\x80`@\x84\x01R_[\x81\x81\x10\x15a)XW` \x81\x87\x01\x81\x01Q``\x86\x84\x01\x01R\x01a);V[P_``\x82\x85\x01\x01R`\x1F\x19`\x1F\x82\x01\x16\x83\x01\x90P``\x81\x01` ``\x85\x84\x03\x01` \x86\x01R\x81\x86Q\x80\x84R`\x80\x85\x01\x91P` \x88\x01\x94P_\x93P[\x80\x84\x10\x15a)\xB4W\x84Q\x82R\x93\x82\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a)\x94V[P\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \xEB \xA6ql\xE9\xA9\xEB{n7\x13\x93\xE6\xE8\xEA\xB3\xCC\0<\xF1\xD3\xD9\\\xB2\xFD\xB9: \xAD`\xDEdsolcC\0\x08\x18\x003";
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
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
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
    pub struct WithdrawProcessedReturn(pub u8);
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
