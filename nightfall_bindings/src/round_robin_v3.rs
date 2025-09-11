pub use round_robin_v3::*;
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
pub mod round_robin_v3 {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("COOLDOWN_BLOCKS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("COOLDOWN_BLOCKS"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("DING"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("DING"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("EXIT_PENALTY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("EXIT_PENALTY"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("FINALIZATION_BLOCKS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "FINALIZATION_BLOCKS",
                            ),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("ROTATION_BLOCKS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ROTATION_BLOCKS"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("STAKE"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("STAKE"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("add_proposer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("add_proposer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proposer_url"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("bootstrapDefaultProposer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "bootstrapDefaultProposer",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "default_proposer_address",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "default_proposer_url",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nightfall_address"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("escrow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("escrow"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("get_current_proposer_address"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "get_current_proposer_address",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("get_proposers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("get_proposers"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::String,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Proposer[]"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ding"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exit_penalty"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("cooling_blocks"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rotation_blocks"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("last_exit_block"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("last_exit_block"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("pending_withdraws"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pending_withdraws"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("proposer_count"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposer_count"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("proposer_urls"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposer_urls"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                (
                    ::std::borrow::ToOwned::to_owned("proposers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposers"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("stake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("addr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("url"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("next_addr"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("previous_addr"),
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
                    ::std::borrow::ToOwned::to_owned("remove_proposer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("remove_proposer"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rotate_proposer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rotate_proposer"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("setStakeRequirement"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "setStakeRequirement",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newStake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("set_nightfall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("set_nightfall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nightfall_address"),
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
                    ::std::borrow::ToOwned::to_owned("start_l1_block"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("start_l1_block"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("start_l2_block"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("start_l2_block"),
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
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
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
                    ::std::borrow::ToOwned::to_owned("version"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("version"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("withdraw"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AuthoritiesUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AuthoritiesUpdated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sanctionsList"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("x509"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
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
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProposerRotated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProposerRotated"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("proposer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("StakeRequirementUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StakeRequirementUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldStake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newStake"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "effectiveFromL1Block",
                                    ),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ROUNDROBINV3_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R0`\x80R4\x80\x15a\0\x13W_\x80\xFD[P`\x80Qa2\x8Ea\0:_9_\x81\x81a\"\x18\x01R\x81\x81a\"A\x01Ra#\xA5\x01Ra2\x8E_\xF3\xFE`\x80`@R`\x046\x10a\x01\xE6W_5`\xE0\x1C\x80cu\x0F%\xF4\x11a\x01\x08W\x80c\xC1/fn\x11a\0\x9DW\x80c\xD7\x10\xED\x93\x11a\0mW\x80c\xD7\x10\xED\x93\x14a\x05\x7FW\x80c\xD7\xAEv\xB6\x14a\x05\x94W\x80c\xE2\xFD\xCC\x17\x14a\x05\xA9W\x80c\xED\xBFJ\xC2\x14a\x05\xBEW\x80c\xF2\xFD\xE3\x8B\x14a\x05\xDDW_\x80\xFD[\x80c\xC1/fn\x14a\x04\xD7W\x80c\xC79\xD7\x9C\x14a\x04\xEBW\x80c\xD1\xCE\xF1\xEE\x14a\x05\x16W\x80c\xD5\xE6\xC6\xF9\x14a\x05`W_\x80\xFD[\x80c\xA8@J\xEE\x11a\0\xD8W\x80c\xA8@J\xEE\x14a\x04iW\x80c\xAC+\xEC\xA0\x14a\x04~W\x80c\xAD<\xB1\xCC\x14a\x04\x93W\x80c\xB6\xAC\x99)\x14a\x04\xC3W_\x80\xFD[\x80cu\x0F%\xF4\x14a\x03\xE7W\x80c\x7F\x11\xF2f\x14a\x04\x18W\x80c\x8D\xA5\xCB[\x14a\x04+W\x80c\x97\xE2\r\x0E\x14a\x04JW_\x80\xFD[\x80cJ\x8A@s\x11a\x01~W\x80cR\xD1\x90-\x11a\x01NW\x80cR\xD1\x90-\x14a\x03]W\x80cT\x83\x13T\x14a\x03qW\x80cT\xFDMP\x14a\x03\x86W\x80cU\xC2]\x17\x14a\x03\xD4W_\x80\xFD[\x80cJ\x8A@s\x14a\x02\xF8W\x80cK\x98\xB3\xB6\x14a\x03\x17W\x80cO\t\xE3\x04\x14a\x03+W\x80cO\x1E\xF2\x86\x14a\x03JW_\x80\xFD[\x80c\x1F\xFF\xF6\x98\x11a\x01\xB9W\x80c\x1F\xFF\xF6\x98\x14a\x02xW\x80c\"\xB0\x92\xAF\x14a\x02\x8DW\x80c.\x1A}M\x14a\x02\xB8W\x80cH\xC0\xF4\x87\x14a\x02\xD7W_\x80\xFD[\x80c\n\xCFO\x93\x14a\x01\xEAW\x80c\x12_\xDB\xBC\x14a\x02\x12W\x80c\x18\x17t\x97\x14a\x02'W\x80c\x19Okd\x14a\x02WW[_\x80\xFD[4\x80\x15a\x01\xF5W_\x80\xFD[Pa\x01\xFF`7T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x1DW_\x80\xFD[Pa\x01\xFF`5T\x81V[4\x80\x15a\x022W_\x80\xFD[Pa\x02Fa\x02A6`\x04a(\xECV[a\x05\xFCV[`@Qa\x02\t\x95\x94\x93\x92\x91\x90a)RV[4\x80\x15a\x02bW_\x80\xFD[Pa\x02va\x02q6`\x04a(\xECV[a\x06\xC7V[\0[4\x80\x15a\x02\x83W_\x80\xFD[Pa\x01\xFF`ET\x81V[4\x80\x15a\x02\x98W_\x80\xFD[Pa\x01\xFFa\x02\xA76`\x04a(\xECV[`=` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x02\xC3W_\x80\xFD[Pa\x02va\x02\xD26`\x04a)\x93V[a\x07\x1CV[4\x80\x15a\x02\xE2W_\x80\xFD[Pa\x02\xEBa\x08\x01V[`@Qa\x02\t\x91\x90a)\xAAV[4\x80\x15a\x03\x03W_\x80\xFD[Pa\x02va\x03\x126`\x04a*QV[a\n\xE8V[4\x80\x15a\x03\"W_\x80\xFD[Pa\x02va\x0BmV[4\x80\x15a\x036W_\x80\xFD[Pa\x02va\x03E6`\x04a)\x93V[a\x0BxV[a\x02va\x03X6`\x04a+\tV[a\x0C?V[4\x80\x15a\x03hW_\x80\xFD[Pa\x01\xFFa\x0C^V[4\x80\x15a\x03|W_\x80\xFD[Pa\x01\xFF`8T\x81V[4\x80\x15a\x03\x91W_\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x19\x81R\x7FRoundRobinV3-stake-config\0\0\0\0\0\0\0` \x82\x01R[`@Qa\x02\t\x91\x90a+gV[a\x02va\x03\xE26`\x04a+\xBEV[a\x0CyV[4\x80\x15a\x03\xF2W_\x80\xFD[P`?T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\tV[a\x02va\x04&6`\x04a+\xFDV[a\x12)V[4\x80\x15a\x046W_\x80\xFD[P`\x02Ta\x04\0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04UW_\x80\xFD[Pa\x02va\x04d6`\x04a(\xECV[a\x17\tV[4\x80\x15a\x04tW_\x80\xFD[Pa\x01\xFF`DT\x81V[4\x80\x15a\x04\x89W_\x80\xFD[Pa\x01\xFF`CT\x81V[4\x80\x15a\x04\x9EW_\x80\xFD[Pa\x03\xC7`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x04\xCEW_\x80\xFD[Pa\x01\xFF`@\x81V[4\x80\x15a\x04\xE2W_\x80\xFD[Pa\x02va\x17UV[4\x80\x15a\x04\xF6W_\x80\xFD[Pa\x01\xFFa\x05\x056`\x04a(\xECV[`;` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x05!W_\x80\xFD[Pa\x05Pa\x0506`\x04a,^V[\x80Q` \x81\x83\x01\x81\x01\x80Q`<\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\tV[4\x80\x15a\x05kW_\x80\xFD[Pa\x02va\x05z6`\x04a(\xECV[a\x19\x85V[4\x80\x15a\x05\x8AW_\x80\xFD[Pa\x01\xFF`9T\x81V[4\x80\x15a\x05\x9FW_\x80\xFD[Pa\x01\xFF`6T\x81V[4\x80\x15a\x05\xB4W_\x80\xFD[Pa\x01\xFF`FT\x81V[4\x80\x15a\x05\xC9W_\x80\xFD[Pa\x02va\x05\xD86`\x04a,\xABV[a\x19\xD0V[4\x80\x15a\x05\xE8W_\x80\xFD[Pa\x02va\x05\xF76`\x04a(\xECV[a\x1C\tV[`:` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01\x80T\x92\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92a\x06-\x90a-\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06Y\x90a-\x04V[\x80\x15a\x06\xA4W\x80`\x1F\x10a\x06{Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xA4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x87W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPP`\x03\x83\x01T`\x04\x90\x93\x01T\x91\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x16\x90P\x85V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[`@Q\x80\x91\x03\x90\xFD[`G\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[3_\x90\x81R`;` R`@\x90 T\x80\x82\x11\x15a\x07\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FAmount exceeds balance. You are `D\x82\x01R\x7Ftrying to withdraw more than you`d\x82\x01Rd have`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xF1V[3_\x90\x81R`;` R`@\x81 \x80T\x84\x92\x90a\x07\xCD\x90\x84\x90a-\x8CV[\x90\x91UPP`@Q3\x90\x83\x15a\x08\xFC\x02\x90\x84\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x07\xFCW=_\x80>=_\xFD[PPPV[``_`ETg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x1FWa\x08\x1Fa*\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08wW\x81` \x01[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R``\x93\x83\x01\x84\x90R\x92\x82\x01\x81\x90R`\x80\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x08=W\x90P[P`@\x80Q`\xA0\x81\x01\x82R`>\x80T\x82R`?T`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R\x82T\x93\x94P\x90\x92\x90\x91\x83\x81\x01\x91a\x08\xAF\x90a-\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xDB\x90a-\x04V[\x80\x15a\t&W\x80`\x1F\x10a\x08\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t&V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\tW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R`\x04\x90\x92\x01T\x90\x91\x16`@\x90\x91\x01R\x81Q\x82\x90_\x90a\tdWa\tda-\x9FV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01[`ET\x81\x10\x15a\n\xE2W`:_\x83a\t\x8B`\x01\x85a-\x8CV[\x81Q\x81\x10a\t\x9BWa\t\x9Ba-\x9FV[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x02\x82\x01\x80Ta\n\x18\x90a-\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\nD\x90a-\x04V[\x80\x15a\n\x8FW\x80`\x1F\x10a\nfWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x8FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\nrW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R`\x04\x90\x92\x01T\x90\x91\x16`@\x90\x91\x01R\x82Q\x83\x90\x83\x90\x81\x10a\n\xCFWa\n\xCFa-\x9FV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\trV[P\x91\x90PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x84U`\x01\x80T\x90\x93\x16\x90\x86\x16\x90\x81\x17\x90\x92U`@Q\x90\x92\x7F\x8F2\x04\xD4Wm\x86,F\xEA8\xD3\x93[\"\x89\xA03\x03\xFF=\x8B\x84\xD7\xF5\xA0\xA3\xDC\xF3r8R\x91\xA3PPV[a\x0Bv3a\x1C\xE4V[V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[`7T\x81\x10\x15a\x0B\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fnew stake < exit penalty\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[`5\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90RC\x81\x83\x01R\x90Q\x7F\x08]\x02a\x9A\xD1\x81\xF4Z\x04C\xE6\x83R\xF0\xBE\x01\xEE\xA6\xA1\xBB~\xE5\xB0\xD2\x0F\xE8\xAC\xB9\xA1\xC9\xCB\x91\x81\x90\x03``\x01\x90\xA1PPV[a\x0CGa\"\rV[a\x0CP\x82a\"\xB1V[a\x0CZ\x82\x82a\"\xDEV[PPV[_a\x0Cga#\x9AV[P_\x80Q` a2\x18\x839\x81Q\x91R\x90V[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xBEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE2\x91\x90a-\xB3V[a\r8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FCertified: not authorised by X50`D\x82\x01R`9`\xF8\x1B`d\x82\x01R`\x84\x01a\x06\xF1V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r~W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xA2\x91\x90a-\xB3V[\x15a\r\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCertified: address is sanctioned`D\x82\x01R`d\x01a\x06\xF1V[3_\x90\x81R`=` R`@\x90 T\x15a\x0EmW`8T3_\x90\x81R`=` R`@\x90 Ta\x0E\x1F\x91\x90a-\xD2V[C\x11a\x0EmW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCooldown period not met\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[`5T4\x14a\x0E\xE6W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FYou have not paid the correct st`D\x82\x01R\x7Faking amount during registration`d\x82\x01R`\x84\x01a\x06\xF1V[3_\x90\x81R`:` R`@\x90 `\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0FNW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FYou are already a proposer\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[`<\x82\x82`@Qa\x0F`\x92\x91\x90a-\xE5V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\xFF\x16\x15a\x0F\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FThis proposer URL is already in `D\x82\x01Rbuse`\xE8\x1B`d\x82\x01R`\x84\x01a\x06\xF1V[`5T`F_\x82\x82Ta\x0F\xDF\x91\x90a-\xD2V[\x90\x91UPP`?T`BT`AT`@\x80Q`\xA0\x81\x01\x82R`5T\x81R3` \x80\x83\x01\x91\x90\x91R\x82Q`\x1F\x88\x01\x82\x90\x04\x82\x02\x81\x01\x82\x01\x84R\x87\x81R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x96\x95\x86\x16\x95\x90\x94\x16\x93\x91\x92\x83\x01\x91\x90\x88\x90\x88\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x93\x85RPPP`\x01`\x01`\xA0\x1B\x03\x86\x81\x16` \x80\x85\x01\x91\x90\x91R\x86\x82\x16`@\x94\x85\x01R3\x83R`:\x81R\x91\x83\x90 \x84Q\x81U\x91\x84\x01Q`\x01\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U\x90\x82\x01Q`\x02\x82\x01\x90a\x10\xA9\x90\x82a.?V[P``\x82\x01Q`\x03\x82\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90\x91U`\x80\x90\x94\x01Q`\x04\x93\x84\x01\x80T\x86\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x86\x82\x16_\x81\x81R`:` R`@\x80\x82 \x90\x95\x01\x80T3\x90\x88\x16\x81\x17\x90\x91U\x88\x85\x16\x82R\x94\x90 \x90\x91\x01\x80T\x90\x94\x16\x90\x92\x17\x90\x92U\x90\x82\x16\x03a\x11QW`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`:` R`@\x90 `\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U[`\x01`<\x86\x86`@Qa\x11e\x92\x91\x90a-\xE5V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T`\xFF\x19\x16\x93\x15\x15\x93\x90\x93\x17\x90\x92U`\x01`\x01`\xA0\x1B\x03\x85\x81\x16_\x90\x81R`:\x90\x92R\x90\x82\x90 \x80T`>\x90\x81U`\x01\x82\x01T`?\x80T\x91\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90\x92U\x91a\x11\xD0`\x02\x84\x01\x82a.\xFFV[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x93\x90\x92\x01\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90U`E\x80T\x90_a\x12\x1D\x83a/\xCEV[\x91\x90PUPPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12SW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[a\x12t`@Q\x80``\x01`@R\x80`!\x81R` \x01a28`!\x919a#\xE3V[a\x12\xB3`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01\x7FdefaultProposerAddress: \0\0\0\0\0\0\0\0\x81RP\x85a$&V[a\x13\x1E`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x03#+3\x0B\xABc\xA2\x83\x93{\x83{\x9B+\x92\xAB\x93a\xD1`e\x1B\x81RP\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa$k\x92PPPV[a\x13R`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x03sK;C\xA33\x0Bcb\x0B##\x93+\x9B\x99\xD1`u\x1B\x81RP\x82a$&V[`ET\x15a\x13\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x10[\x1C\x99XY\x1EH\x18\x9B\xDB\xDD\x1C\xDD\x1C\x98\\\x1C\x19Y`b\x1B`D\x82\x01R`d\x01a\x06\xF1V[`5T4\x14a\x14\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FYou have not paid the correct st`D\x82\x01R\x7Faking amount during deployment\0\0`d\x82\x01R`\x84\x01a\x06\xF1V[`<\x83\x83`@Qa\x14\"\x92\x91\x90a-\xE5V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\xFF\x16\x15a\x14wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqURL already in use`p\x1B`D\x82\x01R`d\x01a\x06\xF1V[\x80`G_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`@Q\x80`\xA0\x01`@R\x80`5T\x81R` \x01\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP`\x01`\x01`\xA0\x1B\x03\x86\x81\x16` \x80\x84\x01\x82\x90R`@\x93\x84\x01\x91\x90\x91R\x83Q`>\x90\x81U\x90\x84\x01Q`?\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x93\x16\x17\x90\x91U\x82\x82\x01Q\x90\x91\x90a\x15E\x90\x82a.?V[P``\x82\x01Q`\x03\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x80\x90\x93\x01Q`\x04\x90\x92\x01\x80T\x90\x93\x16\x91\x16\x17\x90U`5T`F\x80T_\x90a\x15\x95\x90\x84\x90a-\xD2V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`:` R`@\x90\x81\x90 `>\x80T\x82U`?T`\x01\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x95\x16\x17\x90\x93U\x90`\x02\x82\x01\x90a\x15\xE5\x90\x82a.\xFFV[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x93\x90\x92\x01\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90U`@Q`\x01\x90`<\x90a\x169\x90\x86\x90\x86\x90a-\xE5V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x81 \x80T`\xFF\x19\x16\x94\x15\x15\x94\x90\x94\x17\x90\x93U`\x01`EUC`CU`GTc\x14a\xEB\xF3`\xE1\x1B\x84R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92c(\xC3\xD7\xE6\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x16\xA3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xC7\x91\x90a/\xE6V[`DU`@Q\x7F\xAC;\x1Ci)\xA3\xB1\xB5)\xEA\xCCFr\x0F\x90\xC2\xD5K\x83CE\xDC\xA9\xBA}z\x94\x83\xE0\xDER\xBE\x90a\x16\xFB\x90`>\x90a/\xFDV[`@Q\x80\x91\x03\x90\xA1PPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x173W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x17]a$\xB0V[a\x17\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FIt is not time to rotate the pro`D\x82\x01Rd87\xB9\xB2\xB9`\xD9\x1B`d\x82\x01R`\x84\x01a\x06\xF1V[`@`DTa\x17\xC6\x91\x90a0\xD2V[`G_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c(\xC3\xD7\xE6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x16W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18:\x91\x90a/\xE6V[\x03a\x18TW`?Ta\x18T\x90`\x01`\x01`\xA0\x1B\x03\x16a$\xD5V[`AT`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`:` R`@\x90\x81\x90 \x80T`>\x90\x81U`\x01\x82\x01T`?\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x95\x16\x17\x90\x93U\x91\x90a\x18\xA2`\x02\x84\x01\x82a.\xFFV[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x92\x84\x01\x80T\x90\x91\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91UC`CU`GT`@\x80Qc\x14a\xEB\xF3`\xE1\x1B\x81R\x90Q\x91\x90\x92\x16\x92c(\xC3\xD7\xE6\x92\x80\x82\x01\x92` \x92\x90\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x19#W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19G\x91\x90a/\xE6V[`DU`@Q\x7F\xAC;\x1Ci)\xA3\xB1\xB5)\xEA\xCCFr\x0F\x90\xC2\xD5K\x83CE\xDC\xA9\xBA}z\x94\x83\xE0\xDER\xBE\x90a\x19{\x90`>\x90a/\xFDV[`@Q\x80\x91\x03\x90\xA1V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x81\x15\x80\x15a\x1A\x15WP\x82[\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x1A1WP0;\x15[\x90P\x81\x15\x80\x15a\x1A?WP\x80\x15[\x15a\x1A]W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x1A\x87W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x1A\x8Fa%2V[a\x1A\x9A3\x8D\x8Da%:V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U\x86a\x1A\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FCooling blocks must be > 0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[\x87\x8A\x10\x15a\x1B@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsStake < exit penalty``\x1B`D\x82\x01R`d\x01a\x06\xF1V[\x87\x89\x11a\x1B\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsDing <= exit penalty``\x1B`D\x82\x01R`d\x01a\x06\xF1V[`5\x8A\x90U`6\x89\x90U`7\x88\x90U`8\x87\x90U`9\x86\x90Ua\x1B\xA8\x8Ca\x19\x85V[a\x1B\xB1\x8Ba\x17\tV[_`FU\x83\x15a\x1B\xFBW\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1C\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FCertified: new owner is zero\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[`\x02T`@Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x92\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x81\x81R`:` R`@\x90 `\x01\x01T\x90\x91\x16\x14a\x1DPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FProposer does not exist\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1D\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FThe proposer address cannot be t`D\x82\x01Rnhe zero address`\x88\x1B`d\x82\x01R`\x84\x01a\x06\xF1V[`?T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x03a\x1F\xFBW`\x01`ET\x11a\x1E9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FCannot deregister the only activ`D\x82\x01Ri2\x90897\xB87\xB9\xB2\xB9`\xB1\x1B`d\x82\x01R`\x84\x01a\x06\xF1V[`7T`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`:` R`@\x90 T\x10\x15a\x1E\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FInsufficient stake for exit\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[`7T`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`:` R`@\x81 \x80T\x90\x91\x90a\x1E\xCC\x90\x84\x90a-\x8CV[\x90\x91UPP`7T`F\x80T_\x90a\x1E\xE5\x90\x84\x90a-\x8CV[\x90\x91UPP`AT`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`:` R`@\x90\x81\x90 \x80T`>\x90\x81U`\x01\x82\x01T`?\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x95\x16\x17\x90\x93U\x91\x90a\x1F8`\x02\x84\x01\x82a.\xFFV[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x92\x84\x01\x80T\x90\x91\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91UC`CU`GT`@\x80Qc\x14a\xEB\xF3`\xE1\x1B\x81R\x90Q\x91\x90\x92\x16\x92c(\xC3\xD7\xE6\x92\x80\x82\x01\x92` \x92\x90\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1F\xB9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xDD\x91\x90a/\xE6V[`DU`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`=` R`@\x90 C\x90U[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`:` R`@\x80\x82 `\x03\x80\x82\x01\x80T\x86\x16\x85R\x83\x85 `\x04\x80\x85\x01T\x88\x16\x80\x88R\x95\x87 \x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x97\x17\x90U\x91T\x92\x82\x01\x80T\x90\x95\x16\x92\x90\x96\x16\x91\x90\x91\x17\x90\x92U\x80T`F\x80T\x92\x95\x94\x91\x92\x90\x91\x90a t\x90\x84\x90a-\x8CV[\x90\x91UPP\x82T`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`;` R`@\x81 \x80T\x90\x91\x90a \xA2\x90\x84\x90a-\xD2V[\x90\x91UPP`@Q`<\x90a \xBB\x90`\x02\x86\x01\x90a0\xF9V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T`\xFF\x19\x16\x90U`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`:\x90\x92R\x81 \x81\x81U`\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x90a!\x0C`\x02\x83\x01\x82a(\x87V[P`\x03\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x04\x90\x91\x01\x80T\x90\x91\x16\x90U`E\x80T\x90_a!=\x83a1kV[\x91\x90PUP`ET`\x01\x03a\"\x07W`?\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`:` R`@\x80\x82 `\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x94\x17\x90U\x84T\x84\x16\x80\x83R\x81\x83 `\x04\x01\x80T\x85\x16\x90\x91\x17\x90U\x84T\x80\x85\x16\x83R\x91\x81\x90 \x80T`>\x90\x81U`\x01\x82\x01T\x90\x95\x16\x92\x90\x93\x16\x91\x90\x91\x17\x90\x93U\x91a!\xC8`\x02\x84\x01\x82a.\xFFV[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x93\x90\x92\x01\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90U[PPPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\"\x93WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\"\x87_\x80Q` a2\x18\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0BvW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\"\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a#8WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra#5\x91\x81\x01\x90a/\xE6V[`\x01[a#`W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x06\xF1V[_\x80Q` a2\x18\x839\x81Q\x91R\x81\x14a#\x90W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x06\xF1V[a\x07\xFC\x83\x83a&CV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0BvW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\"\xDB\x81`@Q`$\x01a#\xF7\x91\x90a+gV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra&\x98V[a\x0CZ\x82\x82`@Q`$\x01a$<\x92\x91\x90a1\x80V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c1\x9A\xF33`\xE0\x1B\x17\x90Ra&\x98V[a\x0CZ\x82\x82`@Q`$\x01a$\x81\x92\x91\x90a1\xA9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90Ra&\x98V[_`@`9T`CTa$\xC3\x91\x90a-\xD2V[a$\xCD\x91\x90a-\xD2V[C\x10\x15\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`:` R`@\x81 `6T\x81T\x91\x92\x91a$\xFE\x91\x90a1\xD6V[\x90P_\x81\x12\x15a%\x11Wa\x07\xFC\x83a\x1C\xE4V[\x80\x82U`6T`F\x80T_\x90a%(\x90\x84\x90a-\x8CV[\x90\x91UPPPPPV[a\x0Bva&\xA1V[a%Ba&\xA1V[`\x01`\x01`\xA0\x1B\x03\x83\x16a%\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FCertified: owner is zero\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x81\x17\x90\x93U_\x80T\x86\x83\x16\x90\x84\x16\x17\x81U`\x01\x80T\x92\x86\x16\x92\x90\x93\x16\x91\x90\x91\x17\x90\x91U`@Q\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8F2\x04\xD4Wm\x86,F\xEA8\xD3\x93[\"\x89\xA03\x03\xFF=\x8B\x84\xD7\xF5\xA0\xA3\xDC\xF3r8R`@Q`@Q\x80\x91\x03\x90\xA3PPPV[a&L\x82a&\xEAV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a&\x90Wa\x07\xFC\x82\x82a'MV[a\x0CZa'\xC1V[a\"\xDB\x81a'\xE0V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a\x0BvW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a'\x1FW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x06\xF1V[_\x80Q` a2\x18\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa'i\x91\x90a1\xFCV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a'\xA1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a'\xA6V[``\x91P[P\x91P\x91Pa'\xB6\x85\x83\x83a'\xFFV[\x92PPP[\x92\x91PPV[4\x15a\x0BvW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_jconsole.log\x90P_\x80\x83Q` \x85\x01\x84Z\xFAPPPV[``\x82a(\x14Wa(\x0F\x82a(^V[a(WV[\x81Q\x15\x80\x15a(+WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a(TW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x06\xF1V[P\x80[\x93\x92PPPV[\x80Q\x15a(nW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Ta(\x93\x90a-\x04V[_\x82U\x80`\x1F\x10a(\xA2WPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\"\xDB\x91\x90[\x80\x82\x11\x15a(\xCDW_\x81U`\x01\x01a(\xBAV[P\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a(\xE7W_\x80\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a(\xFCW_\x80\xFD[a(W\x82a(\xD1V[_[\x83\x81\x10\x15a)\x1FW\x81\x81\x01Q\x83\x82\x01R` \x01a)\x07V[PP_\x91\x01RV[_\x81Q\x80\x84Ra)>\x81` \x86\x01` \x86\x01a)\x05V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x85\x81R_`\x01\x80`\xA0\x1B\x03\x80\x87\x16` \x84\x01R`\xA0`@\x84\x01Ra)y`\xA0\x84\x01\x87a)'V[\x94\x81\x16``\x84\x01R\x92\x90\x92\x16`\x80\x90\x91\x01RP\x93\x92PPPV[_` \x82\x84\x03\x12\x15a)\xA3W_\x80\xFD[P5\x91\x90PV[_` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x84\x88\x01_[\x83\x81\x10\x15a*CW\x88\x83\x03`?\x19\x01\x85R\x81Q\x80Q\x84R\x87\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x89\x86\x01R\x87\x82\x01Q`\xA0\x89\x87\x01\x81\x90R\x91a*\x14\x83\x88\x01\x83a)'V[``\x85\x81\x01Q\x83\x16\x90\x89\x01R`\x80\x94\x85\x01Q\x90\x91\x16\x93\x90\x96\x01\x92\x90\x92RPP\x93\x86\x01\x93\x90\x86\x01\x90`\x01\x01a)\xD1V[P\x90\x98\x97PPPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a*bW_\x80\xFD[a*k\x83a(\xD1V[\x91Pa*y` \x84\x01a(\xD1V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a*\xB0Wa*\xB0a*\x82V[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a*\xD8Wa*\xD8a*\x82V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a*\xF0W_\x80\xFD[\x85\x85` \x83\x017_` \x87\x83\x01\x01RPPP\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a+\x1AW_\x80\xFD[a+#\x83a(\xD1V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+>W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a+NW_\x80\xFD[a+]\x85\x825` \x84\x01a*\x96V[\x91PP\x92P\x92\x90PV[` \x81R_a(W` \x83\x01\x84a)'V[_\x80\x83`\x1F\x84\x01\x12a+\x89W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xA0W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a+\xB7W_\x80\xFD[\x92P\x92\x90PV[_\x80` \x83\x85\x03\x12\x15a+\xCFW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xE5W_\x80\xFD[a+\xF1\x85\x82\x86\x01a+yV[\x90\x96\x90\x95P\x93PPPPV[_\x80_\x80``\x85\x87\x03\x12\x15a,\x10W_\x80\xFD[a,\x19\x85a(\xD1V[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,4W_\x80\xFD[a,@\x87\x82\x88\x01a+yV[\x90\x94P\x92Pa,S\x90P`@\x86\x01a(\xD1V[\x90P\x92\x95\x91\x94P\x92PV[_` \x82\x84\x03\x12\x15a,nW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\x84W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a,\x94W_\x80\xFD[a,\xA3\x84\x825` \x84\x01a*\x96V[\x94\x93PPPPV[_\x80_\x80_\x80_`\xE0\x88\x8A\x03\x12\x15a,\xC1W_\x80\xFD[a,\xCA\x88a(\xD1V[\x96Pa,\xD8` \x89\x01a(\xD1V[\x96\x99\x96\x98PPPP`@\x85\x015\x94``\x81\x015\x94`\x80\x82\x015\x94P`\xA0\x82\x015\x93P`\xC0\x90\x91\x015\x91PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a-\x18W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\n\xE2WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[` \x80\x82R`\"\x90\x82\x01R\x7FCertified: caller is not the own`@\x82\x01Ra2\xB9`\xF1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a'\xBBWa'\xBBa-xV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a-\xC3W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a(WW_\x80\xFD[\x80\x82\x01\x80\x82\x11\x15a'\xBBWa'\xBBa-xV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[`\x1F\x82\x11\x15a\x07\xFCW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a.\x19WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a.8W_\x81U`\x01\x01a.%V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.YWa.Ya*\x82V[a.m\x81a.g\x84Ta-\x04V[\x84a-\xF4V[` \x80`\x1F\x83\x11`\x01\x81\x14a.\xA0W_\x84\x15a.\x89WP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua.\xF7V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a.\xCEW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a.\xAFV[P\x85\x82\x10\x15a.\xEBW\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV[\x81\x81\x03a/\nWPPV[a/\x14\x82Ta-\x04V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/,Wa/,a*\x82V[a/:\x81a.g\x84Ta-\x04V[_`\x1F\x82\x11`\x01\x81\x14a/kW_\x83\x15a/TWP\x84\x82\x01T[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua.8V[_\x85\x81R` \x80\x82 \x86\x83R\x90\x82 `\x1F\x19\x86\x16\x92[\x83\x81\x10\x15a/\xA1W\x82\x86\x01T\x82U`\x01\x95\x86\x01\x95\x90\x91\x01\x90` \x01a/\x81V[P\x85\x83\x10\x15a/\xBEW\x81\x85\x01T_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[_`\x01\x82\x01a/\xDFWa/\xDFa-xV[P`\x01\x01\x90V[_` \x82\x84\x03\x12\x15a/\xF6W_\x80\xFD[PQ\x91\x90PV[_` \x80\x83R\x83T` \x84\x01R`\x01\x80`\x01`\xA0\x1B\x03`\x01\x86\x01T\x16`@\x85\x01R`\x02\x85\x01`\xA0``\x86\x01R_\x81Ta05\x81a-\x04V[\x80`\xC0\x89\x01R`\xE0`\x01\x83\x16_\x81\x14a0UW`\x01\x81\x14a0qWa0\x9EV[`\xFF\x19\x84\x16`\xE0\x8B\x01R`\xE0\x83\x15\x15`\x05\x1B\x8B\x01\x01\x94Pa0\x9EV[\x85_R` _ _[\x84\x81\x10\x15a0\x95W\x81T\x8C\x82\x01\x85\x01R\x90\x88\x01\x90\x89\x01a0zV[\x8B\x01`\xE0\x01\x95PP[PPPP`\x03\x87\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x88\x01R`\x04\x90\x97\x01T\x90\x96\x16`\xA0\x90\x95\x01\x94\x90\x94RP\x92\x93\x92PPPV[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a0\xF1Wa0\xF1a-xV[PP\x92\x91PPV[_\x80\x83Ta1\x06\x81a-\x04V[`\x01\x82\x81\x16\x80\x15a1\x1EW`\x01\x81\x14a13Wa1_V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa1_V[\x87_R` \x80_ _[\x85\x81\x10\x15a1VW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a1=V[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[_\x81a1yWa1ya-xV[P_\x19\x01\x90V[`@\x81R_a1\x92`@\x83\x01\x85a)'V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`@\x81R_a1\xBB`@\x83\x01\x85a)'V[\x82\x81\x03` \x84\x01Ra1\xCD\x81\x85a)'V[\x95\x94PPPPPV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a1\xF5Wa1\xF5a-xV[P\x92\x91PPV[_\x82Qa2\r\x81\x84` \x87\x01a)\x05V[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCBootstrapping default proposer...\xA2dipfsX\"\x12 \xF8\0m\xDE\x8B_;\x87\xC4\xFC\x17\xB1:*\x10r\x13I;\xCF\xC6j\xD9\x04\xD4\xC3W*\x112\xF48dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static ROUNDROBINV3_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01\xE6W_5`\xE0\x1C\x80cu\x0F%\xF4\x11a\x01\x08W\x80c\xC1/fn\x11a\0\x9DW\x80c\xD7\x10\xED\x93\x11a\0mW\x80c\xD7\x10\xED\x93\x14a\x05\x7FW\x80c\xD7\xAEv\xB6\x14a\x05\x94W\x80c\xE2\xFD\xCC\x17\x14a\x05\xA9W\x80c\xED\xBFJ\xC2\x14a\x05\xBEW\x80c\xF2\xFD\xE3\x8B\x14a\x05\xDDW_\x80\xFD[\x80c\xC1/fn\x14a\x04\xD7W\x80c\xC79\xD7\x9C\x14a\x04\xEBW\x80c\xD1\xCE\xF1\xEE\x14a\x05\x16W\x80c\xD5\xE6\xC6\xF9\x14a\x05`W_\x80\xFD[\x80c\xA8@J\xEE\x11a\0\xD8W\x80c\xA8@J\xEE\x14a\x04iW\x80c\xAC+\xEC\xA0\x14a\x04~W\x80c\xAD<\xB1\xCC\x14a\x04\x93W\x80c\xB6\xAC\x99)\x14a\x04\xC3W_\x80\xFD[\x80cu\x0F%\xF4\x14a\x03\xE7W\x80c\x7F\x11\xF2f\x14a\x04\x18W\x80c\x8D\xA5\xCB[\x14a\x04+W\x80c\x97\xE2\r\x0E\x14a\x04JW_\x80\xFD[\x80cJ\x8A@s\x11a\x01~W\x80cR\xD1\x90-\x11a\x01NW\x80cR\xD1\x90-\x14a\x03]W\x80cT\x83\x13T\x14a\x03qW\x80cT\xFDMP\x14a\x03\x86W\x80cU\xC2]\x17\x14a\x03\xD4W_\x80\xFD[\x80cJ\x8A@s\x14a\x02\xF8W\x80cK\x98\xB3\xB6\x14a\x03\x17W\x80cO\t\xE3\x04\x14a\x03+W\x80cO\x1E\xF2\x86\x14a\x03JW_\x80\xFD[\x80c\x1F\xFF\xF6\x98\x11a\x01\xB9W\x80c\x1F\xFF\xF6\x98\x14a\x02xW\x80c\"\xB0\x92\xAF\x14a\x02\x8DW\x80c.\x1A}M\x14a\x02\xB8W\x80cH\xC0\xF4\x87\x14a\x02\xD7W_\x80\xFD[\x80c\n\xCFO\x93\x14a\x01\xEAW\x80c\x12_\xDB\xBC\x14a\x02\x12W\x80c\x18\x17t\x97\x14a\x02'W\x80c\x19Okd\x14a\x02WW[_\x80\xFD[4\x80\x15a\x01\xF5W_\x80\xFD[Pa\x01\xFF`7T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x1DW_\x80\xFD[Pa\x01\xFF`5T\x81V[4\x80\x15a\x022W_\x80\xFD[Pa\x02Fa\x02A6`\x04a(\xECV[a\x05\xFCV[`@Qa\x02\t\x95\x94\x93\x92\x91\x90a)RV[4\x80\x15a\x02bW_\x80\xFD[Pa\x02va\x02q6`\x04a(\xECV[a\x06\xC7V[\0[4\x80\x15a\x02\x83W_\x80\xFD[Pa\x01\xFF`ET\x81V[4\x80\x15a\x02\x98W_\x80\xFD[Pa\x01\xFFa\x02\xA76`\x04a(\xECV[`=` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x02\xC3W_\x80\xFD[Pa\x02va\x02\xD26`\x04a)\x93V[a\x07\x1CV[4\x80\x15a\x02\xE2W_\x80\xFD[Pa\x02\xEBa\x08\x01V[`@Qa\x02\t\x91\x90a)\xAAV[4\x80\x15a\x03\x03W_\x80\xFD[Pa\x02va\x03\x126`\x04a*QV[a\n\xE8V[4\x80\x15a\x03\"W_\x80\xFD[Pa\x02va\x0BmV[4\x80\x15a\x036W_\x80\xFD[Pa\x02va\x03E6`\x04a)\x93V[a\x0BxV[a\x02va\x03X6`\x04a+\tV[a\x0C?V[4\x80\x15a\x03hW_\x80\xFD[Pa\x01\xFFa\x0C^V[4\x80\x15a\x03|W_\x80\xFD[Pa\x01\xFF`8T\x81V[4\x80\x15a\x03\x91W_\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x19\x81R\x7FRoundRobinV3-stake-config\0\0\0\0\0\0\0` \x82\x01R[`@Qa\x02\t\x91\x90a+gV[a\x02va\x03\xE26`\x04a+\xBEV[a\x0CyV[4\x80\x15a\x03\xF2W_\x80\xFD[P`?T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\tV[a\x02va\x04&6`\x04a+\xFDV[a\x12)V[4\x80\x15a\x046W_\x80\xFD[P`\x02Ta\x04\0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04UW_\x80\xFD[Pa\x02va\x04d6`\x04a(\xECV[a\x17\tV[4\x80\x15a\x04tW_\x80\xFD[Pa\x01\xFF`DT\x81V[4\x80\x15a\x04\x89W_\x80\xFD[Pa\x01\xFF`CT\x81V[4\x80\x15a\x04\x9EW_\x80\xFD[Pa\x03\xC7`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[4\x80\x15a\x04\xCEW_\x80\xFD[Pa\x01\xFF`@\x81V[4\x80\x15a\x04\xE2W_\x80\xFD[Pa\x02va\x17UV[4\x80\x15a\x04\xF6W_\x80\xFD[Pa\x01\xFFa\x05\x056`\x04a(\xECV[`;` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x05!W_\x80\xFD[Pa\x05Pa\x0506`\x04a,^V[\x80Q` \x81\x83\x01\x81\x01\x80Q`<\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\tV[4\x80\x15a\x05kW_\x80\xFD[Pa\x02va\x05z6`\x04a(\xECV[a\x19\x85V[4\x80\x15a\x05\x8AW_\x80\xFD[Pa\x01\xFF`9T\x81V[4\x80\x15a\x05\x9FW_\x80\xFD[Pa\x01\xFF`6T\x81V[4\x80\x15a\x05\xB4W_\x80\xFD[Pa\x01\xFF`FT\x81V[4\x80\x15a\x05\xC9W_\x80\xFD[Pa\x02va\x05\xD86`\x04a,\xABV[a\x19\xD0V[4\x80\x15a\x05\xE8W_\x80\xFD[Pa\x02va\x05\xF76`\x04a(\xECV[a\x1C\tV[`:` R_\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01\x80T\x92\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92a\x06-\x90a-\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06Y\x90a-\x04V[\x80\x15a\x06\xA4W\x80`\x1F\x10a\x06{Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xA4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x87W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPP`\x03\x83\x01T`\x04\x90\x93\x01T\x91\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x16\x90P\x85V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x06\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[`@Q\x80\x91\x03\x90\xFD[`G\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[3_\x90\x81R`;` R`@\x90 T\x80\x82\x11\x15a\x07\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FAmount exceeds balance. You are `D\x82\x01R\x7Ftrying to withdraw more than you`d\x82\x01Rd have`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xF1V[3_\x90\x81R`;` R`@\x81 \x80T\x84\x92\x90a\x07\xCD\x90\x84\x90a-\x8CV[\x90\x91UPP`@Q3\x90\x83\x15a\x08\xFC\x02\x90\x84\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x07\xFCW=_\x80>=_\xFD[PPPV[``_`ETg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\x1FWa\x08\x1Fa*\x82V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08wW\x81` \x01[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R``\x93\x83\x01\x84\x90R\x92\x82\x01\x81\x90R`\x80\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x08=W\x90P[P`@\x80Q`\xA0\x81\x01\x82R`>\x80T\x82R`?T`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R\x82T\x93\x94P\x90\x92\x90\x91\x83\x81\x01\x91a\x08\xAF\x90a-\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xDB\x90a-\x04V[\x80\x15a\t&W\x80`\x1F\x10a\x08\xFDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t&V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\t\tW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R`\x04\x90\x92\x01T\x90\x91\x16`@\x90\x91\x01R\x81Q\x82\x90_\x90a\tdWa\tda-\x9FV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01[`ET\x81\x10\x15a\n\xE2W`:_\x83a\t\x8B`\x01\x85a-\x8CV[\x81Q\x81\x10a\t\x9BWa\t\x9Ba-\x9FV[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x02\x82\x01\x80Ta\n\x18\x90a-\x04V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\nD\x90a-\x04V[\x80\x15a\n\x8FW\x80`\x1F\x10a\nfWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n\x8FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\nrW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R`\x04\x90\x92\x01T\x90\x91\x16`@\x90\x91\x01R\x82Q\x83\x90\x83\x90\x81\x10a\n\xCFWa\n\xCFa-\x9FV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\trV[P\x91\x90PV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x84U`\x01\x80T\x90\x93\x16\x90\x86\x16\x90\x81\x17\x90\x92U`@Q\x90\x92\x7F\x8F2\x04\xD4Wm\x86,F\xEA8\xD3\x93[\"\x89\xA03\x03\xFF=\x8B\x84\xD7\xF5\xA0\xA3\xDC\xF3r8R\x91\xA3PPV[a\x0Bv3a\x1C\xE4V[V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0B\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[`7T\x81\x10\x15a\x0B\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Fnew stake < exit penalty\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[`5\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90RC\x81\x83\x01R\x90Q\x7F\x08]\x02a\x9A\xD1\x81\xF4Z\x04C\xE6\x83R\xF0\xBE\x01\xEE\xA6\xA1\xBB~\xE5\xB0\xD2\x0F\xE8\xAC\xB9\xA1\xC9\xCB\x91\x81\x90\x03``\x01\x90\xA1PPV[a\x0CGa\"\rV[a\x0CP\x82a\"\xB1V[a\x0CZ\x82\x82a\"\xDEV[PPV[_a\x0Cga#\x9AV[P_\x80Q` a2\x18\x839\x81Q\x91R\x90V[_T`@Qc\xE2<'\xE9`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xE2<'\xE9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xBEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE2\x91\x90a-\xB3V[a\r8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FCertified: not authorised by X50`D\x82\x01R`9`\xF8\x1B`d\x82\x01R`\x84\x01a\x06\xF1V[`\x01T`@Qc\xDFY/}`\xE0\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDFY/}\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r~W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xA2\x91\x90a-\xB3V[\x15a\r\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCertified: address is sanctioned`D\x82\x01R`d\x01a\x06\xF1V[3_\x90\x81R`=` R`@\x90 T\x15a\x0EmW`8T3_\x90\x81R`=` R`@\x90 Ta\x0E\x1F\x91\x90a-\xD2V[C\x11a\x0EmW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FCooldown period not met\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[`5T4\x14a\x0E\xE6W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FYou have not paid the correct st`D\x82\x01R\x7Faking amount during registration`d\x82\x01R`\x84\x01a\x06\xF1V[3_\x90\x81R`:` R`@\x90 `\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\x0FNW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FYou are already a proposer\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[`<\x82\x82`@Qa\x0F`\x92\x91\x90a-\xE5V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\xFF\x16\x15a\x0F\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FThis proposer URL is already in `D\x82\x01Rbuse`\xE8\x1B`d\x82\x01R`\x84\x01a\x06\xF1V[`5T`F_\x82\x82Ta\x0F\xDF\x91\x90a-\xD2V[\x90\x91UPP`?T`BT`AT`@\x80Q`\xA0\x81\x01\x82R`5T\x81R3` \x80\x83\x01\x91\x90\x91R\x82Q`\x1F\x88\x01\x82\x90\x04\x82\x02\x81\x01\x82\x01\x84R\x87\x81R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x96\x95\x86\x16\x95\x90\x94\x16\x93\x91\x92\x83\x01\x91\x90\x88\x90\x88\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x93\x85RPPP`\x01`\x01`\xA0\x1B\x03\x86\x81\x16` \x80\x85\x01\x91\x90\x91R\x86\x82\x16`@\x94\x85\x01R3\x83R`:\x81R\x91\x83\x90 \x84Q\x81U\x91\x84\x01Q`\x01\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U\x90\x82\x01Q`\x02\x82\x01\x90a\x10\xA9\x90\x82a.?V[P``\x82\x01Q`\x03\x82\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90\x91U`\x80\x90\x94\x01Q`\x04\x93\x84\x01\x80T\x86\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x86\x82\x16_\x81\x81R`:` R`@\x80\x82 \x90\x95\x01\x80T3\x90\x88\x16\x81\x17\x90\x91U\x88\x85\x16\x82R\x94\x90 \x90\x91\x01\x80T\x90\x94\x16\x90\x92\x17\x90\x92U\x90\x82\x16\x03a\x11QW`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`:` R`@\x90 `\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U[`\x01`<\x86\x86`@Qa\x11e\x92\x91\x90a-\xE5V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T`\xFF\x19\x16\x93\x15\x15\x93\x90\x93\x17\x90\x92U`\x01`\x01`\xA0\x1B\x03\x85\x81\x16_\x90\x81R`:\x90\x92R\x90\x82\x90 \x80T`>\x90\x81U`\x01\x82\x01T`?\x80T\x91\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90\x92U\x91a\x11\xD0`\x02\x84\x01\x82a.\xFFV[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x93\x90\x92\x01\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90U`E\x80T\x90_a\x12\x1D\x83a/\xCEV[\x91\x90PUPPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12SW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[a\x12t`@Q\x80``\x01`@R\x80`!\x81R` \x01a28`!\x919a#\xE3V[a\x12\xB3`@Q\x80`@\x01`@R\x80`\x18\x81R` \x01\x7FdefaultProposerAddress: \0\0\0\0\0\0\0\0\x81RP\x85a$&V[a\x13\x1E`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x03#+3\x0B\xABc\xA2\x83\x93{\x83{\x9B+\x92\xAB\x93a\xD1`e\x1B\x81RP\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPa$k\x92PPPV[a\x13R`@Q\x80`@\x01`@R\x80`\x12\x81R` \x01q\x03sK;C\xA33\x0Bcb\x0B##\x93+\x9B\x99\xD1`u\x1B\x81RP\x82a$&V[`ET\x15a\x13\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x10[\x1C\x99XY\x1EH\x18\x9B\xDB\xDD\x1C\xDD\x1C\x98\\\x1C\x19Y`b\x1B`D\x82\x01R`d\x01a\x06\xF1V[`5T4\x14a\x14\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R\x7FYou have not paid the correct st`D\x82\x01R\x7Faking amount during deployment\0\0`d\x82\x01R`\x84\x01a\x06\xF1V[`<\x83\x83`@Qa\x14\"\x92\x91\x90a-\xE5V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\xFF\x16\x15a\x14wW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x12`$\x82\x01RqURL already in use`p\x1B`D\x82\x01R`d\x01a\x06\xF1V[\x80`G_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`@Q\x80`\xA0\x01`@R\x80`5T\x81R` \x01\x85`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP`\x01`\x01`\xA0\x1B\x03\x86\x81\x16` \x80\x84\x01\x82\x90R`@\x93\x84\x01\x91\x90\x91R\x83Q`>\x90\x81U\x90\x84\x01Q`?\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x93\x16\x17\x90\x91U\x82\x82\x01Q\x90\x91\x90a\x15E\x90\x82a.?V[P``\x82\x01Q`\x03\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x80\x90\x93\x01Q`\x04\x90\x92\x01\x80T\x90\x93\x16\x91\x16\x17\x90U`5T`F\x80T_\x90a\x15\x95\x90\x84\x90a-\xD2V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`:` R`@\x90\x81\x90 `>\x80T\x82U`?T`\x01\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x95\x16\x17\x90\x93U\x90`\x02\x82\x01\x90a\x15\xE5\x90\x82a.\xFFV[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x93\x90\x92\x01\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90U`@Q`\x01\x90`<\x90a\x169\x90\x86\x90\x86\x90a-\xE5V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x81 \x80T`\xFF\x19\x16\x94\x15\x15\x94\x90\x94\x17\x90\x93U`\x01`EUC`CU`GTc\x14a\xEB\xF3`\xE1\x1B\x84R\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92c(\xC3\xD7\xE6\x92`\x04\x80\x83\x01\x93\x91\x92\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x16\xA3W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xC7\x91\x90a/\xE6V[`DU`@Q\x7F\xAC;\x1Ci)\xA3\xB1\xB5)\xEA\xCCFr\x0F\x90\xC2\xD5K\x83CE\xDC\xA9\xBA}z\x94\x83\xE0\xDER\xBE\x90a\x16\xFB\x90`>\x90a/\xFDV[`@Q\x80\x91\x03\x90\xA1PPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x173W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[`\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x17]a$\xB0V[a\x17\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FIt is not time to rotate the pro`D\x82\x01Rd87\xB9\xB2\xB9`\xD9\x1B`d\x82\x01R`\x84\x01a\x06\xF1V[`@`DTa\x17\xC6\x91\x90a0\xD2V[`G_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c(\xC3\xD7\xE6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x16W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18:\x91\x90a/\xE6V[\x03a\x18TW`?Ta\x18T\x90`\x01`\x01`\xA0\x1B\x03\x16a$\xD5V[`AT`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`:` R`@\x90\x81\x90 \x80T`>\x90\x81U`\x01\x82\x01T`?\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x95\x16\x17\x90\x93U\x91\x90a\x18\xA2`\x02\x84\x01\x82a.\xFFV[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x92\x84\x01\x80T\x90\x91\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91UC`CU`GT`@\x80Qc\x14a\xEB\xF3`\xE1\x1B\x81R\x90Q\x91\x90\x92\x16\x92c(\xC3\xD7\xE6\x92\x80\x82\x01\x92` \x92\x90\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x19#W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19G\x91\x90a/\xE6V[`DU`@Q\x7F\xAC;\x1Ci)\xA3\xB1\xB5)\xEA\xCCFr\x0F\x90\xC2\xD5K\x83CE\xDC\xA9\xBA}z\x94\x83\xE0\xDER\xBE\x90a\x19{\x90`>\x90a/\xFDV[`@Q\x80\x91\x03\x90\xA1V[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x19\xAFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x81\x15\x80\x15a\x1A\x15WP\x82[\x90P_\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x14\x80\x15a\x1A1WP0;\x15[\x90P\x81\x15\x80\x15a\x1A?WP\x80\x15[\x15a\x1A]W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x1A\x87W\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[a\x1A\x8Fa%2V[a\x1A\x9A3\x8D\x8Da%:V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U\x86a\x1A\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FCooling blocks must be > 0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[\x87\x8A\x10\x15a\x1B@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsStake < exit penalty``\x1B`D\x82\x01R`d\x01a\x06\xF1V[\x87\x89\x11a\x1B\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01RsDing <= exit penalty``\x1B`D\x82\x01R`d\x01a\x06\xF1V[`5\x8A\x90U`6\x89\x90U`7\x88\x90U`8\x87\x90U`9\x86\x90Ua\x1B\xA8\x8Ca\x19\x85V[a\x1B\xB1\x8Ba\x17\tV[_`FU\x83\x15a\x1B\xFBW\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPPPPPPV[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1C\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FCertified: new owner is zero\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[`\x02T`@Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x92\x16\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x81\x81R`:` R`@\x90 `\x01\x01T\x90\x91\x16\x14a\x1DPW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FProposer does not exist\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1D\xBEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FThe proposer address cannot be t`D\x82\x01Rnhe zero address`\x88\x1B`d\x82\x01R`\x84\x01a\x06\xF1V[`?T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x03a\x1F\xFBW`\x01`ET\x11a\x1E9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FCannot deregister the only activ`D\x82\x01Ri2\x90897\xB87\xB9\xB2\xB9`\xB1\x1B`d\x82\x01R`\x84\x01a\x06\xF1V[`7T`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`:` R`@\x90 T\x10\x15a\x1E\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7FInsufficient stake for exit\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[`7T`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`:` R`@\x81 \x80T\x90\x91\x90a\x1E\xCC\x90\x84\x90a-\x8CV[\x90\x91UPP`7T`F\x80T_\x90a\x1E\xE5\x90\x84\x90a-\x8CV[\x90\x91UPP`AT`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`:` R`@\x90\x81\x90 \x80T`>\x90\x81U`\x01\x82\x01T`?\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x95\x16\x17\x90\x93U\x91\x90a\x1F8`\x02\x84\x01\x82a.\xFFV[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x92\x84\x01\x80T\x90\x91\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91UC`CU`GT`@\x80Qc\x14a\xEB\xF3`\xE1\x1B\x81R\x90Q\x91\x90\x92\x16\x92c(\xC3\xD7\xE6\x92\x80\x82\x01\x92` \x92\x90\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x1F\xB9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\xDD\x91\x90a/\xE6V[`DU`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`=` R`@\x90 C\x90U[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`:` R`@\x80\x82 `\x03\x80\x82\x01\x80T\x86\x16\x85R\x83\x85 `\x04\x80\x85\x01T\x88\x16\x80\x88R\x95\x87 \x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x97\x17\x90U\x91T\x92\x82\x01\x80T\x90\x95\x16\x92\x90\x96\x16\x91\x90\x91\x17\x90\x92U\x80T`F\x80T\x92\x95\x94\x91\x92\x90\x91\x90a t\x90\x84\x90a-\x8CV[\x90\x91UPP\x82T`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`;` R`@\x81 \x80T\x90\x91\x90a \xA2\x90\x84\x90a-\xD2V[\x90\x91UPP`@Q`<\x90a \xBB\x90`\x02\x86\x01\x90a0\xF9V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T`\xFF\x19\x16\x90U`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`:\x90\x92R\x81 \x81\x81U`\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x90a!\x0C`\x02\x83\x01\x82a(\x87V[P`\x03\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x04\x90\x91\x01\x80T\x90\x91\x16\x90U`E\x80T\x90_a!=\x83a1kV[\x91\x90PUP`ET`\x01\x03a\"\x07W`?\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`:` R`@\x80\x82 `\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x94\x17\x90U\x84T\x84\x16\x80\x83R\x81\x83 `\x04\x01\x80T\x85\x16\x90\x91\x17\x90U\x84T\x80\x85\x16\x83R\x91\x81\x90 \x80T`>\x90\x81U`\x01\x82\x01T\x90\x95\x16\x92\x90\x93\x16\x91\x90\x91\x17\x90\x93U\x91a!\xC8`\x02\x84\x01\x82a.\xFFV[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x93\x90\x92\x01\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90U[PPPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\"\x93WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\"\x87_\x80Q` a2\x18\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x0BvW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02T`\x01`\x01`\xA0\x1B\x03\x163\x14a\"\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xF1\x90a-6V[PV[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a#8WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra#5\x91\x81\x01\x90a/\xE6V[`\x01[a#`W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x06\xF1V[_\x80Q` a2\x18\x839\x81Q\x91R\x81\x14a#\x90W`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x06\xF1V[a\x07\xFC\x83\x83a&CV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0BvW`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\"\xDB\x81`@Q`$\x01a#\xF7\x91\x90a+gV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90Ra&\x98V[a\x0CZ\x82\x82`@Q`$\x01a$<\x92\x91\x90a1\x80V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c1\x9A\xF33`\xE0\x1B\x17\x90Ra&\x98V[a\x0CZ\x82\x82`@Q`$\x01a$\x81\x92\x91\x90a1\xA9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16cK\\Bw`\xE0\x1B\x17\x90Ra&\x98V[_`@`9T`CTa$\xC3\x91\x90a-\xD2V[a$\xCD\x91\x90a-\xD2V[C\x10\x15\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`:` R`@\x81 `6T\x81T\x91\x92\x91a$\xFE\x91\x90a1\xD6V[\x90P_\x81\x12\x15a%\x11Wa\x07\xFC\x83a\x1C\xE4V[\x80\x82U`6T`F\x80T_\x90a%(\x90\x84\x90a-\x8CV[\x90\x91UPPPPPV[a\x0Bva&\xA1V[a%Ba&\xA1V[`\x01`\x01`\xA0\x1B\x03\x83\x16a%\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7FCertified: owner is zero\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06\xF1V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x81\x17\x90\x93U_\x80T\x86\x83\x16\x90\x84\x16\x17\x81U`\x01\x80T\x92\x86\x16\x92\x90\x93\x16\x91\x90\x91\x17\x90\x91U`@Q\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3\x81`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8F2\x04\xD4Wm\x86,F\xEA8\xD3\x93[\"\x89\xA03\x03\xFF=\x8B\x84\xD7\xF5\xA0\xA3\xDC\xF3r8R`@Q`@Q\x80\x91\x03\x90\xA3PPPV[a&L\x82a&\xEAV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a&\x90Wa\x07\xFC\x82\x82a'MV[a\x0CZa'\xC1V[a\"\xDB\x81a'\xE0V[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a\x0BvW`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a'\x1FW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x06\xF1V[_\x80Q` a2\x18\x839\x81Q\x91R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa'i\x91\x90a1\xFCV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a'\xA1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a'\xA6V[``\x91P[P\x91P\x91Pa'\xB6\x85\x83\x83a'\xFFV[\x92PPP[\x92\x91PPV[4\x15a\x0BvW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_jconsole.log\x90P_\x80\x83Q` \x85\x01\x84Z\xFAPPPV[``\x82a(\x14Wa(\x0F\x82a(^V[a(WV[\x81Q\x15\x80\x15a(+WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a(TW`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x06\xF1V[P\x80[\x93\x92PPPV[\x80Q\x15a(nW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80Ta(\x93\x90a-\x04V[_\x82U\x80`\x1F\x10a(\xA2WPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\"\xDB\x91\x90[\x80\x82\x11\x15a(\xCDW_\x81U`\x01\x01a(\xBAV[P\x90V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a(\xE7W_\x80\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a(\xFCW_\x80\xFD[a(W\x82a(\xD1V[_[\x83\x81\x10\x15a)\x1FW\x81\x81\x01Q\x83\x82\x01R` \x01a)\x07V[PP_\x91\x01RV[_\x81Q\x80\x84Ra)>\x81` \x86\x01` \x86\x01a)\x05V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x85\x81R_`\x01\x80`\xA0\x1B\x03\x80\x87\x16` \x84\x01R`\xA0`@\x84\x01Ra)y`\xA0\x84\x01\x87a)'V[\x94\x81\x16``\x84\x01R\x92\x90\x92\x16`\x80\x90\x91\x01RP\x93\x92PPPV[_` \x82\x84\x03\x12\x15a)\xA3W_\x80\xFD[P5\x91\x90PV[_` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x84\x88\x01_[\x83\x81\x10\x15a*CW\x88\x83\x03`?\x19\x01\x85R\x81Q\x80Q\x84R\x87\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x89\x86\x01R\x87\x82\x01Q`\xA0\x89\x87\x01\x81\x90R\x91a*\x14\x83\x88\x01\x83a)'V[``\x85\x81\x01Q\x83\x16\x90\x89\x01R`\x80\x94\x85\x01Q\x90\x91\x16\x93\x90\x96\x01\x92\x90\x92RPP\x93\x86\x01\x93\x90\x86\x01\x90`\x01\x01a)\xD1V[P\x90\x98\x97PPPPPPPPV[_\x80`@\x83\x85\x03\x12\x15a*bW_\x80\xFD[a*k\x83a(\xD1V[\x91Pa*y` \x84\x01a(\xD1V[\x90P\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x11\x15a*\xB0Wa*\xB0a*\x82V[`@Q`\x1F\x85\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a*\xD8Wa*\xD8a*\x82V[\x81`@R\x80\x93P\x85\x81R\x86\x86\x86\x01\x11\x15a*\xF0W_\x80\xFD[\x85\x85` \x83\x017_` \x87\x83\x01\x01RPPP\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a+\x1AW_\x80\xFD[a+#\x83a(\xD1V[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+>W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a+NW_\x80\xFD[a+]\x85\x825` \x84\x01a*\x96V[\x91PP\x92P\x92\x90PV[` \x81R_a(W` \x83\x01\x84a)'V[_\x80\x83`\x1F\x84\x01\x12a+\x89W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xA0W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a+\xB7W_\x80\xFD[\x92P\x92\x90PV[_\x80` \x83\x85\x03\x12\x15a+\xCFW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a+\xE5W_\x80\xFD[a+\xF1\x85\x82\x86\x01a+yV[\x90\x96\x90\x95P\x93PPPPV[_\x80_\x80``\x85\x87\x03\x12\x15a,\x10W_\x80\xFD[a,\x19\x85a(\xD1V[\x93P` \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,4W_\x80\xFD[a,@\x87\x82\x88\x01a+yV[\x90\x94P\x92Pa,S\x90P`@\x86\x01a(\xD1V[\x90P\x92\x95\x91\x94P\x92PV[_` \x82\x84\x03\x12\x15a,nW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a,\x84W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a,\x94W_\x80\xFD[a,\xA3\x84\x825` \x84\x01a*\x96V[\x94\x93PPPPV[_\x80_\x80_\x80_`\xE0\x88\x8A\x03\x12\x15a,\xC1W_\x80\xFD[a,\xCA\x88a(\xD1V[\x96Pa,\xD8` \x89\x01a(\xD1V[\x96\x99\x96\x98PPPP`@\x85\x015\x94``\x81\x015\x94`\x80\x82\x015\x94P`\xA0\x82\x015\x93P`\xC0\x90\x91\x015\x91PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a-\x18W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\n\xE2WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[` \x80\x82R`\"\x90\x82\x01R\x7FCertified: caller is not the own`@\x82\x01Ra2\xB9`\xF1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a'\xBBWa'\xBBa-xV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a-\xC3W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a(WW_\x80\xFD[\x80\x82\x01\x80\x82\x11\x15a'\xBBWa'\xBBa-xV[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[`\x1F\x82\x11\x15a\x07\xFCW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a.\x19WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a.8W_\x81U`\x01\x01a.%V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.YWa.Ya*\x82V[a.m\x81a.g\x84Ta-\x04V[\x84a-\xF4V[` \x80`\x1F\x83\x11`\x01\x81\x14a.\xA0W_\x84\x15a.\x89WP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua.\xF7V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a.\xCEW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a.\xAFV[P\x85\x82\x10\x15a.\xEBW\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PP`\x01\x84`\x01\x1B\x01\x85U[PPPPPPV[\x81\x81\x03a/\nWPPV[a/\x14\x82Ta-\x04V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/,Wa/,a*\x82V[a/:\x81a.g\x84Ta-\x04V[_`\x1F\x82\x11`\x01\x81\x14a/kW_\x83\x15a/TWP\x84\x82\x01T[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua.8V[_\x85\x81R` \x80\x82 \x86\x83R\x90\x82 `\x1F\x19\x86\x16\x92[\x83\x81\x10\x15a/\xA1W\x82\x86\x01T\x82U`\x01\x95\x86\x01\x95\x90\x91\x01\x90` \x01a/\x81V[P\x85\x83\x10\x15a/\xBEW\x81\x85\x01T_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[_`\x01\x82\x01a/\xDFWa/\xDFa-xV[P`\x01\x01\x90V[_` \x82\x84\x03\x12\x15a/\xF6W_\x80\xFD[PQ\x91\x90PV[_` \x80\x83R\x83T` \x84\x01R`\x01\x80`\x01`\xA0\x1B\x03`\x01\x86\x01T\x16`@\x85\x01R`\x02\x85\x01`\xA0``\x86\x01R_\x81Ta05\x81a-\x04V[\x80`\xC0\x89\x01R`\xE0`\x01\x83\x16_\x81\x14a0UW`\x01\x81\x14a0qWa0\x9EV[`\xFF\x19\x84\x16`\xE0\x8B\x01R`\xE0\x83\x15\x15`\x05\x1B\x8B\x01\x01\x94Pa0\x9EV[\x85_R` _ _[\x84\x81\x10\x15a0\x95W\x81T\x8C\x82\x01\x85\x01R\x90\x88\x01\x90\x89\x01a0zV[\x8B\x01`\xE0\x01\x95PP[PPPP`\x03\x87\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x88\x01R`\x04\x90\x97\x01T\x90\x96\x16`\xA0\x90\x95\x01\x94\x90\x94RP\x92\x93\x92PPPV[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a0\xF1Wa0\xF1a-xV[PP\x92\x91PPV[_\x80\x83Ta1\x06\x81a-\x04V[`\x01\x82\x81\x16\x80\x15a1\x1EW`\x01\x81\x14a13Wa1_V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa1_V[\x87_R` \x80_ _[\x85\x81\x10\x15a1VW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a1=V[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[_\x81a1yWa1ya-xV[P_\x19\x01\x90V[`@\x81R_a1\x92`@\x83\x01\x85a)'V[\x90P`\x01\x80`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`@\x81R_a1\xBB`@\x83\x01\x85a)'V[\x82\x81\x03` \x84\x01Ra1\xCD\x81\x85a)'V[\x95\x94PPPPPV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a1\xF5Wa1\xF5a-xV[P\x92\x91PPV[_\x82Qa2\r\x81\x84` \x87\x01a)\x05V[\x91\x90\x91\x01\x92\x91PPV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCBootstrapping default proposer...\xA2dipfsX\"\x12 \xF8\0m\xDE\x8B_;\x87\xC4\xFC\x17\xB1:*\x10r\x13I;\xCF\xC6j\xD9\x04\xD4\xC3W*\x112\xF48dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static ROUNDROBINV3_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RoundRobinV3<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RoundRobinV3<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RoundRobinV3<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RoundRobinV3<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RoundRobinV3<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RoundRobinV3))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RoundRobinV3<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ROUNDROBINV3_ABI.clone(),
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
                ROUNDROBINV3_ABI.clone(),
                ROUNDROBINV3_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `COOLDOWN_BLOCKS` (0x54831354) function
        pub fn cooldown_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 131, 19, 84], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DING` (0xd7ae76b6) function
        pub fn ding(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([215, 174, 118, 182], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `EXIT_PENALTY` (0x0acf4f93) function
        pub fn exit_penalty(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([10, 207, 79, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `FINALIZATION_BLOCKS` (0xb6ac9929) function
        pub fn finalization_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([182, 172, 153, 41], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ROTATION_BLOCKS` (0xd710ed93) function
        pub fn rotation_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([215, 16, 237, 147], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `STAKE` (0x125fdbbc) function
        pub fn stake(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([18, 95, 219, 188], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `UPGRADE_INTERFACE_VERSION` (0xad3cb1cc) function
        pub fn upgrade_interface_version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([173, 60, 177, 204], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `add_proposer` (0x55c25d17) function
        pub fn add_proposer(
            &self,
            proposer_url: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 194, 93, 23], proposer_url)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bootstrapDefaultProposer` (0x7f11f266) function
        pub fn bootstrap_default_proposer(
            &self,
            default_proposer_address: ::ethers::core::types::Address,
            default_proposer_url: ::std::string::String,
            nightfall_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [127, 17, 242, 102],
                    (default_proposer_address, default_proposer_url, nightfall_address),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `escrow` (0xe2fdcc17) function
        pub fn escrow(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([226, 253, 204, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_current_proposer_address` (0x750f25f4) function
        pub fn get_current_proposer_address(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([117, 15, 37, 244], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `get_proposers` (0x48c0f487) function
        pub fn get_proposers(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Proposer>> {
            self.0
                .method_hash([72, 192, 244, 135], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xedbf4ac2) function
        pub fn initialize(
            &self,
            x_509_address: ::ethers::core::types::Address,
            sanctions_list_address: ::ethers::core::types::Address,
            stake: ::ethers::core::types::U256,
            ding: ::ethers::core::types::U256,
            exit_penalty: ::ethers::core::types::U256,
            cooling_blocks: ::ethers::core::types::U256,
            rotation_blocks: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [237, 191, 74, 194],
                    (
                        x_509_address,
                        sanctions_list_address,
                        stake,
                        ding,
                        exit_penalty,
                        cooling_blocks,
                        rotation_blocks,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `last_exit_block` (0x22b092af) function
        pub fn last_exit_block(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([34, 176, 146, 175], p0)
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
        ///Calls the contract's `pending_withdraws` (0xc739d79c) function
        pub fn pending_withdraws(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([199, 57, 215, 156], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposer_count` (0x1ffff698) function
        pub fn proposer_count(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([31, 255, 246, 152], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposer_urls` (0xd1cef1ee) function
        pub fn proposer_urls(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([209, 206, 241, 238], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposers` (0x18177497) function
        pub fn proposers(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                ::std::string::String,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([24, 23, 116, 151], p0)
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
        ///Calls the contract's `remove_proposer` (0x4b98b3b6) function
        pub fn remove_proposer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 152, 179, 182], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rotate_proposer` (0xc12f666e) function
        pub fn rotate_proposer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([193, 47, 102, 110], ())
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
        ///Calls the contract's `setStakeRequirement` (0x4f09e304) function
        pub fn set_stake_requirement(
            &self,
            new_stake: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 9, 227, 4], new_stake)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `set_nightfall` (0x194f6b64) function
        pub fn set_nightfall(
            &self,
            nightfall_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 79, 107, 100], nightfall_address)
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
        ///Calls the contract's `start_l1_block` (0xac2beca0) function
        pub fn start_l_1_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([172, 43, 236, 160], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `start_l2_block` (0xa8404aee) function
        pub fn start_l_2_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([168, 64, 74, 238], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
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
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdraw` (0x2e1a7d4d) function
        pub fn withdraw(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], amount)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AuthoritiesUpdated` event
        pub fn authorities_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AuthoritiesUpdatedFilter,
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
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposerRotated` event
        pub fn proposer_rotated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposerRotatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `StakeRequirementUpdated` event
        pub fn stake_requirement_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            StakeRequirementUpdatedFilter,
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
            RoundRobinV3Events,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RoundRobinV3<M> {
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
    pub enum RoundRobinV3Errors {
        AddressEmptyCode(AddressEmptyCode),
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        ERC1967NonPayable(ERC1967NonPayable),
        FailedCall(FailedCall),
        InvalidInitialization(InvalidInitialization),
        NotInitializing(NotInitializing),
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for RoundRobinV3Errors {
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RoundRobinV3Errors {
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
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for RoundRobinV3Errors {
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for RoundRobinV3Errors {
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
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for RoundRobinV3Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for RoundRobinV3Errors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation> for RoundRobinV3Errors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for RoundRobinV3Errors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedCall> for RoundRobinV3Errors {
        fn from(value: FailedCall) -> Self {
            Self::FailedCall(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization> for RoundRobinV3Errors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for RoundRobinV3Errors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<UUPSUnauthorizedCallContext> for RoundRobinV3Errors {
        fn from(value: UUPSUnauthorizedCallContext) -> Self {
            Self::UUPSUnauthorizedCallContext(value)
        }
    }
    impl ::core::convert::From<UUPSUnsupportedProxiableUUID> for RoundRobinV3Errors {
        fn from(value: UUPSUnsupportedProxiableUUID) -> Self {
            Self::UUPSUnsupportedProxiableUUID(value)
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
    #[ethevent(name = "AuthoritiesUpdated", abi = "AuthoritiesUpdated(address,address)")]
    pub struct AuthoritiesUpdatedFilter {
        #[ethevent(indexed)]
        pub sanctions_list: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub x_509: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    #[ethevent(
        name = "ProposerRotated",
        abi = "ProposerRotated((uint256,address,string,address,address))"
    )]
    pub struct ProposerRotatedFilter {
        pub proposer: Proposer,
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
    #[ethevent(
        name = "StakeRequirementUpdated",
        abi = "StakeRequirementUpdated(uint256,uint256,uint256)"
    )]
    pub struct StakeRequirementUpdatedFilter {
        pub old_stake: ::ethers::core::types::U256,
        pub new_stake: ::ethers::core::types::U256,
        pub effective_from_l1_block: ::ethers::core::types::U256,
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
    pub enum RoundRobinV3Events {
        AuthoritiesUpdatedFilter(AuthoritiesUpdatedFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProposerRotatedFilter(ProposerRotatedFilter),
        StakeRequirementUpdatedFilter(StakeRequirementUpdatedFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for RoundRobinV3Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AuthoritiesUpdatedFilter::decode_log(log) {
                return Ok(RoundRobinV3Events::AuthoritiesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(RoundRobinV3Events::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(RoundRobinV3Events::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ProposerRotatedFilter::decode_log(log) {
                return Ok(RoundRobinV3Events::ProposerRotatedFilter(decoded));
            }
            if let Ok(decoded) = StakeRequirementUpdatedFilter::decode_log(log) {
                return Ok(RoundRobinV3Events::StakeRequirementUpdatedFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(RoundRobinV3Events::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for RoundRobinV3Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AuthoritiesUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposerRotatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StakeRequirementUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AuthoritiesUpdatedFilter> for RoundRobinV3Events {
        fn from(value: AuthoritiesUpdatedFilter) -> Self {
            Self::AuthoritiesUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for RoundRobinV3Events {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for RoundRobinV3Events {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<ProposerRotatedFilter> for RoundRobinV3Events {
        fn from(value: ProposerRotatedFilter) -> Self {
            Self::ProposerRotatedFilter(value)
        }
    }
    impl ::core::convert::From<StakeRequirementUpdatedFilter> for RoundRobinV3Events {
        fn from(value: StakeRequirementUpdatedFilter) -> Self {
            Self::StakeRequirementUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for RoundRobinV3Events {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `COOLDOWN_BLOCKS` function with signature `COOLDOWN_BLOCKS()` and selector `0x54831354`
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
    #[ethcall(name = "COOLDOWN_BLOCKS", abi = "COOLDOWN_BLOCKS()")]
    pub struct CooldownBlocksCall;
    ///Container type for all input parameters for the `DING` function with signature `DING()` and selector `0xd7ae76b6`
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
    #[ethcall(name = "DING", abi = "DING()")]
    pub struct DingCall;
    ///Container type for all input parameters for the `EXIT_PENALTY` function with signature `EXIT_PENALTY()` and selector `0x0acf4f93`
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
    #[ethcall(name = "EXIT_PENALTY", abi = "EXIT_PENALTY()")]
    pub struct ExitPenaltyCall;
    ///Container type for all input parameters for the `FINALIZATION_BLOCKS` function with signature `FINALIZATION_BLOCKS()` and selector `0xb6ac9929`
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
    #[ethcall(name = "FINALIZATION_BLOCKS", abi = "FINALIZATION_BLOCKS()")]
    pub struct FinalizationBlocksCall;
    ///Container type for all input parameters for the `ROTATION_BLOCKS` function with signature `ROTATION_BLOCKS()` and selector `0xd710ed93`
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
    #[ethcall(name = "ROTATION_BLOCKS", abi = "ROTATION_BLOCKS()")]
    pub struct RotationBlocksCall;
    ///Container type for all input parameters for the `STAKE` function with signature `STAKE()` and selector `0x125fdbbc`
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
    #[ethcall(name = "STAKE", abi = "STAKE()")]
    pub struct StakeCall;
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
    ///Container type for all input parameters for the `add_proposer` function with signature `add_proposer(string)` and selector `0x55c25d17`
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
    #[ethcall(name = "add_proposer", abi = "add_proposer(string)")]
    pub struct AddProposerCall {
        pub proposer_url: ::std::string::String,
    }
    ///Container type for all input parameters for the `bootstrapDefaultProposer` function with signature `bootstrapDefaultProposer(address,string,address)` and selector `0x7f11f266`
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
        name = "bootstrapDefaultProposer",
        abi = "bootstrapDefaultProposer(address,string,address)"
    )]
    pub struct BootstrapDefaultProposerCall {
        pub default_proposer_address: ::ethers::core::types::Address,
        pub default_proposer_url: ::std::string::String,
        pub nightfall_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `escrow` function with signature `escrow()` and selector `0xe2fdcc17`
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
    #[ethcall(name = "escrow", abi = "escrow()")]
    pub struct EscrowCall;
    ///Container type for all input parameters for the `get_current_proposer_address` function with signature `get_current_proposer_address()` and selector `0x750f25f4`
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
        name = "get_current_proposer_address",
        abi = "get_current_proposer_address()"
    )]
    pub struct GetCurrentProposerAddressCall;
    ///Container type for all input parameters for the `get_proposers` function with signature `get_proposers()` and selector `0x48c0f487`
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
    #[ethcall(name = "get_proposers", abi = "get_proposers()")]
    pub struct GetProposersCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,uint256,uint256,uint256,uint256,uint256)` and selector `0xedbf4ac2`
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
        name = "initialize",
        abi = "initialize(address,address,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct InitializeCall {
        pub x_509_address: ::ethers::core::types::Address,
        pub sanctions_list_address: ::ethers::core::types::Address,
        pub stake: ::ethers::core::types::U256,
        pub ding: ::ethers::core::types::U256,
        pub exit_penalty: ::ethers::core::types::U256,
        pub cooling_blocks: ::ethers::core::types::U256,
        pub rotation_blocks: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `last_exit_block` function with signature `last_exit_block(address)` and selector `0x22b092af`
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
    #[ethcall(name = "last_exit_block", abi = "last_exit_block(address)")]
    pub struct LastExitBlockCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `pending_withdraws` function with signature `pending_withdraws(address)` and selector `0xc739d79c`
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
    #[ethcall(name = "pending_withdraws", abi = "pending_withdraws(address)")]
    pub struct PendingWithdrawsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `proposer_count` function with signature `proposer_count()` and selector `0x1ffff698`
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
    #[ethcall(name = "proposer_count", abi = "proposer_count()")]
    pub struct ProposerCountCall;
    ///Container type for all input parameters for the `proposer_urls` function with signature `proposer_urls(string)` and selector `0xd1cef1ee`
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
    #[ethcall(name = "proposer_urls", abi = "proposer_urls(string)")]
    pub struct ProposerUrlsCall(pub ::std::string::String);
    ///Container type for all input parameters for the `proposers` function with signature `proposers(address)` and selector `0x18177497`
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
    #[ethcall(name = "proposers", abi = "proposers(address)")]
    pub struct ProposersCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `remove_proposer` function with signature `remove_proposer()` and selector `0x4b98b3b6`
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
    #[ethcall(name = "remove_proposer", abi = "remove_proposer()")]
    pub struct RemoveProposerCall;
    ///Container type for all input parameters for the `rotate_proposer` function with signature `rotate_proposer()` and selector `0xc12f666e`
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
    #[ethcall(name = "rotate_proposer", abi = "rotate_proposer()")]
    pub struct RotateProposerCall;
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
    ///Container type for all input parameters for the `setStakeRequirement` function with signature `setStakeRequirement(uint256)` and selector `0x4f09e304`
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
    #[ethcall(name = "setStakeRequirement", abi = "setStakeRequirement(uint256)")]
    pub struct SetStakeRequirementCall {
        pub new_stake: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `set_nightfall` function with signature `set_nightfall(address)` and selector `0x194f6b64`
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
    #[ethcall(name = "set_nightfall", abi = "set_nightfall(address)")]
    pub struct SetNightfallCall {
        pub nightfall_address: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `start_l1_block` function with signature `start_l1_block()` and selector `0xac2beca0`
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
    #[ethcall(name = "start_l1_block", abi = "start_l1_block()")]
    pub struct StartL1BlockCall;
    ///Container type for all input parameters for the `start_l2_block` function with signature `start_l2_block()` and selector `0xa8404aee`
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
    #[ethcall(name = "start_l2_block", abi = "start_l2_block()")]
    pub struct StartL2BlockCall;
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
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
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
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
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `withdraw` function with signature `withdraw(uint256)` and selector `0x2e1a7d4d`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub amount: ::ethers::core::types::U256,
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
    pub enum RoundRobinV3Calls {
        CooldownBlocks(CooldownBlocksCall),
        Ding(DingCall),
        ExitPenalty(ExitPenaltyCall),
        FinalizationBlocks(FinalizationBlocksCall),
        RotationBlocks(RotationBlocksCall),
        Stake(StakeCall),
        UpgradeInterfaceVersion(UpgradeInterfaceVersionCall),
        AddProposer(AddProposerCall),
        BootstrapDefaultProposer(BootstrapDefaultProposerCall),
        Escrow(EscrowCall),
        GetCurrentProposerAddress(GetCurrentProposerAddressCall),
        GetProposers(GetProposersCall),
        Initialize(InitializeCall),
        LastExitBlock(LastExitBlockCall),
        Owner(OwnerCall),
        PendingWithdraws(PendingWithdrawsCall),
        ProposerCount(ProposerCountCall),
        ProposerUrls(ProposerUrlsCall),
        Proposers(ProposersCall),
        ProxiableUUID(ProxiableUUIDCall),
        RemoveProposer(RemoveProposerCall),
        RotateProposer(RotateProposerCall),
        SetAuthorities(SetAuthoritiesCall),
        SetStakeRequirement(SetStakeRequirementCall),
        SetNightfall(SetNightfallCall),
        SetSanctionsList(SetSanctionsListCall),
        SetX509Address(SetX509AddressCall),
        StartL1Block(StartL1BlockCall),
        StartL2Block(StartL2BlockCall),
        TransferOwnership(TransferOwnershipCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        Version(VersionCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for RoundRobinV3Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CooldownBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CooldownBlocks(decoded));
            }
            if let Ok(decoded) = <DingCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ding(decoded));
            }
            if let Ok(decoded) = <ExitPenaltyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExitPenalty(decoded));
            }
            if let Ok(decoded) = <FinalizationBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FinalizationBlocks(decoded));
            }
            if let Ok(decoded) = <RotationBlocksCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RotationBlocks(decoded));
            }
            if let Ok(decoded) = <StakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Stake(decoded));
            }
            if let Ok(decoded) = <UpgradeInterfaceVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeInterfaceVersion(decoded));
            }
            if let Ok(decoded) = <AddProposerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddProposer(decoded));
            }
            if let Ok(decoded) = <BootstrapDefaultProposerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BootstrapDefaultProposer(decoded));
            }
            if let Ok(decoded) = <EscrowCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Escrow(decoded));
            }
            if let Ok(decoded) = <GetCurrentProposerAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCurrentProposerAddress(decoded));
            }
            if let Ok(decoded) = <GetProposersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetProposers(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <LastExitBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastExitBlock(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PendingWithdrawsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PendingWithdraws(decoded));
            }
            if let Ok(decoded) = <ProposerCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposerCount(decoded));
            }
            if let Ok(decoded) = <ProposerUrlsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposerUrls(decoded));
            }
            if let Ok(decoded) = <ProposersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Proposers(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded) = <RemoveProposerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveProposer(decoded));
            }
            if let Ok(decoded) = <RotateProposerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RotateProposer(decoded));
            }
            if let Ok(decoded) = <SetAuthoritiesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetAuthorities(decoded));
            }
            if let Ok(decoded) = <SetStakeRequirementCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetStakeRequirement(decoded));
            }
            if let Ok(decoded) = <SetNightfallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetNightfall(decoded));
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
            if let Ok(decoded) = <StartL1BlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartL1Block(decoded));
            }
            if let Ok(decoded) = <StartL2BlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartL2Block(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <UpgradeToAndCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeToAndCall(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RoundRobinV3Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CooldownBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ding(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExitPenalty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FinalizationBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RotationBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Stake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpgradeInterfaceVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddProposer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BootstrapDefaultProposer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Escrow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCurrentProposerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProposers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastExitBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingWithdraws(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposerCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposerUrls(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Proposers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveProposer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RotateProposer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAuthorities(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetStakeRequirement(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetNightfall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetSanctionsList(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetX509Address(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartL1Block(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StartL2Block(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeToAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RoundRobinV3Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CooldownBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ding(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExitPenalty(element) => ::core::fmt::Display::fmt(element, f),
                Self::FinalizationBlocks(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RotationBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::Stake(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeInterfaceVersion(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AddProposer(element) => ::core::fmt::Display::fmt(element, f),
                Self::BootstrapDefaultProposer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Escrow(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCurrentProposerAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetProposers(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastExitBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingWithdraws(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposerCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposerUrls(element) => ::core::fmt::Display::fmt(element, f),
                Self::Proposers(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveProposer(element) => ::core::fmt::Display::fmt(element, f),
                Self::RotateProposer(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAuthorities(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStakeRequirement(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetNightfall(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSanctionsList(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetX509Address(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartL1Block(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartL2Block(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CooldownBlocksCall> for RoundRobinV3Calls {
        fn from(value: CooldownBlocksCall) -> Self {
            Self::CooldownBlocks(value)
        }
    }
    impl ::core::convert::From<DingCall> for RoundRobinV3Calls {
        fn from(value: DingCall) -> Self {
            Self::Ding(value)
        }
    }
    impl ::core::convert::From<ExitPenaltyCall> for RoundRobinV3Calls {
        fn from(value: ExitPenaltyCall) -> Self {
            Self::ExitPenalty(value)
        }
    }
    impl ::core::convert::From<FinalizationBlocksCall> for RoundRobinV3Calls {
        fn from(value: FinalizationBlocksCall) -> Self {
            Self::FinalizationBlocks(value)
        }
    }
    impl ::core::convert::From<RotationBlocksCall> for RoundRobinV3Calls {
        fn from(value: RotationBlocksCall) -> Self {
            Self::RotationBlocks(value)
        }
    }
    impl ::core::convert::From<StakeCall> for RoundRobinV3Calls {
        fn from(value: StakeCall) -> Self {
            Self::Stake(value)
        }
    }
    impl ::core::convert::From<UpgradeInterfaceVersionCall> for RoundRobinV3Calls {
        fn from(value: UpgradeInterfaceVersionCall) -> Self {
            Self::UpgradeInterfaceVersion(value)
        }
    }
    impl ::core::convert::From<AddProposerCall> for RoundRobinV3Calls {
        fn from(value: AddProposerCall) -> Self {
            Self::AddProposer(value)
        }
    }
    impl ::core::convert::From<BootstrapDefaultProposerCall> for RoundRobinV3Calls {
        fn from(value: BootstrapDefaultProposerCall) -> Self {
            Self::BootstrapDefaultProposer(value)
        }
    }
    impl ::core::convert::From<EscrowCall> for RoundRobinV3Calls {
        fn from(value: EscrowCall) -> Self {
            Self::Escrow(value)
        }
    }
    impl ::core::convert::From<GetCurrentProposerAddressCall> for RoundRobinV3Calls {
        fn from(value: GetCurrentProposerAddressCall) -> Self {
            Self::GetCurrentProposerAddress(value)
        }
    }
    impl ::core::convert::From<GetProposersCall> for RoundRobinV3Calls {
        fn from(value: GetProposersCall) -> Self {
            Self::GetProposers(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for RoundRobinV3Calls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LastExitBlockCall> for RoundRobinV3Calls {
        fn from(value: LastExitBlockCall) -> Self {
            Self::LastExitBlock(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for RoundRobinV3Calls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PendingWithdrawsCall> for RoundRobinV3Calls {
        fn from(value: PendingWithdrawsCall) -> Self {
            Self::PendingWithdraws(value)
        }
    }
    impl ::core::convert::From<ProposerCountCall> for RoundRobinV3Calls {
        fn from(value: ProposerCountCall) -> Self {
            Self::ProposerCount(value)
        }
    }
    impl ::core::convert::From<ProposerUrlsCall> for RoundRobinV3Calls {
        fn from(value: ProposerUrlsCall) -> Self {
            Self::ProposerUrls(value)
        }
    }
    impl ::core::convert::From<ProposersCall> for RoundRobinV3Calls {
        fn from(value: ProposersCall) -> Self {
            Self::Proposers(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for RoundRobinV3Calls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RemoveProposerCall> for RoundRobinV3Calls {
        fn from(value: RemoveProposerCall) -> Self {
            Self::RemoveProposer(value)
        }
    }
    impl ::core::convert::From<RotateProposerCall> for RoundRobinV3Calls {
        fn from(value: RotateProposerCall) -> Self {
            Self::RotateProposer(value)
        }
    }
    impl ::core::convert::From<SetAuthoritiesCall> for RoundRobinV3Calls {
        fn from(value: SetAuthoritiesCall) -> Self {
            Self::SetAuthorities(value)
        }
    }
    impl ::core::convert::From<SetStakeRequirementCall> for RoundRobinV3Calls {
        fn from(value: SetStakeRequirementCall) -> Self {
            Self::SetStakeRequirement(value)
        }
    }
    impl ::core::convert::From<SetNightfallCall> for RoundRobinV3Calls {
        fn from(value: SetNightfallCall) -> Self {
            Self::SetNightfall(value)
        }
    }
    impl ::core::convert::From<SetSanctionsListCall> for RoundRobinV3Calls {
        fn from(value: SetSanctionsListCall) -> Self {
            Self::SetSanctionsList(value)
        }
    }
    impl ::core::convert::From<SetX509AddressCall> for RoundRobinV3Calls {
        fn from(value: SetX509AddressCall) -> Self {
            Self::SetX509Address(value)
        }
    }
    impl ::core::convert::From<StartL1BlockCall> for RoundRobinV3Calls {
        fn from(value: StartL1BlockCall) -> Self {
            Self::StartL1Block(value)
        }
    }
    impl ::core::convert::From<StartL2BlockCall> for RoundRobinV3Calls {
        fn from(value: StartL2BlockCall) -> Self {
            Self::StartL2Block(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for RoundRobinV3Calls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall> for RoundRobinV3Calls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<VersionCall> for RoundRobinV3Calls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for RoundRobinV3Calls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    ///Container type for all return fields from the `COOLDOWN_BLOCKS` function with signature `COOLDOWN_BLOCKS()` and selector `0x54831354`
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
    pub struct CooldownBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `DING` function with signature `DING()` and selector `0xd7ae76b6`
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
    pub struct DingReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `EXIT_PENALTY` function with signature `EXIT_PENALTY()` and selector `0x0acf4f93`
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
    pub struct ExitPenaltyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `FINALIZATION_BLOCKS` function with signature `FINALIZATION_BLOCKS()` and selector `0xb6ac9929`
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
    pub struct FinalizationBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ROTATION_BLOCKS` function with signature `ROTATION_BLOCKS()` and selector `0xd710ed93`
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
    pub struct RotationBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `STAKE` function with signature `STAKE()` and selector `0x125fdbbc`
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
    pub struct StakeReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `escrow` function with signature `escrow()` and selector `0xe2fdcc17`
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
    pub struct EscrowReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `get_current_proposer_address` function with signature `get_current_proposer_address()` and selector `0x750f25f4`
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
    pub struct GetCurrentProposerAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `get_proposers` function with signature `get_proposers()` and selector `0x48c0f487`
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
    pub struct GetProposersReturn(pub ::std::vec::Vec<Proposer>);
    ///Container type for all return fields from the `last_exit_block` function with signature `last_exit_block(address)` and selector `0x22b092af`
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
    pub struct LastExitBlockReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `pending_withdraws` function with signature `pending_withdraws(address)` and selector `0xc739d79c`
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
    pub struct PendingWithdrawsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proposer_count` function with signature `proposer_count()` and selector `0x1ffff698`
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
    pub struct ProposerCountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `proposer_urls` function with signature `proposer_urls(string)` and selector `0xd1cef1ee`
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
    pub struct ProposerUrlsReturn(pub bool);
    ///Container type for all return fields from the `proposers` function with signature `proposers(address)` and selector `0x18177497`
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
    pub struct ProposersReturn {
        pub stake: ::ethers::core::types::U256,
        pub addr: ::ethers::core::types::Address,
        pub url: ::std::string::String,
        pub next_addr: ::ethers::core::types::Address,
        pub previous_addr: ::ethers::core::types::Address,
    }
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
    ///Container type for all return fields from the `start_l1_block` function with signature `start_l1_block()` and selector `0xac2beca0`
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
    pub struct StartL1BlockReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `start_l2_block` function with signature `start_l2_block()` and selector `0xa8404aee`
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
    pub struct StartL2BlockReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
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
    pub struct VersionReturn(pub ::std::string::String);
}
