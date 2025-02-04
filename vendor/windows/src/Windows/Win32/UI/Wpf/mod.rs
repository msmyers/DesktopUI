#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_MILBitmapEffectBevel: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4248182206, 27803, 19936, [130, 144, 246, 64, 12, 39, 55, 237]);
pub const CLSID_MILBitmapEffectBlur: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2837766023, 8797, 17267, [143, 91, 185, 14, 200, 90, 227, 222]);
pub const CLSID_MILBitmapEffectDropShadow: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1167736766, 55468, 18066, [135, 75, 122, 38, 87, 21, 170, 22]);
pub const CLSID_MILBitmapEffectEmboss: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3442055238, 33359, 18412, [160, 7, 18, 170, 118, 127, 40, 22]);
pub const CLSID_MILBitmapEffectGroup: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2895911578, 32280, 20324, [172, 126, 71, 207, 127, 5, 30, 149]);
pub const CLSID_MILBitmapEffectOuterGlow: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3793099741, 32438, 18213, [156, 11, 138, 42, 27, 79, 6, 103]);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffect(::windows::runtime::IUnknown);
impl IMILBitmapEffect {
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutput<'a, Param1: ::windows::runtime::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, pcontext: Param1) -> ::windows::runtime::Result<super::super::Graphics::Imaging::IWICBitmapSource> {
        let mut result__: <super::super::Graphics::Imaging::IWICBitmapSource as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), pcontext.into_param().abi(), &mut result__).from_abi::<super::super::Graphics::Imaging::IWICBitmapSource>(result__)
    }
    pub unsafe fn GetParentEffect(&self) -> ::windows::runtime::Result<IMILBitmapEffectGroup> {
        let mut result__: <IMILBitmapEffectGroup as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffectGroup>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn SetInputSource<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::IWICBitmapSource>>(&self, uiindex: u32, pbitmapsource: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), pbitmapsource.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffect {
    type Vtable = IMILBitmapEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2322592545, 51524, 18971, [153, 68, 153, 84, 175, 48, 18, 88]);
}
impl ::std::convert::From<IMILBitmapEffect> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, pcontext: ::windows::runtime::RawPtr, ppbitmapsource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppparenteffect: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, pbitmapsource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectConnections(::windows::runtime::IUnknown);
impl IMILBitmapEffectConnections {
    pub unsafe fn GetInputConnector(&self, uiindex: u32) -> ::windows::runtime::Result<IMILBitmapEffectInputConnector> {
        let mut result__: <IMILBitmapEffectInputConnector as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), &mut result__).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
    pub unsafe fn GetOutputConnector(&self, uiindex: u32) -> ::windows::runtime::Result<IMILBitmapEffectOutputConnector> {
        let mut result__: <IMILBitmapEffectOutputConnector as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), &mut result__).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectConnections {
    type Vtable = IMILBitmapEffectConnections_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3266697313, 39706, 17268, [137, 176, 222, 196, 135, 77, 106, 129]);
}
impl ::std::convert::From<IMILBitmapEffectConnections> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectConnections) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectConnections> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectConnections) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectConnections {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectConnections {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnections_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, ppconnector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, ppconnector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectConnectionsInfo(::windows::runtime::IUnknown);
impl IMILBitmapEffectConnectionsInfo {
    pub unsafe fn GetNumberInputs(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetNumberOutputs(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetInputConnectorInfo(&self, uiindex: u32) -> ::windows::runtime::Result<IMILBitmapEffectConnectorInfo> {
        let mut result__: <IMILBitmapEffectConnectorInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), &mut result__).from_abi::<IMILBitmapEffectConnectorInfo>(result__)
    }
    pub unsafe fn GetOutputConnectorInfo(&self, uiindex: u32) -> ::windows::runtime::Result<IMILBitmapEffectConnectorInfo> {
        let mut result__: <IMILBitmapEffectConnectorInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), &mut result__).from_abi::<IMILBitmapEffectConnectorInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectConnectionsInfo {
    type Vtable = IMILBitmapEffectConnectionsInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1198216074, 51045, 16951, [186, 74, 214, 168, 128, 255, 12, 252]);
}
impl ::std::convert::From<IMILBitmapEffectConnectionsInfo> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectConnectionsInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectConnectionsInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectConnectionsInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectConnectionsInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectConnectionsInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnectionsInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puinuminputs: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puinumoutputs: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, ppconnectorinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, ppconnectorinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectConnector(::windows::runtime::IUnknown);
impl IMILBitmapEffectConnector {
    pub unsafe fn GetIndex(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulindex), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows::runtime::Result<IMILBitmapEffect> {
        let mut result__: <IMILBitmapEffect as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffect>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectConnector {
    type Vtable = IMILBitmapEffectConnector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4120209331, 30401, 19783, [186, 30, 121, 249, 85, 227, 80, 239]);
}
impl ::std::convert::From<IMILBitmapEffectConnector> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectConnector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectConnector> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectConnector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMILBitmapEffectConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: IMILBitmapEffectConnector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: &IMILBitmapEffectConnector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMILBitmapEffectConnectorInfo> for IMILBitmapEffectConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMILBitmapEffectConnectorInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMILBitmapEffectConnectorInfo>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMILBitmapEffectConnectorInfo> for &IMILBitmapEffectConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMILBitmapEffectConnectorInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMILBitmapEffectConnectorInfo>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puiindex: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformat: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulnumberformats: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulindex: u32, pformat: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfconnected: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppeffect: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectConnectorInfo(::windows::runtime::IUnknown);
impl IMILBitmapEffectConnectorInfo {
    pub unsafe fn GetIndex(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulindex), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectConnectorInfo {
    type Vtable = IMILBitmapEffectConnectorInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4134350411, 46187, 17148, [133, 158, 61, 160, 236, 219, 60, 67]);
}
impl ::std::convert::From<IMILBitmapEffectConnectorInfo> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectConnectorInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectConnectorInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectConnectorInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectConnectorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectConnectorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectConnectorInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puiindex: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformat: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulnumberformats: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulindex: u32, pformat: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectEvents(::windows::runtime::IUnknown);
impl IMILBitmapEffectEvents {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PropertyChange<'a, Param0: ::windows::runtime::IntoParam<'a, IMILBitmapEffect>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, peffect: Param0, bstrpropertyname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), peffect.into_param().abi(), bstrpropertyname.into_param().abi()).ok()
    }
    pub unsafe fn DirtyRegion<'a, Param0: ::windows::runtime::IntoParam<'a, IMILBitmapEffect>>(&self, peffect: Param0, prect: *const MilRectD) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), peffect.into_param().abi(), ::std::mem::transmute(prect)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectEvents {
    type Vtable = IMILBitmapEffectEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(780668376, 63694, 17787, [129, 153, 214, 11, 179, 215, 239, 152]);
}
impl ::std::convert::From<IMILBitmapEffectEvents> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectEvents> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, peffect: ::windows::runtime::RawPtr, bstrpropertyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, peffect: ::windows::runtime::RawPtr, prect: *const MilRectD) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectFactory(::windows::runtime::IUnknown);
impl IMILBitmapEffectFactory {
    pub unsafe fn CreateEffect(&self, pguideffect: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IMILBitmapEffect> {
        let mut result__: <IMILBitmapEffect as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguideffect), &mut result__).from_abi::<IMILBitmapEffect>(result__)
    }
    pub unsafe fn CreateContext(&self) -> ::windows::runtime::Result<IMILBitmapEffectRenderContext> {
        let mut result__: <IMILBitmapEffectRenderContext as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffectRenderContext>(result__)
    }
    pub unsafe fn CreateEffectOuter(&self) -> ::windows::runtime::Result<IMILBitmapEffect> {
        let mut result__: <IMILBitmapEffect as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffect>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectFactory {
    type Vtable = IMILBitmapEffectFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(866770740, 41987, 20167, [176, 126, 188, 6, 130, 55, 8, 69]);
}
impl ::std::convert::From<IMILBitmapEffectFactory> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguideffect: *const ::windows::runtime::GUID, ppeffect: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppeffect: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectGroup(::windows::runtime::IUnknown);
impl IMILBitmapEffectGroup {
    pub unsafe fn GetInteriorInputConnector(&self, uiindex: u32) -> ::windows::runtime::Result<IMILBitmapEffectOutputConnector> {
        let mut result__: <IMILBitmapEffectOutputConnector as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), &mut result__).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
    pub unsafe fn GetInteriorOutputConnector(&self, uiindex: u32) -> ::windows::runtime::Result<IMILBitmapEffectInputConnector> {
        let mut result__: <IMILBitmapEffectInputConnector as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), &mut result__).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, IMILBitmapEffect>>(&self, peffect: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), peffect.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectGroup {
    type Vtable = IMILBitmapEffectGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(798303072, 27018, 19142, [129, 161, 188, 253, 240, 142, 184, 232]);
}
impl ::std::convert::From<IMILBitmapEffectGroup> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectGroup> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, ppconnector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, ppconnector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, peffect: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectGroupImpl(::windows::runtime::IUnknown);
impl IMILBitmapEffectGroupImpl {
    pub unsafe fn Preprocess<'a, Param0: ::windows::runtime::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, pcontext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pcontext.into_param().abi()).ok()
    }
    pub unsafe fn GetNumberChildren(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetChildren(&self) -> ::windows::runtime::Result<IMILBitmapEffects> {
        let mut result__: <IMILBitmapEffects as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffects>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectGroupImpl {
    type Vtable = IMILBitmapEffectGroupImpl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2029966616, 7420, 18439, [139, 133, 107, 110, 81, 57, 143, 98]);
}
impl ::std::convert::From<IMILBitmapEffectGroupImpl> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectGroupImpl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectGroupImpl> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectGroupImpl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectGroupImpl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectGroupImpl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectGroupImpl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontext: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puinumberchildren: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchildren: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectImpl(::windows::runtime::IUnknown);
impl IMILBitmapEffectImpl {
    pub unsafe fn IsInPlaceModificationAllowed<'a, Param0: ::windows::runtime::IntoParam<'a, IMILBitmapEffectOutputConnector>>(&self, poutputconnector: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), poutputconnector.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetParentEffect<'a, Param0: ::windows::runtime::IntoParam<'a, IMILBitmapEffectGroup>>(&self, pparenteffect: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pparenteffect.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetInputSource(&self, uiindex: u32) -> ::windows::runtime::Result<super::super::Graphics::Imaging::IWICBitmapSource> {
        let mut result__: <super::super::Graphics::Imaging::IWICBitmapSource as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), &mut result__).from_abi::<super::super::Graphics::Imaging::IWICBitmapSource>(result__)
    }
    pub unsafe fn GetInputSourceBounds(&self, uiindex: u32) -> ::windows::runtime::Result<MilRectD> {
        let mut result__: <MilRectD as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), &mut result__).from_abi::<MilRectD>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetInputBitmapSource<'a, Param1: ::windows::runtime::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, prendercontext: Param1, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::std::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), prendercontext.into_param().abi(), ::std::mem::transmute(pfmodifyinplace), ::std::mem::transmute(ppbitmapsource)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutputBitmapSource<'a, Param1: ::windows::runtime::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, prendercontext: Param1, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::std::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), prendercontext.into_param().abi(), ::std::mem::transmute(pfmodifyinplace), ::std::mem::transmute(ppbitmapsource)).ok()
    }
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pinner: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pinner.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectImpl {
    type Vtable = IMILBitmapEffectImpl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3424938226, 39222, 18366, [180, 175, 6, 181, 223, 93, 188, 187]);
}
impl ::std::convert::From<IMILBitmapEffectImpl> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectImpl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectImpl> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectImpl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectImpl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectImpl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectImpl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poutputconnector: ::windows::runtime::RawPtr, pfmodifyinplace: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pparenteffect: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, ppbitmapsource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, prect: *mut MilRectD) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, prendercontext: ::windows::runtime::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))] usize,
    #[cfg(feature = "Win32_Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, prendercontext: ::windows::runtime::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinner: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectInputConnector(::windows::runtime::IUnknown);
impl IMILBitmapEffectInputConnector {
    pub unsafe fn GetIndex(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulindex), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows::runtime::Result<IMILBitmapEffect> {
        let mut result__: <IMILBitmapEffect as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffect>(result__)
    }
    pub unsafe fn ConnectTo<'a, Param0: ::windows::runtime::IntoParam<'a, IMILBitmapEffectOutputConnector>>(&self, pconnector: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pconnector.into_param().abi()).ok()
    }
    pub unsafe fn GetConnection(&self) -> ::windows::runtime::Result<IMILBitmapEffectOutputConnector> {
        let mut result__: <IMILBitmapEffectOutputConnector as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectInputConnector {
    type Vtable = IMILBitmapEffectInputConnector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2847206570, 31292, 17895, [133, 115, 244, 184, 27, 96, 221, 108]);
}
impl ::std::convert::From<IMILBitmapEffectInputConnector> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectInputConnector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectInputConnector> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectInputConnector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMILBitmapEffectInputConnector> for IMILBitmapEffectConnector {
    fn from(value: IMILBitmapEffectInputConnector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectInputConnector> for IMILBitmapEffectConnector {
    fn from(value: &IMILBitmapEffectInputConnector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMILBitmapEffectConnector> for IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMILBitmapEffectConnector> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMILBitmapEffectConnector>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMILBitmapEffectConnector> for &IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMILBitmapEffectConnector> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMILBitmapEffectConnector>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMILBitmapEffectInputConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: IMILBitmapEffectInputConnector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectInputConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: &IMILBitmapEffectInputConnector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMILBitmapEffectConnectorInfo> for IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMILBitmapEffectConnectorInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMILBitmapEffectConnectorInfo>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMILBitmapEffectConnectorInfo> for &IMILBitmapEffectInputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMILBitmapEffectConnectorInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMILBitmapEffectConnectorInfo>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInputConnector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puiindex: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformat: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulnumberformats: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulindex: u32, pformat: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfconnected: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppeffect: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnector: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppconnector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectInteriorInputConnector(::windows::runtime::IUnknown);
impl IMILBitmapEffectInteriorInputConnector {
    pub unsafe fn GetInputConnector(&self) -> ::windows::runtime::Result<IMILBitmapEffectInputConnector> {
        let mut result__: <IMILBitmapEffectInputConnector as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectInteriorInputConnector {
    type Vtable = IMILBitmapEffectInteriorInputConnector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(539524766, 34466, 19989, [149, 61, 235, 20, 56, 165, 184, 66]);
}
impl ::std::convert::From<IMILBitmapEffectInteriorInputConnector> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectInteriorInputConnector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectInteriorInputConnector> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectInteriorInputConnector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectInteriorInputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectInteriorInputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInteriorInputConnector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinputconnector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectInteriorOutputConnector(::windows::runtime::IUnknown);
impl IMILBitmapEffectInteriorOutputConnector {
    pub unsafe fn GetOutputConnector(&self) -> ::windows::runtime::Result<IMILBitmapEffectOutputConnector> {
        let mut result__: <IMILBitmapEffectOutputConnector as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffectOutputConnector>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectInteriorOutputConnector {
    type Vtable = IMILBitmapEffectInteriorOutputConnector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(12302044, 44233, 19452, [179, 68, 139, 238, 56, 61, 254, 250]);
}
impl ::std::convert::From<IMILBitmapEffectInteriorOutputConnector> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectInteriorOutputConnector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectInteriorOutputConnector> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectInteriorOutputConnector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectInteriorOutputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectInteriorOutputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectInteriorOutputConnector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, poutputconnector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectOutputConnector(::windows::runtime::IUnknown);
impl IMILBitmapEffectOutputConnector {
    pub unsafe fn GetIndex(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetOptimalFormat(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn GetNumberFormats(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetFormat(&self, ulindex: u32) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulindex), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn IsConnected(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn GetBitmapEffect(&self) -> ::windows::runtime::Result<IMILBitmapEffect> {
        let mut result__: <IMILBitmapEffect as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffect>(result__)
    }
    pub unsafe fn GetNumberConnections(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn GetConnection(&self, uiindex: u32) -> ::windows::runtime::Result<IMILBitmapEffectInputConnector> {
        let mut result__: <IMILBitmapEffectInputConnector as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), &mut result__).from_abi::<IMILBitmapEffectInputConnector>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectOutputConnector {
    type Vtable = IMILBitmapEffectOutputConnector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2459269805, 33819, 18534, [130, 236, 135, 82, 70, 139, 7, 253]);
}
impl ::std::convert::From<IMILBitmapEffectOutputConnector> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectOutputConnector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectOutputConnector> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectOutputConnector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMILBitmapEffectOutputConnector> for IMILBitmapEffectConnector {
    fn from(value: IMILBitmapEffectOutputConnector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectOutputConnector> for IMILBitmapEffectConnector {
    fn from(value: &IMILBitmapEffectOutputConnector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMILBitmapEffectConnector> for IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMILBitmapEffectConnector> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMILBitmapEffectConnector>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMILBitmapEffectConnector> for &IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMILBitmapEffectConnector> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMILBitmapEffectConnector>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IMILBitmapEffectOutputConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: IMILBitmapEffectOutputConnector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectOutputConnector> for IMILBitmapEffectConnectorInfo {
    fn from(value: &IMILBitmapEffectOutputConnector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMILBitmapEffectConnectorInfo> for IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMILBitmapEffectConnectorInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMILBitmapEffectConnectorInfo>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMILBitmapEffectConnectorInfo> for &IMILBitmapEffectOutputConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMILBitmapEffectConnectorInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMILBitmapEffectConnectorInfo>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectOutputConnector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puiindex: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformat: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulnumberformats: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulindex: u32, pformat: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfconnected: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppeffect: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puinumberconnections: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, ppconnection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectOutputConnectorImpl(::windows::runtime::IUnknown);
impl IMILBitmapEffectOutputConnectorImpl {
    pub unsafe fn AddBackLink<'a, Param0: ::windows::runtime::IntoParam<'a, IMILBitmapEffectInputConnector>>(&self, pconnection: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pconnection.into_param().abi()).ok()
    }
    pub unsafe fn RemoveBackLink<'a, Param0: ::windows::runtime::IntoParam<'a, IMILBitmapEffectInputConnector>>(&self, pconnection: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pconnection.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectOutputConnectorImpl {
    type Vtable = IMILBitmapEffectOutputConnectorImpl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(570091383, 35641, 19450, [159, 45, 243, 148, 30, 211, 105, 19]);
}
impl ::std::convert::From<IMILBitmapEffectOutputConnectorImpl> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectOutputConnectorImpl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectOutputConnectorImpl> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectOutputConnectorImpl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectOutputConnectorImpl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectOutputConnectorImpl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectOutputConnectorImpl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnection: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconnection: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectPrimitive(::windows::runtime::IUnknown);
impl IMILBitmapEffectPrimitive {
    #[cfg(feature = "Win32_Graphics_Imaging")]
    pub unsafe fn GetOutput<'a, Param1: ::windows::runtime::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, pcontext: Param1, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::std::option::Option<super::super::Graphics::Imaging::IWICBitmapSource>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), pcontext.into_param().abi(), ::std::mem::transmute(pfmodifyinplace), ::std::mem::transmute(ppbitmapsource)).ok()
    }
    pub unsafe fn TransformPoint<'a, Param3: ::windows::runtime::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: i16, pcontext: Param3, pfpointtransformed: *mut i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), ::std::mem::transmute(p), ::std::mem::transmute(fforwardtransform), pcontext.into_param().abi(), ::std::mem::transmute(pfpointtransformed)).ok()
    }
    pub unsafe fn TransformRect<'a, Param3: ::windows::runtime::IntoParam<'a, IMILBitmapEffectRenderContext>>(&self, uiindex: u32, p: *mut MilRectD, fforwardtransform: i16, pcontext: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), ::std::mem::transmute(p), ::std::mem::transmute(fforwardtransform), pcontext.into_param().abi()).ok()
    }
    pub unsafe fn HasAffineTransform(&self, uiindex: u32) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn HasInverseTransform(&self, uiindex: u32) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dwm")]
    pub unsafe fn GetAffineMatrix(&self, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(uiindex), ::std::mem::transmute(pmatrix)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectPrimitive {
    type Vtable = IMILBitmapEffectPrimitive_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1742934053, 12433, 19964, [152, 214, 221, 73, 69, 81, 70, 29]);
}
impl ::std::convert::From<IMILBitmapEffectPrimitive> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectPrimitive) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectPrimitive> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectPrimitive) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectPrimitive {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectPrimitive {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectPrimitive_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, pcontext: ::windows::runtime::RawPtr, pfmodifyinplace: *mut i16, ppbitmapsource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, p: *mut MilPoint2D, fforwardtransform: i16, pcontext: ::windows::runtime::RawPtr, pfpointtransformed: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, p: *mut MilRectD, fforwardtransform: i16, pcontext: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, pfaffine: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, pfhasinverse: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dwm")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiindex: u32, pmatrix: *mut super::super::Graphics::Dwm::MilMatrix3x2D) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dwm"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectPrimitiveImpl(::windows::runtime::IUnknown);
impl IMILBitmapEffectPrimitiveImpl {
    pub unsafe fn IsDirty(&self, uioutputindex: u32) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(uioutputindex), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn IsVolatile(&self, uioutputindex: u32) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(uioutputindex), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectPrimitiveImpl {
    type Vtable = IMILBitmapEffectPrimitiveImpl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3460423691, 61350, 17639, [176, 7, 221, 4, 46, 58, 225, 38]);
}
impl ::std::convert::From<IMILBitmapEffectPrimitiveImpl> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectPrimitiveImpl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectPrimitiveImpl> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectPrimitiveImpl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectPrimitiveImpl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectPrimitiveImpl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectPrimitiveImpl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uioutputindex: u32, pfdirty: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uioutputindex: u32, pfvolatile: *mut i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectRenderContext(::windows::runtime::IUnknown);
impl IMILBitmapEffectRenderContext {
    pub unsafe fn SetOutputPixelFormat(&self, format: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(format)).ok()
    }
    pub unsafe fn GetOutputPixelFormat(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn SetUseSoftwareRenderer(&self, fsoftware: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(fsoftware)).ok()
    }
    pub unsafe fn SetInitialTransform(&self, pmatrix: *const MILMatrixF) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmatrix)).ok()
    }
    pub unsafe fn GetFinalTransform(&self) -> ::windows::runtime::Result<MILMatrixF> {
        let mut result__: <MILMatrixF as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<MILMatrixF>(result__)
    }
    pub unsafe fn SetOutputDPI(&self, dbldpix: f64, dbldpiy: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dbldpix), ::std::mem::transmute(dbldpiy)).ok()
    }
    pub unsafe fn GetOutputDPI(&self, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdbldpix), ::std::mem::transmute(pdbldpiy)).ok()
    }
    pub unsafe fn SetRegionOfInterest(&self, prect: *const MilRectD) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(prect)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectRenderContext {
    type Vtable = IMILBitmapEffectRenderContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(312667262, 11571, 17586, [179, 52, 26, 187, 120, 70, 227, 144]);
}
impl ::std::convert::From<IMILBitmapEffectRenderContext> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectRenderContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectRenderContext> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectRenderContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectRenderContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectRenderContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectRenderContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pformat: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fsoftware: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmatrix: *const MILMatrixF) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmatrix: *mut MILMatrixF) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dbldpix: f64, dbldpiy: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdbldpix: *mut f64, pdbldpiy: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prect: *const MilRectD) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffectRenderContextImpl(::windows::runtime::IUnknown);
impl IMILBitmapEffectRenderContextImpl {
    pub unsafe fn GetUseSoftwareRenderer(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn GetTransform(&self, pmatrix: *mut MILMatrixF) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmatrix)).ok()
    }
    pub unsafe fn UpdateTransform(&self, pmatrix: *const MILMatrixF) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pmatrix)).ok()
    }
    pub unsafe fn GetOutputBounds(&self, prect: *mut MilRectD) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(prect)).ok()
    }
    pub unsafe fn UpdateOutputBounds(&self, prect: *const MilRectD) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(prect)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffectRenderContextImpl {
    type Vtable = IMILBitmapEffectRenderContextImpl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1294314699, 31101, 20434, [177, 40, 223, 254, 255, 132, 252, 195]);
}
impl ::std::convert::From<IMILBitmapEffectRenderContextImpl> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffectRenderContextImpl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffectRenderContextImpl> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffectRenderContextImpl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffectRenderContextImpl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffectRenderContextImpl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffectRenderContextImpl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfsoftware: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmatrix: *mut MILMatrixF) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmatrix: *const MILMatrixF) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prect: *mut MilRectD) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prect: *const MilRectD) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMILBitmapEffects(::windows::runtime::IUnknown);
impl IMILBitmapEffects {
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    pub unsafe fn Parent(&self) -> ::windows::runtime::Result<IMILBitmapEffectGroup> {
        let mut result__: <IMILBitmapEffectGroup as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IMILBitmapEffectGroup>(result__)
    }
    pub unsafe fn Item(&self, uindex: u32) -> ::windows::runtime::Result<IMILBitmapEffect> {
        let mut result__: <IMILBitmapEffect as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(uindex), &mut result__).from_abi::<IMILBitmapEffect>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMILBitmapEffects {
    type Vtable = IMILBitmapEffects_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1370242510, 26565, 17547, [145, 128, 173, 62, 171, 221, 213, 221]);
}
impl ::std::convert::From<IMILBitmapEffects> for ::windows::runtime::IUnknown {
    fn from(value: IMILBitmapEffects) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IMILBitmapEffects> for ::windows::runtime::IUnknown {
    fn from(value: &IMILBitmapEffects) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMILBitmapEffects {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IMILBitmapEffects {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMILBitmapEffects_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppiureturn: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppeffect: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uindex: u32, ppeffect: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puicount: *mut u32) -> ::windows::runtime::HRESULT,
);
pub const MILBITMAPEFFECT_SDK_VERSION: u32 = 16777216u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MILMatrixF {
    pub _11: f64,
    pub _12: f64,
    pub _13: f64,
    pub _14: f64,
    pub _21: f64,
    pub _22: f64,
    pub _23: f64,
    pub _24: f64,
    pub _31: f64,
    pub _32: f64,
    pub _33: f64,
    pub _34: f64,
    pub _41: f64,
    pub _42: f64,
    pub _43: f64,
    pub _44: f64,
}
impl MILMatrixF {}
impl ::std::default::Default for MILMatrixF {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MILMatrixF {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MILMatrixF")
            .field("_11", &self._11)
            .field("_12", &self._12)
            .field("_13", &self._13)
            .field("_14", &self._14)
            .field("_21", &self._21)
            .field("_22", &self._22)
            .field("_23", &self._23)
            .field("_24", &self._24)
            .field("_31", &self._31)
            .field("_32", &self._32)
            .field("_33", &self._33)
            .field("_34", &self._34)
            .field("_41", &self._41)
            .field("_42", &self._42)
            .field("_43", &self._43)
            .field("_44", &self._44)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MILMatrixF {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11 && self._12 == other._12 && self._13 == other._13 && self._14 == other._14 && self._21 == other._21 && self._22 == other._22 && self._23 == other._23 && self._24 == other._24 && self._31 == other._31 && self._32 == other._32 && self._33 == other._33 && self._34 == other._34 && self._41 == other._41 && self._42 == other._42 && self._43 == other._43 && self._44 == other._44
    }
}
impl ::std::cmp::Eq for MILMatrixF {}
unsafe impl ::windows::runtime::Abi for MILMatrixF {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MilPoint2D {
    pub X: f64,
    pub Y: f64,
}
impl MilPoint2D {}
impl ::std::default::Default for MilPoint2D {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MilPoint2D {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MilPoint2D").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::std::cmp::PartialEq for MilPoint2D {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::std::cmp::Eq for MilPoint2D {}
unsafe impl ::windows::runtime::Abi for MilPoint2D {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MilRectD {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}
impl MilRectD {}
impl ::std::default::Default for MilRectD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MilRectD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MilRectD").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
impl ::std::cmp::PartialEq for MilRectD {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.right == other.right && self.bottom == other.bottom
    }
}
impl ::std::cmp::Eq for MilRectD {}
unsafe impl ::windows::runtime::Abi for MilRectD {
    type Abi = Self;
    type DefaultType = Self;
}
