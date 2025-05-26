pub use der_parser::*;
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
pub mod der_parser {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("computeNumberOfTlvs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeNumberOfTlvs",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("derBytes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pointer"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("parseDER"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("parseDER"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("derBytes"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pointer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tlvLength"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(1usize),
                                                        ],
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct DERParser.DecodedTlv[]",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
    pub static DERPARSER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0EW__\xFD[Pa\x0B\x05\x80a\0\x1C_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80c\x16\x93(\n\x14a\08W\x80c\xB0\xC5\x05U\x14a\0aW[__\xFD[a\0Ka\0F6`\x04a\x08\x8CV[a\0\x82V[`@Qa\0X\x91\x90a\t\x07V[`@Q\x80\x91\x03\x90\xF3[a\0ta\0o6`\x04a\t\xDEV[a\0\x99V[`@Q\x90\x81R` \x01a\0XV[``a\0\x90\x85\x85\x85\x85a\x01[V[\x95\x94PPPPPV[_a\0\xA2a\x07\xD9V[_\x80a\0\xACa\x08)V[a\0\xB8\x88\x88\x88\x86a\x02\x91V[\x96P\x93P\x81a\0\xC6\x81a\n:V[\x92PP\x83`@\x01Q_\x01Q\x15a\x01\nW``\x84\x01Qa\0\xE5\x90\x87a\nRV[\x81\x84`\x05\x81\x10a\0\xF7Wa\0\xF7a\nkV[` \x02\x01R\x82a\x01\x06\x81a\n:V[\x93PP[_[`\x05\x81\x10\x15a\x01HW\x81\x81`\x05\x81\x10a\x01'Wa\x01'a\nkV[` \x02\x01Q\x87\x03a\x01@W\x83a\x01<\x81a\n\x7FV[\x94PP[`\x01\x01a\x01\x0CV[P\x86\x86\x10a\0\xACWP\x96\x95PPPPPPV[``a\x01ea\x07\xD9V[_\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x7FWa\x01\x7Fa\n\x94V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xB8W\x81` \x01[a\x01\xA5a\x07\xD9V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01\x9DW\x90P[P\x90P_\x80a\x01\xC5a\x08)V[a\x01\xD1\x8A\x8A\x8A\x86a\x02\x91V[\x98P\x94P\x84\x84\x83a\x01\xE1\x81a\n:V[\x94P\x81Q\x81\x10a\x01\xF3Wa\x01\xF3a\nkV[` \x90\x81\x02\x91\x90\x91\x01\x01R`@\x85\x01QQ\x15a\x02=W``\x85\x01Qa\x02\x18\x90\x89a\nRV[\x81\x84`\x05\x81\x10a\x02*Wa\x02*a\nkV[` \x02\x01R\x82a\x029\x81a\n:V[\x93PP[_[`\x05\x81\x10\x15a\x02{W\x81\x81`\x05\x81\x10a\x02ZWa\x02Za\nkV[` \x02\x01Q\x89\x03a\x02sW\x83a\x02o\x81a\n\x7FV[\x94PP[`\x01\x01a\x02?V[P\x88\x88\x10a\x01\xC5WP\x91\x98\x97PPPPPPPPV[a\x02\x99a\x07\xD9V[_a\x02\xB3`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x90V[_``\x81\x87a\x02\xDD\x8B\x8B\x83\x81\x81\x10a\x02\xCDWa\x02\xCDa\nkV[\x90P\x015`\xF8\x1C`\xF8\x1B\x8Aa\x03\xBDV[\x90\x9AP\x90\x95P\x91Pa\x02\xFBa\x02\xF4\x8B\x8B\x81\x8Fa\n\xA8V[\x8B\x85a\x05NV[\x90\x9AP\x90\x94P\x91Pa\x03\x1Aa\x03\x12\x8B\x8B\x81\x8Fa\n\xA8V[\x86\x8C\x89a\x07\x1FV[\x99P\x92P_\x8B\x82\x8C\x87a\x03-\x87\x84a\nRV[a\x037\x91\x90a\nRV[\x92a\x03D\x93\x92\x91\x90a\n\xA8V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q`\xE0\x81\x01\x82R\x95\x86R` \x86\x01\x96\x90\x96RPP\x92\x82\x01\x95\x90\x95R``\x81\x01\x93\x90\x93R`\x80\x83\x01\x91\x90\x91R`\xA0\x82\x01R`\xC0\x81\x01\x94\x90\x94RP\x91\x95\x92\x94P\x91\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_\x80`\x03`\xFE\x1B\x85\x16`\x01`\xFD\x1B\x86\x16\x15\x15`\x1F`\xF8\x1B\x80\x88\x16\x90\x84\x90\x82\x10a\x04\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FDERParser: Tag is Long Form, whi`D\x82\x01Rr\x18\xDA\x08\x1A\\\xC8\x1B\x9B\xDD\x08\x1C\xDD\\\x1C\x1B\xDC\x9D\x19Y`j\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xF8\x1B\x03\x19\x84\x16\x15\x80a\x04\x80WP`\x01`\xFF\x1B`\x01`\x01`\xF8\x1B\x03\x19\x85\x16\x14[a\x05\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FDERParser: Only the Universal or`D\x82\x01R\x7F ContextSpecific tag classes are`d\x82\x01Ri\x08\x1C\xDD\\\x1C\x1B\xDC\x9D\x19Y`\xB2\x1B`\x84\x82\x01R`\xA4\x01a\x04SV[\x80a\x05\x0F\x81a\n:V[\x91PP`@Q\x80`@\x01`@R\x80\x84\x15\x15\x81R` \x01\x83`\x01`\x01`\xF8\x1B\x03\x19\x16\x81RP\x88a\x05=\x90a\n:V[\x90\x9A\x90\x99P\x90\x97P\x95PPPPPPV[_\x80\x80\x83a\x05[\x81a\n:V[\x94PP_\x87\x87_\x81\x81\x10a\x05qWa\x05qa\nkV[\x90\x91\x015`\x01`\xFF\x1B\x16\x15\x91P_\x90P\x88\x88\x82\x81a\x05\x91Wa\x05\x91a\nkV[\x90\x91\x015`\xF8\x1C`\x7F\x16\x91PP\x81\x15a\x05\xBFW\x80a\x05\xAE\x88a\n:V[\x97P\x87\x87\x94P\x94P\x94PPPa\x07\x15V[\x80_\x03a\x06&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FDERParser: Indefinite lengths ar`D\x82\x01Rn\x19H\x1B\x9B\xDD\x08\x1C\xDD\\\x1C\x1B\xDC\x9D\x19Y`\x8A\x1B`d\x82\x01R`\x84\x01a\x04SV[\x80`\x7F\x03a\x06\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDERParser: A value of 0x7F for a`D\x82\x01R\x7F long form length is a reserved `d\x82\x01Rdvalue`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\x04SV[_\x80[\x82\x81\x10\x15a\x06\xEAW\x8A\x8Aa\x06\xC2\x83`\x01a\nRV[\x81\x81\x10a\x06\xD1Wa\x06\xD1a\nkV[`\x08\x94\x90\x94\x1B\x91\x90\x93\x015`\xF8\x1C\x17\x91P`\x01\x01a\x06\xADV[P\x80a\x06\xF6\x83\x8Aa\nRV[a\x07\x01\x90`\x01a\nRV[a\x07\x0B\x84\x8Aa\nRV[\x95P\x95P\x95PPPP[\x94P\x94P\x94\x91PPV[``_\x82_\x01Q\x15a\x07{Wa\x077\x85_\x88\x8Aa\n\xA8V[\x85\x82\x82\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x92\x97P\x92\x95Pa\x07\xCF\x94PPPPPV[a\x07\x87\x85_\x88\x8Aa\n\xA8V[a\x07\x91\x87\x87a\nRV[\x82\x82\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x92\x97P\x92\x95PPPPPP[\x95P\x95\x93PPPPV[`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01a\x08\t`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x90V[\x81R` \x01_\x81R` \x01``\x81R` \x01``\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80`\x05\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[__\x83`\x1F\x84\x01\x12a\x08WW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08nW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x08\x85W__\xFD[\x92P\x92\x90PV[____``\x85\x87\x03\x12\x15a\x08\x9FW__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xB5W__\xFD[a\x08\xC1\x87\x82\x88\x01a\x08GV[\x90\x98\x90\x97P` \x87\x015\x96`@\x015\x95P\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\t\xD2W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q\x86R` \x81\x01Q` \x87\x01R`@\x81\x01Q\x80Q\x15\x15`@\x88\x01R`\xFF`\xF8\x1B` \x82\x01Q\x16``\x88\x01RP``\x81\x01Q`\x80\x87\x01R`\x80\x81\x01Qa\x01\0`\xA0\x88\x01Ra\t\x92a\x01\0\x88\x01\x82a\x08\xD9V[\x90P`\xA0\x82\x01Q\x87\x82\x03`\xC0\x89\x01Ra\t\xAB\x82\x82a\x08\xD9V[`\xC0\x93\x90\x93\x01Q`\xE0\x98\x90\x98\x01\x97\x90\x97RP\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\t-V[P\x92\x96\x95PPPPPPV[___`@\x84\x86\x03\x12\x15a\t\xF0W__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\x06W__\xFD[a\n\x12\x86\x82\x87\x01a\x08GV[\x90\x97\x90\x96P` \x95\x90\x95\x015\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01a\nKWa\nKa\n&V[P`\x01\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\neWa\nea\n&V[\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x81a\n\x8DWa\n\x8Da\n&V[P_\x19\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__\x85\x85\x11\x15a\n\xB6W__\xFD[\x83\x86\x11\x15a\n\xC2W__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV\xFE\xA2dipfsX\"\x12 U\xD93\xB2*\x0F=\x08\xA0<jUd\x13\x0E\xD1b\xD9-\\Q\x06\x14:Qz3{_\x8A\x84\xCFdsolcC\0\x08\x1C\x003";
    /// The bytecode of the contract.
    pub static DERPARSER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80c\x16\x93(\n\x14a\08W\x80c\xB0\xC5\x05U\x14a\0aW[__\xFD[a\0Ka\0F6`\x04a\x08\x8CV[a\0\x82V[`@Qa\0X\x91\x90a\t\x07V[`@Q\x80\x91\x03\x90\xF3[a\0ta\0o6`\x04a\t\xDEV[a\0\x99V[`@Q\x90\x81R` \x01a\0XV[``a\0\x90\x85\x85\x85\x85a\x01[V[\x95\x94PPPPPV[_a\0\xA2a\x07\xD9V[_\x80a\0\xACa\x08)V[a\0\xB8\x88\x88\x88\x86a\x02\x91V[\x96P\x93P\x81a\0\xC6\x81a\n:V[\x92PP\x83`@\x01Q_\x01Q\x15a\x01\nW``\x84\x01Qa\0\xE5\x90\x87a\nRV[\x81\x84`\x05\x81\x10a\0\xF7Wa\0\xF7a\nkV[` \x02\x01R\x82a\x01\x06\x81a\n:V[\x93PP[_[`\x05\x81\x10\x15a\x01HW\x81\x81`\x05\x81\x10a\x01'Wa\x01'a\nkV[` \x02\x01Q\x87\x03a\x01@W\x83a\x01<\x81a\n\x7FV[\x94PP[`\x01\x01a\x01\x0CV[P\x86\x86\x10a\0\xACWP\x96\x95PPPPPPV[``a\x01ea\x07\xD9V[_\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x01\x7FWa\x01\x7Fa\n\x94V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x01\xB8W\x81` \x01[a\x01\xA5a\x07\xD9V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x01\x9DW\x90P[P\x90P_\x80a\x01\xC5a\x08)V[a\x01\xD1\x8A\x8A\x8A\x86a\x02\x91V[\x98P\x94P\x84\x84\x83a\x01\xE1\x81a\n:V[\x94P\x81Q\x81\x10a\x01\xF3Wa\x01\xF3a\nkV[` \x90\x81\x02\x91\x90\x91\x01\x01R`@\x85\x01QQ\x15a\x02=W``\x85\x01Qa\x02\x18\x90\x89a\nRV[\x81\x84`\x05\x81\x10a\x02*Wa\x02*a\nkV[` \x02\x01R\x82a\x029\x81a\n:V[\x93PP[_[`\x05\x81\x10\x15a\x02{W\x81\x81`\x05\x81\x10a\x02ZWa\x02Za\nkV[` \x02\x01Q\x89\x03a\x02sW\x83a\x02o\x81a\n\x7FV[\x94PP[`\x01\x01a\x02?V[P\x88\x88\x10a\x01\xC5WP\x91\x98\x97PPPPPPPPV[a\x02\x99a\x07\xD9V[_a\x02\xB3`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x90V[_``\x81\x87a\x02\xDD\x8B\x8B\x83\x81\x81\x10a\x02\xCDWa\x02\xCDa\nkV[\x90P\x015`\xF8\x1C`\xF8\x1B\x8Aa\x03\xBDV[\x90\x9AP\x90\x95P\x91Pa\x02\xFBa\x02\xF4\x8B\x8B\x81\x8Fa\n\xA8V[\x8B\x85a\x05NV[\x90\x9AP\x90\x94P\x91Pa\x03\x1Aa\x03\x12\x8B\x8B\x81\x8Fa\n\xA8V[\x86\x8C\x89a\x07\x1FV[\x99P\x92P_\x8B\x82\x8C\x87a\x03-\x87\x84a\nRV[a\x037\x91\x90a\nRV[\x92a\x03D\x93\x92\x91\x90a\n\xA8V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPP`@\x80Q`\xE0\x81\x01\x82R\x95\x86R` \x86\x01\x96\x90\x96RPP\x92\x82\x01\x95\x90\x95R``\x81\x01\x93\x90\x93R`\x80\x83\x01\x91\x90\x91R`\xA0\x82\x01R`\xC0\x81\x01\x94\x90\x94RP\x91\x95\x92\x94P\x91\x92PPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R_\x80`\x03`\xFE\x1B\x85\x16`\x01`\xFD\x1B\x86\x16\x15\x15`\x1F`\xF8\x1B\x80\x88\x16\x90\x84\x90\x82\x10a\x04\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FDERParser: Tag is Long Form, whi`D\x82\x01Rr\x18\xDA\x08\x1A\\\xC8\x1B\x9B\xDD\x08\x1C\xDD\\\x1C\x1B\xDC\x9D\x19Y`j\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xF8\x1B\x03\x19\x84\x16\x15\x80a\x04\x80WP`\x01`\xFF\x1B`\x01`\x01`\xF8\x1B\x03\x19\x85\x16\x14[a\x05\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R\x7FDERParser: Only the Universal or`D\x82\x01R\x7F ContextSpecific tag classes are`d\x82\x01Ri\x08\x1C\xDD\\\x1C\x1B\xDC\x9D\x19Y`\xB2\x1B`\x84\x82\x01R`\xA4\x01a\x04SV[\x80a\x05\x0F\x81a\n:V[\x91PP`@Q\x80`@\x01`@R\x80\x84\x15\x15\x81R` \x01\x83`\x01`\x01`\xF8\x1B\x03\x19\x16\x81RP\x88a\x05=\x90a\n:V[\x90\x9A\x90\x99P\x90\x97P\x95PPPPPPV[_\x80\x80\x83a\x05[\x81a\n:V[\x94PP_\x87\x87_\x81\x81\x10a\x05qWa\x05qa\nkV[\x90\x91\x015`\x01`\xFF\x1B\x16\x15\x91P_\x90P\x88\x88\x82\x81a\x05\x91Wa\x05\x91a\nkV[\x90\x91\x015`\xF8\x1C`\x7F\x16\x91PP\x81\x15a\x05\xBFW\x80a\x05\xAE\x88a\n:V[\x97P\x87\x87\x94P\x94P\x94PPPa\x07\x15V[\x80_\x03a\x06&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`/`$\x82\x01R\x7FDERParser: Indefinite lengths ar`D\x82\x01Rn\x19H\x1B\x9B\xDD\x08\x1C\xDD\\\x1C\x1B\xDC\x9D\x19Y`\x8A\x1B`d\x82\x01R`\x84\x01a\x04SV[\x80`\x7F\x03a\x06\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDERParser: A value of 0x7F for a`D\x82\x01R\x7F long form length is a reserved `d\x82\x01Rdvalue`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\x04SV[_\x80[\x82\x81\x10\x15a\x06\xEAW\x8A\x8Aa\x06\xC2\x83`\x01a\nRV[\x81\x81\x10a\x06\xD1Wa\x06\xD1a\nkV[`\x08\x94\x90\x94\x1B\x91\x90\x93\x015`\xF8\x1C\x17\x91P`\x01\x01a\x06\xADV[P\x80a\x06\xF6\x83\x8Aa\nRV[a\x07\x01\x90`\x01a\nRV[a\x07\x0B\x84\x8Aa\nRV[\x95P\x95P\x95PPPP[\x94P\x94P\x94\x91PPV[``_\x82_\x01Q\x15a\x07{Wa\x077\x85_\x88\x8Aa\n\xA8V[\x85\x82\x82\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x92\x97P\x92\x95Pa\x07\xCF\x94PPPPPV[a\x07\x87\x85_\x88\x8Aa\n\xA8V[a\x07\x91\x87\x87a\nRV[\x82\x82\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RP\x92\x97P\x92\x95PPPPPP[\x95P\x95\x93PPPPV[`@Q\x80`\xE0\x01`@R\x80_\x81R` \x01_\x81R` \x01a\x08\t`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x90V[\x81R` \x01_\x81R` \x01``\x81R` \x01``\x81R` \x01_\x81RP\x90V[`@Q\x80`\xA0\x01`@R\x80`\x05\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[__\x83`\x1F\x84\x01\x12a\x08WW__\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08nW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x08\x85W__\xFD[\x92P\x92\x90PV[____``\x85\x87\x03\x12\x15a\x08\x9FW__\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xB5W__\xFD[a\x08\xC1\x87\x82\x88\x01a\x08GV[\x90\x98\x90\x97P` \x87\x015\x96`@\x015\x95P\x93PPPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a\t\xD2W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q\x86R` \x81\x01Q` \x87\x01R`@\x81\x01Q\x80Q\x15\x15`@\x88\x01R`\xFF`\xF8\x1B` \x82\x01Q\x16``\x88\x01RP``\x81\x01Q`\x80\x87\x01R`\x80\x81\x01Qa\x01\0`\xA0\x88\x01Ra\t\x92a\x01\0\x88\x01\x82a\x08\xD9V[\x90P`\xA0\x82\x01Q\x87\x82\x03`\xC0\x89\x01Ra\t\xAB\x82\x82a\x08\xD9V[`\xC0\x93\x90\x93\x01Q`\xE0\x98\x90\x98\x01\x97\x90\x97RP\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a\t-V[P\x92\x96\x95PPPPPPV[___`@\x84\x86\x03\x12\x15a\t\xF0W__\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n\x06W__\xFD[a\n\x12\x86\x82\x87\x01a\x08GV[\x90\x97\x90\x96P` \x95\x90\x95\x015\x94\x93PPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_`\x01\x82\x01a\nKWa\nKa\n&V[P`\x01\x01\x90V[\x80\x82\x01\x80\x82\x11\x15a\neWa\nea\n&V[\x92\x91PPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x81a\n\x8DWa\n\x8Da\n&V[P_\x19\x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[__\x85\x85\x11\x15a\n\xB6W__\xFD[\x83\x86\x11\x15a\n\xC2W__\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV\xFE\xA2dipfsX\"\x12 U\xD93\xB2*\x0F=\x08\xA0<jUd\x13\x0E\xD1b\xD9-\\Q\x06\x14:Qz3{_\x8A\x84\xCFdsolcC\0\x08\x1C\x003";
    /// The deployed bytecode of the contract.
    pub static DERPARSER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DERParser<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DERParser<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DERParser<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DERParser<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DERParser<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DERParser)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DERParser<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DERPARSER_ABI.clone(),
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
                DERPARSER_ABI.clone(),
                DERPARSER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `computeNumberOfTlvs` (0xb0c50555) function
        pub fn compute_number_of_tlvs(
            &self,
            der_bytes: ::ethers::core::types::Bytes,
            pointer: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([176, 197, 5, 85], (der_bytes, pointer))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseDER` (0x1693280a) function
        pub fn parse_der(
            &self,
            der_bytes: ::ethers::core::types::Bytes,
            pointer: ::ethers::core::types::U256,
            tlv_length: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<DecodedTlv>> {
            self.0
                .method_hash([22, 147, 40, 10], (der_bytes, pointer, tlv_length))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DERParser<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `computeNumberOfTlvs` function with signature `computeNumberOfTlvs(bytes,uint256)` and selector `0xb0c50555`
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
    #[ethcall(name = "computeNumberOfTlvs", abi = "computeNumberOfTlvs(bytes,uint256)")]
    pub struct ComputeNumberOfTlvsCall {
        pub der_bytes: ::ethers::core::types::Bytes,
        pub pointer: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `parseDER` function with signature `parseDER(bytes,uint256,uint256)` and selector `0x1693280a`
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
    #[ethcall(name = "parseDER", abi = "parseDER(bytes,uint256,uint256)")]
    pub struct ParseDERCall {
        pub der_bytes: ::ethers::core::types::Bytes,
        pub pointer: ::ethers::core::types::U256,
        pub tlv_length: ::ethers::core::types::U256,
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
    pub enum DERParserCalls {
        ComputeNumberOfTlvs(ComputeNumberOfTlvsCall),
        ParseDER(ParseDERCall),
    }
    impl ::ethers::core::abi::AbiDecode for DERParserCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ComputeNumberOfTlvsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeNumberOfTlvs(decoded));
            }
            if let Ok(decoded) = <ParseDERCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseDER(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DERParserCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ComputeNumberOfTlvs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ParseDER(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DERParserCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ComputeNumberOfTlvs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseDER(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ComputeNumberOfTlvsCall> for DERParserCalls {
        fn from(value: ComputeNumberOfTlvsCall) -> Self {
            Self::ComputeNumberOfTlvs(value)
        }
    }
    impl ::core::convert::From<ParseDERCall> for DERParserCalls {
        fn from(value: ParseDERCall) -> Self {
            Self::ParseDER(value)
        }
    }
    ///Container type for all return fields from the `computeNumberOfTlvs` function with signature `computeNumberOfTlvs(bytes,uint256)` and selector `0xb0c50555`
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
    pub struct ComputeNumberOfTlvsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `parseDER` function with signature `parseDER(bytes,uint256,uint256)` and selector `0x1693280a`
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
    pub struct ParseDERReturn(pub ::std::vec::Vec<DecodedTlv>);
}
