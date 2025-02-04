#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CredentialPromptType(pub i32);
impl CredentialPromptType {
    pub const PromptIfNeeded: CredentialPromptType = CredentialPromptType(0i32);
    pub const RetypeCredentials: CredentialPromptType = CredentialPromptType(1i32);
    pub const DoNotPrompt: CredentialPromptType = CredentialPromptType(2i32);
}
impl ::std::convert::From<i32> for CredentialPromptType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CredentialPromptType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CredentialPromptType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.OnlineId.CredentialPromptType;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOnlineIdAuthenticator(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOnlineIdAuthenticator {
    type Vtable = IOnlineIdAuthenticator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2684614026, 10667, 18455, [184, 132, 215, 81, 109, 173, 24, 185]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdAuthenticator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, request: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requests: ::windows::runtime::RawPtr, credentialprompttype: CredentialPromptType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicket(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOnlineIdServiceTicket {
    type Vtable = IOnlineIdServiceTicket_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3378271359, 55169, 19092, [172, 184, 197, 152, 116, 35, 140, 38]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicket_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicketRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOnlineIdServiceTicketRequest {
    type Vtable = IOnlineIdServiceTicketRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(695485907, 64355, 16693, [137, 9, 78, 53, 76, 6, 20, 102]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicketRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicketRequestFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOnlineIdServiceTicketRequestFactory {
    type Vtable = IOnlineIdServiceTicketRequestFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3199928840, 40563, 16503, [150, 20, 8, 97, 76, 11, 194, 69]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicketRequestFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, service: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, policy: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, service: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOnlineIdSystemAuthenticatorForUser(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOnlineIdSystemAuthenticatorForUser {
    type Vtable = IOnlineIdSystemAuthenticatorForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1469628155, 7652, 16774, [162, 230, 181, 99, 248, 106, 175, 68]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemAuthenticatorForUser_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, request: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOnlineIdSystemAuthenticatorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOnlineIdSystemAuthenticatorStatics {
    type Vtable = IOnlineIdSystemAuthenticatorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2231662482, 63028, 16867, [150, 164, 81, 100, 233, 2, 199, 64]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemAuthenticatorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOnlineIdSystemIdentity(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOnlineIdSystemIdentity {
    type Vtable = IOnlineIdSystemIdentity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1950142989, 46794, 17229, [129, 36, 83, 234, 18, 104, 83, 7]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemIdentity_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IOnlineIdSystemTicketResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IOnlineIdSystemTicketResult {
    type Vtable = IOnlineIdSystemTicketResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3674890232, 45208, 19149, [157, 19, 158, 100, 6, 82, 181, 182]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemTicketResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut OnlineIdSystemTicketStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IUserIdentity(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserIdentity {
    type Vtable = IUserIdentity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(558291405, 1858, 19427, [138, 28, 124, 122, 230, 121, 170, 136]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserIdentity_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct OnlineIdAuthenticator(::windows::runtime::IInspectable);
impl OnlineIdAuthenticator {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<OnlineIdAuthenticator, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, OnlineIdServiceTicketRequest>>(&self, request: Param0) -> ::windows::runtime::Result<UserAuthenticationOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<UserAuthenticationOperation>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn AuthenticateUserAsyncAdvanced<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<OnlineIdServiceTicketRequest>>>(&self, requests: Param0, credentialprompttype: CredentialPromptType) -> ::windows::runtime::Result<UserAuthenticationOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), requests.into_param().abi(), credentialprompttype, &mut result__).from_abi::<UserAuthenticationOperation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SignOutUserAsync(&self) -> ::windows::runtime::Result<SignOutUserOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SignOutUserOperation>(result__)
        }
    }
    pub fn SetApplicationId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ApplicationId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    pub fn CanSignOut(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AuthenticatedSafeCustomerId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OnlineIdAuthenticator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator;{a003f58a-29ab-4817-b884-d7516dad18b9})");
}
unsafe impl ::windows::runtime::Interface for OnlineIdAuthenticator {
    type Vtable = IOnlineIdAuthenticator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2684614026, 10667, 18455, [184, 132, 215, 81, 109, 173, 24, 185]);
}
impl ::windows::runtime::RuntimeName for OnlineIdAuthenticator {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator";
}
impl ::std::convert::From<OnlineIdAuthenticator> for ::windows::runtime::IUnknown {
    fn from(value: OnlineIdAuthenticator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OnlineIdAuthenticator> for ::windows::runtime::IUnknown {
    fn from(value: &OnlineIdAuthenticator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OnlineIdAuthenticator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OnlineIdAuthenticator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<OnlineIdAuthenticator> for ::windows::runtime::IInspectable {
    fn from(value: OnlineIdAuthenticator) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OnlineIdAuthenticator> for ::windows::runtime::IInspectable {
    fn from(value: &OnlineIdAuthenticator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OnlineIdAuthenticator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OnlineIdAuthenticator {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for OnlineIdAuthenticator {}
unsafe impl ::std::marker::Sync for OnlineIdAuthenticator {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct OnlineIdServiceTicket(::windows::runtime::IInspectable);
impl OnlineIdServiceTicket {
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Request(&self) -> ::windows::runtime::Result<OnlineIdServiceTicketRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<OnlineIdServiceTicketRequest>(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OnlineIdServiceTicket {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket;{c95c547f-d781-4a94-acb8-c59874238c26})");
}
unsafe impl ::windows::runtime::Interface for OnlineIdServiceTicket {
    type Vtable = IOnlineIdServiceTicket_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3378271359, 55169, 19092, [172, 184, 197, 152, 116, 35, 140, 38]);
}
impl ::windows::runtime::RuntimeName for OnlineIdServiceTicket {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket";
}
impl ::std::convert::From<OnlineIdServiceTicket> for ::windows::runtime::IUnknown {
    fn from(value: OnlineIdServiceTicket) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OnlineIdServiceTicket> for ::windows::runtime::IUnknown {
    fn from(value: &OnlineIdServiceTicket) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OnlineIdServiceTicket {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OnlineIdServiceTicket {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<OnlineIdServiceTicket> for ::windows::runtime::IInspectable {
    fn from(value: OnlineIdServiceTicket) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OnlineIdServiceTicket> for ::windows::runtime::IInspectable {
    fn from(value: &OnlineIdServiceTicket) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OnlineIdServiceTicket {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OnlineIdServiceTicket {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for OnlineIdServiceTicket {}
unsafe impl ::std::marker::Sync for OnlineIdServiceTicket {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct OnlineIdServiceTicketRequest(::windows::runtime::IInspectable);
impl OnlineIdServiceTicketRequest {
    pub fn Service(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Policy(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn CreateOnlineIdServiceTicketRequest<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(service: Param0, policy: Param1) -> ::windows::runtime::Result<OnlineIdServiceTicketRequest> {
        Self::IOnlineIdServiceTicketRequestFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), service.into_param().abi(), policy.into_param().abi(), &mut result__).from_abi::<OnlineIdServiceTicketRequest>(result__)
        })
    }
    pub fn CreateOnlineIdServiceTicketRequestAdvanced<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(service: Param0) -> ::windows::runtime::Result<OnlineIdServiceTicketRequest> {
        Self::IOnlineIdServiceTicketRequestFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), service.into_param().abi(), &mut result__).from_abi::<OnlineIdServiceTicketRequest>(result__)
        })
    }
    pub fn IOnlineIdServiceTicketRequestFactory<R, F: FnOnce(&IOnlineIdServiceTicketRequestFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<OnlineIdServiceTicketRequest, IOnlineIdServiceTicketRequestFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OnlineIdServiceTicketRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest;{297445d3-fb63-4135-8909-4e354c061466})");
}
unsafe impl ::windows::runtime::Interface for OnlineIdServiceTicketRequest {
    type Vtable = IOnlineIdServiceTicketRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(695485907, 64355, 16693, [137, 9, 78, 53, 76, 6, 20, 102]);
}
impl ::windows::runtime::RuntimeName for OnlineIdServiceTicketRequest {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest";
}
impl ::std::convert::From<OnlineIdServiceTicketRequest> for ::windows::runtime::IUnknown {
    fn from(value: OnlineIdServiceTicketRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OnlineIdServiceTicketRequest> for ::windows::runtime::IUnknown {
    fn from(value: &OnlineIdServiceTicketRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<OnlineIdServiceTicketRequest> for ::windows::runtime::IInspectable {
    fn from(value: OnlineIdServiceTicketRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OnlineIdServiceTicketRequest> for ::windows::runtime::IInspectable {
    fn from(value: &OnlineIdServiceTicketRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for OnlineIdServiceTicketRequest {}
unsafe impl ::std::marker::Sync for OnlineIdServiceTicketRequest {}
pub struct OnlineIdSystemAuthenticator {}
impl OnlineIdSystemAuthenticator {
    pub fn Default() -> ::windows::runtime::Result<OnlineIdSystemAuthenticatorForUser> {
        Self::IOnlineIdSystemAuthenticatorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<OnlineIdSystemAuthenticatorForUser>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<OnlineIdSystemAuthenticatorForUser> {
        Self::IOnlineIdSystemAuthenticatorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<OnlineIdSystemAuthenticatorForUser>(result__)
        })
    }
    pub fn IOnlineIdSystemAuthenticatorStatics<R, F: FnOnce(&IOnlineIdSystemAuthenticatorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<OnlineIdSystemAuthenticator, IOnlineIdSystemAuthenticatorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for OnlineIdSystemAuthenticator {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticator";
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct OnlineIdSystemAuthenticatorForUser(::windows::runtime::IInspectable);
impl OnlineIdSystemAuthenticatorForUser {
    #[cfg(feature = "Foundation")]
    pub fn GetTicketAsync<'a, Param0: ::windows::runtime::IntoParam<'a, OnlineIdServiceTicketRequest>>(&self, request: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<OnlineIdSystemTicketResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OnlineIdSystemTicketResult>>(result__)
        }
    }
    pub fn SetApplicationId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ApplicationId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OnlineIdSystemAuthenticatorForUser {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser;{5798befb-1de4-4186-a2e6-b563f86aaf44})");
}
unsafe impl ::windows::runtime::Interface for OnlineIdSystemAuthenticatorForUser {
    type Vtable = IOnlineIdSystemAuthenticatorForUser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1469628155, 7652, 16774, [162, 230, 181, 99, 248, 106, 175, 68]);
}
impl ::windows::runtime::RuntimeName for OnlineIdSystemAuthenticatorForUser {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser";
}
impl ::std::convert::From<OnlineIdSystemAuthenticatorForUser> for ::windows::runtime::IUnknown {
    fn from(value: OnlineIdSystemAuthenticatorForUser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OnlineIdSystemAuthenticatorForUser> for ::windows::runtime::IUnknown {
    fn from(value: &OnlineIdSystemAuthenticatorForUser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<OnlineIdSystemAuthenticatorForUser> for ::windows::runtime::IInspectable {
    fn from(value: OnlineIdSystemAuthenticatorForUser) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OnlineIdSystemAuthenticatorForUser> for ::windows::runtime::IInspectable {
    fn from(value: &OnlineIdSystemAuthenticatorForUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for OnlineIdSystemAuthenticatorForUser {}
unsafe impl ::std::marker::Sync for OnlineIdSystemAuthenticatorForUser {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct OnlineIdSystemIdentity(::windows::runtime::IInspectable);
impl OnlineIdSystemIdentity {
    pub fn Ticket(&self) -> ::windows::runtime::Result<OnlineIdServiceTicket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<OnlineIdServiceTicket>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OnlineIdSystemIdentity {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity;{743cd20d-b6ca-434d-8124-53ea12685307})");
}
unsafe impl ::windows::runtime::Interface for OnlineIdSystemIdentity {
    type Vtable = IOnlineIdSystemIdentity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1950142989, 46794, 17229, [129, 36, 83, 234, 18, 104, 83, 7]);
}
impl ::windows::runtime::RuntimeName for OnlineIdSystemIdentity {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity";
}
impl ::std::convert::From<OnlineIdSystemIdentity> for ::windows::runtime::IUnknown {
    fn from(value: OnlineIdSystemIdentity) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OnlineIdSystemIdentity> for ::windows::runtime::IUnknown {
    fn from(value: &OnlineIdSystemIdentity) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<OnlineIdSystemIdentity> for ::windows::runtime::IInspectable {
    fn from(value: OnlineIdSystemIdentity) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OnlineIdSystemIdentity> for ::windows::runtime::IInspectable {
    fn from(value: &OnlineIdSystemIdentity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for OnlineIdSystemIdentity {}
unsafe impl ::std::marker::Sync for OnlineIdSystemIdentity {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct OnlineIdSystemTicketResult(::windows::runtime::IInspectable);
impl OnlineIdSystemTicketResult {
    pub fn Identity(&self) -> ::windows::runtime::Result<OnlineIdSystemIdentity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<OnlineIdSystemIdentity>(result__)
        }
    }
    pub fn Status(&self) -> ::windows::runtime::Result<OnlineIdSystemTicketStatus> {
        let this = self;
        unsafe {
            let mut result__: OnlineIdSystemTicketStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<OnlineIdSystemTicketStatus>(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for OnlineIdSystemTicketResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult;{db0a5ff8-b098-4acd-9d13-9e640652b5b6})");
}
unsafe impl ::windows::runtime::Interface for OnlineIdSystemTicketResult {
    type Vtable = IOnlineIdSystemTicketResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3674890232, 45208, 19149, [157, 19, 158, 100, 6, 82, 181, 182]);
}
impl ::windows::runtime::RuntimeName for OnlineIdSystemTicketResult {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult";
}
impl ::std::convert::From<OnlineIdSystemTicketResult> for ::windows::runtime::IUnknown {
    fn from(value: OnlineIdSystemTicketResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&OnlineIdSystemTicketResult> for ::windows::runtime::IUnknown {
    fn from(value: &OnlineIdSystemTicketResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<OnlineIdSystemTicketResult> for ::windows::runtime::IInspectable {
    fn from(value: OnlineIdSystemTicketResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&OnlineIdSystemTicketResult> for ::windows::runtime::IInspectable {
    fn from(value: &OnlineIdSystemTicketResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for OnlineIdSystemTicketResult {}
unsafe impl ::std::marker::Sync for OnlineIdSystemTicketResult {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OnlineIdSystemTicketStatus(pub i32);
impl OnlineIdSystemTicketStatus {
    pub const Success: OnlineIdSystemTicketStatus = OnlineIdSystemTicketStatus(0i32);
    pub const Error: OnlineIdSystemTicketStatus = OnlineIdSystemTicketStatus(1i32);
    pub const ServiceConnectionError: OnlineIdSystemTicketStatus = OnlineIdSystemTicketStatus(2i32);
}
impl ::std::convert::From<i32> for OnlineIdSystemTicketStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OnlineIdSystemTicketStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for OnlineIdSystemTicketStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketStatus;i4)");
}
#[cfg(feature = "Foundation")]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SignOutUserOperation(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation")]
impl SignOutUserOperation {
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::AsyncActionCompletedHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows::runtime::Result<super::super::super::Foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::AsyncActionCompletedHandler>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::runtime::Result<super::super::super::Foundation::AsyncStatus> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::AsyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::runtime::Result<()> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let (waiter, signaler) = ::windows::runtime::Waiter::new();
            self.SetCompleted(super::super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::RuntimeType for SignOutUserOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.SignOutUserOperation;{5a648006-843a-4da9-865b-9d26e5dfad7b})");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Interface for SignOutUserOperation {
    type Vtable = super::super::super::Foundation::IAsyncAction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1516535814, 33850, 19881, [134, 91, 157, 38, 229, 223, 173, 123]);
}
#[cfg(feature = "Foundation")]
impl ::windows::runtime::RuntimeName for SignOutUserOperation {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.SignOutUserOperation";
}
impl ::std::future::Future for SignOutUserOperation {
    type Output = ::windows::runtime::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(super::super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<SignOutUserOperation> for ::windows::runtime::IUnknown {
    fn from(value: SignOutUserOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&SignOutUserOperation> for ::windows::runtime::IUnknown {
    fn from(value: &SignOutUserOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SignOutUserOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SignOutUserOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<SignOutUserOperation> for ::windows::runtime::IInspectable {
    fn from(value: SignOutUserOperation) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&SignOutUserOperation> for ::windows::runtime::IInspectable {
    fn from(value: &SignOutUserOperation) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SignOutUserOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SignOutUserOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<SignOutUserOperation> for super::super::super::Foundation::IAsyncAction {
    fn from(value: SignOutUserOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&SignOutUserOperation> for super::super::super::Foundation::IAsyncAction {
    fn from(value: &SignOutUserOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IAsyncAction> for SignOutUserOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IAsyncAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::IAsyncAction>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IAsyncAction> for &SignOutUserOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IAsyncAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::IAsyncAction>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<SignOutUserOperation> for super::super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SignOutUserOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&SignOutUserOperation> for super::super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SignOutUserOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IAsyncInfo> for SignOutUserOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IAsyncInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IAsyncInfo> for &SignOutUserOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IAsyncInfo> {
        ::std::convert::TryInto::<super::super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::std::marker::Send for SignOutUserOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::std::marker::Sync for SignOutUserOperation {}
#[cfg(feature = "Foundation")]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UserAuthenticationOperation(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation")]
impl UserAuthenticationOperation {
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::AsyncOperationCompletedHandler<UserIdentity>>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows::runtime::Result<super::super::super::Foundation::AsyncOperationCompletedHandler<UserIdentity>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::AsyncOperationCompletedHandler<UserIdentity>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows::runtime::Result<UserIdentity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserIdentity>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::runtime::Result<super::super::super::Foundation::AsyncStatus> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::AsyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::runtime::Result<UserIdentity> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let (waiter, signaler) = ::windows::runtime::Waiter::new();
            self.SetCompleted(super::super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::RuntimeType for UserAuthenticationOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.UserAuthenticationOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};rc(Windows.Security.Authentication.OnlineId.UserIdentity;{2146d9cd-0742-4be3-8a1c-7c7ae679aa88})))");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Interface for UserAuthenticationOperation {
    type Vtable = super::super::super::Foundation::IAsyncOperation_abi<UserIdentity>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::IAsyncOperation<UserIdentity> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation")]
impl ::windows::runtime::RuntimeName for UserAuthenticationOperation {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.UserAuthenticationOperation";
}
impl ::std::future::Future for UserAuthenticationOperation {
    type Output = ::windows::runtime::Result<UserIdentity>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(super::super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<UserAuthenticationOperation> for ::windows::runtime::IUnknown {
    fn from(value: UserAuthenticationOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&UserAuthenticationOperation> for ::windows::runtime::IUnknown {
    fn from(value: &UserAuthenticationOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserAuthenticationOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<UserAuthenticationOperation> for ::windows::runtime::IInspectable {
    fn from(value: UserAuthenticationOperation) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&UserAuthenticationOperation> for ::windows::runtime::IInspectable {
    fn from(value: &UserAuthenticationOperation) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserAuthenticationOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<UserAuthenticationOperation> for super::super::super::Foundation::IAsyncOperation<UserIdentity> {
    fn from(value: UserAuthenticationOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&UserAuthenticationOperation> for super::super::super::Foundation::IAsyncOperation<UserIdentity> {
    fn from(value: &UserAuthenticationOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IAsyncOperation<UserIdentity>> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IAsyncOperation<UserIdentity>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::IAsyncOperation<UserIdentity>>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IAsyncOperation<UserIdentity>> for &UserAuthenticationOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IAsyncOperation<UserIdentity>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::IAsyncOperation<UserIdentity>>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<UserAuthenticationOperation> for super::super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: UserAuthenticationOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&UserAuthenticationOperation> for super::super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &UserAuthenticationOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IAsyncInfo> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IAsyncInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IAsyncInfo> for &UserAuthenticationOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IAsyncInfo> {
        ::std::convert::TryInto::<super::super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::std::marker::Send for UserAuthenticationOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::std::marker::Sync for UserAuthenticationOperation {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UserIdentity(::windows::runtime::IInspectable);
impl UserIdentity {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tickets(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<OnlineIdServiceTicket>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<OnlineIdServiceTicket>>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SafeCustomerId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SignInName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn FirstName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn LastName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn IsBetaAccount(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsConfirmedPC(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserIdentity {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.UserIdentity;{2146d9cd-0742-4be3-8a1c-7c7ae679aa88})");
}
unsafe impl ::windows::runtime::Interface for UserIdentity {
    type Vtable = IUserIdentity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(558291405, 1858, 19427, [138, 28, 124, 122, 230, 121, 170, 136]);
}
impl ::windows::runtime::RuntimeName for UserIdentity {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.UserIdentity";
}
impl ::std::convert::From<UserIdentity> for ::windows::runtime::IUnknown {
    fn from(value: UserIdentity) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UserIdentity> for ::windows::runtime::IUnknown {
    fn from(value: &UserIdentity) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UserIdentity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UserIdentity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UserIdentity> for ::windows::runtime::IInspectable {
    fn from(value: UserIdentity) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UserIdentity> for ::windows::runtime::IInspectable {
    fn from(value: &UserIdentity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UserIdentity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UserIdentity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UserIdentity {}
unsafe impl ::std::marker::Sync for UserIdentity {}
