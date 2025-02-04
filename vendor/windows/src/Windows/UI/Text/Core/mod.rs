#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreTextCompositionCompletedEventArgs(::windows::runtime::IInspectable);
impl CoreTextCompositionCompletedEventArgs {
    pub fn IsCanceled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CompositionSegments(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<CoreTextCompositionSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<CoreTextCompositionSegment>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextCompositionCompletedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs;{1f34ebb6-b79f-4121-a5e7-fda9b8616e30})");
}
unsafe impl ::windows::runtime::Interface for CoreTextCompositionCompletedEventArgs {
    type Vtable = ICoreTextCompositionCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(523561910, 47007, 16673, [165, 231, 253, 169, 184, 97, 110, 48]);
}
impl ::windows::runtime::RuntimeName for CoreTextCompositionCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs";
}
impl ::std::convert::From<CoreTextCompositionCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreTextCompositionCompletedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreTextCompositionCompletedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreTextCompositionCompletedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreTextCompositionCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreTextCompositionCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreTextCompositionCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CoreTextCompositionCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreTextCompositionCompletedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CoreTextCompositionCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreTextCompositionCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreTextCompositionCompletedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreTextCompositionCompletedEventArgs {}
unsafe impl ::std::marker::Sync for CoreTextCompositionCompletedEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreTextCompositionSegment(::windows::runtime::IInspectable);
impl CoreTextCompositionSegment {
    pub fn PreconversionString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Range(&self) -> ::windows::runtime::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextCompositionSegment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextCompositionSegment;{776c6bd9-4ead-4da7-8f47-3a88b523cc34})");
}
unsafe impl ::windows::runtime::Interface for CoreTextCompositionSegment {
    type Vtable = ICoreTextCompositionSegment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2003594201, 20141, 19879, [143, 71, 58, 136, 181, 35, 204, 52]);
}
impl ::windows::runtime::RuntimeName for CoreTextCompositionSegment {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextCompositionSegment";
}
impl ::std::convert::From<CoreTextCompositionSegment> for ::windows::runtime::IUnknown {
    fn from(value: CoreTextCompositionSegment) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreTextCompositionSegment> for ::windows::runtime::IUnknown {
    fn from(value: &CoreTextCompositionSegment) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreTextCompositionSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreTextCompositionSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreTextCompositionSegment> for ::windows::runtime::IInspectable {
    fn from(value: CoreTextCompositionSegment) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreTextCompositionSegment> for ::windows::runtime::IInspectable {
    fn from(value: &CoreTextCompositionSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreTextCompositionSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreTextCompositionSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreTextCompositionSegment {}
unsafe impl ::std::marker::Sync for CoreTextCompositionSegment {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreTextCompositionStartedEventArgs(::windows::runtime::IInspectable);
impl CoreTextCompositionStartedEventArgs {
    pub fn IsCanceled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextCompositionStartedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs;{276b16a9-64e7-4ab0-bc4b-a02d73835bfb})");
}
unsafe impl ::windows::runtime::Interface for CoreTextCompositionStartedEventArgs {
    type Vtable = ICoreTextCompositionStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(661329577, 25831, 19120, [188, 75, 160, 45, 115, 131, 91, 251]);
}
impl ::windows::runtime::RuntimeName for CoreTextCompositionStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs";
}
impl ::std::convert::From<CoreTextCompositionStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreTextCompositionStartedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreTextCompositionStartedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreTextCompositionStartedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreTextCompositionStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreTextCompositionStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreTextCompositionStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CoreTextCompositionStartedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreTextCompositionStartedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CoreTextCompositionStartedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreTextCompositionStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreTextCompositionStartedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreTextCompositionStartedEventArgs {}
unsafe impl ::std::marker::Sync for CoreTextCompositionStartedEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreTextEditContext(::windows::runtime::IInspectable);
impl CoreTextEditContext {
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn InputScope(&self) -> ::windows::runtime::Result<CoreTextInputScope> {
        let this = self;
        unsafe {
            let mut result__: CoreTextInputScope = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextInputScope>(result__)
        }
    }
    pub fn SetInputScope(&self, value: CoreTextInputScope) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsReadOnly(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsReadOnly(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn InputPaneDisplayPolicy(&self) -> ::windows::runtime::Result<CoreTextInputPaneDisplayPolicy> {
        let this = self;
        unsafe {
            let mut result__: CoreTextInputPaneDisplayPolicy = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextInputPaneDisplayPolicy>(result__)
        }
    }
    pub fn SetInputPaneDisplayPolicy(&self, value: CoreTextInputPaneDisplayPolicy) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn TextRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveTextRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SelectionRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSelectionRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn LayoutRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextLayoutRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveLayoutRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn TextUpdating<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextTextUpdatingEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveTextUpdating<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SelectionUpdating<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextSelectionUpdatingEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSelectionUpdating<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn FormatUpdating<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextFormatUpdatingEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveFormatUpdating<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CompositionStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionStartedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompositionStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CompositionCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, CoreTextCompositionCompletedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompositionCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn FocusRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveFocusRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn NotifyFocusEnter(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn NotifyFocusLeave(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn NotifyTextChanged<'a, Param0: ::windows::runtime::IntoParam<'a, CoreTextRange>, Param2: ::windows::runtime::IntoParam<'a, CoreTextRange>>(&self, modifiedrange: Param0, newlength: i32, newselection: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), modifiedrange.into_param().abi(), newlength, newselection.into_param().abi()).ok() }
    }
    pub fn NotifySelectionChanged<'a, Param0: ::windows::runtime::IntoParam<'a, CoreTextRange>>(&self, selection: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), selection.into_param().abi()).ok() }
    }
    pub fn NotifyLayoutChanged(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn NotifyFocusLeaveCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextEditContext, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<ICoreTextEditContext2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveNotifyFocusLeaveCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICoreTextEditContext2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextEditContext {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextEditContext;{bf6608af-4041-47c3-b263-a918eb5eaef2})");
}
unsafe impl ::windows::runtime::Interface for CoreTextEditContext {
    type Vtable = ICoreTextEditContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3211135151, 16449, 18371, [178, 99, 169, 24, 235, 94, 174, 242]);
}
impl ::windows::runtime::RuntimeName for CoreTextEditContext {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextEditContext";
}
impl ::std::convert::From<CoreTextEditContext> for ::windows::runtime::IUnknown {
    fn from(value: CoreTextEditContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreTextEditContext> for ::windows::runtime::IUnknown {
    fn from(value: &CoreTextEditContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreTextEditContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreTextEditContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreTextEditContext> for ::windows::runtime::IInspectable {
    fn from(value: CoreTextEditContext) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreTextEditContext> for ::windows::runtime::IInspectable {
    fn from(value: &CoreTextEditContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreTextEditContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreTextEditContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreTextEditContext {}
unsafe impl ::std::marker::Sync for CoreTextEditContext {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreTextFormatUpdatingEventArgs(::windows::runtime::IInspectable);
impl CoreTextFormatUpdatingEventArgs {
    pub fn Range(&self) -> ::windows::runtime::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub fn TextColor(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub fn BackgroundColor(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))]
    pub fn UnderlineColor(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::ViewManagement::UIElementType>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UnderlineType(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::UnderlineType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::UnderlineType>>(result__)
        }
    }
    pub fn Reason(&self) -> ::windows::runtime::Result<CoreTextFormatUpdatingReason> {
        let this = self;
        unsafe {
            let mut result__: CoreTextFormatUpdatingReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextFormatUpdatingReason>(result__)
        }
    }
    pub fn Result(&self) -> ::windows::runtime::Result<CoreTextFormatUpdatingResult> {
        let this = self;
        unsafe {
            let mut result__: CoreTextFormatUpdatingResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextFormatUpdatingResult>(result__)
        }
    }
    pub fn SetResult(&self, value: CoreTextFormatUpdatingResult) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsCanceled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextFormatUpdatingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs;{7310bd33-b4a8-43b1-b37b-0724d4aca7ab})");
}
unsafe impl ::windows::runtime::Interface for CoreTextFormatUpdatingEventArgs {
    type Vtable = ICoreTextFormatUpdatingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1930476851, 46248, 17329, [179, 123, 7, 36, 212, 172, 167, 171]);
}
impl ::windows::runtime::RuntimeName for CoreTextFormatUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs";
}
impl ::std::convert::From<CoreTextFormatUpdatingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreTextFormatUpdatingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreTextFormatUpdatingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreTextFormatUpdatingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreTextFormatUpdatingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreTextFormatUpdatingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreTextFormatUpdatingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CoreTextFormatUpdatingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreTextFormatUpdatingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CoreTextFormatUpdatingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreTextFormatUpdatingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreTextFormatUpdatingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreTextFormatUpdatingEventArgs {}
unsafe impl ::std::marker::Sync for CoreTextFormatUpdatingEventArgs {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreTextFormatUpdatingReason(pub i32);
impl CoreTextFormatUpdatingReason {
    pub const None: CoreTextFormatUpdatingReason = CoreTextFormatUpdatingReason(0i32);
    pub const CompositionUnconverted: CoreTextFormatUpdatingReason = CoreTextFormatUpdatingReason(1i32);
    pub const CompositionConverted: CoreTextFormatUpdatingReason = CoreTextFormatUpdatingReason(2i32);
    pub const CompositionTargetUnconverted: CoreTextFormatUpdatingReason = CoreTextFormatUpdatingReason(3i32);
    pub const CompositionTargetConverted: CoreTextFormatUpdatingReason = CoreTextFormatUpdatingReason(4i32);
}
impl ::std::convert::From<i32> for CoreTextFormatUpdatingReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreTextFormatUpdatingReason {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextFormatUpdatingReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextFormatUpdatingReason;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreTextFormatUpdatingResult(pub i32);
impl CoreTextFormatUpdatingResult {
    pub const Succeeded: CoreTextFormatUpdatingResult = CoreTextFormatUpdatingResult(0i32);
    pub const Failed: CoreTextFormatUpdatingResult = CoreTextFormatUpdatingResult(1i32);
}
impl ::std::convert::From<i32> for CoreTextFormatUpdatingResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreTextFormatUpdatingResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextFormatUpdatingResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextFormatUpdatingResult;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreTextInputPaneDisplayPolicy(pub i32);
impl CoreTextInputPaneDisplayPolicy {
    pub const Automatic: CoreTextInputPaneDisplayPolicy = CoreTextInputPaneDisplayPolicy(0i32);
    pub const Manual: CoreTextInputPaneDisplayPolicy = CoreTextInputPaneDisplayPolicy(1i32);
}
impl ::std::convert::From<i32> for CoreTextInputPaneDisplayPolicy {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreTextInputPaneDisplayPolicy {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextInputPaneDisplayPolicy {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextInputPaneDisplayPolicy;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreTextInputScope(pub i32);
impl CoreTextInputScope {
    pub const Default: CoreTextInputScope = CoreTextInputScope(0i32);
    pub const Url: CoreTextInputScope = CoreTextInputScope(1i32);
    pub const FilePath: CoreTextInputScope = CoreTextInputScope(2i32);
    pub const FileName: CoreTextInputScope = CoreTextInputScope(3i32);
    pub const EmailUserName: CoreTextInputScope = CoreTextInputScope(4i32);
    pub const EmailAddress: CoreTextInputScope = CoreTextInputScope(5i32);
    pub const UserName: CoreTextInputScope = CoreTextInputScope(6i32);
    pub const PersonalFullName: CoreTextInputScope = CoreTextInputScope(7i32);
    pub const PersonalNamePrefix: CoreTextInputScope = CoreTextInputScope(8i32);
    pub const PersonalGivenName: CoreTextInputScope = CoreTextInputScope(9i32);
    pub const PersonalMiddleName: CoreTextInputScope = CoreTextInputScope(10i32);
    pub const PersonalSurname: CoreTextInputScope = CoreTextInputScope(11i32);
    pub const PersonalNameSuffix: CoreTextInputScope = CoreTextInputScope(12i32);
    pub const Address: CoreTextInputScope = CoreTextInputScope(13i32);
    pub const AddressPostalCode: CoreTextInputScope = CoreTextInputScope(14i32);
    pub const AddressStreet: CoreTextInputScope = CoreTextInputScope(15i32);
    pub const AddressStateOrProvince: CoreTextInputScope = CoreTextInputScope(16i32);
    pub const AddressCity: CoreTextInputScope = CoreTextInputScope(17i32);
    pub const AddressCountryName: CoreTextInputScope = CoreTextInputScope(18i32);
    pub const AddressCountryShortName: CoreTextInputScope = CoreTextInputScope(19i32);
    pub const CurrencyAmountAndSymbol: CoreTextInputScope = CoreTextInputScope(20i32);
    pub const CurrencyAmount: CoreTextInputScope = CoreTextInputScope(21i32);
    pub const Date: CoreTextInputScope = CoreTextInputScope(22i32);
    pub const DateMonth: CoreTextInputScope = CoreTextInputScope(23i32);
    pub const DateDay: CoreTextInputScope = CoreTextInputScope(24i32);
    pub const DateYear: CoreTextInputScope = CoreTextInputScope(25i32);
    pub const DateMonthName: CoreTextInputScope = CoreTextInputScope(26i32);
    pub const DateDayName: CoreTextInputScope = CoreTextInputScope(27i32);
    pub const Number: CoreTextInputScope = CoreTextInputScope(29i32);
    pub const SingleCharacter: CoreTextInputScope = CoreTextInputScope(30i32);
    pub const Password: CoreTextInputScope = CoreTextInputScope(31i32);
    pub const TelephoneNumber: CoreTextInputScope = CoreTextInputScope(32i32);
    pub const TelephoneCountryCode: CoreTextInputScope = CoreTextInputScope(33i32);
    pub const TelephoneAreaCode: CoreTextInputScope = CoreTextInputScope(34i32);
    pub const TelephoneLocalNumber: CoreTextInputScope = CoreTextInputScope(35i32);
    pub const Time: CoreTextInputScope = CoreTextInputScope(36i32);
    pub const TimeHour: CoreTextInputScope = CoreTextInputScope(37i32);
    pub const TimeMinuteOrSecond: CoreTextInputScope = CoreTextInputScope(38i32);
    pub const NumberFullWidth: CoreTextInputScope = CoreTextInputScope(39i32);
    pub const AlphanumericHalfWidth: CoreTextInputScope = CoreTextInputScope(40i32);
    pub const AlphanumericFullWidth: CoreTextInputScope = CoreTextInputScope(41i32);
    pub const CurrencyChinese: CoreTextInputScope = CoreTextInputScope(42i32);
    pub const Bopomofo: CoreTextInputScope = CoreTextInputScope(43i32);
    pub const Hiragana: CoreTextInputScope = CoreTextInputScope(44i32);
    pub const KatakanaHalfWidth: CoreTextInputScope = CoreTextInputScope(45i32);
    pub const KatakanaFullWidth: CoreTextInputScope = CoreTextInputScope(46i32);
    pub const Hanja: CoreTextInputScope = CoreTextInputScope(47i32);
    pub const HangulHalfWidth: CoreTextInputScope = CoreTextInputScope(48i32);
    pub const HangulFullWidth: CoreTextInputScope = CoreTextInputScope(49i32);
    pub const Search: CoreTextInputScope = CoreTextInputScope(50i32);
    pub const Formula: CoreTextInputScope = CoreTextInputScope(51i32);
    pub const SearchIncremental: CoreTextInputScope = CoreTextInputScope(52i32);
    pub const ChineseHalfWidth: CoreTextInputScope = CoreTextInputScope(53i32);
    pub const ChineseFullWidth: CoreTextInputScope = CoreTextInputScope(54i32);
    pub const NativeScript: CoreTextInputScope = CoreTextInputScope(55i32);
    pub const Text: CoreTextInputScope = CoreTextInputScope(57i32);
    pub const Chat: CoreTextInputScope = CoreTextInputScope(58i32);
    pub const NameOrPhoneNumber: CoreTextInputScope = CoreTextInputScope(59i32);
    pub const EmailUserNameOrAddress: CoreTextInputScope = CoreTextInputScope(60i32);
    pub const Private: CoreTextInputScope = CoreTextInputScope(61i32);
    pub const Maps: CoreTextInputScope = CoreTextInputScope(62i32);
    pub const PasswordNumeric: CoreTextInputScope = CoreTextInputScope(63i32);
    pub const FormulaNumber: CoreTextInputScope = CoreTextInputScope(67i32);
    pub const ChatWithoutEmoji: CoreTextInputScope = CoreTextInputScope(68i32);
    pub const Digits: CoreTextInputScope = CoreTextInputScope(28i32);
    pub const PinNumeric: CoreTextInputScope = CoreTextInputScope(64i32);
    pub const PinAlphanumeric: CoreTextInputScope = CoreTextInputScope(65i32);
}
impl ::std::convert::From<i32> for CoreTextInputScope {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreTextInputScope {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextInputScope {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextInputScope;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreTextLayoutBounds(::windows::runtime::IInspectable);
impl CoreTextLayoutBounds {
    #[cfg(feature = "Foundation")]
    pub fn TextBounds(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetTextBounds<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn ControlBounds(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetControlBounds<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextLayoutBounds {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextLayoutBounds;{e972c974-4436-4917-80d0-a525e4ca6780})");
}
unsafe impl ::windows::runtime::Interface for CoreTextLayoutBounds {
    type Vtable = ICoreTextLayoutBounds_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3916614004, 17462, 18711, [128, 208, 165, 37, 228, 202, 103, 128]);
}
impl ::windows::runtime::RuntimeName for CoreTextLayoutBounds {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextLayoutBounds";
}
impl ::std::convert::From<CoreTextLayoutBounds> for ::windows::runtime::IUnknown {
    fn from(value: CoreTextLayoutBounds) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreTextLayoutBounds> for ::windows::runtime::IUnknown {
    fn from(value: &CoreTextLayoutBounds) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreTextLayoutBounds {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreTextLayoutBounds {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreTextLayoutBounds> for ::windows::runtime::IInspectable {
    fn from(value: CoreTextLayoutBounds) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreTextLayoutBounds> for ::windows::runtime::IInspectable {
    fn from(value: &CoreTextLayoutBounds) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreTextLayoutBounds {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreTextLayoutBounds {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreTextLayoutBounds {}
unsafe impl ::std::marker::Sync for CoreTextLayoutBounds {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreTextLayoutRequest(::windows::runtime::IInspectable);
impl CoreTextLayoutRequest {
    pub fn Range(&self) -> ::windows::runtime::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    pub fn LayoutBounds(&self) -> ::windows::runtime::Result<CoreTextLayoutBounds> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextLayoutBounds>(result__)
        }
    }
    pub fn IsCanceled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
    pub fn LayoutBoundsVisualPixels(&self) -> ::windows::runtime::Result<CoreTextLayoutBounds> {
        let this = &::windows::runtime::Interface::cast::<ICoreTextLayoutRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextLayoutBounds>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextLayoutRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextLayoutRequest;{2555a8cc-51fd-4f03-98bf-ac78174d68e0})");
}
unsafe impl ::windows::runtime::Interface for CoreTextLayoutRequest {
    type Vtable = ICoreTextLayoutRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(626370764, 20989, 20227, [152, 191, 172, 120, 23, 77, 104, 224]);
}
impl ::windows::runtime::RuntimeName for CoreTextLayoutRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextLayoutRequest";
}
impl ::std::convert::From<CoreTextLayoutRequest> for ::windows::runtime::IUnknown {
    fn from(value: CoreTextLayoutRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreTextLayoutRequest> for ::windows::runtime::IUnknown {
    fn from(value: &CoreTextLayoutRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreTextLayoutRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreTextLayoutRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreTextLayoutRequest> for ::windows::runtime::IInspectable {
    fn from(value: CoreTextLayoutRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreTextLayoutRequest> for ::windows::runtime::IInspectable {
    fn from(value: &CoreTextLayoutRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreTextLayoutRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreTextLayoutRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreTextLayoutRequest {}
unsafe impl ::std::marker::Sync for CoreTextLayoutRequest {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreTextLayoutRequestedEventArgs(::windows::runtime::IInspectable);
impl CoreTextLayoutRequestedEventArgs {
    pub fn Request(&self) -> ::windows::runtime::Result<CoreTextLayoutRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextLayoutRequest>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextLayoutRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs;{b1dc6ae0-9a7b-4e9e-a566-4a6b5f8ad676})");
}
unsafe impl ::windows::runtime::Interface for CoreTextLayoutRequestedEventArgs {
    type Vtable = ICoreTextLayoutRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2984012512, 39547, 20126, [165, 102, 74, 107, 95, 138, 214, 118]);
}
impl ::windows::runtime::RuntimeName for CoreTextLayoutRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs";
}
impl ::std::convert::From<CoreTextLayoutRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreTextLayoutRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreTextLayoutRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreTextLayoutRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreTextLayoutRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreTextLayoutRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreTextLayoutRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CoreTextLayoutRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreTextLayoutRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CoreTextLayoutRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreTextLayoutRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreTextLayoutRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreTextLayoutRequestedEventArgs {}
unsafe impl ::std::marker::Sync for CoreTextLayoutRequestedEventArgs {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct CoreTextRange {
    pub StartCaretPosition: i32,
    pub EndCaretPosition: i32,
}
impl CoreTextRange {}
impl ::std::default::Default for CoreTextRange {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CoreTextRange {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CoreTextRange").field("StartCaretPosition", &self.StartCaretPosition).field("EndCaretPosition", &self.EndCaretPosition).finish()
    }
}
impl ::std::cmp::PartialEq for CoreTextRange {
    fn eq(&self, other: &Self) -> bool {
        self.StartCaretPosition == other.StartCaretPosition && self.EndCaretPosition == other.EndCaretPosition
    }
}
impl ::std::cmp::Eq for CoreTextRange {}
unsafe impl ::windows::runtime::Abi for CoreTextRange {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextRange {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.UI.Text.Core.CoreTextRange;i4;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreTextSelectionRequest(::windows::runtime::IInspectable);
impl CoreTextSelectionRequest {
    pub fn Selection(&self) -> ::windows::runtime::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    pub fn SetSelection<'a, Param0: ::windows::runtime::IntoParam<'a, CoreTextRange>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsCanceled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextSelectionRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextSelectionRequest;{f0a70403-208b-4301-883c-74ca7485fd8d})");
}
unsafe impl ::windows::runtime::Interface for CoreTextSelectionRequest {
    type Vtable = ICoreTextSelectionRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4037477379, 8331, 17153, [136, 60, 116, 202, 116, 133, 253, 141]);
}
impl ::windows::runtime::RuntimeName for CoreTextSelectionRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextSelectionRequest";
}
impl ::std::convert::From<CoreTextSelectionRequest> for ::windows::runtime::IUnknown {
    fn from(value: CoreTextSelectionRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreTextSelectionRequest> for ::windows::runtime::IUnknown {
    fn from(value: &CoreTextSelectionRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreTextSelectionRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreTextSelectionRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreTextSelectionRequest> for ::windows::runtime::IInspectable {
    fn from(value: CoreTextSelectionRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreTextSelectionRequest> for ::windows::runtime::IInspectable {
    fn from(value: &CoreTextSelectionRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreTextSelectionRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreTextSelectionRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreTextSelectionRequest {}
unsafe impl ::std::marker::Sync for CoreTextSelectionRequest {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreTextSelectionRequestedEventArgs(::windows::runtime::IInspectable);
impl CoreTextSelectionRequestedEventArgs {
    pub fn Request(&self) -> ::windows::runtime::Result<CoreTextSelectionRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextSelectionRequest>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextSelectionRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs;{13c6682b-f614-421a-8f4b-9ec8a5a37fcd})");
}
unsafe impl ::windows::runtime::Interface for CoreTextSelectionRequestedEventArgs {
    type Vtable = ICoreTextSelectionRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(331769899, 62996, 16922, [143, 75, 158, 200, 165, 163, 127, 205]);
}
impl ::windows::runtime::RuntimeName for CoreTextSelectionRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs";
}
impl ::std::convert::From<CoreTextSelectionRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreTextSelectionRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreTextSelectionRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreTextSelectionRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreTextSelectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreTextSelectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreTextSelectionRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CoreTextSelectionRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreTextSelectionRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CoreTextSelectionRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreTextSelectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreTextSelectionRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreTextSelectionRequestedEventArgs {}
unsafe impl ::std::marker::Sync for CoreTextSelectionRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreTextSelectionUpdatingEventArgs(::windows::runtime::IInspectable);
impl CoreTextSelectionUpdatingEventArgs {
    pub fn Selection(&self) -> ::windows::runtime::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    pub fn Result(&self) -> ::windows::runtime::Result<CoreTextSelectionUpdatingResult> {
        let this = self;
        unsafe {
            let mut result__: CoreTextSelectionUpdatingResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextSelectionUpdatingResult>(result__)
        }
    }
    pub fn SetResult(&self, value: CoreTextSelectionUpdatingResult) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsCanceled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextSelectionUpdatingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs;{d445839f-fe7f-4bd5-8a26-0922c1b3e639})");
}
unsafe impl ::windows::runtime::Interface for CoreTextSelectionUpdatingEventArgs {
    type Vtable = ICoreTextSelectionUpdatingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3561325471, 65151, 19413, [138, 38, 9, 34, 193, 179, 230, 57]);
}
impl ::windows::runtime::RuntimeName for CoreTextSelectionUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs";
}
impl ::std::convert::From<CoreTextSelectionUpdatingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreTextSelectionUpdatingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreTextSelectionUpdatingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreTextSelectionUpdatingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreTextSelectionUpdatingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreTextSelectionUpdatingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreTextSelectionUpdatingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CoreTextSelectionUpdatingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreTextSelectionUpdatingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CoreTextSelectionUpdatingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreTextSelectionUpdatingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreTextSelectionUpdatingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreTextSelectionUpdatingEventArgs {}
unsafe impl ::std::marker::Sync for CoreTextSelectionUpdatingEventArgs {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreTextSelectionUpdatingResult(pub i32);
impl CoreTextSelectionUpdatingResult {
    pub const Succeeded: CoreTextSelectionUpdatingResult = CoreTextSelectionUpdatingResult(0i32);
    pub const Failed: CoreTextSelectionUpdatingResult = CoreTextSelectionUpdatingResult(1i32);
}
impl ::std::convert::From<i32> for CoreTextSelectionUpdatingResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreTextSelectionUpdatingResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextSelectionUpdatingResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextSelectionUpdatingResult;i4)");
}
pub struct CoreTextServicesConstants {}
impl CoreTextServicesConstants {
    pub fn HiddenCharacter() -> ::windows::runtime::Result<u16> {
        Self::ICoreTextServicesStatics(|this| unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        })
    }
    pub fn ICoreTextServicesStatics<R, F: FnOnce(&ICoreTextServicesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreTextServicesConstants, ICoreTextServicesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for CoreTextServicesConstants {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextServicesConstants";
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreTextServicesManager(::windows::runtime::IInspectable);
impl CoreTextServicesManager {
    #[cfg(feature = "Globalization")]
    pub fn InputLanguage(&self) -> ::windows::runtime::Result<super::super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Globalization::Language>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn InputLanguageChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<CoreTextServicesManager, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveInputLanguageChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn CreateEditContext(&self) -> ::windows::runtime::Result<CoreTextEditContext> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextEditContext>(result__)
        }
    }
    pub fn GetForCurrentView() -> ::windows::runtime::Result<CoreTextServicesManager> {
        Self::ICoreTextServicesManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextServicesManager>(result__)
        })
    }
    pub fn ICoreTextServicesManagerStatics<R, F: FnOnce(&ICoreTextServicesManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CoreTextServicesManager, ICoreTextServicesManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextServicesManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextServicesManager;{c2507d83-6e0a-4a8a-bdf8-1948874854ba})");
}
unsafe impl ::windows::runtime::Interface for CoreTextServicesManager {
    type Vtable = ICoreTextServicesManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3260054915, 28170, 19082, [189, 248, 25, 72, 135, 72, 84, 186]);
}
impl ::windows::runtime::RuntimeName for CoreTextServicesManager {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextServicesManager";
}
impl ::std::convert::From<CoreTextServicesManager> for ::windows::runtime::IUnknown {
    fn from(value: CoreTextServicesManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreTextServicesManager> for ::windows::runtime::IUnknown {
    fn from(value: &CoreTextServicesManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreTextServicesManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreTextServicesManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreTextServicesManager> for ::windows::runtime::IInspectable {
    fn from(value: CoreTextServicesManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreTextServicesManager> for ::windows::runtime::IInspectable {
    fn from(value: &CoreTextServicesManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreTextServicesManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreTextServicesManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreTextServicesManager {}
unsafe impl ::std::marker::Sync for CoreTextServicesManager {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreTextTextRequest(::windows::runtime::IInspectable);
impl CoreTextTextRequest {
    pub fn Range(&self) -> ::windows::runtime::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IsCanceled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextTextRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextTextRequest;{50d950a9-f51e-4cc1-8ca1-e6346d1a61be})");
}
unsafe impl ::windows::runtime::Interface for CoreTextTextRequest {
    type Vtable = ICoreTextTextRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1356419241, 62750, 19649, [140, 161, 230, 52, 109, 26, 97, 190]);
}
impl ::windows::runtime::RuntimeName for CoreTextTextRequest {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextTextRequest";
}
impl ::std::convert::From<CoreTextTextRequest> for ::windows::runtime::IUnknown {
    fn from(value: CoreTextTextRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreTextTextRequest> for ::windows::runtime::IUnknown {
    fn from(value: &CoreTextTextRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreTextTextRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreTextTextRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreTextTextRequest> for ::windows::runtime::IInspectable {
    fn from(value: CoreTextTextRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreTextTextRequest> for ::windows::runtime::IInspectable {
    fn from(value: &CoreTextTextRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreTextTextRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreTextTextRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreTextTextRequest {}
unsafe impl ::std::marker::Sync for CoreTextTextRequest {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreTextTextRequestedEventArgs(::windows::runtime::IInspectable);
impl CoreTextTextRequestedEventArgs {
    pub fn Request(&self) -> ::windows::runtime::Result<CoreTextTextRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextTextRequest>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextTextRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextTextRequestedEventArgs;{f096a2d0-41c6-4c02-8b1a-d953b00cabb3})");
}
unsafe impl ::windows::runtime::Interface for CoreTextTextRequestedEventArgs {
    type Vtable = ICoreTextTextRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4036403920, 16838, 19458, [139, 26, 217, 83, 176, 12, 171, 179]);
}
impl ::windows::runtime::RuntimeName for CoreTextTextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextTextRequestedEventArgs";
}
impl ::std::convert::From<CoreTextTextRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreTextTextRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreTextTextRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreTextTextRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreTextTextRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreTextTextRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreTextTextRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CoreTextTextRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreTextTextRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CoreTextTextRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreTextTextRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreTextTextRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreTextTextRequestedEventArgs {}
unsafe impl ::std::marker::Sync for CoreTextTextRequestedEventArgs {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CoreTextTextUpdatingEventArgs(::windows::runtime::IInspectable);
impl CoreTextTextUpdatingEventArgs {
    pub fn Range(&self) -> ::windows::runtime::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    pub fn Text(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn NewSelection(&self) -> ::windows::runtime::Result<CoreTextRange> {
        let this = self;
        unsafe {
            let mut result__: CoreTextRange = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextRange>(result__)
        }
    }
    #[cfg(feature = "Globalization")]
    pub fn InputLanguage(&self) -> ::windows::runtime::Result<super::super::super::Globalization::Language> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Globalization::Language>(result__)
        }
    }
    pub fn Result(&self) -> ::windows::runtime::Result<CoreTextTextUpdatingResult> {
        let this = self;
        unsafe {
            let mut result__: CoreTextTextUpdatingResult = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CoreTextTextUpdatingResult>(result__)
        }
    }
    pub fn SetResult(&self, value: CoreTextTextUpdatingResult) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsCanceled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextTextUpdatingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs;{eea7918d-cc2b-4f03-8ff6-02fd217db450})");
}
unsafe impl ::windows::runtime::Interface for CoreTextTextUpdatingEventArgs {
    type Vtable = ICoreTextTextUpdatingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4003959181, 52267, 20227, [143, 246, 2, 253, 33, 125, 180, 80]);
}
impl ::windows::runtime::RuntimeName for CoreTextTextUpdatingEventArgs {
    const NAME: &'static str = "Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs";
}
impl ::std::convert::From<CoreTextTextUpdatingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CoreTextTextUpdatingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CoreTextTextUpdatingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CoreTextTextUpdatingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CoreTextTextUpdatingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CoreTextTextUpdatingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CoreTextTextUpdatingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CoreTextTextUpdatingEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CoreTextTextUpdatingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CoreTextTextUpdatingEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CoreTextTextUpdatingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CoreTextTextUpdatingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CoreTextTextUpdatingEventArgs {}
unsafe impl ::std::marker::Sync for CoreTextTextUpdatingEventArgs {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CoreTextTextUpdatingResult(pub i32);
impl CoreTextTextUpdatingResult {
    pub const Succeeded: CoreTextTextUpdatingResult = CoreTextTextUpdatingResult(0i32);
    pub const Failed: CoreTextTextUpdatingResult = CoreTextTextUpdatingResult(1i32);
}
impl ::std::convert::From<i32> for CoreTextTextUpdatingResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CoreTextTextUpdatingResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CoreTextTextUpdatingResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Text.Core.CoreTextTextUpdatingResult;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICoreTextCompositionCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextCompositionCompletedEventArgs {
    type Vtable = ICoreTextCompositionCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(523561910, 47007, 16673, [165, 231, 253, 169, 184, 97, 110, 48]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextCompositionCompletedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICoreTextCompositionSegment(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextCompositionSegment {
    type Vtable = ICoreTextCompositionSegment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2003594201, 20141, 19879, [143, 71, 58, 136, 181, 35, 204, 52]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextCompositionSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreTextRange) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICoreTextCompositionStartedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextCompositionStartedEventArgs {
    type Vtable = ICoreTextCompositionStartedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(661329577, 25831, 19120, [188, 75, 160, 45, 115, 131, 91, 251]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextCompositionStartedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICoreTextEditContext(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextEditContext {
    type Vtable = ICoreTextEditContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3211135151, 16449, 18371, [178, 99, 169, 24, 235, 94, 174, 242]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextEditContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreTextInputScope) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CoreTextInputScope) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreTextInputPaneDisplayPolicy) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CoreTextInputPaneDisplayPolicy) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modifiedrange: CoreTextRange, newlength: i32, newselection: CoreTextRange) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: CoreTextRange) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICoreTextEditContext2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextEditContext2 {
    type Vtable = ICoreTextEditContext2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2978381243, 2107, 18913, [178, 129, 43, 53, 214, 43, 244, 102]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextEditContext2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICoreTextFormatUpdatingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextFormatUpdatingEventArgs {
    type Vtable = ICoreTextFormatUpdatingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1930476851, 46248, 17329, [179, 123, 7, 36, 212, 172, 167, 171]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextFormatUpdatingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreTextRange) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_ViewManagement")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_ViewManagement")))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_ViewManagement"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_ViewManagement")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreTextFormatUpdatingReason) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreTextFormatUpdatingResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CoreTextFormatUpdatingResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICoreTextLayoutBounds(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextLayoutBounds {
    type Vtable = ICoreTextLayoutBounds_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3916614004, 17462, 18711, [128, 208, 165, 37, 228, 202, 103, 128]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutBounds_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextLayoutRequest {
    type Vtable = ICoreTextLayoutRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(626370764, 20989, 20227, [152, 191, 172, 120, 23, 77, 104, 224]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreTextRange) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequest2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextLayoutRequest2 {
    type Vtable = ICoreTextLayoutRequest2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1735255588, 52541, 19405, [191, 1, 127, 113, 16, 149, 69, 17]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequest2_abi(
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
pub struct ICoreTextLayoutRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextLayoutRequestedEventArgs {
    type Vtable = ICoreTextLayoutRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2984012512, 39547, 20126, [165, 102, 74, 107, 95, 138, 214, 118]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextLayoutRequestedEventArgs_abi(
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
pub struct ICoreTextSelectionRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextSelectionRequest {
    type Vtable = ICoreTextSelectionRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4037477379, 8331, 17153, [136, 60, 116, 202, 116, 133, 253, 141]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextSelectionRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreTextRange) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CoreTextRange) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICoreTextSelectionRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextSelectionRequestedEventArgs {
    type Vtable = ICoreTextSelectionRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(331769899, 62996, 16922, [143, 75, 158, 200, 165, 163, 127, 205]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextSelectionRequestedEventArgs_abi(
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
pub struct ICoreTextSelectionUpdatingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextSelectionUpdatingEventArgs {
    type Vtable = ICoreTextSelectionUpdatingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3561325471, 65151, 19413, [138, 38, 9, 34, 193, 179, 230, 57]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextSelectionUpdatingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreTextRange) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreTextSelectionUpdatingResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CoreTextSelectionUpdatingResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICoreTextServicesManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextServicesManager {
    type Vtable = ICoreTextServicesManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3260054915, 28170, 19082, [189, 248, 25, 72, 135, 72, 84, 186]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextServicesManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICoreTextServicesManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextServicesManagerStatics {
    type Vtable = ICoreTextServicesManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(354460552, 58063, 19813, [174, 185, 179, 45, 134, 254, 57, 185]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextServicesManagerStatics_abi(
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
pub struct ICoreTextServicesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextServicesStatics {
    type Vtable = ICoreTextServicesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2441452102, 60623, 18340, [138, 231, 9, 138, 156, 111, 187, 21]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextServicesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICoreTextTextRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextTextRequest {
    type Vtable = ICoreTextTextRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1356419241, 62750, 19649, [140, 161, 230, 52, 109, 26, 97, 190]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextTextRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreTextRange) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct ICoreTextTextRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextTextRequestedEventArgs {
    type Vtable = ICoreTextTextRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4036403920, 16838, 19458, [139, 26, 217, 83, 176, 12, 171, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextTextRequestedEventArgs_abi(
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
pub struct ICoreTextTextUpdatingEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICoreTextTextUpdatingEventArgs {
    type Vtable = ICoreTextTextUpdatingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4003959181, 52267, 20227, [143, 246, 2, 253, 33, 125, 180, 80]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreTextTextUpdatingEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreTextRange) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreTextRange) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Globalization")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Globalization"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CoreTextTextUpdatingResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CoreTextTextUpdatingResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
