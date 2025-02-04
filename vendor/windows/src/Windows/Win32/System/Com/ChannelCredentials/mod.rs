#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IChannelCredentials(::windows::runtime::IUnknown);
impl IChannelCredentials {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWindowsCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BOOL>>(
        &self,
        domain: Param0,
        username: Param1,
        password: Param2,
        impersonationlevel: i32,
        allowntlm: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), domain.into_param().abi(), username.into_param().abi(), password.into_param().abi(), ::std::mem::transmute(impersonationlevel), allowntlm.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserNameCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, username: Param0, password: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), username.into_param().abi(), password.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetClientCertificateFromStore<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::VARIANT>>(&self, storelocation: Param0, storename: Param1, findyype: Param2, findvalue: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), storelocation.into_param().abi(), storename.into_param().abi(), findyype.into_param().abi(), findvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientCertificateFromStoreByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, subjectname: Param0, storelocation: Param1, storename: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), subjectname.into_param().abi(), storelocation.into_param().abi(), storename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientCertificateFromFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, filename: Param0, password: Param1, keystorageflags: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), filename.into_param().abi(), password.into_param().abi(), keystorageflags.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetDefaultServiceCertificateFromStore<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::VARIANT>>(&self, storelocation: Param0, storename: Param1, findtype: Param2, findvalue: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), storelocation.into_param().abi(), storename.into_param().abi(), findtype.into_param().abi(), findvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDefaultServiceCertificateFromStoreByName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, subjectname: Param0, storelocation: Param1, storename: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), subjectname.into_param().abi(), storelocation.into_param().abi(), storename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDefaultServiceCertificateFromFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, filename: Param0, password: Param1, keystorageflags: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), filename.into_param().abi(), password.into_param().abi(), keystorageflags.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetServiceCertificateAuthentication<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, storelocation: Param0, revocationmode: Param1, certificatevalidationmode: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), storelocation.into_param().abi(), revocationmode.into_param().abi(), certificatevalidationmode.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIssuedToken<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, localissueraddres: Param0, localissuerbindingtype: Param1, localissuerbinding: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), localissueraddres.into_param().abi(), localissuerbindingtype.into_param().abi(), localissuerbinding.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IChannelCredentials {
    type Vtable = IChannelCredentials_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(404440204, 49532, 19223, [172, 109, 6, 105, 155, 147, 25, 143]);
}
impl ::std::convert::From<IChannelCredentials> for ::windows::runtime::IUnknown {
    fn from(value: IChannelCredentials) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IChannelCredentials> for ::windows::runtime::IUnknown {
    fn from(value: &IChannelCredentials) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IChannelCredentials {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IChannelCredentials {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IChannelCredentials> for super::super::Ole::Automation::IDispatch {
    fn from(value: IChannelCredentials) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IChannelCredentials> for super::super::Ole::Automation::IDispatch {
    fn from(value: &IChannelCredentials) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Ole::Automation::IDispatch> for IChannelCredentials {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Ole::Automation::IDispatch> for &IChannelCredentials {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelCredentials_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, domain: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, username: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, impersonationlevel: i32, allowntlm: super::super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, username: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storelocation: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findyype: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findvalue: ::std::mem::ManuallyDrop<super::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, subjectname: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storelocation: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, keystorageflags: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storelocation: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findtype: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, findvalue: ::std::mem::ManuallyDrop<super::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, subjectname: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storelocation: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, storename: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, password: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, keystorageflags: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storelocation: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, revocationmode: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, certificatevalidationmode: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localissueraddres: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, localissuerbindingtype: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, localissuerbinding: ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
