pub use rollup_proof_verification_key::*;
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
pub mod rollup_proof_verification_key {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
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
                    ::std::borrow::ToOwned::to_owned("getVerificationKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getVerificationKey"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct Types.VerificationKey",
                                        ),
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
                                    name: ::std::borrow::ToOwned::to_owned("vkBlob"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("replaceVK"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("replaceVK"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vkBlob"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("vkHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vkHash"),
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
                    ::std::borrow::ToOwned::to_owned("vkVersion"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("vkVersion"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                (
                    ::std::borrow::ToOwned::to_owned("VKInitialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("VKInitialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("vkHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
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
                    ::std::borrow::ToOwned::to_owned("VKReplaced"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("VKReplaced"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newVersion"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
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
                    ::std::borrow::ToOwned::to_owned("OwnableInvalidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableInvalidOwner",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("OwnableUnauthorizedAccount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnableUnauthorizedAccount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
    pub static ROLLUPPROOFVERIFICATIONKEY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R0`\x80R4\x80\x15`\x12W__\xFD[P`\x80Qa!ia\09_9_\x81\x81a\x0Ek\x01R\x81\x81a\x0E\x94\x01Ra\x0F\xD8\x01Ra!i_\xF3\xFE`\x80`@R`\x046\x10a\0\x9AW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0bW\x80c\x8D\xA5\xCB[\x14a\x01\x1DW\x80c\x9EL\xC7\xED\x14a\x01cW\x80c\xAD<\xB1\xCC\x14a\x01\x8AW\x80c\xDF\xC4\xCDN\x14a\x01\xC7W\x80c\xE2?\xF9\xA1\x14a\x01\xE8W\x80c\xF2\xFD\xE3\x8B\x14a\x02\x07W__\xFD[\x80cC\x9F\xAB\x91\x14a\0\x9EW\x80cO\x1E\xF2\x86\x14a\0\xBFW\x80cO\xE8@\xF5\x14a\0\xD2W\x80cR\xD1\x90-\x14a\0\xF5W\x80cqP\x18\xA6\x14a\x01\tW[__\xFD[4\x80\x15a\0\xA9W__\xFD[Pa\0\xBDa\0\xB86`\x04a\x17HV[a\x02&V[\0[a\0\xBDa\0\xCD6`\x04a\x18<V[a\x06wV[4\x80\x15a\0\xDDW__\xFD[P`MT[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\0W__\xFD[Pa\0\xE2a\x06\x96V[4\x80\x15a\x01\x14W__\xFD[Pa\0\xBDa\x06\xB1V[4\x80\x15a\x01(W__\xFD[P\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xECV[4\x80\x15a\x01nW__\xFD[P`NT`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\0\xECV[4\x80\x15a\x01\x95W__\xFD[Pa\x01\xBA`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\0\xEC\x91\x90a\x18\xDFV[4\x80\x15a\x01\xD2W__\xFD[Pa\x01\xDBa\x06\xC4V[`@Qa\0\xEC\x91\x90a\x19\x14V[4\x80\x15a\x01\xF3W__\xFD[Pa\0\xBDa\x02\x026`\x04a\x17HV[a\njV[4\x80\x15a\x02\x12W__\xFD[Pa\0\xBDa\x02!6`\x04a\x1C\xE8V[a\r\xEFV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16_\x81\x15\x80\x15a\x02jWP\x82[\x90P_\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x02\x85WP0;\x15[\x90P\x81\x15\x80\x15a\x02\x93WP\x80\x15[\x15a\x02\xB1W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x02\xDBW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[_\x86\x90\x03a\x03\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01RhVK: empty`\xB8\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x03%3a\x0E,V[a\x03-a\x0E=V[_a\x038\x88\x88a\x0EEV[\x80Q_U` \x80\x82\x01Q`\x01U`@\x80\x83\x01Q\x80Q`\x02U\x82\x01Q`\x03U``\x80\x84\x01Q\x80Q`\x04U\x83\x01Q`\x05U`\x80\x84\x01Q\x80Q`\x06U\x83\x01Q`\x07U`\xA0\x84\x01Q\x80Q`\x08U\x83\x01Q`\tU`\xC0\x84\x01Q\x80Q`\nU\x83\x01Q`\x0BU`\xE0\x84\x01Q\x80Q`\x0CU\x83\x01Q`\rUa\x01\0\x84\x01Q\x80Q`\x0EU\x83\x01Q`\x0FUa\x01 \x84\x01Q\x80Q`\x10U\x83\x01Q`\x11Ua\x01@\x84\x01Q\x80Q`\x12U\x83\x01Q`\x13Ua\x01`\x84\x01Q\x80Q`\x14U\x83\x01Q`\x15Ua\x01\x80\x84\x01Q\x80Q`\x16U\x83\x01Q`\x17Ua\x01\xA0\x84\x01Q\x80Q`\x18U\x83\x01Q`\x19Ua\x01\xC0\x84\x01Q\x80Q`\x1AU\x83\x01Q`\x1BUa\x01\xE0\x84\x01Q\x80Q`\x1CU\x83\x01Q`\x1DUa\x02\0\x84\x01Q\x80Q`\x1EU\x83\x01Q`\x1FUa\x02 \x84\x01Q\x80Q\x84U\x83\x01Q`!Ua\x02@\x84\x01Q\x80Q`\"U\x83\x01Q`#Ua\x02`\x84\x01Q\x80Q`$U\x83\x01Q`%Ua\x02\x80\x84\x01Q\x80Q`&U\x83\x01Q`'Ua\x02\xA0\x84\x01Q\x80Q`(U\x83\x01Q`)Ua\x02\xC0\x84\x01Q\x80Q`*U\x83\x01Q`+Ua\x02\xE0\x84\x01Q\x80Q`,U\x83\x01Q`-Ua\x03\0\x84\x01Q\x80Q`.U\x83\x01Q`/Ua\x03 \x84\x01Q\x80Q`0U\x83\x01Q`1Ua\x03@\x84\x01Q`2Ua\x03`\x84\x01Q`3Ua\x03\x80\x84\x01Q`4Ua\x03\xA0\x84\x01Q`5Ua\x03\xC0\x84\x01Q`6Ua\x03\xE0\x84\x01Q`7Ua\x04\0\x84\x01Q\x80Q`8U\x83\x01Q`9Ua\x04 \x84\x01Q\x80Q`:U\x83\x01Q`;Ua\x04@\x84\x01Q\x80Q`<U\x83\x01Q`=Ua\x04`\x84\x01Q\x80Q`>U\x83\x01Q`?Ua\x04\x80\x84\x01Q\x82Ua\x04\xA0\x84\x01Q`AUa\x04\xC0\x84\x01Q`BUa\x04\xE0\x84\x01Q\x80Q`CU\x83\x01Q`DUa\x05\0\x84\x01Q\x80Q`EU\x80\x84\x01Q`FU\x80\x83\x01Q`GU\x81\x01Q`HUa\x05 \x84\x01Q\x80Q`IU\x92\x83\x01Q`JU\x82\x82\x01Q`KU\x91\x90\x91\x01Q`LUQ\x90\x91Pa\x05\xCE\x90\x89\x90\x89\x90a\x1D\x01V[`@\x80Q\x91\x82\x90\x03\x82 `M\x81\x90U`N\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x90\x83R` \x83\x01R\x7F\xF0l\x01a+\x88vy\x01\x07\xCC`[\\\x9A\xC9\x93\x08\xD73\xDD\xD8\xFB\x03-\xE7f\xA7\n\xCD\xCDY\x91\x01`@Q\x80\x91\x03\x90\xA1P\x83\x15a\x06nW\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[a\x06\x7Fa\x0E`V[a\x06\x88\x82a\x0F\x04V[a\x06\x92\x82\x82a\x0F\x0CV[PPV[_a\x06\x9Fa\x0F\xCDV[P_Q` a!\x14_9_Q\x90_R\x90V[a\x06\xB9a\x10\x16V[a\x06\xC2_a\x10qV[V[a\x06\xCCa\x13\x02V[P`@\x80Qa\x05@\x81\x01\x82R_T\x81R`\x01T` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x84R`\x02T\x81R`\x03T\x81\x83\x01R\x82\x84\x01R\x82Q\x80\x84\x01\x84R`\x04T\x81R`\x05T\x81\x83\x01R``\x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x06T\x81R`\x07T\x81\x84\x01R`\x80\x80\x85\x01\x91\x90\x91R\x84Q\x80\x86\x01\x86R`\x08T\x81R`\tT\x81\x85\x01R`\xA0\x85\x01R\x84Q\x80\x86\x01\x86R`\nT\x81R`\x0BT\x81\x85\x01R`\xC0\x85\x01R\x84Q\x80\x86\x01\x86R`\x0CT\x81R`\rT\x81\x85\x01R`\xE0\x85\x01R\x84Q\x80\x86\x01\x86R`\x0ET\x81R`\x0FT\x81\x85\x01Ra\x01\0\x85\x01R\x84Q\x80\x86\x01\x86R`\x10T\x81R`\x11T\x81\x85\x01Ra\x01 \x85\x01R\x84Q\x80\x86\x01\x86R`\x12T\x81R`\x13T\x81\x85\x01Ra\x01@\x85\x01R\x84Q\x80\x86\x01\x86R`\x14T\x81R`\x15T\x81\x85\x01Ra\x01`\x85\x01R\x84Q\x80\x86\x01\x86R`\x16T\x81R`\x17T\x81\x85\x01Ra\x01\x80\x85\x01R\x84Q\x80\x86\x01\x86R`\x18T\x81R`\x19T\x81\x85\x01Ra\x01\xA0\x85\x01R\x84Q\x80\x86\x01\x86R`\x1AT\x81R`\x1BT\x81\x85\x01Ra\x01\xC0\x85\x01R\x84Q\x80\x86\x01\x86R`\x1CT\x81R`\x1DT\x81\x85\x01Ra\x01\xE0\x85\x01R\x84Q\x80\x86\x01\x86R`\x1ET\x81R`\x1FT\x81\x85\x01Ra\x02\0\x85\x01R\x84Q\x80\x86\x01\x86R\x83T\x81R`!T\x81\x85\x01Ra\x02 \x85\x01R\x84Q\x80\x86\x01\x86R`\"T\x81R`#T\x81\x85\x01Ra\x02@\x85\x01R\x84Q\x80\x86\x01\x86R`$T\x81R`%T\x81\x85\x01Ra\x02`\x85\x01R\x84Q\x80\x86\x01\x86R`&T\x81R`'T\x81\x85\x01Ra\x02\x80\x85\x01R\x84Q\x80\x86\x01\x86R`(T\x81R`)T\x81\x85\x01Ra\x02\xA0\x85\x01R\x84Q\x80\x86\x01\x86R`*T\x81R`+T\x81\x85\x01Ra\x02\xC0\x85\x01R\x84Q\x80\x86\x01\x86R`,T\x81R`-T\x81\x85\x01Ra\x02\xE0\x85\x01R\x84Q\x80\x86\x01\x86R`.T\x81R`/T\x81\x85\x01Ra\x03\0\x85\x01R\x84Q\x80\x86\x01\x86R`0T\x81R`1T\x81\x85\x01Ra\x03 \x85\x01R`2Ta\x03@\x85\x01R`3Ta\x03`\x85\x01R`4Ta\x03\x80\x85\x01R`5Ta\x03\xA0\x85\x01R`6Ta\x03\xC0\x85\x01R`7Ta\x03\xE0\x85\x01R\x84Q\x80\x86\x01\x86R`8T\x81R`9T\x81\x85\x01Ra\x04\0\x85\x01R\x84Q\x80\x86\x01\x86R`:T\x81R`;T\x81\x85\x01Ra\x04 \x85\x01R\x84Q\x80\x86\x01\x86R`<T\x81R`=T\x81\x85\x01Ra\x04@\x85\x01R\x84Q\x80\x86\x01\x86R`>T\x81R`?T\x81\x85\x01Ra\x04`\x85\x01R\x84Ta\x04\x80\x85\x01R`ATa\x04\xA0\x85\x01R`BTa\x04\xC0\x85\x01R\x84Q\x80\x86\x01\x86R`CT\x81R`DT\x81\x85\x01Ra\x04\xE0\x85\x01R\x84Q\x80\x82\x01\x86R`ET\x81R`FT\x81\x85\x01R`GT\x81\x87\x01R`HT\x81\x84\x01Ra\x05\0\x85\x01R\x84Q\x90\x81\x01\x85R`IT\x81R`JT\x92\x81\x01\x92\x90\x92R`KT\x93\x82\x01\x93\x90\x93R`LT\x92\x81\x01\x92\x90\x92Ra\x05 \x81\x01\x91\x90\x91R\x90V[a\nra\x10\x16V[_\x81\x90\x03a\n\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01RhVK: empty`\xB8\x1B`D\x82\x01R`d\x01a\x03\x13V[_a\n\xB9\x83\x83a\x0EEV[`MT\x81Q_U` \x80\x83\x01Q`\x01U`@\x80\x84\x01Q\x80Q`\x02U\x82\x01Q`\x03U``\x80\x85\x01Q\x80Q`\x04U\x83\x01Q`\x05U`\x80\x85\x01Q\x80Q`\x06U\x83\x01Q`\x07U`\xA0\x85\x01Q\x80Q`\x08U\x83\x01Q`\tU`\xC0\x85\x01Q\x80Q`\nU\x83\x01Q`\x0BU`\xE0\x85\x01Q\x80Q`\x0CU\x83\x01Q`\rUa\x01\0\x85\x01Q\x80Q`\x0EU\x83\x01Q`\x0FUa\x01 \x85\x01Q\x80Q`\x10U\x83\x01Q`\x11Ua\x01@\x85\x01Q\x80Q`\x12U\x83\x01Q`\x13Ua\x01`\x85\x01Q\x80Q`\x14U\x83\x01Q`\x15Ua\x01\x80\x85\x01Q\x80Q`\x16U\x83\x01Q`\x17Ua\x01\xA0\x85\x01Q\x80Q`\x18U\x83\x01Q`\x19Ua\x01\xC0\x85\x01Q\x80Q`\x1AU\x83\x01Q`\x1BUa\x01\xE0\x85\x01Q\x80Q`\x1CU\x83\x01Q`\x1DUa\x02\0\x85\x01Q\x80Q`\x1EU\x83\x01Q`\x1FUa\x02 \x85\x01Q\x80Q\x84U\x83\x01Q`!Ua\x02@\x85\x01Q\x80Q`\"U\x83\x01Q`#Ua\x02`\x85\x01Q\x80Q`$U\x83\x01Q`%Ua\x02\x80\x85\x01Q\x80Q`&U\x83\x01Q`'Ua\x02\xA0\x85\x01Q\x80Q`(U\x83\x01Q`)Ua\x02\xC0\x85\x01Q\x80Q`*U\x83\x01Q`+Ua\x02\xE0\x85\x01Q\x80Q`,U\x83\x01Q`-Ua\x03\0\x85\x01Q\x80Q`.U\x83\x01Q`/Ua\x03 \x85\x01Q\x80Q`0U\x83\x01Q`1Ua\x03@\x85\x01Q`2Ua\x03`\x85\x01Q`3Ua\x03\x80\x85\x01Q`4Ua\x03\xA0\x85\x01Q`5Ua\x03\xC0\x85\x01Q`6Ua\x03\xE0\x85\x01Q`7Ua\x04\0\x85\x01Q\x80Q`8U\x83\x01Q`9Ua\x04 \x85\x01Q\x80Q`:U\x83\x01Q`;Ua\x04@\x85\x01Q\x80Q`<U\x83\x01Q`=Ua\x04`\x85\x01Q\x80Q`>U\x83\x01Q`?Ua\x04\x80\x85\x01Q\x82Ua\x04\xA0\x85\x01Q`AUa\x04\xC0\x85\x01Q`BUa\x04\xE0\x85\x01Q\x80Q`CU\x83\x01Q`DUa\x05\0\x85\x01Q\x80Q`EU\x80\x84\x01Q`FU\x80\x83\x01Q`GU\x81\x01Q`HUa\x05 \x85\x01Q\x80Q`IU\x92\x83\x01Q`JU\x82\x82\x01Q`KU\x91\x90\x91\x01Q`LUQ\x91\x92P\x90a\rS\x90\x85\x90\x85\x90a\x1D\x01V[`@Q\x90\x81\x90\x03\x90 `MU`N\x80T`\x01\x91\x90_\x90a\r}\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a\x1D\x10V[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`@\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91U`MT`NT`@\x80Q\x86\x81R` \x81\x01\x93\x90\x93R\x92\x16\x91\x81\x01\x91\x90\x91R\x7Fn\x92\xB6\xDAdz\xE4j\xD3_\xC1 \xC2q{as\x05\xCB\xDBh\xE3F}\xDFHm\xCA6\xC9(\x9C\x91P``\x01`@Q\x80\x91\x03\x90\xA1PPPPV[a\r\xF7a\x10\x16V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x03\x13V[a\x0E)\x81a\x10qV[PV[a\x0E4a\x10\xE1V[a\x0E)\x81a\x11*V[a\x06\xC2a\x10\xE1V[a\x0EMa\x13\x02V[a\x0EY\x82\x84\x01\x84a\x1D\xE7V[\x93\x92PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x0E\xE6WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x0E\xDA_Q` a!\x14_9_Q\x90_RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x06\xC2W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E)a\x10\x16V[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x0FfWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x0Fc\x91\x81\x01\x90a \xE6V[`\x01[a\x0F\x8EW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x03\x13V[_Q` a!\x14_9_Q\x90_R\x81\x14a\x0F\xBEW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\x13V[a\x0F\xC8\x83\x83a\x112V[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xC2W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\x10H\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\xC2W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x03\x13V[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x84U`@Q\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a\x06\xC2W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\xF7a\x10\xE1V[a\x11;\x82a\x11\x87V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a\x11\x7FWa\x0F\xC8\x82\x82a\x11\xEAV[a\x06\x92a\x12^V[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a\x11\xBCW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x03\x13V[_Q` a!\x14_9_Q\x90_R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``__\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x12\x06\x91\x90a \xFDV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x12>W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x12CV[``\x91P[P\x91P\x91Pa\x12S\x85\x83\x83a\x12}V[\x92PPP[\x92\x91PPV[4\x15a\x06\xC2W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\x12\x92Wa\x12\x8D\x82a\x12\xD9V[a\x0EYV[\x81Q\x15\x80\x15a\x12\xA9WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x12\xD2W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x03\x13V[P\x92\x91PPV[\x80Q\x15a\x12\xE9W\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80a\x05@\x01`@R\x80_\x81R` \x01_\x81R` \x01a\x135`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x13U`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x13u`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x13\x95`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x13\xB5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x13\xD5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x13\xF5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x14\x15`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x145`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x14U`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x14u`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x14\x95`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x14\xB5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x14\xD5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x14\xF5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x15\x15`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x155`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x15U`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x15u`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x15\x95`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x15\xB5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x15\xD5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x15\xF5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x16\x15`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01a\x16Y`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x16y`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x16\x99`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x16\xB9`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01a\x16\xEB`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x17\x17`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x17C`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x90R\x90V[__` \x83\x85\x03\x12\x15a\x17YW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17nW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x17~W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\x93W__\xFD[\x85` \x82\x84\x01\x01\x11\x15a\x17\xA4W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x17\xCAW__\xFD[\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Qa\x05@\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x18\x06Wa\x18\x06a\x17\xCFV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x184Wa\x184a\x17\xCFV[`@R\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x18MW__\xFD[a\x18V\x83a\x17\xB4V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18pW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x18\x80W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x99Wa\x18\x99a\x17\xCFV[a\x18\xAC`\x1F\x82\x01`\x1F\x19\x16` \x01a\x18\x0CV[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a\x18\xC0W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_a\t\xA0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Qa\x19F`@\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P``\x83\x01Q\x80Q`\x80\x84\x01R` \x81\x01Q`\xA0\x84\x01RP`\x80\x83\x01Q\x80Q`\xC0\x84\x01R` \x81\x01Q`\xE0\x84\x01RP`\xA0\x83\x01Q\x80Qa\x01\0\x84\x01R` \x81\x01Qa\x01 \x84\x01RP`\xC0\x83\x01Q\x80Qa\x01@\x84\x01R` \x81\x01Qa\x01`\x84\x01RP`\xE0\x83\x01Q\x80Qa\x01\x80\x84\x01R` \x81\x01Qa\x01\xA0\x84\x01RPa\x01\0\x83\x01Q\x80Qa\x01\xC0\x84\x01R` \x81\x01Qa\x01\xE0\x84\x01RPa\x01 \x83\x01Q\x80Qa\x02\0\x84\x01R` \x81\x01Qa\x02 \x84\x01RPa\x01@\x83\x01Q\x80Qa\x02@\x84\x01R` \x81\x01Qa\x02`\x84\x01RPa\x01`\x83\x01Q\x80Qa\x02\x80\x84\x01R` \x81\x01Qa\x02\xA0\x84\x01RPa\x01\x80\x83\x01Q\x80Qa\x02\xC0\x84\x01R` \x81\x01Qa\x02\xE0\x84\x01RPa\x01\xA0\x83\x01Q\x80Qa\x03\0\x84\x01R` \x81\x01Qa\x03 \x84\x01RPa\x01\xC0\x83\x01Q\x80Qa\x03@\x84\x01R` \x81\x01Qa\x03`\x84\x01RPa\x01\xE0\x83\x01Q\x80Qa\x03\x80\x84\x01R` \x81\x01Qa\x03\xA0\x84\x01RPa\x02\0\x83\x01Q\x80Qa\x03\xC0\x84\x01R` \x81\x01Qa\x03\xE0\x84\x01RPa\x02 \x83\x01Q\x80Qa\x04\0\x84\x01R` \x81\x01Qa\x04 \x84\x01RPa\x02@\x83\x01Q\x80Qa\x04@\x84\x01R` \x81\x01Qa\x04`\x84\x01RPa\x02`\x83\x01Q\x80Qa\x04\x80\x84\x01R` \x81\x01Qa\x04\xA0\x84\x01RPa\x02\x80\x83\x01Q\x80Qa\x04\xC0\x84\x01R` \x81\x01Qa\x04\xE0\x84\x01RPa\x02\xA0\x83\x01Q\x80Qa\x05\0\x84\x01R` \x81\x01Qa\x05 \x84\x01RPa\x02\xC0\x83\x01Q\x80Qa\x05@\x84\x01R` \x81\x01Qa\x05`\x84\x01RPa\x02\xE0\x83\x01Q\x80Qa\x05\x80\x84\x01R` \x81\x01Qa\x05\xA0\x84\x01RPa\x03\0\x83\x01Q\x80Qa\x05\xC0\x84\x01R` \x81\x01Qa\x05\xE0\x84\x01RPa\x03 \x83\x01Q\x80Qa\x06\0\x84\x01R` \x81\x01Qa\x06 \x84\x01RPa\x03@\x83\x01Qa\x06@\x83\x01Ra\x03`\x83\x01Qa\x06`\x83\x01Ra\x03\x80\x83\x01Qa\x06\x80\x83\x01Ra\x03\xA0\x83\x01Qa\x06\xA0\x83\x01Ra\x03\xC0\x83\x01Qa\x06\xC0\x83\x01Ra\x03\xE0\x83\x01Qa\x06\xE0\x83\x01Ra\x04\0\x83\x01Q\x80Qa\x07\0\x84\x01R` \x81\x01Qa\x07 \x84\x01RPa\x04 \x83\x01Q\x80Qa\x07@\x84\x01R` \x81\x01Qa\x07`\x84\x01RPa\x04@\x83\x01Q\x80Qa\x07\x80\x84\x01R` \x81\x01Qa\x07\xA0\x84\x01RPa\x04`\x83\x01Q\x80Qa\x07\xC0\x84\x01R` \x81\x01Qa\x07\xE0\x84\x01RPa\x04\x80\x83\x01Qa\x08\0\x83\x01Ra\x04\xA0\x83\x01Qa\x08 \x83\x01Ra\x04\xC0\x83\x01Qa\x08@\x83\x01Ra\x04\xE0\x83\x01Q\x80Qa\x08`\x84\x01R` \x81\x01Qa\x08\x80\x84\x01RPa\x05\0\x83\x01Q\x80Qa\x08\xA0\x84\x01R` \x81\x01Qa\x08\xC0\x84\x01R`@\x81\x01Qa\x08\xE0\x84\x01R``\x81\x01Qa\t\0\x84\x01RPa\x05 \x83\x01Q\x80Qa\t \x84\x01R` \x81\x01Qa\t@\x84\x01R`@\x81\x01Qa\t`\x84\x01R``\x81\x01Qa\t\x80\x84\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1C\xF8W__\xFD[a\x0EY\x82a\x17\xB4V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x12XWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`@\x82\x84\x03\x12\x15a\x1DKW__\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1DmWa\x1Dma\x17\xCFV[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a\x1D\x97W__\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D\xB9Wa\x1D\xB9a\x17\xCFV[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x83\x81\x015\x90\x82\x01R``\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_a\t\xA0\x82\x84\x03\x12\x80\x15a\x1D\xF9W__\xFD[Pa\x1E\x02a\x17\xE3V[\x825\x81R` \x80\x84\x015\x90\x82\x01Ra\x1E\x1D\x84`@\x85\x01a\x1D;V[`@\x82\x01Ra\x1E/\x84`\x80\x85\x01a\x1D;V[``\x82\x01Ra\x1EA\x84`\xC0\x85\x01a\x1D;V[`\x80\x82\x01Ra\x1ET\x84a\x01\0\x85\x01a\x1D;V[`\xA0\x82\x01Ra\x1Eg\x84a\x01@\x85\x01a\x1D;V[`\xC0\x82\x01Ra\x1Ez\x84a\x01\x80\x85\x01a\x1D;V[`\xE0\x82\x01Ra\x1E\x8D\x84a\x01\xC0\x85\x01a\x1D;V[a\x01\0\x82\x01Ra\x1E\xA1\x84a\x02\0\x85\x01a\x1D;V[a\x01 \x82\x01Ra\x1E\xB5\x84a\x02@\x85\x01a\x1D;V[a\x01@\x82\x01Ra\x1E\xC9\x84a\x02\x80\x85\x01a\x1D;V[a\x01`\x82\x01Ra\x1E\xDD\x84a\x02\xC0\x85\x01a\x1D;V[a\x01\x80\x82\x01Ra\x1E\xF1\x84a\x03\0\x85\x01a\x1D;V[a\x01\xA0\x82\x01Ra\x1F\x05\x84a\x03@\x85\x01a\x1D;V[a\x01\xC0\x82\x01Ra\x1F\x19\x84a\x03\x80\x85\x01a\x1D;V[a\x01\xE0\x82\x01Ra\x1F-\x84a\x03\xC0\x85\x01a\x1D;V[a\x02\0\x82\x01Ra\x1FA\x84a\x04\0\x85\x01a\x1D;V[a\x02 \x82\x01Ra\x1FU\x84a\x04@\x85\x01a\x1D;V[a\x02@\x82\x01Ra\x1Fi\x84a\x04\x80\x85\x01a\x1D;V[a\x02`\x82\x01Ra\x1F}\x84a\x04\xC0\x85\x01a\x1D;V[a\x02\x80\x82\x01Ra\x1F\x91\x84a\x05\0\x85\x01a\x1D;V[a\x02\xA0\x82\x01Ra\x1F\xA5\x84a\x05@\x85\x01a\x1D;V[a\x02\xC0\x82\x01Ra\x1F\xB9\x84a\x05\x80\x85\x01a\x1D;V[a\x02\xE0\x82\x01Ra\x1F\xCD\x84a\x05\xC0\x85\x01a\x1D;V[a\x03\0\x82\x01Ra\x1F\xE1\x84a\x06\0\x85\x01a\x1D;V[a\x03 \x82\x01Ra\x06@\x83\x015a\x03@\x82\x01Ra\x06`\x83\x015a\x03`\x82\x01Ra\x06\x80\x83\x015a\x03\x80\x82\x01Ra\x06\xA0\x83\x015a\x03\xA0\x82\x01Ra\x06\xC0\x83\x015a\x03\xC0\x82\x01Ra\x06\xE0\x83\x015a\x03\xE0\x82\x01Ra =\x84a\x07\0\x85\x01a\x1D;V[a\x04\0\x82\x01Ra Q\x84a\x07@\x85\x01a\x1D;V[a\x04 \x82\x01Ra e\x84a\x07\x80\x85\x01a\x1D;V[a\x04@\x82\x01Ra y\x84a\x07\xC0\x85\x01a\x1D;V[a\x04`\x82\x01Ra\x08\0\x83\x015a\x04\x80\x82\x01Ra\x08 \x83\x015a\x04\xA0\x82\x01Ra\x08@\x83\x015a\x04\xC0\x82\x01Ra \xB1\x84a\x08`\x85\x01a\x1D;V[a\x04\xE0\x82\x01Ra \xC5\x84a\x08\xA0\x85\x01a\x1D\x87V[a\x05\0\x82\x01Ra \xD9\x84a\t \x85\x01a\x1D\x87V[a\x05 \x82\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a \xF6W__\xFD[PQ\x91\x90PV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA2dipfsX\"\x12 a\xF0M\xBAF_\x7F\x88\xF4\x15\xFA\xC3\x82\xB2\xAE\x80\xA3\xF6g\x9F\xD5\x83\x85Z|8NTh*\xB6\x04dsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static ROLLUPPROOFVERIFICATIONKEY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x9AW_5`\xE0\x1C\x80c\x8D\xA5\xCB[\x11a\0bW\x80c\x8D\xA5\xCB[\x14a\x01\x1DW\x80c\x9EL\xC7\xED\x14a\x01cW\x80c\xAD<\xB1\xCC\x14a\x01\x8AW\x80c\xDF\xC4\xCDN\x14a\x01\xC7W\x80c\xE2?\xF9\xA1\x14a\x01\xE8W\x80c\xF2\xFD\xE3\x8B\x14a\x02\x07W__\xFD[\x80cC\x9F\xAB\x91\x14a\0\x9EW\x80cO\x1E\xF2\x86\x14a\0\xBFW\x80cO\xE8@\xF5\x14a\0\xD2W\x80cR\xD1\x90-\x14a\0\xF5W\x80cqP\x18\xA6\x14a\x01\tW[__\xFD[4\x80\x15a\0\xA9W__\xFD[Pa\0\xBDa\0\xB86`\x04a\x17HV[a\x02&V[\0[a\0\xBDa\0\xCD6`\x04a\x18<V[a\x06wV[4\x80\x15a\0\xDDW__\xFD[P`MT[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\0W__\xFD[Pa\0\xE2a\x06\x96V[4\x80\x15a\x01\x14W__\xFD[Pa\0\xBDa\x06\xB1V[4\x80\x15a\x01(W__\xFD[P\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xECV[4\x80\x15a\x01nW__\xFD[P`NT`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\0\xECV[4\x80\x15a\x01\x95W__\xFD[Pa\x01\xBA`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\0\xEC\x91\x90a\x18\xDFV[4\x80\x15a\x01\xD2W__\xFD[Pa\x01\xDBa\x06\xC4V[`@Qa\0\xEC\x91\x90a\x19\x14V[4\x80\x15a\x01\xF3W__\xFD[Pa\0\xBDa\x02\x026`\x04a\x17HV[a\njV[4\x80\x15a\x02\x12W__\xFD[Pa\0\xBDa\x02!6`\x04a\x1C\xE8V[a\r\xEFV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0\x80T`\x01`@\x1B\x81\x04`\xFF\x16\x15\x90`\x01`\x01`@\x1B\x03\x16_\x81\x15\x80\x15a\x02jWP\x82[\x90P_\x82`\x01`\x01`@\x1B\x03\x16`\x01\x14\x80\x15a\x02\x85WP0;\x15[\x90P\x81\x15\x80\x15a\x02\x93WP\x80\x15[\x15a\x02\xB1W`@Qc\xF9.\xE8\xA9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x17\x85U\x83\x15a\x02\xDBW\x84T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x85U[_\x86\x90\x03a\x03\x1CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01RhVK: empty`\xB8\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[a\x03%3a\x0E,V[a\x03-a\x0E=V[_a\x038\x88\x88a\x0EEV[\x80Q_U` \x80\x82\x01Q`\x01U`@\x80\x83\x01Q\x80Q`\x02U\x82\x01Q`\x03U``\x80\x84\x01Q\x80Q`\x04U\x83\x01Q`\x05U`\x80\x84\x01Q\x80Q`\x06U\x83\x01Q`\x07U`\xA0\x84\x01Q\x80Q`\x08U\x83\x01Q`\tU`\xC0\x84\x01Q\x80Q`\nU\x83\x01Q`\x0BU`\xE0\x84\x01Q\x80Q`\x0CU\x83\x01Q`\rUa\x01\0\x84\x01Q\x80Q`\x0EU\x83\x01Q`\x0FUa\x01 \x84\x01Q\x80Q`\x10U\x83\x01Q`\x11Ua\x01@\x84\x01Q\x80Q`\x12U\x83\x01Q`\x13Ua\x01`\x84\x01Q\x80Q`\x14U\x83\x01Q`\x15Ua\x01\x80\x84\x01Q\x80Q`\x16U\x83\x01Q`\x17Ua\x01\xA0\x84\x01Q\x80Q`\x18U\x83\x01Q`\x19Ua\x01\xC0\x84\x01Q\x80Q`\x1AU\x83\x01Q`\x1BUa\x01\xE0\x84\x01Q\x80Q`\x1CU\x83\x01Q`\x1DUa\x02\0\x84\x01Q\x80Q`\x1EU\x83\x01Q`\x1FUa\x02 \x84\x01Q\x80Q\x84U\x83\x01Q`!Ua\x02@\x84\x01Q\x80Q`\"U\x83\x01Q`#Ua\x02`\x84\x01Q\x80Q`$U\x83\x01Q`%Ua\x02\x80\x84\x01Q\x80Q`&U\x83\x01Q`'Ua\x02\xA0\x84\x01Q\x80Q`(U\x83\x01Q`)Ua\x02\xC0\x84\x01Q\x80Q`*U\x83\x01Q`+Ua\x02\xE0\x84\x01Q\x80Q`,U\x83\x01Q`-Ua\x03\0\x84\x01Q\x80Q`.U\x83\x01Q`/Ua\x03 \x84\x01Q\x80Q`0U\x83\x01Q`1Ua\x03@\x84\x01Q`2Ua\x03`\x84\x01Q`3Ua\x03\x80\x84\x01Q`4Ua\x03\xA0\x84\x01Q`5Ua\x03\xC0\x84\x01Q`6Ua\x03\xE0\x84\x01Q`7Ua\x04\0\x84\x01Q\x80Q`8U\x83\x01Q`9Ua\x04 \x84\x01Q\x80Q`:U\x83\x01Q`;Ua\x04@\x84\x01Q\x80Q`<U\x83\x01Q`=Ua\x04`\x84\x01Q\x80Q`>U\x83\x01Q`?Ua\x04\x80\x84\x01Q\x82Ua\x04\xA0\x84\x01Q`AUa\x04\xC0\x84\x01Q`BUa\x04\xE0\x84\x01Q\x80Q`CU\x83\x01Q`DUa\x05\0\x84\x01Q\x80Q`EU\x80\x84\x01Q`FU\x80\x83\x01Q`GU\x81\x01Q`HUa\x05 \x84\x01Q\x80Q`IU\x92\x83\x01Q`JU\x82\x82\x01Q`KU\x91\x90\x91\x01Q`LUQ\x90\x91Pa\x05\xCE\x90\x89\x90\x89\x90a\x1D\x01V[`@\x80Q\x91\x82\x90\x03\x82 `M\x81\x90U`N\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x90\x83R` \x83\x01R\x7F\xF0l\x01a+\x88vy\x01\x07\xCC`[\\\x9A\xC9\x93\x08\xD73\xDD\xD8\xFB\x03-\xE7f\xA7\n\xCD\xCDY\x91\x01`@Q\x80\x91\x03\x90\xA1P\x83\x15a\x06nW\x84T`\xFF`@\x1B\x19\x16\x85U`@Q`\x01\x81R\x7F\xC7\xF5\x05\xB2\xF3q\xAE!u\xEEI\x13\xF4I\x9E\x1F&3\xA7\xB5\x93c!\xEE\xD1\xCD\xAE\xB6\x11Q\x81\xD2\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[a\x06\x7Fa\x0E`V[a\x06\x88\x82a\x0F\x04V[a\x06\x92\x82\x82a\x0F\x0CV[PPV[_a\x06\x9Fa\x0F\xCDV[P_Q` a!\x14_9_Q\x90_R\x90V[a\x06\xB9a\x10\x16V[a\x06\xC2_a\x10qV[V[a\x06\xCCa\x13\x02V[P`@\x80Qa\x05@\x81\x01\x82R_T\x81R`\x01T` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x84R`\x02T\x81R`\x03T\x81\x83\x01R\x82\x84\x01R\x82Q\x80\x84\x01\x84R`\x04T\x81R`\x05T\x81\x83\x01R``\x80\x84\x01\x91\x90\x91R\x83Q\x80\x85\x01\x85R`\x06T\x81R`\x07T\x81\x84\x01R`\x80\x80\x85\x01\x91\x90\x91R\x84Q\x80\x86\x01\x86R`\x08T\x81R`\tT\x81\x85\x01R`\xA0\x85\x01R\x84Q\x80\x86\x01\x86R`\nT\x81R`\x0BT\x81\x85\x01R`\xC0\x85\x01R\x84Q\x80\x86\x01\x86R`\x0CT\x81R`\rT\x81\x85\x01R`\xE0\x85\x01R\x84Q\x80\x86\x01\x86R`\x0ET\x81R`\x0FT\x81\x85\x01Ra\x01\0\x85\x01R\x84Q\x80\x86\x01\x86R`\x10T\x81R`\x11T\x81\x85\x01Ra\x01 \x85\x01R\x84Q\x80\x86\x01\x86R`\x12T\x81R`\x13T\x81\x85\x01Ra\x01@\x85\x01R\x84Q\x80\x86\x01\x86R`\x14T\x81R`\x15T\x81\x85\x01Ra\x01`\x85\x01R\x84Q\x80\x86\x01\x86R`\x16T\x81R`\x17T\x81\x85\x01Ra\x01\x80\x85\x01R\x84Q\x80\x86\x01\x86R`\x18T\x81R`\x19T\x81\x85\x01Ra\x01\xA0\x85\x01R\x84Q\x80\x86\x01\x86R`\x1AT\x81R`\x1BT\x81\x85\x01Ra\x01\xC0\x85\x01R\x84Q\x80\x86\x01\x86R`\x1CT\x81R`\x1DT\x81\x85\x01Ra\x01\xE0\x85\x01R\x84Q\x80\x86\x01\x86R`\x1ET\x81R`\x1FT\x81\x85\x01Ra\x02\0\x85\x01R\x84Q\x80\x86\x01\x86R\x83T\x81R`!T\x81\x85\x01Ra\x02 \x85\x01R\x84Q\x80\x86\x01\x86R`\"T\x81R`#T\x81\x85\x01Ra\x02@\x85\x01R\x84Q\x80\x86\x01\x86R`$T\x81R`%T\x81\x85\x01Ra\x02`\x85\x01R\x84Q\x80\x86\x01\x86R`&T\x81R`'T\x81\x85\x01Ra\x02\x80\x85\x01R\x84Q\x80\x86\x01\x86R`(T\x81R`)T\x81\x85\x01Ra\x02\xA0\x85\x01R\x84Q\x80\x86\x01\x86R`*T\x81R`+T\x81\x85\x01Ra\x02\xC0\x85\x01R\x84Q\x80\x86\x01\x86R`,T\x81R`-T\x81\x85\x01Ra\x02\xE0\x85\x01R\x84Q\x80\x86\x01\x86R`.T\x81R`/T\x81\x85\x01Ra\x03\0\x85\x01R\x84Q\x80\x86\x01\x86R`0T\x81R`1T\x81\x85\x01Ra\x03 \x85\x01R`2Ta\x03@\x85\x01R`3Ta\x03`\x85\x01R`4Ta\x03\x80\x85\x01R`5Ta\x03\xA0\x85\x01R`6Ta\x03\xC0\x85\x01R`7Ta\x03\xE0\x85\x01R\x84Q\x80\x86\x01\x86R`8T\x81R`9T\x81\x85\x01Ra\x04\0\x85\x01R\x84Q\x80\x86\x01\x86R`:T\x81R`;T\x81\x85\x01Ra\x04 \x85\x01R\x84Q\x80\x86\x01\x86R`<T\x81R`=T\x81\x85\x01Ra\x04@\x85\x01R\x84Q\x80\x86\x01\x86R`>T\x81R`?T\x81\x85\x01Ra\x04`\x85\x01R\x84Ta\x04\x80\x85\x01R`ATa\x04\xA0\x85\x01R`BTa\x04\xC0\x85\x01R\x84Q\x80\x86\x01\x86R`CT\x81R`DT\x81\x85\x01Ra\x04\xE0\x85\x01R\x84Q\x80\x82\x01\x86R`ET\x81R`FT\x81\x85\x01R`GT\x81\x87\x01R`HT\x81\x84\x01Ra\x05\0\x85\x01R\x84Q\x90\x81\x01\x85R`IT\x81R`JT\x92\x81\x01\x92\x90\x92R`KT\x93\x82\x01\x93\x90\x93R`LT\x92\x81\x01\x92\x90\x92Ra\x05 \x81\x01\x91\x90\x91R\x90V[a\nra\x10\x16V[_\x81\x90\x03a\n\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01RhVK: empty`\xB8\x1B`D\x82\x01R`d\x01a\x03\x13V[_a\n\xB9\x83\x83a\x0EEV[`MT\x81Q_U` \x80\x83\x01Q`\x01U`@\x80\x84\x01Q\x80Q`\x02U\x82\x01Q`\x03U``\x80\x85\x01Q\x80Q`\x04U\x83\x01Q`\x05U`\x80\x85\x01Q\x80Q`\x06U\x83\x01Q`\x07U`\xA0\x85\x01Q\x80Q`\x08U\x83\x01Q`\tU`\xC0\x85\x01Q\x80Q`\nU\x83\x01Q`\x0BU`\xE0\x85\x01Q\x80Q`\x0CU\x83\x01Q`\rUa\x01\0\x85\x01Q\x80Q`\x0EU\x83\x01Q`\x0FUa\x01 \x85\x01Q\x80Q`\x10U\x83\x01Q`\x11Ua\x01@\x85\x01Q\x80Q`\x12U\x83\x01Q`\x13Ua\x01`\x85\x01Q\x80Q`\x14U\x83\x01Q`\x15Ua\x01\x80\x85\x01Q\x80Q`\x16U\x83\x01Q`\x17Ua\x01\xA0\x85\x01Q\x80Q`\x18U\x83\x01Q`\x19Ua\x01\xC0\x85\x01Q\x80Q`\x1AU\x83\x01Q`\x1BUa\x01\xE0\x85\x01Q\x80Q`\x1CU\x83\x01Q`\x1DUa\x02\0\x85\x01Q\x80Q`\x1EU\x83\x01Q`\x1FUa\x02 \x85\x01Q\x80Q\x84U\x83\x01Q`!Ua\x02@\x85\x01Q\x80Q`\"U\x83\x01Q`#Ua\x02`\x85\x01Q\x80Q`$U\x83\x01Q`%Ua\x02\x80\x85\x01Q\x80Q`&U\x83\x01Q`'Ua\x02\xA0\x85\x01Q\x80Q`(U\x83\x01Q`)Ua\x02\xC0\x85\x01Q\x80Q`*U\x83\x01Q`+Ua\x02\xE0\x85\x01Q\x80Q`,U\x83\x01Q`-Ua\x03\0\x85\x01Q\x80Q`.U\x83\x01Q`/Ua\x03 \x85\x01Q\x80Q`0U\x83\x01Q`1Ua\x03@\x85\x01Q`2Ua\x03`\x85\x01Q`3Ua\x03\x80\x85\x01Q`4Ua\x03\xA0\x85\x01Q`5Ua\x03\xC0\x85\x01Q`6Ua\x03\xE0\x85\x01Q`7Ua\x04\0\x85\x01Q\x80Q`8U\x83\x01Q`9Ua\x04 \x85\x01Q\x80Q`:U\x83\x01Q`;Ua\x04@\x85\x01Q\x80Q`<U\x83\x01Q`=Ua\x04`\x85\x01Q\x80Q`>U\x83\x01Q`?Ua\x04\x80\x85\x01Q\x82Ua\x04\xA0\x85\x01Q`AUa\x04\xC0\x85\x01Q`BUa\x04\xE0\x85\x01Q\x80Q`CU\x83\x01Q`DUa\x05\0\x85\x01Q\x80Q`EU\x80\x84\x01Q`FU\x80\x83\x01Q`GU\x81\x01Q`HUa\x05 \x85\x01Q\x80Q`IU\x92\x83\x01Q`JU\x82\x82\x01Q`KU\x91\x90\x91\x01Q`LUQ\x91\x92P\x90a\rS\x90\x85\x90\x85\x90a\x1D\x01V[`@Q\x90\x81\x90\x03\x90 `MU`N\x80T`\x01\x91\x90_\x90a\r}\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a\x1D\x10V[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`@\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91U`MT`NT`@\x80Q\x86\x81R` \x81\x01\x93\x90\x93R\x92\x16\x91\x81\x01\x91\x90\x91R\x7Fn\x92\xB6\xDAdz\xE4j\xD3_\xC1 \xC2q{as\x05\xCB\xDBh\xE3F}\xDFHm\xCA6\xC9(\x9C\x91P``\x01`@Q\x80\x91\x03\x90\xA1PPPPV[a\r\xF7a\x10\x16V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x0E W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01a\x03\x13V[a\x0E)\x81a\x10qV[PV[a\x0E4a\x10\xE1V[a\x0E)\x81a\x11*V[a\x06\xC2a\x10\xE1V[a\x0EMa\x13\x02V[a\x0EY\x82\x84\x01\x84a\x1D\xE7V[\x93\x92PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x80a\x0E\xE6WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\x0E\xDA_Q` a!\x14_9_Q\x90_RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x06\xC2W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0E)a\x10\x16V[\x81`\x01`\x01`\xA0\x1B\x03\x16cR\xD1\x90-`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x0FfWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x0Fc\x91\x81\x01\x90a \xE6V[`\x01[a\x0F\x8EW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x03\x13V[_Q` a!\x14_9_Q\x90_R\x81\x14a\x0F\xBEW`@Qc*\x87Ri`\xE2\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\x13V[a\x0F\xC8\x83\x83a\x112V[PPPV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xC2W`@Qcp>F\xDD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\x10H\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\xC2W`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x03\x13V[\x7F\x90\x16\xD0\x9Dr\xD4\x0F\xDA\xE2\xFD\x8C\xEA\xC6\xB6#Lw\x06!O\xD3\x9C\x1C\xD1\xE6\t\xA0R\x8C\x19\x93\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x91\x82\x17\x84U`@Q\x92\x16\x91\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPPV[\x7F\xF0\xC5~\x16\x84\r\xF0@\xF1P\x88\xDC/\x81\xFE9\x1C9#\xBE\xC7>#\xA9f.\xFC\x9C\"\x9Cj\0T`\x01`@\x1B\x90\x04`\xFF\x16a\x06\xC2W`@Qc\x1A\xFC\xD7\x9F`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\xF7a\x10\xE1V[a\x11;\x82a\x11\x87V[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a\x11\x7FWa\x0F\xC8\x82\x82a\x11\xEAV[a\x06\x92a\x12^V[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a\x11\xBCW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01a\x03\x13V[_Q` a!\x14_9_Q\x90_R\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``__\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x12\x06\x91\x90a \xFDV[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x12>W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x12CV[``\x91P[P\x91P\x91Pa\x12S\x85\x83\x83a\x12}V[\x92PPP[\x92\x91PPV[4\x15a\x06\xC2W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\x12\x92Wa\x12\x8D\x82a\x12\xD9V[a\x0EYV[\x81Q\x15\x80\x15a\x12\xA9WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x12\xD2W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x03\x13V[P\x92\x91PPV[\x80Q\x15a\x12\xE9W\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x80a\x05@\x01`@R\x80_\x81R` \x01_\x81R` \x01a\x135`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x13U`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x13u`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x13\x95`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x13\xB5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x13\xD5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x13\xF5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x14\x15`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x145`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x14U`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x14u`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x14\x95`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x14\xB5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x14\xD5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x14\xF5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x15\x15`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x155`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x15U`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x15u`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x15\x95`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x15\xB5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x15\xD5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x15\xF5`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x16\x15`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01a\x16Y`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x16y`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x16\x99`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x16\xB9`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01a\x16\xEB`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x17\x17`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x17C`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x90R\x90V[__` \x83\x85\x03\x12\x15a\x17YW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17nW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x17~W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\x93W__\xFD[\x85` \x82\x84\x01\x01\x11\x15a\x17\xA4W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x17\xCAW__\xFD[\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Qa\x05@\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x18\x06Wa\x18\x06a\x17\xCFV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x184Wa\x184a\x17\xCFV[`@R\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x18MW__\xFD[a\x18V\x83a\x17\xB4V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18pW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x18\x80W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x99Wa\x18\x99a\x17\xCFV[a\x18\xAC`\x1F\x82\x01`\x1F\x19\x16` \x01a\x18\x0CV[\x81\x81R\x86` \x83\x85\x01\x01\x11\x15a\x18\xC0W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92P\x92\x90PV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_a\t\xA0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Qa\x19F`@\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P``\x83\x01Q\x80Q`\x80\x84\x01R` \x81\x01Q`\xA0\x84\x01RP`\x80\x83\x01Q\x80Q`\xC0\x84\x01R` \x81\x01Q`\xE0\x84\x01RP`\xA0\x83\x01Q\x80Qa\x01\0\x84\x01R` \x81\x01Qa\x01 \x84\x01RP`\xC0\x83\x01Q\x80Qa\x01@\x84\x01R` \x81\x01Qa\x01`\x84\x01RP`\xE0\x83\x01Q\x80Qa\x01\x80\x84\x01R` \x81\x01Qa\x01\xA0\x84\x01RPa\x01\0\x83\x01Q\x80Qa\x01\xC0\x84\x01R` \x81\x01Qa\x01\xE0\x84\x01RPa\x01 \x83\x01Q\x80Qa\x02\0\x84\x01R` \x81\x01Qa\x02 \x84\x01RPa\x01@\x83\x01Q\x80Qa\x02@\x84\x01R` \x81\x01Qa\x02`\x84\x01RPa\x01`\x83\x01Q\x80Qa\x02\x80\x84\x01R` \x81\x01Qa\x02\xA0\x84\x01RPa\x01\x80\x83\x01Q\x80Qa\x02\xC0\x84\x01R` \x81\x01Qa\x02\xE0\x84\x01RPa\x01\xA0\x83\x01Q\x80Qa\x03\0\x84\x01R` \x81\x01Qa\x03 \x84\x01RPa\x01\xC0\x83\x01Q\x80Qa\x03@\x84\x01R` \x81\x01Qa\x03`\x84\x01RPa\x01\xE0\x83\x01Q\x80Qa\x03\x80\x84\x01R` \x81\x01Qa\x03\xA0\x84\x01RPa\x02\0\x83\x01Q\x80Qa\x03\xC0\x84\x01R` \x81\x01Qa\x03\xE0\x84\x01RPa\x02 \x83\x01Q\x80Qa\x04\0\x84\x01R` \x81\x01Qa\x04 \x84\x01RPa\x02@\x83\x01Q\x80Qa\x04@\x84\x01R` \x81\x01Qa\x04`\x84\x01RPa\x02`\x83\x01Q\x80Qa\x04\x80\x84\x01R` \x81\x01Qa\x04\xA0\x84\x01RPa\x02\x80\x83\x01Q\x80Qa\x04\xC0\x84\x01R` \x81\x01Qa\x04\xE0\x84\x01RPa\x02\xA0\x83\x01Q\x80Qa\x05\0\x84\x01R` \x81\x01Qa\x05 \x84\x01RPa\x02\xC0\x83\x01Q\x80Qa\x05@\x84\x01R` \x81\x01Qa\x05`\x84\x01RPa\x02\xE0\x83\x01Q\x80Qa\x05\x80\x84\x01R` \x81\x01Qa\x05\xA0\x84\x01RPa\x03\0\x83\x01Q\x80Qa\x05\xC0\x84\x01R` \x81\x01Qa\x05\xE0\x84\x01RPa\x03 \x83\x01Q\x80Qa\x06\0\x84\x01R` \x81\x01Qa\x06 \x84\x01RPa\x03@\x83\x01Qa\x06@\x83\x01Ra\x03`\x83\x01Qa\x06`\x83\x01Ra\x03\x80\x83\x01Qa\x06\x80\x83\x01Ra\x03\xA0\x83\x01Qa\x06\xA0\x83\x01Ra\x03\xC0\x83\x01Qa\x06\xC0\x83\x01Ra\x03\xE0\x83\x01Qa\x06\xE0\x83\x01Ra\x04\0\x83\x01Q\x80Qa\x07\0\x84\x01R` \x81\x01Qa\x07 \x84\x01RPa\x04 \x83\x01Q\x80Qa\x07@\x84\x01R` \x81\x01Qa\x07`\x84\x01RPa\x04@\x83\x01Q\x80Qa\x07\x80\x84\x01R` \x81\x01Qa\x07\xA0\x84\x01RPa\x04`\x83\x01Q\x80Qa\x07\xC0\x84\x01R` \x81\x01Qa\x07\xE0\x84\x01RPa\x04\x80\x83\x01Qa\x08\0\x83\x01Ra\x04\xA0\x83\x01Qa\x08 \x83\x01Ra\x04\xC0\x83\x01Qa\x08@\x83\x01Ra\x04\xE0\x83\x01Q\x80Qa\x08`\x84\x01R` \x81\x01Qa\x08\x80\x84\x01RPa\x05\0\x83\x01Q\x80Qa\x08\xA0\x84\x01R` \x81\x01Qa\x08\xC0\x84\x01R`@\x81\x01Qa\x08\xE0\x84\x01R``\x81\x01Qa\t\0\x84\x01RPa\x05 \x83\x01Q\x80Qa\t \x84\x01R` \x81\x01Qa\t@\x84\x01R`@\x81\x01Qa\t`\x84\x01R``\x81\x01Qa\t\x80\x84\x01RP\x92\x91PPV[_` \x82\x84\x03\x12\x15a\x1C\xF8W__\xFD[a\x0EY\x82a\x17\xB4V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x12XWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`@\x82\x84\x03\x12\x15a\x1DKW__\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1DmWa\x1Dma\x17\xCFV[`@R\x825\x81R` \x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a\x1D\x97W__\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1D\xB9Wa\x1D\xB9a\x17\xCFV[`@\x90\x81R\x835\x82R` \x80\x85\x015\x90\x83\x01R\x83\x81\x015\x90\x82\x01R``\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x91\x90PV[_a\t\xA0\x82\x84\x03\x12\x80\x15a\x1D\xF9W__\xFD[Pa\x1E\x02a\x17\xE3V[\x825\x81R` \x80\x84\x015\x90\x82\x01Ra\x1E\x1D\x84`@\x85\x01a\x1D;V[`@\x82\x01Ra\x1E/\x84`\x80\x85\x01a\x1D;V[``\x82\x01Ra\x1EA\x84`\xC0\x85\x01a\x1D;V[`\x80\x82\x01Ra\x1ET\x84a\x01\0\x85\x01a\x1D;V[`\xA0\x82\x01Ra\x1Eg\x84a\x01@\x85\x01a\x1D;V[`\xC0\x82\x01Ra\x1Ez\x84a\x01\x80\x85\x01a\x1D;V[`\xE0\x82\x01Ra\x1E\x8D\x84a\x01\xC0\x85\x01a\x1D;V[a\x01\0\x82\x01Ra\x1E\xA1\x84a\x02\0\x85\x01a\x1D;V[a\x01 \x82\x01Ra\x1E\xB5\x84a\x02@\x85\x01a\x1D;V[a\x01@\x82\x01Ra\x1E\xC9\x84a\x02\x80\x85\x01a\x1D;V[a\x01`\x82\x01Ra\x1E\xDD\x84a\x02\xC0\x85\x01a\x1D;V[a\x01\x80\x82\x01Ra\x1E\xF1\x84a\x03\0\x85\x01a\x1D;V[a\x01\xA0\x82\x01Ra\x1F\x05\x84a\x03@\x85\x01a\x1D;V[a\x01\xC0\x82\x01Ra\x1F\x19\x84a\x03\x80\x85\x01a\x1D;V[a\x01\xE0\x82\x01Ra\x1F-\x84a\x03\xC0\x85\x01a\x1D;V[a\x02\0\x82\x01Ra\x1FA\x84a\x04\0\x85\x01a\x1D;V[a\x02 \x82\x01Ra\x1FU\x84a\x04@\x85\x01a\x1D;V[a\x02@\x82\x01Ra\x1Fi\x84a\x04\x80\x85\x01a\x1D;V[a\x02`\x82\x01Ra\x1F}\x84a\x04\xC0\x85\x01a\x1D;V[a\x02\x80\x82\x01Ra\x1F\x91\x84a\x05\0\x85\x01a\x1D;V[a\x02\xA0\x82\x01Ra\x1F\xA5\x84a\x05@\x85\x01a\x1D;V[a\x02\xC0\x82\x01Ra\x1F\xB9\x84a\x05\x80\x85\x01a\x1D;V[a\x02\xE0\x82\x01Ra\x1F\xCD\x84a\x05\xC0\x85\x01a\x1D;V[a\x03\0\x82\x01Ra\x1F\xE1\x84a\x06\0\x85\x01a\x1D;V[a\x03 \x82\x01Ra\x06@\x83\x015a\x03@\x82\x01Ra\x06`\x83\x015a\x03`\x82\x01Ra\x06\x80\x83\x015a\x03\x80\x82\x01Ra\x06\xA0\x83\x015a\x03\xA0\x82\x01Ra\x06\xC0\x83\x015a\x03\xC0\x82\x01Ra\x06\xE0\x83\x015a\x03\xE0\x82\x01Ra =\x84a\x07\0\x85\x01a\x1D;V[a\x04\0\x82\x01Ra Q\x84a\x07@\x85\x01a\x1D;V[a\x04 \x82\x01Ra e\x84a\x07\x80\x85\x01a\x1D;V[a\x04@\x82\x01Ra y\x84a\x07\xC0\x85\x01a\x1D;V[a\x04`\x82\x01Ra\x08\0\x83\x015a\x04\x80\x82\x01Ra\x08 \x83\x015a\x04\xA0\x82\x01Ra\x08@\x83\x015a\x04\xC0\x82\x01Ra \xB1\x84a\x08`\x85\x01a\x1D;V[a\x04\xE0\x82\x01Ra \xC5\x84a\x08\xA0\x85\x01a\x1D\x87V[a\x05\0\x82\x01Ra \xD9\x84a\t \x85\x01a\x1D\x87V[a\x05 \x82\x01R\x93\x92PPPV[_` \x82\x84\x03\x12\x15a \xF6W__\xFD[PQ\x91\x90PV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV\xFE6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\xA2dipfsX\"\x12 a\xF0M\xBAF_\x7F\x88\xF4\x15\xFA\xC3\x82\xB2\xAE\x80\xA3\xF6g\x9F\xD5\x83\x85Z|8NTh*\xB6\x04dsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static ROLLUPPROOFVERIFICATIONKEY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct RollupProofVerificationKey<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for RollupProofVerificationKey<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for RollupProofVerificationKey<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for RollupProofVerificationKey<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for RollupProofVerificationKey<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(RollupProofVerificationKey))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> RollupProofVerificationKey<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ROLLUPPROOFVERIFICATIONKEY_ABI.clone(),
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
                ROLLUPPROOFVERIFICATIONKEY_ABI.clone(),
                ROLLUPPROOFVERIFICATIONKEY_BYTECODE.clone().into(),
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
        ///Calls the contract's `getVerificationKey` (0xdfc4cd4e) function
        pub fn get_verification_key(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, VerificationKey> {
            self.0
                .method_hash([223, 196, 205, 78], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x439fab91) function
        pub fn initialize(
            &self,
            vk_blob: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([67, 159, 171, 145], vk_blob)
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
        ///Calls the contract's `proxiableUUID` (0x52d1902d) function
        pub fn proxiable_uuid(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([82, 209, 144, 45], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `replaceVK` (0xe23ff9a1) function
        pub fn replace_vk(
            &self,
            vk_blob: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 63, 249, 161], vk_blob)
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
        ///Calls the contract's `vkHash` (0x4fe840f5) function
        pub fn vk_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([79, 232, 64, 245], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `vkVersion` (0x9e4cc7ed) function
        pub fn vk_version(&self) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([158, 76, 199, 237], ())
                .expect("method not found (this should never happen)")
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
        ///Gets the contract's `VKInitialized` event
        pub fn vk_initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VkinitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `VKReplaced` event
        pub fn vk_replaced_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VkreplacedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RollupProofVerificationKeyEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for RollupProofVerificationKey<M> {
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
    ///Custom Error type `OwnableInvalidOwner` with signature `OwnableInvalidOwner(address)` and selector `0x1e4fbdf7`
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
    #[etherror(name = "OwnableInvalidOwner", abi = "OwnableInvalidOwner(address)")]
    pub struct OwnableInvalidOwner {
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `OwnableUnauthorizedAccount` with signature `OwnableUnauthorizedAccount(address)` and selector `0x118cdaa7`
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
        name = "OwnableUnauthorizedAccount",
        abi = "OwnableUnauthorizedAccount(address)"
    )]
    pub struct OwnableUnauthorizedAccount {
        pub account: ::ethers::core::types::Address,
    }
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
    pub enum RollupProofVerificationKeyErrors {
        AddressEmptyCode(AddressEmptyCode),
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        ERC1967NonPayable(ERC1967NonPayable),
        FailedCall(FailedCall),
        InvalidInitialization(InvalidInitialization),
        NotInitializing(NotInitializing),
        OwnableInvalidOwner(OwnableInvalidOwner),
        OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
        UUPSUnauthorizedCallContext(UUPSUnauthorizedCallContext),
        UUPSUnsupportedProxiableUUID(UUPSUnsupportedProxiableUUID),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for RollupProofVerificationKeyErrors {
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
            if let Ok(decoded) = <OwnableInvalidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableInvalidOwner(decoded));
            }
            if let Ok(decoded) = <OwnableUnauthorizedAccount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnableUnauthorizedAccount(decoded));
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
    impl ::ethers::core::abi::AbiEncode for RollupProofVerificationKeyErrors {
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
                Self::OwnableInvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OwnableUnauthorizedAccount(element) => {
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
    impl ::ethers::contract::ContractRevert for RollupProofVerificationKeyErrors {
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
                    == <OwnableInvalidOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OwnableUnauthorizedAccount as ::ethers::contract::EthError>::selector() => {
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
    impl ::core::fmt::Display for RollupProofVerificationKeyErrors {
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
                Self::OwnableInvalidOwner(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnableUnauthorizedAccount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<::std::string::String>
    for RollupProofVerificationKeyErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for RollupProofVerificationKeyErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation>
    for RollupProofVerificationKeyErrors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for RollupProofVerificationKeyErrors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedCall> for RollupProofVerificationKeyErrors {
        fn from(value: FailedCall) -> Self {
            Self::FailedCall(value)
        }
    }
    impl ::core::convert::From<InvalidInitialization>
    for RollupProofVerificationKeyErrors {
        fn from(value: InvalidInitialization) -> Self {
            Self::InvalidInitialization(value)
        }
    }
    impl ::core::convert::From<NotInitializing> for RollupProofVerificationKeyErrors {
        fn from(value: NotInitializing) -> Self {
            Self::NotInitializing(value)
        }
    }
    impl ::core::convert::From<OwnableInvalidOwner>
    for RollupProofVerificationKeyErrors {
        fn from(value: OwnableInvalidOwner) -> Self {
            Self::OwnableInvalidOwner(value)
        }
    }
    impl ::core::convert::From<OwnableUnauthorizedAccount>
    for RollupProofVerificationKeyErrors {
        fn from(value: OwnableUnauthorizedAccount) -> Self {
            Self::OwnableUnauthorizedAccount(value)
        }
    }
    impl ::core::convert::From<UUPSUnauthorizedCallContext>
    for RollupProofVerificationKeyErrors {
        fn from(value: UUPSUnauthorizedCallContext) -> Self {
            Self::UUPSUnauthorizedCallContext(value)
        }
    }
    impl ::core::convert::From<UUPSUnsupportedProxiableUUID>
    for RollupProofVerificationKeyErrors {
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
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
    #[ethevent(name = "VKInitialized", abi = "VKInitialized(bytes32,uint64)")]
    pub struct VkinitializedFilter {
        pub vk_hash: [u8; 32],
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
    #[ethevent(name = "VKReplaced", abi = "VKReplaced(bytes32,bytes32,uint64)")]
    pub struct VkreplacedFilter {
        pub old_hash: [u8; 32],
        pub new_hash: [u8; 32],
        pub new_version: u64,
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
    pub enum RollupProofVerificationKeyEvents {
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        UpgradedFilter(UpgradedFilter),
        VkinitializedFilter(VkinitializedFilter),
        VkreplacedFilter(VkreplacedFilter),
    }
    impl ::ethers::contract::EthLogDecode for RollupProofVerificationKeyEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(RollupProofVerificationKeyEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    RollupProofVerificationKeyEvents::OwnershipTransferredFilter(decoded),
                );
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(RollupProofVerificationKeyEvents::UpgradedFilter(decoded));
            }
            if let Ok(decoded) = VkinitializedFilter::decode_log(log) {
                return Ok(
                    RollupProofVerificationKeyEvents::VkinitializedFilter(decoded),
                );
            }
            if let Ok(decoded) = VkreplacedFilter::decode_log(log) {
                return Ok(RollupProofVerificationKeyEvents::VkreplacedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for RollupProofVerificationKeyEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::VkinitializedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::VkreplacedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<InitializedFilter> for RollupProofVerificationKeyEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for RollupProofVerificationKeyEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for RollupProofVerificationKeyEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    impl ::core::convert::From<VkinitializedFilter>
    for RollupProofVerificationKeyEvents {
        fn from(value: VkinitializedFilter) -> Self {
            Self::VkinitializedFilter(value)
        }
    }
    impl ::core::convert::From<VkreplacedFilter> for RollupProofVerificationKeyEvents {
        fn from(value: VkreplacedFilter) -> Self {
            Self::VkreplacedFilter(value)
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
    ///Container type for all input parameters for the `getVerificationKey` function with signature `getVerificationKey()` and selector `0xdfc4cd4e`
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
    #[ethcall(name = "getVerificationKey", abi = "getVerificationKey()")]
    pub struct GetVerificationKeyCall;
    ///Container type for all input parameters for the `initialize` function with signature `initialize(bytes)` and selector `0x439fab91`
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
    #[ethcall(name = "initialize", abi = "initialize(bytes)")]
    pub struct InitializeCall {
        pub vk_blob: ::ethers::core::types::Bytes,
    }
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
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `replaceVK` function with signature `replaceVK(bytes)` and selector `0xe23ff9a1`
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
    #[ethcall(name = "replaceVK", abi = "replaceVK(bytes)")]
    pub struct ReplaceVKCall {
        pub vk_blob: ::ethers::core::types::Bytes,
    }
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
    ///Container type for all input parameters for the `vkHash` function with signature `vkHash()` and selector `0x4fe840f5`
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
    #[ethcall(name = "vkHash", abi = "vkHash()")]
    pub struct VkHashCall;
    ///Container type for all input parameters for the `vkVersion` function with signature `vkVersion()` and selector `0x9e4cc7ed`
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
    #[ethcall(name = "vkVersion", abi = "vkVersion()")]
    pub struct VkVersionCall;
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
    pub enum RollupProofVerificationKeyCalls {
        UpgradeInterfaceVersion(UpgradeInterfaceVersionCall),
        GetVerificationKey(GetVerificationKeyCall),
        Initialize(InitializeCall),
        Owner(OwnerCall),
        ProxiableUUID(ProxiableUUIDCall),
        RenounceOwnership(RenounceOwnershipCall),
        ReplaceVK(ReplaceVKCall),
        TransferOwnership(TransferOwnershipCall),
        UpgradeToAndCall(UpgradeToAndCallCall),
        VkHash(VkHashCall),
        VkVersion(VkVersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for RollupProofVerificationKeyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <UpgradeInterfaceVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UpgradeInterfaceVersion(decoded));
            }
            if let Ok(decoded) = <GetVerificationKeyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetVerificationKey(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <ProxiableUUIDCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxiableUUID(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <ReplaceVKCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReplaceVK(decoded));
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
            if let Ok(decoded) = <VkHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VkHash(decoded));
            }
            if let Ok(decoded) = <VkVersionCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::VkVersion(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for RollupProofVerificationKeyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::UpgradeInterfaceVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetVerificationKey(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProxiableUUID(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReplaceVK(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpgradeToAndCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::VkHash(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VkVersion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for RollupProofVerificationKeyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::UpgradeInterfaceVersion(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetVerificationKey(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxiableUUID(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReplaceVK(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradeToAndCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::VkHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::VkVersion(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<UpgradeInterfaceVersionCall>
    for RollupProofVerificationKeyCalls {
        fn from(value: UpgradeInterfaceVersionCall) -> Self {
            Self::UpgradeInterfaceVersion(value)
        }
    }
    impl ::core::convert::From<GetVerificationKeyCall>
    for RollupProofVerificationKeyCalls {
        fn from(value: GetVerificationKeyCall) -> Self {
            Self::GetVerificationKey(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for RollupProofVerificationKeyCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for RollupProofVerificationKeyCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<ProxiableUUIDCall> for RollupProofVerificationKeyCalls {
        fn from(value: ProxiableUUIDCall) -> Self {
            Self::ProxiableUUID(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall>
    for RollupProofVerificationKeyCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<ReplaceVKCall> for RollupProofVerificationKeyCalls {
        fn from(value: ReplaceVKCall) -> Self {
            Self::ReplaceVK(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall>
    for RollupProofVerificationKeyCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<UpgradeToAndCallCall>
    for RollupProofVerificationKeyCalls {
        fn from(value: UpgradeToAndCallCall) -> Self {
            Self::UpgradeToAndCall(value)
        }
    }
    impl ::core::convert::From<VkHashCall> for RollupProofVerificationKeyCalls {
        fn from(value: VkHashCall) -> Self {
            Self::VkHash(value)
        }
    }
    impl ::core::convert::From<VkVersionCall> for RollupProofVerificationKeyCalls {
        fn from(value: VkVersionCall) -> Self {
            Self::VkVersion(value)
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
    ///Container type for all return fields from the `getVerificationKey` function with signature `getVerificationKey()` and selector `0xdfc4cd4e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
    )]
    pub struct GetVerificationKeyReturn(pub VerificationKey);
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
    ///Container type for all return fields from the `vkHash` function with signature `vkHash()` and selector `0x4fe840f5`
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
    pub struct VkHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `vkVersion` function with signature `vkVersion()` and selector `0x9e4cc7ed`
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
    pub struct VkVersionReturn(pub u64);
}
