#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPaymentAppCanMakePaymentTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentAppCanMakePaymentTriggerDetails {
    type Vtable = IPaymentAppCanMakePaymentTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(216138224, 35731, 20150, [140, 70, 46, 74, 108, 106, 38, 246]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAppCanMakePaymentTriggerDetails_abi(
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
#[doc(hidden)]
pub struct IPaymentAppManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentAppManager {
    type Vtable = IPaymentAppManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(239577683, 34081, 18793, [169, 87, 223, 37, 56, 163, 169, 143]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAppManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedpaymentmethodids: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPaymentAppManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentAppManagerStatics {
    type Vtable = IPaymentAppManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2738990120, 64649, 17414, [180, 217, 52, 231, 254, 121, 223, 182]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentAppManagerStatics_abi(
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
pub struct IPaymentTransaction(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentTransaction {
    type Vtable = IPaymentTransaction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1649941920, 9893, 20123, [166, 235, 102, 96, 108, 240, 1, 211]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTransaction_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, shippingaddress: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selectedshippingoption: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paymenttoken: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPaymentTransactionAcceptResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentTransactionAcceptResult {
    type Vtable = IPaymentTransactionAcceptResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(101593718, 54028, 18455, [149, 162, 223, 122, 233, 39, 59, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTransactionAcceptResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::PaymentRequestCompletionStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPaymentTransactionStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPaymentTransactionStatics {
    type Vtable = IPaymentTransactionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2372114256, 60938, 19957, [155, 30, 28, 15, 158, 197, 152, 129]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaymentTransactionStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentAppCanMakePaymentTriggerDetails(::windows::runtime::IInspectable);
impl PaymentAppCanMakePaymentTriggerDetails {
    pub fn Request(&self) -> ::windows::runtime::Result<super::PaymentRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::PaymentRequest>(result__)
        }
    }
    pub fn ReportCanMakePaymentResult<'a, Param0: ::windows::runtime::IntoParam<'a, super::PaymentCanMakePaymentResult>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentAppCanMakePaymentTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails;{0ce201f0-8b93-4eb6-8c46-2e4a6c6a26f6})");
}
unsafe impl ::windows::runtime::Interface for PaymentAppCanMakePaymentTriggerDetails {
    type Vtable = IPaymentAppCanMakePaymentTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(216138224, 35731, 20150, [140, 70, 46, 74, 108, 106, 38, 246]);
}
impl ::windows::runtime::RuntimeName for PaymentAppCanMakePaymentTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentAppCanMakePaymentTriggerDetails";
}
impl ::std::convert::From<PaymentAppCanMakePaymentTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: PaymentAppCanMakePaymentTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PaymentAppCanMakePaymentTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentAppCanMakePaymentTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PaymentAppCanMakePaymentTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: PaymentAppCanMakePaymentTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentAppCanMakePaymentTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentAppCanMakePaymentTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentAppCanMakePaymentTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentAppCanMakePaymentTriggerDetails {}
unsafe impl ::std::marker::Sync for PaymentAppCanMakePaymentTriggerDetails {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentAppManager(::windows::runtime::IInspectable);
impl PaymentAppManager {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RegisterAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, supportedpaymentmethodids: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), supportedpaymentmethodids.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UnregisterAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn Current() -> ::windows::runtime::Result<PaymentAppManager> {
        Self::IPaymentAppManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PaymentAppManager>(result__)
        })
    }
    pub fn IPaymentAppManagerStatics<R, F: FnOnce(&IPaymentAppManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentAppManager, IPaymentAppManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentAppManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentAppManager;{0e47aa53-8521-4969-a957-df2538a3a98f})");
}
unsafe impl ::windows::runtime::Interface for PaymentAppManager {
    type Vtable = IPaymentAppManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(239577683, 34081, 18793, [169, 87, 223, 37, 56, 163, 169, 143]);
}
impl ::windows::runtime::RuntimeName for PaymentAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentAppManager";
}
impl ::std::convert::From<PaymentAppManager> for ::windows::runtime::IUnknown {
    fn from(value: PaymentAppManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PaymentAppManager> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentAppManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentAppManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PaymentAppManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PaymentAppManager> for ::windows::runtime::IInspectable {
    fn from(value: PaymentAppManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentAppManager> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentAppManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentAppManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentAppManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentAppManager {}
unsafe impl ::std::marker::Sync for PaymentAppManager {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentTransaction(::windows::runtime::IInspectable);
impl PaymentTransaction {
    pub fn PaymentRequest(&self) -> ::windows::runtime::Result<super::PaymentRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::PaymentRequest>(result__)
        }
    }
    pub fn PayerEmail(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetPayerEmail<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PayerName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetPayerName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PayerPhoneNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetPayerPhoneNumber<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateShippingAddressAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::PaymentAddress>>(&self, shippingaddress: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), shippingaddress.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateSelectedShippingOptionAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::PaymentShippingOption>>(&self, selectedshippingoption: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), selectedshippingoption.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::PaymentRequestChangedResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AcceptAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::PaymentToken>>(&self, paymenttoken: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransactionAcceptResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), paymenttoken.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<PaymentTransactionAcceptResult>>(result__)
        }
    }
    pub fn Reject(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(id: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<PaymentTransaction>> {
        Self::IPaymentTransactionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<PaymentTransaction>>(result__)
        })
    }
    pub fn IPaymentTransactionStatics<R, F: FnOnce(&IPaymentTransactionStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PaymentTransaction, IPaymentTransactionStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentTransaction {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentTransaction;{62581da0-26a5-4e9b-a6eb-66606cf001d3})");
}
unsafe impl ::windows::runtime::Interface for PaymentTransaction {
    type Vtable = IPaymentTransaction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1649941920, 9893, 20123, [166, 235, 102, 96, 108, 240, 1, 211]);
}
impl ::windows::runtime::RuntimeName for PaymentTransaction {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentTransaction";
}
impl ::std::convert::From<PaymentTransaction> for ::windows::runtime::IUnknown {
    fn from(value: PaymentTransaction) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PaymentTransaction> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentTransaction) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentTransaction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PaymentTransaction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PaymentTransaction> for ::windows::runtime::IInspectable {
    fn from(value: PaymentTransaction) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentTransaction> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentTransaction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentTransaction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentTransaction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentTransaction {}
unsafe impl ::std::marker::Sync for PaymentTransaction {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PaymentTransactionAcceptResult(::windows::runtime::IInspectable);
impl PaymentTransactionAcceptResult {
    pub fn Status(&self) -> ::windows::runtime::Result<super::PaymentRequestCompletionStatus> {
        let this = self;
        unsafe {
            let mut result__: super::PaymentRequestCompletionStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::PaymentRequestCompletionStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PaymentTransactionAcceptResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult;{060e3276-d30c-4817-95a2-df7ae9273b56})");
}
unsafe impl ::windows::runtime::Interface for PaymentTransactionAcceptResult {
    type Vtable = IPaymentTransactionAcceptResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(101593718, 54028, 18455, [149, 162, 223, 122, 233, 39, 59, 86]);
}
impl ::windows::runtime::RuntimeName for PaymentTransactionAcceptResult {
    const NAME: &'static str = "Windows.ApplicationModel.Payments.Provider.PaymentTransactionAcceptResult";
}
impl ::std::convert::From<PaymentTransactionAcceptResult> for ::windows::runtime::IUnknown {
    fn from(value: PaymentTransactionAcceptResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PaymentTransactionAcceptResult> for ::windows::runtime::IUnknown {
    fn from(value: &PaymentTransactionAcceptResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PaymentTransactionAcceptResult> for ::windows::runtime::IInspectable {
    fn from(value: PaymentTransactionAcceptResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PaymentTransactionAcceptResult> for ::windows::runtime::IInspectable {
    fn from(value: &PaymentTransactionAcceptResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PaymentTransactionAcceptResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PaymentTransactionAcceptResult {}
unsafe impl ::std::marker::Sync for PaymentTransactionAcceptResult {}
