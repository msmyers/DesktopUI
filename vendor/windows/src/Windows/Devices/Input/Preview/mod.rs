#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GazeDeviceConfigurationStatePreview(pub i32);
impl GazeDeviceConfigurationStatePreview {
    pub const Unknown: GazeDeviceConfigurationStatePreview = GazeDeviceConfigurationStatePreview(0i32);
    pub const Ready: GazeDeviceConfigurationStatePreview = GazeDeviceConfigurationStatePreview(1i32);
    pub const Configuring: GazeDeviceConfigurationStatePreview = GazeDeviceConfigurationStatePreview(2i32);
    pub const ScreenSetupNeeded: GazeDeviceConfigurationStatePreview = GazeDeviceConfigurationStatePreview(3i32);
    pub const UserCalibrationNeeded: GazeDeviceConfigurationStatePreview = GazeDeviceConfigurationStatePreview(4i32);
}
impl ::std::convert::From<i32> for GazeDeviceConfigurationStatePreview {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GazeDeviceConfigurationStatePreview {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for GazeDeviceConfigurationStatePreview {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Input.Preview.GazeDeviceConfigurationStatePreview;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GazeDevicePreview(::windows::runtime::IInspectable);
impl GazeDevicePreview {
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn CanTrackEyes(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CanTrackHead(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ConfigurationState(&self) -> ::windows::runtime::Result<GazeDeviceConfigurationStatePreview> {
        let this = self;
        unsafe {
            let mut result__: GazeDeviceConfigurationStatePreview = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GazeDeviceConfigurationStatePreview>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestCalibrationAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub fn GetNumericControlDescriptions(&self, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidNumericControlDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidNumericControlDescription>>(result__)
        }
    }
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))]
    pub fn GetBooleanControlDescriptions(&self, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidBooleanControlDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidBooleanControlDescription>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GazeDevicePreview {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDevicePreview;{e79e7ee9-b389-11e7-b201-c8d3ffb75721})");
}
unsafe impl ::windows::runtime::Interface for GazeDevicePreview {
    type Vtable = IGazeDevicePreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3885924073, 45961, 4583, [178, 1, 200, 211, 255, 183, 87, 33]);
}
impl ::windows::runtime::RuntimeName for GazeDevicePreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDevicePreview";
}
impl ::std::convert::From<GazeDevicePreview> for ::windows::runtime::IUnknown {
    fn from(value: GazeDevicePreview) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GazeDevicePreview> for ::windows::runtime::IUnknown {
    fn from(value: &GazeDevicePreview) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GazeDevicePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GazeDevicePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GazeDevicePreview> for ::windows::runtime::IInspectable {
    fn from(value: GazeDevicePreview) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GazeDevicePreview> for ::windows::runtime::IInspectable {
    fn from(value: &GazeDevicePreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GazeDevicePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GazeDevicePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GazeDevicePreview {}
unsafe impl ::std::marker::Sync for GazeDevicePreview {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GazeDeviceWatcherAddedPreviewEventArgs(::windows::runtime::IInspectable);
impl GazeDeviceWatcherAddedPreviewEventArgs {
    pub fn Device(&self) -> ::windows::runtime::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GazeDevicePreview>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GazeDeviceWatcherAddedPreviewEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs;{e79e7eed-b389-11e7-b201-c8d3ffb75721})");
}
unsafe impl ::windows::runtime::Interface for GazeDeviceWatcherAddedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherAddedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3885924077, 45961, 4583, [178, 1, 200, 211, 255, 183, 87, 33]);
}
impl ::windows::runtime::RuntimeName for GazeDeviceWatcherAddedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherAddedPreviewEventArgs";
}
impl ::std::convert::From<GazeDeviceWatcherAddedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: GazeDeviceWatcherAddedPreviewEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GazeDeviceWatcherAddedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &GazeDeviceWatcherAddedPreviewEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GazeDeviceWatcherAddedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GazeDeviceWatcherAddedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GazeDeviceWatcherAddedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: GazeDeviceWatcherAddedPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GazeDeviceWatcherAddedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &GazeDeviceWatcherAddedPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GazeDeviceWatcherAddedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GazeDeviceWatcherAddedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GazeDeviceWatcherAddedPreviewEventArgs {}
unsafe impl ::std::marker::Sync for GazeDeviceWatcherAddedPreviewEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GazeDeviceWatcherPreview(::windows::runtime::IInspectable);
impl GazeDeviceWatcherPreview {
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherAddedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Removed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherRemovedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Updated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherUpdatedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GazeDeviceWatcherPreview {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherPreview;{e79e7ee7-b389-11e7-b201-c8d3ffb75721})");
}
unsafe impl ::windows::runtime::Interface for GazeDeviceWatcherPreview {
    type Vtable = IGazeDeviceWatcherPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3885924071, 45961, 4583, [178, 1, 200, 211, 255, 183, 87, 33]);
}
impl ::windows::runtime::RuntimeName for GazeDeviceWatcherPreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherPreview";
}
impl ::std::convert::From<GazeDeviceWatcherPreview> for ::windows::runtime::IUnknown {
    fn from(value: GazeDeviceWatcherPreview) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GazeDeviceWatcherPreview> for ::windows::runtime::IUnknown {
    fn from(value: &GazeDeviceWatcherPreview) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GazeDeviceWatcherPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GazeDeviceWatcherPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GazeDeviceWatcherPreview> for ::windows::runtime::IInspectable {
    fn from(value: GazeDeviceWatcherPreview) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GazeDeviceWatcherPreview> for ::windows::runtime::IInspectable {
    fn from(value: &GazeDeviceWatcherPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GazeDeviceWatcherPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GazeDeviceWatcherPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GazeDeviceWatcherPreview {}
unsafe impl ::std::marker::Sync for GazeDeviceWatcherPreview {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GazeDeviceWatcherRemovedPreviewEventArgs(::windows::runtime::IInspectable);
impl GazeDeviceWatcherRemovedPreviewEventArgs {
    pub fn Device(&self) -> ::windows::runtime::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GazeDevicePreview>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GazeDeviceWatcherRemovedPreviewEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs;{f2631f08-0e3f-431f-a606-50b35af94a1c})");
}
unsafe impl ::windows::runtime::Interface for GazeDeviceWatcherRemovedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherRemovedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4066582280, 3647, 17183, [166, 6, 80, 179, 90, 249, 74, 28]);
}
impl ::windows::runtime::RuntimeName for GazeDeviceWatcherRemovedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherRemovedPreviewEventArgs";
}
impl ::std::convert::From<GazeDeviceWatcherRemovedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: GazeDeviceWatcherRemovedPreviewEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GazeDeviceWatcherRemovedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &GazeDeviceWatcherRemovedPreviewEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GazeDeviceWatcherRemovedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GazeDeviceWatcherRemovedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GazeDeviceWatcherRemovedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: GazeDeviceWatcherRemovedPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GazeDeviceWatcherRemovedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &GazeDeviceWatcherRemovedPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GazeDeviceWatcherRemovedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GazeDeviceWatcherRemovedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GazeDeviceWatcherRemovedPreviewEventArgs {}
unsafe impl ::std::marker::Sync for GazeDeviceWatcherRemovedPreviewEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GazeDeviceWatcherUpdatedPreviewEventArgs(::windows::runtime::IInspectable);
impl GazeDeviceWatcherUpdatedPreviewEventArgs {
    pub fn Device(&self) -> ::windows::runtime::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GazeDevicePreview>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GazeDeviceWatcherUpdatedPreviewEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs;{7fe830ef-7f08-4737-88e1-4a83ae4e4885})");
}
unsafe impl ::windows::runtime::Interface for GazeDeviceWatcherUpdatedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherUpdatedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2145923311, 32520, 18231, [136, 225, 74, 131, 174, 78, 72, 133]);
}
impl ::windows::runtime::RuntimeName for GazeDeviceWatcherUpdatedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeDeviceWatcherUpdatedPreviewEventArgs";
}
impl ::std::convert::From<GazeDeviceWatcherUpdatedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: GazeDeviceWatcherUpdatedPreviewEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GazeDeviceWatcherUpdatedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &GazeDeviceWatcherUpdatedPreviewEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GazeDeviceWatcherUpdatedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: GazeDeviceWatcherUpdatedPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GazeDeviceWatcherUpdatedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &GazeDeviceWatcherUpdatedPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GazeDeviceWatcherUpdatedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GazeDeviceWatcherUpdatedPreviewEventArgs {}
unsafe impl ::std::marker::Sync for GazeDeviceWatcherUpdatedPreviewEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GazeEnteredPreviewEventArgs(::windows::runtime::IInspectable);
impl GazeEnteredPreviewEventArgs {
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn CurrentPoint(&self) -> ::windows::runtime::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GazePointPreview>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GazeEnteredPreviewEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs;{2567bf43-1225-489f-9dd1-daa7c50fbf4b})");
}
unsafe impl ::windows::runtime::Interface for GazeEnteredPreviewEventArgs {
    type Vtable = IGazeEnteredPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(627556163, 4645, 18591, [157, 209, 218, 167, 197, 15, 191, 75]);
}
impl ::windows::runtime::RuntimeName for GazeEnteredPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeEnteredPreviewEventArgs";
}
impl ::std::convert::From<GazeEnteredPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: GazeEnteredPreviewEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GazeEnteredPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &GazeEnteredPreviewEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GazeEnteredPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GazeEnteredPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GazeEnteredPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: GazeEnteredPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GazeEnteredPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &GazeEnteredPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GazeEnteredPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GazeEnteredPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GazeEnteredPreviewEventArgs {}
unsafe impl ::std::marker::Sync for GazeEnteredPreviewEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GazeExitedPreviewEventArgs(::windows::runtime::IInspectable);
impl GazeExitedPreviewEventArgs {
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn CurrentPoint(&self) -> ::windows::runtime::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GazePointPreview>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GazeExitedPreviewEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs;{5d0af07e-7d83-40ef-9f0a-fbc1bbdcc5ac})");
}
unsafe impl ::windows::runtime::Interface for GazeExitedPreviewEventArgs {
    type Vtable = IGazeExitedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1560998014, 32131, 16623, [159, 10, 251, 193, 187, 220, 197, 172]);
}
impl ::windows::runtime::RuntimeName for GazeExitedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeExitedPreviewEventArgs";
}
impl ::std::convert::From<GazeExitedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: GazeExitedPreviewEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GazeExitedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &GazeExitedPreviewEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GazeExitedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GazeExitedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GazeExitedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: GazeExitedPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GazeExitedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &GazeExitedPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GazeExitedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GazeExitedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GazeExitedPreviewEventArgs {}
unsafe impl ::std::marker::Sync for GazeExitedPreviewEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GazeInputSourcePreview(::windows::runtime::IInspectable);
impl GazeInputSourcePreview {
    #[cfg(feature = "Foundation")]
    pub fn GazeMoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeMovedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveGazeMoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GazeEntered<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeEnteredPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveGazeEntered<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GazeExited<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeExitedPreviewEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveGazeExited<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::runtime::Result<GazeInputSourcePreview> {
        Self::IGazeInputSourcePreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GazeInputSourcePreview>(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows::runtime::Result<GazeDeviceWatcherPreview> {
        Self::IGazeInputSourcePreviewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GazeDeviceWatcherPreview>(result__)
        })
    }
    pub fn IGazeInputSourcePreviewStatics<R, F: FnOnce(&IGazeInputSourcePreviewStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GazeInputSourcePreview, IGazeInputSourcePreviewStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GazeInputSourcePreview {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeInputSourcePreview;{e79e7ee8-b389-11e7-b201-c8d3ffb75721})");
}
unsafe impl ::windows::runtime::Interface for GazeInputSourcePreview {
    type Vtable = IGazeInputSourcePreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3885924072, 45961, 4583, [178, 1, 200, 211, 255, 183, 87, 33]);
}
impl ::windows::runtime::RuntimeName for GazeInputSourcePreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeInputSourcePreview";
}
impl ::std::convert::From<GazeInputSourcePreview> for ::windows::runtime::IUnknown {
    fn from(value: GazeInputSourcePreview) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GazeInputSourcePreview> for ::windows::runtime::IUnknown {
    fn from(value: &GazeInputSourcePreview) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GazeInputSourcePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GazeInputSourcePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GazeInputSourcePreview> for ::windows::runtime::IInspectable {
    fn from(value: GazeInputSourcePreview) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GazeInputSourcePreview> for ::windows::runtime::IInspectable {
    fn from(value: &GazeInputSourcePreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GazeInputSourcePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GazeInputSourcePreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GazeInputSourcePreview {}
unsafe impl ::std::marker::Sync for GazeInputSourcePreview {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GazeMovedPreviewEventArgs(::windows::runtime::IInspectable);
impl GazeMovedPreviewEventArgs {
    pub fn Handled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn CurrentPoint(&self) -> ::windows::runtime::Result<GazePointPreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GazePointPreview>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetIntermediatePoints(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<GazePointPreview>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<GazePointPreview>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GazeMovedPreviewEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs;{e79e7eeb-b389-11e7-b201-c8d3ffb75721})");
}
unsafe impl ::windows::runtime::Interface for GazeMovedPreviewEventArgs {
    type Vtable = IGazeMovedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3885924075, 45961, 4583, [178, 1, 200, 211, 255, 183, 87, 33]);
}
impl ::windows::runtime::RuntimeName for GazeMovedPreviewEventArgs {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazeMovedPreviewEventArgs";
}
impl ::std::convert::From<GazeMovedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: GazeMovedPreviewEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GazeMovedPreviewEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &GazeMovedPreviewEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GazeMovedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GazeMovedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GazeMovedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: GazeMovedPreviewEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GazeMovedPreviewEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &GazeMovedPreviewEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GazeMovedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GazeMovedPreviewEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GazeMovedPreviewEventArgs {}
unsafe impl ::std::marker::Sync for GazeMovedPreviewEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GazePointPreview(::windows::runtime::IInspectable);
impl GazePointPreview {
    pub fn SourceDevice(&self) -> ::windows::runtime::Result<GazeDevicePreview> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<GazeDevicePreview>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn EyeGazePosition(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn HeadGazePosition(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Devices_HumanInterfaceDevice")]
    pub fn HidInputReport(&self) -> ::windows::runtime::Result<super::super::HumanInterfaceDevice::HidInputReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::HumanInterfaceDevice::HidInputReport>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for GazePointPreview {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Input.Preview.GazePointPreview;{e79e7eea-b389-11e7-b201-c8d3ffb75721})");
}
unsafe impl ::windows::runtime::Interface for GazePointPreview {
    type Vtable = IGazePointPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3885924074, 45961, 4583, [178, 1, 200, 211, 255, 183, 87, 33]);
}
impl ::windows::runtime::RuntimeName for GazePointPreview {
    const NAME: &'static str = "Windows.Devices.Input.Preview.GazePointPreview";
}
impl ::std::convert::From<GazePointPreview> for ::windows::runtime::IUnknown {
    fn from(value: GazePointPreview) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&GazePointPreview> for ::windows::runtime::IUnknown {
    fn from(value: &GazePointPreview) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GazePointPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GazePointPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<GazePointPreview> for ::windows::runtime::IInspectable {
    fn from(value: GazePointPreview) -> Self {
        value.0
    }
}
impl ::std::convert::From<&GazePointPreview> for ::windows::runtime::IInspectable {
    fn from(value: &GazePointPreview) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GazePointPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GazePointPreview {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for GazePointPreview {}
unsafe impl ::std::marker::Sync for GazePointPreview {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGazeDevicePreview(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGazeDevicePreview {
    type Vtable = IGazeDevicePreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3885924073, 45961, 4583, [178, 1, 200, 211, 255, 183, 87, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDevicePreview_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut GazeDeviceConfigurationStatePreview) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Devices_HumanInterfaceDevice", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherAddedPreviewEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGazeDeviceWatcherAddedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherAddedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3885924077, 45961, 4583, [178, 1, 200, 211, 255, 183, 87, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherAddedPreviewEventArgs_abi(
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
pub struct IGazeDeviceWatcherPreview(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGazeDeviceWatcherPreview {
    type Vtable = IGazeDeviceWatcherPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3885924071, 45961, 4583, [178, 1, 200, 211, 255, 183, 87, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherRemovedPreviewEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGazeDeviceWatcherRemovedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherRemovedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4066582280, 3647, 17183, [166, 6, 80, 179, 90, 249, 74, 28]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherRemovedPreviewEventArgs_abi(
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
pub struct IGazeDeviceWatcherUpdatedPreviewEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGazeDeviceWatcherUpdatedPreviewEventArgs {
    type Vtable = IGazeDeviceWatcherUpdatedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2145923311, 32520, 18231, [136, 225, 74, 131, 174, 78, 72, 133]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeDeviceWatcherUpdatedPreviewEventArgs_abi(
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
pub struct IGazeEnteredPreviewEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGazeEnteredPreviewEventArgs {
    type Vtable = IGazeEnteredPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(627556163, 4645, 18591, [157, 209, 218, 167, 197, 15, 191, 75]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeEnteredPreviewEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGazeExitedPreviewEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGazeExitedPreviewEventArgs {
    type Vtable = IGazeExitedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1560998014, 32131, 16623, [159, 10, 251, 193, 187, 220, 197, 172]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeExitedPreviewEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGazeInputSourcePreview(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGazeInputSourcePreview {
    type Vtable = IGazeInputSourcePreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3885924072, 45961, 4583, [178, 1, 200, 211, 255, 183, 87, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeInputSourcePreview_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGazeInputSourcePreviewStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGazeInputSourcePreviewStatics {
    type Vtable = IGazeInputSourcePreviewStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3885924070, 45961, 4583, [178, 1, 200, 211, 255, 183, 87, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeInputSourcePreviewStatics_abi(
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
pub struct IGazeMovedPreviewEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGazeMovedPreviewEventArgs {
    type Vtable = IGazeMovedPreviewEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3885924075, 45961, 4583, [178, 1, 200, 211, 255, 183, 87, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazeMovedPreviewEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IGazePointPreview(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGazePointPreview {
    type Vtable = IGazePointPreview_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3885924074, 45961, 4583, [178, 1, 200, 211, 255, 183, 87, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGazePointPreview_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Devices_HumanInterfaceDevice")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Devices_HumanInterfaceDevice"))] usize,
);
