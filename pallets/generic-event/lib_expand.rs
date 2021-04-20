#![feature(prelude_import)]
//! Demonstration of Event variants that use type(s) from the pallet's configuration trait
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use frame_support::{decl_event, decl_module, dispatch::DispatchResult};
use frame_system::ensure_signed;
pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}
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
                        "generic_event",
                        ::tracing::Level::TRACE,
                        Some("pallets/generic-event/src/lib.rs"),
                        Some(15u32),
                        Some("generic_event"),
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
impl<T: Config> Module<T> {
    /// Deposits an event using `frame_system::Module::deposit_event`.
    fn deposit_event(event: impl Into<<T as Config>::Event>) {
        <frame_system::Module<T>>::deposit_event(event.into())
    }
}
#[cfg(feature = "std")]
impl<T: Config> ::frame_support::traits::IntegrityTest for Module<T> {}
/// Can also be called using [`Call`].
///
/// [`Call`]: enum.Call.html
impl<T: Config> Module<T> {
    /// A simple call that does little more than emit an event
    ///
    /// NOTE: Calling this function will bypass origin filters.
    fn do_something(origin: T::Origin, input: u32) -> DispatchResult {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "do_something",
                        "generic_event",
                        ::tracing::Level::TRACE,
                        Some("pallets/generic-event/src/lib.rs"),
                        Some(15u32),
                        Some("generic_event"),
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
        let user = ensure_signed(origin)?;
        let new_number = input;
        Self::deposit_event(RawEvent::EmitInput(user, new_number));
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
    /// A simple call that does little more than emit an event
    do_something(u32),
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
                Call::do_something(ref aa) => {
                    __codec_dest_edqy.push_byte(0usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
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
                __codec_x_edqy if __codec_x_edqy == 0usize as u8 => Ok(Call::<T>::do_something({
                    let __codec_res_edqy =
                        <u32 as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                    match __codec_res_edqy {
                        Err(e) => return Err(e.chain("Could not decode `Call::do_something.0`")),
                        Ok(__codec_res_edqy) => __codec_res_edqy,
                    }
                })),
                _ => Err("Could not decode `Call`, variant doesn\'t exist".into()),
            }
        }
    }
};
impl<T: Config> ::frame_support::dispatch::GetDispatchInfo for Call<T> {
    fn get_dispatch_info(&self) -> ::frame_support::dispatch::DispatchInfo {
        match *self {
            Call::do_something(ref input) => {
                let base_weight = 10_000;
                let weight = <dyn ::frame_support::dispatch::WeighData<(&u32,)>>::weigh_data(
                    &base_weight,
                    (input,),
                );
                let class =
                    <dyn ::frame_support::dispatch::ClassifyDispatch<(&u32,)>>::classify_dispatch(
                        &base_weight,
                        (input,),
                    );
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(&u32,)>>::pays_fee(
                    &base_weight,
                    (input,),
                );
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
            Call::do_something(ref input) => {
                let _ = (input);
                "do_something"
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
        &["do_something"]
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
            Call::do_something(ref input) => Call::do_something((*input).clone()),
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Config> ::frame_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::do_something(ref input) => {
                let self_params = (input,);
                if let Call::do_something(ref input) = *_other {
                    self_params == (input,)
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
            Call::do_something(ref input) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"do_something", &(input.clone(),)) {
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
            Call::do_something(input) => <Module<T>>::do_something(_origin, input)
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
            name: ::frame_support::dispatch::DecodeDifferent::Encode("do_something"),
            arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                ::frame_support::dispatch::FunctionArgumentMetadata {
                    name: ::frame_support::dispatch::DecodeDifferent::Encode("input"),
                    ty: ::frame_support::dispatch::DecodeDifferent::Encode("u32"),
                },
            ]),
            documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                r" A simple call that does little more than emit an event",
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
/// [`RawEvent`] specialized for the configuration [`Config`]
///
/// [`RawEvent`]: enum.RawEvent.html
/// [`Config`]: trait.Config.html
pub type Event<T> = RawEvent<<T as frame_system::Config>::AccountId>;
/// Events for this module.
///
pub enum RawEvent<AccountId> {
    /// Some input was sent
    EmitInput(AccountId, u32),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::clone::Clone> ::core::clone::Clone for RawEvent<AccountId> {
    #[inline]
    fn clone(&self) -> RawEvent<AccountId> {
        match (&*self,) {
            (&RawEvent::EmitInput(ref __self_0, ref __self_1),) => RawEvent::EmitInput(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
            ),
        }
    }
}
impl<AccountId> ::core::marker::StructuralPartialEq for RawEvent<AccountId> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::cmp::PartialEq> ::core::cmp::PartialEq for RawEvent<AccountId> {
    #[inline]
    fn eq(&self, other: &RawEvent<AccountId>) -> bool {
        match (&*self, &*other) {
            (
                &RawEvent::EmitInput(ref __self_0, ref __self_1),
                &RawEvent::EmitInput(ref __arg_1_0, ref __arg_1_1),
            ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<AccountId>) -> bool {
        match (&*self, &*other) {
            (
                &RawEvent::EmitInput(ref __self_0, ref __self_1),
                &RawEvent::EmitInput(ref __arg_1_0, ref __arg_1_1),
            ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
        }
    }
}
impl<AccountId> ::core::marker::StructuralEq for RawEvent<AccountId> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::cmp::Eq> ::core::cmp::Eq for RawEvent<AccountId> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_scale_codec as _parity_scale_codec;
    impl<AccountId> _parity_scale_codec::Encode for RawEvent<AccountId>
    where
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
    {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                RawEvent::EmitInput(ref aa, ref ba) => {
                    __codec_dest_edqy.push_byte(0usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    impl<AccountId> _parity_scale_codec::EncodeLike for RawEvent<AccountId>
    where
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_scale_codec as _parity_scale_codec;
    impl<AccountId> _parity_scale_codec::Decode for RawEvent<AccountId>
    where
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
    {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| e.chain("Could not decode `RawEvent`, failed to read variant byte"))?
            {
                __codec_x_edqy if __codec_x_edqy == 0usize as u8 => {
                    Ok(RawEvent::<AccountId>::EmitInput(
                        {
                            let __codec_res_edqy =
                                <AccountId as _parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                            match __codec_res_edqy {
                                Err(e) => {
                                    return Err(e.chain("Could not decode `RawEvent::EmitInput.0`"))
                                }
                                Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <u32 as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                Err(e) => {
                                    return Err(e.chain("Could not decode `RawEvent::EmitInput.1`"))
                                }
                                Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    ))
                }
                _ => Err("Could not decode `RawEvent`, variant doesn\'t exist".into()),
            }
        }
    }
};
impl<AccountId> core::fmt::Debug for RawEvent<AccountId>
where
    AccountId: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::EmitInput(ref a0, ref a1) => fmt
                .debug_tuple("RawEvent::EmitInput")
                .field(a0)
                .field(a1)
                .finish(),
            _ => Ok(()),
        }
    }
}
impl<AccountId> From<RawEvent<AccountId>> for () {
    fn from(_: RawEvent<AccountId>) -> () {
        ()
    }
}
impl<AccountId> RawEvent<AccountId> {
    #[allow(dead_code)]
    #[doc(hidden)]
    pub fn metadata() -> &'static [::frame_support::event::EventMetadata] {
        &[::frame_support::event::EventMetadata {
            name: ::frame_support::event::DecodeDifferent::Encode("EmitInput"),
            arguments: ::frame_support::event::DecodeDifferent::Encode(&["AccountId", "u32"]),
            documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                r" Some input was sent",
            ]),
        }]
    }
}
