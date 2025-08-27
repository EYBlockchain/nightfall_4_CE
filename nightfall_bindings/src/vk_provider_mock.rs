pub use vk_provider_mock::*;
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
pub mod vk_provider_mock {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("domainSize_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getVerificationKey"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getVerificationKey"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vk"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static VKPROVIDERMOCK_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\tC8\x03\x80a\tC\x839\x81\x01`@\x81\x90Ra\0.\x91a\x005V[_Ua\0LV[_` \x82\x84\x03\x12\x15a\0EW_\x80\xFD[PQ\x91\x90PV[a\x08\xEA\x80a\0Y_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80c\xDF\xC4\xCDN\x14a\0-W[_\x80\xFD[a\x005a\0KV[`@Qa\0B\x91\x90a\x04\xA0V[`@Q\x80\x91\x03\x90\xF3[a\0Sa\0ZV[_T\x81R\x90V[`@Q\x80a\x05@\x01`@R\x80_\x81R` \x01_\x81R` \x01a\0\x8D`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\0\xAD`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\0\xCD`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\0\xED`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01\r`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01-`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01M`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01m`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01\x8D`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01\xAD`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01\xCD`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01\xED`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02\r`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02-`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02M`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02m`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02\x8D`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02\xAD`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02\xCD`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02\xED`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x03\r`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x03-`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x03M`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x03m`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01a\x03\xB1`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x03\xD1`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x03\xF1`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x04\x11`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01a\x04C`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x04o`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x04\x9B`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x90R\x90V[_a\t\xA0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Qa\x04\xD2`@\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P``\x83\x01Q\x80Q`\x80\x84\x01R` \x81\x01Q`\xA0\x84\x01RP`\x80\x83\x01Q\x80Q`\xC0\x84\x01R` \x81\x01Q`\xE0\x84\x01RP`\xA0\x83\x01Qa\x01\0a\x05\x1F\x81\x85\x01\x83\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\xC0\x85\x01Q\x91Pa\x01@a\x05?\x81\x86\x01\x84\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\xE0\x86\x01Q\x92Pa\x01\x80a\x05_\x81\x87\x01\x85\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x91\x86\x01Q\x92Pa\x01\xC0\x91a\x05\x7F\x86\x84\x01\x85\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01 \x87\x01Q\x93Pa\x02\0a\x05\xA0\x81\x88\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x91\x87\x01Q\x93Pa\x02@\x91a\x05\xC0\x87\x84\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01`\x88\x01Q\x94Pa\x02\x80a\x05\xE1\x81\x89\x01\x87\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x91\x88\x01Q\x94Pa\x02\xC0\x91a\x06\x01\x88\x84\x01\x87\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01\xA0\x89\x01Q\x95Pa\x03\0a\x06\"\x81\x8A\x01\x88\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x94\x89\x01Q\x95Pa\x03@\x94a\x06B\x89\x87\x01\x88\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01\xE0\x8A\x01Q\x96Pa\x03\x80a\x06c\x81\x8B\x01\x89\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x92\x8A\x01Q\x96Pa\x03\xC0\x92a\x06\x83\x8A\x85\x01\x89\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x02 \x8B\x01Q\x97Pa\x04\0a\x06\xA4\x81\x8C\x01\x8A\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x95\x8B\x01Q\x97Pa\x04@\x95a\x06\xC4\x8B\x88\x01\x8A\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x02`\x8C\x01Q\x98Pa\x04\x80a\x06\xE5\x81\x8D\x01\x8B\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x93\x8C\x01Q\x98Pa\x04\xC0\x93a\x07\x05\x8C\x86\x01\x8B\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x02\xA0\x8D\x01Q\x99Pa\x05\0a\x07&\x81\x8E\x01\x8C\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x96\x8D\x01Q\x80Qa\x05@\x8E\x01R` \x90\x81\x01Qa\x05`\x8E\x01Ra\x02\xE0\x8E\x01Q\x80Qa\x05\x80\x8F\x01R\x81\x01Qa\x05\xA0\x8E\x01R\x93\x8D\x01Q\x80Qa\x05\xC0\x8E\x01R\x84\x01Qa\x05\xE0\x8D\x01Ra\x03 \x8D\x01Q\x80Qa\x06\0\x8E\x01R\x84\x01Qa\x06 \x8D\x01R\x97\x8C\x01Qa\x06@\x8C\x01Ra\x03`\x8C\x01Qa\x06`\x8C\x01R\x90\x8B\x01Qa\x06\x80\x8B\x01Ra\x03\xA0\x8B\x01Qa\x06\xA0\x8B\x01R\x92\x8A\x01Qa\x06\xC0\x8A\x01Ra\x03\xE0\x8A\x01Qa\x06\xE0\x8A\x01R\x91\x89\x01Q\x80Qa\x07\0\x8A\x01R\x82\x01Qa\x07 \x89\x01Ra\x04 \x89\x01Q\x80Qa\x07@\x8A\x01R\x82\x01Qa\x07`\x89\x01R\x92\x88\x01Q\x80Qa\x07\x80\x89\x01R\x81\x01Qa\x07\xA0\x88\x01Ra\x04`\x88\x01Q\x80Qa\x07\xC0\x89\x01R\x81\x01Qa\x07\xE0\x88\x01R\x92\x87\x01Qa\x08\0\x87\x01Ra\x04\xA0\x87\x01Qa\x08 \x87\x01R\x90\x86\x01Qa\x08@\x86\x01Ra\x04\xE0\x86\x01Q\x80Qa\x08`\x87\x01R\x82\x01Qa\x08\x80\x86\x01R\x85\x01Q\x80Qa\x08\xA0\x86\x01R\x90\x81\x01Qa\x08\xC0\x85\x01R`@\x81\x01Qa\x08\xE0\x85\x01R``\x81\x01Qa\t\0\x85\x01R\x90PPa\x05 \x92\x90\x92\x01Q\x80Qa\t \x83\x01R` \x81\x01Qa\t@\x83\x01R`@\x81\x01Qa\t`\x83\x01R``\x01Qa\t\x80\x90\x91\x01R\x90V\xFE\xA2dipfsX\"\x12 \x13\xE6<\x1AyD\xAA\xE7\x18\xCBELa#.\xA6\x12/\x99\xC8\xE17w\xCDl\"b\xC7\\fW?dsolcC\0\x08\x18\x003";
    /// The bytecode of the contract.
    pub static VKPROVIDERMOCK_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0)W_5`\xE0\x1C\x80c\xDF\xC4\xCDN\x14a\0-W[_\x80\xFD[a\x005a\0KV[`@Qa\0B\x91\x90a\x04\xA0V[`@Q\x80\x91\x03\x90\xF3[a\0Sa\0ZV[_T\x81R\x90V[`@Q\x80a\x05@\x01`@R\x80_\x81R` \x01_\x81R` \x01a\0\x8D`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\0\xAD`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\0\xCD`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\0\xED`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01\r`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01-`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01M`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01m`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01\x8D`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01\xAD`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01\xCD`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x01\xED`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02\r`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02-`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02M`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02m`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02\x8D`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02\xAD`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02\xCD`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x02\xED`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x03\r`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x03-`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x03M`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x03m`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01a\x03\xB1`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x03\xD1`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x03\xF1`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x04\x11`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81R` \x01a\x04C`@Q\x80`@\x01`@R\x80_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x04o`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x81R` \x01a\x04\x9B`@Q\x80`\x80\x01`@R\x80_\x81R` \x01_\x81R` \x01_\x81R` \x01_\x81RP\x90V[\x90R\x90V[_a\t\xA0\x82\x01\x90P\x82Q\x82R` \x83\x01Q` \x83\x01R`@\x83\x01Qa\x04\xD2`@\x84\x01\x82\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[P``\x83\x01Q\x80Q`\x80\x84\x01R` \x81\x01Q`\xA0\x84\x01RP`\x80\x83\x01Q\x80Q`\xC0\x84\x01R` \x81\x01Q`\xE0\x84\x01RP`\xA0\x83\x01Qa\x01\0a\x05\x1F\x81\x85\x01\x83\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\xC0\x85\x01Q\x91Pa\x01@a\x05?\x81\x86\x01\x84\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[`\xE0\x86\x01Q\x92Pa\x01\x80a\x05_\x81\x87\x01\x85\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x91\x86\x01Q\x92Pa\x01\xC0\x91a\x05\x7F\x86\x84\x01\x85\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01 \x87\x01Q\x93Pa\x02\0a\x05\xA0\x81\x88\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x91\x87\x01Q\x93Pa\x02@\x91a\x05\xC0\x87\x84\x01\x86\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01`\x88\x01Q\x94Pa\x02\x80a\x05\xE1\x81\x89\x01\x87\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x91\x88\x01Q\x94Pa\x02\xC0\x91a\x06\x01\x88\x84\x01\x87\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01\xA0\x89\x01Q\x95Pa\x03\0a\x06\"\x81\x8A\x01\x88\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x94\x89\x01Q\x95Pa\x03@\x94a\x06B\x89\x87\x01\x88\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x01\xE0\x8A\x01Q\x96Pa\x03\x80a\x06c\x81\x8B\x01\x89\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x92\x8A\x01Q\x96Pa\x03\xC0\x92a\x06\x83\x8A\x85\x01\x89\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x02 \x8B\x01Q\x97Pa\x04\0a\x06\xA4\x81\x8C\x01\x8A\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x95\x8B\x01Q\x97Pa\x04@\x95a\x06\xC4\x8B\x88\x01\x8A\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x02`\x8C\x01Q\x98Pa\x04\x80a\x06\xE5\x81\x8D\x01\x8B\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x93\x8C\x01Q\x98Pa\x04\xC0\x93a\x07\x05\x8C\x86\x01\x8B\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[a\x02\xA0\x8D\x01Q\x99Pa\x05\0a\x07&\x81\x8E\x01\x8C\x80Q\x82R` \x90\x81\x01Q\x91\x01RV[\x96\x8D\x01Q\x80Qa\x05@\x8E\x01R` \x90\x81\x01Qa\x05`\x8E\x01Ra\x02\xE0\x8E\x01Q\x80Qa\x05\x80\x8F\x01R\x81\x01Qa\x05\xA0\x8E\x01R\x93\x8D\x01Q\x80Qa\x05\xC0\x8E\x01R\x84\x01Qa\x05\xE0\x8D\x01Ra\x03 \x8D\x01Q\x80Qa\x06\0\x8E\x01R\x84\x01Qa\x06 \x8D\x01R\x97\x8C\x01Qa\x06@\x8C\x01Ra\x03`\x8C\x01Qa\x06`\x8C\x01R\x90\x8B\x01Qa\x06\x80\x8B\x01Ra\x03\xA0\x8B\x01Qa\x06\xA0\x8B\x01R\x92\x8A\x01Qa\x06\xC0\x8A\x01Ra\x03\xE0\x8A\x01Qa\x06\xE0\x8A\x01R\x91\x89\x01Q\x80Qa\x07\0\x8A\x01R\x82\x01Qa\x07 \x89\x01Ra\x04 \x89\x01Q\x80Qa\x07@\x8A\x01R\x82\x01Qa\x07`\x89\x01R\x92\x88\x01Q\x80Qa\x07\x80\x89\x01R\x81\x01Qa\x07\xA0\x88\x01Ra\x04`\x88\x01Q\x80Qa\x07\xC0\x89\x01R\x81\x01Qa\x07\xE0\x88\x01R\x92\x87\x01Qa\x08\0\x87\x01Ra\x04\xA0\x87\x01Qa\x08 \x87\x01R\x90\x86\x01Qa\x08@\x86\x01Ra\x04\xE0\x86\x01Q\x80Qa\x08`\x87\x01R\x82\x01Qa\x08\x80\x86\x01R\x85\x01Q\x80Qa\x08\xA0\x86\x01R\x90\x81\x01Qa\x08\xC0\x85\x01R`@\x81\x01Qa\x08\xE0\x85\x01R``\x81\x01Qa\t\0\x85\x01R\x90PPa\x05 \x92\x90\x92\x01Q\x80Qa\t \x83\x01R` \x81\x01Qa\t@\x83\x01R`@\x81\x01Qa\t`\x83\x01R``\x01Qa\t\x80\x90\x91\x01R\x90V\xFE\xA2dipfsX\"\x12 \x13\xE6<\x1AyD\xAA\xE7\x18\xCBELa#.\xA6\x12/\x99\xC8\xE17w\xCDl\"b\xC7\\fW?dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static VKPROVIDERMOCK_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct VKProviderMock<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for VKProviderMock<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for VKProviderMock<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for VKProviderMock<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for VKProviderMock<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(VKProviderMock))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> VKProviderMock<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    VKPROVIDERMOCK_ABI.clone(),
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
                VKPROVIDERMOCK_ABI.clone(),
                VKPROVIDERMOCK_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getVerificationKey` (0xdfc4cd4e) function
        pub fn get_verification_key(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, VerificationKey> {
            self.0
                .method_hash([223, 196, 205, 78], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for VKProviderMock<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Container type for all return fields from the `getVerificationKey` function with signature `getVerificationKey()` and selector `0xdfc4cd4e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
    )]
    pub struct GetVerificationKeyReturn {
        pub vk: VerificationKey,
    }
}
