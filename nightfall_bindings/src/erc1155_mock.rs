pub use erc1155_mock::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod erc1155_mock {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initial_id"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("value"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initial_id_two"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("value_two"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initialOwner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOfBatch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balanceOfBatch"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("accounts"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ids"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mint"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("safeBatchTransferFrom"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("safeBatchTransferFrom",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ids"),
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
                                name: ::std::borrow::ToOwned::to_owned("values"),
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
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("approved"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes4"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("uri"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("uri"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("approved"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferBatch"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TransferBatch"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("ids"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("values"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferSingle"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TransferSingle"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("URI"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("URI"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InsufficientBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InsufficientBalance",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("needed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InvalidApprover"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InvalidApprover",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("approver"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InvalidArrayLength"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InvalidArrayLength",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("idsLength"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("valuesLength"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InvalidOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InvalidOperator",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InvalidReceiver"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InvalidReceiver",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("receiver"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InvalidSender"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InvalidSender",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("sender"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155MissingApprovalForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155MissingApprovalForAll",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ERC1155MOCK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x10W_\x80\xFD[P`@Qb\0\x1A_8\x03\x80b\0\x1A_\x839\x81\x01`@\x81\x90Rb\0\x003\x91b\0\x05\xD4V[`@\x80Q\x80\x82\x01\x90\x91R`\x0B\x81RjERC1155Mock`\xA8\x1B` \x82\x01Rb\0\0`\x81b\0\0\x86V[Pb\0\0n\x81\x86\x86b\0\0\x98V[b\0\0{\x81\x84\x84b\0\0\x98V[PPPPPb\0\t;V[`\x02b\0\0\x94\x82\x82b\0\x06\xC2V[PPV[b\0\0\xBA\x83\x83\x83`@Q\x80` \x01`@R\x80_\x81RPb\0\0\xBF` \x1B` \x1CV[PPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16b\0\0\xEEW`@Qc+\xFA#\xE7`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01\x80\x82R` \x82\x01\x86\x90R\x81\x83\x01\x90\x81R``\x82\x01\x85\x90R`\x80\x82\x01\x90\x92R\x90b\0\x01\"_\x87\x84\x84\x87b\0\x01*V[PPPPPPV[b\0\x018\x85\x85\x85\x85b\0\x01\x8DV[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15b\0\x01\x86W\x82Q3\x90`\x01\x03b\0\x01vW` \x84\x81\x01Q\x90\x84\x01Qb\0\x01n\x83\x89\x89\x85\x85\x89b\0\x03\xABV[PPb\0\x01\"V[b\0\x01\"\x81\x87\x87\x87\x87\x87b\0\x04\xE2V[PPPPPV[\x80Q\x82Q\x14b\0\x01\xBEW\x81Q\x81Q`@Qc[\x05\x99\x91`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01b\0\0\xE5V[3_[\x83Q\x81\x10\x15b\0\x02\xC8W` \x81\x81\x02\x85\x81\x01\x82\x01Q\x90\x85\x01\x90\x91\x01Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x15b\0\x02vW_\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x8C\x16\x84R\x90\x91R\x90 T\x81\x81\x10\x15b\0\x02PW`@Qc\x03\xDE\xE4\xC5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x81\x01\x84\x90R`\x84\x01b\0\0\xE5V[_\x83\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x8D\x16\x84R\x90\x91R\x90 \x90\x82\x90\x03\x90U[`\x01`\x01`\xA0\x1B\x03\x87\x16\x15b\0\x02\xBDW_\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x8B\x16\x84R\x90\x91R\x81 \x80T\x83\x92\x90b\0\x02\xB7\x90\x84\x90b\0\x07\x8AV[\x90\x91UPP[PP`\x01\x01b\0\x01\xC1V[P\x82Q`\x01\x03b\0\x03KW` \x83\x01Q_\x90` \x84\x01Q\x90\x91P\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb\x85\x85`@Qb\0\x03;\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPb\0\x01\x86V[\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x7FJ9\xDC\x06\xD4\xC0\xDB\xC6Kp\xAF\x90\xFDi\x8A#:Q\x8A\xA5\xD0~Y]\x98;\x8C\x05&\xC8\xF7\xFB\x86\x86`@Qb\0\x03\x9C\x92\x91\x90b\0\x07\xECV[`@Q\x80\x91\x03\x90\xA4PPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15b\0\x01\"W`@Qc\xF2:na`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xF2:na\x90b\0\x03\xF2\x90\x89\x90\x89\x90\x88\x90\x88\x90\x88\x90`\x04\x01b\0\x08bV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15b\0\x04/WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Rb\0\x04,\x91\x81\x01\x90b\0\x08\xA8V[`\x01[b\0\x04\x9BW=\x80\x80\x15b\0\x04_W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>b\0\x04dV[``\x91P[P\x80Q_\x03b\0\x04\x93W`@Qc+\xFA#\xE7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x04\x82\x01R`$\x01b\0\0\xE5V[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16c\xF2:na`\xE0\x1B\x14b\0\x04\xD9W`@Qc+\xFA#\xE7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x04\x82\x01R`$\x01b\0\0\xE5V[PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15b\0\x01\"W`@Qc\xBC\x19|\x81`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xBC\x19|\x81\x90b\0\x05)\x90\x89\x90\x89\x90\x88\x90\x88\x90\x88\x90`\x04\x01b\0\x08\xD8V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15b\0\x05fWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Rb\0\x05c\x91\x81\x01\x90b\0\x08\xA8V[`\x01[b\0\x05\x96W=\x80\x80\x15b\0\x04_W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>b\0\x04dV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16c\xBC\x19|\x81`\xE0\x1B\x14b\0\x04\xD9W`@Qc+\xFA#\xE7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x04\x82\x01R`$\x01b\0\0\xE5V[_\x80_\x80_`\xA0\x86\x88\x03\x12\x15b\0\x05\xE9W_\x80\xFD[\x85Q` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x8A\x01Q\x93\x98P\x91\x96P\x94P\x92P`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x06\x1EW_\x80\xFD[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\x06UW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x06tWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\0\xBAW\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15b\0\x06\xA1WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\x01\x86W_\x81U`\x01\x01b\0\x06\xADV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x06\xDEWb\0\x06\xDEb\0\x06,V[b\0\x06\xF6\x81b\0\x06\xEF\x84Tb\0\x06@V[\x84b\0\x06zV[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\x07,W_\x84\x15b\0\x07\x14WP\x85\x83\x01Q[_\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\x01\"V[_\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\x07\\W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\x07;V[P\x85\x82\x10\x15b\0\x07zW\x87\x85\x01Q_\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[\x80\x82\x01\x80\x82\x11\x15b\0\x07\xAAWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x92\x91PPV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15b\0\x07\xE1W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01b\0\x07\xC3V[P\x94\x95\x94PPPPPV[`@\x81R_b\0\x08\0`@\x83\x01\x85b\0\x07\xB0V[\x82\x81\x03` \x84\x01Rb\0\x08\x14\x81\x85b\0\x07\xB0V[\x95\x94PPPPPV[_\x81Q\x80\x84R_[\x81\x81\x10\x15b\0\x08CW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01b\0\x08%V[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x16` \x82\x01R`@\x81\x01\x84\x90R``\x81\x01\x83\x90R`\xA0`\x80\x82\x01\x81\x90R_\x90b\0\x08\x9D\x90\x83\x01\x84b\0\x08\x1DV[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15b\0\x08\xB9W_\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14b\0\x08\xD1W_\x80\xFD[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x16` \x82\x01R`\xA0`@\x82\x01\x81\x90R_\x90b\0\t\x05\x90\x83\x01\x86b\0\x07\xB0V[\x82\x81\x03``\x84\x01Rb\0\t\x19\x81\x86b\0\x07\xB0V[\x90P\x82\x81\x03`\x80\x84\x01Rb\0\t/\x81\x85b\0\x08\x1DV[\x98\x97PPPPPPPPV[a\x11\x16\x80b\0\tI_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\x8FW_5`\xE0\x1C\x80c.\xB2\xC2\xD6\x11a\0cW\x80c.\xB2\xC2\xD6\x14a\x01\x11W\x80cN\x12s\xF4\x14a\x01$W\x80c\xA2,\xB4e\x14a\x01DW\x80c\xE9\x85\xE9\xC5\x14a\x01WW\x80c\xF2BC*\x14a\x01jW_\x80\xFD[\x80b\xFD\xD5\x8E\x14a\0\x93W\x80c\x01\xFF\xC9\xA7\x14a\0\xB9W\x80c\x0E\x894\x1C\x14a\0\xDCW\x80c\x15n)\xF6\x14a\0\xFCW[_\x80\xFD[a\0\xA6a\0\xA16`\x04a\n\xDBV[a\x01}V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCCa\0\xC76`\x04a\x0B\x1BV[a\x01\xA4V[`@Q\x90\x15\x15\x81R` \x01a\0\xB0V[a\0\xEFa\0\xEA6`\x04a\x0B=V[a\x01\xF3V[`@Qa\0\xB0\x91\x90a\x0B\x97V[a\x01\x0Fa\x01\n6`\x04a\x0B\xA9V[a\x02\x85V[\0[a\x01\x0Fa\x01\x1F6`\x04a\r\x19V[a\x02\xA4V[a\x017a\x0126`\x04a\r\xBCV[a\x03\x10V[`@Qa\0\xB0\x91\x90a\x0E\xB0V[a\x01\x0Fa\x01R6`\x04a\x0E\xC2V[a\x03\xDBV[a\0\xCCa\x01e6`\x04a\x0E\xFBV[a\x03\xEAV[a\x01\x0Fa\x01x6`\x04a\x0F,V[a\x04\x17V[_\x81\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 T[\x92\x91PPV[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cl\xDB=\x13`\xE1\x1B\x14\x80a\x01\xD4WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x03\xA2M\x07`\xE2\x1B\x14[\x80a\x01\x9EWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x01\x9EV[```\x02\x80Ta\x02\x02\x90a\x0F\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02.\x90a\x0F\x8CV[\x80\x15a\x02yW\x80`\x1F\x10a\x02PWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02yV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\\W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[a\x02\x9F\x83\x83\x83`@Q\x80` \x01`@R\x80_\x81RPa\x04vV[PPPV[3`\x01`\x01`\xA0\x1B\x03\x86\x16\x81\x14\x80\x15\x90a\x02\xC5WPa\x02\xC3\x86\x82a\x03\xEAV[\x15[\x15a\x02\xFBW`@Qcq\x1B\xEC\x91`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x87\x16`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\x08\x86\x86\x86\x86\x86a\x04\xD1V[PPPPPPV[``\x81Q\x83Q\x14a\x03AW\x81Q\x83Q`@Qc[\x05\x99\x91`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x02\xF2V[_\x83Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\\Wa\x03\\a\x0B\xD9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\x85W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84Q\x81\x10\x15a\x03\xD3W` \x80\x82\x02\x86\x01\x01Qa\x03\xAE\x90` \x80\x84\x02\x87\x01\x01Qa\x01}V[\x82\x82\x81Q\x81\x10a\x03\xC0Wa\x03\xC0a\x0F\xC4V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x03\x8AV[P\x93\x92PPPV[a\x03\xE63\x83\x83a\x056V[PPV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[3`\x01`\x01`\xA0\x1B\x03\x86\x16\x81\x14\x80\x15\x90a\x048WPa\x046\x86\x82a\x03\xEAV[\x15[\x15a\x04iW`@Qcq\x1B\xEC\x91`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x87\x16`$\x82\x01R`D\x01a\x02\xF2V[a\x03\x08\x86\x86\x86\x86\x86a\x05\xCAV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x04\x9FW`@Qc+\xFA#\xE7`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x02\xF2V[`@\x80Q`\x01\x80\x82R` \x82\x01\x86\x90R\x81\x83\x01\x90\x81R``\x82\x01\x85\x90R`\x80\x82\x01\x90\x92R\x90a\x03\x08_\x87\x84\x84\x87a\x06VV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x04\xFAW`@Qc+\xFA#\xE7`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x02\xF2V[`\x01`\x01`\xA0\x1B\x03\x85\x16a\x05\"W`@Qbj\rE`\xE2\x1B\x81R_`\x04\x82\x01R`$\x01a\x02\xF2V[a\x05/\x85\x85\x85\x85\x85a\x06VV[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x05^W`@Qb\xCE\xD3\xE1`\xE8\x1B\x81R_`\x04\x82\x01R`$\x01a\x02\xF2V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x05\xF3W`@Qc+\xFA#\xE7`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x02\xF2V[`\x01`\x01`\xA0\x1B\x03\x85\x16a\x06\x1BW`@Qbj\rE`\xE2\x1B\x81R_`\x04\x82\x01R`$\x01a\x02\xF2V[`@\x80Q`\x01\x80\x82R` \x82\x01\x86\x90R\x81\x83\x01\x90\x81R``\x82\x01\x85\x90R`\x80\x82\x01\x90\x92R\x90a\x06M\x87\x87\x84\x84\x87a\x06VV[PPPPPPPV[a\x06b\x85\x85\x85\x85a\x06\xA9V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x05/W\x82Q3\x90`\x01\x03a\x06\x9BW` \x84\x81\x01Q\x90\x84\x01Qa\x06\x94\x83\x89\x89\x85\x85\x89a\x08\xB8V[PPa\x03\x08V[a\x03\x08\x81\x87\x87\x87\x87\x87a\t\xD9V[\x80Q\x82Q\x14a\x06\xD8W\x81Q\x81Q`@Qc[\x05\x99\x91`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x02\xF2V[3_[\x83Q\x81\x10\x15a\x07\xDAW` \x81\x81\x02\x85\x81\x01\x82\x01Q\x90\x85\x01\x90\x91\x01Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x15a\x07\x8CW_\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x8C\x16\x84R\x90\x91R\x90 T\x81\x81\x10\x15a\x07fW`@Qc\x03\xDE\xE4\xC5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x81\x01\x84\x90R`\x84\x01a\x02\xF2V[_\x83\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x8D\x16\x84R\x90\x91R\x90 \x90\x82\x90\x03\x90U[`\x01`\x01`\xA0\x1B\x03\x87\x16\x15a\x07\xD0W_\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x8B\x16\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x07\xCA\x90\x84\x90a\x0F\xD8V[\x90\x91UPP[PP`\x01\x01a\x06\xDBV[P\x82Q`\x01\x03a\x08ZW` \x83\x01Q_\x90` \x84\x01Q\x90\x91P\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb\x85\x85`@Qa\x08K\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPa\x05/V[\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x7FJ9\xDC\x06\xD4\xC0\xDB\xC6Kp\xAF\x90\xFDi\x8A#:Q\x8A\xA5\xD0~Y]\x98;\x8C\x05&\xC8\xF7\xFB\x86\x86`@Qa\x08\xA9\x92\x91\x90a\x0F\xF7V[`@Q\x80\x91\x03\x90\xA4PPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15a\x03\x08W`@Qc\xF2:na`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xF2:na\x90a\x08\xFC\x90\x89\x90\x89\x90\x88\x90\x88\x90\x88\x90`\x04\x01a\x10$V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\t6WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\t3\x91\x81\x01\x90a\x10hV[`\x01[a\t\x9DW=\x80\x80\x15a\tcW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\thV[``\x91P[P\x80Q_\x03a\t\x95W`@Qc+\xFA#\xE7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x04\x82\x01R`$\x01a\x02\xF2V[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16c\xF2:na`\xE0\x1B\x14a\x06MW`@Qc+\xFA#\xE7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x04\x82\x01R`$\x01a\x02\xF2V[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15a\x03\x08W`@Qc\xBC\x19|\x81`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xBC\x19|\x81\x90a\n\x1D\x90\x89\x90\x89\x90\x88\x90\x88\x90\x88\x90`\x04\x01a\x10\x83V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\nWWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\nT\x91\x81\x01\x90a\x10hV[`\x01[a\n\x84W=\x80\x80\x15a\tcW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\thV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16c\xBC\x19|\x81`\xE0\x1B\x14a\x06MW`@Qc+\xFA#\xE7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x04\x82\x01R`$\x01a\x02\xF2V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xD6W_\x80\xFD[\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a\n\xECW_\x80\xFD[a\n\xF5\x83a\n\xC0V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0B\x18W_\x80\xFD[PV[_` \x82\x84\x03\x12\x15a\x0B+W_\x80\xFD[\x815a\x0B6\x81a\x0B\x03V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x0BMW_\x80\xFD[P5\x91\x90PV[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x0BxW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0B\\V[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x0B6` \x83\x01\x84a\x0BTV[_\x80_``\x84\x86\x03\x12\x15a\x0B\xBBW_\x80\xFD[a\x0B\xC4\x84a\n\xC0V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0C\x16Wa\x0C\x16a\x0B\xD9V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C7Wa\x0C7a\x0B\xD9V[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x0CPW_\x80\xFD[\x815` a\x0Cea\x0C`\x83a\x0C\x1EV[a\x0B\xEDV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x0C\x86W_\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x0C\xA2W\x805\x83R\x91\x83\x01\x91\x83\x01a\x0C\x8BV[P\x96\x95PPPPPPV[_\x82`\x1F\x83\x01\x12a\x0C\xBCW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xD6Wa\x0C\xD6a\x0B\xD9V[a\x0C\xE9`\x1F\x82\x01`\x1F\x19\x16` \x01a\x0B\xEDV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0C\xFDW_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x80_\x80_`\xA0\x86\x88\x03\x12\x15a\r-W_\x80\xFD[a\r6\x86a\n\xC0V[\x94Pa\rD` \x87\x01a\n\xC0V[\x93P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r`W_\x80\xFD[a\rl\x89\x83\x8A\x01a\x0CAV[\x94P``\x88\x015\x91P\x80\x82\x11\x15a\r\x81W_\x80\xFD[a\r\x8D\x89\x83\x8A\x01a\x0CAV[\x93P`\x80\x88\x015\x91P\x80\x82\x11\x15a\r\xA2W_\x80\xFD[Pa\r\xAF\x88\x82\x89\x01a\x0C\xADV[\x91PP\x92\x95P\x92\x95\x90\x93PV[_\x80`@\x83\x85\x03\x12\x15a\r\xCDW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\xE4W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\r\xF7W_\x80\xFD[\x815` a\x0E\x07a\x0C`\x83a\x0C\x1EV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x0E%W_\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x0EJWa\x0E;\x86a\n\xC0V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x0E*V[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\x0E_W_\x80\xFD[Pa\x0El\x85\x82\x86\x01a\x0CAV[\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a\x0E\xA5W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x0E\x89V[P\x94\x95\x94PPPPPV[` \x81R_a\x0B6` \x83\x01\x84a\x0EvV[_\x80`@\x83\x85\x03\x12\x15a\x0E\xD3W_\x80\xFD[a\x0E\xDC\x83a\n\xC0V[\x91P` \x83\x015\x80\x15\x15\x81\x14a\x0E\xF0W_\x80\xFD[\x80\x91PP\x92P\x92\x90PV[_\x80`@\x83\x85\x03\x12\x15a\x0F\x0CW_\x80\xFD[a\x0F\x15\x83a\n\xC0V[\x91Pa\x0F#` \x84\x01a\n\xC0V[\x90P\x92P\x92\x90PV[_\x80_\x80_`\xA0\x86\x88\x03\x12\x15a\x0F@W_\x80\xFD[a\x0FI\x86a\n\xC0V[\x94Pa\x0FW` \x87\x01a\n\xC0V[\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x80W_\x80\xFD[a\r\xAF\x88\x82\x89\x01a\x0C\xADV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0F\xA0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0F\xBEWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x01\x9EWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`@\x81R_a\x10\t`@\x83\x01\x85a\x0EvV[\x82\x81\x03` \x84\x01Ra\x10\x1B\x81\x85a\x0EvV[\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x16` \x82\x01R`@\x81\x01\x84\x90R``\x81\x01\x83\x90R`\xA0`\x80\x82\x01\x81\x90R_\x90a\x10]\x90\x83\x01\x84a\x0BTV[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a\x10xW_\x80\xFD[\x81Qa\x0B6\x81a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x16` \x82\x01R`\xA0`@\x82\x01\x81\x90R_\x90a\x10\xAE\x90\x83\x01\x86a\x0EvV[\x82\x81\x03``\x84\x01Ra\x10\xC0\x81\x86a\x0EvV[\x90P\x82\x81\x03`\x80\x84\x01Ra\x10\xD4\x81\x85a\x0BTV[\x98\x97PPPPPPPPV\xFE\xA2dipfsX\"\x12 e\x08\xB8frn@\x92Q\xFF8zO\x92\xE6\x889G\x85-R\x0E\xE89\xC4\x85\xC06\x98\xFE\x9B\x12dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static ERC1155MOCK_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\x8FW_5`\xE0\x1C\x80c.\xB2\xC2\xD6\x11a\0cW\x80c.\xB2\xC2\xD6\x14a\x01\x11W\x80cN\x12s\xF4\x14a\x01$W\x80c\xA2,\xB4e\x14a\x01DW\x80c\xE9\x85\xE9\xC5\x14a\x01WW\x80c\xF2BC*\x14a\x01jW_\x80\xFD[\x80b\xFD\xD5\x8E\x14a\0\x93W\x80c\x01\xFF\xC9\xA7\x14a\0\xB9W\x80c\x0E\x894\x1C\x14a\0\xDCW\x80c\x15n)\xF6\x14a\0\xFCW[_\x80\xFD[a\0\xA6a\0\xA16`\x04a\n\xDBV[a\x01}V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xCCa\0\xC76`\x04a\x0B\x1BV[a\x01\xA4V[`@Q\x90\x15\x15\x81R` \x01a\0\xB0V[a\0\xEFa\0\xEA6`\x04a\x0B=V[a\x01\xF3V[`@Qa\0\xB0\x91\x90a\x0B\x97V[a\x01\x0Fa\x01\n6`\x04a\x0B\xA9V[a\x02\x85V[\0[a\x01\x0Fa\x01\x1F6`\x04a\r\x19V[a\x02\xA4V[a\x017a\x0126`\x04a\r\xBCV[a\x03\x10V[`@Qa\0\xB0\x91\x90a\x0E\xB0V[a\x01\x0Fa\x01R6`\x04a\x0E\xC2V[a\x03\xDBV[a\0\xCCa\x01e6`\x04a\x0E\xFBV[a\x03\xEAV[a\x01\x0Fa\x01x6`\x04a\x0F,V[a\x04\x17V[_\x81\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x86\x16\x84R\x90\x91R\x90 T[\x92\x91PPV[_`\x01`\x01`\xE0\x1B\x03\x19\x82\x16cl\xDB=\x13`\xE1\x1B\x14\x80a\x01\xD4WP`\x01`\x01`\xE0\x1B\x03\x19\x82\x16c\x03\xA2M\x07`\xE2\x1B\x14[\x80a\x01\x9EWPc\x01\xFF\xC9\xA7`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x14a\x01\x9EV[```\x02\x80Ta\x02\x02\x90a\x0F\x8CV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02.\x90a\x0F\x8CV[\x80\x15a\x02yW\x80`\x1F\x10a\x02PWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02yV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\\W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[a\x02\x9F\x83\x83\x83`@Q\x80` \x01`@R\x80_\x81RPa\x04vV[PPPV[3`\x01`\x01`\xA0\x1B\x03\x86\x16\x81\x14\x80\x15\x90a\x02\xC5WPa\x02\xC3\x86\x82a\x03\xEAV[\x15[\x15a\x02\xFBW`@Qcq\x1B\xEC\x91`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x87\x16`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[a\x03\x08\x86\x86\x86\x86\x86a\x04\xD1V[PPPPPPV[``\x81Q\x83Q\x14a\x03AW\x81Q\x83Q`@Qc[\x05\x99\x91`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x02\xF2V[_\x83Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\\Wa\x03\\a\x0B\xD9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03\x85W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84Q\x81\x10\x15a\x03\xD3W` \x80\x82\x02\x86\x01\x01Qa\x03\xAE\x90` \x80\x84\x02\x87\x01\x01Qa\x01}V[\x82\x82\x81Q\x81\x10a\x03\xC0Wa\x03\xC0a\x0F\xC4V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x03\x8AV[P\x93\x92PPPV[a\x03\xE63\x83\x83a\x056V[PPV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R T`\xFF\x16\x90V[3`\x01`\x01`\xA0\x1B\x03\x86\x16\x81\x14\x80\x15\x90a\x048WPa\x046\x86\x82a\x03\xEAV[\x15[\x15a\x04iW`@Qcq\x1B\xEC\x91`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x87\x16`$\x82\x01R`D\x01a\x02\xF2V[a\x03\x08\x86\x86\x86\x86\x86a\x05\xCAV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x04\x9FW`@Qc+\xFA#\xE7`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x02\xF2V[`@\x80Q`\x01\x80\x82R` \x82\x01\x86\x90R\x81\x83\x01\x90\x81R``\x82\x01\x85\x90R`\x80\x82\x01\x90\x92R\x90a\x03\x08_\x87\x84\x84\x87a\x06VV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x04\xFAW`@Qc+\xFA#\xE7`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x02\xF2V[`\x01`\x01`\xA0\x1B\x03\x85\x16a\x05\"W`@Qbj\rE`\xE2\x1B\x81R_`\x04\x82\x01R`$\x01a\x02\xF2V[a\x05/\x85\x85\x85\x85\x85a\x06VV[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x05^W`@Qb\xCE\xD3\xE1`\xE8\x1B\x81R_`\x04\x82\x01R`$\x01a\x02\xF2V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x94\x87\x16\x80\x84R\x94\x82R\x91\x82\x90 \x80T`\xFF\x19\x16\x86\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16a\x05\xF3W`@Qc+\xFA#\xE7`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01a\x02\xF2V[`\x01`\x01`\xA0\x1B\x03\x85\x16a\x06\x1BW`@Qbj\rE`\xE2\x1B\x81R_`\x04\x82\x01R`$\x01a\x02\xF2V[`@\x80Q`\x01\x80\x82R` \x82\x01\x86\x90R\x81\x83\x01\x90\x81R``\x82\x01\x85\x90R`\x80\x82\x01\x90\x92R\x90a\x06M\x87\x87\x84\x84\x87a\x06VV[PPPPPPPV[a\x06b\x85\x85\x85\x85a\x06\xA9V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x15a\x05/W\x82Q3\x90`\x01\x03a\x06\x9BW` \x84\x81\x01Q\x90\x84\x01Qa\x06\x94\x83\x89\x89\x85\x85\x89a\x08\xB8V[PPa\x03\x08V[a\x03\x08\x81\x87\x87\x87\x87\x87a\t\xD9V[\x80Q\x82Q\x14a\x06\xD8W\x81Q\x81Q`@Qc[\x05\x99\x91`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x02\xF2V[3_[\x83Q\x81\x10\x15a\x07\xDAW` \x81\x81\x02\x85\x81\x01\x82\x01Q\x90\x85\x01\x90\x91\x01Q`\x01`\x01`\xA0\x1B\x03\x88\x16\x15a\x07\x8CW_\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x8C\x16\x84R\x90\x91R\x90 T\x81\x81\x10\x15a\x07fW`@Qc\x03\xDE\xE4\xC5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\x04\x82\x01R`$\x81\x01\x82\x90R`D\x81\x01\x83\x90R`d\x81\x01\x84\x90R`\x84\x01a\x02\xF2V[_\x83\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x8D\x16\x84R\x90\x91R\x90 \x90\x82\x90\x03\x90U[`\x01`\x01`\xA0\x1B\x03\x87\x16\x15a\x07\xD0W_\x82\x81R` \x81\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x8B\x16\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x07\xCA\x90\x84\x90a\x0F\xD8V[\x90\x91UPP[PP`\x01\x01a\x06\xDBV[P\x82Q`\x01\x03a\x08ZW` \x83\x01Q_\x90` \x84\x01Q\x90\x91P\x85`\x01`\x01`\xA0\x1B\x03\x16\x87`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb\x85\x85`@Qa\x08K\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPa\x05/V[\x83`\x01`\x01`\xA0\x1B\x03\x16\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x7FJ9\xDC\x06\xD4\xC0\xDB\xC6Kp\xAF\x90\xFDi\x8A#:Q\x8A\xA5\xD0~Y]\x98;\x8C\x05&\xC8\xF7\xFB\x86\x86`@Qa\x08\xA9\x92\x91\x90a\x0F\xF7V[`@Q\x80\x91\x03\x90\xA4PPPPPV[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15a\x03\x08W`@Qc\xF2:na`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xF2:na\x90a\x08\xFC\x90\x89\x90\x89\x90\x88\x90\x88\x90\x88\x90`\x04\x01a\x10$V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\t6WP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\t3\x91\x81\x01\x90a\x10hV[`\x01[a\t\x9DW=\x80\x80\x15a\tcW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\thV[``\x91P[P\x80Q_\x03a\t\x95W`@Qc+\xFA#\xE7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x04\x82\x01R`$\x01a\x02\xF2V[\x80Q\x81` \x01\xFD[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16c\xF2:na`\xE0\x1B\x14a\x06MW`@Qc+\xFA#\xE7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x04\x82\x01R`$\x01a\x02\xF2V[`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15a\x03\x08W`@Qc\xBC\x19|\x81`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\xBC\x19|\x81\x90a\n\x1D\x90\x89\x90\x89\x90\x88\x90\x88\x90\x88\x90`\x04\x01a\x10\x83V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\nWWP`@\x80Q`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01\x90\x92Ra\nT\x91\x81\x01\x90a\x10hV[`\x01[a\n\x84W=\x80\x80\x15a\tcW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\thV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16c\xBC\x19|\x81`\xE0\x1B\x14a\x06MW`@Qc+\xFA#\xE7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16`\x04\x82\x01R`$\x01a\x02\xF2V[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\xD6W_\x80\xFD[\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a\n\xECW_\x80\xFD[a\n\xF5\x83a\n\xC0V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x0B\x18W_\x80\xFD[PV[_` \x82\x84\x03\x12\x15a\x0B+W_\x80\xFD[\x815a\x0B6\x81a\x0B\x03V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x0BMW_\x80\xFD[P5\x91\x90PV[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x0BxW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0B\\V[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x0B6` \x83\x01\x84a\x0BTV[_\x80_``\x84\x86\x03\x12\x15a\x0B\xBBW_\x80\xFD[a\x0B\xC4\x84a\n\xC0V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0C\x16Wa\x0C\x16a\x0B\xD9V[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C7Wa\x0C7a\x0B\xD9V[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x0CPW_\x80\xFD[\x815` a\x0Cea\x0C`\x83a\x0C\x1EV[a\x0B\xEDV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x0C\x86W_\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x0C\xA2W\x805\x83R\x91\x83\x01\x91\x83\x01a\x0C\x8BV[P\x96\x95PPPPPPV[_\x82`\x1F\x83\x01\x12a\x0C\xBCW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xD6Wa\x0C\xD6a\x0B\xD9V[a\x0C\xE9`\x1F\x82\x01`\x1F\x19\x16` \x01a\x0B\xEDV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x0C\xFDW_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x80_\x80_`\xA0\x86\x88\x03\x12\x15a\r-W_\x80\xFD[a\r6\x86a\n\xC0V[\x94Pa\rD` \x87\x01a\n\xC0V[\x93P`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r`W_\x80\xFD[a\rl\x89\x83\x8A\x01a\x0CAV[\x94P``\x88\x015\x91P\x80\x82\x11\x15a\r\x81W_\x80\xFD[a\r\x8D\x89\x83\x8A\x01a\x0CAV[\x93P`\x80\x88\x015\x91P\x80\x82\x11\x15a\r\xA2W_\x80\xFD[Pa\r\xAF\x88\x82\x89\x01a\x0C\xADV[\x91PP\x92\x95P\x92\x95\x90\x93PV[_\x80`@\x83\x85\x03\x12\x15a\r\xCDW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\xE4W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\r\xF7W_\x80\xFD[\x815` a\x0E\x07a\x0C`\x83a\x0C\x1EV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a\x0E%W_\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x0EJWa\x0E;\x86a\n\xC0V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x0E*V[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a\x0E_W_\x80\xFD[Pa\x0El\x85\x82\x86\x01a\x0CAV[\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01_[\x83\x81\x10\x15a\x0E\xA5W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x0E\x89V[P\x94\x95\x94PPPPPV[` \x81R_a\x0B6` \x83\x01\x84a\x0EvV[_\x80`@\x83\x85\x03\x12\x15a\x0E\xD3W_\x80\xFD[a\x0E\xDC\x83a\n\xC0V[\x91P` \x83\x015\x80\x15\x15\x81\x14a\x0E\xF0W_\x80\xFD[\x80\x91PP\x92P\x92\x90PV[_\x80`@\x83\x85\x03\x12\x15a\x0F\x0CW_\x80\xFD[a\x0F\x15\x83a\n\xC0V[\x91Pa\x0F#` \x84\x01a\n\xC0V[\x90P\x92P\x92\x90PV[_\x80_\x80_`\xA0\x86\x88\x03\x12\x15a\x0F@W_\x80\xFD[a\x0FI\x86a\n\xC0V[\x94Pa\x0FW` \x87\x01a\n\xC0V[\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x80W_\x80\xFD[a\r\xAF\x88\x82\x89\x01a\x0C\xADV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0F\xA0W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0F\xBEWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x01\x9EWcNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`@\x81R_a\x10\t`@\x83\x01\x85a\x0EvV[\x82\x81\x03` \x84\x01Ra\x10\x1B\x81\x85a\x0EvV[\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x16` \x82\x01R`@\x81\x01\x84\x90R``\x81\x01\x83\x90R`\xA0`\x80\x82\x01\x81\x90R_\x90a\x10]\x90\x83\x01\x84a\x0BTV[\x97\x96PPPPPPPV[_` \x82\x84\x03\x12\x15a\x10xW_\x80\xFD[\x81Qa\x0B6\x81a\x0B\x03V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x16` \x82\x01R`\xA0`@\x82\x01\x81\x90R_\x90a\x10\xAE\x90\x83\x01\x86a\x0EvV[\x82\x81\x03``\x84\x01Ra\x10\xC0\x81\x86a\x0EvV[\x90P\x82\x81\x03`\x80\x84\x01Ra\x10\xD4\x81\x85a\x0BTV[\x98\x97PPPPPPPPV\xFE\xA2dipfsX\"\x12 e\x08\xB8frn@\x92Q\xFF8zO\x92\xE6\x889G\x85-R\x0E\xE89\xC4\x85\xC06\x98\xFE\x9B\x12dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static ERC1155MOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct ERC1155Mock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ERC1155Mock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ERC1155Mock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ERC1155Mock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ERC1155Mock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ERC1155Mock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ERC1155Mock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ERC1155MOCK_ABI.clone(),
                client,
            ))
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
                ERC1155MOCK_ABI.clone(),
                ERC1155MOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `balanceOf` (0x00fdd58e) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([0, 253, 213, 142], (account, id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOfBatch` (0x4e1273f4) function
        pub fn balance_of_batch(
            &self,
            accounts: ::std::vec::Vec<::ethers::core::types::Address>,
            ids: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([78, 18, 115, 244], (accounts, ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            account: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (account, operator))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mint` (0x156e29f6) function
        pub fn mint(
            &self,
            owner: ::ethers::core::types::Address,
            token_id: ::ethers::core::types::U256,
            value: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 110, 41, 246], (owner, token_id, value))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeBatchTransferFrom` (0x2eb2c2d6) function
        pub fn safe_batch_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            ids: ::std::vec::Vec<::ethers::core::types::U256>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 178, 194, 214], (from, to, ids, values, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0xf242432a) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 66, 67, 42], (from, to, id, value, data))
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
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uri` (0x0e89341c) function
        pub fn uri(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([14, 137, 52, 28], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalForAllFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `TransferBatch` event
        pub fn transfer_batch_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferBatchFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `TransferSingle` event
        pub fn transfer_single_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferSingleFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `URI` event
        pub fn uri_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UriFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ERC1155MockEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for ERC1155Mock<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ERC1155InsufficientBalance` with signature `ERC1155InsufficientBalance(address,uint256,uint256,uint256)` and selector `0x03dee4c5`
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
        Hash,
    )]
    #[etherror(
        name = "ERC1155InsufficientBalance",
        abi = "ERC1155InsufficientBalance(address,uint256,uint256,uint256)"
    )]
    pub struct ERC1155InsufficientBalance {
        pub sender: ::ethers::core::types::Address,
        pub balance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC1155InvalidApprover` with signature `ERC1155InvalidApprover(address)` and selector `0x3e31884e`
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
        Hash,
    )]
    #[etherror(
        name = "ERC1155InvalidApprover",
        abi = "ERC1155InvalidApprover(address)"
    )]
    pub struct ERC1155InvalidApprover {
        pub approver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1155InvalidArrayLength` with signature `ERC1155InvalidArrayLength(uint256,uint256)` and selector `0x5b059991`
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
        Hash,
    )]
    #[etherror(
        name = "ERC1155InvalidArrayLength",
        abi = "ERC1155InvalidArrayLength(uint256,uint256)"
    )]
    pub struct ERC1155InvalidArrayLength {
        pub ids_length: ::ethers::core::types::U256,
        pub values_length: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC1155InvalidOperator` with signature `ERC1155InvalidOperator(address)` and selector `0xced3e100`
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
        Hash,
    )]
    #[etherror(
        name = "ERC1155InvalidOperator",
        abi = "ERC1155InvalidOperator(address)"
    )]
    pub struct ERC1155InvalidOperator {
        pub operator: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1155InvalidReceiver` with signature `ERC1155InvalidReceiver(address)` and selector `0x57f447ce`
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
        Hash,
    )]
    #[etherror(
        name = "ERC1155InvalidReceiver",
        abi = "ERC1155InvalidReceiver(address)"
    )]
    pub struct ERC1155InvalidReceiver {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1155InvalidSender` with signature `ERC1155InvalidSender(address)` and selector `0x01a83514`
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
        Hash,
    )]
    #[etherror(name = "ERC1155InvalidSender", abi = "ERC1155InvalidSender(address)")]
    pub struct ERC1155InvalidSender {
        pub sender: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1155MissingApprovalForAll` with signature `ERC1155MissingApprovalForAll(address,address)` and selector `0xe237d922`
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
        Hash,
    )]
    #[etherror(
        name = "ERC1155MissingApprovalForAll",
        abi = "ERC1155MissingApprovalForAll(address,address)"
    )]
    pub struct ERC1155MissingApprovalForAll {
        pub operator: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
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
        Hash,
    )]
    pub enum ERC1155MockErrors {
        ERC1155InsufficientBalance(ERC1155InsufficientBalance),
        ERC1155InvalidApprover(ERC1155InvalidApprover),
        ERC1155InvalidArrayLength(ERC1155InvalidArrayLength),
        ERC1155InvalidOperator(ERC1155InvalidOperator),
        ERC1155InvalidReceiver(ERC1155InvalidReceiver),
        ERC1155InvalidSender(ERC1155InvalidSender),
        ERC1155MissingApprovalForAll(ERC1155MissingApprovalForAll),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ERC1155MockErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InsufficientBalance(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InvalidApprover as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InvalidApprover(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InvalidArrayLength as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InvalidArrayLength(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InvalidOperator as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InvalidOperator(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InvalidReceiver as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InvalidReceiver(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InvalidSender as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InvalidSender(decoded));
            }
            if let Ok(decoded) =
                <ERC1155MissingApprovalForAll as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155MissingApprovalForAll(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC1155MockErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ERC1155InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InvalidApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InvalidArrayLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InvalidOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InvalidReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155MissingApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ERC1155MockErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ERC1155InsufficientBalance as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InvalidApprover as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InvalidArrayLength as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InvalidOperator as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InvalidReceiver as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InvalidSender as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155MissingApprovalForAll as ::ethers::contract::EthError>::selector(
                    ) =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ERC1155MockErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ERC1155InsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InvalidApprover(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InvalidArrayLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InvalidOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InvalidReceiver(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155MissingApprovalForAll(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ERC1155MockErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ERC1155InsufficientBalance> for ERC1155MockErrors {
        fn from(value: ERC1155InsufficientBalance) -> Self {
            Self::ERC1155InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<ERC1155InvalidApprover> for ERC1155MockErrors {
        fn from(value: ERC1155InvalidApprover) -> Self {
            Self::ERC1155InvalidApprover(value)
        }
    }
    impl ::core::convert::From<ERC1155InvalidArrayLength> for ERC1155MockErrors {
        fn from(value: ERC1155InvalidArrayLength) -> Self {
            Self::ERC1155InvalidArrayLength(value)
        }
    }
    impl ::core::convert::From<ERC1155InvalidOperator> for ERC1155MockErrors {
        fn from(value: ERC1155InvalidOperator) -> Self {
            Self::ERC1155InvalidOperator(value)
        }
    }
    impl ::core::convert::From<ERC1155InvalidReceiver> for ERC1155MockErrors {
        fn from(value: ERC1155InvalidReceiver) -> Self {
            Self::ERC1155InvalidReceiver(value)
        }
    }
    impl ::core::convert::From<ERC1155InvalidSender> for ERC1155MockErrors {
        fn from(value: ERC1155InvalidSender) -> Self {
            Self::ERC1155InvalidSender(value)
        }
    }
    impl ::core::convert::From<ERC1155MissingApprovalForAll> for ERC1155MockErrors {
        fn from(value: ERC1155MissingApprovalForAll) -> Self {
            Self::ERC1155MissingApprovalForAll(value)
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
        Hash,
    )]
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
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
        Hash,
    )]
    #[ethevent(
        name = "TransferBatch",
        abi = "TransferBatch(address,address,address,uint256[],uint256[])"
    )]
    pub struct TransferBatchFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
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
        Hash,
    )]
    #[ethevent(
        name = "TransferSingle",
        abi = "TransferSingle(address,address,address,uint256,uint256)"
    )]
    pub struct TransferSingleFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
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
        Hash,
    )]
    #[ethevent(name = "URI", abi = "URI(string,uint256)")]
    pub struct UriFilter {
        pub value: ::std::string::String,
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
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
        Hash,
    )]
    pub enum ERC1155MockEvents {
        ApprovalForAllFilter(ApprovalForAllFilter),
        TransferBatchFilter(TransferBatchFilter),
        TransferSingleFilter(TransferSingleFilter),
        UriFilter(UriFilter),
    }
    impl ::ethers::contract::EthLogDecode for ERC1155MockEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(ERC1155MockEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = TransferBatchFilter::decode_log(log) {
                return Ok(ERC1155MockEvents::TransferBatchFilter(decoded));
            }
            if let Ok(decoded) = TransferSingleFilter::decode_log(log) {
                return Ok(ERC1155MockEvents::TransferSingleFilter(decoded));
            }
            if let Ok(decoded) = UriFilter::decode_log(log) {
                return Ok(ERC1155MockEvents::UriFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ERC1155MockEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalForAllFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferBatchFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferSingleFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UriFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for ERC1155MockEvents {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<TransferBatchFilter> for ERC1155MockEvents {
        fn from(value: TransferBatchFilter) -> Self {
            Self::TransferBatchFilter(value)
        }
    }
    impl ::core::convert::From<TransferSingleFilter> for ERC1155MockEvents {
        fn from(value: TransferSingleFilter) -> Self {
            Self::TransferSingleFilter(value)
        }
    }
    impl ::core::convert::From<UriFilter> for ERC1155MockEvents {
        fn from(value: UriFilter) -> Self {
            Self::UriFilter(value)
        }
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `0x00fdd58e`
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
        Hash,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address,uint256)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOfBatch` function with signature `balanceOfBatch(address[],uint256[])` and selector `0x4e1273f4`
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
        Hash,
    )]
    #[ethcall(name = "balanceOfBatch", abi = "balanceOfBatch(address[],uint256[])")]
    pub struct BalanceOfBatchCall {
        pub accounts: ::std::vec::Vec<::ethers::core::types::Address>,
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
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
        Hash,
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub account: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `mint` function with signature `mint(address,uint256,uint256)` and selector `0x156e29f6`
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
        Hash,
    )]
    #[ethcall(name = "mint", abi = "mint(address,uint256,uint256)")]
    pub struct MintCall {
        pub owner: ::ethers::core::types::Address,
        pub token_id: ::ethers::core::types::U256,
        pub value: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `safeBatchTransferFrom` function with signature `safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)` and selector `0x2eb2c2d6`
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
        Hash,
    )]
    #[ethcall(
        name = "safeBatchTransferFrom",
        abi = "safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct SafeBatchTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,uint256,bytes)` and selector `0xf242432a`
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
        Hash,
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,uint256,bytes)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
        pub value: ::ethers::core::types::U256,
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
        Hash,
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
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
        Hash,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `uri` function with signature `uri(uint256)` and selector `0x0e89341c`
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
        Hash,
    )]
    #[ethcall(name = "uri", abi = "uri(uint256)")]
    pub struct UriCall(pub ::ethers::core::types::U256);
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum ERC1155MockCalls {
        BalanceOf(BalanceOfCall),
        BalanceOfBatch(BalanceOfBatchCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Mint(MintCall),
        SafeBatchTransferFrom(SafeBatchTransferFromCall),
        SafeTransferFrom(SafeTransferFromCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Uri(UriCall),
    }
    impl ::ethers::core::abi::AbiDecode for ERC1155MockCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfBatchCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BalanceOfBatch(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) =
                <SafeBatchTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeBatchTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <UriCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Uri(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ERC1155MockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOfBatch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsApprovedForAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeBatchTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetApprovalForAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Uri(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ERC1155MockCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOfBatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeBatchTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Uri(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BalanceOfCall> for ERC1155MockCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BalanceOfBatchCall> for ERC1155MockCalls {
        fn from(value: BalanceOfBatchCall) -> Self {
            Self::BalanceOfBatch(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for ERC1155MockCalls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<MintCall> for ERC1155MockCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<SafeBatchTransferFromCall> for ERC1155MockCalls {
        fn from(value: SafeBatchTransferFromCall) -> Self {
            Self::SafeBatchTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for ERC1155MockCalls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for ERC1155MockCalls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for ERC1155MockCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<UriCall> for ERC1155MockCalls {
        fn from(value: UriCall) -> Self {
            Self::Uri(value)
        }
    }
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `0x00fdd58e`
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
        Hash,
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balanceOfBatch` function with signature `balanceOfBatch(address[],uint256[])` and selector `0x4e1273f4`
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
        Hash,
    )]
    pub struct BalanceOfBatchReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
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
        Hash,
    )]
    pub struct IsApprovedForAllReturn(pub bool);
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
        Hash,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `uri` function with signature `uri(uint256)` and selector `0x0e89341c`
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
        Hash,
    )]
    pub struct UriReturn(pub ::std::string::String);
}
