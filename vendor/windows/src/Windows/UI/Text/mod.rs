#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "UI_Text_Core")]
pub mod Core;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CaretType(pub i32);
impl CaretType {
    pub const Normal: CaretType = CaretType(0i32);
    pub const Null: CaretType = CaretType(1i32);
}
impl ::std::convert::From<i32> for CaretType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CaretType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CaretType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.CaretType;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ContentLinkInfo(::windows::runtime::IInspectable);
impl ContentLinkInfo {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ContentLinkInfo, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetId(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn DisplayText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetDisplayText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SecondaryText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetSecondaryText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn LinkContentKind(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetLinkContentKind<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ContentLinkInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.ContentLinkInfo;{1ed52525-1c5f-48cb-b335-78b50a2ee642})");
}
unsafe impl ::windows::runtime::Interface for ContentLinkInfo {
    type Vtable = IContentLinkInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(517285157, 7263, 18635, [179, 53, 120, 181, 10, 46, 230, 66]);
}
impl ::windows::runtime::RuntimeName for ContentLinkInfo {
    const NAME: &'static str = "Windows.UI.Text.ContentLinkInfo";
}
impl ::std::convert::From<ContentLinkInfo> for ::windows::runtime::IUnknown {
    fn from(value: ContentLinkInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ContentLinkInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ContentLinkInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ContentLinkInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ContentLinkInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ContentLinkInfo> for ::windows::runtime::IInspectable {
    fn from(value: ContentLinkInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ContentLinkInfo> for ::windows::runtime::IInspectable {
    fn from(value: &ContentLinkInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ContentLinkInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ContentLinkInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ContentLinkInfo {}
unsafe impl ::std::marker::Sync for ContentLinkInfo {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FindOptions(pub u32);
impl FindOptions {
    pub const None: FindOptions = FindOptions(0u32);
    pub const Word: FindOptions = FindOptions(2u32);
    pub const Case: FindOptions = FindOptions(4u32);
}
impl ::std::convert::From<u32> for FindOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FindOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FindOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FindOptions;u4)");
}
impl ::std::ops::BitOr for FindOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FindOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FindOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FindOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FindOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FontStretch(pub i32);
impl FontStretch {
    pub const Undefined: FontStretch = FontStretch(0i32);
    pub const UltraCondensed: FontStretch = FontStretch(1i32);
    pub const ExtraCondensed: FontStretch = FontStretch(2i32);
    pub const Condensed: FontStretch = FontStretch(3i32);
    pub const SemiCondensed: FontStretch = FontStretch(4i32);
    pub const Normal: FontStretch = FontStretch(5i32);
    pub const SemiExpanded: FontStretch = FontStretch(6i32);
    pub const Expanded: FontStretch = FontStretch(7i32);
    pub const ExtraExpanded: FontStretch = FontStretch(8i32);
    pub const UltraExpanded: FontStretch = FontStretch(9i32);
}
impl ::std::convert::From<i32> for FontStretch {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FontStretch {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FontStretch {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FontStretch;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FontStyle(pub i32);
impl FontStyle {
    pub const Normal: FontStyle = FontStyle(0i32);
    pub const Oblique: FontStyle = FontStyle(1i32);
    pub const Italic: FontStyle = FontStyle(2i32);
}
impl ::std::convert::From<i32> for FontStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FontStyle {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FontStyle {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FontStyle;i4)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FontWeight {
    pub Weight: u16,
}
impl FontWeight {}
impl ::std::default::Default for FontWeight {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FontWeight {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FontWeight").field("Weight", &self.Weight).finish()
    }
}
impl ::std::cmp::PartialEq for FontWeight {
    fn eq(&self, other: &Self) -> bool {
        self.Weight == other.Weight
    }
}
impl ::std::cmp::Eq for FontWeight {}
unsafe impl ::windows::runtime::Abi for FontWeight {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FontWeight {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Text.FontWeight;u2)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct FontWeights(::windows::runtime::IInspectable);
impl FontWeights {
    pub fn Black() -> ::windows::runtime::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    pub fn Bold() -> ::windows::runtime::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    pub fn ExtraBlack() -> ::windows::runtime::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    pub fn ExtraBold() -> ::windows::runtime::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    pub fn ExtraLight() -> ::windows::runtime::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    pub fn Light() -> ::windows::runtime::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    pub fn Medium() -> ::windows::runtime::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    pub fn Normal() -> ::windows::runtime::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    pub fn SemiBold() -> ::windows::runtime::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    pub fn SemiLight() -> ::windows::runtime::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    pub fn Thin() -> ::windows::runtime::Result<FontWeight> {
        Self::IFontWeightsStatics(|this| unsafe {
            let mut result__: FontWeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FontWeight>(result__)
        })
    }
    pub fn IFontWeightsStatics<R, F: FnOnce(&IFontWeightsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FontWeights, IFontWeightsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FontWeights {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.FontWeights;{7880a444-01ab-4997-8517-df822a0c45f1})");
}
unsafe impl ::windows::runtime::Interface for FontWeights {
    type Vtable = IFontWeights_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2021696580, 427, 18839, [133, 23, 223, 130, 42, 12, 69, 241]);
}
impl ::windows::runtime::RuntimeName for FontWeights {
    const NAME: &'static str = "Windows.UI.Text.FontWeights";
}
impl ::std::convert::From<FontWeights> for ::windows::runtime::IUnknown {
    fn from(value: FontWeights) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FontWeights> for ::windows::runtime::IUnknown {
    fn from(value: &FontWeights) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FontWeights {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &FontWeights {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<FontWeights> for ::windows::runtime::IInspectable {
    fn from(value: FontWeights) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FontWeights> for ::windows::runtime::IInspectable {
    fn from(value: &FontWeights) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FontWeights {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FontWeights {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for FontWeights {}
unsafe impl ::std::marker::Sync for FontWeights {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FormatEffect(pub i32);
impl FormatEffect {
    pub const Off: FormatEffect = FormatEffect(0i32);
    pub const On: FormatEffect = FormatEffect(1i32);
    pub const Toggle: FormatEffect = FormatEffect(2i32);
    pub const Undefined: FormatEffect = FormatEffect(3i32);
}
impl ::std::convert::From<i32> for FormatEffect {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FormatEffect {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FormatEffect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.FormatEffect;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HorizontalCharacterAlignment(pub i32);
impl HorizontalCharacterAlignment {
    pub const Left: HorizontalCharacterAlignment = HorizontalCharacterAlignment(0i32);
    pub const Right: HorizontalCharacterAlignment = HorizontalCharacterAlignment(1i32);
    pub const Center: HorizontalCharacterAlignment = HorizontalCharacterAlignment(2i32);
}
impl ::std::convert::From<i32> for HorizontalCharacterAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HorizontalCharacterAlignment {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HorizontalCharacterAlignment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.HorizontalCharacterAlignment;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IContentLinkInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IContentLinkInfo {
    type Vtable = IContentLinkInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(517285157, 7263, 18635, [179, 53, 120, 181, 10, 46, 230, 66]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentLinkInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IFontWeights(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFontWeights {
    type Vtable = IFontWeights_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2021696580, 427, 18839, [133, 23, 223, 130, 42, 12, 69, 241]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontWeights_abi(
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
pub struct IFontWeightsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFontWeightsStatics {
    type Vtable = IFontWeightsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3015014869, 7081, 18667, [157, 173, 192, 149, 232, 194, 59, 163]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontWeightsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FontWeight) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FontWeight) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FontWeight) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FontWeight) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FontWeight) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FontWeight) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FontWeight) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FontWeight) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FontWeight) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FontWeight) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FontWeight) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IRichEditTextRange(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRichEditTextRange {
    type Vtable = IRichEditTextRange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(927872277, 47754, 19054, [140, 89, 13, 222, 61, 12, 245, 205]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRichEditTextRange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITextCharacterFormat(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextCharacterFormat {
    type Vtable = ITextCharacterFormat_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1524560859, 1531, 17453, [128, 101, 100, 42, 254, 160, 44, 237]);
}
impl ITextCharacterFormat {
    pub fn AllCaps(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetAllCaps(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn BackgroundColor(&self) -> ::windows::runtime::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Color>(result__)
        }
    }
    pub fn SetBackgroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Bold(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetBold(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn FontStretch(&self) -> ::windows::runtime::Result<FontStretch> {
        let this = self;
        unsafe {
            let mut result__: FontStretch = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FontStretch>(result__)
        }
    }
    pub fn SetFontStretch(&self, value: FontStretch) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn FontStyle(&self) -> ::windows::runtime::Result<FontStyle> {
        let this = self;
        unsafe {
            let mut result__: FontStyle = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FontStyle>(result__)
        }
    }
    pub fn SetFontStyle(&self, value: FontStyle) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ForegroundColor(&self) -> ::windows::runtime::Result<super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Color>(result__)
        }
    }
    pub fn SetForegroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Hidden(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetHidden(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Italic(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetItalic(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Kerning(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetKerning(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn LanguageTag(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetLanguageTag<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn LinkType(&self) -> ::windows::runtime::Result<LinkType> {
        let this = self;
        unsafe {
            let mut result__: LinkType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LinkType>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Outline(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetOutline(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Position(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetPosition(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ProtectedText(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetProtectedText(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Size(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetSize(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn SmallCaps(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetSmallCaps(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Spacing(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetSpacing(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Strikethrough(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetStrikethrough(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Subscript(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetSubscript(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Superscript(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetSuperscript(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).46)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn TextScript(&self) -> ::windows::runtime::Result<TextScript> {
        let this = self;
        unsafe {
            let mut result__: TextScript = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TextScript>(result__)
        }
    }
    pub fn SetTextScript(&self, value: TextScript) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).48)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Underline(&self) -> ::windows::runtime::Result<UnderlineType> {
        let this = self;
        unsafe {
            let mut result__: UnderlineType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnderlineType>(result__)
        }
    }
    pub fn SetUnderline(&self, value: UnderlineType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).50)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Weight(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetWeight(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).52)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn SetClone<'a, Param0: ::windows::runtime::IntoParam<'a, ITextCharacterFormat>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).53)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn GetClone(&self) -> ::windows::runtime::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextCharacterFormat>(result__)
        }
    }
    pub fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextCharacterFormat>>(&self, format: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(::std::mem::transmute_copy(this), format.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITextCharacterFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{5adef3db-05fb-442d-8065-642afea02ced}");
}
impl ::std::convert::From<ITextCharacterFormat> for ::windows::runtime::IUnknown {
    fn from(value: ITextCharacterFormat) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITextCharacterFormat> for ::windows::runtime::IUnknown {
    fn from(value: &ITextCharacterFormat) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextCharacterFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITextCharacterFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ITextCharacterFormat> for ::windows::runtime::IInspectable {
    fn from(value: ITextCharacterFormat) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextCharacterFormat> for ::windows::runtime::IInspectable {
    fn from(value: &ITextCharacterFormat) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ITextCharacterFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ITextCharacterFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextCharacterFormat_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FontStretch) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FontStretch) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FontStyle) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FontStyle) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LinkType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TextScript) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: TextScript) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UnderlineType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: UnderlineType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ITextConstantsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextConstantsStatics {
    type Vtable = ITextConstantsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2006875187, 6301, 19450, [151, 200, 16, 219, 19, 93, 151, 110]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextConstantsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FontStretch) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FontStyle) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITextDocument(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextDocument {
    type Vtable = ITextDocument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3203288539, 37042, 16524, [162, 246, 10, 10, 195, 30, 51, 228]);
}
impl ITextDocument {
    pub fn CaretType(&self) -> ::windows::runtime::Result<CaretType> {
        let this = self;
        unsafe {
            let mut result__: CaretType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CaretType>(result__)
        }
    }
    pub fn SetCaretType(&self, value: CaretType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn DefaultTabStop(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetDefaultTabStop(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Selection(&self) -> ::windows::runtime::Result<ITextSelection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextSelection>(result__)
        }
    }
    pub fn UndoLimit(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetUndoLimit(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn CanCopy(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CanPaste(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CanRedo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CanUndo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ApplyDisplayUpdates(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn BatchDisplayUpdates(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn BeginUndoGroup(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn EndUndoGroup(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn GetDefaultCharacterFormat(&self) -> ::windows::runtime::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextCharacterFormat>(result__)
        }
    }
    pub fn GetDefaultParagraphFormat(&self) -> ::windows::runtime::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextParagraphFormat>(result__)
        }
    }
    pub fn GetRange(&self, startposition: i32, endposition: i32) -> ::windows::runtime::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), startposition, endposition, &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetRangeFromPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, point: Param0, options: PointOptions) -> ::windows::runtime::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), point.into_param().abi(), options, &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    pub fn GetText(&self, options: TextGetOptions, value: &mut ::windows::runtime::HSTRING) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), options, value as *mut _ as _).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextSetOptions, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    pub fn Redo(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SaveToStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextGetOptions, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    pub fn SetDefaultCharacterFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ITextCharacterFormat>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SetDefaultParagraphFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ITextParagraphFormat>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SetText<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, options: TextSetOptions, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    pub fn Undo(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITextDocument {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{beee4ddb-90b2-408c-a2f6-0a0ac31e33e4}");
}
impl ::std::convert::From<ITextDocument> for ::windows::runtime::IUnknown {
    fn from(value: ITextDocument) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITextDocument> for ::windows::runtime::IUnknown {
    fn from(value: &ITextDocument) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITextDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ITextDocument> for ::windows::runtime::IInspectable {
    fn from(value: ITextDocument) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextDocument> for ::windows::runtime::IInspectable {
    fn from(value: &ITextDocument) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ITextDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ITextDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CaretType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CaretType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startposition: i32, endposition: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, point: super::super::Foundation::Point, options: PointOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: TextGetOptions, value: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: TextSetOptions, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: TextGetOptions, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: TextSetOptions, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ITextDocument2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextDocument2 {
    type Vtable = ITextDocument2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4063301906, 35977, 18889, [145, 24, 240, 87, 203, 184, 20, 238]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ITextDocument3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextDocument3 {
    type Vtable = ITextDocument3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1974141857, 42744, 17437, [170, 24, 10, 133, 29, 110, 94, 60]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ITextDocument4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextDocument4 {
    type Vtable = ITextDocument4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1637622002, 52027, 17697, [152, 31, 40, 101, 177, 185, 63, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextDocument4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: RichEditMathMode) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITextParagraphFormat(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextParagraphFormat {
    type Vtable = ITextParagraphFormat_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(754503590, 18038, 18826, [147, 245, 187, 219, 252, 11, 216, 131]);
}
impl ITextParagraphFormat {
    pub fn Alignment(&self) -> ::windows::runtime::Result<ParagraphAlignment> {
        let this = self;
        unsafe {
            let mut result__: ParagraphAlignment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ParagraphAlignment>(result__)
        }
    }
    pub fn SetAlignment(&self, value: ParagraphAlignment) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn FirstLineIndent(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn KeepTogether(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetKeepTogether(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn KeepWithNext(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetKeepWithNext(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn LeftIndent(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn LineSpacing(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn LineSpacingRule(&self) -> ::windows::runtime::Result<LineSpacingRule> {
        let this = self;
        unsafe {
            let mut result__: LineSpacingRule = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineSpacingRule>(result__)
        }
    }
    pub fn ListAlignment(&self) -> ::windows::runtime::Result<MarkerAlignment> {
        let this = self;
        unsafe {
            let mut result__: MarkerAlignment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MarkerAlignment>(result__)
        }
    }
    pub fn SetListAlignment(&self, value: MarkerAlignment) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ListLevelIndex(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetListLevelIndex(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ListStart(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetListStart(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ListStyle(&self) -> ::windows::runtime::Result<MarkerStyle> {
        let this = self;
        unsafe {
            let mut result__: MarkerStyle = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MarkerStyle>(result__)
        }
    }
    pub fn SetListStyle(&self, value: MarkerStyle) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ListTab(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetListTab(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ListType(&self) -> ::windows::runtime::Result<MarkerType> {
        let this = self;
        unsafe {
            let mut result__: MarkerType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MarkerType>(result__)
        }
    }
    pub fn SetListType(&self, value: MarkerType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn NoLineNumber(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetNoLineNumber(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn PageBreakBefore(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetPageBreakBefore(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn RightIndent(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetRightIndent(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn RightToLeft(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetRightToLeft(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Style(&self) -> ::windows::runtime::Result<ParagraphStyle> {
        let this = self;
        unsafe {
            let mut result__: ParagraphStyle = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ParagraphStyle>(result__)
        }
    }
    pub fn SetStyle(&self, value: ParagraphStyle) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn SpaceAfter(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetSpaceAfter(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn SpaceBefore(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetSpaceBefore(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn WidowControl(&self) -> ::windows::runtime::Result<FormatEffect> {
        let this = self;
        unsafe {
            let mut result__: FormatEffect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FormatEffect>(result__)
        }
    }
    pub fn SetWidowControl(&self, value: FormatEffect) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn TabCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn AddTab(&self, position: f32, align: TabAlignment, leader: TabLeader) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).45)(::std::mem::transmute_copy(this), position, align, leader).ok() }
    }
    pub fn ClearAllTabs(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).46)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn DeleteTab(&self, position: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).47)(::std::mem::transmute_copy(this), position).ok() }
    }
    pub fn GetClone(&self) -> ::windows::runtime::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextParagraphFormat>(result__)
        }
    }
    pub fn GetTab(&self, index: i32, position: &mut f32, align: &mut TabAlignment, leader: &mut TabLeader) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).49)(::std::mem::transmute_copy(this), index, position, align, leader).ok() }
    }
    pub fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextParagraphFormat>>(&self, format: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(::std::mem::transmute_copy(this), format.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetClone<'a, Param0: ::windows::runtime::IntoParam<'a, ITextParagraphFormat>>(&self, format: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this), format.into_param().abi()).ok() }
    }
    pub fn SetIndents(&self, start: f32, left: f32, right: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).52)(::std::mem::transmute_copy(this), start, left, right).ok() }
    }
    pub fn SetLineSpacing(&self, rule: LineSpacingRule, spacing: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).53)(::std::mem::transmute_copy(this), rule, spacing).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITextParagraphFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{2cf8cfa6-4676-498a-93f5-bbdbfc0bd883}");
}
impl ::std::convert::From<ITextParagraphFormat> for ::windows::runtime::IUnknown {
    fn from(value: ITextParagraphFormat) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITextParagraphFormat> for ::windows::runtime::IUnknown {
    fn from(value: &ITextParagraphFormat) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextParagraphFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITextParagraphFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ITextParagraphFormat> for ::windows::runtime::IInspectable {
    fn from(value: ITextParagraphFormat) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextParagraphFormat> for ::windows::runtime::IInspectable {
    fn from(value: &ITextParagraphFormat) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ITextParagraphFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ITextParagraphFormat {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextParagraphFormat_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ParagraphAlignment) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ParagraphAlignment) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LineSpacingRule) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MarkerAlignment) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MarkerAlignment) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MarkerStyle) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MarkerStyle) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MarkerType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MarkerType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ParagraphStyle) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ParagraphStyle) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: FormatEffect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: f32, align: TabAlignment, leader: TabLeader) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, position: *mut f32, align: *mut TabAlignment, leader: *mut TabLeader) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, start: f32, left: f32, right: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rule: LineSpacingRule, spacing: f32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITextRange(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextRange {
    type Vtable = ITextRange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1537101399, 49266, 17056, [137, 69, 175, 80, 62, 229, 71, 104]);
}
impl ITextRange {
    pub fn Character(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn SetCharacter(&self, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn CharacterFormat(&self) -> ::windows::runtime::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextCharacterFormat>(result__)
        }
    }
    pub fn SetCharacterFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ITextCharacterFormat>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FormattedText(&self) -> ::windows::runtime::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    pub fn SetFormattedText<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EndPosition(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetEndPosition(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Gravity(&self) -> ::windows::runtime::Result<RangeGravity> {
        let this = self;
        unsafe {
            let mut result__: RangeGravity = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RangeGravity>(result__)
        }
    }
    pub fn SetGravity(&self, value: RangeGravity) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Length(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Link(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetLink<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ParagraphFormat(&self) -> ::windows::runtime::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextParagraphFormat>(result__)
        }
    }
    pub fn SetParagraphFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ITextParagraphFormat>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn StartPosition(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetStartPosition(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn StoryLength(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn CanPaste(&self, format: i32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ChangeCase(&self, value: LetterCase) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Collapse(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Copy(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn Cut(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn Delete(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Expand(&self, unit: TextRangeUnit) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), unit, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn FindText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0, scanlength: i32, options: FindOptions) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), value.into_param().abi(), scanlength, options, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn GetCharacterUtf32(&self, value: &mut u32, offset: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), value, offset).ok() }
    }
    pub fn GetClone(&self) -> ::windows::runtime::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    pub fn GetIndex(&self, unit: TextRangeUnit) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), unit, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPoint(&self, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), horizontalalign, verticalalign, options, point).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetRect(&self, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), options, rect, hit).ok() }
    }
    pub fn GetText(&self, options: TextGetOptions, value: &mut ::windows::runtime::HSTRING) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), options, value as *mut _ as _).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetTextViaStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextGetOptions, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    pub fn InRange<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn InsertImage<'a, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param5: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: Param4, value: Param5) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), width, height, ascent, verticalalign, alternatetext.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn InStory<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(::std::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Move(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(::std::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(::std::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(::std::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Paste(&self, format: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).49)(::std::mem::transmute_copy(this), format).ok() }
    }
    pub fn ScrollIntoView(&self, value: PointOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).50)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn MatchSelection(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn SetIndex(&self, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).52)(::std::mem::transmute_copy(this), unit, index, extend).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, point: Param0, options: PointOptions, extend: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).53)(::std::mem::transmute_copy(this), point.into_param().abi(), options, extend).ok() }
    }
    pub fn SetRange(&self, startposition: i32, endposition: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).54)(::std::mem::transmute_copy(this), startposition, endposition).ok() }
    }
    pub fn SetText2<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, options: TextSetOptions, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).55)(::std::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetTextViaStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextSetOptions, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).56)(::std::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    pub fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(::std::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITextRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{5b9e4e57-c072-42a0-8945-af503ee54768}");
}
impl ::std::convert::From<ITextRange> for ::windows::runtime::IUnknown {
    fn from(value: ITextRange) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITextRange> for ::windows::runtime::IUnknown {
    fn from(value: &ITextRange) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ITextRange> for ::windows::runtime::IInspectable {
    fn from(value: ITextRange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextRange> for ::windows::runtime::IInspectable {
    fn from(value: &ITextRange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ITextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ITextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut RangeGravity) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: RangeGravity) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: i32, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: LetterCase) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: TextRangeUnit, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, scanlength: i32, options: FindOptions, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut u32, offset: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: TextRangeUnit, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: PointOptions, rect: *mut super::super::Foundation::Rect, hit: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: TextGetOptions, value: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: TextGetOptions, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, range: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, range: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, range: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PointOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, point: super::super::Foundation::Point, options: PointOptions, extend: bool) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, startposition: i32, endposition: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: TextSetOptions, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: TextSetOptions, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITextSelection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITextSelection {
    type Vtable = ITextSelection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2798872356, 62095, 17162, [178, 207, 195, 67, 103, 30, 192, 233]);
}
impl ITextSelection {
    pub fn Options(&self) -> ::windows::runtime::Result<SelectionOptions> {
        let this = self;
        unsafe {
            let mut result__: SelectionOptions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SelectionOptions>(result__)
        }
    }
    pub fn SetOptions(&self, value: SelectionOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Type(&self) -> ::windows::runtime::Result<SelectionType> {
        let this = self;
        unsafe {
            let mut result__: SelectionType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SelectionType>(result__)
        }
    }
    pub fn EndKey(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn HomeKey(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn MoveDown(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), unit, count, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn MoveLeft(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), unit, count, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn MoveRight(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), unit, count, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn MoveUp(&self, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), unit, count, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn TypeText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Character(&self) -> ::windows::runtime::Result<u16> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn SetCharacter(&self, value: u16) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn CharacterFormat(&self) -> ::windows::runtime::Result<ITextCharacterFormat> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextCharacterFormat>(result__)
        }
    }
    pub fn SetCharacterFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ITextCharacterFormat>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FormattedText(&self) -> ::windows::runtime::Result<ITextRange> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    pub fn SetFormattedText<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EndPosition(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetEndPosition(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Gravity(&self) -> ::windows::runtime::Result<RangeGravity> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: RangeGravity = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RangeGravity>(result__)
        }
    }
    pub fn SetGravity(&self, value: RangeGravity) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Length(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Link(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetLink<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ParagraphFormat(&self) -> ::windows::runtime::Result<ITextParagraphFormat> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextParagraphFormat>(result__)
        }
    }
    pub fn SetParagraphFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ITextParagraphFormat>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn StartPosition(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetStartPosition(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn StoryLength(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn CanPaste(&self, format: i32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ChangeCase(&self, value: LetterCase) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Collapse(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Copy(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn Cut(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn Delete(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Expand(&self, unit: TextRangeUnit) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), unit, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn FindText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0, scanlength: i32, options: FindOptions) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), value.into_param().abi(), scanlength, options, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn GetCharacterUtf32(&self, value: &mut u32, offset: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), value, offset).ok() }
    }
    pub fn GetClone(&self) -> ::windows::runtime::Result<ITextRange> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    pub fn GetIndex(&self, unit: TextRangeUnit) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), unit, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPoint(&self, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), horizontalalign, verticalalign, options, point).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetRect(&self, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), options, rect, hit).ok() }
    }
    pub fn GetText(&self, options: TextGetOptions, value: &mut ::windows::runtime::HSTRING) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), options, value as *mut _ as _).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetTextViaStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextGetOptions, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    pub fn InRange<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn InsertImage<'a, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param5: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: Param4, value: Param5) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), width, height, ascent, verticalalign, alternatetext.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn InStory<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(::std::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Move(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(::std::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(::std::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(::std::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Paste(&self, format: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).49)(::std::mem::transmute_copy(this), format).ok() }
    }
    pub fn ScrollIntoView(&self, value: PointOptions) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).50)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn MatchSelection(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn SetIndex(&self, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).52)(::std::mem::transmute_copy(this), unit, index, extend).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, point: Param0, options: PointOptions, extend: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).53)(::std::mem::transmute_copy(this), point.into_param().abi(), options, extend).ok() }
    }
    pub fn SetRange(&self, startposition: i32, endposition: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).54)(::std::mem::transmute_copy(this), startposition, endposition).ok() }
    }
    pub fn SetText2<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, options: TextSetOptions, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).55)(::std::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetTextViaStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextSetOptions, value: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).56)(::std::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    pub fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<ITextRange>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(::std::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ITextSelection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{a6d36724-f28f-430a-b2cf-c343671ec0e9}");
}
impl ::std::convert::From<ITextSelection> for ::windows::runtime::IUnknown {
    fn from(value: ITextSelection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ITextSelection> for ::windows::runtime::IUnknown {
    fn from(value: &ITextSelection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ITextSelection> for ::windows::runtime::IInspectable {
    fn from(value: ITextSelection) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextSelection> for ::windows::runtime::IInspectable {
    fn from(value: &ITextSelection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ITextSelection> for ITextRange {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ITextSelection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ITextSelection> for ITextRange {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ITextSelection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange> for ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange> for &ITextSelection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange> {
        ::std::convert::TryInto::<ITextRange>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextSelection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SelectionOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SelectionOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SelectionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LetterCase(pub i32);
impl LetterCase {
    pub const Lower: LetterCase = LetterCase(0i32);
    pub const Upper: LetterCase = LetterCase(1i32);
}
impl ::std::convert::From<i32> for LetterCase {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LetterCase {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LetterCase {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.LetterCase;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineSpacingRule(pub i32);
impl LineSpacingRule {
    pub const Undefined: LineSpacingRule = LineSpacingRule(0i32);
    pub const Single: LineSpacingRule = LineSpacingRule(1i32);
    pub const OneAndHalf: LineSpacingRule = LineSpacingRule(2i32);
    pub const Double: LineSpacingRule = LineSpacingRule(3i32);
    pub const AtLeast: LineSpacingRule = LineSpacingRule(4i32);
    pub const Exactly: LineSpacingRule = LineSpacingRule(5i32);
    pub const Multiple: LineSpacingRule = LineSpacingRule(6i32);
    pub const Percent: LineSpacingRule = LineSpacingRule(7i32);
}
impl ::std::convert::From<i32> for LineSpacingRule {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LineSpacingRule {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LineSpacingRule {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.LineSpacingRule;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LinkType(pub i32);
impl LinkType {
    pub const Undefined: LinkType = LinkType(0i32);
    pub const NotALink: LinkType = LinkType(1i32);
    pub const ClientLink: LinkType = LinkType(2i32);
    pub const FriendlyLinkName: LinkType = LinkType(3i32);
    pub const FriendlyLinkAddress: LinkType = LinkType(4i32);
    pub const AutoLink: LinkType = LinkType(5i32);
    pub const AutoLinkEmail: LinkType = LinkType(6i32);
    pub const AutoLinkPhone: LinkType = LinkType(7i32);
    pub const AutoLinkPath: LinkType = LinkType(8i32);
}
impl ::std::convert::From<i32> for LinkType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LinkType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LinkType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.LinkType;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MarkerAlignment(pub i32);
impl MarkerAlignment {
    pub const Undefined: MarkerAlignment = MarkerAlignment(0i32);
    pub const Left: MarkerAlignment = MarkerAlignment(1i32);
    pub const Center: MarkerAlignment = MarkerAlignment(2i32);
    pub const Right: MarkerAlignment = MarkerAlignment(3i32);
}
impl ::std::convert::From<i32> for MarkerAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MarkerAlignment {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MarkerAlignment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.MarkerAlignment;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MarkerStyle(pub i32);
impl MarkerStyle {
    pub const Undefined: MarkerStyle = MarkerStyle(0i32);
    pub const Parenthesis: MarkerStyle = MarkerStyle(1i32);
    pub const Parentheses: MarkerStyle = MarkerStyle(2i32);
    pub const Period: MarkerStyle = MarkerStyle(3i32);
    pub const Plain: MarkerStyle = MarkerStyle(4i32);
    pub const Minus: MarkerStyle = MarkerStyle(5i32);
    pub const NoNumber: MarkerStyle = MarkerStyle(6i32);
}
impl ::std::convert::From<i32> for MarkerStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MarkerStyle {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MarkerStyle {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.MarkerStyle;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MarkerType(pub i32);
impl MarkerType {
    pub const Undefined: MarkerType = MarkerType(0i32);
    pub const None: MarkerType = MarkerType(1i32);
    pub const Bullet: MarkerType = MarkerType(2i32);
    pub const Arabic: MarkerType = MarkerType(3i32);
    pub const LowercaseEnglishLetter: MarkerType = MarkerType(4i32);
    pub const UppercaseEnglishLetter: MarkerType = MarkerType(5i32);
    pub const LowercaseRoman: MarkerType = MarkerType(6i32);
    pub const UppercaseRoman: MarkerType = MarkerType(7i32);
    pub const UnicodeSequence: MarkerType = MarkerType(8i32);
    pub const CircledNumber: MarkerType = MarkerType(9i32);
    pub const BlackCircleWingding: MarkerType = MarkerType(10i32);
    pub const WhiteCircleWingding: MarkerType = MarkerType(11i32);
    pub const ArabicWide: MarkerType = MarkerType(12i32);
    pub const SimplifiedChinese: MarkerType = MarkerType(13i32);
    pub const TraditionalChinese: MarkerType = MarkerType(14i32);
    pub const JapanSimplifiedChinese: MarkerType = MarkerType(15i32);
    pub const JapanKorea: MarkerType = MarkerType(16i32);
    pub const ArabicDictionary: MarkerType = MarkerType(17i32);
    pub const ArabicAbjad: MarkerType = MarkerType(18i32);
    pub const Hebrew: MarkerType = MarkerType(19i32);
    pub const ThaiAlphabetic: MarkerType = MarkerType(20i32);
    pub const ThaiNumeric: MarkerType = MarkerType(21i32);
    pub const DevanagariVowel: MarkerType = MarkerType(22i32);
    pub const DevanagariConsonant: MarkerType = MarkerType(23i32);
    pub const DevanagariNumeric: MarkerType = MarkerType(24i32);
}
impl ::std::convert::From<i32> for MarkerType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MarkerType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MarkerType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.MarkerType;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ParagraphAlignment(pub i32);
impl ParagraphAlignment {
    pub const Undefined: ParagraphAlignment = ParagraphAlignment(0i32);
    pub const Left: ParagraphAlignment = ParagraphAlignment(1i32);
    pub const Center: ParagraphAlignment = ParagraphAlignment(2i32);
    pub const Right: ParagraphAlignment = ParagraphAlignment(3i32);
    pub const Justify: ParagraphAlignment = ParagraphAlignment(4i32);
}
impl ::std::convert::From<i32> for ParagraphAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ParagraphAlignment {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ParagraphAlignment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.ParagraphAlignment;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ParagraphStyle(pub i32);
impl ParagraphStyle {
    pub const Undefined: ParagraphStyle = ParagraphStyle(0i32);
    pub const None: ParagraphStyle = ParagraphStyle(1i32);
    pub const Normal: ParagraphStyle = ParagraphStyle(2i32);
    pub const Heading1: ParagraphStyle = ParagraphStyle(3i32);
    pub const Heading2: ParagraphStyle = ParagraphStyle(4i32);
    pub const Heading3: ParagraphStyle = ParagraphStyle(5i32);
    pub const Heading4: ParagraphStyle = ParagraphStyle(6i32);
    pub const Heading5: ParagraphStyle = ParagraphStyle(7i32);
    pub const Heading6: ParagraphStyle = ParagraphStyle(8i32);
    pub const Heading7: ParagraphStyle = ParagraphStyle(9i32);
    pub const Heading8: ParagraphStyle = ParagraphStyle(10i32);
    pub const Heading9: ParagraphStyle = ParagraphStyle(11i32);
}
impl ::std::convert::From<i32> for ParagraphStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ParagraphStyle {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ParagraphStyle {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.ParagraphStyle;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PointOptions(pub u32);
impl PointOptions {
    pub const None: PointOptions = PointOptions(0u32);
    pub const IncludeInset: PointOptions = PointOptions(1u32);
    pub const Start: PointOptions = PointOptions(32u32);
    pub const ClientCoordinates: PointOptions = PointOptions(256u32);
    pub const AllowOffClient: PointOptions = PointOptions(512u32);
    pub const Transform: PointOptions = PointOptions(1024u32);
    pub const NoHorizontalScroll: PointOptions = PointOptions(65536u32);
    pub const NoVerticalScroll: PointOptions = PointOptions(262144u32);
}
impl ::std::convert::From<u32> for PointOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PointOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PointOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.PointOptions;u4)");
}
impl ::std::ops::BitOr for PointOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PointOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PointOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PointOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PointOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RangeGravity(pub i32);
impl RangeGravity {
    pub const UIBehavior: RangeGravity = RangeGravity(0i32);
    pub const Backward: RangeGravity = RangeGravity(1i32);
    pub const Forward: RangeGravity = RangeGravity(2i32);
    pub const Inward: RangeGravity = RangeGravity(3i32);
    pub const Outward: RangeGravity = RangeGravity(4i32);
}
impl ::std::convert::From<i32> for RangeGravity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RangeGravity {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RangeGravity {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.RangeGravity;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RichEditMathMode(pub i32);
impl RichEditMathMode {
    pub const NoMath: RichEditMathMode = RichEditMathMode(0i32);
    pub const MathOnly: RichEditMathMode = RichEditMathMode(1i32);
}
impl ::std::convert::From<i32> for RichEditMathMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RichEditMathMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RichEditMathMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.RichEditMathMode;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RichEditTextDocument(::windows::runtime::IInspectable);
impl RichEditTextDocument {
    pub fn CaretType(&self) -> ::windows::runtime::Result<CaretType> {
        let this = self;
        unsafe {
            let mut result__: CaretType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CaretType>(result__)
        }
    }
    pub fn SetCaretType(&self, value: CaretType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn DefaultTabStop(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetDefaultTabStop(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Selection(&self) -> ::windows::runtime::Result<ITextSelection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextSelection>(result__)
        }
    }
    pub fn UndoLimit(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetUndoLimit(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn CanCopy(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CanPaste(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CanRedo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn CanUndo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ApplyDisplayUpdates(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn BatchDisplayUpdates(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn BeginUndoGroup(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn EndUndoGroup(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn GetDefaultCharacterFormat(&self) -> ::windows::runtime::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextCharacterFormat>(result__)
        }
    }
    pub fn GetDefaultParagraphFormat(&self) -> ::windows::runtime::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextParagraphFormat>(result__)
        }
    }
    pub fn GetRange(&self, startposition: i32, endposition: i32) -> ::windows::runtime::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), startposition, endposition, &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetRangeFromPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, point: Param0, options: PointOptions) -> ::windows::runtime::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), point.into_param().abi(), options, &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    pub fn GetText(&self, options: TextGetOptions, value: &mut ::windows::runtime::HSTRING) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), options, value as *mut _ as _).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadFromStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextSetOptions, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    pub fn Redo(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SaveToStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextGetOptions, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    pub fn SetDefaultCharacterFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ITextCharacterFormat>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SetDefaultParagraphFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ITextParagraphFormat>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SetText<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, options: TextSetOptions, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    pub fn Undo(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn AlignmentIncludesTrailingWhitespace(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextDocument2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAlignmentIncludesTrailingWhitespace(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextDocument2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn IgnoreTrailingCharacterSpacing(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ITextDocument2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIgnoreTrailingCharacterSpacing(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextDocument2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn ClearUndoRedoHistory(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextDocument3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn SetMath<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextDocument4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn GetMath(&self, value: &mut ::windows::runtime::HSTRING) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextDocument4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value as *mut _ as _).ok() }
    }
    pub fn SetMathMode(&self, mode: RichEditMathMode) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ITextDocument4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), mode).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RichEditTextDocument {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.RichEditTextDocument;{beee4ddb-90b2-408c-a2f6-0a0ac31e33e4})");
}
unsafe impl ::windows::runtime::Interface for RichEditTextDocument {
    type Vtable = ITextDocument_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3203288539, 37042, 16524, [162, 246, 10, 10, 195, 30, 51, 228]);
}
impl ::windows::runtime::RuntimeName for RichEditTextDocument {
    const NAME: &'static str = "Windows.UI.Text.RichEditTextDocument";
}
impl ::std::convert::From<RichEditTextDocument> for ::windows::runtime::IUnknown {
    fn from(value: RichEditTextDocument) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RichEditTextDocument> for ::windows::runtime::IUnknown {
    fn from(value: &RichEditTextDocument) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RichEditTextDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RichEditTextDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<RichEditTextDocument> for ::windows::runtime::IInspectable {
    fn from(value: RichEditTextDocument) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RichEditTextDocument> for ::windows::runtime::IInspectable {
    fn from(value: &RichEditTextDocument) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RichEditTextDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RichEditTextDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<RichEditTextDocument> for ITextDocument {
    fn from(value: RichEditTextDocument) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RichEditTextDocument> for ITextDocument {
    fn from(value: &RichEditTextDocument) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextDocument> for RichEditTextDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextDocument> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITextDocument>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextDocument> for &RichEditTextDocument {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextDocument> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITextDocument>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for RichEditTextDocument {}
unsafe impl ::std::marker::Sync for RichEditTextDocument {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RichEditTextRange(::windows::runtime::IInspectable);
impl RichEditTextRange {
    pub fn Character(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    pub fn SetCharacter(&self, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn CharacterFormat(&self) -> ::windows::runtime::Result<ITextCharacterFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextCharacterFormat>(result__)
        }
    }
    pub fn SetCharacterFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ITextCharacterFormat>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn FormattedText(&self) -> ::windows::runtime::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    pub fn SetFormattedText<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn EndPosition(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetEndPosition(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Gravity(&self) -> ::windows::runtime::Result<RangeGravity> {
        let this = self;
        unsafe {
            let mut result__: RangeGravity = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<RangeGravity>(result__)
        }
    }
    pub fn SetGravity(&self, value: RangeGravity) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Length(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Link(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetLink<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ParagraphFormat(&self) -> ::windows::runtime::Result<ITextParagraphFormat> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextParagraphFormat>(result__)
        }
    }
    pub fn SetParagraphFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ITextParagraphFormat>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn StartPosition(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetStartPosition(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn StoryLength(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn CanPaste(&self, format: i32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn ChangeCase(&self, value: LetterCase) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Collapse(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn Copy(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn Cut(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn Delete(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn EndOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Expand(&self, unit: TextRangeUnit) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), unit, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn FindText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0, scanlength: i32, options: FindOptions) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), value.into_param().abi(), scanlength, options, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn GetCharacterUtf32(&self, value: &mut u32, offset: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), value, offset).ok() }
    }
    pub fn GetClone(&self) -> ::windows::runtime::Result<ITextRange> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ITextRange>(result__)
        }
    }
    pub fn GetIndex(&self, unit: TextRangeUnit) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), unit, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPoint(&self, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), horizontalalign, verticalalign, options, point).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetRect(&self, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), options, rect, hit).ok() }
    }
    pub fn GetText(&self, options: TextGetOptions, value: &mut ::windows::runtime::HSTRING) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), options, value as *mut _ as _).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetTextViaStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextGetOptions, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    pub fn InRange<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn InsertImage<'a, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param5: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: Param4, value: Param5) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), width, height, ascent, verticalalign, alternatetext.into_param().abi(), value.into_param().abi()).ok() }
    }
    pub fn InStory<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, ITextRange>>(&self, range: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(::std::mem::transmute_copy(this), range.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn Move(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(::std::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn MoveEnd(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(::std::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn MoveStart(&self, unit: TextRangeUnit, count: i32) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(::std::mem::transmute_copy(this), unit, count, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Paste(&self, format: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).49)(::std::mem::transmute_copy(this), format).ok() }
    }
    pub fn ScrollIntoView(&self, value: PointOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).50)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn MatchSelection(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn SetIndex(&self, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).52)(::std::mem::transmute_copy(this), unit, index, extend).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, point: Param0, options: PointOptions, extend: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).53)(::std::mem::transmute_copy(this), point.into_param().abi(), options, extend).ok() }
    }
    pub fn SetRange(&self, startposition: i32, endposition: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).54)(::std::mem::transmute_copy(this), startposition, endposition).ok() }
    }
    pub fn SetText2<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, options: TextSetOptions, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).55)(::std::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetTextViaStream<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(&self, options: TextSetOptions, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).56)(::std::mem::transmute_copy(this), options, value.into_param().abi()).ok() }
    }
    pub fn StartOf(&self, unit: TextRangeUnit, extend: bool) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(::std::mem::transmute_copy(this), unit, extend, &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn ContentLinkInfo(&self) -> ::windows::runtime::Result<ContentLinkInfo> {
        let this = &::windows::runtime::Interface::cast::<IRichEditTextRange>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ContentLinkInfo>(result__)
        }
    }
    pub fn SetContentLinkInfo<'a, Param0: ::windows::runtime::IntoParam<'a, ContentLinkInfo>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IRichEditTextRange>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RichEditTextRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.RichEditTextRange;{5b9e4e57-c072-42a0-8945-af503ee54768})");
}
unsafe impl ::windows::runtime::Interface for RichEditTextRange {
    type Vtable = ITextRange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1537101399, 49266, 17056, [137, 69, 175, 80, 62, 229, 71, 104]);
}
impl ::windows::runtime::RuntimeName for RichEditTextRange {
    const NAME: &'static str = "Windows.UI.Text.RichEditTextRange";
}
impl ::std::convert::From<RichEditTextRange> for ::windows::runtime::IUnknown {
    fn from(value: RichEditTextRange) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RichEditTextRange> for ::windows::runtime::IUnknown {
    fn from(value: &RichEditTextRange) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RichEditTextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RichEditTextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<RichEditTextRange> for ::windows::runtime::IInspectable {
    fn from(value: RichEditTextRange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RichEditTextRange> for ::windows::runtime::IInspectable {
    fn from(value: &RichEditTextRange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RichEditTextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RichEditTextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<RichEditTextRange> for ITextRange {
    fn from(value: RichEditTextRange) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RichEditTextRange> for ITextRange {
    fn from(value: &RichEditTextRange) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange> for RichEditTextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITextRange>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ITextRange> for &RichEditTextRange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ITextRange> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ITextRange>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for RichEditTextRange {}
unsafe impl ::std::marker::Sync for RichEditTextRange {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SelectionOptions(pub u32);
impl SelectionOptions {
    pub const StartActive: SelectionOptions = SelectionOptions(1u32);
    pub const AtEndOfLine: SelectionOptions = SelectionOptions(2u32);
    pub const Overtype: SelectionOptions = SelectionOptions(4u32);
    pub const Active: SelectionOptions = SelectionOptions(8u32);
    pub const Replace: SelectionOptions = SelectionOptions(16u32);
}
impl ::std::convert::From<u32> for SelectionOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SelectionOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SelectionOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.SelectionOptions;u4)");
}
impl ::std::ops::BitOr for SelectionOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SelectionOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SelectionOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SelectionOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SelectionOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SelectionType(pub i32);
impl SelectionType {
    pub const None: SelectionType = SelectionType(0i32);
    pub const InsertionPoint: SelectionType = SelectionType(1i32);
    pub const Normal: SelectionType = SelectionType(2i32);
    pub const InlineShape: SelectionType = SelectionType(7i32);
    pub const Shape: SelectionType = SelectionType(8i32);
}
impl ::std::convert::From<i32> for SelectionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SelectionType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SelectionType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.SelectionType;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TabAlignment(pub i32);
impl TabAlignment {
    pub const Left: TabAlignment = TabAlignment(0i32);
    pub const Center: TabAlignment = TabAlignment(1i32);
    pub const Right: TabAlignment = TabAlignment(2i32);
    pub const Decimal: TabAlignment = TabAlignment(3i32);
    pub const Bar: TabAlignment = TabAlignment(4i32);
}
impl ::std::convert::From<i32> for TabAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TabAlignment {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TabAlignment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TabAlignment;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TabLeader(pub i32);
impl TabLeader {
    pub const Spaces: TabLeader = TabLeader(0i32);
    pub const Dots: TabLeader = TabLeader(1i32);
    pub const Dashes: TabLeader = TabLeader(2i32);
    pub const Lines: TabLeader = TabLeader(3i32);
    pub const ThickLines: TabLeader = TabLeader(4i32);
    pub const Equals: TabLeader = TabLeader(5i32);
}
impl ::std::convert::From<i32> for TabLeader {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TabLeader {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TabLeader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TabLeader;i4)");
}
pub struct TextConstants {}
impl TextConstants {
    pub fn AutoColor() -> ::windows::runtime::Result<super::Color> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Color>(result__)
        })
    }
    pub fn MinUnitCount() -> ::windows::runtime::Result<i32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    pub fn MaxUnitCount() -> ::windows::runtime::Result<i32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    pub fn UndefinedColor() -> ::windows::runtime::Result<super::Color> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Color>(result__)
        })
    }
    pub fn UndefinedFloatValue() -> ::windows::runtime::Result<f32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        })
    }
    pub fn UndefinedInt32Value() -> ::windows::runtime::Result<i32> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        })
    }
    pub fn UndefinedFontStretch() -> ::windows::runtime::Result<FontStretch> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: FontStretch = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FontStretch>(result__)
        })
    }
    pub fn UndefinedFontStyle() -> ::windows::runtime::Result<FontStyle> {
        Self::ITextConstantsStatics(|this| unsafe {
            let mut result__: FontStyle = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FontStyle>(result__)
        })
    }
    pub fn ITextConstantsStatics<R, F: FnOnce(&ITextConstantsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<TextConstants, ITextConstantsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for TextConstants {
    const NAME: &'static str = "Windows.UI.Text.TextConstants";
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TextDecorations(pub u32);
impl TextDecorations {
    pub const None: TextDecorations = TextDecorations(0u32);
    pub const Underline: TextDecorations = TextDecorations(1u32);
    pub const Strikethrough: TextDecorations = TextDecorations(2u32);
}
impl ::std::convert::From<u32> for TextDecorations {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TextDecorations {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TextDecorations {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextDecorations;u4)");
}
impl ::std::ops::BitOr for TextDecorations {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TextDecorations {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TextDecorations {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TextDecorations {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TextDecorations {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TextGetOptions(pub u32);
impl TextGetOptions {
    pub const None: TextGetOptions = TextGetOptions(0u32);
    pub const AdjustCrlf: TextGetOptions = TextGetOptions(1u32);
    pub const UseCrlf: TextGetOptions = TextGetOptions(2u32);
    pub const UseObjectText: TextGetOptions = TextGetOptions(4u32);
    pub const AllowFinalEop: TextGetOptions = TextGetOptions(8u32);
    pub const NoHidden: TextGetOptions = TextGetOptions(32u32);
    pub const IncludeNumbering: TextGetOptions = TextGetOptions(64u32);
    pub const FormatRtf: TextGetOptions = TextGetOptions(8192u32);
    pub const UseLf: TextGetOptions = TextGetOptions(16777216u32);
}
impl ::std::convert::From<u32> for TextGetOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TextGetOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TextGetOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextGetOptions;u4)");
}
impl ::std::ops::BitOr for TextGetOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TextGetOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TextGetOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TextGetOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TextGetOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TextRangeUnit(pub i32);
impl TextRangeUnit {
    pub const Character: TextRangeUnit = TextRangeUnit(0i32);
    pub const Word: TextRangeUnit = TextRangeUnit(1i32);
    pub const Sentence: TextRangeUnit = TextRangeUnit(2i32);
    pub const Paragraph: TextRangeUnit = TextRangeUnit(3i32);
    pub const Line: TextRangeUnit = TextRangeUnit(4i32);
    pub const Story: TextRangeUnit = TextRangeUnit(5i32);
    pub const Screen: TextRangeUnit = TextRangeUnit(6i32);
    pub const Section: TextRangeUnit = TextRangeUnit(7i32);
    pub const Window: TextRangeUnit = TextRangeUnit(8i32);
    pub const CharacterFormat: TextRangeUnit = TextRangeUnit(9i32);
    pub const ParagraphFormat: TextRangeUnit = TextRangeUnit(10i32);
    pub const Object: TextRangeUnit = TextRangeUnit(11i32);
    pub const HardParagraph: TextRangeUnit = TextRangeUnit(12i32);
    pub const Cluster: TextRangeUnit = TextRangeUnit(13i32);
    pub const Bold: TextRangeUnit = TextRangeUnit(14i32);
    pub const Italic: TextRangeUnit = TextRangeUnit(15i32);
    pub const Underline: TextRangeUnit = TextRangeUnit(16i32);
    pub const Strikethrough: TextRangeUnit = TextRangeUnit(17i32);
    pub const ProtectedText: TextRangeUnit = TextRangeUnit(18i32);
    pub const Link: TextRangeUnit = TextRangeUnit(19i32);
    pub const SmallCaps: TextRangeUnit = TextRangeUnit(20i32);
    pub const AllCaps: TextRangeUnit = TextRangeUnit(21i32);
    pub const Hidden: TextRangeUnit = TextRangeUnit(22i32);
    pub const Outline: TextRangeUnit = TextRangeUnit(23i32);
    pub const Shadow: TextRangeUnit = TextRangeUnit(24i32);
    pub const Imprint: TextRangeUnit = TextRangeUnit(25i32);
    pub const Disabled: TextRangeUnit = TextRangeUnit(26i32);
    pub const Revised: TextRangeUnit = TextRangeUnit(27i32);
    pub const Subscript: TextRangeUnit = TextRangeUnit(28i32);
    pub const Superscript: TextRangeUnit = TextRangeUnit(29i32);
    pub const FontBound: TextRangeUnit = TextRangeUnit(30i32);
    pub const LinkProtected: TextRangeUnit = TextRangeUnit(31i32);
    pub const ContentLink: TextRangeUnit = TextRangeUnit(32i32);
}
impl ::std::convert::From<i32> for TextRangeUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TextRangeUnit {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TextRangeUnit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextRangeUnit;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TextScript(pub i32);
impl TextScript {
    pub const Undefined: TextScript = TextScript(0i32);
    pub const Ansi: TextScript = TextScript(1i32);
    pub const EastEurope: TextScript = TextScript(2i32);
    pub const Cyrillic: TextScript = TextScript(3i32);
    pub const Greek: TextScript = TextScript(4i32);
    pub const Turkish: TextScript = TextScript(5i32);
    pub const Hebrew: TextScript = TextScript(6i32);
    pub const Arabic: TextScript = TextScript(7i32);
    pub const Baltic: TextScript = TextScript(8i32);
    pub const Vietnamese: TextScript = TextScript(9i32);
    pub const Default: TextScript = TextScript(10i32);
    pub const Symbol: TextScript = TextScript(11i32);
    pub const Thai: TextScript = TextScript(12i32);
    pub const ShiftJis: TextScript = TextScript(13i32);
    pub const GB2312: TextScript = TextScript(14i32);
    pub const Hangul: TextScript = TextScript(15i32);
    pub const Big5: TextScript = TextScript(16i32);
    pub const PC437: TextScript = TextScript(17i32);
    pub const Oem: TextScript = TextScript(18i32);
    pub const Mac: TextScript = TextScript(19i32);
    pub const Armenian: TextScript = TextScript(20i32);
    pub const Syriac: TextScript = TextScript(21i32);
    pub const Thaana: TextScript = TextScript(22i32);
    pub const Devanagari: TextScript = TextScript(23i32);
    pub const Bengali: TextScript = TextScript(24i32);
    pub const Gurmukhi: TextScript = TextScript(25i32);
    pub const Gujarati: TextScript = TextScript(26i32);
    pub const Oriya: TextScript = TextScript(27i32);
    pub const Tamil: TextScript = TextScript(28i32);
    pub const Telugu: TextScript = TextScript(29i32);
    pub const Kannada: TextScript = TextScript(30i32);
    pub const Malayalam: TextScript = TextScript(31i32);
    pub const Sinhala: TextScript = TextScript(32i32);
    pub const Lao: TextScript = TextScript(33i32);
    pub const Tibetan: TextScript = TextScript(34i32);
    pub const Myanmar: TextScript = TextScript(35i32);
    pub const Georgian: TextScript = TextScript(36i32);
    pub const Jamo: TextScript = TextScript(37i32);
    pub const Ethiopic: TextScript = TextScript(38i32);
    pub const Cherokee: TextScript = TextScript(39i32);
    pub const Aboriginal: TextScript = TextScript(40i32);
    pub const Ogham: TextScript = TextScript(41i32);
    pub const Runic: TextScript = TextScript(42i32);
    pub const Khmer: TextScript = TextScript(43i32);
    pub const Mongolian: TextScript = TextScript(44i32);
    pub const Braille: TextScript = TextScript(45i32);
    pub const Yi: TextScript = TextScript(46i32);
    pub const Limbu: TextScript = TextScript(47i32);
    pub const TaiLe: TextScript = TextScript(48i32);
    pub const NewTaiLue: TextScript = TextScript(49i32);
    pub const SylotiNagri: TextScript = TextScript(50i32);
    pub const Kharoshthi: TextScript = TextScript(51i32);
    pub const Kayahli: TextScript = TextScript(52i32);
    pub const UnicodeSymbol: TextScript = TextScript(53i32);
    pub const Emoji: TextScript = TextScript(54i32);
    pub const Glagolitic: TextScript = TextScript(55i32);
    pub const Lisu: TextScript = TextScript(56i32);
    pub const Vai: TextScript = TextScript(57i32);
    pub const NKo: TextScript = TextScript(58i32);
    pub const Osmanya: TextScript = TextScript(59i32);
    pub const PhagsPa: TextScript = TextScript(60i32);
    pub const Gothic: TextScript = TextScript(61i32);
    pub const Deseret: TextScript = TextScript(62i32);
    pub const Tifinagh: TextScript = TextScript(63i32);
}
impl ::std::convert::From<i32> for TextScript {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TextScript {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TextScript {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextScript;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TextSetOptions(pub u32);
impl TextSetOptions {
    pub const None: TextSetOptions = TextSetOptions(0u32);
    pub const UnicodeBidi: TextSetOptions = TextSetOptions(1u32);
    pub const Unlink: TextSetOptions = TextSetOptions(8u32);
    pub const Unhide: TextSetOptions = TextSetOptions(16u32);
    pub const CheckTextLimit: TextSetOptions = TextSetOptions(32u32);
    pub const FormatRtf: TextSetOptions = TextSetOptions(8192u32);
    pub const ApplyRtfDocumentDefaults: TextSetOptions = TextSetOptions(16384u32);
}
impl ::std::convert::From<u32> for TextSetOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TextSetOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TextSetOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.TextSetOptions;u4)");
}
impl ::std::ops::BitOr for TextSetOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TextSetOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TextSetOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TextSetOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TextSetOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UnderlineType(pub i32);
impl UnderlineType {
    pub const Undefined: UnderlineType = UnderlineType(0i32);
    pub const None: UnderlineType = UnderlineType(1i32);
    pub const Single: UnderlineType = UnderlineType(2i32);
    pub const Words: UnderlineType = UnderlineType(3i32);
    pub const Double: UnderlineType = UnderlineType(4i32);
    pub const Dotted: UnderlineType = UnderlineType(5i32);
    pub const Dash: UnderlineType = UnderlineType(6i32);
    pub const DashDot: UnderlineType = UnderlineType(7i32);
    pub const DashDotDot: UnderlineType = UnderlineType(8i32);
    pub const Wave: UnderlineType = UnderlineType(9i32);
    pub const Thick: UnderlineType = UnderlineType(10i32);
    pub const Thin: UnderlineType = UnderlineType(11i32);
    pub const DoubleWave: UnderlineType = UnderlineType(12i32);
    pub const HeavyWave: UnderlineType = UnderlineType(13i32);
    pub const LongDash: UnderlineType = UnderlineType(14i32);
    pub const ThickDash: UnderlineType = UnderlineType(15i32);
    pub const ThickDashDot: UnderlineType = UnderlineType(16i32);
    pub const ThickDashDotDot: UnderlineType = UnderlineType(17i32);
    pub const ThickDotted: UnderlineType = UnderlineType(18i32);
    pub const ThickLongDash: UnderlineType = UnderlineType(19i32);
}
impl ::std::convert::From<i32> for UnderlineType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UnderlineType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UnderlineType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.UnderlineType;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VerticalCharacterAlignment(pub i32);
impl VerticalCharacterAlignment {
    pub const Top: VerticalCharacterAlignment = VerticalCharacterAlignment(0i32);
    pub const Baseline: VerticalCharacterAlignment = VerticalCharacterAlignment(1i32);
    pub const Bottom: VerticalCharacterAlignment = VerticalCharacterAlignment(2i32);
}
impl ::std::convert::From<i32> for VerticalCharacterAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VerticalCharacterAlignment {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for VerticalCharacterAlignment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.VerticalCharacterAlignment;i4)");
}
