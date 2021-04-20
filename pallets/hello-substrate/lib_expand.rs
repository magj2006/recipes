#![feature(prelude_import)]
//! A simple Substrate pallet that demonstrates declaring dispatchable functions, and
//! Printing text to the terminal.
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use frame_support::{debug, decl_module, dispatch::DispatchResult};
use frame_system::ensure_signed;
use sp_runtime::print;
pub trait Config: frame_system::Config {}
pub struct Module<T: Config>(::frame_support::sp_std::marker::PhantomData<(T,)>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::clone::Clone + Config> ::core::clone::Clone for Module<T> {
    #[inline]
    fn clone(&self) -> Module<T> {
        match *self {
            Module(ref __self_0_0) => Module(::core::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::marker::Copy + Config> ::core::marker::Copy for Module<T> {}
impl<T: Config> ::core::marker::StructuralPartialEq for Module<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::PartialEq + Config> ::core::cmp::PartialEq for Module<T> {
    #[inline]
    fn eq(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) => match *self {
                Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) => match *self {
                Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl<T: Config> ::core::marker::StructuralEq for Module<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::Eq + Config> ::core::cmp::Eq for Module<T> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<
                ::frame_support::sp_std::marker::PhantomData<(T,)>,
            >;
        }
    }
}
impl<T: Config> core::fmt::Debug for Module<T>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("Module").field(&self.0).finish()
    }
}
impl<T: frame_system::Config + Config>
    ::frame_support::traits::OnInitialize<<T as frame_system::Config>::BlockNumber> for Module<T>
{
}
impl<T: Config> ::frame_support::traits::OnRuntimeUpgrade for Module<T> {
    fn on_runtime_upgrade() -> ::frame_support::dispatch::Weight {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "on_runtime_upgrade",
                        "hello_substrate",
                        ::tracing::Level::TRACE,
                        Some("pallets/hello-substrate/src/lib.rs"),
                        Some(15u32),
                        Some("hello_substrate"),
                        ::tracing_core::field::FieldSet::new(
                            &[],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                MacroCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && CALLSITE.is_enabled(interest)
            {
                let meta = CALLSITE.metadata();
                ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
            } else {
                let span = CALLSITE.disabled_span();
                {};
                span
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        frame_support::traits::PalletVersion {
            major: 3u16,
            minor: 0u8,
            patch: 0u8,
        }
        .put_into_storage::<<T as frame_system::Config>::PalletInfo, Self>();
        <<T as frame_system::Config>::DbWeight as ::frame_support::traits::Get<_>>::get().writes(1)
    }
}
impl<T: frame_system::Config + Config>
    ::frame_support::traits::OnFinalize<<T as frame_system::Config>::BlockNumber> for Module<T>
{
}
impl<T: frame_system::Config + Config>
    ::frame_support::traits::OffchainWorker<<T as frame_system::Config>::BlockNumber>
    for Module<T>
{
}
#[cfg(feature = "std")]
impl<T: Config> ::frame_support::traits::IntegrityTest for Module<T> {}
/// Can also be called using [`Call`].
///
/// [`Call`]: enum.Call.html
impl<T: Config> Module<T> {
    /// A function that says hello to the user by printing messages to the node log
    ///
    /// NOTE: Calling this function will bypass origin filters.
    pub fn say_hello(origin: T::Origin) -> DispatchResult {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "say_hello",
                        "hello_substrate",
                        ::tracing::Level::TRACE,
                        Some("pallets/hello-substrate/src/lib.rs"),
                        Some(15u32),
                        Some("hello_substrate"),
                        ::tracing_core::field::FieldSet::new(
                            &[],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                MacroCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && CALLSITE.is_enabled(interest)
            {
                let meta = CALLSITE.metadata();
                ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
            } else {
                let span = CALLSITE.disabled_span();
                {};
                span
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        let caller = ensure_signed(origin)?;
        print("Hello World");
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1(
                        &["Request sent by: "],
                        &match (&caller,) {
                            (arg0,) => {
                                [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)]
                            }
                        },
                    ),
                    lvl,
                    &(
                        "hello_substrate",
                        "hello_substrate",
                        "pallets/hello-substrate/src/lib.rs",
                        27u32,
                    ),
                );
            }
        };
        Ok(())
    }
}
/// Dispatchable calls.
///
/// Each variant of this enum maps to a dispatchable function from the associated module.
pub enum Call<T: Config> {
    #[doc(hidden)]
    #[codec(skip)]
    __PhantomItem(
        ::frame_support::sp_std::marker::PhantomData<(T,)>,
        ::frame_support::Never,
    ),
    #[allow(non_camel_case_types)]
    /// A function that says hello to the user by printing messages to the node log
    say_hello(),
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_scale_codec as _parity_scale_codec;
    impl<T: Config> _parity_scale_codec::Encode for Call<T> {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                Call::say_hello() => {
                    __codec_dest_edqy.push_byte(0usize as u8);
                }
                _ => (),
            }
        }
    }
    impl<T: Config> _parity_scale_codec::EncodeLike for Call<T> {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_scale_codec as _parity_scale_codec;
    impl<T: Config> _parity_scale_codec::Decode for Call<T> {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
            {
                __codec_x_edqy if __codec_x_edqy == 0usize as u8 => Ok(Call::<T>::say_hello()),
                _ => Err("Could not decode `Call`, variant doesn\'t exist".into()),
            }
        }
    }
};
impl<T: Config> ::frame_support::dispatch::GetDispatchInfo for Call<T> {
    fn get_dispatch_info(&self) -> ::frame_support::dispatch::DispatchInfo {
        match *self {
            Call::say_hello() => {
                let base_weight = 10_000;
                let weight =
                    <dyn ::frame_support::dispatch::WeighData<()>>::weigh_data(&base_weight, ());
                let class =
                    <dyn ::frame_support::dispatch::ClassifyDispatch<()>>::classify_dispatch(
                        &base_weight,
                        (),
                    );
                let pays_fee =
                    <dyn ::frame_support::dispatch::PaysFee<()>>::pays_fee(&base_weight, ());
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::__PhantomItem(_, _) => {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
}
impl<T: Config> ::frame_support::dispatch::GetCallName for Call<T> {
    fn get_call_name(&self) -> &'static str {
        match *self {
            Call::say_hello() => {
                let _ = ();
                "say_hello"
            }
            Call::__PhantomItem(_, _) => {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
    fn get_call_names() -> &'static [&'static str] {
        &["say_hello"]
    }
}
pub use ::frame_support::traits::GetPalletVersion as _;
impl<T: Config> ::frame_support::traits::GetPalletVersion for Module<T> {
    fn current_version() -> ::frame_support::traits::PalletVersion {
        frame_support::traits::PalletVersion {
            major: 3u16,
            minor: 0u8,
            patch: 0u8,
        }
    }
    fn storage_version() -> Option<::frame_support::traits::PalletVersion> {
        let key = ::frame_support::traits::PalletVersion::storage_key::<
            <T as frame_system::Config>::PalletInfo,
            Self,
        >()
        .expect("Every active pallet has a name in the runtime; qed");
        ::frame_support::storage::unhashed::get(&key)
    }
}
impl<T: Config> ::frame_support::traits::OnGenesis for Module<T> {
    fn on_genesis() {
        frame_support::traits::PalletVersion {
            major: 3u16,
            minor: 0u8,
            patch: 0u8,
        }
        .put_into_storage::<<T as frame_system::Config>::PalletInfo, Self>();
    }
}
impl<T: Config> ::frame_support::dispatch::Clone for Call<T> {
    fn clone(&self) -> Self {
        match *self {
            Call::say_hello() => Call::say_hello(),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Config> ::frame_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::say_hello() => {
                let self_params = ();
                if let Call::say_hello() = *_other {
                    self_params == ()
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::core::panicking::panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Config> ::frame_support::dispatch::Eq for Call<T> {}
impl<T: Config> ::frame_support::dispatch::fmt::Debug for Call<T> {
    fn fmt(
        &self,
        _f: &mut ::frame_support::dispatch::fmt::Formatter,
    ) -> ::frame_support::dispatch::result::Result<(), ::frame_support::dispatch::fmt::Error> {
        match *self {
            Call::say_hello() => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"say_hello", &()) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Config> ::frame_support::traits::UnfilteredDispatchable for Call<T> {
    type Origin = T::Origin;
    fn dispatch_bypass_filter(
        self,
        _origin: Self::Origin,
    ) -> ::frame_support::dispatch::DispatchResultWithPostInfo {
        match self {
            Call::say_hello() => <Module<T>>::say_hello(_origin)
                .map(Into::into)
                .map_err(Into::into),
            Call::__PhantomItem(_, _) => {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
}
impl<T: Config> ::frame_support::dispatch::Callable<T> for Module<T> {
    type Call = Call<T>;
}
impl<T: Config> Module<T> {
    #[doc(hidden)]
    #[allow(dead_code)]
    pub fn call_functions() -> &'static [::frame_support::dispatch::FunctionMetadata] {
        &[::frame_support::dispatch::FunctionMetadata {
            name: ::frame_support::dispatch::DecodeDifferent::Encode("say_hello"),
            arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                r" A function that says hello to the user by printing messages to the node log",
            ]),
        }]
    }
}
impl<T: 'static + Config> Module<T> {
    #[doc(hidden)]
    #[allow(dead_code)]
    pub fn module_constants_metadata(
    ) -> &'static [::frame_support::dispatch::ModuleConstantMetadata] {
        &[]
    }
}
impl<T: Config> ::frame_support::dispatch::ModuleErrorMetadata for Module<T> {
    fn metadata() -> &'static [::frame_support::dispatch::ErrorMetadata] {
        <&'static str as ::frame_support::dispatch::ModuleErrorMetadata>::metadata()
    }
}
