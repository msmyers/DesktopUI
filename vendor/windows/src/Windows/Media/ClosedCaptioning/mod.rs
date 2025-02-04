#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ClosedCaptionColor(pub i32);
impl ClosedCaptionColor {
    pub const Default: ClosedCaptionColor = ClosedCaptionColor(0i32);
    pub const White: ClosedCaptionColor = ClosedCaptionColor(1i32);
    pub const Black: ClosedCaptionColor = ClosedCaptionColor(2i32);
    pub const Red: ClosedCaptionColor = ClosedCaptionColor(3i32);
    pub const Green: ClosedCaptionColor = ClosedCaptionColor(4i32);
    pub const Blue: ClosedCaptionColor = ClosedCaptionColor(5i32);
    pub const Yellow: ClosedCaptionColor = ClosedCaptionColor(6i32);
    pub const Magenta: ClosedCaptionColor = ClosedCaptionColor(7i32);
    pub const Cyan: ClosedCaptionColor = ClosedCaptionColor(8i32);
}
impl ::std::convert::From<i32> for ClosedCaptionColor {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ClosedCaptionColor {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ClosedCaptionColor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionColor;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ClosedCaptionEdgeEffect(pub i32);
impl ClosedCaptionEdgeEffect {
    pub const Default: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(0i32);
    pub const None: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(1i32);
    pub const Raised: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(2i32);
    pub const Depressed: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(3i32);
    pub const Uniform: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(4i32);
    pub const DropShadow: ClosedCaptionEdgeEffect = ClosedCaptionEdgeEffect(5i32);
}
impl ::std::convert::From<i32> for ClosedCaptionEdgeEffect {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ClosedCaptionEdgeEffect {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ClosedCaptionEdgeEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionEdgeEffect;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ClosedCaptionOpacity(pub i32);
impl ClosedCaptionOpacity {
    pub const Default: ClosedCaptionOpacity = ClosedCaptionOpacity(0i32);
    pub const OneHundredPercent: ClosedCaptionOpacity = ClosedCaptionOpacity(1i32);
    pub const SeventyFivePercent: ClosedCaptionOpacity = ClosedCaptionOpacity(2i32);
    pub const TwentyFivePercent: ClosedCaptionOpacity = ClosedCaptionOpacity(3i32);
    pub const ZeroPercent: ClosedCaptionOpacity = ClosedCaptionOpacity(4i32);
}
impl ::std::convert::From<i32> for ClosedCaptionOpacity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ClosedCaptionOpacity {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ClosedCaptionOpacity {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionOpacity;i4)");
}
pub struct ClosedCaptionProperties {}
impl ClosedCaptionProperties {
    pub fn FontColor() -> ::windows::runtime::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionColor = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionColor>(result__)
        })
    }
    #[cfg(feature = "UI")]
    pub fn ComputedFontColor() -> ::windows::runtime::Result<super::super::UI::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: super::super::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        })
    }
    pub fn FontOpacity() -> ::windows::runtime::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionOpacity = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionOpacity>(result__)
        })
    }
    pub fn FontSize() -> ::windows::runtime::Result<ClosedCaptionSize> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionSize = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionSize>(result__)
        })
    }
    pub fn FontStyle() -> ::windows::runtime::Result<ClosedCaptionStyle> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionStyle = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionStyle>(result__)
        })
    }
    pub fn FontEffect() -> ::windows::runtime::Result<ClosedCaptionEdgeEffect> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionEdgeEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionEdgeEffect>(result__)
        })
    }
    pub fn BackgroundColor() -> ::windows::runtime::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionColor = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionColor>(result__)
        })
    }
    #[cfg(feature = "UI")]
    pub fn ComputedBackgroundColor() -> ::windows::runtime::Result<super::super::UI::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: super::super::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        })
    }
    pub fn BackgroundOpacity() -> ::windows::runtime::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionOpacity = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionOpacity>(result__)
        })
    }
    pub fn RegionColor() -> ::windows::runtime::Result<ClosedCaptionColor> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionColor = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionColor>(result__)
        })
    }
    #[cfg(feature = "UI")]
    pub fn ComputedRegionColor() -> ::windows::runtime::Result<super::super::UI::Color> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: super::super::UI::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Color>(result__)
        })
    }
    pub fn RegionOpacity() -> ::windows::runtime::Result<ClosedCaptionOpacity> {
        Self::IClosedCaptionPropertiesStatics(|this| unsafe {
            let mut result__: ClosedCaptionOpacity = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ClosedCaptionOpacity>(result__)
        })
    }
    pub fn IClosedCaptionPropertiesStatics<R, F: FnOnce(&IClosedCaptionPropertiesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ClosedCaptionProperties, IClosedCaptionPropertiesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ClosedCaptionProperties {
    const NAME: &'static str = "Windows.Media.ClosedCaptioning.ClosedCaptionProperties";
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ClosedCaptionSize(pub i32);
impl ClosedCaptionSize {
    pub const Default: ClosedCaptionSize = ClosedCaptionSize(0i32);
    pub const FiftyPercent: ClosedCaptionSize = ClosedCaptionSize(1i32);
    pub const OneHundredPercent: ClosedCaptionSize = ClosedCaptionSize(2i32);
    pub const OneHundredFiftyPercent: ClosedCaptionSize = ClosedCaptionSize(3i32);
    pub const TwoHundredPercent: ClosedCaptionSize = ClosedCaptionSize(4i32);
}
impl ::std::convert::From<i32> for ClosedCaptionSize {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ClosedCaptionSize {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ClosedCaptionSize {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionSize;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ClosedCaptionStyle(pub i32);
impl ClosedCaptionStyle {
    pub const Default: ClosedCaptionStyle = ClosedCaptionStyle(0i32);
    pub const MonospacedWithSerifs: ClosedCaptionStyle = ClosedCaptionStyle(1i32);
    pub const ProportionalWithSerifs: ClosedCaptionStyle = ClosedCaptionStyle(2i32);
    pub const MonospacedWithoutSerifs: ClosedCaptionStyle = ClosedCaptionStyle(3i32);
    pub const ProportionalWithoutSerifs: ClosedCaptionStyle = ClosedCaptionStyle(4i32);
    pub const Casual: ClosedCaptionStyle = ClosedCaptionStyle(5i32);
    pub const Cursive: ClosedCaptionStyle = ClosedCaptionStyle(6i32);
    pub const SmallCapitals: ClosedCaptionStyle = ClosedCaptionStyle(7i32);
}
impl ::std::convert::From<i32> for ClosedCaptionStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ClosedCaptionStyle {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ClosedCaptionStyle {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Media.ClosedCaptioning.ClosedCaptionStyle;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IClosedCaptionPropertiesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClosedCaptionPropertiesStatics {
    type Vtable = IClosedCaptionPropertiesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(279584644, 52272, 16705, [181, 3, 82, 114, 40, 158, 12, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosedCaptionPropertiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ClosedCaptionColor) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ClosedCaptionOpacity) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ClosedCaptionSize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ClosedCaptionStyle) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ClosedCaptionEdgeEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ClosedCaptionColor) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ClosedCaptionOpacity) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ClosedCaptionColor) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::UI::Color) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ClosedCaptionOpacity) -> ::windows::runtime::HRESULT,
);
