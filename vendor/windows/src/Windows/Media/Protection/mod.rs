#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Media_Protection_PlayReady")]
pub mod PlayReady;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ComponentLoadFailedEventArgs(::windows::runtime::IInspectable);
impl ComponentLoadFailedEventArgs {
    pub fn Information(&self) -> ::windows::runtime::Result<RevocationAndRenewalInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RevocationAndRenewalInformation>(result__)
        }
    }
    pub fn Completion(&self) -> ::windows::runtime::Result<MediaProtectionServiceCompletion> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaProtectionServiceCompletion>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ComponentLoadFailedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.ComponentLoadFailedEventArgs;{95972e93-7746-417e-8495-f031bbc5862c})");
}
unsafe impl ::windows::runtime::Interface for ComponentLoadFailedEventArgs {
    type Vtable = IComponentLoadFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2509713043, 30534, 16766, [132, 149, 240, 49, 187, 197, 134, 44]);
}
impl ::windows::runtime::RuntimeName for ComponentLoadFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.ComponentLoadFailedEventArgs";
}
impl ::std::convert::From<ComponentLoadFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ComponentLoadFailedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ComponentLoadFailedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ComponentLoadFailedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ComponentLoadFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ComponentLoadFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ComponentLoadFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ComponentLoadFailedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ComponentLoadFailedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ComponentLoadFailedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ComponentLoadFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ComponentLoadFailedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ComponentLoadFailedEventArgs {}
unsafe impl ::std::marker::Sync for ComponentLoadFailedEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ComponentLoadFailedEventHandler(::windows::runtime::IUnknown);
impl ComponentLoadFailedEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<MediaProtectionManager>, &::std::option::Option<ComponentLoadFailedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = ComponentLoadFailedEventHandler_box::<F> {
            vtable: &ComponentLoadFailedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, MediaProtectionManager>, Param1: ::windows::runtime::IntoParam<'a, ComponentLoadFailedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ComponentLoadFailedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({95da643c-6db9-424b-86ca-091af432081c})");
}
unsafe impl ::windows::runtime::Interface for ComponentLoadFailedEventHandler {
    type Vtable = ComponentLoadFailedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2514117692, 28089, 16971, [134, 202, 9, 26, 244, 50, 8, 28]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ComponentLoadFailedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct ComponentLoadFailedEventHandler_box<F: FnMut(&::std::option::Option<MediaProtectionManager>, &::std::option::Option<ComponentLoadFailedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const ComponentLoadFailedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<MediaProtectionManager>, &::std::option::Option<ComponentLoadFailedEventArgs>) -> ::windows::runtime::Result<()> + 'static> ComponentLoadFailedEventHandler_box<F> {
    const VTABLE: ComponentLoadFailedEventHandler_abi = ComponentLoadFailedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<ComponentLoadFailedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <MediaProtectionManager as ::windows::runtime::Abi>::Abi as *const <MediaProtectionManager as ::windows::runtime::Abi>::DefaultType),
            &*(&e as *const <ComponentLoadFailedEventArgs as ::windows::runtime::Abi>::Abi as *const <ComponentLoadFailedEventArgs as ::windows::runtime::Abi>::DefaultType),
        )
        .into()
    }
}
pub struct ComponentRenewal {}
impl ComponentRenewal {
    #[cfg(feature = "Foundation")]
    pub fn RenewSystemComponentsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, RevocationAndRenewalInformation>>(information: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<RenewalStatus, u32>> {
        Self::IComponentRenewalStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), information.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<RenewalStatus, u32>>(result__)
        })
    }
    pub fn IComponentRenewalStatics<R, F: FnOnce(&IComponentRenewalStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ComponentRenewal, IComponentRenewalStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ComponentRenewal {
    const NAME: &'static str = "Windows.Media.Protection.ComponentRenewal";
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GraphicsTrustStatus(pub i32);
impl GraphicsTrustStatus {
    pub const TrustNotRequired: GraphicsTrustStatus = GraphicsTrustStatus(0i32);
    pub const TrustEstablished: GraphicsTrustStatus = GraphicsTrustStatus(1i32);
    pub const EnvironmentNotSupported: GraphicsTrustStatus = GraphicsTrustStatus(2i32);
    pub const DriverNotSupported: GraphicsTrustStatus = GraphicsTrustStatus(3i32);
    pub const DriverSigningFailure: GraphicsTrustStatus = GraphicsTrustStatus(4i32);
    pub const UnknownFailure: GraphicsTrustStatus = GraphicsTrustStatus(5i32);
}
impl ::std::convert::From<i32> for GraphicsTrustStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GraphicsTrustStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GraphicsTrustStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.GraphicsTrustStatus;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HdcpProtection(pub i32);
impl HdcpProtection {
    pub const Off: HdcpProtection = HdcpProtection(0i32);
    pub const On: HdcpProtection = HdcpProtection(1i32);
    pub const OnWithTypeEnforcement: HdcpProtection = HdcpProtection(2i32);
}
impl ::std::convert::From<i32> for HdcpProtection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HdcpProtection {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HdcpProtection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.HdcpProtection;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HdcpSession(::windows::runtime::IInspectable);
impl HdcpSession {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HdcpSession, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn IsEffectiveProtectionAtLeast(&self, protection: HdcpProtection) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), protection, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetEffectiveProtection(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<HdcpProtection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<HdcpProtection>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDesiredMinProtectionAsync(&self, protection: HdcpProtection) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<HdcpSetProtectionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), protection, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HdcpSetProtectionResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ProtectionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<HdcpSession, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveProtectionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HdcpSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.HdcpSession;{718845e9-64d7-426d-809b-1be461941a2a})");
}
unsafe impl ::windows::runtime::Interface for HdcpSession {
    type Vtable = IHdcpSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1904756201, 25815, 17005, [128, 155, 27, 228, 97, 148, 26, 42]);
}
impl ::windows::runtime::RuntimeName for HdcpSession {
    const NAME: &'static str = "Windows.Media.Protection.HdcpSession";
}
impl ::std::convert::From<HdcpSession> for ::windows::runtime::IUnknown {
    fn from(value: HdcpSession) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HdcpSession> for ::windows::runtime::IUnknown {
    fn from(value: &HdcpSession) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HdcpSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HdcpSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HdcpSession> for ::windows::runtime::IInspectable {
    fn from(value: HdcpSession) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HdcpSession> for ::windows::runtime::IInspectable {
    fn from(value: &HdcpSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HdcpSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HdcpSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HdcpSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HdcpSession) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HdcpSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HdcpSession) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for HdcpSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &HdcpSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HdcpSession {}
unsafe impl ::std::marker::Sync for HdcpSession {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HdcpSetProtectionResult(pub i32);
impl HdcpSetProtectionResult {
    pub const Success: HdcpSetProtectionResult = HdcpSetProtectionResult(0i32);
    pub const TimedOut: HdcpSetProtectionResult = HdcpSetProtectionResult(1i32);
    pub const NotSupported: HdcpSetProtectionResult = HdcpSetProtectionResult(2i32);
    pub const UnknownFailure: HdcpSetProtectionResult = HdcpSetProtectionResult(3i32);
}
impl ::std::convert::From<i32> for HdcpSetProtectionResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HdcpSetProtectionResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HdcpSetProtectionResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.HdcpSetProtectionResult;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IComponentLoadFailedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IComponentLoadFailedEventArgs {
    type Vtable = IComponentLoadFailedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2509713043, 30534, 16766, [132, 149, 240, 49, 187, 197, 134, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentLoadFailedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IComponentRenewalStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IComponentRenewalStatics {
    type Vtable = IComponentRenewalStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1878773095, 46997, 18629, [139, 123, 167, 196, 239, 226, 2, 227]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentRenewalStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, information: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHdcpSession(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHdcpSession {
    type Vtable = IHdcpSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1904756201, 25815, 17005, [128, 155, 27, 228, 97, 148, 26, 42]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHdcpSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protection: HdcpProtection, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, protection: HdcpProtection, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaProtectionManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaProtectionManager {
    type Vtable = IMediaProtectionManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1164527943, 51009, 17227, [167, 158, 71, 76, 18, 217, 61, 47]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaProtectionPMPServer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaProtectionPMPServer {
    type Vtable = IMediaProtectionPMPServer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(202445350, 31526, 19761, [149, 187, 156, 27, 8, 239, 127, 192]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionPMPServer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaProtectionPMPServerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaProtectionPMPServerFactory {
    type Vtable = IMediaProtectionPMPServerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1613532766, 63442, 18558, [175, 145, 219, 196, 37, 43, 33, 130]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionPMPServerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pproperties: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMediaProtectionServiceCompletion(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaProtectionServiceCompletion {
    type Vtable = IMediaProtectionServiceCompletion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2338114072, 53205, 17646, [162, 237, 223, 118, 1, 12, 20, 181]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionServiceCompletion_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, success: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMediaProtectionServiceRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMediaProtectionServiceRequest {
    type Vtable = IMediaProtectionServiceRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2984119974, 8340, 18317, [135, 164, 139, 149, 32, 15, 133, 198]);
}
impl IMediaProtectionServiceRequest {
    pub fn ProtectionSystem(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IMediaProtectionServiceRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{b1de0ea6-2094-478d-87a4-8b95200f85c6}");
}
impl ::std::convert::From<IMediaProtectionServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: IMediaProtectionServiceRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMediaProtectionServiceRequest> for ::windows::runtime::IUnknown {
    fn from(value: &IMediaProtectionServiceRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMediaProtectionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMediaProtectionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMediaProtectionServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: IMediaProtectionServiceRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMediaProtectionServiceRequest> for ::windows::runtime::IInspectable {
    fn from(value: &IMediaProtectionServiceRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IMediaProtectionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IMediaProtectionServiceRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaProtectionServiceRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IProtectionCapabilities(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IProtectionCapabilities {
    type Vtable = IProtectionCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3349962110, 29824, 19753, [164, 100, 123, 205, 145, 61, 216, 228]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, keysystem: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ProtectionCapabilityResult) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRevocationAndRenewalInformation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRevocationAndRenewalInformation {
    type Vtable = IRevocationAndRenewalInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4087452539, 9473, 17310, [166, 231, 111, 201, 94, 23, 95, 207]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevocationAndRenewalInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRevocationAndRenewalItem(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRevocationAndRenewalItem {
    type Vtable = IRevocationAndRenewalItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(815383052, 15600, 18922, [144, 45, 202, 243, 45, 45, 222, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRevocationAndRenewalItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RevocationAndRenewalReasons) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IServiceRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IServiceRequestedEventArgs {
    type Vtable = IServiceRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(875051951, 43956, 20417, [189, 137, 147, 241, 6, 87, 58, 73]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IServiceRequestedEventArgs2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IServiceRequestedEventArgs2 {
    type Vtable = IServiceRequestedEventArgs2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1430022614, 64254, 16680, [141, 250, 19, 14, 57, 138, 19, 167]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IServiceRequestedEventArgs2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Media_Playback")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Media_Playback"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MediaProtectionManager(::windows::runtime::IInspectable);
impl MediaProtectionManager {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaProtectionManager, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn ServiceRequested<'a, Param0: ::windows::runtime::IntoParam<'a, ServiceRequestedEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveServiceRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RebootNeeded<'a, Param0: ::windows::runtime::IntoParam<'a, RebootNeededEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRebootNeeded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ComponentLoadFailed<'a, Param0: ::windows::runtime::IntoParam<'a, ComponentLoadFailedEventHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveComponentLoadFailed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaProtectionManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.MediaProtectionManager;{45694947-c741-434b-a79e-474c12d93d2f})");
}
unsafe impl ::windows::runtime::Interface for MediaProtectionManager {
    type Vtable = IMediaProtectionManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1164527943, 51009, 17227, [167, 158, 71, 76, 18, 217, 61, 47]);
}
impl ::windows::runtime::RuntimeName for MediaProtectionManager {
    const NAME: &'static str = "Windows.Media.Protection.MediaProtectionManager";
}
impl ::std::convert::From<MediaProtectionManager> for ::windows::runtime::IUnknown {
    fn from(value: MediaProtectionManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MediaProtectionManager> for ::windows::runtime::IUnknown {
    fn from(value: &MediaProtectionManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaProtectionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MediaProtectionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MediaProtectionManager> for ::windows::runtime::IInspectable {
    fn from(value: MediaProtectionManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MediaProtectionManager> for ::windows::runtime::IInspectable {
    fn from(value: &MediaProtectionManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaProtectionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaProtectionManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MediaProtectionManager {}
unsafe impl ::std::marker::Sync for MediaProtectionManager {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MediaProtectionPMPServer(::windows::runtime::IInspectable);
impl MediaProtectionPMPServer {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IPropertySet>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreatePMPServer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IPropertySet>>(pproperties: Param0) -> ::windows::runtime::Result<MediaProtectionPMPServer> {
        Self::IMediaProtectionPMPServerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), pproperties.into_param().abi(), &mut result__).from_abi::<MediaProtectionPMPServer>(result__)
        })
    }
    pub fn IMediaProtectionPMPServerFactory<R, F: FnOnce(&IMediaProtectionPMPServerFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MediaProtectionPMPServer, IMediaProtectionPMPServerFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaProtectionPMPServer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.MediaProtectionPMPServer;{0c111226-7b26-4d31-95bb-9c1b08ef7fc0})");
}
unsafe impl ::windows::runtime::Interface for MediaProtectionPMPServer {
    type Vtable = IMediaProtectionPMPServer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(202445350, 31526, 19761, [149, 187, 156, 27, 8, 239, 127, 192]);
}
impl ::windows::runtime::RuntimeName for MediaProtectionPMPServer {
    const NAME: &'static str = "Windows.Media.Protection.MediaProtectionPMPServer";
}
impl ::std::convert::From<MediaProtectionPMPServer> for ::windows::runtime::IUnknown {
    fn from(value: MediaProtectionPMPServer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MediaProtectionPMPServer> for ::windows::runtime::IUnknown {
    fn from(value: &MediaProtectionPMPServer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaProtectionPMPServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MediaProtectionPMPServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MediaProtectionPMPServer> for ::windows::runtime::IInspectable {
    fn from(value: MediaProtectionPMPServer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MediaProtectionPMPServer> for ::windows::runtime::IInspectable {
    fn from(value: &MediaProtectionPMPServer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaProtectionPMPServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaProtectionPMPServer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MediaProtectionPMPServer {}
unsafe impl ::std::marker::Sync for MediaProtectionPMPServer {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MediaProtectionServiceCompletion(::windows::runtime::IInspectable);
impl MediaProtectionServiceCompletion {
    pub fn Complete(&self, success: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), success).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MediaProtectionServiceCompletion {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.MediaProtectionServiceCompletion;{8b5cca18-cfd5-44ee-a2ed-df76010c14b5})");
}
unsafe impl ::windows::runtime::Interface for MediaProtectionServiceCompletion {
    type Vtable = IMediaProtectionServiceCompletion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2338114072, 53205, 17646, [162, 237, 223, 118, 1, 12, 20, 181]);
}
impl ::windows::runtime::RuntimeName for MediaProtectionServiceCompletion {
    const NAME: &'static str = "Windows.Media.Protection.MediaProtectionServiceCompletion";
}
impl ::std::convert::From<MediaProtectionServiceCompletion> for ::windows::runtime::IUnknown {
    fn from(value: MediaProtectionServiceCompletion) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MediaProtectionServiceCompletion> for ::windows::runtime::IUnknown {
    fn from(value: &MediaProtectionServiceCompletion) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MediaProtectionServiceCompletion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MediaProtectionServiceCompletion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MediaProtectionServiceCompletion> for ::windows::runtime::IInspectable {
    fn from(value: MediaProtectionServiceCompletion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MediaProtectionServiceCompletion> for ::windows::runtime::IInspectable {
    fn from(value: &MediaProtectionServiceCompletion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MediaProtectionServiceCompletion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MediaProtectionServiceCompletion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MediaProtectionServiceCompletion {}
unsafe impl ::std::marker::Sync for MediaProtectionServiceCompletion {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ProtectionCapabilities(::windows::runtime::IInspectable);
impl ProtectionCapabilities {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ProtectionCapabilities, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsTypeSupported<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, r#type: Param0, keysystem: Param1) -> ::windows::runtime::Result<ProtectionCapabilityResult> {
        let this = self;
        unsafe {
            let mut result__: ProtectionCapabilityResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), r#type.into_param().abi(), keysystem.into_param().abi(), &mut result__).from_abi::<ProtectionCapabilityResult>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ProtectionCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.ProtectionCapabilities;{c7ac5d7e-7480-4d29-a464-7bcd913dd8e4})");
}
unsafe impl ::windows::runtime::Interface for ProtectionCapabilities {
    type Vtable = IProtectionCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3349962110, 29824, 19753, [164, 100, 123, 205, 145, 61, 216, 228]);
}
impl ::windows::runtime::RuntimeName for ProtectionCapabilities {
    const NAME: &'static str = "Windows.Media.Protection.ProtectionCapabilities";
}
impl ::std::convert::From<ProtectionCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: ProtectionCapabilities) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ProtectionCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &ProtectionCapabilities) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ProtectionCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ProtectionCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ProtectionCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: ProtectionCapabilities) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ProtectionCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: &ProtectionCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ProtectionCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ProtectionCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ProtectionCapabilities {}
unsafe impl ::std::marker::Sync for ProtectionCapabilities {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ProtectionCapabilityResult(pub i32);
impl ProtectionCapabilityResult {
    pub const NotSupported: ProtectionCapabilityResult = ProtectionCapabilityResult(0i32);
    pub const Maybe: ProtectionCapabilityResult = ProtectionCapabilityResult(1i32);
    pub const Probably: ProtectionCapabilityResult = ProtectionCapabilityResult(2i32);
}
impl ::std::convert::From<i32> for ProtectionCapabilityResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ProtectionCapabilityResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ProtectionCapabilityResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.ProtectionCapabilityResult;i4)");
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct ProtectionRenewalContract(pub u8);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RebootNeededEventHandler(::windows::runtime::IUnknown);
impl RebootNeededEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<MediaProtectionManager>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = RebootNeededEventHandler_box::<F> {
            vtable: &RebootNeededEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, MediaProtectionManager>>(&self, sender: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RebootNeededEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({64e12a45-973b-4a3a-b260-91898a49a82c})");
}
unsafe impl ::windows::runtime::Interface for RebootNeededEventHandler {
    type Vtable = RebootNeededEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1692478021, 38715, 19002, [178, 96, 145, 137, 138, 73, 168, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct RebootNeededEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct RebootNeededEventHandler_box<F: FnMut(&::std::option::Option<MediaProtectionManager>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const RebootNeededEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<MediaProtectionManager>) -> ::windows::runtime::Result<()> + 'static> RebootNeededEventHandler_box<F> {
    const VTABLE: RebootNeededEventHandler_abi = RebootNeededEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<RebootNeededEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <MediaProtectionManager as ::windows::runtime::Abi>::Abi as *const <MediaProtectionManager as ::windows::runtime::Abi>::DefaultType)).into()
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RenewalStatus(pub i32);
impl RenewalStatus {
    pub const NotStarted: RenewalStatus = RenewalStatus(0i32);
    pub const UpdatesInProgress: RenewalStatus = RenewalStatus(1i32);
    pub const UserCancelled: RenewalStatus = RenewalStatus(2i32);
    pub const AppComponentsMayNeedUpdating: RenewalStatus = RenewalStatus(3i32);
    pub const NoComponentsFound: RenewalStatus = RenewalStatus(4i32);
}
impl ::std::convert::From<i32> for RenewalStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RenewalStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RenewalStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.RenewalStatus;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RevocationAndRenewalInformation(::windows::runtime::IInspectable);
impl RevocationAndRenewalInformation {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Items(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<RevocationAndRenewalItem>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<RevocationAndRenewalItem>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RevocationAndRenewalInformation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.RevocationAndRenewalInformation;{f3a1937b-2501-439e-a6e7-6fc95e175fcf})");
}
unsafe impl ::windows::runtime::Interface for RevocationAndRenewalInformation {
    type Vtable = IRevocationAndRenewalInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4087452539, 9473, 17310, [166, 231, 111, 201, 94, 23, 95, 207]);
}
impl ::windows::runtime::RuntimeName for RevocationAndRenewalInformation {
    const NAME: &'static str = "Windows.Media.Protection.RevocationAndRenewalInformation";
}
impl ::std::convert::From<RevocationAndRenewalInformation> for ::windows::runtime::IUnknown {
    fn from(value: RevocationAndRenewalInformation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RevocationAndRenewalInformation> for ::windows::runtime::IUnknown {
    fn from(value: &RevocationAndRenewalInformation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RevocationAndRenewalInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RevocationAndRenewalInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<RevocationAndRenewalInformation> for ::windows::runtime::IInspectable {
    fn from(value: RevocationAndRenewalInformation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RevocationAndRenewalInformation> for ::windows::runtime::IInspectable {
    fn from(value: &RevocationAndRenewalInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RevocationAndRenewalInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RevocationAndRenewalInformation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RevocationAndRenewalInformation {}
unsafe impl ::std::marker::Sync for RevocationAndRenewalInformation {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RevocationAndRenewalItem(::windows::runtime::IInspectable);
impl RevocationAndRenewalItem {
    pub fn Reasons(&self) -> ::windows::runtime::Result<RevocationAndRenewalReasons> {
        let this = self;
        unsafe {
            let mut result__: RevocationAndRenewalReasons = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RevocationAndRenewalReasons>(result__)
        }
    }
    pub fn HeaderHash(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn PublicKeyHash(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn RenewalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RevocationAndRenewalItem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.RevocationAndRenewalItem;{3099c20c-3cf0-49ea-902d-caf32d2dde2c})");
}
unsafe impl ::windows::runtime::Interface for RevocationAndRenewalItem {
    type Vtable = IRevocationAndRenewalItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(815383052, 15600, 18922, [144, 45, 202, 243, 45, 45, 222, 44]);
}
impl ::windows::runtime::RuntimeName for RevocationAndRenewalItem {
    const NAME: &'static str = "Windows.Media.Protection.RevocationAndRenewalItem";
}
impl ::std::convert::From<RevocationAndRenewalItem> for ::windows::runtime::IUnknown {
    fn from(value: RevocationAndRenewalItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RevocationAndRenewalItem> for ::windows::runtime::IUnknown {
    fn from(value: &RevocationAndRenewalItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RevocationAndRenewalItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RevocationAndRenewalItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<RevocationAndRenewalItem> for ::windows::runtime::IInspectable {
    fn from(value: RevocationAndRenewalItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RevocationAndRenewalItem> for ::windows::runtime::IInspectable {
    fn from(value: &RevocationAndRenewalItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RevocationAndRenewalItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RevocationAndRenewalItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RevocationAndRenewalItem {}
unsafe impl ::std::marker::Sync for RevocationAndRenewalItem {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RevocationAndRenewalReasons(pub u32);
impl RevocationAndRenewalReasons {
    pub const UserModeComponentLoad: RevocationAndRenewalReasons = RevocationAndRenewalReasons(1u32);
    pub const KernelModeComponentLoad: RevocationAndRenewalReasons = RevocationAndRenewalReasons(2u32);
    pub const AppComponent: RevocationAndRenewalReasons = RevocationAndRenewalReasons(4u32);
    pub const GlobalRevocationListLoadFailed: RevocationAndRenewalReasons = RevocationAndRenewalReasons(16u32);
    pub const InvalidGlobalRevocationListSignature: RevocationAndRenewalReasons = RevocationAndRenewalReasons(32u32);
    pub const GlobalRevocationListAbsent: RevocationAndRenewalReasons = RevocationAndRenewalReasons(4096u32);
    pub const ComponentRevoked: RevocationAndRenewalReasons = RevocationAndRenewalReasons(8192u32);
    pub const InvalidComponentCertificateExtendedKeyUse: RevocationAndRenewalReasons = RevocationAndRenewalReasons(16384u32);
    pub const ComponentCertificateRevoked: RevocationAndRenewalReasons = RevocationAndRenewalReasons(32768u32);
    pub const InvalidComponentCertificateRoot: RevocationAndRenewalReasons = RevocationAndRenewalReasons(65536u32);
    pub const ComponentHighSecurityCertificateRevoked: RevocationAndRenewalReasons = RevocationAndRenewalReasons(131072u32);
    pub const ComponentLowSecurityCertificateRevoked: RevocationAndRenewalReasons = RevocationAndRenewalReasons(262144u32);
    pub const BootDriverVerificationFailed: RevocationAndRenewalReasons = RevocationAndRenewalReasons(1048576u32);
    pub const ComponentSignedWithTestCertificate: RevocationAndRenewalReasons = RevocationAndRenewalReasons(16777216u32);
    pub const EncryptionFailure: RevocationAndRenewalReasons = RevocationAndRenewalReasons(268435456u32);
}
impl ::std::convert::From<u32> for RevocationAndRenewalReasons {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RevocationAndRenewalReasons {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RevocationAndRenewalReasons {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.Protection.RevocationAndRenewalReasons;u4)");
}
impl ::std::ops::BitOr for RevocationAndRenewalReasons {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RevocationAndRenewalReasons {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RevocationAndRenewalReasons {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RevocationAndRenewalReasons {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RevocationAndRenewalReasons {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ServiceRequestedEventArgs(::windows::runtime::IInspectable);
impl ServiceRequestedEventArgs {
    pub fn Request(&self) -> ::windows::runtime::Result<IMediaProtectionServiceRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IMediaProtectionServiceRequest>(result__)
        }
    }
    pub fn Completion(&self) -> ::windows::runtime::Result<MediaProtectionServiceCompletion> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MediaProtectionServiceCompletion>(result__)
        }
    }
    #[cfg(feature = "Media_Playback")]
    pub fn MediaPlaybackItem(&self) -> ::windows::runtime::Result<super::Playback::MediaPlaybackItem> {
        let this = &::windows::runtime::Interface::cast::<IServiceRequestedEventArgs2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Playback::MediaPlaybackItem>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ServiceRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Media.Protection.ServiceRequestedEventArgs;{34283baf-abb4-4fc1-bd89-93f106573a49})");
}
unsafe impl ::windows::runtime::Interface for ServiceRequestedEventArgs {
    type Vtable = IServiceRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(875051951, 43956, 20417, [189, 137, 147, 241, 6, 87, 58, 73]);
}
impl ::windows::runtime::RuntimeName for ServiceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.ServiceRequestedEventArgs";
}
impl ::std::convert::From<ServiceRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ServiceRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ServiceRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ServiceRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ServiceRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ServiceRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ServiceRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ServiceRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ServiceRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ServiceRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ServiceRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ServiceRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ServiceRequestedEventArgs {}
unsafe impl ::std::marker::Sync for ServiceRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ServiceRequestedEventHandler(::windows::runtime::IUnknown);
impl ServiceRequestedEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<MediaProtectionManager>, &::std::option::Option<ServiceRequestedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = ServiceRequestedEventHandler_box::<F> {
            vtable: &ServiceRequestedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, MediaProtectionManager>, Param1: ::windows::runtime::IntoParam<'a, ServiceRequestedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ServiceRequestedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({d2d690ba-cac9-48e1-95c0-d38495a84055})");
}
unsafe impl ::windows::runtime::Interface for ServiceRequestedEventHandler {
    type Vtable = ServiceRequestedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3537277114, 51913, 18657, [149, 192, 211, 132, 149, 168, 64, 85]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ServiceRequestedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct ServiceRequestedEventHandler_box<F: FnMut(&::std::option::Option<MediaProtectionManager>, &::std::option::Option<ServiceRequestedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const ServiceRequestedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<MediaProtectionManager>, &::std::option::Option<ServiceRequestedEventArgs>) -> ::windows::runtime::Result<()> + 'static> ServiceRequestedEventHandler_box<F> {
    const VTABLE: ServiceRequestedEventHandler_abi = ServiceRequestedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<ServiceRequestedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <MediaProtectionManager as ::windows::runtime::Abi>::Abi as *const <MediaProtectionManager as ::windows::runtime::Abi>::DefaultType),
            &*(&e as *const <ServiceRequestedEventArgs as ::windows::runtime::Abi>::Abi as *const <ServiceRequestedEventArgs as ::windows::runtime::Abi>::DefaultType),
        )
        .into()
    }
}
