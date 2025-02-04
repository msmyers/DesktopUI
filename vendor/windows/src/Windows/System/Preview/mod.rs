#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HingeState(pub i32);
impl HingeState {
    pub const Unknown: HingeState = HingeState(0i32);
    pub const Closed: HingeState = HingeState(1i32);
    pub const Concave: HingeState = HingeState(2i32);
    pub const Flat: HingeState = HingeState(3i32);
    pub const Convex: HingeState = HingeState(4i32);
    pub const Full: HingeState = HingeState(5i32);
}
impl ::std::convert::From<i32> for HingeState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HingeState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HingeState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.System.Preview.HingeState;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreview(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITwoPanelHingedDevicePosturePreview {
    type Vtable = ITwoPanelHingedDevicePosturePreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1914985521, 19257, 17062, [142, 115, 114, 53, 173, 225, 104, 83]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreview_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewReading(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITwoPanelHingedDevicePosturePreviewReading {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReading_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2686784594, 19158, 19256, [132, 38, 197, 154, 21, 73, 58, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewReading_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HingeState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Sensors")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Devices::Sensors::SimpleOrientation) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Sensors"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_Sensors")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Devices::Sensors::SimpleOrientation) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_Sensors"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(757930950, 718, 18250, [165, 86, 167, 91, 28, 249, 58, 3]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITwoPanelHingedDevicePosturePreviewStatics {
    type Vtable = ITwoPanelHingedDevicePosturePreviewStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(205992914, 22496, 16768, [189, 94, 243, 26, 33, 56, 66, 62]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITwoPanelHingedDevicePosturePreviewStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TwoPanelHingedDevicePosturePreview(::windows::runtime::IInspectable);
impl TwoPanelHingedDevicePosturePreview {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentPostureAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreviewReading>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreviewReading>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn PostureChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<TwoPanelHingedDevicePosturePreview, TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn RemovePostureChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreview>> {
        Self::ITwoPanelHingedDevicePosturePreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreview>>(result__)
        })
    }
    pub fn ITwoPanelHingedDevicePosturePreviewStatics<R, F: FnOnce(&ITwoPanelHingedDevicePosturePreviewStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TwoPanelHingedDevicePosturePreview, ITwoPanelHingedDevicePosturePreviewStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TwoPanelHingedDevicePosturePreview {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Preview.TwoPanelHingedDevicePosturePreview;{72245c31-4b39-42a6-8e73-7235ade16853})");
}
unsafe impl ::windows::runtime::Interface for TwoPanelHingedDevicePosturePreview {
    type Vtable = ITwoPanelHingedDevicePosturePreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1914985521, 19257, 17062, [142, 115, 114, 53, 173, 225, 104, 83]);
}
impl ::windows::runtime::RuntimeName for TwoPanelHingedDevicePosturePreview {
    const NAME: &'static str = "Windows.System.Preview.TwoPanelHingedDevicePosturePreview";
}
impl ::std::convert::From<TwoPanelHingedDevicePosturePreview> for ::windows::runtime::IUnknown {
    fn from(value: TwoPanelHingedDevicePosturePreview) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TwoPanelHingedDevicePosturePreview> for ::windows::runtime::IUnknown {
    fn from(value: &TwoPanelHingedDevicePosturePreview) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<TwoPanelHingedDevicePosturePreview> for ::windows::runtime::IInspectable {
    fn from(value: TwoPanelHingedDevicePosturePreview) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TwoPanelHingedDevicePosturePreview> for ::windows::runtime::IInspectable {
    fn from(value: &TwoPanelHingedDevicePosturePreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TwoPanelHingedDevicePosturePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TwoPanelHingedDevicePosturePreview {}
unsafe impl ::std::marker::Sync for TwoPanelHingedDevicePosturePreview {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TwoPanelHingedDevicePosturePreviewReading(::windows::runtime::IInspectable);
impl TwoPanelHingedDevicePosturePreviewReading {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn HingeState(&self) -> ::windows::runtime::Result<HingeState> {
        let this = self;
        unsafe {
            let mut result__: HingeState = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HingeState>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Devices_Sensors")]
    pub fn Panel1Orientation(&self) -> ::windows::runtime::Result<super::super::Devices::Sensors::SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Sensors::SimpleOrientation = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Sensors::SimpleOrientation>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Panel1Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Devices_Sensors")]
    pub fn Panel2Orientation(&self) -> ::windows::runtime::Result<super::super::Devices::Sensors::SimpleOrientation> {
        let this = self;
        unsafe {
            let mut result__: super::super::Devices::Sensors::SimpleOrientation = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Devices::Sensors::SimpleOrientation>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    pub fn Panel2Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TwoPanelHingedDevicePosturePreviewReading {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading;{a0251452-4ad6-4b38-8426-c59a15493a7d})");
}
unsafe impl ::windows::runtime::Interface for TwoPanelHingedDevicePosturePreviewReading {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReading_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2686784594, 19158, 19256, [132, 38, 197, 154, 21, 73, 58, 125]);
}
impl ::windows::runtime::RuntimeName for TwoPanelHingedDevicePosturePreviewReading {
    const NAME: &'static str = "Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReading";
}
impl ::std::convert::From<TwoPanelHingedDevicePosturePreviewReading> for ::windows::runtime::IUnknown {
    fn from(value: TwoPanelHingedDevicePosturePreviewReading) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TwoPanelHingedDevicePosturePreviewReading> for ::windows::runtime::IUnknown {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReading) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<TwoPanelHingedDevicePosturePreviewReading> for ::windows::runtime::IInspectable {
    fn from(value: TwoPanelHingedDevicePosturePreviewReading) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TwoPanelHingedDevicePosturePreviewReading> for ::windows::runtime::IInspectable {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReading) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TwoPanelHingedDevicePosturePreviewReading {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TwoPanelHingedDevicePosturePreviewReading {}
unsafe impl ::std::marker::Sync for TwoPanelHingedDevicePosturePreviewReading {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs(::windows::runtime::IInspectable);
impl TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    #[cfg(feature = "deprecated")]
    pub fn Reading(&self) -> ::windows::runtime::Result<TwoPanelHingedDevicePosturePreviewReading> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TwoPanelHingedDevicePosturePreviewReading>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs;{2d2d1bc6-02ce-474a-a556-a75b1cf93a03})");
}
unsafe impl ::windows::runtime::Interface for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    type Vtable = ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(757930950, 718, 18250, [165, 86, 167, 91, 28, 249, 58, 3]);
}
impl ::windows::runtime::RuntimeName for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    const NAME: &'static str = "Windows.System.Preview.TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs";
}
impl ::std::convert::From<TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {}
unsafe impl ::std::marker::Sync for TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs {}
