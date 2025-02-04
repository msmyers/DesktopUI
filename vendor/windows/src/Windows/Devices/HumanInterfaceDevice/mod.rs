#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HidBooleanControl(::windows::runtime::IInspectable);
impl HidBooleanControl {
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn IsActive(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsActive(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ControlDescription(&self) -> ::windows::runtime::Result<HidBooleanControlDescription> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidBooleanControlDescription>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidBooleanControl {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidBooleanControl;{524df48a-3695-408c-bba2-e2eb5abfbc20})");
}
unsafe impl ::windows::runtime::Interface for HidBooleanControl {
    type Vtable = IHidBooleanControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1380840586, 13973, 16524, [187, 162, 226, 235, 90, 191, 188, 32]);
}
impl ::windows::runtime::RuntimeName for HidBooleanControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidBooleanControl";
}
impl ::std::convert::From<HidBooleanControl> for ::windows::runtime::IUnknown {
    fn from(value: HidBooleanControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HidBooleanControl> for ::windows::runtime::IUnknown {
    fn from(value: &HidBooleanControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HidBooleanControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HidBooleanControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HidBooleanControl> for ::windows::runtime::IInspectable {
    fn from(value: HidBooleanControl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HidBooleanControl> for ::windows::runtime::IInspectable {
    fn from(value: &HidBooleanControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HidBooleanControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HidBooleanControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HidBooleanControl {}
unsafe impl ::std::marker::Sync for HidBooleanControl {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HidBooleanControlDescription(::windows::runtime::IInspectable);
impl HidBooleanControlDescription {
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn ReportId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn ReportType(&self) -> ::windows::runtime::Result<HidReportType> {
        let this = self;
        unsafe {
            let mut result__: HidReportType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidReportType>(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ParentCollections(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HidCollection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidCollection>>(result__)
        }
    }
    pub fn IsAbsolute(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IHidBooleanControlDescription2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidBooleanControlDescription {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription;{6196e543-29d8-4a2a-8683-849e207bbe31})");
}
unsafe impl ::windows::runtime::Interface for HidBooleanControlDescription {
    type Vtable = IHidBooleanControlDescription_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1637279043, 10712, 18986, [134, 131, 132, 158, 32, 123, 190, 49]);
}
impl ::windows::runtime::RuntimeName for HidBooleanControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidBooleanControlDescription";
}
impl ::std::convert::From<HidBooleanControlDescription> for ::windows::runtime::IUnknown {
    fn from(value: HidBooleanControlDescription) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HidBooleanControlDescription> for ::windows::runtime::IUnknown {
    fn from(value: &HidBooleanControlDescription) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HidBooleanControlDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HidBooleanControlDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HidBooleanControlDescription> for ::windows::runtime::IInspectable {
    fn from(value: HidBooleanControlDescription) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HidBooleanControlDescription> for ::windows::runtime::IInspectable {
    fn from(value: &HidBooleanControlDescription) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HidBooleanControlDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HidBooleanControlDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HidBooleanControlDescription {}
unsafe impl ::std::marker::Sync for HidBooleanControlDescription {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HidCollection(::windows::runtime::IInspectable);
impl HidCollection {
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::runtime::Result<HidCollectionType> {
        let this = self;
        unsafe {
            let mut result__: HidCollectionType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidCollectionType>(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidCollection;{7189f5a3-32f1-46e3-befd-44d2663b7e6a})");
}
unsafe impl ::windows::runtime::Interface for HidCollection {
    type Vtable = IHidCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1904866723, 13041, 18147, [190, 253, 68, 210, 102, 59, 126, 106]);
}
impl ::windows::runtime::RuntimeName for HidCollection {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidCollection";
}
impl ::std::convert::From<HidCollection> for ::windows::runtime::IUnknown {
    fn from(value: HidCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HidCollection> for ::windows::runtime::IUnknown {
    fn from(value: &HidCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HidCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HidCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HidCollection> for ::windows::runtime::IInspectable {
    fn from(value: HidCollection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HidCollection> for ::windows::runtime::IInspectable {
    fn from(value: &HidCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HidCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HidCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HidCollection {}
unsafe impl ::std::marker::Sync for HidCollection {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HidCollectionType(pub i32);
impl HidCollectionType {
    pub const Physical: HidCollectionType = HidCollectionType(0i32);
    pub const Application: HidCollectionType = HidCollectionType(1i32);
    pub const Logical: HidCollectionType = HidCollectionType(2i32);
    pub const Report: HidCollectionType = HidCollectionType(3i32);
    pub const NamedArray: HidCollectionType = HidCollectionType(4i32);
    pub const UsageSwitch: HidCollectionType = HidCollectionType(5i32);
    pub const UsageModifier: HidCollectionType = HidCollectionType(6i32);
    pub const Other: HidCollectionType = HidCollectionType(7i32);
}
impl ::std::convert::From<i32> for HidCollectionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HidCollectionType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HidCollectionType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.HumanInterfaceDevice.HidCollectionType;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HidDevice(::windows::runtime::IInspectable);
impl HidDevice {
    pub fn VendorId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn ProductId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn Version(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetInputReportAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<HidInputReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HidInputReport>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetInputReportByIdAsync(&self, reportid: u16) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<HidInputReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), reportid, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HidInputReport>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFeatureReportAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HidFeatureReport>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFeatureReportByIdAsync(&self, reportid: u16) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<HidFeatureReport>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), reportid, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HidFeatureReport>>(result__)
        }
    }
    pub fn CreateOutputReport(&self) -> ::windows::runtime::Result<HidOutputReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidOutputReport>(result__)
        }
    }
    pub fn CreateOutputReportById(&self, reportid: u16) -> ::windows::runtime::Result<HidOutputReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), reportid, &mut result__).from_abi::<HidOutputReport>(result__)
        }
    }
    pub fn CreateFeatureReport(&self) -> ::windows::runtime::Result<HidFeatureReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidFeatureReport>(result__)
        }
    }
    pub fn CreateFeatureReportById(&self, reportid: u16) -> ::windows::runtime::Result<HidFeatureReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), reportid, &mut result__).from_abi::<HidFeatureReport>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SendOutputReportAsync<'a, Param0: ::windows::runtime::IntoParam<'a, HidOutputReport>>(&self, outputreport: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), outputreport.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SendFeatureReportAsync<'a, Param0: ::windows::runtime::IntoParam<'a, HidFeatureReport>>(&self, featurereport: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), featurereport.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetBooleanControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControlDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), reporttype, usagepage, usageid, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidBooleanControlDescription>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetNumericControlDescriptions(&self, reporttype: HidReportType, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HidNumericControlDescription>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), reporttype, usagepage, usageid, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidNumericControlDescription>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn InputReportReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<HidDevice, HidInputReportReceivedEventArgs>>>(&self, reporthandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), reporthandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputReportReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn GetDeviceSelector(usagepage: u16, usageid: u16) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn GetDeviceSelectorVidPid(usagepage: u16, usageid: u16, vendorid: u16, productid: u16) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), usagepage, usageid, vendorid, productid, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0, accessmode: super::super::Storage::FileAccessMode) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<HidDevice>> {
        Self::IHidDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), accessmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<HidDevice>>(result__)
        })
    }
    pub fn IHidDeviceStatics<R, F: FnOnce(&IHidDeviceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HidDevice, IHidDeviceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidDevice;{5f8a14e7-2200-432e-95da-d09b87d574a8})");
}
unsafe impl ::windows::runtime::Interface for HidDevice {
    type Vtable = IHidDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1602884839, 8704, 17198, [149, 218, 208, 155, 135, 213, 116, 168]);
}
impl ::windows::runtime::RuntimeName for HidDevice {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidDevice";
}
impl ::std::convert::From<HidDevice> for ::windows::runtime::IUnknown {
    fn from(value: HidDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HidDevice> for ::windows::runtime::IUnknown {
    fn from(value: &HidDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HidDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HidDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HidDevice> for ::windows::runtime::IInspectable {
    fn from(value: HidDevice) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HidDevice> for ::windows::runtime::IInspectable {
    fn from(value: &HidDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HidDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HidDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HidDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HidDevice) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HidDevice> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HidDevice) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for HidDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &HidDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HidDevice {}
unsafe impl ::std::marker::Sync for HidDevice {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HidFeatureReport(::windows::runtime::IInspectable);
impl HidFeatureReport {
    pub fn Id(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    pub fn GetBooleanControlByDescription<'a, Param0: ::windows::runtime::IntoParam<'a, HidBooleanControlDescription>>(&self, controldescription: Param0) -> ::windows::runtime::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
    pub fn GetNumericControlByDescription<'a, Param0: ::windows::runtime::IntoParam<'a, HidNumericControlDescription>>(&self, controldescription: Param0) -> ::windows::runtime::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidFeatureReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidFeatureReport;{841d9b79-5ae5-46e3-82ef-1fec5c8942f4})");
}
unsafe impl ::windows::runtime::Interface for HidFeatureReport {
    type Vtable = IHidFeatureReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2216532857, 23269, 18147, [130, 239, 31, 236, 92, 137, 66, 244]);
}
impl ::windows::runtime::RuntimeName for HidFeatureReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidFeatureReport";
}
impl ::std::convert::From<HidFeatureReport> for ::windows::runtime::IUnknown {
    fn from(value: HidFeatureReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HidFeatureReport> for ::windows::runtime::IUnknown {
    fn from(value: &HidFeatureReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HidFeatureReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HidFeatureReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HidFeatureReport> for ::windows::runtime::IInspectable {
    fn from(value: HidFeatureReport) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HidFeatureReport> for ::windows::runtime::IInspectable {
    fn from(value: &HidFeatureReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HidFeatureReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HidFeatureReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HidFeatureReport {}
unsafe impl ::std::marker::Sync for HidFeatureReport {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HidInputReport(::windows::runtime::IInspectable);
impl HidInputReport {
    pub fn Id(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ActivatedBooleanControls(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidBooleanControl>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TransitionedBooleanControls(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HidBooleanControl>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidBooleanControl>>(result__)
        }
    }
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    pub fn GetBooleanControlByDescription<'a, Param0: ::windows::runtime::IntoParam<'a, HidBooleanControlDescription>>(&self, controldescription: Param0) -> ::windows::runtime::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
    pub fn GetNumericControlByDescription<'a, Param0: ::windows::runtime::IntoParam<'a, HidNumericControlDescription>>(&self, controldescription: Param0) -> ::windows::runtime::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidInputReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidInputReport;{c35d0e50-f7e7-4e8d-b23e-cabbe56b90e9})");
}
unsafe impl ::windows::runtime::Interface for HidInputReport {
    type Vtable = IHidInputReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3277655632, 63463, 20109, [178, 62, 202, 187, 229, 107, 144, 233]);
}
impl ::windows::runtime::RuntimeName for HidInputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidInputReport";
}
impl ::std::convert::From<HidInputReport> for ::windows::runtime::IUnknown {
    fn from(value: HidInputReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HidInputReport> for ::windows::runtime::IUnknown {
    fn from(value: &HidInputReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HidInputReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HidInputReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HidInputReport> for ::windows::runtime::IInspectable {
    fn from(value: HidInputReport) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HidInputReport> for ::windows::runtime::IInspectable {
    fn from(value: &HidInputReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HidInputReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HidInputReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HidInputReport {}
unsafe impl ::std::marker::Sync for HidInputReport {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HidInputReportReceivedEventArgs(::windows::runtime::IInspectable);
impl HidInputReportReceivedEventArgs {
    pub fn Report(&self) -> ::windows::runtime::Result<HidInputReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidInputReport>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidInputReportReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs;{7059c5cb-59b2-4dc2-985c-0adc6136fa2d})");
}
unsafe impl ::windows::runtime::Interface for HidInputReportReceivedEventArgs {
    type Vtable = IHidInputReportReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1884931531, 22962, 19906, [152, 92, 10, 220, 97, 54, 250, 45]);
}
impl ::windows::runtime::RuntimeName for HidInputReportReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidInputReportReceivedEventArgs";
}
impl ::std::convert::From<HidInputReportReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: HidInputReportReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HidInputReportReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &HidInputReportReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HidInputReportReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HidInputReportReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HidInputReportReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: HidInputReportReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HidInputReportReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &HidInputReportReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HidInputReportReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HidInputReportReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HidInputReportReceivedEventArgs {}
unsafe impl ::std::marker::Sync for HidInputReportReceivedEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HidNumericControl(::windows::runtime::IInspectable);
impl HidNumericControl {
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn IsGrouped(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn Value(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn SetValue(&self, value: i64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ScaledValue(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    pub fn SetScaledValue(&self, value: i64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ControlDescription(&self) -> ::windows::runtime::Result<HidNumericControlDescription> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidNumericControlDescription>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidNumericControl {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidNumericControl;{e38a12a5-35a7-4b75-89c8-fb1f28b10823})");
}
unsafe impl ::windows::runtime::Interface for HidNumericControl {
    type Vtable = IHidNumericControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3817476773, 13735, 19317, [137, 200, 251, 31, 40, 177, 8, 35]);
}
impl ::windows::runtime::RuntimeName for HidNumericControl {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidNumericControl";
}
impl ::std::convert::From<HidNumericControl> for ::windows::runtime::IUnknown {
    fn from(value: HidNumericControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HidNumericControl> for ::windows::runtime::IUnknown {
    fn from(value: &HidNumericControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HidNumericControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HidNumericControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HidNumericControl> for ::windows::runtime::IInspectable {
    fn from(value: HidNumericControl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HidNumericControl> for ::windows::runtime::IInspectable {
    fn from(value: &HidNumericControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HidNumericControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HidNumericControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HidNumericControl {}
unsafe impl ::std::marker::Sync for HidNumericControl {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HidNumericControlDescription(::windows::runtime::IInspectable);
impl HidNumericControlDescription {
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn ReportId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn ReportType(&self) -> ::windows::runtime::Result<HidReportType> {
        let this = self;
        unsafe {
            let mut result__: HidReportType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HidReportType>(result__)
        }
    }
    pub fn ReportSize(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn ReportCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn UsagePage(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn UsageId(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn LogicalMinimum(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn LogicalMaximum(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn PhysicalMinimum(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn PhysicalMaximum(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn UnitExponent(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Unit(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn IsAbsolute(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn HasNull(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn ParentCollections(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<HidCollection>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<HidCollection>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidNumericControlDescription {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription;{638d5e86-1d97-4c75-927f-5ff58ba05e32})");
}
unsafe impl ::windows::runtime::Interface for HidNumericControlDescription {
    type Vtable = IHidNumericControlDescription_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1670209158, 7575, 19573, [146, 127, 95, 245, 139, 160, 94, 50]);
}
impl ::windows::runtime::RuntimeName for HidNumericControlDescription {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidNumericControlDescription";
}
impl ::std::convert::From<HidNumericControlDescription> for ::windows::runtime::IUnknown {
    fn from(value: HidNumericControlDescription) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HidNumericControlDescription> for ::windows::runtime::IUnknown {
    fn from(value: &HidNumericControlDescription) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HidNumericControlDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HidNumericControlDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HidNumericControlDescription> for ::windows::runtime::IInspectable {
    fn from(value: HidNumericControlDescription) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HidNumericControlDescription> for ::windows::runtime::IInspectable {
    fn from(value: &HidNumericControlDescription) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HidNumericControlDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HidNumericControlDescription {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HidNumericControlDescription {}
unsafe impl ::std::marker::Sync for HidNumericControlDescription {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct HidOutputReport(::windows::runtime::IInspectable);
impl HidOutputReport {
    pub fn Id(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn GetBooleanControl(&self, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    pub fn GetBooleanControlByDescription<'a, Param0: ::windows::runtime::IntoParam<'a, HidBooleanControlDescription>>(&self, controldescription: Param0) -> ::windows::runtime::Result<HidBooleanControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidBooleanControl>(result__)
        }
    }
    pub fn GetNumericControl(&self, usagepage: u16, usageid: u16) -> ::windows::runtime::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), usagepage, usageid, &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
    pub fn GetNumericControlByDescription<'a, Param0: ::windows::runtime::IntoParam<'a, HidNumericControlDescription>>(&self, controldescription: Param0) -> ::windows::runtime::Result<HidNumericControl> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), controldescription.into_param().abi(), &mut result__).from_abi::<HidNumericControl>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HidOutputReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.HumanInterfaceDevice.HidOutputReport;{62cb2544-c896-4463-93c1-df9db053c450})");
}
unsafe impl ::windows::runtime::Interface for HidOutputReport {
    type Vtable = IHidOutputReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1657480516, 51350, 17507, [147, 193, 223, 157, 176, 83, 196, 80]);
}
impl ::windows::runtime::RuntimeName for HidOutputReport {
    const NAME: &'static str = "Windows.Devices.HumanInterfaceDevice.HidOutputReport";
}
impl ::std::convert::From<HidOutputReport> for ::windows::runtime::IUnknown {
    fn from(value: HidOutputReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HidOutputReport> for ::windows::runtime::IUnknown {
    fn from(value: &HidOutputReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HidOutputReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HidOutputReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<HidOutputReport> for ::windows::runtime::IInspectable {
    fn from(value: HidOutputReport) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HidOutputReport> for ::windows::runtime::IInspectable {
    fn from(value: &HidOutputReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HidOutputReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HidOutputReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for HidOutputReport {}
unsafe impl ::std::marker::Sync for HidOutputReport {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HidReportType(pub i32);
impl HidReportType {
    pub const Input: HidReportType = HidReportType(0i32);
    pub const Output: HidReportType = HidReportType(1i32);
    pub const Feature: HidReportType = HidReportType(2i32);
}
impl ::std::convert::From<i32> for HidReportType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HidReportType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HidReportType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.HumanInterfaceDevice.HidReportType;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHidBooleanControl(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidBooleanControl {
    type Vtable = IHidBooleanControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1380840586, 13973, 16524, [187, 162, 226, 235, 90, 191, 188, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidBooleanControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHidBooleanControlDescription(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidBooleanControlDescription {
    type Vtable = IHidBooleanControlDescription_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1637279043, 10712, 18986, [134, 131, 132, 158, 32, 123, 190, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidBooleanControlDescription_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HidReportType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHidBooleanControlDescription2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidBooleanControlDescription2 {
    type Vtable = IHidBooleanControlDescription2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3371094762, 35447, 19510, [170, 0, 95, 240, 68, 157, 62, 115]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidBooleanControlDescription2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHidCollection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidCollection {
    type Vtable = IHidCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1904866723, 13041, 18147, [190, 253, 68, 210, 102, 59, 126, 106]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HidCollectionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHidDevice(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidDevice {
    type Vtable = IHidDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1602884839, 8704, 17198, [149, 218, 208, 155, 135, 213, 116, 168]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reportid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reportid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reportid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reportid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputreport: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, featurereport: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reporttype: HidReportType, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reporthandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHidDeviceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidDeviceStatics {
    type Vtable = IHidDeviceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2656666084, 38998, 16780, [159, 115, 119, 222, 12, 216, 87, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, vendorid: u16, productid: u16, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, accessmode: super::super::Storage::FileAccessMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHidFeatureReport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidFeatureReport {
    type Vtable = IHidFeatureReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2216532857, 23269, 18147, [130, 239, 31, 236, 92, 137, 66, 244]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidFeatureReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, controldescription: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, controldescription: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHidInputReport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidInputReport {
    type Vtable = IHidInputReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3277655632, 63463, 20109, [178, 62, 202, 187, 229, 107, 144, 233]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidInputReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, controldescription: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, controldescription: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHidInputReportReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidInputReportReceivedEventArgs {
    type Vtable = IHidInputReportReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1884931531, 22962, 19906, [152, 92, 10, 220, 97, 54, 250, 45]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidInputReportReceivedEventArgs_abi(
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
pub struct IHidNumericControl(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidNumericControl {
    type Vtable = IHidNumericControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3817476773, 13735, 19317, [137, 200, 251, 31, 40, 177, 8, 35]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidNumericControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHidNumericControlDescription(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidNumericControlDescription {
    type Vtable = IHidNumericControlDescription_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1670209158, 7575, 19573, [146, 127, 95, 245, 139, 160, 94, 50]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidNumericControlDescription_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HidReportType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IHidOutputReport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHidOutputReport {
    type Vtable = IHidOutputReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1657480516, 51350, 17507, [147, 193, 223, 157, 176, 83, 196, 80]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHidOutputReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, controldescription: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usagepage: u16, usageid: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, controldescription: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
