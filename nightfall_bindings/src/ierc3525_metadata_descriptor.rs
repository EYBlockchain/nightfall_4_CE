pub use ierc3525_metadata_descriptor::*;
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
pub mod ierc3525_metadata_descriptor {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("constructContractURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "constructContractURI",
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
                    ::std::borrow::ToOwned::to_owned("constructSlotURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("constructSlotURI"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("slot"),
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
                    ::std::borrow::ToOwned::to_owned("constructTokenURI"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("constructTokenURI"),
                            inputs: ::std::vec![
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static IERC3525METADATADESCRIPTOR_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    pub struct IERC3525MetadataDescriptor<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IERC3525MetadataDescriptor<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IERC3525MetadataDescriptor<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IERC3525MetadataDescriptor<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IERC3525MetadataDescriptor<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(IERC3525MetadataDescriptor))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IERC3525MetadataDescriptor<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IERC3525METADATADESCRIPTOR_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `constructContractURI` (0x725fa09c) function
        pub fn construct_contract_uri(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([114, 95, 160, 156], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `constructSlotURI` (0x6c037f8a) function
        pub fn construct_slot_uri(
            &self,
            slot: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([108, 3, 127, 138], slot)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `constructTokenURI` (0x894b4c2e) function
        pub fn construct_token_uri(
            &self,
            token_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([137, 75, 76, 46], token_id)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IERC3525MetadataDescriptor<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `constructContractURI` function with signature `constructContractURI()` and selector `0x725fa09c`
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
    #[ethcall(name = "constructContractURI", abi = "constructContractURI()")]
    pub struct ConstructContractURICall;
    ///Container type for all input parameters for the `constructSlotURI` function with signature `constructSlotURI(uint256)` and selector `0x6c037f8a`
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
    #[ethcall(name = "constructSlotURI", abi = "constructSlotURI(uint256)")]
    pub struct ConstructSlotURICall {
        pub slot: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `constructTokenURI` function with signature `constructTokenURI(uint256)` and selector `0x894b4c2e`
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
    #[ethcall(name = "constructTokenURI", abi = "constructTokenURI(uint256)")]
    pub struct ConstructTokenURICall {
        pub token_id: ::ethers::core::types::U256,
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
    pub enum IERC3525MetadataDescriptorCalls {
        ConstructContractURI(ConstructContractURICall),
        ConstructSlotURI(ConstructSlotURICall),
        ConstructTokenURI(ConstructTokenURICall),
    }
    impl ::ethers::core::abi::AbiDecode for IERC3525MetadataDescriptorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ConstructContractURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConstructContractURI(decoded));
            }
            if let Ok(decoded) = <ConstructSlotURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConstructSlotURI(decoded));
            }
            if let Ok(decoded) = <ConstructTokenURICall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ConstructTokenURI(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IERC3525MetadataDescriptorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ConstructContractURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConstructSlotURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConstructTokenURI(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IERC3525MetadataDescriptorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ConstructContractURI(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConstructSlotURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConstructTokenURI(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ConstructContractURICall>
    for IERC3525MetadataDescriptorCalls {
        fn from(value: ConstructContractURICall) -> Self {
            Self::ConstructContractURI(value)
        }
    }
    impl ::core::convert::From<ConstructSlotURICall>
    for IERC3525MetadataDescriptorCalls {
        fn from(value: ConstructSlotURICall) -> Self {
            Self::ConstructSlotURI(value)
        }
    }
    impl ::core::convert::From<ConstructTokenURICall>
    for IERC3525MetadataDescriptorCalls {
        fn from(value: ConstructTokenURICall) -> Self {
            Self::ConstructTokenURI(value)
        }
    }
    ///Container type for all return fields from the `constructContractURI` function with signature `constructContractURI()` and selector `0x725fa09c`
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
    pub struct ConstructContractURIReturn(pub ::std::string::String);
    ///Container type for all return fields from the `constructSlotURI` function with signature `constructSlotURI(uint256)` and selector `0x6c037f8a`
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
    pub struct ConstructSlotURIReturn(pub ::std::string::String);
    ///Container type for all return fields from the `constructTokenURI` function with signature `constructTokenURI(uint256)` and selector `0x894b4c2e`
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
    pub struct ConstructTokenURIReturn(pub ::std::string::String);
}
