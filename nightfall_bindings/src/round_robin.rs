pub use round_robin::*;
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
pub mod round_robin {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
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
                        name: ::std::borrow::ToOwned::to_owned("default_proposer_url"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("stake"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("ding"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("exit_penalty"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("cooling_blocks"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("rotation_blocks"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
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
                    ::std::borrow::ToOwned::to_owned("ROTATION_BlOCKS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ROTATION_BlOCKS"),
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
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ROUNDROBIN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R_`\x0BU`@Qa\x1F-8\x03\x80a\x1F-\x839\x81\x01`@\x81\x90Ra\0&\x91a\x02\x16V[`\x80\x83\x90R`\xA0\x82\x90R`\xC0\x81\x90R4\x83\x14a\0\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FYou have not paid the correct st`D\x82\x01Rk\x18Z\xDA[\x99\xC8\x18[[\xDD[\x9D`\xA2\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R`\x80\x80Q\x80\x83R`\x01`\x01`\xA0\x1B\x03\x89\x16` \x84\x01\x81\x90R\x93\x83\x01\x88\x90R``\x83\x01\x84\x90R\x90\x82\x01\x83\x90R`\x03\x90\x81U`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90\x93\x17\x90\x92U\x90`\x05a\0\xF8\x87\x82a\x03\x82V[P``\x82\x01Q`\x03\x82\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90\x91U`\x80\x90\x94\x01Q`\x04\x93\x84\x01\x80T\x86\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x88\x82\x16_\x90\x81R` \x81\x90R`@\x90 \x81T\x81U\x92T`\x01\x84\x01\x80T\x91\x90\x93\x16\x94\x16\x93\x90\x93\x17\x90U`\x02\x81\x01a\x01o`\x05\x82a\x04?V[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x93\x90\x92\x01\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90U`@Q`\x01\x90`\x02\x90a\x01\xC1\x90\x87\x90a\x05\x0BV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80T\x91\x15\x15`\xFF\x19\x90\x92\x16\x91\x90\x91\x17\x90UPP`\x01`\nUPP`\r\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90UPa\x05!V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_____`\xA0\x86\x88\x03\x12\x15a\x02*W__\xFD[\x85Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02@W__\xFD[` \x87\x01Q\x90\x95P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02[W__\xFD[\x86\x01`\x1F\x81\x01\x88\x13a\x02kW__\xFD[\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\x84Wa\x02\x84a\x02\x02V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x02\xB2Wa\x02\xB2a\x02\x02V[`@R\x81\x81R\x82\x82\x01` \x01\x8A\x10\x15a\x02\xC9W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x97\x9A\x91\x99P\x97\x96\x95P\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x03\x12W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x030WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x03}W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x03[WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x03zW_\x81U`\x01\x01a\x03gV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03\x9BWa\x03\x9Ba\x02\x02V[a\x03\xAF\x81a\x03\xA9\x84Ta\x02\xFEV[\x84a\x036V[` `\x1F\x82\x11`\x01\x81\x14a\x03\xE4W_\x83\x15a\x03\xCAWP\x84\x82\x01Q[`\x01\x84\x90\x1B_\x19`\x03\x86\x90\x1B\x1C\x19\x82\x16\x17[\x85UPa\x03zV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x04\x13W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x03\xF3V[P\x84\x82\x10\x15a\x040W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x81\x03a\x04JWPPV[a\x04T\x82Ta\x02\xFEV[`\x01`\x01`@\x1B\x03\x81\x11\x15a\x04kWa\x04ka\x02\x02V[a\x04y\x81a\x03\xA9\x84Ta\x02\xFEV[_`\x1F\x82\x11`\x01\x81\x14a\x04\xA8W_\x83\x15a\x03\xCAWP\x81\x85\x01T`\x01\x84\x90\x1B_\x19`\x03\x86\x90\x1B\x1C\x19\x82\x16\x17a\x03\xDCV[_\x85\x81R` \x80\x82 \x86\x83R\x90\x82 `\x1F\x19\x86\x16\x92[\x83\x81\x10\x15a\x04\xDEW\x82\x86\x01T\x82U`\x01\x95\x86\x01\x95\x90\x91\x01\x90` \x01a\x04\xBEV[P\x85\x83\x10\x15a\x04\xFBW\x81\x85\x01T_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[`\x80Q`\xA0Q`\xC0Qa\x19\xBFa\x05n_9_\x81\x81a\x02I\x01Ra\x11\xEC\x01R_\x81\x81a\x03/\x01Ra\x12B\x01R_\x81\x81a\x01\x0F\x01R\x81\x81a\x08|\x01R\x81\x81a\t\xE6\x01Ra\n/\x01Ra\x19\xBF_\xF3\xFE`\x80`@R`\x046\x10a\0\xFAW_5`\xE0\x1C\x80cu\x0F%\xF4\x11a\0\x92W\x80c\xC1/fn\x11a\0bW\x80c\xC1/fn\x14a\x02\x95W\x80c\xC79\xD7\x9C\x14a\x02\xA9W\x80c\xD1\xCE\xF1\xEE\x14a\x02\xD4W\x80c\xD7\xAEv\xB6\x14a\x03\x1EW\x80c\xE2\xFD\xCC\x17\x14a\x03QW__\xFD[\x80cu\x0F%\xF4\x14a\x02\x11W\x80c\x97\xC1\xD0G\x14a\x028W\x80c\xA8@J\xEE\x14a\x02kW\x80c\xAC+\xEC\xA0\x14a\x02\x80W__\xFD[\x80c.\x1A}M\x11a\0\xCDW\x80c.\x1A}M\x14a\x01\xAAW\x80cH\xC0\xF4\x87\x14a\x01\xC9W\x80cK\x98\xB3\xB6\x14a\x01\xEAW\x80cU\xC2]\x17\x14a\x01\xFEW__\xFD[\x80c\x12_\xDB\xBC\x14a\0\xFEW\x80c\x18\x17t\x97\x14a\x01DW\x80c\x19Okd\x14a\x01tW\x80c\x1F\xFF\xF6\x98\x14a\x01\x95W[__\xFD[4\x80\x15a\x01\tW__\xFD[Pa\x011\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01OW__\xFD[Pa\x01ca\x01^6`\x04a\x12\xD1V[a\x03fV[`@Qa\x01;\x95\x94\x93\x92\x91\x90a\x13,V[4\x80\x15a\x01\x7FW__\xFD[Pa\x01\x93a\x01\x8E6`\x04a\x12\xD1V[a\x041V[\0[4\x80\x15a\x01\xA0W__\xFD[Pa\x011`\nT\x81V[4\x80\x15a\x01\xB5W__\xFD[Pa\x01\x93a\x01\xC46`\x04a\x13wV[a\x04\xC0V[4\x80\x15a\x01\xD4W__\xFD[Pa\x01\xDDa\x05\x87V[`@Qa\x01;\x91\x90a\x13\x8EV[4\x80\x15a\x01\xF5W__\xFD[Pa\x01\x93a\x08oV[a\x01\x93a\x02\x0C6`\x04a\x14=V[a\x08zV[4\x80\x15a\x02\x1CW__\xFD[P`\x04T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01;V[4\x80\x15a\x02CW__\xFD[Pa\x011\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02vW__\xFD[Pa\x011`\tT\x81V[4\x80\x15a\x02\x8BW__\xFD[Pa\x011`\x08T\x81V[4\x80\x15a\x02\xA0W__\xFD[Pa\x01\x93a\x0C|V[4\x80\x15a\x02\xB4W__\xFD[Pa\x011a\x02\xC36`\x04a\x12\xD1V[`\x01` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x02\xDFW__\xFD[Pa\x03\x0Ea\x02\xEE6`\x04a\x14\xBFV[\x80Q` \x81\x83\x01\x81\x01\x80Q`\x02\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01;V[4\x80\x15a\x03)W__\xFD[Pa\x011\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\\W__\xFD[Pa\x011`\x0BT\x81V[_` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01\x80T\x92\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92a\x03\x97\x90a\x15rV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xC3\x90a\x15rV[\x80\x15a\x04\x0EW\x80`\x1F\x10a\x03\xE5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\x0EV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\xF1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPP`\x03\x83\x01T`\x04\x90\x93\x01T\x91\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x16\x90P\x85V[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FOnly the owner can call this fun`D\x82\x01Rd1\xBA4\xB7\xB7`\xD9\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[3_\x90\x81R`\x01` R`@\x90 T\x80\x82\x11\x15a\x055W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FYou are trying to withdraw more `D\x82\x01Rlthan you have`\x98\x1B`d\x82\x01R`\x84\x01a\x04\x95V[3_\x90\x81R`\x01` R`@\x81 \x80T\x84\x92\x90a\x05S\x90\x84\x90a\x15\xB8V[\x90\x91UPP`@Q3\x90\x83\x15a\x08\xFC\x02\x90\x84\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x05\x82W=__>=_\xFD[PPPV[``_`\nTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xA5Wa\x05\xA5a\x14\xABV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xFDW\x81` \x01[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R``\x93\x83\x01\x84\x90R\x92\x82\x01\x81\x90R`\x80\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x05\xC3W\x90P[P`@\x80Q`\xA0\x81\x01\x82R`\x03\x80T\x82R`\x04T`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R`\x05\x80T\x94\x95P\x91\x93\x90\x92\x84\x01\x91\x90a\x067\x90a\x15rV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06c\x90a\x15rV[\x80\x15a\x06\xAEW\x80`\x1F\x10a\x06\x85Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xAEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R`\x04\x90\x92\x01T\x90\x91\x16`@\x90\x91\x01R\x81Q\x82\x90_\x90a\x06\xECWa\x06\xECa\x15\xD1V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01[`\nT\x81\x10\x15a\x08iW_\x80\x83a\x07\x12`\x01\x85a\x15\xB8V[\x81Q\x81\x10a\x07\"Wa\x07\"a\x15\xD1V[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x02\x82\x01\x80Ta\x07\x9F\x90a\x15rV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xCB\x90a\x15rV[\x80\x15a\x08\x16W\x80`\x1F\x10a\x07\xEDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x16V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xF9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R`\x04\x90\x92\x01T\x90\x91\x16`@\x90\x91\x01R\x82Q\x83\x90\x83\x90\x81\x10a\x08VWa\x08Va\x15\xD1V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x06\xFAV[P\x91\x90PV[a\x08x3a\x0E\x9FV[V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x004\x14a\x08\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FYou have not paid the correct st`D\x82\x01Rk\x18Z\xDA[\x99\xC8\x18[[\xDD[\x9D`\xA2\x1B`d\x82\x01R`\x84\x01a\x04\x95V[3_\x90\x81R` \x81\x90R`@\x90 `\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\tfW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FYou are already a proposer\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x95V[`\x02\x82\x82`@Qa\tx\x92\x91\x90a\x15\xE5V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\xFF\x16\x15a\t\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FThis proposer URL is already in `D\x82\x01Rbuse`\xE8\x1B`d\x82\x01R`\x84\x01a\x04\x95V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0B_\x82\x82Ta\n\x15\x91\x90a\x15\xF4V[\x90\x91UPP`\x04T`\x07T`\x06T`@\x80Q`\xA0\x81\x01\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3` \x80\x83\x01\x91\x90\x91R\x82Q`\x1F\x88\x01\x82\x90\x04\x82\x02\x81\x01\x82\x01\x84R\x87\x81R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x96\x95\x86\x16\x95\x90\x94\x16\x93\x91\x92\x83\x01\x91\x90\x88\x90\x88\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x93\x85RPPP`\x01`\x01`\xA0\x1B\x03\x86\x81\x16` \x80\x85\x01\x91\x90\x91R\x86\x82\x16`@\x94\x85\x01R3\x83R\x82\x81R\x91\x83\x90 \x84Q\x81U\x91\x84\x01Q`\x01\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U\x90\x82\x01Q`\x02\x82\x01\x90a\n\xFC\x90\x82a\x16RV[P``\x82\x01Q`\x03\x82\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90\x91U`\x80\x90\x94\x01Q`\x04\x93\x84\x01\x80T\x86\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x86\x82\x16_\x81\x81R` \x81\x90R`@\x80\x82 \x90\x95\x01\x80T3\x90\x88\x16\x81\x17\x90\x91U\x88\x85\x16\x82R\x94\x90 \x90\x91\x01\x80T\x90\x94\x16\x90\x92\x17\x90\x92U\x90\x82\x16\x03a\x0B\xA4W`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R` \x81\x90R`@\x90 `\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U[`\x01`\x02\x86\x86`@Qa\x0B\xB8\x92\x91\x90a\x15\xE5V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T`\xFF\x19\x16\x93\x15\x15\x93\x90\x93\x17\x90\x92U`\x01`\x01`\xA0\x1B\x03\x85\x81\x16_\x90\x81R\x91\x82\x90R\x91\x90 \x80T`\x03\x90\x81U`\x01\x82\x01T`\x04\x80T\x91\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90\x92U\x90`\x05a\x0C#`\x02\x84\x01\x82a\x17\x10V[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x93\x90\x92\x01\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90U`\n\x80T\x90_a\x0Cp\x83a\x17\xDDV[\x91\x90PUPPPPPPV[a\x0C\x84a\x11\xE9V[a\x0C\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FIt is not time to rotate the pro`D\x82\x01Rd87\xB9\xB2\xB9`\xD9\x1B`d\x82\x01R`\x84\x01a\x04\x95V[`\tT`\x0C_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c(\xC3\xD7\xE6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rU\x91\x90a\x17\xF5V[\x03a\roW`\x04Ta\ro\x90`\x01`\x01`\xA0\x1B\x03\x16a\x12 V[`\x06T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R` \x81\x90R`@\x90 \x80T`\x03\x90\x81U`\x01\x82\x01T`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x94\x16\x17\x90\x92U\x90`\x05a\r\xBC`\x02\x84\x01\x82a\x17\x10V[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x92\x84\x01\x80T\x90\x91\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91UC`\x08U`\x0CT`@\x80Qc\x14a\xEB\xF3`\xE1\x1B\x81R\x90Q\x91\x90\x92\x16\x92c(\xC3\xD7\xE6\x92\x80\x82\x01\x92` \x92\x90\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0E=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ea\x91\x90a\x17\xF5V[`\tU`@Q\x7F\xAC;\x1Ci)\xA3\xB1\xB5)\xEA\xCCFr\x0F\x90\xC2\xD5K\x83CE\xDC\xA9\xBA}z\x94\x83\xE0\xDER\xBE\x90a\x0E\x95\x90`\x03\x90a\x18\x0CV[`@Q\x80\x91\x03\x90\xA1V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x81\x81R` \x81\x90R`@\x90 `\x01\x01T\x90\x91\x16\x14a\x0F\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FThis proposer does not exist\0\0\0\0`D\x82\x01R`d\x01a\x04\x95V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0FmW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FThe proposer address cannot be z`D\x82\x01Rbero`\xE8\x1B`d\x82\x01R`\x84\x01a\x04\x95V[`\x04T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x03a\x0F\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FYou cannot remove the current pr`D\x82\x01Re7\xB87\xB9\xB2\xB9`\xD1\x1B`d\x82\x01R`\x84\x01a\x04\x95V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R` \x81\x90R`@\x80\x82 `\x03\x80\x82\x01\x80T\x86\x16\x85R\x83\x85 `\x04\x80\x85\x01T\x88\x16\x80\x88R\x95\x87 \x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x97\x17\x90U\x91T\x92\x82\x01\x80T\x90\x95\x16\x92\x90\x96\x16\x91\x90\x91\x17\x90\x92U\x80T`\x0B\x80T\x92\x95\x94\x91\x92\x90\x91\x90a\x10S\x90\x84\x90a\x15\xB8V[\x90\x91UPP\x82T`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x01` R`@\x81 \x80T\x90\x91\x90a\x10\x81\x90\x84\x90a\x15\xF4V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R` \x81\x90R`@\x81 \x81\x81U`\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x90a\x10\xC0`\x02\x83\x01\x82a\x12\x7FV[P`\x03\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x04\x90\x91\x01\x80T\x90\x91\x16\x90U`@Q`\x02\x90a\x10\xF5\x90\x85\x83\x01\x90a\x18\xDDV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80T`\xFF\x19\x16\x90U`\n\x80T\x90_a\x11\x1B\x83a\x19NV[\x91\x90PUP`\nT`\x01\x03a\x11\xE3W`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R` \x81\x90R`@\x80\x82 `\x03\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x95\x17\x90U\x85T\x85\x16\x80\x84R\x82\x84 \x87\x01\x80T\x86\x16\x90\x91\x17\x90U\x85T\x80\x86\x16\x84R\x91\x90\x92 \x80T\x83U`\x01\x81\x01T\x91\x90\x93\x16\x93\x16\x92\x90\x92\x17\x90\x92U`\x05a\x11\xA4`\x02\x84\x01\x82a\x17\x10V[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x93\x90\x92\x01\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90U[PPPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x08Ta\x12\x18\x91\x90a\x15\xF4V[C\x10\x15\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R` \x81\x90R`@\x81 \x80T\x90\x91\x90a\x12g\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x19cV[\x90P_\x81\x12\x15a\x12zWa\x05\x82\x83a\x0E\x9FV[\x90UPV[P\x80Ta\x12\x8B\x90a\x15rV[_\x82U\x80`\x1F\x10a\x12\x9AWPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\x12\xB6\x91\x90a\x12\xB9V[PV[[\x80\x82\x11\x15a\x12\xCDW_\x81U`\x01\x01a\x12\xBAV[P\x90V[_` \x82\x84\x03\x12\x15a\x12\xE1W__\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x12\xF7W__\xFD[\x93\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x85\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16` \x82\x01R`\xA0`@\x82\x01\x81\x90R_\x90a\x13U\x90\x83\x01\x86a\x12\xFEV[`\x01`\x01`\xA0\x1B\x03\x94\x85\x16``\x84\x01R\x92\x90\x93\x16`\x80\x90\x91\x01R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x13\x87W__\xFD[P5\x91\x90PV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x141W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q\x86R`\x01\x80`\xA0\x1B\x03` \x82\x01Q\x16` \x87\x01R`@\x81\x01Q`\xA0`@\x88\x01Ra\x13\xF6`\xA0\x88\x01\x82a\x12\xFEV[``\x83\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x8A\x01\x91\x90\x91R`\x80\x93\x84\x01Q\x16\x92\x90\x97\x01\x91\x90\x91RP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x13\xB4V[P\x92\x96\x95PPPPPPV[__` \x83\x85\x03\x12\x15a\x14NW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14dW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x14tW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\x8AW__\xFD[\x85` \x82\x84\x01\x01\x11\x15a\x14\x9BW__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x14\xCFW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xE5W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x14\xF5W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\x0FWa\x15\x0Fa\x14\xABV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x15>Wa\x15>a\x14\xABV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x15UW__\xFD[\x81` \x84\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x15\x86W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x08iWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x15\xCBWa\x15\xCBa\x15\xA4V[\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x15\xCBWa\x15\xCBa\x15\xA4V[`\x1F\x82\x11\x15a\x05\x82W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x16,WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x16KW_\x81U`\x01\x01a\x168V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16lWa\x16la\x14\xABV[a\x16\x80\x81a\x16z\x84Ta\x15rV[\x84a\x16\x07V[` `\x1F\x82\x11`\x01\x81\x14a\x16\xB5W_\x83\x15a\x16\x9BWP\x84\x82\x01Q[`\x01\x84\x90\x1B_\x19`\x03\x86\x90\x1B\x1C\x19\x82\x16\x17[\x85UPa\x16KV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x16\xE4W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x16\xC4V[P\x84\x82\x10\x15a\x17\x01W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x81\x03a\x17\x1BWPPV[a\x17%\x82Ta\x15rV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17=Wa\x17=a\x14\xABV[a\x17K\x81a\x16z\x84Ta\x15rV[_`\x1F\x82\x11`\x01\x81\x14a\x17zW_\x83\x15a\x16\x9BWP\x81\x85\x01T`\x01\x84\x90\x1B_\x19`\x03\x86\x90\x1B\x1C\x19\x82\x16\x17a\x16\xADV[_\x85\x81R` \x80\x82 \x86\x83R\x90\x82 `\x1F\x19\x86\x16\x92[\x83\x81\x10\x15a\x17\xB0W\x82\x86\x01T\x82U`\x01\x95\x86\x01\x95\x90\x91\x01\x90` \x01a\x17\x90V[P\x85\x83\x10\x15a\x17\xCDW\x81\x85\x01T_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[_`\x01\x82\x01a\x17\xEEWa\x17\xEEa\x15\xA4V[P`\x01\x01\x90V[_` \x82\x84\x03\x12\x15a\x18\x05W__\xFD[PQ\x91\x90PV[` \x81R\x81T` \x82\x01R`\x01\x80`\xA0\x1B\x03`\x01\x83\x01T\x16`@\x82\x01R_`\x02\x83\x01`\xA0``\x84\x01R_\x81Ta\x18A\x81a\x15rV[\x80`\xC0\x87\x01R`\x01\x82\x16_\x81\x14a\x18_W`\x01\x81\x14a\x18{Wa\x18\xACV[`\xFF\x19\x83\x16`\xE0\x88\x01R`\xE0\x82\x15\x15`\x05\x1B\x88\x01\x01\x93Pa\x18\xACV[\x84_R` _ _[\x83\x81\x10\x15a\x18\xA3W\x81T\x89\x82\x01`\xE0\x01R`\x01\x90\x91\x01\x90` \x01a\x18\x84V[\x88\x01`\xE0\x01\x94PP[PPP`\x03\x85\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x86\x01R`\x04\x90\x95\x01T\x90\x94\x16`\xA0\x90\x93\x01\x92\x90\x92RP\x90\x91\x90PV[__\x83Ta\x18\xEA\x81a\x15rV[`\x01\x82\x16\x80\x15a\x19\x01W`\x01\x81\x14a\x19\x16Wa\x19CV[`\xFF\x19\x83\x16\x86R\x81\x15\x15\x82\x02\x86\x01\x93Pa\x19CV[\x86_R` _ _[\x83\x81\x10\x15a\x19;W\x81T\x88\x82\x01R`\x01\x90\x91\x01\x90` \x01a\x19\x1FV[PP\x81\x86\x01\x93P[P\x91\x95\x94PPPPPV[_\x81a\x19\\Wa\x19\\a\x15\xA4V[P_\x19\x01\x90V[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x19\x82Wa\x19\x82a\x15\xA4V[P\x92\x91PPV\xFE\xA2dipfsX\"\x12 N8\x87\xEC\xD0\x13>\xCEzmm\xC8mL\xAA:\xBC\x9D%\xCF(\xC9\x1CD\xA9\x87\xC2\x80$\x16\x9C\x8DdsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static ROUNDROBIN_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xFAW_5`\xE0\x1C\x80cu\x0F%\xF4\x11a\0\x92W\x80c\xC1/fn\x11a\0bW\x80c\xC1/fn\x14a\x02\x95W\x80c\xC79\xD7\x9C\x14a\x02\xA9W\x80c\xD1\xCE\xF1\xEE\x14a\x02\xD4W\x80c\xD7\xAEv\xB6\x14a\x03\x1EW\x80c\xE2\xFD\xCC\x17\x14a\x03QW__\xFD[\x80cu\x0F%\xF4\x14a\x02\x11W\x80c\x97\xC1\xD0G\x14a\x028W\x80c\xA8@J\xEE\x14a\x02kW\x80c\xAC+\xEC\xA0\x14a\x02\x80W__\xFD[\x80c.\x1A}M\x11a\0\xCDW\x80c.\x1A}M\x14a\x01\xAAW\x80cH\xC0\xF4\x87\x14a\x01\xC9W\x80cK\x98\xB3\xB6\x14a\x01\xEAW\x80cU\xC2]\x17\x14a\x01\xFEW__\xFD[\x80c\x12_\xDB\xBC\x14a\0\xFEW\x80c\x18\x17t\x97\x14a\x01DW\x80c\x19Okd\x14a\x01tW\x80c\x1F\xFF\xF6\x98\x14a\x01\x95W[__\xFD[4\x80\x15a\x01\tW__\xFD[Pa\x011\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01OW__\xFD[Pa\x01ca\x01^6`\x04a\x12\xD1V[a\x03fV[`@Qa\x01;\x95\x94\x93\x92\x91\x90a\x13,V[4\x80\x15a\x01\x7FW__\xFD[Pa\x01\x93a\x01\x8E6`\x04a\x12\xD1V[a\x041V[\0[4\x80\x15a\x01\xA0W__\xFD[Pa\x011`\nT\x81V[4\x80\x15a\x01\xB5W__\xFD[Pa\x01\x93a\x01\xC46`\x04a\x13wV[a\x04\xC0V[4\x80\x15a\x01\xD4W__\xFD[Pa\x01\xDDa\x05\x87V[`@Qa\x01;\x91\x90a\x13\x8EV[4\x80\x15a\x01\xF5W__\xFD[Pa\x01\x93a\x08oV[a\x01\x93a\x02\x0C6`\x04a\x14=V[a\x08zV[4\x80\x15a\x02\x1CW__\xFD[P`\x04T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01;V[4\x80\x15a\x02CW__\xFD[Pa\x011\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02vW__\xFD[Pa\x011`\tT\x81V[4\x80\x15a\x02\x8BW__\xFD[Pa\x011`\x08T\x81V[4\x80\x15a\x02\xA0W__\xFD[Pa\x01\x93a\x0C|V[4\x80\x15a\x02\xB4W__\xFD[Pa\x011a\x02\xC36`\x04a\x12\xD1V[`\x01` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x02\xDFW__\xFD[Pa\x03\x0Ea\x02\xEE6`\x04a\x14\xBFV[\x80Q` \x81\x83\x01\x81\x01\x80Q`\x02\x82R\x92\x82\x01\x91\x90\x93\x01 \x91RT`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01;V[4\x80\x15a\x03)W__\xFD[Pa\x011\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\\W__\xFD[Pa\x011`\x0BT\x81V[_` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x83\x01\x80T\x92\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92a\x03\x97\x90a\x15rV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03\xC3\x90a\x15rV[\x80\x15a\x04\x0EW\x80`\x1F\x10a\x03\xE5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04\x0EV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\xF1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPP`\x03\x83\x01T`\x04\x90\x93\x01T\x91\x92`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x92\x16\x90P\x85V[`\rT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FOnly the owner can call this fun`D\x82\x01Rd1\xBA4\xB7\xB7`\xD9\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\x0C\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[3_\x90\x81R`\x01` R`@\x90 T\x80\x82\x11\x15a\x055W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FYou are trying to withdraw more `D\x82\x01Rlthan you have`\x98\x1B`d\x82\x01R`\x84\x01a\x04\x95V[3_\x90\x81R`\x01` R`@\x81 \x80T\x84\x92\x90a\x05S\x90\x84\x90a\x15\xB8V[\x90\x91UPP`@Q3\x90\x83\x15a\x08\xFC\x02\x90\x84\x90_\x81\x81\x81\x85\x88\x88\xF1\x93PPPP\x15\x80\x15a\x05\x82W=__>=_\xFD[PPPV[``_`\nTg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x05\xA5Wa\x05\xA5a\x14\xABV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\xFDW\x81` \x01[`@\x80Q`\xA0\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R``\x93\x83\x01\x84\x90R\x92\x82\x01\x81\x90R`\x80\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x05\xC3W\x90P[P`@\x80Q`\xA0\x81\x01\x82R`\x03\x80T\x82R`\x04T`\x01`\x01`\xA0\x1B\x03\x16` \x83\x01R`\x05\x80T\x94\x95P\x91\x93\x90\x92\x84\x01\x91\x90a\x067\x90a\x15rV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06c\x90a\x15rV[\x80\x15a\x06\xAEW\x80`\x1F\x10a\x06\x85Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06\xAEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\x91W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R`\x04\x90\x92\x01T\x90\x91\x16`@\x90\x91\x01R\x81Q\x82\x90_\x90a\x06\xECWa\x06\xECa\x15\xD1V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01[`\nT\x81\x10\x15a\x08iW_\x80\x83a\x07\x12`\x01\x85a\x15\xB8V[\x81Q\x81\x10a\x07\"Wa\x07\"a\x15\xD1V[` \x02` \x01\x01Q``\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`\xA0\x01`@R\x90\x81_\x82\x01T\x81R` \x01`\x01\x82\x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\x02\x82\x01\x80Ta\x07\x9F\x90a\x15rV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x07\xCB\x90a\x15rV[\x80\x15a\x08\x16W\x80`\x1F\x10a\x07\xEDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x16V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x07\xF9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16` \x83\x01R`\x04\x90\x92\x01T\x90\x91\x16`@\x90\x91\x01R\x82Q\x83\x90\x83\x90\x81\x10a\x08VWa\x08Va\x15\xD1V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x06\xFAV[P\x91\x90PV[a\x08x3a\x0E\x9FV[V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x004\x14a\x08\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FYou have not paid the correct st`D\x82\x01Rk\x18Z\xDA[\x99\xC8\x18[[\xDD[\x9D`\xA2\x1B`d\x82\x01R`\x84\x01a\x04\x95V[3_\x90\x81R` \x81\x90R`@\x90 `\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x15a\tfW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FYou are already a proposer\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x95V[`\x02\x82\x82`@Qa\tx\x92\x91\x90a\x15\xE5V[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 T`\xFF\x16\x15a\t\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FThis proposer URL is already in `D\x82\x01Rbuse`\xE8\x1B`d\x82\x01R`\x84\x01a\x04\x95V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x0B_\x82\x82Ta\n\x15\x91\x90a\x15\xF4V[\x90\x91UPP`\x04T`\x07T`\x06T`@\x80Q`\xA0\x81\x01\x82R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3` \x80\x83\x01\x91\x90\x91R\x82Q`\x1F\x88\x01\x82\x90\x04\x82\x02\x81\x01\x82\x01\x84R\x87\x81R`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x96\x95\x86\x16\x95\x90\x94\x16\x93\x91\x92\x83\x01\x91\x90\x88\x90\x88\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847_\x92\x01\x82\x90RP\x93\x85RPPP`\x01`\x01`\xA0\x1B\x03\x86\x81\x16` \x80\x85\x01\x91\x90\x91R\x86\x82\x16`@\x94\x85\x01R3\x83R\x82\x81R\x91\x83\x90 \x84Q\x81U\x91\x84\x01Q`\x01\x83\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x92\x16\x17\x90U\x90\x82\x01Q`\x02\x82\x01\x90a\n\xFC\x90\x82a\x16RV[P``\x82\x01Q`\x03\x82\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90\x91U`\x80\x90\x94\x01Q`\x04\x93\x84\x01\x80T\x86\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x86\x82\x16_\x81\x81R` \x81\x90R`@\x80\x82 \x90\x95\x01\x80T3\x90\x88\x16\x81\x17\x90\x91U\x88\x85\x16\x82R\x94\x90 \x90\x91\x01\x80T\x90\x94\x16\x90\x92\x17\x90\x92U\x90\x82\x16\x03a\x0B\xA4W`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R` \x81\x90R`@\x90 `\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U[`\x01`\x02\x86\x86`@Qa\x0B\xB8\x92\x91\x90a\x15\xE5V[\x90\x81R`@\x80Q` \x92\x81\x90\x03\x83\x01\x90 \x80T`\xFF\x19\x16\x93\x15\x15\x93\x90\x93\x17\x90\x92U`\x01`\x01`\xA0\x1B\x03\x85\x81\x16_\x90\x81R\x91\x82\x90R\x91\x90 \x80T`\x03\x90\x81U`\x01\x82\x01T`\x04\x80T\x91\x90\x94\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x17\x90\x92U\x90`\x05a\x0C#`\x02\x84\x01\x82a\x17\x10V[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x93\x90\x92\x01\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90U`\n\x80T\x90_a\x0Cp\x83a\x17\xDDV[\x91\x90PUPPPPPPV[a\x0C\x84a\x11\xE9V[a\x0C\xDEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FIt is not time to rotate the pro`D\x82\x01Rd87\xB9\xB2\xB9`\xD9\x1B`d\x82\x01R`\x84\x01a\x04\x95V[`\tT`\x0C_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c(\xC3\xD7\xE6`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rU\x91\x90a\x17\xF5V[\x03a\roW`\x04Ta\ro\x90`\x01`\x01`\xA0\x1B\x03\x16a\x12 V[`\x06T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R` \x81\x90R`@\x90 \x80T`\x03\x90\x81U`\x01\x82\x01T`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x94\x16\x17\x90\x92U\x90`\x05a\r\xBC`\x02\x84\x01\x82a\x17\x10V[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x92\x84\x01\x80T\x90\x91\x16\x92\x82\x16\x92\x90\x92\x17\x90\x91UC`\x08U`\x0CT`@\x80Qc\x14a\xEB\xF3`\xE1\x1B\x81R\x90Q\x91\x90\x92\x16\x92c(\xC3\xD7\xE6\x92\x80\x82\x01\x92` \x92\x90\x91\x82\x90\x03\x01\x81\x86Z\xFA\x15\x80\x15a\x0E=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ea\x91\x90a\x17\xF5V[`\tU`@Q\x7F\xAC;\x1Ci)\xA3\xB1\xB5)\xEA\xCCFr\x0F\x90\xC2\xD5K\x83CE\xDC\xA9\xBA}z\x94\x83\xE0\xDER\xBE\x90a\x0E\x95\x90`\x03\x90a\x18\x0CV[`@Q\x80\x91\x03\x90\xA1V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x81\x81R` \x81\x90R`@\x90 `\x01\x01T\x90\x91\x16\x14a\x0F\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FThis proposer does not exist\0\0\0\0`D\x82\x01R`d\x01a\x04\x95V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0FmW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FThe proposer address cannot be z`D\x82\x01Rbero`\xE8\x1B`d\x82\x01R`\x84\x01a\x04\x95V[`\x04T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x82\x16\x03a\x0F\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FYou cannot remove the current pr`D\x82\x01Re7\xB87\xB9\xB2\xB9`\xD1\x1B`d\x82\x01R`\x84\x01a\x04\x95V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R` \x81\x90R`@\x80\x82 `\x03\x80\x82\x01\x80T\x86\x16\x85R\x83\x85 `\x04\x80\x85\x01T\x88\x16\x80\x88R\x95\x87 \x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x97\x17\x90U\x91T\x92\x82\x01\x80T\x90\x95\x16\x92\x90\x96\x16\x91\x90\x91\x17\x90\x92U\x80T`\x0B\x80T\x92\x95\x94\x91\x92\x90\x91\x90a\x10S\x90\x84\x90a\x15\xB8V[\x90\x91UPP\x82T`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x01` R`@\x81 \x80T\x90\x91\x90a\x10\x81\x90\x84\x90a\x15\xF4V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R` \x81\x90R`@\x81 \x81\x81U`\x01\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x90a\x10\xC0`\x02\x83\x01\x82a\x12\x7FV[P`\x03\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x91U`\x04\x90\x91\x01\x80T\x90\x91\x16\x90U`@Q`\x02\x90a\x10\xF5\x90\x85\x83\x01\x90a\x18\xDDV[\x90\x81R`@Q\x90\x81\x90\x03` \x01\x90 \x80T`\xFF\x19\x16\x90U`\n\x80T\x90_a\x11\x1B\x83a\x19NV[\x91\x90PUP`\nT`\x01\x03a\x11\xE3W`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R` \x81\x90R`@\x80\x82 `\x03\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x90\x95\x17\x90U\x85T\x85\x16\x80\x84R\x82\x84 \x87\x01\x80T\x86\x16\x90\x91\x17\x90U\x85T\x80\x86\x16\x84R\x91\x90\x92 \x80T\x83U`\x01\x81\x01T\x91\x90\x93\x16\x93\x16\x92\x90\x92\x17\x90\x92U`\x05a\x11\xA4`\x02\x84\x01\x82a\x17\x10V[P`\x03\x82\x81\x01T\x90\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\x04\x93\x84\x01T\x93\x90\x92\x01\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90U[PPPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x08Ta\x12\x18\x91\x90a\x15\xF4V[C\x10\x15\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R` \x81\x90R`@\x81 \x80T\x90\x91\x90a\x12g\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x19cV[\x90P_\x81\x12\x15a\x12zWa\x05\x82\x83a\x0E\x9FV[\x90UPV[P\x80Ta\x12\x8B\x90a\x15rV[_\x82U\x80`\x1F\x10a\x12\x9AWPPV[`\x1F\x01` \x90\x04\x90_R` _ \x90\x81\x01\x90a\x12\xB6\x91\x90a\x12\xB9V[PV[[\x80\x82\x11\x15a\x12\xCDW_\x81U`\x01\x01a\x12\xBAV[P\x90V[_` \x82\x84\x03\x12\x15a\x12\xE1W__\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x12\xF7W__\xFD[\x93\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x85\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16` \x82\x01R`\xA0`@\x82\x01\x81\x90R_\x90a\x13U\x90\x83\x01\x86a\x12\xFEV[`\x01`\x01`\xA0\x1B\x03\x94\x85\x16``\x84\x01R\x92\x90\x93\x16`\x80\x90\x91\x01R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a\x13\x87W__\xFD[P5\x91\x90PV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\x141W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q\x86R`\x01\x80`\xA0\x1B\x03` \x82\x01Q\x16` \x87\x01R`@\x81\x01Q`\xA0`@\x88\x01Ra\x13\xF6`\xA0\x88\x01\x82a\x12\xFEV[``\x83\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x91\x8A\x01\x91\x90\x91R`\x80\x93\x84\x01Q\x16\x92\x90\x97\x01\x91\x90\x91RP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\x13\xB4V[P\x92\x96\x95PPPPPPV[__` \x83\x85\x03\x12\x15a\x14NW__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14dW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x14tW__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\x8AW__\xFD[\x85` \x82\x84\x01\x01\x11\x15a\x14\x9BW__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\x14\xCFW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x14\xE5W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x14\xF5W__\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\x0FWa\x15\x0Fa\x14\xABV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x15>Wa\x15>a\x14\xABV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x15UW__\xFD[\x81` \x84\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x15\x86W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x08iWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x15\xCBWa\x15\xCBa\x15\xA4V[\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x15\xCBWa\x15\xCBa\x15\xA4V[`\x1F\x82\x11\x15a\x05\x82W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x16,WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x16KW_\x81U`\x01\x01a\x168V[PPPPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16lWa\x16la\x14\xABV[a\x16\x80\x81a\x16z\x84Ta\x15rV[\x84a\x16\x07V[` `\x1F\x82\x11`\x01\x81\x14a\x16\xB5W_\x83\x15a\x16\x9BWP\x84\x82\x01Q[`\x01\x84\x90\x1B_\x19`\x03\x86\x90\x1B\x1C\x19\x82\x16\x17[\x85UPa\x16KV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x16\xE4W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x16\xC4V[P\x84\x82\x10\x15a\x17\x01W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[\x81\x81\x03a\x17\x1BWPPV[a\x17%\x82Ta\x15rV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x17=Wa\x17=a\x14\xABV[a\x17K\x81a\x16z\x84Ta\x15rV[_`\x1F\x82\x11`\x01\x81\x14a\x17zW_\x83\x15a\x16\x9BWP\x81\x85\x01T`\x01\x84\x90\x1B_\x19`\x03\x86\x90\x1B\x1C\x19\x82\x16\x17a\x16\xADV[_\x85\x81R` \x80\x82 \x86\x83R\x90\x82 `\x1F\x19\x86\x16\x92[\x83\x81\x10\x15a\x17\xB0W\x82\x86\x01T\x82U`\x01\x95\x86\x01\x95\x90\x91\x01\x90` \x01a\x17\x90V[P\x85\x83\x10\x15a\x17\xCDW\x81\x85\x01T_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[_`\x01\x82\x01a\x17\xEEWa\x17\xEEa\x15\xA4V[P`\x01\x01\x90V[_` \x82\x84\x03\x12\x15a\x18\x05W__\xFD[PQ\x91\x90PV[` \x81R\x81T` \x82\x01R`\x01\x80`\xA0\x1B\x03`\x01\x83\x01T\x16`@\x82\x01R_`\x02\x83\x01`\xA0``\x84\x01R_\x81Ta\x18A\x81a\x15rV[\x80`\xC0\x87\x01R`\x01\x82\x16_\x81\x14a\x18_W`\x01\x81\x14a\x18{Wa\x18\xACV[`\xFF\x19\x83\x16`\xE0\x88\x01R`\xE0\x82\x15\x15`\x05\x1B\x88\x01\x01\x93Pa\x18\xACV[\x84_R` _ _[\x83\x81\x10\x15a\x18\xA3W\x81T\x89\x82\x01`\xE0\x01R`\x01\x90\x91\x01\x90` \x01a\x18\x84V[\x88\x01`\xE0\x01\x94PP[PPP`\x03\x85\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x86\x01R`\x04\x90\x95\x01T\x90\x94\x16`\xA0\x90\x93\x01\x92\x90\x92RP\x90\x91\x90PV[__\x83Ta\x18\xEA\x81a\x15rV[`\x01\x82\x16\x80\x15a\x19\x01W`\x01\x81\x14a\x19\x16Wa\x19CV[`\xFF\x19\x83\x16\x86R\x81\x15\x15\x82\x02\x86\x01\x93Pa\x19CV[\x86_R` _ _[\x83\x81\x10\x15a\x19;W\x81T\x88\x82\x01R`\x01\x90\x91\x01\x90` \x01a\x19\x1FV[PP\x81\x86\x01\x93P[P\x91\x95\x94PPPPPV[_\x81a\x19\\Wa\x19\\a\x15\xA4V[P_\x19\x01\x90V[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x19\x82Wa\x19\x82a\x15\xA4V[P\x92\x91PPV\xFE\xA2dipfsX\"\x12 N8\x87\xEC\xD0\x13>\xCEzmm\xC8mL\xAA:\xBC\x9D%\xCF(\xC9\x1CD\xA9\x87\xC2\x80$\x16\x9C\x8DdsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static ROUNDROBIN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RoundRobin<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RoundRobin<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RoundRobin<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RoundRobin<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RoundRobin<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RoundRobin)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RoundRobin<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ROUNDROBIN_ABI.clone(),
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
                ROUNDROBIN_ABI.clone(),
                ROUNDROBIN_BYTECODE.clone().into(),
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
        ///Calls the contract's `ROTATION_BlOCKS` (0x97c1d047) function
        pub fn rotation_bl_ocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([151, 193, 208, 71], ())
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
        ///Calls the contract's `add_proposer` (0x55c25d17) function
        pub fn add_proposer(
            &self,
            proposer_url: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 194, 93, 23], proposer_url)
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
        ///Calls the contract's `withdraw` (0x2e1a7d4d) function
        pub fn withdraw(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], amount)
                .expect("method not found (this should never happen)")
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
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposerRotatedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RoundRobin<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
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
    #[ethevent(
        name = "ProposerRotated",
        abi = "ProposerRotated((uint256,address,string,address,address))"
    )]
    pub struct ProposerRotatedFilter {
        pub proposer: Proposer,
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
    ///Container type for all input parameters for the `ROTATION_BlOCKS` function with signature `ROTATION_BlOCKS()` and selector `0x97c1d047`
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
    #[ethcall(name = "ROTATION_BlOCKS", abi = "ROTATION_BlOCKS()")]
    pub struct RotationBlOCKSCall;
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
    pub enum RoundRobinCalls {
        CooldownBlocks(CooldownBlocksCall),
        Ding(DingCall),
        ExitPenalty(ExitPenaltyCall),
        RotationBlOCKS(RotationBlOCKSCall),
        Stake(StakeCall),
        AddProposer(AddProposerCall),
        Escrow(EscrowCall),
        GetCurrentProposerAddress(GetCurrentProposerAddressCall),
        GetProposers(GetProposersCall),
        LastExitBlock(LastExitBlockCall),
        Owner(OwnerCall),
        PendingWithdraws(PendingWithdrawsCall),
        ProposerCount(ProposerCountCall),
        ProposerUrls(ProposerUrlsCall),
        Proposers(ProposersCall),
        RemoveProposer(RemoveProposerCall),
        RotateProposer(RotateProposerCall),
        SetAuthorities(SetAuthoritiesCall),
        SetNightfall(SetNightfallCall),
        SetSanctionsList(SetSanctionsListCall),
        SetX509Address(SetX509AddressCall),
        StartL1Block(StartL1BlockCall),
        StartL2Block(StartL2BlockCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for RoundRobinCalls {
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
            if let Ok(decoded) = <RotationBlOCKSCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RotationBlOCKS(decoded));
            }
            if let Ok(decoded) = <StakeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Stake(decoded));
            }
            if let Ok(decoded) = <AddProposerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AddProposer(decoded));
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
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RoundRobinCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CooldownBlocks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ding(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExitPenalty(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RotationBlOCKS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Stake(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddProposer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Escrow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCurrentProposerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetProposers(element) => {
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
                Self::RemoveProposer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RotateProposer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAuthorities(element) => {
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
                Self::Withdraw(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RoundRobinCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CooldownBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ding(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExitPenalty(element) => ::core::fmt::Display::fmt(element, f),
                Self::RotationBlOCKS(element) => ::core::fmt::Display::fmt(element, f),
                Self::Stake(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddProposer(element) => ::core::fmt::Display::fmt(element, f),
                Self::Escrow(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCurrentProposerAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetProposers(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastExitBlock(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingWithdraws(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposerCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposerUrls(element) => ::core::fmt::Display::fmt(element, f),
                Self::Proposers(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveProposer(element) => ::core::fmt::Display::fmt(element, f),
                Self::RotateProposer(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAuthorities(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNightfall(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetSanctionsList(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetX509Address(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartL1Block(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartL2Block(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CooldownBlocksCall> for RoundRobinCalls {
        fn from(value: CooldownBlocksCall) -> Self {
            Self::CooldownBlocks(value)
        }
    }
    impl ::core::convert::From<DingCall> for RoundRobinCalls {
        fn from(value: DingCall) -> Self {
            Self::Ding(value)
        }
    }
    impl ::core::convert::From<ExitPenaltyCall> for RoundRobinCalls {
        fn from(value: ExitPenaltyCall) -> Self {
            Self::ExitPenalty(value)
        }
    }
    impl ::core::convert::From<RotationBlOCKSCall> for RoundRobinCalls {
        fn from(value: RotationBlOCKSCall) -> Self {
            Self::RotationBlOCKS(value)
        }
    }
    impl ::core::convert::From<StakeCall> for RoundRobinCalls {
        fn from(value: StakeCall) -> Self {
            Self::Stake(value)
        }
    }
    impl ::core::convert::From<AddProposerCall> for RoundRobinCalls {
        fn from(value: AddProposerCall) -> Self {
            Self::AddProposer(value)
        }
    }
    impl ::core::convert::From<EscrowCall> for RoundRobinCalls {
        fn from(value: EscrowCall) -> Self {
            Self::Escrow(value)
        }
    }
    impl ::core::convert::From<GetCurrentProposerAddressCall> for RoundRobinCalls {
        fn from(value: GetCurrentProposerAddressCall) -> Self {
            Self::GetCurrentProposerAddress(value)
        }
    }
    impl ::core::convert::From<GetProposersCall> for RoundRobinCalls {
        fn from(value: GetProposersCall) -> Self {
            Self::GetProposers(value)
        }
    }
    impl ::core::convert::From<LastExitBlockCall> for RoundRobinCalls {
        fn from(value: LastExitBlockCall) -> Self {
            Self::LastExitBlock(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for RoundRobinCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PendingWithdrawsCall> for RoundRobinCalls {
        fn from(value: PendingWithdrawsCall) -> Self {
            Self::PendingWithdraws(value)
        }
    }
    impl ::core::convert::From<ProposerCountCall> for RoundRobinCalls {
        fn from(value: ProposerCountCall) -> Self {
            Self::ProposerCount(value)
        }
    }
    impl ::core::convert::From<ProposerUrlsCall> for RoundRobinCalls {
        fn from(value: ProposerUrlsCall) -> Self {
            Self::ProposerUrls(value)
        }
    }
    impl ::core::convert::From<ProposersCall> for RoundRobinCalls {
        fn from(value: ProposersCall) -> Self {
            Self::Proposers(value)
        }
    }
    impl ::core::convert::From<RemoveProposerCall> for RoundRobinCalls {
        fn from(value: RemoveProposerCall) -> Self {
            Self::RemoveProposer(value)
        }
    }
    impl ::core::convert::From<RotateProposerCall> for RoundRobinCalls {
        fn from(value: RotateProposerCall) -> Self {
            Self::RotateProposer(value)
        }
    }
    impl ::core::convert::From<SetAuthoritiesCall> for RoundRobinCalls {
        fn from(value: SetAuthoritiesCall) -> Self {
            Self::SetAuthorities(value)
        }
    }
    impl ::core::convert::From<SetNightfallCall> for RoundRobinCalls {
        fn from(value: SetNightfallCall) -> Self {
            Self::SetNightfall(value)
        }
    }
    impl ::core::convert::From<SetSanctionsListCall> for RoundRobinCalls {
        fn from(value: SetSanctionsListCall) -> Self {
            Self::SetSanctionsList(value)
        }
    }
    impl ::core::convert::From<SetX509AddressCall> for RoundRobinCalls {
        fn from(value: SetX509AddressCall) -> Self {
            Self::SetX509Address(value)
        }
    }
    impl ::core::convert::From<StartL1BlockCall> for RoundRobinCalls {
        fn from(value: StartL1BlockCall) -> Self {
            Self::StartL1Block(value)
        }
    }
    impl ::core::convert::From<StartL2BlockCall> for RoundRobinCalls {
        fn from(value: StartL2BlockCall) -> Self {
            Self::StartL2Block(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for RoundRobinCalls {
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
    ///Container type for all return fields from the `ROTATION_BlOCKS` function with signature `ROTATION_BlOCKS()` and selector `0x97c1d047`
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
    pub struct RotationBlOCKSReturn(pub ::ethers::core::types::U256);
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
}
