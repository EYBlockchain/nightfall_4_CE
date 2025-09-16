pub use transparent_upgradeable_proxy::*;
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
pub mod transparent_upgradeable_proxy {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_logic"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initialOwner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_data"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes"),
                        ),
                    },
                ],
            }),
            functions: ::std::collections::BTreeMap::new(),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AdminChanged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("AdminChanged"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newAdmin"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
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
                    ::std::borrow::ToOwned::to_owned("ERC1967InvalidAdmin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC1967InvalidAdmin",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("admin"),
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
                    ::std::borrow::ToOwned::to_owned("ProxyDeniedAdminAccess"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ProxyDeniedAdminAccess",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static TRANSPARENTUPGRADEABLEPROXY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R`@Qb\0\x0EP8\x03\x80b\0\x0EP\x839\x81\x01`@\x81\x90Rb\0\0&\x91b\0\x03\xBCV[\x82\x81b\0\x004\x82\x82b\0\0\x99V[PP\x81`@Qb\0\0E\x90b\0\x03ZV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90_\xF0\x80\x15\x80\x15b\0\0oW=_\x80>=_\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\x80Rb\0\0\x90b\0\0\x8A`\x80Q\x90V[b\0\0\xFEV[PPPb\0\x04\xB3V[b\0\0\xA4\x82b\0\x01oV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15b\0\0\xF0Wb\0\0\xEB\x82\x82b\0\x01\xEEV[PPPV[b\0\0\xFAb\0\x02gV[PPV[\x7F~dMyB/\x17\xC0\x1EH\x94\xB5\xF4\xF5\x88\xD31\xEB\xFA(e=B\xAE\x83-\xC5\x9E8\xC9y\x8Fb\0\x01?_\x80Q` b\0\x0E0\x839\x81Q\x91RT`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x84\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1b\0\x01l\x81b\0\x02\x89V[PV[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03b\0\x01\xAAW`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[\x80\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC[\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qb\0\x02\x0C\x91\x90b\0\x04\x96V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14b\0\x02FW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>b\0\x02KV[``\x91P[P\x90\x92P\x90Pb\0\x02^\x85\x83\x83b\0\x02\xCAV[\x95\x94PPPPPV[4\x15b\0\x02\x87W`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\x02\xB4W`@Qc1s\xBD\xD1`\xE1\x1B\x81R_`\x04\x82\x01R`$\x01b\0\x01\xA1V[\x80_\x80Q` b\0\x0E0\x839\x81Q\x91Rb\0\x01\xCDV[``\x82b\0\x02\xE3Wb\0\x02\xDD\x82b\0\x030V[b\0\x03)V[\x81Q\x15\x80\x15b\0\x02\xFBWP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15b\0\x03&W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01b\0\x01\xA1V[P\x80[\x93\x92PPPV[\x80Q\x15b\0\x03AW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xFC\x80b\0\t4\x839\x01\x90V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x03\x7FW_\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_[\x83\x81\x10\x15b\0\x03\xB4W\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x03\x9AV[PP_\x91\x01RV[_\x80_``\x84\x86\x03\x12\x15b\0\x03\xCFW_\x80\xFD[b\0\x03\xDA\x84b\0\x03hV[\x92Pb\0\x03\xEA` \x85\x01b\0\x03hV[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x04\x07W_\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12b\0\x04\x1BW_\x80\xFD[\x81Q\x81\x81\x11\x15b\0\x040Wb\0\x040b\0\x03\x84V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15b\0\x04[Wb\0\x04[b\0\x03\x84V[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15b\0\x04tW_\x80\xFD[b\0\x04\x87\x83` \x83\x01` \x88\x01b\0\x03\x98V[\x80\x95PPPPPP\x92P\x92P\x92V[_\x82Qb\0\x04\xA9\x81\x84` \x87\x01b\0\x03\x98V[\x91\x90\x91\x01\x92\x91PPV[`\x80Qa\x04ib\0\x04\xCB_9_`\x10\x01Ra\x04i_\xF3\xFE`\x80`@Ra\0\x0Ca\0\x0EV[\0[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\0zW_5`\x01`\x01`\xE0\x1B\x03\x19\x16c'\x8FyC`\xE1\x1B\x14a\0pW`@Qc4\xAD]\xBB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\0xa\0\x82V[V[a\0xa\0\xB0V[_\x80a\0\x916`\x04\x81\x84a\x03\x03V[\x81\x01\x90a\0\x9E\x91\x90a\x03>V[\x91P\x91Pa\0\xAC\x82\x82a\0\xC0V[PPV[a\0xa\0\xBBa\x01\x1AV[a\x01QV[a\0\xC9\x82a\x01oV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a\x01\x12Wa\x01\r\x82\x82a\x01\xEAV[PPPV[a\0\xACa\x02\\V[_a\x01L\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[6_\x807_\x806_\x84Z\xF4=_\x80>\x80\x80\x15a\x01kW=_\xF3[=_\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a\x01\xA9W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x02\x06\x91\x90a\x04\x07V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x02>W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x02CV[``\x91P[P\x91P\x91Pa\x02S\x85\x83\x83a\x02{V[\x95\x94PPPPPV[4\x15a\0xW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\x02\x90Wa\x02\x8B\x82a\x02\xDAV[a\x02\xD3V[\x81Q\x15\x80\x15a\x02\xA7WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x02\xD0W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x01\xA0V[P\x80[\x93\x92PPPV[\x80Q\x15a\x02\xEAW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x85\x85\x11\x15a\x03\x11W_\x80\xFD[\x83\x86\x11\x15a\x03\x1DW_\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80`@\x83\x85\x03\x12\x15a\x03OW_\x80\xFD[\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03eW_\x80\xFD[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x03\x81W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x03\x94W_\x80\xFD[\x815\x81\x81\x11\x15a\x03\xA6Wa\x03\xA6a\x03*V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x03\xCEWa\x03\xCEa\x03*V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x03\xE6W_\x80\xFD[\x82` \x86\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[_\x82Q_[\x81\x81\x10\x15a\x04&W` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\x04\x0CV[P_\x92\x01\x91\x82RP\x91\x90PV\xFE\xA2dipfsX\"\x12 \xAD\xF5#Ig\x91VH%\xA1p\xAE&IK\x0E\xB0\xB0\xA4\x13\xFF\xEBl2\xD2X\x03u\x85\x81P.dsolcC\0\x08\x18\x003`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa\x04\xFC8\x03\x80a\x04\xFC\x839\x81\x01`@\x81\x90Ra\0.\x91a\0\xBBV[\x80`\x01`\x01`\xA0\x1B\x03\x81\x16a\0\\W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[a\0e\x81a\0lV[PPa\0\xE8V[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[_` \x82\x84\x03\x12\x15a\0\xCBW_\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\xE1W_\x80\xFD[\x93\x92PPPV[a\x04\x07\x80a\0\xF5_9_\xF3\xFE`\x80`@R`\x046\x10a\0IW_5`\xE0\x1C\x80cqP\x18\xA6\x14a\0MW\x80c\x8D\xA5\xCB[\x14a\0cW\x80c\x96#`\x9D\x14a\0\x8EW\x80c\xAD<\xB1\xCC\x14a\0\xA1W\x80c\xF2\xFD\xE3\x8B\x14a\0\xDEW[_\x80\xFD[4\x80\x15a\0XW_\x80\xFD[Pa\0aa\0\xFDV[\0[4\x80\x15a\0nW_\x80\xFD[P_T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0aa\0\x9C6`\x04a\x02`V[a\x01\x10V[4\x80\x15a\0\xACW_\x80\xFD[Pa\0\xD1`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x03R\xE3\x02\xE3`\xDC\x1B\x81RP\x81V[`@Qa\0\x85\x91\x90a\x03rV[4\x80\x15a\0\xE9W_\x80\xFD[Pa\0aa\0\xF86`\x04a\x03\x8BV[a\x01{V[a\x01\x05a\x01\xBDV[a\x01\x0E_a\x01\xE9V[V[a\x01\x18a\x01\xBDV[`@Qc'\x8FyC`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90cO\x1E\xF2\x86\x904\x90a\x01H\x90\x86\x90\x86\x90`\x04\x01a\x03\xA6V[_`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x01_W_\x80\xFD[PZ\xF1\x15\x80\x15a\x01qW=_\x80>=_\xFD[PPPPPPPPV[a\x01\x83a\x01\xBDV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x01\xB1W`@Qc\x1EO\xBD\xF7`\xE0\x1B\x81R_`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\x01\xBA\x81a\x01\xE9V[PV[_T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x01\x0EW`@Qc\x11\x8C\xDA\xA7`\xE0\x1B\x81R3`\x04\x82\x01R`$\x01a\x01\xA8V[_\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x84U`@Q\x91\x90\x92\x16\x92\x83\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xBAW_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80_``\x84\x86\x03\x12\x15a\x02rW_\x80\xFD[\x835a\x02}\x81a\x028V[\x92P` \x84\x015a\x02\x8D\x81a\x028V[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x02\xA9W_\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x02\xBCW_\x80\xFD[\x815\x81\x81\x11\x15a\x02\xCEWa\x02\xCEa\x02LV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x02\xF6Wa\x02\xF6a\x02LV[\x81`@R\x82\x81R\x89` \x84\x87\x01\x01\x11\x15a\x03\x0EW_\x80\xFD[\x82` \x86\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92P\x92V[_\x81Q\x80\x84R_[\x81\x81\x10\x15a\x03SW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x037V[P_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x03\x84` \x83\x01\x84a\x03/V[\x93\x92PPPV[_` \x82\x84\x03\x12\x15a\x03\x9BW_\x80\xFD[\x815a\x03\x84\x81a\x028V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x03\xC9\x90\x83\x01\x84a\x03/V[\x94\x93PPPPV\xFE\xA2dipfsX\"\x12 o\xDA\xC4\x05\xA2\xFF:|\x12\x81\xDC\xE0\x875\\\xF6\xD5\xC7BR\xC4\xC7>\xC7\xA9\xA5\xF14\xE6\xBA\xBE\xFBdsolcC\0\x08\x18\x003\xB51'hJV\x8B1s\xAE\x13\xB9\xF8\xA6\x01n$>c\xB6\xE8\xEE\x11x\xD6\xA7\x17\x85\x0B]a\x03";
    /// The bytecode of the contract.
    pub static TRANSPARENTUPGRADEABLEPROXY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@Ra\0\x0Ca\0\x0EV[\0[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\0zW_5`\x01`\x01`\xE0\x1B\x03\x19\x16c'\x8FyC`\xE1\x1B\x14a\0pW`@Qc4\xAD]\xBB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\0xa\0\x82V[V[a\0xa\0\xB0V[_\x80a\0\x916`\x04\x81\x84a\x03\x03V[\x81\x01\x90a\0\x9E\x91\x90a\x03>V[\x91P\x91Pa\0\xAC\x82\x82a\0\xC0V[PPV[a\0xa\0\xBBa\x01\x1AV[a\x01QV[a\0\xC9\x82a\x01oV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90_\x90\xA2\x80Q\x15a\x01\x12Wa\x01\r\x82\x82a\x01\xEAV[PPPV[a\0\xACa\x02\\V[_a\x01L\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBCT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90P\x90V[6_\x807_\x806_\x84Z\xF4=_\x80>\x80\x80\x15a\x01kW=_\xF3[=_\xFD[\x80`\x01`\x01`\xA0\x1B\x03\x16;_\x03a\x01\xA9W`@QcL\x9C\x8C\xE3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[``_\x80\x84`\x01`\x01`\xA0\x1B\x03\x16\x84`@Qa\x02\x06\x91\x90a\x04\x07V[_`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80_\x81\x14a\x02>W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x02CV[``\x91P[P\x91P\x91Pa\x02S\x85\x83\x83a\x02{V[\x95\x94PPPPPV[4\x15a\0xW`@Qc\xB3\x98\x97\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x82a\x02\x90Wa\x02\x8B\x82a\x02\xDAV[a\x02\xD3V[\x81Q\x15\x80\x15a\x02\xA7WP`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15[\x15a\x02\xD0W`@Qc\x99\x96\xB3\x15`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01a\x01\xA0V[P\x80[\x93\x92PPPV[\x80Q\x15a\x02\xEAW\x80Q\x80\x82` \x01\xFD[`@Qc\xD6\xBD\xA2u`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x85\x85\x11\x15a\x03\x11W_\x80\xFD[\x83\x86\x11\x15a\x03\x1DW_\x80\xFD[PP\x82\x01\x93\x91\x90\x92\x03\x91PV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[_\x80`@\x83\x85\x03\x12\x15a\x03OW_\x80\xFD[\x825`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03eW_\x80\xFD[\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x03\x81W_\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x03\x94W_\x80\xFD[\x815\x81\x81\x11\x15a\x03\xA6Wa\x03\xA6a\x03*V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x03\xCEWa\x03\xCEa\x03*V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x03\xE6W_\x80\xFD[\x82` \x86\x01` \x83\x017_` \x84\x83\x01\x01R\x80\x95PPPPPP\x92P\x92\x90PV[_\x82Q_[\x81\x81\x10\x15a\x04&W` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\x04\x0CV[P_\x92\x01\x91\x82RP\x91\x90PV\xFE\xA2dipfsX\"\x12 \xAD\xF5#Ig\x91VH%\xA1p\xAE&IK\x0E\xB0\xB0\xA4\x13\xFF\xEBl2\xD2X\x03u\x85\x81P.dsolcC\0\x08\x18\x003";
    /// The deployed bytecode of the contract.
    pub static TRANSPARENTUPGRADEABLEPROXY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct TransparentUpgradeableProxy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TransparentUpgradeableProxy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TransparentUpgradeableProxy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TransparentUpgradeableProxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TransparentUpgradeableProxy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(TransparentUpgradeableProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TransparentUpgradeableProxy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TRANSPARENTUPGRADEABLEPROXY_ABI.clone(),
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
                TRANSPARENTUPGRADEABLEPROXY_ABI.clone(),
                TRANSPARENTUPGRADEABLEPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Gets the contract's `AdminChanged` event
        pub fn admin_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AdminChangedFilter,
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
            TransparentUpgradeableProxyEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for TransparentUpgradeableProxy<M> {
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
    ///Custom Error type `ERC1967InvalidAdmin` with signature `ERC1967InvalidAdmin(address)` and selector `0x62e77ba2`
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
    #[etherror(name = "ERC1967InvalidAdmin", abi = "ERC1967InvalidAdmin(address)")]
    pub struct ERC1967InvalidAdmin {
        pub admin: ::ethers::core::types::Address,
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
    ///Custom Error type `ProxyDeniedAdminAccess` with signature `ProxyDeniedAdminAccess()` and selector `0xd2b576ec`
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
    #[etherror(name = "ProxyDeniedAdminAccess", abi = "ProxyDeniedAdminAccess()")]
    pub struct ProxyDeniedAdminAccess;
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
    pub enum TransparentUpgradeableProxyErrors {
        AddressEmptyCode(AddressEmptyCode),
        ERC1967InvalidAdmin(ERC1967InvalidAdmin),
        ERC1967InvalidImplementation(ERC1967InvalidImplementation),
        ERC1967NonPayable(ERC1967NonPayable),
        FailedCall(FailedCall),
        ProxyDeniedAdminAccess(ProxyDeniedAdminAccess),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for TransparentUpgradeableProxyErrors {
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
            if let Ok(decoded) = <ERC1967InvalidAdmin as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC1967InvalidAdmin(decoded));
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
            if let Ok(decoded) = <ProxyDeniedAdminAccess as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProxyDeniedAdminAccess(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for TransparentUpgradeableProxyErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AddressEmptyCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1967InvalidAdmin(element) => {
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
                Self::ProxyDeniedAdminAccess(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for TransparentUpgradeableProxyErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <AddressEmptyCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1967InvalidAdmin as ::ethers::contract::EthError>::selector() => {
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
                    == <ProxyDeniedAdminAccess as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for TransparentUpgradeableProxyErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AddressEmptyCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1967InvalidAdmin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967InvalidImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1967NonPayable(element) => ::core::fmt::Display::fmt(element, f),
                Self::FailedCall(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProxyDeniedAdminAccess(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String>
    for TransparentUpgradeableProxyErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AddressEmptyCode> for TransparentUpgradeableProxyErrors {
        fn from(value: AddressEmptyCode) -> Self {
            Self::AddressEmptyCode(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidAdmin>
    for TransparentUpgradeableProxyErrors {
        fn from(value: ERC1967InvalidAdmin) -> Self {
            Self::ERC1967InvalidAdmin(value)
        }
    }
    impl ::core::convert::From<ERC1967InvalidImplementation>
    for TransparentUpgradeableProxyErrors {
        fn from(value: ERC1967InvalidImplementation) -> Self {
            Self::ERC1967InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<ERC1967NonPayable> for TransparentUpgradeableProxyErrors {
        fn from(value: ERC1967NonPayable) -> Self {
            Self::ERC1967NonPayable(value)
        }
    }
    impl ::core::convert::From<FailedCall> for TransparentUpgradeableProxyErrors {
        fn from(value: FailedCall) -> Self {
            Self::FailedCall(value)
        }
    }
    impl ::core::convert::From<ProxyDeniedAdminAccess>
    for TransparentUpgradeableProxyErrors {
        fn from(value: ProxyDeniedAdminAccess) -> Self {
            Self::ProxyDeniedAdminAccess(value)
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
    #[ethevent(name = "AdminChanged", abi = "AdminChanged(address,address)")]
    pub struct AdminChangedFilter {
        pub previous_admin: ::ethers::core::types::Address,
        pub new_admin: ::ethers::core::types::Address,
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
    pub enum TransparentUpgradeableProxyEvents {
        AdminChangedFilter(AdminChangedFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for TransparentUpgradeableProxyEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AdminChangedFilter::decode_log(log) {
                return Ok(
                    TransparentUpgradeableProxyEvents::AdminChangedFilter(decoded),
                );
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(TransparentUpgradeableProxyEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for TransparentUpgradeableProxyEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminChangedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminChangedFilter>
    for TransparentUpgradeableProxyEvents {
        fn from(value: AdminChangedFilter) -> Self {
            Self::AdminChangedFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for TransparentUpgradeableProxyEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
}
