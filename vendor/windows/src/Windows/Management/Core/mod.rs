#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ApplicationDataManager(::windows::runtime::IInspectable);
impl ApplicationDataManager {
    #[cfg(feature = "Storage")]
    pub fn CreateForPackageFamily<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(packagefamilyname: Param0) -> ::windows::runtime::Result<super::super::Storage::ApplicationData> {
        Self::IApplicationDataManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), packagefamilyname.into_param().abi(), &mut result__).from_abi::<super::super::Storage::ApplicationData>(result__)
        })
    }
    pub fn IApplicationDataManagerStatics<R, F: FnOnce(&IApplicationDataManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ApplicationDataManager, IApplicationDataManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ApplicationDataManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Management.Core.ApplicationDataManager;{74d10432-2e99-4000-9a3a-64307e858129})");
}
unsafe impl ::windows::runtime::Interface for ApplicationDataManager {
    type Vtable = IApplicationDataManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1959855154, 11929, 16384, [154, 58, 100, 48, 126, 133, 129, 41]);
}
impl ::windows::runtime::RuntimeName for ApplicationDataManager {
    const NAME: &'static str = "Windows.Management.Core.ApplicationDataManager";
}
impl ::std::convert::From<ApplicationDataManager> for ::windows::runtime::IUnknown {
    fn from(value: ApplicationDataManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ApplicationDataManager> for ::windows::runtime::IUnknown {
    fn from(value: &ApplicationDataManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ApplicationDataManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ApplicationDataManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ApplicationDataManager> for ::windows::runtime::IInspectable {
    fn from(value: ApplicationDataManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ApplicationDataManager> for ::windows::runtime::IInspectable {
    fn from(value: &ApplicationDataManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ApplicationDataManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ApplicationDataManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ApplicationDataManager {}
unsafe impl ::std::marker::Sync for ApplicationDataManager {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IApplicationDataManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IApplicationDataManager {
    type Vtable = IApplicationDataManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1959855154, 11929, 16384, [154, 58, 100, 48, 126, 133, 129, 41]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDataManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IApplicationDataManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IApplicationDataManagerStatics {
    type Vtable = IApplicationDataManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(504914659, 27022, 18849, [151, 82, 222, 233, 73, 37, 185, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationDataManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
