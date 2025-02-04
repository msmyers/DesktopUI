#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CustomDevice(::windows::runtime::IInspectable);
impl CustomDevice {
    #[cfg(feature = "Storage_Streams")]
    pub fn InputStream(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IInputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn OutputStream(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IOutputStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SendIOControlAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IIOControlCode>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param2: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, iocontrolcode: Param0, inputbuffer: Param1, outputbuffer: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), iocontrolcode.into_param().abi(), inputbuffer.into_param().abi(), outputbuffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn TrySendIOControlAsync<'a, Param0: ::windows::runtime::IntoParam<'a, IIOControlCode>, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param2: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, iocontrolcode: Param0, inputbuffer: Param1, outputbuffer: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), iocontrolcode.into_param().abi(), inputbuffer.into_param().abi(), outputbuffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn GetDeviceSelector<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(classguid: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ICustomDeviceStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), classguid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0, desiredaccess: DeviceAccessMode, sharingmode: DeviceSharingMode) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<CustomDevice>> {
        Self::ICustomDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), desiredaccess, sharingmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CustomDevice>>(result__)
        })
    }
    pub fn ICustomDeviceStatics<R, F: FnOnce(&ICustomDeviceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CustomDevice, ICustomDeviceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CustomDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Custom.CustomDevice;{dd30251f-c48b-43bd-bcb1-dec88f15143e})");
}
unsafe impl ::windows::runtime::Interface for CustomDevice {
    type Vtable = ICustomDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3710919967, 50315, 17341, [188, 177, 222, 200, 143, 21, 20, 62]);
}
impl ::windows::runtime::RuntimeName for CustomDevice {
    const NAME: &'static str = "Windows.Devices.Custom.CustomDevice";
}
impl ::std::convert::From<CustomDevice> for ::windows::runtime::IUnknown {
    fn from(value: CustomDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CustomDevice> for ::windows::runtime::IUnknown {
    fn from(value: &CustomDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CustomDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CustomDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CustomDevice> for ::windows::runtime::IInspectable {
    fn from(value: CustomDevice) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CustomDevice> for ::windows::runtime::IInspectable {
    fn from(value: &CustomDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CustomDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CustomDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CustomDevice {}
unsafe impl ::std::marker::Sync for CustomDevice {}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct CustomDeviceContract(pub u8);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeviceAccessMode(pub i32);
impl DeviceAccessMode {
    pub const Read: DeviceAccessMode = DeviceAccessMode(0i32);
    pub const Write: DeviceAccessMode = DeviceAccessMode(1i32);
    pub const ReadWrite: DeviceAccessMode = DeviceAccessMode(2i32);
}
impl ::std::convert::From<i32> for DeviceAccessMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeviceAccessMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeviceAccessMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.DeviceAccessMode;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeviceSharingMode(pub i32);
impl DeviceSharingMode {
    pub const Shared: DeviceSharingMode = DeviceSharingMode(0i32);
    pub const Exclusive: DeviceSharingMode = DeviceSharingMode(1i32);
}
impl ::std::convert::From<i32> for DeviceSharingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeviceSharingMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeviceSharingMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.DeviceSharingMode;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICustomDevice(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomDevice {
    type Vtable = ICustomDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3710919967, 50315, 17341, [188, 177, 222, 200, 143, 21, 20, 62]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iocontrolcode: ::windows::runtime::RawPtr, inputbuffer: ::windows::runtime::RawPtr, outputbuffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iocontrolcode: ::windows::runtime::RawPtr, inputbuffer: ::windows::runtime::RawPtr, outputbuffer: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICustomDeviceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICustomDeviceStatics {
    type Vtable = ICustomDeviceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3357672210, 61260, 18097, [165, 142, 238, 179, 8, 220, 137, 23]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, classguid: ::windows::runtime::GUID, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, desiredaccess: DeviceAccessMode, sharingmode: DeviceSharingMode, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IIOControlCode(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIOControlCode {
    type Vtable = IIOControlCode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(244668903, 24776, 17269, [167, 97, 127, 136, 8, 6, 108, 96]);
}
impl IIOControlCode {
    pub fn AccessMode(&self) -> ::windows::runtime::Result<IOControlAccessMode> {
        let this = self;
        unsafe {
            let mut result__: IOControlAccessMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IOControlAccessMode>(result__)
        }
    }
    pub fn BufferingMethod(&self) -> ::windows::runtime::Result<IOControlBufferingMethod> {
        let this = self;
        unsafe {
            let mut result__: IOControlBufferingMethod = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IOControlBufferingMethod>(result__)
        }
    }
    pub fn Function(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn DeviceType(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn ControlCode(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IIOControlCode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{0e9559e7-60c8-4375-a761-7f8808066c60}");
}
impl ::std::convert::From<IIOControlCode> for ::windows::runtime::IUnknown {
    fn from(value: IIOControlCode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IIOControlCode> for ::windows::runtime::IUnknown {
    fn from(value: &IIOControlCode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IIOControlCode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IIOControlCode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IIOControlCode> for ::windows::runtime::IInspectable {
    fn from(value: IIOControlCode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IIOControlCode> for ::windows::runtime::IInspectable {
    fn from(value: &IIOControlCode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IIOControlCode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IIOControlCode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIOControlCode_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut IOControlAccessMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut IOControlBufferingMethod) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IIOControlCodeFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IIOControlCodeFactory {
    type Vtable = IIOControlCodeFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2238348528, 19473, 17582, [175, 198, 184, 212, 162, 18, 120, 143]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIOControlCodeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IKnownDeviceTypesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownDeviceTypesStatics {
    type Vtable = IKnownDeviceTypesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3998513602, 21576, 17882, [173, 27, 36, 148, 140, 35, 144, 148]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownDeviceTypesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IOControlAccessMode(pub i32);
impl IOControlAccessMode {
    pub const Any: IOControlAccessMode = IOControlAccessMode(0i32);
    pub const Read: IOControlAccessMode = IOControlAccessMode(1i32);
    pub const Write: IOControlAccessMode = IOControlAccessMode(2i32);
    pub const ReadWrite: IOControlAccessMode = IOControlAccessMode(3i32);
}
impl ::std::convert::From<i32> for IOControlAccessMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IOControlAccessMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IOControlAccessMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.IOControlAccessMode;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IOControlBufferingMethod(pub i32);
impl IOControlBufferingMethod {
    pub const Buffered: IOControlBufferingMethod = IOControlBufferingMethod(0i32);
    pub const DirectInput: IOControlBufferingMethod = IOControlBufferingMethod(1i32);
    pub const DirectOutput: IOControlBufferingMethod = IOControlBufferingMethod(2i32);
    pub const Neither: IOControlBufferingMethod = IOControlBufferingMethod(3i32);
}
impl ::std::convert::From<i32> for IOControlBufferingMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IOControlBufferingMethod {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for IOControlBufferingMethod {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Custom.IOControlBufferingMethod;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IOControlCode(::windows::runtime::IInspectable);
impl IOControlCode {
    pub fn AccessMode(&self) -> ::windows::runtime::Result<IOControlAccessMode> {
        let this = self;
        unsafe {
            let mut result__: IOControlAccessMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IOControlAccessMode>(result__)
        }
    }
    pub fn BufferingMethod(&self) -> ::windows::runtime::Result<IOControlBufferingMethod> {
        let this = self;
        unsafe {
            let mut result__: IOControlBufferingMethod = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IOControlBufferingMethod>(result__)
        }
    }
    pub fn Function(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn DeviceType(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn ControlCode(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn CreateIOControlCode(devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod) -> ::windows::runtime::Result<IOControlCode> {
        Self::IIOControlCodeFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), devicetype, function, accessmode, bufferingmethod, &mut result__).from_abi::<IOControlCode>(result__)
        })
    }
    pub fn IIOControlCodeFactory<R, F: FnOnce(&IIOControlCodeFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<IOControlCode, IIOControlCodeFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IOControlCode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Custom.IOControlCode;{0e9559e7-60c8-4375-a761-7f8808066c60})");
}
unsafe impl ::windows::runtime::Interface for IOControlCode {
    type Vtable = IIOControlCode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(244668903, 24776, 17269, [167, 97, 127, 136, 8, 6, 108, 96]);
}
impl ::windows::runtime::RuntimeName for IOControlCode {
    const NAME: &'static str = "Windows.Devices.Custom.IOControlCode";
}
impl ::std::convert::From<IOControlCode> for ::windows::runtime::IUnknown {
    fn from(value: IOControlCode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOControlCode> for ::windows::runtime::IUnknown {
    fn from(value: &IOControlCode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IOControlCode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IOControlCode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IOControlCode> for ::windows::runtime::IInspectable {
    fn from(value: IOControlCode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IOControlCode> for ::windows::runtime::IInspectable {
    fn from(value: &IOControlCode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IOControlCode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IOControlCode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IOControlCode> for IIOControlCode {
    fn from(value: IOControlCode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IOControlCode> for IIOControlCode {
    fn from(value: &IOControlCode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IIOControlCode> for IOControlCode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIOControlCode> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IIOControlCode>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IIOControlCode> for &IOControlCode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IIOControlCode> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IIOControlCode>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for IOControlCode {}
unsafe impl ::std::marker::Sync for IOControlCode {}
pub struct KnownDeviceTypes {}
impl KnownDeviceTypes {
    pub fn Unknown() -> ::windows::runtime::Result<u16> {
        Self::IKnownDeviceTypesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn IKnownDeviceTypesStatics<R, F: FnOnce(&IKnownDeviceTypesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<KnownDeviceTypes, IKnownDeviceTypesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownDeviceTypes {
    const NAME: &'static str = "Windows.Devices.Custom.KnownDeviceTypes";
}
