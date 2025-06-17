pub use erc3525::*;
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
pub mod erc3525 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("name_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("symbol_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("decimals_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint8"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator_"),
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId_"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("approve"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value_"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("balance"),
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
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId_"),
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
                    ::std::borrow::ToOwned::to_owned("contractURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("contractURI"),
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
                    ::std::borrow::ToOwned::to_owned("getApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getApproved"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId_"),
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
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("metadataDescriptor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("metadataDescriptor"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract IERC3525MetadataDescriptor",
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
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
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
                    ::std::borrow::ToOwned::to_owned("ownerOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("ownerOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId_"),
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
                                    name: ::std::borrow::ToOwned::to_owned("owner_"),
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
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId_"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data_"),
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
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operator_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("approved_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("slotOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("slotOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId_"),
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
                    ::std::borrow::ToOwned::to_owned("slotURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("slotURI"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("slot_"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("symbol"),
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
                    ::std::borrow::ToOwned::to_owned("tokenByIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenByIndex"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index_"),
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
                    ::std::borrow::ToOwned::to_owned("tokenOfOwnerByIndex"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "tokenOfOwnerByIndex",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index_"),
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
                    ::std::borrow::ToOwned::to_owned("tokenURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenURI"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId_"),
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
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fromTokenId_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value_"),
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
                                    name: ::std::borrow::ToOwned::to_owned("newTokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenId_"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fromTokenId_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("toTokenId_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value_"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("valueDecimals"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("valueDecimals"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Approval"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_approved"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalValue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ApprovalValue"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_operator"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_value"),
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
                    ::std::borrow::ToOwned::to_owned("SetMetadataDescriptor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SetMetadataDescriptor",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "metadataDescriptor",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SlotChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SlotChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_oldSlot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_newSlot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Transfer"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_tokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferValue"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TransferValue"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_fromTokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_toTokenId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_value"),
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
                    ::std::borrow::ToOwned::to_owned("StringsInsufficientHexLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StringsInsufficientHexLength",
                            ),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("length"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
    pub static ERC3525_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`@Qa-:8\x03\x80a-:\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01\x05V[`\x01`\x03U_a\0>\x84\x82a\x02\x06V[P`\x01a\0K\x83\x82a\x02\x06V[P`\x02\x80T`\xFF\x19\x16`\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UPa\x02\xC0\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x82`\x1F\x83\x01\x12a\0\x8BW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\0\xA4Wa\0\xA4a\0hV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\0\xD2Wa\0\xD2a\0hV[`@R\x81\x81R\x83\x82\x01` \x01\x85\x10\x15a\0\xE9W__\xFD[\x81` \x85\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[___``\x84\x86\x03\x12\x15a\x01\x17W__\xFD[\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01,W__\xFD[a\x018\x86\x82\x87\x01a\0|V[` \x86\x01Q\x90\x94P\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x01UW__\xFD[a\x01a\x86\x82\x87\x01a\0|V[\x92PP`@\x84\x01Q`\xFF\x81\x16\x81\x14a\x01wW__\xFD[\x80\x91PP\x92P\x92P\x92V[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x01\x96W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x01\xB4WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15a\x02\x01W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x01\xDFWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x01\xFEW_\x81U`\x01\x01a\x01\xEBV[PP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x02\x1FWa\x02\x1Fa\0hV[a\x023\x81a\x02-\x84Ta\x01\x82V[\x84a\x01\xBAV[` `\x1F\x82\x11`\x01\x81\x14a\x02eW_\x83\x15a\x02NWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x01\xFEV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x02\x94W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x02tV[P\x84\x82\x10\x15a\x02\xB1W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[a*m\x80a\x02\xCD_9_\xF3\xFE`\x80`@R`\x046\x10a\x01{W_5`\xE0\x1C\x80cOl\xCC\xE7\x11a\0\xCDW\x80c\x9C\xC7\xF7\x08\x11a\0\x87W\x80c\xC8{V\xDD\x11a\0bW\x80c\xC8{V\xDD\x14a\x04\0W\x80c\xE3E\xE0\xBC\x14a\x04\x1FW\x80c\xE8\xA3\xD4\x85\x14a\x04>W\x80c\xE9\x85\xE9\xC5\x14a\x04RW__\xFD[\x80c\x9C\xC7\xF7\x08\x14a\x03\xAFW\x80c\xA2,\xB4e\x14a\x03\xCEW\x80c\xB8\x8DO\xDE\x14a\x03\xEDW__\xFD[\x80cOl\xCC\xE7\x14a\x03\x0CW\x80ccR!\x1E\x14a\x03+W\x80cp\xA0\x821\x14a\x03JW\x80c\x84\x0Fq\x13\x14a\x03iW\x80c\x8C\xB0\xA5\x11\x14a\x03\x88W\x80c\x95\xD8\x9BA\x14a\x03\x9BW__\xFD[\x80c\x18\x16\r\xDD\x11a\x018W\x80c/t\\Y\x11a\x01\x13W\x80c/t\\Y\x14a\x02\xA6W\x80c1\x0E\xD7\xF0\x14a\x02\xC5W\x80c>~\x86i\x14a\x02\xD8W\x80cB\x84.\x0E\x14a\x02\xF9W__\xFD[\x80c\x18\x16\r\xDD\x14a\x02`W\x80c#\xB8r\xDD\x14a\x02tW\x80c&?>~\x14a\x02\x87W__\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\x7FW\x80c\x06\xFD\xDE\x03\x14a\x01\xB3W\x80c\x08\x18\x12\xFC\x14a\x01\xD4W\x80c\t^\xA7\xB3\x14a\x02\x0BW\x80c\t\xC3\xDD\x87\x14a\x02 W\x80c\x0FH\\\x02\x14a\x02?W[__\xFD[4\x80\x15a\x01\x8AW__\xFD[Pa\x01\x9Ea\x01\x996`\x04a$1V[a\x04\x9DV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xBEW__\xFD[Pa\x01\xC7a\x05?V[`@Qa\x01\xAA\x91\x90a$zV[4\x80\x15a\x01\xDFW__\xFD[Pa\x01\xF3a\x01\xEE6`\x04a$\x8CV[a\x05\xCEV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xAAV[a\x02\x1Ea\x02\x196`\x04a$\xB9V[a\x06\x1DV[\0[4\x80\x15a\x02+W__\xFD[Pa\x01\xC7a\x02:6`\x04a$\x8CV[a\x07\0V[a\x02Ra\x02M6`\x04a$\xE1V[a\x07\xEAV[`@Q\x90\x81R` \x01a\x01\xAAV[4\x80\x15a\x02kW__\xFD[P`\x05Ta\x02RV[a\x02\x1Ea\x02\x826`\x04a%\x14V[a\x08 V[4\x80\x15a\x02\x92W__\xFD[Pa\x02Ra\x02\xA16`\x04a$\x8CV[a\x08QV[4\x80\x15a\x02\xB1W__\xFD[Pa\x02Ra\x02\xC06`\x04a$\xB9V[a\x08\x95V[a\x02\x1Ea\x02\xD36`\x04a%=V[a\t2V[4\x80\x15a\x02\xE3W__\xFD[P`\x02T`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\xAAV[a\x02\x1Ea\x03\x076`\x04a%\x14V[a\tHV[4\x80\x15a\x03\x17W__\xFD[Pa\x02Ra\x03&6`\x04a$\x8CV[a\tbV[4\x80\x15a\x036W__\xFD[Pa\x01\xF3a\x03E6`\x04a$\x8CV[a\t\xEFV[4\x80\x15a\x03UW__\xFD[Pa\x02Ra\x03d6`\x04a%fV[a\n\x88V[4\x80\x15a\x03tW__\xFD[P`\x08Ta\x01\xF3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\x1Ea\x03\x966`\x04a$\xE1V[a\x0B\x0EV[4\x80\x15a\x03\xA6W__\xFD[Pa\x01\xC7a\x0B\xCCV[4\x80\x15a\x03\xBAW__\xFD[Pa\x02Ra\x03\xC96`\x04a$\x8CV[a\x0B\xDBV[4\x80\x15a\x03\xD9W__\xFD[Pa\x02\x1Ea\x03\xE86`\x04a%\x8CV[a\x0C\x1FV[a\x02\x1Ea\x03\xFB6`\x04a&-V[a\x0C.V[4\x80\x15a\x04\x0BW__\xFD[Pa\x01\xC7a\x04\x1A6`\x04a$\x8CV[a\x0C`V[4\x80\x15a\x04*W__\xFD[Pa\x02Ra\x0496`\x04a&\xD1V[a\x0C\xFBV[4\x80\x15a\x04IW__\xFD[Pa\x01\xC7a\r-V[4\x80\x15a\x04]W__\xFD[Pa\x01\x9Ea\x04l6`\x04a&\xFBV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R`\x02\x90\x92\x01\x90\x91R T`\xFF\x16\x90V[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x80a\x04\xCDWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x03T\xD6\x05`\xE6\x1B\x14[\x80a\x04\xE8WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x80\xACX\xCD`\xE0\x1B\x14[\x80a\x05\x03WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cp\xB0\x04\x81`\xE1\x1B\x14[\x80a\x05\x1EWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cx\x0E\x9Dc`\xE0\x1B\x14[\x80a\x059WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c[^\x13\x9F`\xE0\x1B\x14[\x92\x91PPV[``_\x80Ta\x05M\x90a'#V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05y\x90a'#V[\x80\x15a\x05\xC4W\x80`\x1F\x10a\x05\x9BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xC4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xA7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_a\x05\xD8\x82a\x0E\x1EV[_\x82\x81R`\x06` R`@\x90 T`\x05\x80T\x90\x91\x90\x81\x10a\x05\xFBWa\x05\xFBa'[V[_\x91\x82R` \x90\x91 `\x04`\x06\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[_a\x06'\x82a\t\xEFV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x06cW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06Z\x90a'oV[`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80a\x06\x7FWPa\x06\x7F\x813a\x04lV[a\x06\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FERC3525: approve caller is not o`D\x82\x01R\x7Fwner nor approved for all\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06ZV[a\x06\xFB\x83\x83a\x0ErV[PPPV[``_a\x07\x17`@\x80Q` \x81\x01\x90\x91R_\x81R\x90V[`\x08T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16a\x07uW_\x81Q\x11a\x07FW`@Q\x80` \x01`@R\x80_\x81RPa\x07\xE3V[\x80a\x07P\x84a\x0F\x07V[`@Q` \x01a\x07a\x92\x91\x90a'\xC8V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x07\xE3V[`\x08T`@Qc6\x01\xBF\xC5`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cl\x03\x7F\x8A\x90`$\x01[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xBCW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\xE3\x91\x90\x81\x01\x90a'\xF4V[\x93\x92PPPV[_a\x07\xF63\x85\x84a\x0F\x97V[a\x07\xFF\x84a\x10%V[\x90Pa\x08\x15\x83\x82a\x08\x0F\x87a\x08QV[_a\x10.V[a\x07\xE3\x84\x82\x84a\x11WV[a\x08*3\x82a\x14DV[a\x08FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06Z\x90a(iV[a\x06\xFB\x83\x83\x83a\x14\xC5V[_a\x08[\x82a\x0E\x1EV[_\x82\x81R`\x06` R`@\x90 T`\x05\x80T\x90\x91\x90\x81\x10a\x08~Wa\x08~a'[V[\x90_R` _ \x90`\x06\x02\x01`\x01\x01T\x90P\x91\x90PV[_a\x08\x9F\x83a\n\x88V[\x82\x10a\x08\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC3525: owner index out of boun`D\x82\x01Rads`\xF0\x1B`d\x82\x01R`\x84\x01a\x06ZV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x07` R`@\x90 \x80T\x83\x90\x81\x10a\t!Wa\t!a'[V[\x90_R` _ \x01T\x90P\x92\x91PPV[a\t=3\x84\x83a\x0F\x97V[a\x06\xFB\x83\x83\x83a\x11WV[a\x06\xFB\x83\x83\x83`@Q\x80` \x01`@R\x80_\x81RPa\x0C.V[_a\tl`\x05T\x90V[\x82\x10a\t\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC3525: global index out of bou`D\x82\x01Rbnds`\xE8\x1B`d\x82\x01R`\x84\x01a\x06ZV[`\x05\x82\x81T\x81\x10a\t\xD9Wa\t\xD9a'[V[\x90_R` _ \x90`\x06\x02\x01_\x01T\x90P\x91\x90PV[_a\t\xF9\x82a\x0E\x1EV[_\x82\x81R`\x06` R`@\x90 T`\x05\x80T\x90\x91\x90\x81\x10a\n\x1CWa\n\x1Ca'[V[_\x91\x82R` \x90\x91 `\x03`\x06\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x80a\n\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x11T\x90\xCC\xCDL\x8DN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`:\x1B`D\x82\x01R`d\x01a\x06ZV[\x91\x90PV[_`\x01`\x01`\xA0\x1B\x03\x82\x16a\n\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FERC3525: balance query for the z`D\x82\x01Rjero address`\xA8\x1B`d\x82\x01R`\x84\x01a\x06ZV[P`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x07` R`@\x90 T\x90V[_a\x0B\x18\x84a\t\xEFV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x0BKW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06Z\x90a'oV[a\x0BU3\x85a\x14DV[a\x0B\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FERC3525: approve caller is not o`D\x82\x01Rp\x1D\xDB\x99\\\x88\x1B\x9B\xDC\x88\x18\\\x1C\x1C\x9B\xDD\x99Y`z\x1B`d\x82\x01R`\x84\x01a\x06ZV[a\x0B\xC6\x84\x84\x84a\x16#V[PPPPV[```\x01\x80Ta\x05M\x90a'#V[_a\x0B\xE5\x82a\x0E\x1EV[_\x82\x81R`\x06` R`@\x90 T`\x05\x80T\x90\x91\x90\x81\x10a\x0C\x08Wa\x0C\x08a'[V[\x90_R` _ \x90`\x06\x02\x01`\x02\x01T\x90P\x91\x90PV[a\x0C*3\x83\x83a\x17XV[PPV[a\x0C83\x83a\x14DV[a\x0CTW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06Z\x90a(iV[a\x0B\xC6\x84\x84\x84\x84a\x18!V[``a\x0Ck\x82a\x0E\x1EV[_a\x0C\x80`@\x80Q` \x81\x01\x90\x91R_\x81R\x90V[`\x08T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16a\x0C\xCAW_\x81Q\x11a\x0C\xAFW`@Q\x80` \x01`@R\x80_\x81RPa\x07\xE3V[\x80a\x0C\xB9\x84a\x0F\x07V[`@Q` \x01a\x07a\x92\x91\x90a(\xBBV[`\x08T`@QcD\xA5\xA6\x17`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x89KL.\x90`$\x01a\x07\xA2V[_a\r\x05\x83a\x0E\x1EV[P_\x91\x82R`\x04` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[``_a\rD`@\x80Q` \x81\x01\x90\x91R_\x81R\x90V[`\x08T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16a\r\xA2W_\x81Q\x11a\rsW`@Q\x80` \x01`@R\x80_\x81RPa\x0E\x18V[\x80a\r}0a\x18\x94V[`@Q` \x01a\r\x8E\x92\x91\x90a(\xCFV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x0E\x18V[`\x08_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cr_\xA0\x9C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xF1W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\x18\x91\x90\x81\x01\x90a'\xF4V[\x91PP\x90V[a\x0E'\x81a\x18\xAAV[a\x0EoW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x11T\x90\xCC\xCDL\x8DN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`:\x1B`D\x82\x01R`d\x01a\x06ZV[PV[_\x81\x81R`\x06` R`@\x90 T`\x05\x80T\x84\x92\x90\x81\x10a\x0E\x95Wa\x0E\x95a'[V[_\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01`\x04\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U\x81\x90\x83\x16a\x0E\xCE\x82a\t\xEFV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPV[``_a\x0F\x13\x83a\x18\xF1V[`\x01\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F2Wa\x0F2a%\xC1V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0F\\W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x0FfWP\x93\x92PPPV[_a\x0F\xA2\x83\x85a\x0C\xFBV[\x90Pa\x0F\xAE\x84\x84a\x14DV[\x15\x80\x15a\x0F\xBCWP_\x19\x81\x14\x15[\x15a\x0B\xC6W\x81\x81\x10\x15a\x10\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC3525: insufficient allowance\0`D\x82\x01R`d\x01a\x06ZV[a\x0B\xC6\x83\x85a\x10 \x85\x85a)\nV[a\x16#V[_a\x059a\x19\xC8V[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x10\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC3525: mint to the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x06ZV[\x82_\x03a\x10\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC3525: cannot mint zero tokenI`D\x82\x01R`\x19`\xFA\x1B`d\x82\x01R`\x84\x01a\x06ZV[a\x10\xF0\x83a\x18\xAAV[\x15a\x11=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC3525: token already minted\0\0\0`D\x82\x01R`d\x01a\x06ZV[a\x11H\x84\x84\x84a\x19\xE1V[a\x11R\x83\x82a\x1A\xA4V[a\x0B\xC6V[a\x11`\x83a\x18\xAAV[a\x11\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FERC3525: transfer from invalid t`D\x82\x01Rf\x1B\xDA\xD9[\x88\x12Q`\xCA\x1B`d\x82\x01R`\x84\x01a\x06ZV[a\x11\xC5\x82a\x18\xAAV[a\x12\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC3525: transfer to invalid tok`D\x82\x01Rd\x19[\x88\x12Q`\xDA\x1B`d\x82\x01R`\x84\x01a\x06ZV[_\x83\x81R`\x06` R`@\x81 T`\x05\x80T\x90\x91\x90\x81\x10a\x12BWa\x12Ba'[V[\x90_R` _ \x90`\x06\x02\x01\x90P_`\x05`\x06_\x86\x81R` \x01\x90\x81R` \x01_ T\x81T\x81\x10a\x12uWa\x12ua'[V[\x90_R` _ \x90`\x06\x02\x01\x90P\x82\x82`\x02\x01T\x10\x15a\x12\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FERC3525: insufficient balance fo`D\x82\x01Ri9\x10:90\xB79\xB32\xB9`\xB1\x1B`d\x82\x01R`\x84\x01a\x06ZV[\x80`\x01\x01T\x82`\x01\x01T\x14a\x13XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC3525: transfer to token with `D\x82\x01Rm\x19\x1AY\x99\x99\\\x99[\x9D\x08\x1C\xDB\x1B\xDD`\x92\x1B`d\x82\x01R`\x84\x01a\x06ZV[\x82\x82`\x02\x01_\x82\x82Ta\x13k\x91\x90a)\nV[\x92PP\x81\x90UP\x82\x81`\x02\x01_\x82\x82Ta\x13\x85\x91\x90a)\x1DV[\x90\x91UPP`@Q\x83\x81R\x84\x90\x86\x90\x7F\x0B*\xAC\x84\xF3\xEC\x95i\x11\xFDx\xEA\xE51\x10b\x97/\xF9I\xF3\x84\x12\xE8\xDA9\x06\x9D\x9F\x06\x8C\xC6\x90` \x01`@Q\x80\x91\x03\x90\xA3a\x13\xDB\x85\x85\x85`@Q\x80` \x01`@R\x80_\x81RPa\x1B$V[a\x14=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC3525: transfer rejected by ER`D\x82\x01Rl!\x99\x9A\x99\x1A\xA92\xB1\xB2\xB4\xBB2\xB9`\x99\x1B`d\x82\x01R`\x84\x01a\x06ZV[PPPPPV[__a\x14O\x83a\t\xEFV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x14\x99WP`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R`\x02\x90\x93\x01\x90R T`\xFF\x16[\x80a\x14\xBDWP\x83`\x01`\x01`\xA0\x1B\x03\x16a\x14\xB2\x84a\x05\xCEV[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x94\x93PPPPV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x14\xD8\x82a\t\xEFV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC3525: transfer from invalid o`D\x82\x01Rc;\xB72\xB9`\xE1\x1B`d\x82\x01R`\x84\x01a\x06ZV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x15\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC3525: transfer to the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x06ZV[_a\x15\xA8\x82a\x08QV[\x90P_a\x15\xB4\x83a\x0B\xDBV[\x90Pa\x15\xC0_\x84a\x0ErV[a\x15\xC9\x83a\x1C\x90V[a\x15\xD3\x85\x84a\x1D,V[a\x15\xDD\x84\x84a\x1E?V[\x82\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF`@Q`@Q\x80\x91\x03\x90\xA4a\x14=V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x16\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FERC3525: approve value to the ze`D\x82\x01Riro address`\xB0\x1B`d\x82\x01R`\x84\x01a\x06ZV[a\x16\x96\x82\x84a\x1E\xC6V[a\x16\xFAW_\x83\x81R`\x06` R`@\x90 T`\x05\x80T\x90\x91\x90\x81\x10a\x16\xBDWa\x16\xBDa'[V[_\x91\x82R` \x80\x83 `\x06\x92\x90\x92\x02\x90\x91\x01`\x05\x01\x80T`\x01\x81\x01\x82U\x90\x83R\x91 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U[_\x83\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x80\x85R\x90\x83R\x92\x81\x90 \x84\x90UQ\x83\x81R\x85\x91\x7Fb\x1B\x05\r\xE0\xAD\x08\xB5\x1D\x19\xB4\x8B>m\xF7SH\xC4\xDEk\xDD\x93\xE8\x1B%,\xA6.(&[\x1B\x91\x01[`@Q\x80\x91\x03\x90\xA3PPPV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x17\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FERC3525: approve to caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06ZV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`\x07` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R`\x02\x90\x95\x01\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01a\x17KV[a\x18,\x84\x84\x84a\x14\xC5V[a\x188\x84\x84\x84\x84a\x1F\x88V[a\x0B\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FERC3525: transfer to non ERC721R`D\x82\x01Rf2\xB1\xB2\xB4\xBB2\xB9`\xC9\x1B`d\x82\x01R`\x84\x01a\x06ZV[``a\x059`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14a \xCAV[`\x05T_\x90\x15\x80\x15\x90a\x059WP_\x82\x81R`\x06` R`@\x90 T`\x05\x80T\x84\x92\x90\x81\x10a\x18\xDBWa\x18\xDBa'[V[\x90_R` _ \x90`\x06\x02\x01_\x01T\x14\x92\x91PPV[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x19/Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x19[Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x19yWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x19\x91Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x19\xA5Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x19\xB7W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x059W`\x01\x01\x92\x91PPV[`\x03\x80T_\x91\x82a\x19\xD8\x83a)0V[\x91\x90PU\x90P\x90V[`@\x80Q`\xC0\x81\x01\x82R\x83\x81R` \x80\x82\x01\x84\x90R_\x82\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16``\x84\x01R`\x80\x83\x01\x81\x90R\x83Q\x81\x81R\x91\x82\x01\x90\x93R`\xA0\x82\x01R\x90Pa\x1A.\x81a\"<V[a\x1A8\x84\x84a\x1E?V[`@Q\x83\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90_\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x82\x90\xA4\x81_\x84\x7F\xE4\xF4\x8C$\r;\x99IH\xAAT\xF3\xE2\xF5\xFC\xA5\x92c\xDF\xE1\xD5+nL\xF3\x9A]$\x9B\\\xCBe`@Q`@Q\x80\x91\x03\x90\xA4PPPPV[_\x82\x81R`\x06` R`@\x90 T`\x05\x80T\x83\x92\x90\x81\x10a\x1A\xC7Wa\x1A\xC7a'[V[\x90_R` _ \x90`\x06\x02\x01`\x02\x01_\x82\x82Ta\x1A\xE4\x91\x90a)\x1DV[\x90\x91UPP`@Q\x81\x81R\x82\x90_\x90\x7F\x0B*\xAC\x84\xF3\xEC\x95i\x11\xFDx\xEA\xE51\x10b\x97/\xF9I\xF3\x84\x12\xE8\xDA9\x06\x9D\x9F\x06\x8C\xC6\x90` \x01`@Q\x80\x91\x03\x90\xA3PPV[__a\x1B/\x85a\t\xEFV[\x90P\x80;c\xFF\xFF\xFF\xFF\x16\x15a\x1C\x84W`@Qc\x01\xFF\xC9\xA7`\xE0\x1B\x81Rb\x9C\xE2\x0B`\xE0\x1B`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x01\xFF\xC9\xA7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1B\xA4WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1B\xA1\x91\x81\x01\x90a)HV[`\x01[a\x1B\xE2W=\x80\x80\x15a\x1B\xD1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1B\xD6V[``\x91P[P`\x01\x92PPPa\x14\xBDV[\x80\x15a\x1CyW`@Qb\x9C\xE2\x0B`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90b\x9C\xE2\x0B\x90a\x1C\x1C\x903\x90\x8C\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01a)cV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\\\x91\x90a)\xA0V[`\x01`\x01`\xE0\x1B\x03\x19\x16b\x9C\xE2\x0B`\xE0\x1B\x14\x93Pa\x14\xBD\x92PPPV[`\x01\x92PPPa\x14\xBDV[P`\x01\x95\x94PPPPPV[_\x81\x81R`\x06` R`@\x81 T`\x05\x80T\x90\x91\x90\x81\x10a\x1C\xB3Wa\x1C\xB3a'[V[_\x91\x82R` \x82 `\x05`\x06\x90\x92\x02\x01\x90\x81\x01T\x90\x92P\x90[\x81\x81\x10\x15a\x1D\x1EW_\x83`\x05\x01\x82\x81T\x81\x10a\x1C\xEAWa\x1C\xEAa'[V[_\x91\x82R` \x80\x83 \x90\x91\x01T\x87\x83R`\x04\x82R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x84R\x91R\x81 UP`\x01\x01a\x1C\xCCV[Pa\x06\xFB`\x05\x83\x01_a#\x8AV[_\x81\x81R`\x06` R`@\x81 T`\x05\x80T\x90\x91\x90\x81\x10a\x1DOWa\x1DOa'[V[_\x91\x82R` \x80\x83 `\x06\x92\x90\x92\x02\x90\x91\x01`\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90U\x91\x84\x16\x81R`\x07\x90\x91R`@\x81 \x80T\x90\x91\x90a\x1D\x9D\x90`\x01\x90a)\nV[\x90P_\x82_\x01\x82\x81T\x81\x10a\x1D\xB4Wa\x1D\xB4a'[V[\x90_R` _ \x01T\x90P_\x83`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P\x81\x84_\x01\x82\x81T\x81\x10a\x1D\xEBWa\x1D\xEBa'[V[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x83\x81R`\x01\x86\x01\x90\x91R`@\x80\x82 \x83\x90U\x86\x82R\x81 U\x83T\x84\x90\x80a\x1E$Wa\x1E$a)\xBBV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90UPPPPPPV[_\x81\x81R`\x06` R`@\x90 T`\x05\x80T\x84\x92\x90\x81\x10a\x1EbWa\x1Eba'[V[_\x91\x82R` \x80\x83 `\x06\x92\x90\x92\x02\x90\x91\x01`\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90U\x93\x90\x91\x16\x81R`\x07\x80\x84R`@\x80\x83 \x80T\x85\x85R`\x01\x82\x81\x01\x88R\x92\x85 \x81\x90U\x92\x86R\x90\x82\x01\x81U\x82R\x92\x90 \x90\x91\x01UV[_\x81\x81R`\x06` R`@\x81 T`\x05\x80T\x83\x92\x90\x81\x10a\x1E\xE9Wa\x1E\xE9a'[V[_\x91\x82R` \x82 `\x05`\x06\x90\x92\x02\x01\x01T\x91P[\x81\x81\x10\x15a\x1F~W_\x84\x81R`\x06` R`@\x90 T`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x88\x16\x92\x90\x81\x10a\x1F2Wa\x1F2a'[V[\x90_R` _ \x90`\x06\x02\x01`\x05\x01\x82\x81T\x81\x10a\x1FRWa\x1FRa'[V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1FvW`\x01\x92PPPa\x059V[`\x01\x01a\x1E\xFEV[P_\x94\x93PPPPV[_\x83;c\xFF\xFF\xFF\xFF\x16\x15a \xC2W`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x15\x0Bz\x02\x90a\x1F\xC8\x903\x90\x89\x90\x88\x90\x88\x90`\x04\x01a)\xCFV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a \x02WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1F\xFF\x91\x81\x01\x90a)\xA0V[`\x01[a \xA8W=\x80\x80\x15a /W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a 4V[``\x91P[P\x80Q_\x03a \xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FERC721: transfer to non ERC721Re`D\x82\x01Rq1\xB2\xB4\xBB2\xB9\x104\xB6\xB862\xB6\xB2\xB7:2\xB9`q\x1B`d\x82\x01R`\x84\x01a\x06ZV[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x16c\n\x85\xBD\x01`\xE1\x1B\x14\x90Pa\x14\xBDV[P`\x01a\x14\xBDV[``\x82_a \xD9\x84`\x02a*\x0BV[a \xE4\x90`\x02a)\x1DV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xFCWa \xFCa%\xC1V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a!&W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81_\x81Q\x81\x10a!@Wa!@a'[V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a!nWa!na'[V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP_a!\x90\x85`\x02a*\x0BV[a!\x9B\x90`\x01a)\x1DV[\x90P[`\x01\x81\x11\x15a\"\x12Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x83`\x0F\x16`\x10\x81\x10a!\xCFWa!\xCFa'[V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a!\xE5Wa!\xE5a'[V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x04\x92\x90\x92\x1C\x91a\"\x0B\x81a*\"V[\x90Pa!\x9EV[P\x81\x15a\x14\xBDW`@Qc\xE2.'\xEB`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x85\x90R`D\x01a\x06ZV[`\x05\x80T\x82Q_\x90\x81R`\x06` \x81\x81R`@\x80\x84 \x85\x90U`\x01\x85\x01\x86U\x94\x90\x92R\x84Q\x92\x02\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB0\x81\x01\x92\x83U\x81\x85\x01Q\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB1\x82\x01U\x92\x84\x01Q\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB2\x84\x01U``\x84\x01Q\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB3\x84\x01\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x80\x86\x01Q\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB4\x86\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90U`\xA0\x84\x01Q\x80Q\x85\x94a\x0B\xC6\x93\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB5\x90\x91\x01\x92\x01\x90a#\xA5V[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a\x0Eo\x91\x90a$\x08V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15a#\xF8W\x91` \x02\x82\x01[\x82\x81\x11\x15a#\xF8W\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a#\xC3V[Pa$\x04\x92\x91Pa$\x08V[P\x90V[[\x80\x82\x11\x15a$\x04W_\x81U`\x01\x01a$\tV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0EoW__\xFD[_` \x82\x84\x03\x12\x15a$AW__\xFD[\x815a\x07\xE3\x81a$\x1CV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x07\xE3` \x83\x01\x84a$LV[_` \x82\x84\x03\x12\x15a$\x9CW__\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\x83W__\xFD[__`@\x83\x85\x03\x12\x15a$\xCAW__\xFD[a$\xD3\x83a$\xA3V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a$\xF3W__\xFD[\x835\x92Pa%\x03` \x85\x01a$\xA3V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[___``\x84\x86\x03\x12\x15a%&W__\xFD[a%/\x84a$\xA3V[\x92Pa%\x03` \x85\x01a$\xA3V[___``\x84\x86\x03\x12\x15a%OW__\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[_` \x82\x84\x03\x12\x15a%vW__\xFD[a\x07\xE3\x82a$\xA3V[\x80\x15\x15\x81\x14a\x0EoW__\xFD[__`@\x83\x85\x03\x12\x15a%\x9DW__\xFD[a%\xA6\x83a$\xA3V[\x91P` \x83\x015a%\xB6\x81a%\x7FV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\xFEWa%\xFEa%\xC1V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&\x1FWa&\x1Fa%\xC1V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[____`\x80\x85\x87\x03\x12\x15a&@W__\xFD[a&I\x85a$\xA3V[\x93Pa&W` \x86\x01a$\xA3V[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&yW__\xFD[\x85\x01`\x1F\x81\x01\x87\x13a&\x89W__\xFD[\x805a&\x9Ca&\x97\x82a&\x06V[a%\xD5V[\x81\x81R\x88` \x83\x85\x01\x01\x11\x15a&\xB0W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92\x95\x91\x94P\x92PV[__`@\x83\x85\x03\x12\x15a&\xE2W__\xFD[\x825\x91Pa&\xF2` \x84\x01a$\xA3V[\x90P\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a'\x0CW__\xFD[a'\x15\x83a$\xA3V[\x91Pa&\xF2` \x84\x01a$\xA3V[`\x01\x81\x81\x1C\x90\x82\x16\x80a'7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a'UWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[` \x80\x82R`\"\x90\x82\x01R\x7FERC3525: approval to current own`@\x82\x01Ra2\xB9`\xF1\x1B``\x82\x01R`\x80\x01\x90V[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a'\xD3\x82\x85a'\xB1V[dslot/`\xD8\x1B\x81Ra'\xEB`\x05\x82\x01\x85a'\xB1V[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a(\x04W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x1AW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a(*W__\xFD[\x80Qa(8a&\x97\x82a&\x06V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a(LW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[` \x80\x82R`2\x90\x82\x01R\x7FERC3525: transfer caller is not `@\x82\x01Rq\x1B\xDD\xDB\x99\\\x88\x1B\x9B\xDC\x88\x18\\\x1C\x1C\x9B\xDD\x99Y`r\x1B``\x82\x01R`\x80\x01\x90V[_a\x14\xBDa(\xC9\x83\x86a'\xB1V[\x84a'\xB1V[_a(\xDA\x82\x85a'\xB1V[hcontract/`\xB8\x1B\x81Ra'\xEB`\t\x82\x01\x85a'\xB1V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x059Wa\x059a(\xF6V[\x80\x82\x01\x80\x82\x11\x15a\x059Wa\x059a(\xF6V[_`\x01\x82\x01a)AWa)Aa(\xF6V[P`\x01\x01\x90V[_` \x82\x84\x03\x12\x15a)XW__\xFD[\x81Qa\x07\xE3\x81a%\x7FV[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R\x83`@\x82\x01R\x82``\x82\x01R`\xA0`\x80\x82\x01R_a)\x95`\xA0\x83\x01\x84a$LV[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a)\xB0W__\xFD[\x81Qa\x07\xE3\x81a$\x1CV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R_\x90a*\x01\x90\x83\x01\x84a$LV[\x96\x95PPPPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x059Wa\x059a(\xF6V[_\x81a*0Wa*0a(\xF6V[P_\x19\x01\x90V\xFE\xA2dipfsX\"\x12 O\xE6\xF8u\xDF\"#~\xB2\"?\x88\n%\xF4uL9{w\x86i\r\x8D\x90\xE3\xE3b\x8E\xF1f-dsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static ERC3525_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x01{W_5`\xE0\x1C\x80cOl\xCC\xE7\x11a\0\xCDW\x80c\x9C\xC7\xF7\x08\x11a\0\x87W\x80c\xC8{V\xDD\x11a\0bW\x80c\xC8{V\xDD\x14a\x04\0W\x80c\xE3E\xE0\xBC\x14a\x04\x1FW\x80c\xE8\xA3\xD4\x85\x14a\x04>W\x80c\xE9\x85\xE9\xC5\x14a\x04RW__\xFD[\x80c\x9C\xC7\xF7\x08\x14a\x03\xAFW\x80c\xA2,\xB4e\x14a\x03\xCEW\x80c\xB8\x8DO\xDE\x14a\x03\xEDW__\xFD[\x80cOl\xCC\xE7\x14a\x03\x0CW\x80ccR!\x1E\x14a\x03+W\x80cp\xA0\x821\x14a\x03JW\x80c\x84\x0Fq\x13\x14a\x03iW\x80c\x8C\xB0\xA5\x11\x14a\x03\x88W\x80c\x95\xD8\x9BA\x14a\x03\x9BW__\xFD[\x80c\x18\x16\r\xDD\x11a\x018W\x80c/t\\Y\x11a\x01\x13W\x80c/t\\Y\x14a\x02\xA6W\x80c1\x0E\xD7\xF0\x14a\x02\xC5W\x80c>~\x86i\x14a\x02\xD8W\x80cB\x84.\x0E\x14a\x02\xF9W__\xFD[\x80c\x18\x16\r\xDD\x14a\x02`W\x80c#\xB8r\xDD\x14a\x02tW\x80c&?>~\x14a\x02\x87W__\xFD[\x80c\x01\xFF\xC9\xA7\x14a\x01\x7FW\x80c\x06\xFD\xDE\x03\x14a\x01\xB3W\x80c\x08\x18\x12\xFC\x14a\x01\xD4W\x80c\t^\xA7\xB3\x14a\x02\x0BW\x80c\t\xC3\xDD\x87\x14a\x02 W\x80c\x0FH\\\x02\x14a\x02?W[__\xFD[4\x80\x15a\x01\x8AW__\xFD[Pa\x01\x9Ea\x01\x996`\x04a$1V[a\x04\x9DV[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xBEW__\xFD[Pa\x01\xC7a\x05?V[`@Qa\x01\xAA\x91\x90a$zV[4\x80\x15a\x01\xDFW__\xFD[Pa\x01\xF3a\x01\xEE6`\x04a$\x8CV[a\x05\xCEV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xAAV[a\x02\x1Ea\x02\x196`\x04a$\xB9V[a\x06\x1DV[\0[4\x80\x15a\x02+W__\xFD[Pa\x01\xC7a\x02:6`\x04a$\x8CV[a\x07\0V[a\x02Ra\x02M6`\x04a$\xE1V[a\x07\xEAV[`@Q\x90\x81R` \x01a\x01\xAAV[4\x80\x15a\x02kW__\xFD[P`\x05Ta\x02RV[a\x02\x1Ea\x02\x826`\x04a%\x14V[a\x08 V[4\x80\x15a\x02\x92W__\xFD[Pa\x02Ra\x02\xA16`\x04a$\x8CV[a\x08QV[4\x80\x15a\x02\xB1W__\xFD[Pa\x02Ra\x02\xC06`\x04a$\xB9V[a\x08\x95V[a\x02\x1Ea\x02\xD36`\x04a%=V[a\t2V[4\x80\x15a\x02\xE3W__\xFD[P`\x02T`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01\xAAV[a\x02\x1Ea\x03\x076`\x04a%\x14V[a\tHV[4\x80\x15a\x03\x17W__\xFD[Pa\x02Ra\x03&6`\x04a$\x8CV[a\tbV[4\x80\x15a\x036W__\xFD[Pa\x01\xF3a\x03E6`\x04a$\x8CV[a\t\xEFV[4\x80\x15a\x03UW__\xFD[Pa\x02Ra\x03d6`\x04a%fV[a\n\x88V[4\x80\x15a\x03tW__\xFD[P`\x08Ta\x01\xF3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x02\x1Ea\x03\x966`\x04a$\xE1V[a\x0B\x0EV[4\x80\x15a\x03\xA6W__\xFD[Pa\x01\xC7a\x0B\xCCV[4\x80\x15a\x03\xBAW__\xFD[Pa\x02Ra\x03\xC96`\x04a$\x8CV[a\x0B\xDBV[4\x80\x15a\x03\xD9W__\xFD[Pa\x02\x1Ea\x03\xE86`\x04a%\x8CV[a\x0C\x1FV[a\x02\x1Ea\x03\xFB6`\x04a&-V[a\x0C.V[4\x80\x15a\x04\x0BW__\xFD[Pa\x01\xC7a\x04\x1A6`\x04a$\x8CV[a\x0C`V[4\x80\x15a\x04*W__\xFD[Pa\x02Ra\x0496`\x04a&\xD1V[a\x0C\xFBV[4\x80\x15a\x04IW__\xFD[Pa\x01\xC7a\r-V[4\x80\x15a\x04]W__\xFD[Pa\x01\x9Ea\x04l6`\x04a&\xFBV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R`\x02\x90\x92\x01\x90\x91R T`\xFF\x16\x90V[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x01\xFF\xC9\xA7`\xE0\x1B\x14\x80a\x04\xCDWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x03T\xD6\x05`\xE6\x1B\x14[\x80a\x04\xE8WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x80\xACX\xCD`\xE0\x1B\x14[\x80a\x05\x03WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cp\xB0\x04\x81`\xE1\x1B\x14[\x80a\x05\x1EWP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cx\x0E\x9Dc`\xE0\x1B\x14[\x80a\x059WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c[^\x13\x9F`\xE0\x1B\x14[\x92\x91PPV[``_\x80Ta\x05M\x90a'#V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05y\x90a'#V[\x80\x15a\x05\xC4W\x80`\x1F\x10a\x05\x9BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\xC4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05\xA7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x90V[_a\x05\xD8\x82a\x0E\x1EV[_\x82\x81R`\x06` R`@\x90 T`\x05\x80T\x90\x91\x90\x81\x10a\x05\xFBWa\x05\xFBa'[V[_\x91\x82R` \x90\x91 `\x04`\x06\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x92\x91PPV[_a\x06'\x82a\t\xEFV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x06cW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06Z\x90a'oV[`@Q\x80\x91\x03\x90\xFD[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14\x80a\x06\x7FWPa\x06\x7F\x813a\x04lV[a\x06\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FERC3525: approve caller is not o`D\x82\x01R\x7Fwner nor approved for all\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06ZV[a\x06\xFB\x83\x83a\x0ErV[PPPV[``_a\x07\x17`@\x80Q` \x81\x01\x90\x91R_\x81R\x90V[`\x08T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16a\x07uW_\x81Q\x11a\x07FW`@Q\x80` \x01`@R\x80_\x81RPa\x07\xE3V[\x80a\x07P\x84a\x0F\x07V[`@Q` \x01a\x07a\x92\x91\x90a'\xC8V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x07\xE3V[`\x08T`@Qc6\x01\xBF\xC5`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cl\x03\x7F\x8A\x90`$\x01[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xBCW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\xE3\x91\x90\x81\x01\x90a'\xF4V[\x93\x92PPPV[_a\x07\xF63\x85\x84a\x0F\x97V[a\x07\xFF\x84a\x10%V[\x90Pa\x08\x15\x83\x82a\x08\x0F\x87a\x08QV[_a\x10.V[a\x07\xE3\x84\x82\x84a\x11WV[a\x08*3\x82a\x14DV[a\x08FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06Z\x90a(iV[a\x06\xFB\x83\x83\x83a\x14\xC5V[_a\x08[\x82a\x0E\x1EV[_\x82\x81R`\x06` R`@\x90 T`\x05\x80T\x90\x91\x90\x81\x10a\x08~Wa\x08~a'[V[\x90_R` _ \x90`\x06\x02\x01`\x01\x01T\x90P\x91\x90PV[_a\x08\x9F\x83a\n\x88V[\x82\x10a\x08\xF8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FERC3525: owner index out of boun`D\x82\x01Rads`\xF0\x1B`d\x82\x01R`\x84\x01a\x06ZV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`\x07` R`@\x90 \x80T\x83\x90\x81\x10a\t!Wa\t!a'[V[\x90_R` _ \x01T\x90P\x92\x91PPV[a\t=3\x84\x83a\x0F\x97V[a\x06\xFB\x83\x83\x83a\x11WV[a\x06\xFB\x83\x83\x83`@Q\x80` \x01`@R\x80_\x81RPa\x0C.V[_a\tl`\x05T\x90V[\x82\x10a\t\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FERC3525: global index out of bou`D\x82\x01Rbnds`\xE8\x1B`d\x82\x01R`\x84\x01a\x06ZV[`\x05\x82\x81T\x81\x10a\t\xD9Wa\t\xD9a'[V[\x90_R` _ \x90`\x06\x02\x01_\x01T\x90P\x91\x90PV[_a\t\xF9\x82a\x0E\x1EV[_\x82\x81R`\x06` R`@\x90 T`\x05\x80T\x90\x91\x90\x81\x10a\n\x1CWa\n\x1Ca'[V[_\x91\x82R` \x90\x91 `\x03`\x06\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90P\x80a\n\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x11T\x90\xCC\xCDL\x8DN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`:\x1B`D\x82\x01R`d\x01a\x06ZV[\x91\x90PV[_`\x01`\x01`\xA0\x1B\x03\x82\x16a\n\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FERC3525: balance query for the z`D\x82\x01Rjero address`\xA8\x1B`d\x82\x01R`\x84\x01a\x06ZV[P`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x07` R`@\x90 T\x90V[_a\x0B\x18\x84a\t\xEFV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x0BKW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06Z\x90a'oV[a\x0BU3\x85a\x14DV[a\x0B\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FERC3525: approve caller is not o`D\x82\x01Rp\x1D\xDB\x99\\\x88\x1B\x9B\xDC\x88\x18\\\x1C\x1C\x9B\xDD\x99Y`z\x1B`d\x82\x01R`\x84\x01a\x06ZV[a\x0B\xC6\x84\x84\x84a\x16#V[PPPPV[```\x01\x80Ta\x05M\x90a'#V[_a\x0B\xE5\x82a\x0E\x1EV[_\x82\x81R`\x06` R`@\x90 T`\x05\x80T\x90\x91\x90\x81\x10a\x0C\x08Wa\x0C\x08a'[V[\x90_R` _ \x90`\x06\x02\x01`\x02\x01T\x90P\x91\x90PV[a\x0C*3\x83\x83a\x17XV[PPV[a\x0C83\x83a\x14DV[a\x0CTW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06Z\x90a(iV[a\x0B\xC6\x84\x84\x84\x84a\x18!V[``a\x0Ck\x82a\x0E\x1EV[_a\x0C\x80`@\x80Q` \x81\x01\x90\x91R_\x81R\x90V[`\x08T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16a\x0C\xCAW_\x81Q\x11a\x0C\xAFW`@Q\x80` \x01`@R\x80_\x81RPa\x07\xE3V[\x80a\x0C\xB9\x84a\x0F\x07V[`@Q` \x01a\x07a\x92\x91\x90a(\xBBV[`\x08T`@QcD\xA5\xA6\x17`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x89KL.\x90`$\x01a\x07\xA2V[_a\r\x05\x83a\x0E\x1EV[P_\x91\x82R`\x04` \x90\x81R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[``_a\rD`@\x80Q` \x81\x01\x90\x91R_\x81R\x90V[`\x08T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16a\r\xA2W_\x81Q\x11a\rsW`@Q\x80` \x01`@R\x80_\x81RPa\x0E\x18V[\x80a\r}0a\x18\x94V[`@Q` \x01a\r\x8E\x92\x91\x90a(\xCFV[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x0E\x18V[`\x08_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cr_\xA0\x9C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xF1W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\x18\x91\x90\x81\x01\x90a'\xF4V[\x91PP\x90V[a\x0E'\x81a\x18\xAAV[a\x0EoW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01Rx\x11T\x90\xCC\xCDL\x8DN\x88\x1A[\x9D\x98[\x1AY\x08\x1D\x1B\xDA\xD9[\x88\x12Q`:\x1B`D\x82\x01R`d\x01a\x06ZV[PV[_\x81\x81R`\x06` R`@\x90 T`\x05\x80T\x84\x92\x90\x81\x10a\x0E\x95Wa\x0E\x95a'[V[_\x91\x82R` \x90\x91 `\x06\x90\x91\x02\x01`\x04\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x90U\x81\x90\x83\x16a\x0E\xCE\x82a\t\xEFV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%`@Q`@Q\x80\x91\x03\x90\xA4PPV[``_a\x0F\x13\x83a\x18\xF1V[`\x01\x01\x90P_\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F2Wa\x0F2a%\xC1V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a\x0F\\W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81\x81\x01` \x01[_\x19\x01o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B`\n\x86\x06\x1A\x81S`\n\x85\x04\x94P\x84a\x0FfWP\x93\x92PPPV[_a\x0F\xA2\x83\x85a\x0C\xFBV[\x90Pa\x0F\xAE\x84\x84a\x14DV[\x15\x80\x15a\x0F\xBCWP_\x19\x81\x14\x15[\x15a\x0B\xC6W\x81\x81\x10\x15a\x10\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FERC3525: insufficient allowance\0`D\x82\x01R`d\x01a\x06ZV[a\x0B\xC6\x83\x85a\x10 \x85\x85a)\nV[a\x16#V[_a\x059a\x19\xC8V[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x10\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC3525: mint to the zero addres`D\x82\x01R`s`\xF8\x1B`d\x82\x01R`\x84\x01a\x06ZV[\x82_\x03a\x10\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FERC3525: cannot mint zero tokenI`D\x82\x01R`\x19`\xFA\x1B`d\x82\x01R`\x84\x01a\x06ZV[a\x10\xF0\x83a\x18\xAAV[\x15a\x11=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FERC3525: token already minted\0\0\0`D\x82\x01R`d\x01a\x06ZV[a\x11H\x84\x84\x84a\x19\xE1V[a\x11R\x83\x82a\x1A\xA4V[a\x0B\xC6V[a\x11`\x83a\x18\xAAV[a\x11\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FERC3525: transfer from invalid t`D\x82\x01Rf\x1B\xDA\xD9[\x88\x12Q`\xCA\x1B`d\x82\x01R`\x84\x01a\x06ZV[a\x11\xC5\x82a\x18\xAAV[a\x12\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC3525: transfer to invalid tok`D\x82\x01Rd\x19[\x88\x12Q`\xDA\x1B`d\x82\x01R`\x84\x01a\x06ZV[_\x83\x81R`\x06` R`@\x81 T`\x05\x80T\x90\x91\x90\x81\x10a\x12BWa\x12Ba'[V[\x90_R` _ \x90`\x06\x02\x01\x90P_`\x05`\x06_\x86\x81R` \x01\x90\x81R` \x01_ T\x81T\x81\x10a\x12uWa\x12ua'[V[\x90_R` _ \x90`\x06\x02\x01\x90P\x82\x82`\x02\x01T\x10\x15a\x12\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FERC3525: insufficient balance fo`D\x82\x01Ri9\x10:90\xB79\xB32\xB9`\xB1\x1B`d\x82\x01R`\x84\x01a\x06ZV[\x80`\x01\x01T\x82`\x01\x01T\x14a\x13XW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FERC3525: transfer to token with `D\x82\x01Rm\x19\x1AY\x99\x99\\\x99[\x9D\x08\x1C\xDB\x1B\xDD`\x92\x1B`d\x82\x01R`\x84\x01a\x06ZV[\x82\x82`\x02\x01_\x82\x82Ta\x13k\x91\x90a)\nV[\x92PP\x81\x90UP\x82\x81`\x02\x01_\x82\x82Ta\x13\x85\x91\x90a)\x1DV[\x90\x91UPP`@Q\x83\x81R\x84\x90\x86\x90\x7F\x0B*\xAC\x84\xF3\xEC\x95i\x11\xFDx\xEA\xE51\x10b\x97/\xF9I\xF3\x84\x12\xE8\xDA9\x06\x9D\x9F\x06\x8C\xC6\x90` \x01`@Q\x80\x91\x03\x90\xA3a\x13\xDB\x85\x85\x85`@Q\x80` \x01`@R\x80_\x81RPa\x1B$V[a\x14=W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FERC3525: transfer rejected by ER`D\x82\x01Rl!\x99\x9A\x99\x1A\xA92\xB1\xB2\xB4\xBB2\xB9`\x99\x1B`d\x82\x01R`\x84\x01a\x06ZV[PPPPPV[__a\x14O\x83a\t\xEFV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x14\x99WP`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x07` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R`\x02\x90\x93\x01\x90R T`\xFF\x16[\x80a\x14\xBDWP\x83`\x01`\x01`\xA0\x1B\x03\x16a\x14\xB2\x84a\x05\xCEV[`\x01`\x01`\xA0\x1B\x03\x16\x14[\x94\x93PPPPV[\x82`\x01`\x01`\xA0\x1B\x03\x16a\x14\xD8\x82a\t\xEFV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x15:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x80\x82\x01R\x7FERC3525: transfer from invalid o`D\x82\x01Rc;\xB72\xB9`\xE1\x1B`d\x82\x01R`\x84\x01a\x06ZV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x15\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FERC3525: transfer to the zero ad`D\x82\x01Rddress`\xD8\x1B`d\x82\x01R`\x84\x01a\x06ZV[_a\x15\xA8\x82a\x08QV[\x90P_a\x15\xB4\x83a\x0B\xDBV[\x90Pa\x15\xC0_\x84a\x0ErV[a\x15\xC9\x83a\x1C\x90V[a\x15\xD3\x85\x84a\x1D,V[a\x15\xDD\x84\x84a\x1E?V[\x82\x84`\x01`\x01`\xA0\x1B\x03\x16\x86`\x01`\x01`\xA0\x1B\x03\x16\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF`@Q`@Q\x80\x91\x03\x90\xA4a\x14=V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x16\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FERC3525: approve value to the ze`D\x82\x01Riro address`\xB0\x1B`d\x82\x01R`\x84\x01a\x06ZV[a\x16\x96\x82\x84a\x1E\xC6V[a\x16\xFAW_\x83\x81R`\x06` R`@\x90 T`\x05\x80T\x90\x91\x90\x81\x10a\x16\xBDWa\x16\xBDa'[V[_\x91\x82R` \x80\x83 `\x06\x92\x90\x92\x02\x90\x91\x01`\x05\x01\x80T`\x01\x81\x01\x82U\x90\x83R\x91 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U[_\x83\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x80\x85R\x90\x83R\x92\x81\x90 \x84\x90UQ\x83\x81R\x85\x91\x7Fb\x1B\x05\r\xE0\xAD\x08\xB5\x1D\x19\xB4\x8B>m\xF7SH\xC4\xDEk\xDD\x93\xE8\x1B%,\xA6.(&[\x1B\x91\x01[`@Q\x80\x91\x03\x90\xA3PPPV[\x81`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x17\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FERC3525: approve to caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06ZV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`\x07` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R`\x02\x90\x95\x01\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01a\x17KV[a\x18,\x84\x84\x84a\x14\xC5V[a\x188\x84\x84\x84\x84a\x1F\x88V[a\x0B\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FERC3525: transfer to non ERC721R`D\x82\x01Rf2\xB1\xB2\xB4\xBB2\xB9`\xC9\x1B`d\x82\x01R`\x84\x01a\x06ZV[``a\x059`\x01`\x01`\xA0\x1B\x03\x83\x16`\x14a \xCAV[`\x05T_\x90\x15\x80\x15\x90a\x059WP_\x82\x81R`\x06` R`@\x90 T`\x05\x80T\x84\x92\x90\x81\x10a\x18\xDBWa\x18\xDBa'[V[\x90_R` _ \x90`\x06\x02\x01_\x01T\x14\x92\x91PPV[_\x80r\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x10a\x19/Wr\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01`@\x1B\x83\x04\x92P`@\x01[m\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x10a\x19[Wm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x83\x04\x92P` \x01[f#\x86\xF2o\xC1\0\0\x83\x10a\x19yWf#\x86\xF2o\xC1\0\0\x83\x04\x92P`\x10\x01[c\x05\xF5\xE1\0\x83\x10a\x19\x91Wc\x05\xF5\xE1\0\x83\x04\x92P`\x08\x01[a'\x10\x83\x10a\x19\xA5Wa'\x10\x83\x04\x92P`\x04\x01[`d\x83\x10a\x19\xB7W`d\x83\x04\x92P`\x02\x01[`\n\x83\x10a\x059W`\x01\x01\x92\x91PPV[`\x03\x80T_\x91\x82a\x19\xD8\x83a)0V[\x91\x90PU\x90P\x90V[`@\x80Q`\xC0\x81\x01\x82R\x83\x81R` \x80\x82\x01\x84\x90R_\x82\x84\x01\x81\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16``\x84\x01R`\x80\x83\x01\x81\x90R\x83Q\x81\x81R\x91\x82\x01\x90\x93R`\xA0\x82\x01R\x90Pa\x1A.\x81a\"<V[a\x1A8\x84\x84a\x1E?V[`@Q\x83\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90_\x90\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x82\x90\xA4\x81_\x84\x7F\xE4\xF4\x8C$\r;\x99IH\xAAT\xF3\xE2\xF5\xFC\xA5\x92c\xDF\xE1\xD5+nL\xF3\x9A]$\x9B\\\xCBe`@Q`@Q\x80\x91\x03\x90\xA4PPPPV[_\x82\x81R`\x06` R`@\x90 T`\x05\x80T\x83\x92\x90\x81\x10a\x1A\xC7Wa\x1A\xC7a'[V[\x90_R` _ \x90`\x06\x02\x01`\x02\x01_\x82\x82Ta\x1A\xE4\x91\x90a)\x1DV[\x90\x91UPP`@Q\x81\x81R\x82\x90_\x90\x7F\x0B*\xAC\x84\xF3\xEC\x95i\x11\xFDx\xEA\xE51\x10b\x97/\xF9I\xF3\x84\x12\xE8\xDA9\x06\x9D\x9F\x06\x8C\xC6\x90` \x01`@Q\x80\x91\x03\x90\xA3PPV[__a\x1B/\x85a\t\xEFV[\x90P\x80;c\xFF\xFF\xFF\xFF\x16\x15a\x1C\x84W`@Qc\x01\xFF\xC9\xA7`\xE0\x1B\x81Rb\x9C\xE2\x0B`\xE0\x1B`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x01\xFF\xC9\xA7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x92PPP\x80\x15a\x1B\xA4WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1B\xA1\x91\x81\x01\x90a)HV[`\x01[a\x1B\xE2W=\x80\x80\x15a\x1B\xD1W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x1B\xD6V[``\x91P[P`\x01\x92PPPa\x14\xBDV[\x80\x15a\x1CyW`@Qb\x9C\xE2\x0B`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90b\x9C\xE2\x0B\x90a\x1C\x1C\x903\x90\x8C\x90\x8C\x90\x8C\x90\x8C\x90`\x04\x01a)cV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\\\x91\x90a)\xA0V[`\x01`\x01`\xE0\x1B\x03\x19\x16b\x9C\xE2\x0B`\xE0\x1B\x14\x93Pa\x14\xBD\x92PPPV[`\x01\x92PPPa\x14\xBDV[P`\x01\x95\x94PPPPPV[_\x81\x81R`\x06` R`@\x81 T`\x05\x80T\x90\x91\x90\x81\x10a\x1C\xB3Wa\x1C\xB3a'[V[_\x91\x82R` \x82 `\x05`\x06\x90\x92\x02\x01\x90\x81\x01T\x90\x92P\x90[\x81\x81\x10\x15a\x1D\x1EW_\x83`\x05\x01\x82\x81T\x81\x10a\x1C\xEAWa\x1C\xEAa'[V[_\x91\x82R` \x80\x83 \x90\x91\x01T\x87\x83R`\x04\x82R`@\x80\x84 `\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x84R\x91R\x81 UP`\x01\x01a\x1C\xCCV[Pa\x06\xFB`\x05\x83\x01_a#\x8AV[_\x81\x81R`\x06` R`@\x81 T`\x05\x80T\x90\x91\x90\x81\x10a\x1DOWa\x1DOa'[V[_\x91\x82R` \x80\x83 `\x06\x92\x90\x92\x02\x90\x91\x01`\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90U\x91\x84\x16\x81R`\x07\x90\x91R`@\x81 \x80T\x90\x91\x90a\x1D\x9D\x90`\x01\x90a)\nV[\x90P_\x82_\x01\x82\x81T\x81\x10a\x1D\xB4Wa\x1D\xB4a'[V[\x90_R` _ \x01T\x90P_\x83`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ T\x90P\x81\x84_\x01\x82\x81T\x81\x10a\x1D\xEBWa\x1D\xEBa'[V[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x83\x81R`\x01\x86\x01\x90\x91R`@\x80\x82 \x83\x90U\x86\x82R\x81 U\x83T\x84\x90\x80a\x1E$Wa\x1E$a)\xBBV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90UPPPPPPV[_\x81\x81R`\x06` R`@\x90 T`\x05\x80T\x84\x92\x90\x81\x10a\x1EbWa\x1Eba'[V[_\x91\x82R` \x80\x83 `\x06\x92\x90\x92\x02\x90\x91\x01`\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x17\x90U\x93\x90\x91\x16\x81R`\x07\x80\x84R`@\x80\x83 \x80T\x85\x85R`\x01\x82\x81\x01\x88R\x92\x85 \x81\x90U\x92\x86R\x90\x82\x01\x81U\x82R\x92\x90 \x90\x91\x01UV[_\x81\x81R`\x06` R`@\x81 T`\x05\x80T\x83\x92\x90\x81\x10a\x1E\xE9Wa\x1E\xE9a'[V[_\x91\x82R` \x82 `\x05`\x06\x90\x92\x02\x01\x01T\x91P[\x81\x81\x10\x15a\x1F~W_\x84\x81R`\x06` R`@\x90 T`\x05\x80T`\x01`\x01`\xA0\x1B\x03\x88\x16\x92\x90\x81\x10a\x1F2Wa\x1F2a'[V[\x90_R` _ \x90`\x06\x02\x01`\x05\x01\x82\x81T\x81\x10a\x1FRWa\x1FRa'[V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1FvW`\x01\x92PPPa\x059V[`\x01\x01a\x1E\xFEV[P_\x94\x93PPPPV[_\x83;c\xFF\xFF\xFF\xFF\x16\x15a \xC2W`@Qc\n\x85\xBD\x01`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x15\x0Bz\x02\x90a\x1F\xC8\x903\x90\x89\x90\x88\x90\x88\x90`\x04\x01a)\xCFV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a \x02WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\x1F\xFF\x91\x81\x01\x90a)\xA0V[`\x01[a \xA8W=\x80\x80\x15a /W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a 4V[``\x91P[P\x80Q_\x03a \xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FERC721: transfer to non ERC721Re`D\x82\x01Rq1\xB2\xB4\xBB2\xB9\x104\xB6\xB862\xB6\xB2\xB7:2\xB9`q\x1B`d\x82\x01R`\x84\x01a\x06ZV[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x16c\n\x85\xBD\x01`\xE1\x1B\x14\x90Pa\x14\xBDV[P`\x01a\x14\xBDV[``\x82_a \xD9\x84`\x02a*\x0BV[a \xE4\x90`\x02a)\x1DV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a \xFCWa \xFCa%\xC1V[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a!&W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\x03`\xFC\x1B\x81_\x81Q\x81\x10a!@Wa!@a'[V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x0F`\xFB\x1B\x81`\x01\x81Q\x81\x10a!nWa!na'[V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP_a!\x90\x85`\x02a*\x0BV[a!\x9B\x90`\x01a)\x1DV[\x90P[`\x01\x81\x11\x15a\"\x12Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x83`\x0F\x16`\x10\x81\x10a!\xCFWa!\xCFa'[V[\x1A`\xF8\x1B\x82\x82\x81Q\x81\x10a!\xE5Wa!\xE5a'[V[` \x01\x01\x90`\x01`\x01`\xF8\x1B\x03\x19\x16\x90\x81_\x1A\x90SP`\x04\x92\x90\x92\x1C\x91a\"\x0B\x81a*\"V[\x90Pa!\x9EV[P\x81\x15a\x14\xBDW`@Qc\xE2.'\xEB`\xE0\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x85\x90R`D\x01a\x06ZV[`\x05\x80T\x82Q_\x90\x81R`\x06` \x81\x81R`@\x80\x84 \x85\x90U`\x01\x85\x01\x86U\x94\x90\x92R\x84Q\x92\x02\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB0\x81\x01\x92\x83U\x81\x85\x01Q\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB1\x82\x01U\x92\x84\x01Q\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB2\x84\x01U``\x84\x01Q\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB3\x84\x01\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\x80\x86\x01Q\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB4\x86\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90U`\xA0\x84\x01Q\x80Q\x85\x94a\x0B\xC6\x93\x7F\x03kc\x84\xB5\xEC\xA7\x91\xC6'a\x15-\x0Cy\xBB\x06\x04\xC1\x04\xA5\xFBoN\xB0p?1T\xBB=\xB5\x90\x91\x01\x92\x01\x90a#\xA5V[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a\x0Eo\x91\x90a$\x08V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15a#\xF8W\x91` \x02\x82\x01[\x82\x81\x11\x15a#\xF8W\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a#\xC3V[Pa$\x04\x92\x91Pa$\x08V[P\x90V[[\x80\x82\x11\x15a$\x04W_\x81U`\x01\x01a$\tV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0EoW__\xFD[_` \x82\x84\x03\x12\x15a$AW__\xFD[\x815a\x07\xE3\x81a$\x1CV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x07\xE3` \x83\x01\x84a$LV[_` \x82\x84\x03\x12\x15a$\x9CW__\xFD[P5\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\x83W__\xFD[__`@\x83\x85\x03\x12\x15a$\xCAW__\xFD[a$\xD3\x83a$\xA3V[\x94` \x93\x90\x93\x015\x93PPPV[___``\x84\x86\x03\x12\x15a$\xF3W__\xFD[\x835\x92Pa%\x03` \x85\x01a$\xA3V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[___``\x84\x86\x03\x12\x15a%&W__\xFD[a%/\x84a$\xA3V[\x92Pa%\x03` \x85\x01a$\xA3V[___``\x84\x86\x03\x12\x15a%OW__\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[_` \x82\x84\x03\x12\x15a%vW__\xFD[a\x07\xE3\x82a$\xA3V[\x80\x15\x15\x81\x14a\x0EoW__\xFD[__`@\x83\x85\x03\x12\x15a%\x9DW__\xFD[a%\xA6\x83a$\xA3V[\x91P` \x83\x015a%\xB6\x81a%\x7FV[\x80\x91PP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\xFEWa%\xFEa%\xC1V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a&\x1FWa&\x1Fa%\xC1V[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[____`\x80\x85\x87\x03\x12\x15a&@W__\xFD[a&I\x85a$\xA3V[\x93Pa&W` \x86\x01a$\xA3V[\x92P`@\x85\x015\x91P``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&yW__\xFD[\x85\x01`\x1F\x81\x01\x87\x13a&\x89W__\xFD[\x805a&\x9Ca&\x97\x82a&\x06V[a%\xD5V[\x81\x81R\x88` \x83\x85\x01\x01\x11\x15a&\xB0W__\xFD[\x81` \x84\x01` \x83\x017_` \x83\x83\x01\x01R\x80\x93PPPP\x92\x95\x91\x94P\x92PV[__`@\x83\x85\x03\x12\x15a&\xE2W__\xFD[\x825\x91Pa&\xF2` \x84\x01a$\xA3V[\x90P\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15a'\x0CW__\xFD[a'\x15\x83a$\xA3V[\x91Pa&\xF2` \x84\x01a$\xA3V[`\x01\x81\x81\x1C\x90\x82\x16\x80a'7W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a'UWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[` \x80\x82R`\"\x90\x82\x01R\x7FERC3525: approval to current own`@\x82\x01Ra2\xB9`\xF1\x1B``\x82\x01R`\x80\x01\x90V[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a'\xD3\x82\x85a'\xB1V[dslot/`\xD8\x1B\x81Ra'\xEB`\x05\x82\x01\x85a'\xB1V[\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a(\x04W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x1AW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a(*W__\xFD[\x80Qa(8a&\x97\x82a&\x06V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a(LW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[` \x80\x82R`2\x90\x82\x01R\x7FERC3525: transfer caller is not `@\x82\x01Rq\x1B\xDD\xDB\x99\\\x88\x1B\x9B\xDC\x88\x18\\\x1C\x1C\x9B\xDD\x99Y`r\x1B``\x82\x01R`\x80\x01\x90V[_a\x14\xBDa(\xC9\x83\x86a'\xB1V[\x84a'\xB1V[_a(\xDA\x82\x85a'\xB1V[hcontract/`\xB8\x1B\x81Ra'\xEB`\t\x82\x01\x85a'\xB1V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x059Wa\x059a(\xF6V[\x80\x82\x01\x80\x82\x11\x15a\x059Wa\x059a(\xF6V[_`\x01\x82\x01a)AWa)Aa(\xF6V[P`\x01\x01\x90V[_` \x82\x84\x03\x12\x15a)XW__\xFD[\x81Qa\x07\xE3\x81a%\x7FV[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R\x83`@\x82\x01R\x82``\x82\x01R`\xA0`\x80\x82\x01R_a)\x95`\xA0\x83\x01\x84a$LV[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a)\xB0W__\xFD[\x81Qa\x07\xE3\x81a$\x1CV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R`@\x81\x01\x83\x90R`\x80``\x82\x01\x81\x90R_\x90a*\x01\x90\x83\x01\x84a$LV[\x96\x95PPPPPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x059Wa\x059a(\xF6V[_\x81a*0Wa*0a(\xF6V[P_\x19\x01\x90V\xFE\xA2dipfsX\"\x12 O\xE6\xF8u\xDF\"#~\xB2\"?\x88\n%\xF4uL9{w\x86i\r\x8D\x90\xE3\xE3b\x8E\xF1f-dsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static ERC3525_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ERC3525<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC3525<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC3525<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC3525<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC3525<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ERC3525)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC3525<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ERC3525_ABI.clone(),
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
                ERC3525_ABI.clone(),
                ERC3525_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `allowance` (0xe345e0bc) function
        pub fn allowance(
            &self,
            token_id: ::ethers::core::types::U256,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 69, 224, 188], (token_id, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x8cb0a511) function
        pub fn approve_with_token_id_and_to(
            &self,
            token_id: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 176, 165, 17], (token_id, to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x9cc7f708) function
        pub fn balance_of_with_token_id(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([156, 199, 247, 8], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `contractURI` (0xe8a3d485) function
        pub fn contract_uri(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([232, 163, 212, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getApproved` (0x081812fc) function
        pub fn get_approved(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([8, 24, 18, 252], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            owner: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (owner, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `metadataDescriptor` (0x840f7113) function
        pub fn metadata_descriptor(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([132, 15, 113, 19], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ownerOf` (0x6352211e) function
        pub fn owner_of(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([99, 82, 33, 30], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0x42842e0e) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0xb88d4fde) function
        pub fn safe_transfer_from_with_from_and_to_and_data(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, token_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `slotOf` (0x263f3e7e) function
        pub fn slot_of(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([38, 63, 62, 126], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `slotURI` (0x09c3dd87) function
        pub fn slot_uri(
            &self,
            slot: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([9, 195, 221, 135], slot)
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
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenByIndex` (0x4f6ccce7) function
        pub fn token_by_index(
            &self,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([79, 108, 204, 231], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenOfOwnerByIndex` (0x2f745c59) function
        pub fn token_of_owner_by_index(
            &self,
            owner: ::ethers::core::types::Address,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([47, 116, 92, 89], (owner, index))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenURI` (0xc87b56dd) function
        pub fn token_uri(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 123, 86, 221], token_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x0f485c02) function
        pub fn transfer_from(
            &self,
            from_token_id: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([15, 72, 92, 2], (from_token_id, to, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from_with_from_and_to_and_token_id(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, token_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x310ed7f0) function
        pub fn transfer_from_with_from_token_id_and_to_token_id(
            &self,
            from_token_id: ::ethers::core::types::U256,
            to_token_id: ::ethers::core::types::U256,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 14, 215, 240], (from_token_id, to_token_id, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `valueDecimals` (0x3e7e8669) function
        pub fn value_decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([62, 126, 134, 105], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalForAllFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ApprovalValue` event
        pub fn approval_value_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ApprovalValueFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetMetadataDescriptor` event
        pub fn set_metadata_descriptor_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetMetadataDescriptorFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SlotChanged` event
        pub fn slot_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SlotChangedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TransferValue` event
        pub fn transfer_value_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TransferValueFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ERC3525Events> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ERC3525<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `StringsInsufficientHexLength` with signature `StringsInsufficientHexLength(uint256,uint256)` and selector `0xe22e27eb`
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
        name = "StringsInsufficientHexLength",
        abi = "StringsInsufficientHexLength(uint256,uint256)"
    )]
    pub struct StringsInsufficientHexLength {
        pub value: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub approved: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
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
    #[ethevent(name = "ApprovalValue", abi = "ApprovalValue(uint256,address,uint256)")]
    pub struct ApprovalValueFilter {
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
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
    #[ethevent(name = "SetMetadataDescriptor", abi = "SetMetadataDescriptor(address)")]
    pub struct SetMetadataDescriptorFilter {
        #[ethevent(indexed)]
        pub metadata_descriptor: ::ethers::core::types::Address,
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
    #[ethevent(name = "SlotChanged", abi = "SlotChanged(uint256,uint256,uint256)")]
    pub struct SlotChangedFilter {
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub old_slot: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub new_slot: ::ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_id: ::ethers::core::types::U256,
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
    #[ethevent(name = "TransferValue", abi = "TransferValue(uint256,uint256,uint256)")]
    pub struct TransferValueFilter {
        #[ethevent(indexed)]
        pub from_token_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub to_token_id: ::ethers::core::types::U256,
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
    pub enum ERC3525Events {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        ApprovalValueFilter(ApprovalValueFilter),
        SetMetadataDescriptorFilter(SetMetadataDescriptorFilter),
        SlotChangedFilter(SlotChangedFilter),
        TransferFilter(TransferFilter),
        TransferValueFilter(TransferValueFilter),
    }
    impl ::ethers::contract::EthLogDecode for ERC3525Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ERC3525Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ERC3525Events::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = ApprovalValueFilter::decode_log(log) {
                return Ok(ERC3525Events::ApprovalValueFilter(decoded));
            }
            if let Ok(decoded) = SetMetadataDescriptorFilter::decode_log(log) {
                return Ok(ERC3525Events::SetMetadataDescriptorFilter(decoded));
            }
            if let Ok(decoded) = SlotChangedFilter::decode_log(log) {
                return Ok(ERC3525Events::SlotChangedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ERC3525Events::TransferFilter(decoded));
            }
            if let Ok(decoded) = TransferValueFilter::decode_log(log) {
                return Ok(ERC3525Events::TransferValueFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ERC3525Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApprovalValueFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetMetadataDescriptorFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SlotChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferValueFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for ERC3525Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for ERC3525Events {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalValueFilter> for ERC3525Events {
        fn from(value: ApprovalValueFilter) -> Self {
            Self::ApprovalValueFilter(value)
        }
    }
    impl ::core::convert::From<SetMetadataDescriptorFilter> for ERC3525Events {
        fn from(value: SetMetadataDescriptorFilter) -> Self {
            Self::SetMetadataDescriptorFilter(value)
        }
    }
    impl ::core::convert::From<SlotChangedFilter> for ERC3525Events {
        fn from(value: SlotChangedFilter) -> Self {
            Self::SlotChangedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for ERC3525Events {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<TransferValueFilter> for ERC3525Events {
        fn from(value: TransferValueFilter) -> Self {
            Self::TransferValueFilter(value)
        }
    }
    ///Container type for all input parameters for the `allowance` function with signature `allowance(uint256,address)` and selector `0xe345e0bc`
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
    #[ethcall(name = "allowance", abi = "allowance(uint256,address)")]
    pub struct AllowanceCall {
        pub token_id: ::ethers::core::types::U256,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `approve` function with signature `approve(uint256,address,uint256)` and selector `0x8cb0a511`
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
    #[ethcall(name = "approve", abi = "approve(uint256,address,uint256)")]
    pub struct ApproveWithTokenIdAndToCall {
        pub token_id: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(uint256)` and selector `0x9cc7f708`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(uint256)")]
    pub struct BalanceOfWithTokenIdCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `contractURI` function with signature `contractURI()` and selector `0xe8a3d485`
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
    #[ethcall(name = "contractURI", abi = "contractURI()")]
    pub struct ContractURICall;
    ///Container type for all input parameters for the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub owner: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `metadataDescriptor` function with signature `metadataDescriptor()` and selector `0x840f7113`
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
    #[ethcall(name = "metadataDescriptor", abi = "metadataDescriptor()")]
    pub struct MetadataDescriptorCall;
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256)` and selector `0x42842e0e`
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,bytes)` and selector `0xb88d4fde`
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndDataCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`
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
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    ///Container type for all input parameters for the `slotOf` function with signature `slotOf(uint256)` and selector `0x263f3e7e`
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
    #[ethcall(name = "slotOf", abi = "slotOf(uint256)")]
    pub struct SlotOfCall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `slotURI` function with signature `slotURI(uint256)` and selector `0x09c3dd87`
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
    #[ethcall(name = "slotURI", abi = "slotURI(uint256)")]
    pub struct SlotURICall {
        pub slot: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `tokenByIndex` function with signature `tokenByIndex(uint256)` and selector `0x4f6ccce7`
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
    #[ethcall(name = "tokenByIndex", abi = "tokenByIndex(uint256)")]
    pub struct TokenByIndexCall {
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `tokenOfOwnerByIndex` function with signature `tokenOfOwnerByIndex(address,uint256)` and selector `0x2f745c59`
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
        name = "tokenOfOwnerByIndex",
        abi = "tokenOfOwnerByIndex(address,uint256)"
    )]
    pub struct TokenOfOwnerByIndexCall {
        pub owner: ::ethers::core::types::Address,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(uint256,address,uint256)` and selector `0x0f485c02`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(uint256,address,uint256)")]
    pub struct TransferFromCall {
        pub from_token_id: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromWithFromAndToAndTokenIdCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(uint256,uint256,uint256)` and selector `0x310ed7f0`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(uint256,uint256,uint256)")]
    pub struct TransferFromWithFromTokenIdAndToTokenIdCall {
        pub from_token_id: ::ethers::core::types::U256,
        pub to_token_id: ::ethers::core::types::U256,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `valueDecimals` function with signature `valueDecimals()` and selector `0x3e7e8669`
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
    #[ethcall(name = "valueDecimals", abi = "valueDecimals()")]
    pub struct ValueDecimalsCall;
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
    pub enum ERC3525Calls {
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        ApproveWithTokenIdAndTo(ApproveWithTokenIdAndToCall),
        BalanceOf(BalanceOfCall),
        BalanceOfWithTokenId(BalanceOfWithTokenIdCall),
        ContractURI(ContractURICall),
        GetApproved(GetApprovedCall),
        IsApprovedForAll(IsApprovedForAllCall),
        MetadataDescriptor(MetadataDescriptorCall),
        Name(NameCall),
        OwnerOf(OwnerOfCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SlotOf(SlotOfCall),
        SlotURI(SlotURICall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenByIndex(TokenByIndexCall),
        TokenOfOwnerByIndex(TokenOfOwnerByIndexCall),
        TokenURI(TokenURICall),
        TotalSupply(TotalSupplyCall),
        TransferFrom(TransferFromCall),
        TransferFromWithFromAndToAndTokenId(TransferFromWithFromAndToAndTokenIdCall),
        TransferFromWithFromTokenIdAndToTokenId(
            TransferFromWithFromTokenIdAndToTokenIdCall,
        ),
        ValueDecimals(ValueDecimalsCall),
    }
    impl ::ethers::core::abi::AbiDecode for ERC3525Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <ApproveWithTokenIdAndToCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApproveWithTokenIdAndTo(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BalanceOfWithTokenIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BalanceOfWithTokenId(decoded));
            }
            if let Ok(decoded) = <ContractURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ContractURI(decoded));
            }
            if let Ok(decoded) = <GetApprovedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetApproved(decoded));
            }
            if let Ok(decoded) = <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <MetadataDescriptorCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MetadataDescriptor(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OwnerOf(decoded));
            }
            if let Ok(decoded) = <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) = <SafeTransferFromWithFromAndToAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded) = <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) = <SlotOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SlotOf(decoded));
            }
            if let Ok(decoded) = <SlotURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SlotURI(decoded));
            }
            if let Ok(decoded) = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TokenByIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenByIndex(decoded));
            }
            if let Ok(decoded) = <TokenOfOwnerByIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenOfOwnerByIndex(decoded));
            }
            if let Ok(decoded) = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenURI(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) = <TransferFromWithFromAndToAndTokenIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFromWithFromAndToAndTokenId(decoded));
            }
            if let Ok(decoded) = <TransferFromWithFromTokenIdAndToTokenIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferFromWithFromTokenIdAndToTokenId(decoded));
            }
            if let Ok(decoded) = <ValueDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValueDecimals(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC3525Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Allowance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ApproveWithTokenIdAndTo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BalanceOfWithTokenId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ContractURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsApprovedForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MetadataDescriptor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnerOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SlotOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SlotURI(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenOfOwnerByIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFromWithFromAndToAndTokenId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFromWithFromTokenIdAndToTokenId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValueDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ERC3525Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveWithTokenIdAndTo(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOfWithTokenId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ContractURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::MetadataDescriptor(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SlotOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::SlotURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenByIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenOfOwnerByIndex(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFromWithFromAndToAndTokenId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferFromWithFromTokenIdAndToTokenId(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValueDecimals(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllowanceCall> for ERC3525Calls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for ERC3525Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<ApproveWithTokenIdAndToCall> for ERC3525Calls {
        fn from(value: ApproveWithTokenIdAndToCall) -> Self {
            Self::ApproveWithTokenIdAndTo(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for ERC3525Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BalanceOfWithTokenIdCall> for ERC3525Calls {
        fn from(value: BalanceOfWithTokenIdCall) -> Self {
            Self::BalanceOfWithTokenId(value)
        }
    }
    impl ::core::convert::From<ContractURICall> for ERC3525Calls {
        fn from(value: ContractURICall) -> Self {
            Self::ContractURI(value)
        }
    }
    impl ::core::convert::From<GetApprovedCall> for ERC3525Calls {
        fn from(value: GetApprovedCall) -> Self {
            Self::GetApproved(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for ERC3525Calls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<MetadataDescriptorCall> for ERC3525Calls {
        fn from(value: MetadataDescriptorCall) -> Self {
            Self::MetadataDescriptor(value)
        }
    }
    impl ::core::convert::From<NameCall> for ERC3525Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerOfCall> for ERC3525Calls {
        fn from(value: OwnerOfCall) -> Self {
            Self::OwnerOf(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for ERC3525Calls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromWithFromAndToAndDataCall>
    for ERC3525Calls {
        fn from(value: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            Self::SafeTransferFromWithFromAndToAndData(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for ERC3525Calls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SlotOfCall> for ERC3525Calls {
        fn from(value: SlotOfCall) -> Self {
            Self::SlotOf(value)
        }
    }
    impl ::core::convert::From<SlotURICall> for ERC3525Calls {
        fn from(value: SlotURICall) -> Self {
            Self::SlotURI(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ERC3525Calls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for ERC3525Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenByIndexCall> for ERC3525Calls {
        fn from(value: TokenByIndexCall) -> Self {
            Self::TokenByIndex(value)
        }
    }
    impl ::core::convert::From<TokenOfOwnerByIndexCall> for ERC3525Calls {
        fn from(value: TokenOfOwnerByIndexCall) -> Self {
            Self::TokenOfOwnerByIndex(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for ERC3525Calls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for ERC3525Calls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for ERC3525Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferFromWithFromAndToAndTokenIdCall>
    for ERC3525Calls {
        fn from(value: TransferFromWithFromAndToAndTokenIdCall) -> Self {
            Self::TransferFromWithFromAndToAndTokenId(value)
        }
    }
    impl ::core::convert::From<TransferFromWithFromTokenIdAndToTokenIdCall>
    for ERC3525Calls {
        fn from(value: TransferFromWithFromTokenIdAndToTokenIdCall) -> Self {
            Self::TransferFromWithFromTokenIdAndToTokenId(value)
        }
    }
    impl ::core::convert::From<ValueDecimalsCall> for ERC3525Calls {
        fn from(value: ValueDecimalsCall) -> Self {
            Self::ValueDecimals(value)
        }
    }
    ///Container type for all return fields from the `allowance` function with signature `allowance(uint256,address)` and selector `0xe345e0bc`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn {
        pub balance: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(uint256)` and selector `0x9cc7f708`
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
    pub struct BalanceOfWithTokenIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `contractURI` function with signature `contractURI()` and selector `0xe8a3d485`
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
    pub struct ContractURIReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getApproved` function with signature `getApproved(uint256)` and selector `0x081812fc`
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
    pub struct GetApprovedReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
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
    pub struct IsApprovedForAllReturn(pub bool);
    ///Container type for all return fields from the `metadataDescriptor` function with signature `metadataDescriptor()` and selector `0x840f7113`
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
    pub struct MetadataDescriptorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `ownerOf` function with signature `ownerOf(uint256)` and selector `0x6352211e`
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
    pub struct OwnerOfReturn {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `slotOf` function with signature `slotOf(uint256)` and selector `0x263f3e7e`
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
    pub struct SlotOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `slotURI` function with signature `slotURI(uint256)` and selector `0x09c3dd87`
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
    pub struct SlotURIReturn(pub ::std::string::String);
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
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `tokenByIndex` function with signature `tokenByIndex(uint256)` and selector `0x4f6ccce7`
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
    pub struct TokenByIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenOfOwnerByIndex` function with signature `tokenOfOwnerByIndex(address,uint256)` and selector `0x2f745c59`
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
    pub struct TokenOfOwnerByIndexReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenURI` function with signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    pub struct TokenURIReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(uint256,address,uint256)` and selector `0x0f485c02`
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
    pub struct TransferFromReturn {
        pub new_token_id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `valueDecimals` function with signature `valueDecimals()` and selector `0x3e7e8669`
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
    pub struct ValueDecimalsReturn(pub u8);
}
