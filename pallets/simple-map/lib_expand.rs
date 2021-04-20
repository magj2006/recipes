#![feature(prelude_import)]
//! A pallet to demonstrate usage of a simple storage map
//!
//! Storage maps map a key type to a value type. The hasher used to hash the key can be customized.
//! This pallet uses the `blake2_128_concat` hasher. This is a good default hasher.
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
};
use frame_system::ensure_signed;
pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}
use self::sp_api_hidden_includes_decl_storage::hidden_include::{
    StorageValue as _, StorageMap as _, StorageDoubleMap as _, StoragePrefixedMap as _,
    IterableStorageMap as _, IterableStorageDoubleMap as _,
};
#[doc(hidden)]
mod sp_api_hidden_includes_decl_storage {
    pub extern crate frame_support as hidden_include;
}
trait Store {
    type SimpleMap;
}
impl<T: Config + 'static> Store for Module<T> {
    type SimpleMap = SimpleMap<T>;
}
impl<T: Config + 'static> Module<T> {
    pub fn simple_map<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<T::AccountId>,
    >(
        key: K,
    ) -> u32 {
        < SimpleMap < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < T :: AccountId , u32 > > :: get (key)
    }
}
#[doc(hidden)]
pub struct __GetByteStructSimpleMap<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_SimpleMap:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Config> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructSimpleMap<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_SimpleMap
            .get_or_init(|| {
                let def_val: u32 = Default::default();
                <u32 as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Config> Send for __GetByteStructSimpleMap<T> {}
unsafe impl<T: Config> Sync for __GetByteStructSimpleMap<T> {}
impl<T: Config + 'static> Module<T> {
    #[doc(hidden)]
    pub fn storage_metadata(
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageMetadata { prefix : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("SimpleMap") , entries : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("SimpleMap") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::AccountId") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("u32") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructSimpleMap :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& []) , }] [..]) , }
    }
}
/// Hidden instance generated to be internally used when module is used without
/// instance.
#[doc(hidden)]
pub struct __InherentHiddenInstance;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for __InherentHiddenInstance {
    #[inline]
    fn clone(&self) -> __InherentHiddenInstance {
        match *self {
            __InherentHiddenInstance => __InherentHiddenInstance,
        }
    }
}
impl ::core::marker::StructuralEq for __InherentHiddenInstance {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for __InherentHiddenInstance {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
impl ::core::marker::StructuralPartialEq for __InherentHiddenInstance {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for __InherentHiddenInstance {
    #[inline]
    fn eq(&self, other: &__InherentHiddenInstance) -> bool {
        match *other {
            __InherentHiddenInstance => match *self {
                __InherentHiddenInstance => true,
            },
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_scale_codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for __InherentHiddenInstance {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
        }
    }
    impl _parity_scale_codec::EncodeLike for __InherentHiddenInstance {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_scale_codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for __InherentHiddenInstance {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            Ok(__InherentHiddenInstance)
        }
    }
};
impl core::fmt::Debug for __InherentHiddenInstance {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("__InherentHiddenInstance").finish()
    }
}
impl self::sp_api_hidden_includes_decl_storage::hidden_include::traits::Instance
    for __InherentHiddenInstance
{
    const PREFIX: &'static str = "SimpleMap";
}
struct SimpleMap<T: Config>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Config>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<u32>
    for SimpleMap<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"SimpleMap"
    }
}
impl<T: Config>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::AccountId,
        u32,
    > for SimpleMap<T>
{
    type Query = u32;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"SimpleMap"
    }
    fn from_optional_value_to_query(v: Option<u32>) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<u32> {
        Some(v)
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
    /// A user has set their enrty
    EntrySet(AccountId, u32),
    /// A user has read their entry, leaving it in storage
    EntryGot(AccountId, u32),
    /// A user has read their entry removing it fro mstorage
    EntryTaken(AccountId, u32),
    /// A user has read their entry, incremented it, and writtenthe new entry to storage
    /// Parameters are (user, old_entry, new_entry)
    EntryIncreased(AccountId, u32, u32),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::clone::Clone> ::core::clone::Clone for RawEvent<AccountId> {
    #[inline]
    fn clone(&self) -> RawEvent<AccountId> {
        match (&*self,) {
            (&RawEvent::EntrySet(ref __self_0, ref __self_1),) => RawEvent::EntrySet(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
            ),
            (&RawEvent::EntryGot(ref __self_0, ref __self_1),) => RawEvent::EntryGot(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
            ),
            (&RawEvent::EntryTaken(ref __self_0, ref __self_1),) => RawEvent::EntryTaken(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
            ),
            (&RawEvent::EntryIncreased(ref __self_0, ref __self_1, ref __self_2),) => {
                RawEvent::EntryIncreased(
                    ::core::clone::Clone::clone(&(*__self_0)),
                    ::core::clone::Clone::clone(&(*__self_1)),
                    ::core::clone::Clone::clone(&(*__self_2)),
                )
            }
        }
    }
}
impl<AccountId> ::core::marker::StructuralPartialEq for RawEvent<AccountId> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<AccountId: ::core::cmp::PartialEq> ::core::cmp::PartialEq for RawEvent<AccountId> {
    #[inline]
    fn eq(&self, other: &RawEvent<AccountId>) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &RawEvent::EntrySet(ref __self_0, ref __self_1),
                        &RawEvent::EntrySet(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                    (
                        &RawEvent::EntryGot(ref __self_0, ref __self_1),
                        &RawEvent::EntryGot(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                    (
                        &RawEvent::EntryTaken(ref __self_0, ref __self_1),
                        &RawEvent::EntryTaken(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                    (
                        &RawEvent::EntryIncreased(ref __self_0, ref __self_1, ref __self_2),
                        &RawEvent::EntryIncreased(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<AccountId>) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &RawEvent::EntrySet(ref __self_0, ref __self_1),
                        &RawEvent::EntrySet(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                    (
                        &RawEvent::EntryGot(ref __self_0, ref __self_1),
                        &RawEvent::EntryGot(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                    (
                        &RawEvent::EntryTaken(ref __self_0, ref __self_1),
                        &RawEvent::EntryTaken(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                    (
                        &RawEvent::EntryIncreased(ref __self_0, ref __self_1, ref __self_2),
                        &RawEvent::EntryIncreased(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
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
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<u32>;
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
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
    {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                RawEvent::EntrySet(ref aa, ref ba) => {
                    __codec_dest_edqy.push_byte(0usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                }
                RawEvent::EntryGot(ref aa, ref ba) => {
                    __codec_dest_edqy.push_byte(1usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                }
                RawEvent::EntryTaken(ref aa, ref ba) => {
                    __codec_dest_edqy.push_byte(2usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                }
                RawEvent::EntryIncreased(ref aa, ref ba, ref ca) => {
                    __codec_dest_edqy.push_byte(3usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                    _parity_scale_codec::Encode::encode_to(ba, __codec_dest_edqy);
                    _parity_scale_codec::Encode::encode_to(ca, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    impl<AccountId> _parity_scale_codec::EncodeLike for RawEvent<AccountId>
    where
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
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
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
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
                    Ok(RawEvent::<AccountId>::EntrySet(
                        {
                            let __codec_res_edqy =
                                <AccountId as _parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                            match __codec_res_edqy {
                                Err(e) => {
                                    return Err(e.chain("Could not decode `RawEvent::EntrySet.0`"))
                                }
                                Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <u32 as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                Err(e) => {
                                    return Err(e.chain("Could not decode `RawEvent::EntrySet.1`"))
                                }
                                Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    ))
                }
                __codec_x_edqy if __codec_x_edqy == 1usize as u8 => {
                    Ok(RawEvent::<AccountId>::EntryGot(
                        {
                            let __codec_res_edqy =
                                <AccountId as _parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                            match __codec_res_edqy {
                                Err(e) => {
                                    return Err(e.chain("Could not decode `RawEvent::EntryGot.0`"))
                                }
                                Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <u32 as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                Err(e) => {
                                    return Err(e.chain("Could not decode `RawEvent::EntryGot.1`"))
                                }
                                Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    ))
                }
                __codec_x_edqy if __codec_x_edqy == 2usize as u8 => {
                    Ok(RawEvent::<AccountId>::EntryTaken(
                        {
                            let __codec_res_edqy =
                                <AccountId as _parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                            match __codec_res_edqy {
                                Err(e) => {
                                    return Err(e.chain("Could not decode `RawEvent::EntryTaken.0`"))
                                }
                                Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <u32 as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                Err(e) => {
                                    return Err(e.chain("Could not decode `RawEvent::EntryTaken.1`"))
                                }
                                Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                    ))
                }
                __codec_x_edqy if __codec_x_edqy == 3usize as u8 => {
                    Ok(RawEvent::<AccountId>::EntryIncreased(
                        {
                            let __codec_res_edqy =
                                <AccountId as _parity_scale_codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                            match __codec_res_edqy {
                                Err(e) => {
                                    return Err(
                                        e.chain("Could not decode `RawEvent::EntryIncreased.0`")
                                    )
                                }
                                Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <u32 as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                Err(e) => {
                                    return Err(
                                        e.chain("Could not decode `RawEvent::EntryIncreased.1`")
                                    )
                                }
                                Ok(__codec_res_edqy) => __codec_res_edqy,
                            }
                        },
                        {
                            let __codec_res_edqy =
                                <u32 as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                            match __codec_res_edqy {
                                Err(e) => {
                                    return Err(
                                        e.chain("Could not decode `RawEvent::EntryIncreased.2`")
                                    )
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
            Self::EntrySet(ref a0, ref a1) => fmt
                .debug_tuple("RawEvent::EntrySet")
                .field(a0)
                .field(a1)
                .finish(),
            Self::EntryGot(ref a0, ref a1) => fmt
                .debug_tuple("RawEvent::EntryGot")
                .field(a0)
                .field(a1)
                .finish(),
            Self::EntryTaken(ref a0, ref a1) => fmt
                .debug_tuple("RawEvent::EntryTaken")
                .field(a0)
                .field(a1)
                .finish(),
            Self::EntryIncreased(ref a0, ref a1, ref a2) => fmt
                .debug_tuple("RawEvent::EntryIncreased")
                .field(a0)
                .field(a1)
                .field(a2)
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
        &[
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("EntrySet"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&["AccountId", "u32"]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" A user has set their enrty",
                ]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("EntryGot"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&["AccountId", "u32"]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" A user has read their entry, leaving it in storage",
                ]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("EntryTaken"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&["AccountId", "u32"]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" A user has read their entry removing it fro mstorage",
                ]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("EntryIncreased"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "u32",
                    "u32",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[
                    r" A user has read their entry, incremented it, and writtenthe new entry to storage",
                    r" Parameters are (user, old_entry, new_entry)",
                ]),
            },
        ]
    }
}
pub enum Error<T: Config> {
    #[doc(hidden)]
    __Ignore(
        ::frame_support::sp_std::marker::PhantomData<(T,)>,
        ::frame_support::Never,
    ),
    /// The requested user has not stored a value yet
    NoValueStored,
    /// The value cannot be incremented further because it has reached the maimum allowed value
    MaxValueReached,
}
impl<T: Config> ::frame_support::sp_std::fmt::Debug for Error<T> {
    fn fmt(
        &self,
        f: &mut ::frame_support::sp_std::fmt::Formatter<'_>,
    ) -> ::frame_support::sp_std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl<T: Config> Error<T> {
    fn as_u8(&self) -> u8 {
        match self {
            Error::__Ignore(_, _) => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["internal error: entered unreachable code: "],
                &match (&"`__Ignore` can never be constructed",) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            )),
            Error::NoValueStored => 0,
            Error::MaxValueReached => 0 + 1,
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            Self::__Ignore(_, _) => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["internal error: entered unreachable code: "],
                &match (&"`__Ignore` can never be constructed",) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            )),
            Error::NoValueStored => "NoValueStored",
            Error::MaxValueReached => "MaxValueReached",
        }
    }
}
impl<T: Config> From<Error<T>> for &'static str {
    fn from(err: Error<T>) -> &'static str {
        err.as_str()
    }
}
impl<T: Config> From<Error<T>> for ::frame_support::sp_runtime::DispatchError {
    fn from(err: Error<T>) -> Self {
        let index = <T::PalletInfo as ::frame_support::traits::PalletInfo>::index::<Module<T>>()
            .expect("Every active module has an index in the runtime; qed")
            as u8;
        ::frame_support::sp_runtime::DispatchError::Module {
            index,
            error: err.as_u8(),
            message: Some(err.as_str()),
        }
    }
}
impl<T: Config> ::frame_support::error::ModuleErrorMetadata for Error<T> {
    fn metadata() -> &'static [::frame_support::error::ErrorMetadata] {
        &[
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("NoValueStored"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" The requested user has not stored a value yet",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("MaxValueReached"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" The value cannot be incremented further because it has reached the maimum allowed value",
                ]),
            },
        ]
    }
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
                        "simple_map",
                        ::tracing::Level::TRACE,
                        Some("pallets/simple-map/src/lib.rs"),
                        Some(56u32),
                        Some("simple_map"),
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
    /// Set the value stored at a particular key
    ///
    /// NOTE: Calling this function will bypass origin filters.
    fn set_single_entry(origin: T::Origin, entry: u32) -> DispatchResult {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "set_single_entry",
                        "simple_map",
                        ::tracing::Level::TRACE,
                        Some("pallets/simple-map/src/lib.rs"),
                        Some(56u32),
                        Some("simple_map"),
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
        <SimpleMap<T>>::insert(&user, entry);
        Self::deposit_event(RawEvent::EntrySet(user, entry));
        Ok(())
    }
    /// Read the value stored at a particular key and emit it in an event
    ///
    /// NOTE: Calling this function will bypass origin filters.
    fn get_single_entry(origin: T::Origin, account: T::AccountId) -> DispatchResult {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "get_single_entry",
                        "simple_map",
                        ::tracing::Level::TRACE,
                        Some("pallets/simple-map/src/lib.rs"),
                        Some(56u32),
                        Some("simple_map"),
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
        let getter = ensure_signed(origin)?;
        {
            if !<SimpleMap<T>>::contains_key(&account) {
                {
                    return Err(Error::<T>::NoValueStored.into());
                };
            }
        };
        let entry = <SimpleMap<T>>::get(account);
        Self::deposit_event(RawEvent::EntryGot(getter, entry));
        Ok(())
    }
    /// Read the value stored at a particular key,while removing it from the map.
    /// Also emit the read value in an event
    ///
    /// NOTE: Calling this function will bypass origin filters.
    fn take_single_entry(origin: T::Origin) -> DispatchResult {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "take_single_entry",
                        "simple_map",
                        ::tracing::Level::TRACE,
                        Some("pallets/simple-map/src/lib.rs"),
                        Some(56u32),
                        Some("simple_map"),
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
        {
            if !<SimpleMap<T>>::contains_key(&user) {
                {
                    return Err(Error::<T>::NoValueStored.into());
                };
            }
        };
        let entry = <SimpleMap<T>>::take(&user);
        Self::deposit_event(RawEvent::EntryTaken(user, entry));
        Ok(())
    }
    /// Increase the value associated with a particular key
    ///
    /// NOTE: Calling this function will bypass origin filters.
    fn increase_single_entry(origin: T::Origin, add_this_val: u32) -> DispatchResult {
        let __within_span__ = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                use ::tracing::__macro_support::MacroCallsite;
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "increase_single_entry",
                        "simple_map",
                        ::tracing::Level::TRACE,
                        Some("pallets/simple-map/src/lib.rs"),
                        Some(56u32),
                        Some("simple_map"),
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
        {
            if !<SimpleMap<T>>::contains_key(&user) {
                {
                    return Err(Error::<T>::NoValueStored.into());
                };
            }
        };
        let original_value = <SimpleMap<T>>::get(&user);
        let new_value = original_value
            .checked_add(add_this_val)
            .ok_or(Error::<T>::MaxValueReached)?;
        <SimpleMap<T>>::insert(&user, new_value);
        Self::deposit_event(RawEvent::EntryIncreased(user, original_value, new_value));
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
    /// Set the value stored at a particular key
    set_single_entry(u32),
    #[allow(non_camel_case_types)]
    /// Read the value stored at a particular key and emit it in an event
    get_single_entry(T::AccountId),
    #[allow(non_camel_case_types)]
    /// Read the value stored at a particular key,while removing it from the map.
    /// Also emit the read value in an event
    take_single_entry(),
    #[allow(non_camel_case_types)]
    /// Increase the value associated with a particular key
    increase_single_entry(u32),
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_scale_codec as _parity_scale_codec;
    impl<T: Config> _parity_scale_codec::Encode for Call<T>
    where
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
    {
        fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
            &self,
            __codec_dest_edqy: &mut __CodecOutputEdqy,
        ) {
            match *self {
                Call::set_single_entry(ref aa) => {
                    __codec_dest_edqy.push_byte(0usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::get_single_entry(ref aa) => {
                    __codec_dest_edqy.push_byte(1usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                Call::take_single_entry() => {
                    __codec_dest_edqy.push_byte(2usize as u8);
                }
                Call::increase_single_entry(ref aa) => {
                    __codec_dest_edqy.push_byte(3usize as u8);
                    _parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                }
                _ => (),
            }
        }
    }
    impl<T: Config> _parity_scale_codec::EncodeLike for Call<T>
    where
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_scale_codec as _parity_scale_codec;
    impl<T: Config> _parity_scale_codec::Decode for Call<T>
    where
        T::AccountId: _parity_scale_codec::Decode,
        T::AccountId: _parity_scale_codec::Decode,
    {
        fn decode<__CodecInputEdqy: _parity_scale_codec::Input>(
            __codec_input_edqy: &mut __CodecInputEdqy,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match __codec_input_edqy
                .read_byte()
                .map_err(|e| e.chain("Could not decode `Call`, failed to read variant byte"))?
            {
                __codec_x_edqy if __codec_x_edqy == 0usize as u8 => {
                    Ok(Call::<T>::set_single_entry({
                        let __codec_res_edqy =
                            <u32 as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(e) => {
                                return Err(e.chain("Could not decode `Call::set_single_entry.0`"))
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 1usize as u8 => {
                    Ok(Call::<T>::get_single_entry({
                        let __codec_res_edqy =
                            <T::AccountId as _parity_scale_codec::Decode>::decode(
                                __codec_input_edqy,
                            );
                        match __codec_res_edqy {
                            Err(e) => {
                                return Err(e.chain("Could not decode `Call::get_single_entry.0`"))
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                __codec_x_edqy if __codec_x_edqy == 2usize as u8 => {
                    Ok(Call::<T>::take_single_entry())
                }
                __codec_x_edqy if __codec_x_edqy == 3usize as u8 => {
                    Ok(Call::<T>::increase_single_entry({
                        let __codec_res_edqy =
                            <u32 as _parity_scale_codec::Decode>::decode(__codec_input_edqy);
                        match __codec_res_edqy {
                            Err(e) => {
                                return Err(
                                    e.chain("Could not decode `Call::increase_single_entry.0`")
                                )
                            }
                            Ok(__codec_res_edqy) => __codec_res_edqy,
                        }
                    }))
                }
                _ => Err("Could not decode `Call`, variant doesn\'t exist".into()),
            }
        }
    }
};
impl<T: Config> ::frame_support::dispatch::GetDispatchInfo for Call<T> {
    fn get_dispatch_info(&self) -> ::frame_support::dispatch::DispatchInfo {
        match *self {
            Call::set_single_entry(ref entry) => {
                let base_weight = 10_000;
                let weight = <dyn ::frame_support::dispatch::WeighData<(&u32,)>>::weigh_data(
                    &base_weight,
                    (entry,),
                );
                let class =
                    <dyn ::frame_support::dispatch::ClassifyDispatch<(&u32,)>>::classify_dispatch(
                        &base_weight,
                        (entry,),
                    );
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(&u32,)>>::pays_fee(
                    &base_weight,
                    (entry,),
                );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::get_single_entry(ref account) => {
                let base_weight = 10_000;
                let weight =
                    <dyn ::frame_support::dispatch::WeighData<(&T::AccountId,)>>::weigh_data(
                        &base_weight,
                        (account,),
                    );
                let class = < dyn :: frame_support :: dispatch :: ClassifyDispatch < (& T :: AccountId ,) > > :: classify_dispatch (& base_weight , (account ,)) ;
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(&T::AccountId,)>>::pays_fee(
                    &base_weight,
                    (account,),
                );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::take_single_entry() => {
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
            Call::increase_single_entry(ref add_this_val) => {
                let base_weight = 10_000;
                let weight = <dyn ::frame_support::dispatch::WeighData<(&u32,)>>::weigh_data(
                    &base_weight,
                    (add_this_val,),
                );
                let class =
                    <dyn ::frame_support::dispatch::ClassifyDispatch<(&u32,)>>::classify_dispatch(
                        &base_weight,
                        (add_this_val,),
                    );
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(&u32,)>>::pays_fee(
                    &base_weight,
                    (add_this_val,),
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
            Call::set_single_entry(ref entry) => {
                let _ = (entry);
                "set_single_entry"
            }
            Call::get_single_entry(ref account) => {
                let _ = (account);
                "get_single_entry"
            }
            Call::take_single_entry() => {
                let _ = ();
                "take_single_entry"
            }
            Call::increase_single_entry(ref add_this_val) => {
                let _ = (add_this_val);
                "increase_single_entry"
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
        &[
            "set_single_entry",
            "get_single_entry",
            "take_single_entry",
            "increase_single_entry",
        ]
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
            Call::set_single_entry(ref entry) => Call::set_single_entry((*entry).clone()),
            Call::get_single_entry(ref account) => Call::get_single_entry((*account).clone()),
            Call::take_single_entry() => Call::take_single_entry(),
            Call::increase_single_entry(ref add_this_val) => {
                Call::increase_single_entry((*add_this_val).clone())
            }
            _ => ::core::panicking::panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Config> ::frame_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::set_single_entry(ref entry) => {
                let self_params = (entry,);
                if let Call::set_single_entry(ref entry) = *_other {
                    self_params == (entry,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::core::panicking::panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::get_single_entry(ref account) => {
                let self_params = (account,);
                if let Call::get_single_entry(ref account) = *_other {
                    self_params == (account,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::core::panicking::panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::take_single_entry() => {
                let self_params = ();
                if let Call::take_single_entry() = *_other {
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
            Call::increase_single_entry(ref add_this_val) => {
                let self_params = (add_this_val,);
                if let Call::increase_single_entry(ref add_this_val) = *_other {
                    self_params == (add_this_val,)
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
            Call::set_single_entry(ref entry) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"set_single_entry", &(entry.clone(),)) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            Call::get_single_entry(ref account) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"get_single_entry", &(account.clone(),)) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            Call::take_single_entry() => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (&"take_single_entry", &()) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            Call::increase_single_entry(ref add_this_val) => {
                _f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"increase_single_entry", &(add_this_val.clone(),)) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                        ],
                    },
                ))
            }
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
            Call::set_single_entry(entry) => <Module<T>>::set_single_entry(_origin, entry)
                .map(Into::into)
                .map_err(Into::into),
            Call::get_single_entry(account) => <Module<T>>::get_single_entry(_origin, account)
                .map(Into::into)
                .map_err(Into::into),
            Call::take_single_entry() => <Module<T>>::take_single_entry(_origin)
                .map(Into::into)
                .map_err(Into::into),
            Call::increase_single_entry(add_this_val) => {
                <Module<T>>::increase_single_entry(_origin, add_this_val)
                    .map(Into::into)
                    .map_err(Into::into)
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
impl<T: Config> ::frame_support::dispatch::Callable<T> for Module<T> {
    type Call = Call<T>;
}
impl<T: Config> Module<T> {
    #[doc(hidden)]
    #[allow(dead_code)]
    pub fn call_functions() -> &'static [::frame_support::dispatch::FunctionMetadata] {
        &[
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("set_single_entry"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("entry"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("u32"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" Set the value stored at a particular key",
                ]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("get_single_entry"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("account"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("T::AccountId"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" Read the value stored at a particular key and emit it in an event",
                ]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("take_single_entry"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" Read the value stored at a particular key,while removing it from the map.",
                    r" Also emit the read value in an event",
                ]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("increase_single_entry"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("add_this_val"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("u32"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    r" Increase the value associated with a particular key",
                ]),
            },
        ]
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
        <Error<T> as ::frame_support::dispatch::ModuleErrorMetadata>::metadata()
    }
}
