#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::BSTR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserFree(param0: *const u32, param1: *const ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>);
        }
        ::std::mem::transmute(BSTR_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::BSTR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserFree64(param0: *const u32, param1: *const ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>);
        }
        ::std::mem::transmute(BSTR_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> *mut u8;
        }
        ::std::mem::transmute(BSTR_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> *mut u8;
        }
        ::std::mem::transmute(BSTR_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::BSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserSize(param0: *const u32, param1: u32, param2: *const ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> u32;
        }
        ::std::mem::transmute(BSTR_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::BSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserSize64(param0: *const u32, param1: u32, param2: *const ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> u32;
        }
        ::std::mem::transmute(BSTR_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> *mut u8;
        }
        ::std::mem::transmute(BSTR_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BSTR_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::BSTR) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BSTR_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> *mut u8;
        }
        ::std::mem::transmute(BSTR_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserFree(param0: *const u32, param1: *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserFree(param0: *const u32, param1: *const u16);
        }
        ::std::mem::transmute(CLIPFORMAT_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserFree64(param0: *const u32, param1: *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserFree64(param0: *const u32, param1: *const u16);
        }
        ::std::mem::transmute(CLIPFORMAT_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8;
        }
        ::std::mem::transmute(CLIPFORMAT_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8;
        }
        ::std::mem::transmute(CLIPFORMAT_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserSize(param0: *const u32, param1: u32, param2: *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserSize(param0: *const u32, param1: u32, param2: *const u16) -> u32;
        }
        ::std::mem::transmute(CLIPFORMAT_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserSize64(param0: *const u32, param1: u32, param2: *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserSize64(param0: *const u32, param1: u32, param2: *const u16) -> u32;
        }
        ::std::mem::transmute(CLIPFORMAT_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8;
        }
        ::std::mem::transmute(CLIPFORMAT_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CLIPFORMAT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CLIPFORMAT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8;
        }
        ::std::mem::transmute(CLIPFORMAT_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetMarshalSizeMax<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pulsize: *mut u32, riid: *const ::windows::runtime::GUID, punk: Param2, dwdestcontext: u32, pvdestcontext: *const ::std::ffi::c_void, mshlflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetMarshalSizeMax(pulsize: *mut u32, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr, dwdestcontext: u32, pvdestcontext: *const ::std::ffi::c_void, mshlflags: u32) -> ::windows::runtime::HRESULT;
        }
        CoGetMarshalSizeMax(::std::mem::transmute(pulsize), ::std::mem::transmute(riid), punk.into_param().abi(), ::std::mem::transmute(dwdestcontext), ::std::mem::transmute(pvdestcontext), ::std::mem::transmute(mshlflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetStandardMarshal<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(riid: *const ::windows::runtime::GUID, punk: Param1, dwdestcontext: u32, pvdestcontext: *const ::std::ffi::c_void, mshlflags: u32) -> ::windows::runtime::Result<IMarshal> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetStandardMarshal(riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr, dwdestcontext: u32, pvdestcontext: *const ::std::ffi::c_void, mshlflags: u32, ppmarshal: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IMarshal as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CoGetStandardMarshal(::std::mem::transmute(riid), punk.into_param().abi(), ::std::mem::transmute(dwdestcontext), ::std::mem::transmute(pvdestcontext), ::std::mem::transmute(mshlflags), &mut result__).from_abi::<IMarshal>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoGetStdMarshalEx<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(punkouter: Param0, smexflags: u32) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoGetStdMarshalEx(punkouter: ::windows::runtime::RawPtr, smexflags: u32, ppunkinner: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CoGetStdMarshalEx(punkouter.into_param().abi(), ::std::mem::transmute(smexflags), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoMarshalHresult<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>>(pstm: Param0, hresult: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoMarshalHresult(pstm: ::windows::runtime::RawPtr, hresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT;
        }
        CoMarshalHresult(pstm.into_param().abi(), ::std::mem::transmute(hresult)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoMarshalInterThreadInterfaceInStream<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(riid: *const ::windows::runtime::GUID, punk: Param1) -> ::windows::runtime::Result<super::IStream> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoMarshalInterThreadInterfaceInStream(riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr, ppstm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CoMarshalInterThreadInterfaceInStream(::std::mem::transmute(riid), punk.into_param().abi(), &mut result__).from_abi::<super::IStream>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoMarshalInterface<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(pstm: Param0, riid: *const ::windows::runtime::GUID, punk: Param2, dwdestcontext: u32, pvdestcontext: *const ::std::ffi::c_void, mshlflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoMarshalInterface(pstm: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr, dwdestcontext: u32, pvdestcontext: *const ::std::ffi::c_void, mshlflags: u32) -> ::windows::runtime::HRESULT;
        }
        CoMarshalInterface(pstm.into_param().abi(), ::std::mem::transmute(riid), punk.into_param().abi(), ::std::mem::transmute(dwdestcontext), ::std::mem::transmute(pvdestcontext), ::std::mem::transmute(mshlflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoReleaseMarshalData<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>>(pstm: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoReleaseMarshalData(pstm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        CoReleaseMarshalData(pstm.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoUnmarshalHresult<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>>(pstm: Param0) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoUnmarshalHresult(pstm: ::windows::runtime::RawPtr, phresult: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::HRESULT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CoUnmarshalHresult(pstm.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CoUnmarshalInterface<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>, T: ::windows::runtime::Interface>(pstm: Param0) -> ::windows::runtime::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoUnmarshalInterface(pstm: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        let mut result__ = ::std::option::Option::None;
        CoUnmarshalInterface(pstm.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL);
        }
        ::std::mem::transmute(HACCEL_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL);
        }
        ::std::mem::transmute(HACCEL_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::std::mem::transmute(HACCEL_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::std::mem::transmute(HACCEL_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32;
        }
        ::std::mem::transmute(HACCEL_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32;
        }
        ::std::mem::transmute(HACCEL_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::std::mem::transmute(HACCEL_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HACCEL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HACCEL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
        }
        ::std::mem::transmute(HACCEL_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP);
        }
        ::std::mem::transmute(HBITMAP_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP);
        }
        ::std::mem::transmute(HBITMAP_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::std::mem::transmute(HBITMAP_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::std::mem::transmute(HBITMAP_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32;
        }
        ::std::mem::transmute(HBITMAP_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32;
        }
        ::std::mem::transmute(HBITMAP_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::std::mem::transmute(HBITMAP_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HBITMAP_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HBITMAP_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
        }
        ::std::mem::transmute(HBITMAP_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC);
        }
        ::std::mem::transmute(HDC_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC);
        }
        ::std::mem::transmute(HDC_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8;
        }
        ::std::mem::transmute(HDC_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8;
        }
        ::std::mem::transmute(HDC_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32;
        }
        ::std::mem::transmute(HDC_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32;
        }
        ::std::mem::transmute(HDC_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8;
        }
        ::std::mem::transmute(HDC_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HDC_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HDC_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8;
        }
        ::std::mem::transmute(HDC_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserFree(param0: *const u32, param1: *const isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserFree(param0: *const u32, param1: *const isize);
        }
        ::std::mem::transmute(HGLOBAL_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserFree64(param0: *const u32, param1: *const isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserFree64(param0: *const u32, param1: *const isize);
        }
        ::std::mem::transmute(HGLOBAL_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8;
        }
        ::std::mem::transmute(HGLOBAL_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8;
        }
        ::std::mem::transmute(HGLOBAL_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserSize(param0: *const u32, param1: u32, param2: *const isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserSize(param0: *const u32, param1: u32, param2: *const isize) -> u32;
        }
        ::std::mem::transmute(HGLOBAL_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserSize64(param0: *const u32, param1: u32, param2: *const isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserSize64(param0: *const u32, param1: u32, param2: *const isize) -> u32;
        }
        ::std::mem::transmute(HGLOBAL_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8;
        }
        ::std::mem::transmute(HGLOBAL_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn HGLOBAL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HGLOBAL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8;
        }
        ::std::mem::transmute(HGLOBAL_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON);
        }
        ::std::mem::transmute(HICON_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON);
        }
        ::std::mem::transmute(HICON_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::std::mem::transmute(HICON_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::std::mem::transmute(HICON_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32;
        }
        ::std::mem::transmute(HICON_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32;
        }
        ::std::mem::transmute(HICON_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::std::mem::transmute(HICON_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HICON_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HICON_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
        }
        ::std::mem::transmute(HICON_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU);
        }
        ::std::mem::transmute(HMENU_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU);
        }
        ::std::mem::transmute(HMENU_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::std::mem::transmute(HMENU_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::std::mem::transmute(HMENU_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32;
        }
        ::std::mem::transmute(HMENU_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32;
        }
        ::std::mem::transmute(HMENU_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::std::mem::transmute(HMENU_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[inline]
pub unsafe fn HMENU_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HMENU_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
        }
        ::std::mem::transmute(HMENU_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE);
        }
        ::std::mem::transmute(HPALETTE_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE);
        }
        ::std::mem::transmute(HPALETTE_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::std::mem::transmute(HPALETTE_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::std::mem::transmute(HPALETTE_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32;
        }
        ::std::mem::transmute(HPALETTE_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32;
        }
        ::std::mem::transmute(HPALETTE_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::std::mem::transmute(HPALETTE_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn HPALETTE_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HPALETTE_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
        }
        ::std::mem::transmute(HPALETTE_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::HWND) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::HWND);
        }
        ::std::mem::transmute(HWND_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::HWND) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::HWND);
        }
        ::std::mem::transmute(HWND_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8;
        }
        ::std::mem::transmute(HWND_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8;
        }
        ::std::mem::transmute(HWND_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32;
        }
        ::std::mem::transmute(HWND_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32;
        }
        ::std::mem::transmute(HWND_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8;
        }
        ::std::mem::transmute(HWND_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HWND_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HWND_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8;
        }
        ::std::mem::transmute(HWND_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMarshal(::windows::runtime::IUnknown);
impl IMarshal {
    pub unsafe fn GetUnmarshalClass(&self, riid: *const ::windows::runtime::GUID, pv: *const ::std::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::std::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(pv), ::std::mem::transmute(dwdestcontext), ::std::mem::transmute(pvdestcontext), ::std::mem::transmute(mshlflags), ::std::mem::transmute(pcid)).ok()
    }
    pub unsafe fn GetMarshalSizeMax(&self, riid: *const ::windows::runtime::GUID, pv: *const ::std::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::std::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(pv), ::std::mem::transmute(dwdestcontext), ::std::mem::transmute(pvdestcontext), ::std::mem::transmute(mshlflags), ::std::mem::transmute(psize)).ok()
    }
    pub unsafe fn MarshalInterface<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows::runtime::GUID, pv: *const ::std::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::std::ffi::c_void, mshlflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pstm.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(pv), ::std::mem::transmute(dwdestcontext), ::std::mem::transmute(pvdestcontext), ::std::mem::transmute(mshlflags)).ok()
    }
    pub unsafe fn UnmarshalInterface<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pstm.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    pub unsafe fn ReleaseMarshalData<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>>(&self, pstm: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectObject(&self, dwreserved: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwreserved)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMarshal {
    type Vtable = IMarshal_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IMarshal> for ::windows::runtime::IUnknown {
    fn from(value: IMarshal) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMarshal> for ::windows::runtime::IUnknown {
    fn from(value: &IMarshal) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMarshal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMarshal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshal_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, pv: *const ::std::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::std::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, pv: *const ::std::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::std::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, pv: *const ::std::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::std::ffi::c_void, mshlflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwreserved: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMarshal2(::windows::runtime::IUnknown);
impl IMarshal2 {
    pub unsafe fn GetUnmarshalClass(&self, riid: *const ::windows::runtime::GUID, pv: *const ::std::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::std::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(pv), ::std::mem::transmute(dwdestcontext), ::std::mem::transmute(pvdestcontext), ::std::mem::transmute(mshlflags), ::std::mem::transmute(pcid)).ok()
    }
    pub unsafe fn GetMarshalSizeMax(&self, riid: *const ::windows::runtime::GUID, pv: *const ::std::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::std::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(pv), ::std::mem::transmute(dwdestcontext), ::std::mem::transmute(pvdestcontext), ::std::mem::transmute(mshlflags), ::std::mem::transmute(psize)).ok()
    }
    pub unsafe fn MarshalInterface<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows::runtime::GUID, pv: *const ::std::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::std::ffi::c_void, mshlflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pstm.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(pv), ::std::mem::transmute(dwdestcontext), ::std::mem::transmute(pvdestcontext), ::std::mem::transmute(mshlflags)).ok()
    }
    pub unsafe fn UnmarshalInterface<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>>(&self, pstm: Param0, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pstm.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    pub unsafe fn ReleaseMarshalData<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>>(&self, pstm: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pstm.into_param().abi()).ok()
    }
    pub unsafe fn DisconnectObject(&self, dwreserved: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwreserved)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMarshal2 {
    type Vtable = IMarshal2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(463, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IMarshal2> for ::windows::runtime::IUnknown {
    fn from(value: IMarshal2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMarshal2> for ::windows::runtime::IUnknown {
    fn from(value: &IMarshal2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMarshal2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMarshal2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMarshal2> for IMarshal {
    fn from(value: IMarshal2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMarshal2> for IMarshal {
    fn from(value: &IMarshal2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMarshal> for IMarshal2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMarshal> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMarshal>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMarshal> for &IMarshal2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMarshal> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMarshal>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshal2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, pv: *const ::std::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::std::ffi::c_void, mshlflags: u32, pcid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, pv: *const ::std::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::std::ffi::c_void, mshlflags: u32, psize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, pv: *const ::std::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::std::ffi::c_void, mshlflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwreserved: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMarshalingStream(::windows::runtime::IUnknown);
impl IMarshalingStream {
    pub unsafe fn Read(&self, pv: *mut ::std::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pv), ::std::mem::transmute(cb), ::std::mem::transmute(pcbread)).ok()
    }
    pub unsafe fn Write(&self, pv: *const ::std::ffi::c_void, cb: u32) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pv), ::std::mem::transmute(cb), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::STREAM_SEEK) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dlibmove), ::std::mem::transmute(dworigin), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(libnewsize)).ok()
    }
    pub unsafe fn CopyTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::IStream>>(&self, pstm: Param0, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pstm.into_param().abi(), ::std::mem::transmute(cb), ::std::mem::transmute(pcbread), ::std::mem::transmute(pcbwritten)).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(grfcommitflags)).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(liboffset), ::std::mem::transmute(cb), ::std::mem::transmute(dwlocktype)).ok()
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(liboffset), ::std::mem::transmute(cb), ::std::mem::transmute(dwlocktype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstatstg), ::std::mem::transmute(grfstatflag)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<super::IStream> {
        let mut result__: <super::IStream as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::IStream>(result__)
    }
    pub unsafe fn GetMarshalingContextAttribute(&self, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(attribute), &mut result__).from_abi::<usize>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMarshalingStream {
    type Vtable = IMarshalingStream_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3639801318, 24834, 18531, [159, 38, 56, 154, 70, 118, 239, 222]);
}
impl ::std::convert::From<IMarshalingStream> for ::windows::runtime::IUnknown {
    fn from(value: IMarshalingStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMarshalingStream> for ::windows::runtime::IUnknown {
    fn from(value: &IMarshalingStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMarshalingStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMarshalingStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMarshalingStream> for super::IStream {
    fn from(value: IMarshalingStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMarshalingStream> for super::IStream {
    fn from(value: &IMarshalingStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStream> for IMarshalingStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::IStream>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IStream> for &IMarshalingStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::IStream>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMarshalingStream> for super::ISequentialStream {
    fn from(value: IMarshalingStream) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMarshalingStream> for super::ISequentialStream {
    fn from(value: &IMarshalingStream) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ISequentialStream> for IMarshalingStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ISequentialStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ISequentialStream>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::ISequentialStream> for &IMarshalingStream {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::ISequentialStream> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::ISequentialStream>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMarshalingStream_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *mut ::std::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pv: *const ::std::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dlibmove: i64, dworigin: super::STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, libnewsize: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstm: ::windows::runtime::RawPtr, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, grfcommitflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppstm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attribute: super::CO_MARSHALING_CONTEXT_ATTRIBUTES, pattributevalue: *mut usize) -> ::windows::runtime::HRESULT,
);
#[inline]
pub unsafe fn LPSAFEARRAY_UserFree(param0: *const u32, param1: *const *const super::SAFEARRAY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserFree(param0: *const u32, param1: *const *const super::SAFEARRAY);
        }
        ::std::mem::transmute(LPSAFEARRAY_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserFree64(param0: *const u32, param1: *const *const super::SAFEARRAY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserFree64(param0: *const u32, param1: *const *const super::SAFEARRAY);
        }
        ::std::mem::transmute(LPSAFEARRAY_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8;
        }
        ::std::mem::transmute(LPSAFEARRAY_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8;
        }
        ::std::mem::transmute(LPSAFEARRAY_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserSize(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserSize(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32;
        }
        ::std::mem::transmute(LPSAFEARRAY_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserSize64(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserSize64(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32;
        }
        ::std::mem::transmute(LPSAFEARRAY_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8;
        }
        ::std::mem::transmute(LPSAFEARRAY_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LPSAFEARRAY_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LPSAFEARRAY_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8;
        }
        ::std::mem::transmute(LPSAFEARRAY_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserFree(param0: *const u32, param1: *const *const *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserFree(param0: *const u32, param1: *const *const *const u16);
        }
        ::std::mem::transmute(SNB_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserFree64(param0: *const u32, param1: *const *const *const u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserFree64(param0: *const u32, param1: *const *const *const u16);
        }
        ::std::mem::transmute(SNB_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8;
        }
        ::std::mem::transmute(SNB_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8;
        }
        ::std::mem::transmute(SNB_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserSize(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserSize(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32;
        }
        ::std::mem::transmute(SNB_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserSize64(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserSize64(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32;
        }
        ::std::mem::transmute(SNB_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8;
        }
        ::std::mem::transmute(SNB_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SNB_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SNB_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8;
        }
        ::std::mem::transmute(SNB_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct STDMSHLFLAGS(pub i32);
pub const SMEXF_SERVER: STDMSHLFLAGS = STDMSHLFLAGS(1i32);
pub const SMEXF_HANDLER: STDMSHLFLAGS = STDMSHLFLAGS(2i32);
impl ::std::convert::From<i32> for STDMSHLFLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STDMSHLFLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserFree(param0: *const u32, param1: *const super::STGMEDIUM) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserFree(param0: *const u32, param1: *const ::std::mem::ManuallyDrop<super::STGMEDIUM>);
        }
        ::std::mem::transmute(STGMEDIUM_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserFree64(param0: *const u32, param1: *const super::STGMEDIUM) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserFree64(param0: *const u32, param1: *const ::std::mem::ManuallyDrop<super::STGMEDIUM>);
        }
        ::std::mem::transmute(STGMEDIUM_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::std::mem::ManuallyDrop<super::STGMEDIUM>) -> *mut u8;
        }
        ::std::mem::transmute(STGMEDIUM_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::std::mem::ManuallyDrop<super::STGMEDIUM>) -> *mut u8;
        }
        ::std::mem::transmute(STGMEDIUM_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserSize(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserSize(param0: *const u32, param1: u32, param2: *const ::std::mem::ManuallyDrop<super::STGMEDIUM>) -> u32;
        }
        ::std::mem::transmute(STGMEDIUM_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserSize64(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserSize64(param0: *const u32, param1: u32, param2: *const ::std::mem::ManuallyDrop<super::STGMEDIUM>) -> u32;
        }
        ::std::mem::transmute(STGMEDIUM_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::std::mem::ManuallyDrop<super::STGMEDIUM>) -> *mut u8;
        }
        ::std::mem::transmute(STGMEDIUM_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn STGMEDIUM_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn STGMEDIUM_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::std::mem::ManuallyDrop<super::STGMEDIUM>) -> *mut u8;
        }
        ::std::mem::transmute(STGMEDIUM_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn VARIANT_UserFree(param0: *const u32, param1: *const super::VARIANT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserFree(param0: *const u32, param1: *const ::std::mem::ManuallyDrop<super::VARIANT>);
        }
        ::std::mem::transmute(VARIANT_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn VARIANT_UserFree64(param0: *const u32, param1: *const super::VARIANT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserFree64(param0: *const u32, param1: *const ::std::mem::ManuallyDrop<super::VARIANT>);
        }
        ::std::mem::transmute(VARIANT_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn VARIANT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::std::mem::ManuallyDrop<super::VARIANT>) -> *mut u8;
        }
        ::std::mem::transmute(VARIANT_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn VARIANT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::std::mem::ManuallyDrop<super::VARIANT>) -> *mut u8;
        }
        ::std::mem::transmute(VARIANT_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn VARIANT_UserSize(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserSize(param0: *const u32, param1: u32, param2: *const ::std::mem::ManuallyDrop<super::VARIANT>) -> u32;
        }
        ::std::mem::transmute(VARIANT_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn VARIANT_UserSize64(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserSize64(param0: *const u32, param1: u32, param2: *const ::std::mem::ManuallyDrop<super::VARIANT>) -> u32;
        }
        ::std::mem::transmute(VARIANT_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::std::mem::ManuallyDrop<super::VARIANT>) -> *mut u8;
        }
        ::std::mem::transmute(VARIANT_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole_Automation"))]
#[inline]
pub unsafe fn VARIANT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VARIANT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::std::mem::ManuallyDrop<super::VARIANT>) -> *mut u8;
        }
        ::std::mem::transmute(VARIANT_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
