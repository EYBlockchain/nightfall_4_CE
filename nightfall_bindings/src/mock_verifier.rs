pub use mock_verifier::*;
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
pub mod mock_verifier {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("verify"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("verify"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
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
    pub static MOCKVERIFIER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R_\x80T`\xFF\x19\x16`\x01\x17\x90U4\x80\x15a\0\x1BW_\x80\xFD[Pa\x01L\x80a\0)_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80c\xEAP\xD0\xE4\x14a\0-W[_\x80\xFD[a\0Ha\0;6`\x04a\0\\V[_T`\xFF\x16\x94\x93PPPPV[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[_\x80_\x80`@\x85\x87\x03\x12\x15a\0oW_\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\0\x86W_\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\0\x99W_\x80\xFD[\x815\x81\x81\x11\x15a\0\xA7W_\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\0\xB8W_\x80\xFD[` \x92\x83\x01\x96P\x94P\x90\x86\x015\x90\x80\x82\x11\x15a\0\xD2W_\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\0\xE5W_\x80\xFD[\x815\x81\x81\x11\x15a\0\xF3W_\x80\xFD[\x88` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x01\x07W_\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV\xFE\xA2dipfsX\"\x12 \x99\xF7\x90\x12\xFD\xC7!\xB81\x89`\x9Ae\xC5\xFC\xFA\xBB\xFF\\\x0E+\xBF\xCF\xBE\xFE\xB3\xA9\x0F\xD7\x85\xB1TdsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static MOCKVERIFIER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80c\xEAP\xD0\xE4\x14a\0-W[_\x80\xFD[a\0Ha\0;6`\x04a\0\\V[_T`\xFF\x16\x94\x93PPPPV[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3[_\x80_\x80`@\x85\x87\x03\x12\x15a\0oW_\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\0\x86W_\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\0\x99W_\x80\xFD[\x815\x81\x81\x11\x15a\0\xA7W_\x80\xFD[\x88` \x82\x85\x01\x01\x11\x15a\0\xB8W_\x80\xFD[` \x92\x83\x01\x96P\x94P\x90\x86\x015\x90\x80\x82\x11\x15a\0\xD2W_\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\0\xE5W_\x80\xFD[\x815\x81\x81\x11\x15a\0\xF3W_\x80\xFD[\x88` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x01\x07W_\x80\xFD[\x95\x98\x94\x97PP` \x01\x94PPPV\xFE\xA2dipfsX\"\x12 \x99\xF7\x90\x12\xFD\xC7!\xB81\x89`\x9Ae\xC5\xFC\xFA\xBB\xFF\\\x0E+\xBF\xCF\xBE\xFE\xB3\xA9\x0F\xD7\x85\xB1TdsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKVERIFIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MockVerifier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockVerifier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockVerifier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockVerifier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockVerifier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockVerifier))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockVerifier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MOCKVERIFIER_ABI.clone(),
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
                MOCKVERIFIER_ABI.clone(),
                MOCKVERIFIER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `verify` (0xea50d0e4) function
        pub fn verify(
            &self,
            p0: ::ethers::core::types::Bytes,
            p1: ::std::vec::Vec<[u8; 32]>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([234, 80, 208, 228], (p0, p1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MockVerifier<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `verify` function with signature `verify(bytes,bytes32[])` and selector `0xea50d0e4`
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
    #[ethcall(name = "verify", abi = "verify(bytes,bytes32[])")]
    pub struct VerifyCall(
        pub ::ethers::core::types::Bytes,
        pub ::std::vec::Vec<[u8; 32]>,
    );
    ///Container type for all return fields from the `verify` function with signature `verify(bytes,bytes32[])` and selector `0xea50d0e4`
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
    pub struct VerifyReturn {
        pub result: bool,
    }
}
