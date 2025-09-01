pub use sanctions_list_mock::*;
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
pub mod sanctions_list_mock {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_sanctionedUser"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("isSanctioned"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isSanctioned"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("user"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SANCTIONSLISTMOCK_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0EW__\xFD[P`@Qa\x01W8\x03\x80a\x01W\x839\x81\x01`@\x81\x90R`+\x91`NV[_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U`yV[_` \x82\x84\x03\x12\x15`]W__\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`rW__\xFD[\x93\x92PPPV[`\xD2\x80a\0\x85_9_\xF3\xFE`\x80`@R4\x80\x15`\x0EW__\xFD[P`\x046\x10`&W_5`\xE0\x1C\x80c\xDFY/}\x14`*W[__\xFD[`9`56`\x04`qV[`MV[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[_\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x83\x16\x03`jWP`\x01\x91\x90PV[P_\x91\x90PV[_` \x82\x84\x03\x12\x15`\x80W__\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`\x95W__\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xE6u\xE8\xE4F<\xD3\xAB\xB39\x02\xBA5:\x0C\xC5\xD9H\xA7\x07\x05\xE8\xE6\xB2&vD\0q\xCF\x82}dsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static SANCTIONSLISTMOCK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0EW__\xFD[P`\x046\x10`&W_5`\xE0\x1C\x80c\xDFY/}\x14`*W[__\xFD[`9`56`\x04`qV[`MV[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[_\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x83\x16\x03`jWP`\x01\x91\x90PV[P_\x91\x90PV[_` \x82\x84\x03\x12\x15`\x80W__\xFD[\x815`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`\x95W__\xFD[\x93\x92PPPV\xFE\xA2dipfsX\"\x12 \xE6u\xE8\xE4F<\xD3\xAB\xB39\x02\xBA5:\x0C\xC5\xD9H\xA7\x07\x05\xE8\xE6\xB2&vD\0q\xCF\x82}dsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static SANCTIONSLISTMOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SanctionsListMock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SanctionsListMock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SanctionsListMock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SanctionsListMock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SanctionsListMock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(SanctionsListMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SanctionsListMock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SANCTIONSLISTMOCK_ABI.clone(),
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
                SANCTIONSLISTMOCK_ABI.clone(),
                SANCTIONSLISTMOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `isSanctioned` (0xdf592f7d) function
        pub fn is_sanctioned(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([223, 89, 47, 125], user)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SanctionsListMock<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `isSanctioned` function with signature `isSanctioned(address)` and selector `0xdf592f7d`
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
    #[ethcall(name = "isSanctioned", abi = "isSanctioned(address)")]
    pub struct IsSanctionedCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `isSanctioned` function with signature `isSanctioned(address)` and selector `0xdf592f7d`
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
    pub struct IsSanctionedReturn(pub bool);
}
