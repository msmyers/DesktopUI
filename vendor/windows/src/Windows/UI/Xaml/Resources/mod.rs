#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CustomXamlResourceLoader(::windows::runtime::IInspectable);
impl CustomXamlResourceLoader {
    pub fn GetResource<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, resourceid: Param0, objecttype: Param1, propertyname: Param2, propertytype: Param3) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<ICustomXamlResourceLoaderOverrides>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), resourceid.into_param().abi(), objecttype.into_param().abi(), propertyname.into_param().abi(), propertytype.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    pub fn Current() -> ::windows::runtime::Result<CustomXamlResourceLoader> {
        Self::ICustomXamlResourceLoaderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CustomXamlResourceLoader>(result__)
        })
    }
    pub fn SetCurrent<'a, Param0: ::windows::runtime::IntoParam<'a, CustomXamlResourceLoader>>(value: Param0) -> ::windows::runtime::Result<()> {
        Self::ICustomXamlResourceLoaderStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() })
    }
    pub fn new() -> ::windows::runtime::Result<CustomXamlResourceLoader> {
        Self::ICustomXamlResourceLoaderFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), ::std::ptr::null_mut(), &mut ::std::option::Option::<::windows::runtime::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<CustomXamlResourceLoader>(result__)
        })
    }
    pub fn ICustomXamlResourceLoaderStatics<R, F: FnOnce(&ICustomXamlResourceLoaderStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CustomXamlResourceLoader, ICustomXamlResourceLoaderStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICustomXamlResourceLoaderFactory<R, F: FnOnce(&ICustomXamlResourceLoaderFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CustomXamlResourceLoader, ICustomXamlResourceLoaderFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CustomXamlResourceLoader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Resources.CustomXamlResourceLoader;{511a84ab-4a88-419f-852e-54083b90b078})");
}
unsafe impl ::windows::runtime::Interface for CustomXamlResourceLoader {
    type Vtable = ICustomXamlResourceLoader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1360692395, 19080, 16799, [133, 46, 84, 8, 59, 144, 176, 120]);
}
impl ::windows::runtime::RuntimeName for CustomXamlResourceLoader {
    const NAME: &'static str = "Windows.UI.Xaml.Resources.CustomXamlResourceLoader";
}
impl ::std::convert::From<CustomXamlResourceLoader> for ::windows::runtime::IUnknown {
    fn from(value: CustomXamlResourceLoader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CustomXamlResourceLoader> for ::windows::runtime::IUnknown {
    fn from(value: &CustomXamlResourceLoader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CustomXamlResourceLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CustomXamlResourceLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CustomXamlResourceLoader> for ::windows::runtime::IInspectable {
    fn from(value: CustomXamlResourceLoader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomXamlResourceLoader> for ::windows::runtime::IInspectable {
    fn from(value: &CustomXamlResourceLoader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CustomXamlResourceLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CustomXamlResourceLoader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CustomXamlResourceLoader {}
unsafe impl ::std::marker::Sync for CustomXamlResourceLoader {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomXamlResourceLoader {
    type Vtable = ICustomXamlResourceLoader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1360692395, 19080, 16799, [133, 46, 84, 8, 59, 144, 176, 120]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoader_abi(
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
pub struct ICustomXamlResourceLoaderFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomXamlResourceLoaderFactory {
    type Vtable = ICustomXamlResourceLoaderFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1543339593, 30854, 17651, [142, 211, 111, 236, 4, 99, 237, 105]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, baseinterface: ::windows::runtime::RawPtr, innerinterface: *mut ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderOverrides(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomXamlResourceLoaderOverrides {
    type Vtable = ICustomXamlResourceLoaderOverrides_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4166117777, 44802, 18152, [154, 248, 66, 123, 126, 191, 233, 248]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderOverrides_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resourceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, objecttype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, propertyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, propertytype: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomXamlResourceLoaderStatics {
    type Vtable = ICustomXamlResourceLoaderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(575665687, 58588, 19495, [173, 50, 219, 147, 213, 208, 229, 218]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomXamlResourceLoaderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
