#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const AdSyncTask: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(719734609, 46888, 19819, [151, 160, 178, 218, 46, 125, 42, 59]);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AdrClientDisplayFlags(pub i32);
pub const AdrClientDisplayFlags_AllowEmailRequests: AdrClientDisplayFlags = AdrClientDisplayFlags(1i32);
pub const AdrClientDisplayFlags_ShowDeviceTroubleshooting: AdrClientDisplayFlags = AdrClientDisplayFlags(2i32);
impl ::std::convert::From<i32> for AdrClientDisplayFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AdrClientDisplayFlags {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AdrClientErrorType(pub i32);
pub const AdrClientErrorType_Unknown: AdrClientErrorType = AdrClientErrorType(0i32);
pub const AdrClientErrorType_AccessDenied: AdrClientErrorType = AdrClientErrorType(1i32);
pub const AdrClientErrorType_FileNotFound: AdrClientErrorType = AdrClientErrorType(2i32);
impl ::std::convert::From<i32> for AdrClientErrorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AdrClientErrorType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AdrClientFlags(pub i32);
pub const AdrClientFlags_None: AdrClientFlags = AdrClientFlags(0i32);
pub const AdrClientFlags_FailForLocalPaths: AdrClientFlags = AdrClientFlags(1i32);
pub const AdrClientFlags_FailIfNotSupportedByServer: AdrClientFlags = AdrClientFlags(2i32);
pub const AdrClientFlags_FailIfNotDomainJoined: AdrClientFlags = AdrClientFlags(4i32);
impl ::std::convert::From<i32> for AdrClientFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AdrClientFlags {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AdrEmailFlags(pub i32);
pub const AdrEmailFlags_PutDataOwnerOnToLine: AdrEmailFlags = AdrEmailFlags(1i32);
pub const AdrEmailFlags_PutAdminOnToLine: AdrEmailFlags = AdrEmailFlags(2i32);
pub const AdrEmailFlags_IncludeDeviceClaims: AdrEmailFlags = AdrEmailFlags(4i32);
pub const AdrEmailFlags_IncludeUserInfo: AdrEmailFlags = AdrEmailFlags(8i32);
pub const AdrEmailFlags_GenerateEventLog: AdrEmailFlags = AdrEmailFlags(16i32);
impl ::std::convert::From<i32> for AdrEmailFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AdrEmailFlags {
    type Abi = Self;
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DIFsrmClassificationEvents(::windows::runtime::IUnknown);
impl DIFsrmClassificationEvents {}
unsafe impl ::windows::runtime::Interface for DIFsrmClassificationEvents {
    type Vtable = DIFsrmClassificationEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(647245232, 55999, 16856, [187, 221, 177, 41, 169, 247, 4, 36]);
}
impl ::std::convert::From<DIFsrmClassificationEvents> for ::windows::runtime::IUnknown {
    fn from(value: DIFsrmClassificationEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DIFsrmClassificationEvents> for ::windows::runtime::IUnknown {
    fn from(value: &DIFsrmClassificationEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DIFsrmClassificationEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DIFsrmClassificationEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<DIFsrmClassificationEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: DIFsrmClassificationEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&DIFsrmClassificationEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &DIFsrmClassificationEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for DIFsrmClassificationEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &DIFsrmClassificationEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DIFsrmClassificationEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
pub const FSRM_DISPID_FEATURE_CLASSIFICATION: u32 = 83886080u32;
pub const FSRM_DISPID_FEATURE_FILESCREEN: u32 = 50331648u32;
pub const FSRM_DISPID_FEATURE_GENERAL: u32 = 16777216u32;
pub const FSRM_DISPID_FEATURE_MASK: u32 = 251658240u32;
pub const FSRM_DISPID_FEATURE_PIPELINE: u32 = 100663296u32;
pub const FSRM_DISPID_FEATURE_QUOTA: u32 = 33554432u32;
pub const FSRM_DISPID_FEATURE_REPORTS: u32 = 67108864u32;
pub const FSRM_DISPID_INTERFACE_A_MASK: u32 = 15728640u32;
pub const FSRM_DISPID_INTERFACE_B_MASK: u32 = 983040u32;
pub const FSRM_DISPID_INTERFACE_C_MASK: u32 = 61440u32;
pub const FSRM_DISPID_INTERFACE_D_MASK: u32 = 3840u32;
pub const FSRM_DISPID_IS_PROPERTY: u32 = 128u32;
pub const FSRM_DISPID_METHOD_NUM_MASK: u32 = 127u32;
pub const FSRM_E_ADR_MAX_EMAILS_SENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200130i32 as _);
pub const FSRM_E_ADR_NOT_DOMAIN_JOINED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200110i32 as _);
pub const FSRM_E_ADR_PATH_IS_LOCAL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200111i32 as _);
pub const FSRM_E_ADR_SRV_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200112i32 as _);
pub const FSRM_E_ALREADY_EXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200253i32 as _);
pub const FSRM_E_AUTO_QUOTA: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(283419i32 as _);
pub const FSRM_E_CACHE_INVALID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200187i32 as _);
pub const FSRM_E_CACHE_MODULE_ALREADY_EXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200186i32 as _);
pub const FSRM_E_CANNOT_AGGREGATE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200201i32 as _);
pub const FSRM_E_CANNOT_ALLOW_REPARSE_POINT_TAG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200170i32 as _);
pub const FSRM_E_CANNOT_CHANGE_PROPERTY_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200197i32 as _);
pub const FSRM_E_CANNOT_CREATE_TEMP_COPY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200132i32 as _);
pub const FSRM_E_CANNOT_DELETE_SYSTEM_PROPERTY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200135i32 as _);
pub const FSRM_E_CANNOT_REMOVE_READONLY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200109i32 as _);
pub const FSRM_E_CANNOT_RENAME_PROPERTY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200198i32 as _);
pub const FSRM_E_CANNOT_STORE_PROPERTIES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200171i32 as _);
pub const FSRM_E_CANNOT_USE_DELETED_PROPERTY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200143i32 as _);
pub const FSRM_E_CANNOT_USE_DEPRECATED_PROPERTY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200145i32 as _);
pub const FSRM_E_CLASSIFICATION_ALREADY_RUNNING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200195i32 as _);
pub const FSRM_E_CLASSIFICATION_CANCELED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200141i32 as _);
pub const FSRM_E_CLASSIFICATION_NOT_RUNNING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200194i32 as _);
pub const FSRM_E_CLASSIFICATION_PARTIAL_BATCH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200136i32 as _);
pub const FSRM_E_CLASSIFICATION_SCAN_FAIL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200148i32 as _);
pub const FSRM_E_CLASSIFICATION_TIMEOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200137i32 as _);
pub const FSRM_E_CLUSTER_NOT_RUNNING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200210i32 as _);
pub const FSRM_E_CSC_PATH_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200106i32 as _);
pub const FSRM_E_DIFFERENT_CLUSTER_GROUP: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200207i32 as _);
pub const FSRM_E_DRIVER_NOT_READY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200237i32 as _);
pub const FSRM_E_DUPLICATE_NAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200240i32 as _);
pub const FSRM_E_EMAIL_NOT_SENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200228i32 as _);
pub const FSRM_E_ENUM_PROPERTIES_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200173i32 as _);
pub const FSRM_E_ERROR_NOT_ENABLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200133i32 as _);
pub const FSRM_E_EXPIRATION_PATH_NOT_WRITEABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200105i32 as _);
pub const FSRM_E_EXPIRATION_PATH_TOO_LONG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200104i32 as _);
pub const FSRM_E_EXPIRATION_VOLUME_NOT_NTFS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200103i32 as _);
pub const FSRM_E_FAIL_BATCH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200247i32 as _);
pub const FSRM_E_FILE_ENCRYPTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200156i32 as _);
pub const FSRM_E_FILE_IN_USE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200134i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_ACTION_GET_EXITCODE_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200152i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_ACTION_TIMEOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200153i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_EXPIRATION_DIR_IN_SCOPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200185i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_EXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200184i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_ALREADY_RUNNING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200193i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_CUSTOM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200191i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_DEPRECATED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200102i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_EXPIRATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200192i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_INVALID_CONTINUOUS_CONFIG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200108i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_MAX_FILE_CONDITIONS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200146i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOTIFICATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200190i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_NOT_LEGACY_ACCESSIBLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200147i32 as _);
pub const FSRM_E_FILE_MANAGEMENT_JOB_RMS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200120i32 as _);
pub const FSRM_E_FILE_OPEN_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200189i32 as _);
pub const FSRM_E_FILE_SYSTEM_CORRUPT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200225i32 as _);
pub const FSRM_E_INCOMPATIBLE_FORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200157i32 as _);
pub const FSRM_E_INPROC_MODULE_BLOCKED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200174i32 as _);
pub const FSRM_E_INSECURE_PATH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200233i32 as _);
pub const FSRM_E_INSUFFICIENT_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200236i32 as _);
pub const FSRM_E_INVALID_AD_CLAIM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200142i32 as _);
pub const FSRM_E_INVALID_COMBINATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200241i32 as _);
pub const FSRM_E_INVALID_DATASCREEN_DEFINITION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200220i32 as _);
pub const FSRM_E_INVALID_EMAIL_ADDRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200226i32 as _);
pub const FSRM_E_INVALID_FILEGROUP_DEFINITION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200223i32 as _);
pub const FSRM_E_INVALID_FILENAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200214i32 as _);
pub const FSRM_E_INVALID_FOLDER_PROPERTY_STORE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200140i32 as _);
pub const FSRM_E_INVALID_IMPORT_VERSION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200245i32 as _);
pub const FSRM_E_INVALID_LIMIT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200249i32 as _);
pub const FSRM_E_INVALID_NAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200248i32 as _);
pub const FSRM_E_INVALID_PATH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200250i32 as _);
pub const FSRM_E_INVALID_REPORT_DESC: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200215i32 as _);
pub const FSRM_E_INVALID_REPORT_FORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200216i32 as _);
pub const FSRM_E_INVALID_SCHEDULER_ARGUMENT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200254i32 as _);
pub const FSRM_E_INVALID_SMTP_SERVER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200232i32 as _);
pub const FSRM_E_INVALID_TEXT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200246i32 as _);
pub const FSRM_E_INVALID_USER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200251i32 as _);
pub const FSRM_E_LAST_ACCESS_UPDATE_DISABLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200176i32 as _);
pub const FSRM_E_LEGACY_SCHEDULE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200107i32 as _);
pub const FSRM_E_LOADING_DISABLED_MODULE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200202i32 as _);
pub const FSRM_E_LONG_CMDLINE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200224i32 as _);
pub const FSRM_E_MAX_PROPERTY_DEFINITIONS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200196i32 as _);
pub const FSRM_E_MESSAGE_LIMIT_EXCEEDED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200200i32 as _);
pub const FSRM_E_MODULE_INITIALIZATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200150i32 as _);
pub const FSRM_E_MODULE_INVALID_PARAM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200151i32 as _);
pub const FSRM_E_MODULE_SESSION_INITIALIZATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200149i32 as _);
pub const FSRM_E_MODULE_TIMEOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200101i32 as _);
pub const FSRM_E_NOT_CLUSTER_VOLUME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200208i32 as _);
pub const FSRM_E_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200255i32 as _);
pub const FSRM_E_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200239i32 as _);
pub const FSRM_E_NO_EMAIL_ADDRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200131i32 as _);
pub const FSRM_E_NO_PROPERTY_VALUE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200175i32 as _);
pub const FSRM_E_OBJECT_IN_USE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200199i32 as _);
pub const FSRM_E_OUT_OF_RANGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200243i32 as _);
pub const FSRM_E_PARTIAL_CLASSIFICATION_PROPERTY_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200169i32 as _);
pub const FSRM_E_PATH_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200252i32 as _);
pub const FSRM_E_PATH_NOT_IN_NAMESPACE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200129i32 as _);
pub const FSRM_E_PERSIST_PROPERTIES_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200155i32 as _);
pub const FSRM_E_PERSIST_PROPERTIES_FAILED_ENCRYPTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200166i32 as _);
pub const FSRM_E_PROPERTY_DELETED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200183i32 as _);
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FILES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200138i32 as _);
pub const FSRM_E_PROPERTY_MUST_APPLY_TO_FOLDERS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200124i32 as _);
pub const FSRM_E_PROPERTY_MUST_BE_GLOBAL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200122i32 as _);
pub const FSRM_E_PROPERTY_MUST_BE_SECURE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200123i32 as _);
pub const FSRM_E_REBUILDING_FODLER_TYPE_INDEX: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200139i32 as _);
pub const FSRM_E_REPORT_GENERATION_ERR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200204i32 as _);
pub const FSRM_E_REPORT_JOB_ALREADY_RUNNING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200205i32 as _);
pub const FSRM_E_REPORT_TASK_TRIGGER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200203i32 as _);
pub const FSRM_E_REPORT_TYPE_ALREADY_EXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200206i32 as _);
pub const FSRM_E_REQD_PARAM_MISSING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200242i32 as _);
pub const FSRM_E_RMS_NO_PROTECTORS_INSTALLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200126i32 as _);
pub const FSRM_E_RMS_NO_PROTECTOR_INSTALLED_FOR_FILE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200125i32 as _);
pub const FSRM_E_RMS_TEMPLATE_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200128i32 as _);
pub const FSRM_E_SECURE_PROPERTIES_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200127i32 as _);
pub const FSRM_E_SET_PROPERTY_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200172i32 as _);
pub const FSRM_E_SHADOW_COPY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200212i32 as _);
pub const FSRM_E_STORE_NOT_INSTALLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200209i32 as _);
pub const FSRM_E_SYNC_TASK_HAD_ERRORS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200119i32 as _);
pub const FSRM_E_SYNC_TASK_TIMEOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200144i32 as _);
pub const FSRM_E_TEXTREADER_FILENAME_TOO_LONG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200158i32 as _);
pub const FSRM_E_TEXTREADER_IFILTER_CLSID_MALFORMED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200160i32 as _);
pub const FSRM_E_TEXTREADER_IFILTER_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200167i32 as _);
pub const FSRM_E_TEXTREADER_NOT_INITIALIZED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200168i32 as _);
pub const FSRM_E_TEXTREADER_STREAM_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200159i32 as _);
pub const FSRM_E_UNEXPECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200234i32 as _);
pub const FSRM_E_UNSECURE_LINK_TO_HOSTED_MODULE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200188i32 as _);
pub const FSRM_E_VOLUME_OFFLINE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200154i32 as _);
pub const FSRM_E_VOLUME_UNSUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200235i32 as _);
pub const FSRM_E_WMI_FAILURE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200121i32 as _);
pub const FSRM_E_XML_CORRUPTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147200211i32 as _);
pub const FSRM_S_CLASSIFICATION_SCAN_FAILURES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(283398i32 as _);
pub const FSRM_S_PARTIAL_BATCH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(283396i32 as _);
pub const FSRM_S_PARTIAL_CLASSIFICATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(283397i32 as _);
pub const FsrmAccessDeniedRemediationClient: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(269176776, 29889, 18191, [177, 183, 221, 123, 107, 174, 121, 189]);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmAccountType(pub i32);
pub const FsrmAccountType_Unknown: FsrmAccountType = FsrmAccountType(0i32);
pub const FsrmAccountType_NetworkService: FsrmAccountType = FsrmAccountType(1i32);
pub const FsrmAccountType_LocalService: FsrmAccountType = FsrmAccountType(2i32);
pub const FsrmAccountType_LocalSystem: FsrmAccountType = FsrmAccountType(3i32);
pub const FsrmAccountType_InProc: FsrmAccountType = FsrmAccountType(4i32);
pub const FsrmAccountType_External: FsrmAccountType = FsrmAccountType(5i32);
pub const FsrmAccountType_Automatic: FsrmAccountType = FsrmAccountType(500i32);
impl ::std::convert::From<i32> for FsrmAccountType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmAccountType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmActionType(pub i32);
pub const FsrmActionType_Unknown: FsrmActionType = FsrmActionType(0i32);
pub const FsrmActionType_EventLog: FsrmActionType = FsrmActionType(1i32);
pub const FsrmActionType_Email: FsrmActionType = FsrmActionType(2i32);
pub const FsrmActionType_Command: FsrmActionType = FsrmActionType(3i32);
pub const FsrmActionType_Report: FsrmActionType = FsrmActionType(4i32);
impl ::std::convert::From<i32> for FsrmActionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmActionType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmClassificationLoggingFlags(pub i32);
pub const FsrmClassificationLoggingFlags_None: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(0i32);
pub const FsrmClassificationLoggingFlags_ClassificationsInLogFile: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(1i32);
pub const FsrmClassificationLoggingFlags_ErrorsInLogFile: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(2i32);
pub const FsrmClassificationLoggingFlags_ClassificationsInSystemLog: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(4i32);
pub const FsrmClassificationLoggingFlags_ErrorsInSystemLog: FsrmClassificationLoggingFlags = FsrmClassificationLoggingFlags(8i32);
impl ::std::convert::From<i32> for FsrmClassificationLoggingFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmClassificationLoggingFlags {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FsrmClassificationManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2975600199, 50065, 17849, [149, 200, 235, 89, 108, 133, 63, 58]);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmCollectionState(pub i32);
pub const FsrmCollectionState_Fetching: FsrmCollectionState = FsrmCollectionState(1i32);
pub const FsrmCollectionState_Committing: FsrmCollectionState = FsrmCollectionState(2i32);
pub const FsrmCollectionState_Complete: FsrmCollectionState = FsrmCollectionState(3i32);
pub const FsrmCollectionState_Cancelled: FsrmCollectionState = FsrmCollectionState(4i32);
impl ::std::convert::From<i32> for FsrmCollectionState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmCollectionState {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmCommitOptions(pub i32);
pub const FsrmCommitOptions_None: FsrmCommitOptions = FsrmCommitOptions(0i32);
pub const FsrmCommitOptions_Asynchronous: FsrmCommitOptions = FsrmCommitOptions(1i32);
impl ::std::convert::From<i32> for FsrmCommitOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmCommitOptions {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FsrmDaysNotSpecified: i32 = -1i32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmEnumOptions(pub i32);
pub const FsrmEnumOptions_None: FsrmEnumOptions = FsrmEnumOptions(0i32);
pub const FsrmEnumOptions_Asynchronous: FsrmEnumOptions = FsrmEnumOptions(1i32);
pub const FsrmEnumOptions_CheckRecycleBin: FsrmEnumOptions = FsrmEnumOptions(2i32);
pub const FsrmEnumOptions_IncludeClusterNodes: FsrmEnumOptions = FsrmEnumOptions(4i32);
pub const FsrmEnumOptions_IncludeDeprecatedObjects: FsrmEnumOptions = FsrmEnumOptions(8i32);
impl ::std::convert::From<i32> for FsrmEnumOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmEnumOptions {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmEventType(pub i32);
pub const FsrmEventType_Unknown: FsrmEventType = FsrmEventType(0i32);
pub const FsrmEventType_Information: FsrmEventType = FsrmEventType(1i32);
pub const FsrmEventType_Warning: FsrmEventType = FsrmEventType(2i32);
pub const FsrmEventType_Error: FsrmEventType = FsrmEventType(3i32);
impl ::std::convert::From<i32> for FsrmEventType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmEventType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmExecutionOption(pub i32);
pub const FsrmExecutionOption_Unknown: FsrmExecutionOption = FsrmExecutionOption(0i32);
pub const FsrmExecutionOption_EvaluateUnset: FsrmExecutionOption = FsrmExecutionOption(1i32);
pub const FsrmExecutionOption_ReEvaluate_ConsiderExistingValue: FsrmExecutionOption = FsrmExecutionOption(2i32);
pub const FsrmExecutionOption_ReEvaluate_IgnoreExistingValue: FsrmExecutionOption = FsrmExecutionOption(3i32);
impl ::std::convert::From<i32> for FsrmExecutionOption {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmExecutionOption {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FsrmExportImport: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(344120375, 64233, 18311, [144, 37, 140, 228, 224, 36, 171, 86]);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmFileConditionType(pub i32);
pub const FsrmFileConditionType_Unknown: FsrmFileConditionType = FsrmFileConditionType(0i32);
pub const FsrmFileConditionType_Property: FsrmFileConditionType = FsrmFileConditionType(1i32);
impl ::std::convert::From<i32> for FsrmFileConditionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmFileConditionType {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FsrmFileGroupManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2400412662, 25967, 17558, [146, 38, 19, 174, 203, 215, 113, 143]);
pub const FsrmFileManagementJobManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3944282546, 19514, 17185, [178, 3, 32, 81, 32, 207, 246, 20]);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmFileManagementLoggingFlags(pub i32);
pub const FsrmFileManagementLoggingFlags_None: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(0i32);
pub const FsrmFileManagementLoggingFlags_Error: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(1i32);
pub const FsrmFileManagementLoggingFlags_Information: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(2i32);
pub const FsrmFileManagementLoggingFlags_Audit: FsrmFileManagementLoggingFlags = FsrmFileManagementLoggingFlags(4i32);
impl ::std::convert::From<i32> for FsrmFileManagementLoggingFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmFileManagementLoggingFlags {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmFileManagementType(pub i32);
pub const FsrmFileManagementType_Unknown: FsrmFileManagementType = FsrmFileManagementType(0i32);
pub const FsrmFileManagementType_Expiration: FsrmFileManagementType = FsrmFileManagementType(1i32);
pub const FsrmFileManagementType_Custom: FsrmFileManagementType = FsrmFileManagementType(2i32);
pub const FsrmFileManagementType_Rms: FsrmFileManagementType = FsrmFileManagementType(3i32);
impl ::std::convert::From<i32> for FsrmFileManagementType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmFileManagementType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmFileScreenFlags(pub i32);
pub const FsrmFileScreenFlags_Enforce: FsrmFileScreenFlags = FsrmFileScreenFlags(1i32);
impl ::std::convert::From<i32> for FsrmFileScreenFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmFileScreenFlags {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FsrmFileScreenManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2509508995, 56147, 19551, [179, 123, 125, 9, 33, 207, 157, 199]);
pub const FsrmFileScreenTemplateManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(607195615, 58484, 18090, [160, 84, 234, 163, 62, 220, 41, 42]);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmFileStreamingInterfaceType(pub i32);
pub const FsrmFileStreamingInterfaceType_Unknown: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(0i32);
pub const FsrmFileStreamingInterfaceType_ILockBytes: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(1i32);
pub const FsrmFileStreamingInterfaceType_IStream: FsrmFileStreamingInterfaceType = FsrmFileStreamingInterfaceType(2i32);
impl ::std::convert::From<i32> for FsrmFileStreamingInterfaceType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmFileStreamingInterfaceType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmFileStreamingMode(pub i32);
pub const FsrmFileStreamingMode_Unknown: FsrmFileStreamingMode = FsrmFileStreamingMode(0i32);
pub const FsrmFileStreamingMode_Read: FsrmFileStreamingMode = FsrmFileStreamingMode(1i32);
pub const FsrmFileStreamingMode_Write: FsrmFileStreamingMode = FsrmFileStreamingMode(2i32);
impl ::std::convert::From<i32> for FsrmFileStreamingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmFileStreamingMode {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmFileSystemPropertyId(pub i32);
pub const FsrmFileSystemPropertyId_Undefined: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(0i32);
pub const FsrmFileSystemPropertyId_FileName: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(1i32);
pub const FsrmFileSystemPropertyId_DateCreated: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(2i32);
pub const FsrmFileSystemPropertyId_DateLastAccessed: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(3i32);
pub const FsrmFileSystemPropertyId_DateLastModified: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(4i32);
pub const FsrmFileSystemPropertyId_DateNow: FsrmFileSystemPropertyId = FsrmFileSystemPropertyId(5i32);
impl ::std::convert::From<i32> for FsrmFileSystemPropertyId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmFileSystemPropertyId {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmGetFilePropertyOptions(pub i32);
pub const FsrmGetFilePropertyOptions_None: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(0i32);
pub const FsrmGetFilePropertyOptions_NoRuleEvaluation: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(1i32);
pub const FsrmGetFilePropertyOptions_Persistent: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(2i32);
pub const FsrmGetFilePropertyOptions_FailOnPersistErrors: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(4i32);
pub const FsrmGetFilePropertyOptions_SkipOrphaned: FsrmGetFilePropertyOptions = FsrmGetFilePropertyOptions(8i32);
impl ::std::convert::From<i32> for FsrmGetFilePropertyOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmGetFilePropertyOptions {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FsrmMaxExcludeFolders: u32 = 32u32;
pub const FsrmMaxNumberPropertyDefinitions: u32 = 100u32;
pub const FsrmMaxNumberThresholds: u32 = 16u32;
pub const FsrmMaxThresholdValue: u32 = 250u32;
pub const FsrmMinQuotaLimit: u32 = 1024u32;
pub const FsrmMinThresholdValue: u32 = 1u32;
pub const FsrmPathMapper: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4089332413, 35522, 16542, [187, 216, 250, 249, 182, 180, 31, 235]);
pub const FsrmPipelineModuleConnector: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3345232757, 7861, 17630, [160, 98, 98, 53, 71, 217, 51, 188]);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmPipelineModuleType(pub i32);
pub const FsrmPipelineModuleType_Unknown: FsrmPipelineModuleType = FsrmPipelineModuleType(0i32);
pub const FsrmPipelineModuleType_Storage: FsrmPipelineModuleType = FsrmPipelineModuleType(1i32);
pub const FsrmPipelineModuleType_Classifier: FsrmPipelineModuleType = FsrmPipelineModuleType(2i32);
impl ::std::convert::From<i32> for FsrmPipelineModuleType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmPipelineModuleType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmPropertyBagField(pub i32);
pub const FsrmPropertyBagField_AccessVolume: FsrmPropertyBagField = FsrmPropertyBagField(0i32);
pub const FsrmPropertyBagField_VolumeGuidName: FsrmPropertyBagField = FsrmPropertyBagField(1i32);
impl ::std::convert::From<i32> for FsrmPropertyBagField {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmPropertyBagField {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmPropertyBagFlags(pub i32);
pub const FsrmPropertyBagFlags_UpdatedByClassifier: FsrmPropertyBagFlags = FsrmPropertyBagFlags(1i32);
pub const FsrmPropertyBagFlags_FailedLoadingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(2i32);
pub const FsrmPropertyBagFlags_FailedSavingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(4i32);
pub const FsrmPropertyBagFlags_FailedClassifyingProperties: FsrmPropertyBagFlags = FsrmPropertyBagFlags(8i32);
impl ::std::convert::From<i32> for FsrmPropertyBagFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmPropertyBagFlags {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmPropertyConditionType(pub i32);
pub const FsrmPropertyConditionType_Unknown: FsrmPropertyConditionType = FsrmPropertyConditionType(0i32);
pub const FsrmPropertyConditionType_Equal: FsrmPropertyConditionType = FsrmPropertyConditionType(1i32);
pub const FsrmPropertyConditionType_NotEqual: FsrmPropertyConditionType = FsrmPropertyConditionType(2i32);
pub const FsrmPropertyConditionType_GreaterThan: FsrmPropertyConditionType = FsrmPropertyConditionType(3i32);
pub const FsrmPropertyConditionType_LessThan: FsrmPropertyConditionType = FsrmPropertyConditionType(4i32);
pub const FsrmPropertyConditionType_Contain: FsrmPropertyConditionType = FsrmPropertyConditionType(5i32);
pub const FsrmPropertyConditionType_Exist: FsrmPropertyConditionType = FsrmPropertyConditionType(6i32);
pub const FsrmPropertyConditionType_NotExist: FsrmPropertyConditionType = FsrmPropertyConditionType(7i32);
pub const FsrmPropertyConditionType_StartWith: FsrmPropertyConditionType = FsrmPropertyConditionType(8i32);
pub const FsrmPropertyConditionType_EndWith: FsrmPropertyConditionType = FsrmPropertyConditionType(9i32);
pub const FsrmPropertyConditionType_ContainedIn: FsrmPropertyConditionType = FsrmPropertyConditionType(10i32);
pub const FsrmPropertyConditionType_PrefixOf: FsrmPropertyConditionType = FsrmPropertyConditionType(11i32);
pub const FsrmPropertyConditionType_SuffixOf: FsrmPropertyConditionType = FsrmPropertyConditionType(12i32);
pub const FsrmPropertyConditionType_MatchesPattern: FsrmPropertyConditionType = FsrmPropertyConditionType(13i32);
impl ::std::convert::From<i32> for FsrmPropertyConditionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmPropertyConditionType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmPropertyDefinitionAppliesTo(pub i32);
pub const FsrmPropertyDefinitionAppliesTo_Files: FsrmPropertyDefinitionAppliesTo = FsrmPropertyDefinitionAppliesTo(1i32);
pub const FsrmPropertyDefinitionAppliesTo_Folders: FsrmPropertyDefinitionAppliesTo = FsrmPropertyDefinitionAppliesTo(2i32);
impl ::std::convert::From<i32> for FsrmPropertyDefinitionAppliesTo {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmPropertyDefinitionAppliesTo {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmPropertyDefinitionFlags(pub i32);
pub const FsrmPropertyDefinitionFlags_Global: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(1i32);
pub const FsrmPropertyDefinitionFlags_Deprecated: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(2i32);
pub const FsrmPropertyDefinitionFlags_Secure: FsrmPropertyDefinitionFlags = FsrmPropertyDefinitionFlags(4i32);
impl ::std::convert::From<i32> for FsrmPropertyDefinitionFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmPropertyDefinitionFlags {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmPropertyDefinitionType(pub i32);
pub const FsrmPropertyDefinitionType_Unknown: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(0i32);
pub const FsrmPropertyDefinitionType_OrderedList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(1i32);
pub const FsrmPropertyDefinitionType_MultiChoiceList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(2i32);
pub const FsrmPropertyDefinitionType_SingleChoiceList: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(3i32);
pub const FsrmPropertyDefinitionType_String: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(4i32);
pub const FsrmPropertyDefinitionType_MultiString: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(5i32);
pub const FsrmPropertyDefinitionType_Int: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(6i32);
pub const FsrmPropertyDefinitionType_Bool: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(7i32);
pub const FsrmPropertyDefinitionType_Date: FsrmPropertyDefinitionType = FsrmPropertyDefinitionType(8i32);
impl ::std::convert::From<i32> for FsrmPropertyDefinitionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmPropertyDefinitionType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmPropertyFlags(pub i32);
pub const FsrmPropertyFlags_None: FsrmPropertyFlags = FsrmPropertyFlags(0i32);
pub const FsrmPropertyFlags_Orphaned: FsrmPropertyFlags = FsrmPropertyFlags(1i32);
pub const FsrmPropertyFlags_RetrievedFromCache: FsrmPropertyFlags = FsrmPropertyFlags(2i32);
pub const FsrmPropertyFlags_RetrievedFromStorage: FsrmPropertyFlags = FsrmPropertyFlags(4i32);
pub const FsrmPropertyFlags_SetByClassifier: FsrmPropertyFlags = FsrmPropertyFlags(8i32);
pub const FsrmPropertyFlags_Deleted: FsrmPropertyFlags = FsrmPropertyFlags(16i32);
pub const FsrmPropertyFlags_Reclassified: FsrmPropertyFlags = FsrmPropertyFlags(32i32);
pub const FsrmPropertyFlags_AggregationFailed: FsrmPropertyFlags = FsrmPropertyFlags(64i32);
pub const FsrmPropertyFlags_Existing: FsrmPropertyFlags = FsrmPropertyFlags(128i32);
pub const FsrmPropertyFlags_FailedLoadingProperties: FsrmPropertyFlags = FsrmPropertyFlags(256i32);
pub const FsrmPropertyFlags_FailedClassifyingProperties: FsrmPropertyFlags = FsrmPropertyFlags(512i32);
pub const FsrmPropertyFlags_FailedSavingProperties: FsrmPropertyFlags = FsrmPropertyFlags(1024i32);
pub const FsrmPropertyFlags_Secure: FsrmPropertyFlags = FsrmPropertyFlags(2048i32);
pub const FsrmPropertyFlags_PolicyDerived: FsrmPropertyFlags = FsrmPropertyFlags(4096i32);
pub const FsrmPropertyFlags_Inherited: FsrmPropertyFlags = FsrmPropertyFlags(8192i32);
pub const FsrmPropertyFlags_Manual: FsrmPropertyFlags = FsrmPropertyFlags(16384i32);
pub const FsrmPropertyFlags_ExplicitValueDeleted: FsrmPropertyFlags = FsrmPropertyFlags(32768i32);
pub const FsrmPropertyFlags_PropertyDeletedFromClear: FsrmPropertyFlags = FsrmPropertyFlags(65536i32);
pub const FsrmPropertyFlags_PropertySourceMask: FsrmPropertyFlags = FsrmPropertyFlags(14i32);
pub const FsrmPropertyFlags_PersistentMask: FsrmPropertyFlags = FsrmPropertyFlags(20480i32);
impl ::std::convert::From<i32> for FsrmPropertyFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmPropertyFlags {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmPropertyValueType(pub i32);
pub const FsrmPropertyValueType_Undefined: FsrmPropertyValueType = FsrmPropertyValueType(0i32);
pub const FsrmPropertyValueType_Literal: FsrmPropertyValueType = FsrmPropertyValueType(1i32);
pub const FsrmPropertyValueType_DateOffset: FsrmPropertyValueType = FsrmPropertyValueType(2i32);
impl ::std::convert::From<i32> for FsrmPropertyValueType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmPropertyValueType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmQuotaFlags(pub i32);
pub const FsrmQuotaFlags_Enforce: FsrmQuotaFlags = FsrmQuotaFlags(256i32);
pub const FsrmQuotaFlags_Disable: FsrmQuotaFlags = FsrmQuotaFlags(512i32);
pub const FsrmQuotaFlags_StatusIncomplete: FsrmQuotaFlags = FsrmQuotaFlags(65536i32);
pub const FsrmQuotaFlags_StatusRebuilding: FsrmQuotaFlags = FsrmQuotaFlags(131072i32);
impl ::std::convert::From<i32> for FsrmQuotaFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmQuotaFlags {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FsrmQuotaManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2430380927, 13436, 19452, [181, 67, 84, 3, 38, 48, 95, 190]);
pub const FsrmQuotaTemplateManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2547242051, 9500, 17207, [129, 231, 179, 46, 143, 78, 230, 94]);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmReportFilter(pub i32);
pub const FsrmReportFilter_MinSize: FsrmReportFilter = FsrmReportFilter(1i32);
pub const FsrmReportFilter_MinAgeDays: FsrmReportFilter = FsrmReportFilter(2i32);
pub const FsrmReportFilter_MaxAgeDays: FsrmReportFilter = FsrmReportFilter(3i32);
pub const FsrmReportFilter_MinQuotaUsage: FsrmReportFilter = FsrmReportFilter(4i32);
pub const FsrmReportFilter_FileGroups: FsrmReportFilter = FsrmReportFilter(5i32);
pub const FsrmReportFilter_Owners: FsrmReportFilter = FsrmReportFilter(6i32);
pub const FsrmReportFilter_NamePattern: FsrmReportFilter = FsrmReportFilter(7i32);
pub const FsrmReportFilter_Property: FsrmReportFilter = FsrmReportFilter(8i32);
impl ::std::convert::From<i32> for FsrmReportFilter {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmReportFilter {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmReportFormat(pub i32);
pub const FsrmReportFormat_Unknown: FsrmReportFormat = FsrmReportFormat(0i32);
pub const FsrmReportFormat_DHtml: FsrmReportFormat = FsrmReportFormat(1i32);
pub const FsrmReportFormat_Html: FsrmReportFormat = FsrmReportFormat(2i32);
pub const FsrmReportFormat_Txt: FsrmReportFormat = FsrmReportFormat(3i32);
pub const FsrmReportFormat_Csv: FsrmReportFormat = FsrmReportFormat(4i32);
pub const FsrmReportFormat_Xml: FsrmReportFormat = FsrmReportFormat(5i32);
impl ::std::convert::From<i32> for FsrmReportFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmReportFormat {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmReportGenerationContext(pub i32);
pub const FsrmReportGenerationContext_Undefined: FsrmReportGenerationContext = FsrmReportGenerationContext(1i32);
pub const FsrmReportGenerationContext_ScheduledReport: FsrmReportGenerationContext = FsrmReportGenerationContext(2i32);
pub const FsrmReportGenerationContext_InteractiveReport: FsrmReportGenerationContext = FsrmReportGenerationContext(3i32);
pub const FsrmReportGenerationContext_IncidentReport: FsrmReportGenerationContext = FsrmReportGenerationContext(4i32);
impl ::std::convert::From<i32> for FsrmReportGenerationContext {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmReportGenerationContext {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmReportLimit(pub i32);
pub const FsrmReportLimit_MaxFiles: FsrmReportLimit = FsrmReportLimit(1i32);
pub const FsrmReportLimit_MaxFileGroups: FsrmReportLimit = FsrmReportLimit(2i32);
pub const FsrmReportLimit_MaxOwners: FsrmReportLimit = FsrmReportLimit(3i32);
pub const FsrmReportLimit_MaxFilesPerFileGroup: FsrmReportLimit = FsrmReportLimit(4i32);
pub const FsrmReportLimit_MaxFilesPerOwner: FsrmReportLimit = FsrmReportLimit(5i32);
pub const FsrmReportLimit_MaxFilesPerDuplGroup: FsrmReportLimit = FsrmReportLimit(6i32);
pub const FsrmReportLimit_MaxDuplicateGroups: FsrmReportLimit = FsrmReportLimit(7i32);
pub const FsrmReportLimit_MaxQuotas: FsrmReportLimit = FsrmReportLimit(8i32);
pub const FsrmReportLimit_MaxFileScreenEvents: FsrmReportLimit = FsrmReportLimit(9i32);
pub const FsrmReportLimit_MaxPropertyValues: FsrmReportLimit = FsrmReportLimit(10i32);
pub const FsrmReportLimit_MaxFilesPerPropertyValue: FsrmReportLimit = FsrmReportLimit(11i32);
pub const FsrmReportLimit_MaxFolders: FsrmReportLimit = FsrmReportLimit(12i32);
impl ::std::convert::From<i32> for FsrmReportLimit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmReportLimit {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FsrmReportManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(5828407, 43622, 19528, [189, 91, 47, 206, 67, 42, 176, 200]);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmReportRunningStatus(pub i32);
pub const FsrmReportRunningStatus_Unknown: FsrmReportRunningStatus = FsrmReportRunningStatus(0i32);
pub const FsrmReportRunningStatus_NotRunning: FsrmReportRunningStatus = FsrmReportRunningStatus(1i32);
pub const FsrmReportRunningStatus_Queued: FsrmReportRunningStatus = FsrmReportRunningStatus(2i32);
pub const FsrmReportRunningStatus_Running: FsrmReportRunningStatus = FsrmReportRunningStatus(3i32);
impl ::std::convert::From<i32> for FsrmReportRunningStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmReportRunningStatus {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FsrmReportScheduler: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3928355256, 7053, 17040, [142, 232, 225, 124, 18, 194, 254, 32]);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmReportType(pub i32);
pub const FsrmReportType_Unknown: FsrmReportType = FsrmReportType(0i32);
pub const FsrmReportType_LargeFiles: FsrmReportType = FsrmReportType(1i32);
pub const FsrmReportType_FilesByType: FsrmReportType = FsrmReportType(2i32);
pub const FsrmReportType_LeastRecentlyAccessed: FsrmReportType = FsrmReportType(3i32);
pub const FsrmReportType_MostRecentlyAccessed: FsrmReportType = FsrmReportType(4i32);
pub const FsrmReportType_QuotaUsage: FsrmReportType = FsrmReportType(5i32);
pub const FsrmReportType_FilesByOwner: FsrmReportType = FsrmReportType(6i32);
pub const FsrmReportType_ExportReport: FsrmReportType = FsrmReportType(7i32);
pub const FsrmReportType_DuplicateFiles: FsrmReportType = FsrmReportType(8i32);
pub const FsrmReportType_FileScreenAudit: FsrmReportType = FsrmReportType(9i32);
pub const FsrmReportType_FilesByProperty: FsrmReportType = FsrmReportType(10i32);
pub const FsrmReportType_AutomaticClassification: FsrmReportType = FsrmReportType(11i32);
pub const FsrmReportType_Expiration: FsrmReportType = FsrmReportType(12i32);
pub const FsrmReportType_FoldersByProperty: FsrmReportType = FsrmReportType(13i32);
impl ::std::convert::From<i32> for FsrmReportType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmReportType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmRuleFlags(pub i32);
pub const FsrmRuleFlags_Disabled: FsrmRuleFlags = FsrmRuleFlags(256i32);
pub const FsrmRuleFlags_ClearAutomaticallyClassifiedProperty: FsrmRuleFlags = FsrmRuleFlags(1024i32);
pub const FsrmRuleFlags_ClearManuallyClassifiedProperty: FsrmRuleFlags = FsrmRuleFlags(2048i32);
pub const FsrmRuleFlags_Invalid: FsrmRuleFlags = FsrmRuleFlags(4096i32);
impl ::std::convert::From<i32> for FsrmRuleFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmRuleFlags {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmRuleType(pub i32);
pub const FsrmRuleType_Unknown: FsrmRuleType = FsrmRuleType(0i32);
pub const FsrmRuleType_Classification: FsrmRuleType = FsrmRuleType(1i32);
pub const FsrmRuleType_Generic: FsrmRuleType = FsrmRuleType(2i32);
impl ::std::convert::From<i32> for FsrmRuleType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmRuleType {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FsrmSetting: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4116109064, 27981, 17812, [156, 97, 125, 187, 13, 174, 42, 70]);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmStorageModuleCaps(pub i32);
pub const FsrmStorageModuleCaps_Unknown: FsrmStorageModuleCaps = FsrmStorageModuleCaps(0i32);
pub const FsrmStorageModuleCaps_CanGet: FsrmStorageModuleCaps = FsrmStorageModuleCaps(1i32);
pub const FsrmStorageModuleCaps_CanSet: FsrmStorageModuleCaps = FsrmStorageModuleCaps(2i32);
pub const FsrmStorageModuleCaps_CanHandleDirectories: FsrmStorageModuleCaps = FsrmStorageModuleCaps(4i32);
pub const FsrmStorageModuleCaps_CanHandleFiles: FsrmStorageModuleCaps = FsrmStorageModuleCaps(8i32);
impl ::std::convert::From<i32> for FsrmStorageModuleCaps {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmStorageModuleCaps {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmStorageModuleType(pub i32);
pub const FsrmStorageModuleType_Unknown: FsrmStorageModuleType = FsrmStorageModuleType(0i32);
pub const FsrmStorageModuleType_Cache: FsrmStorageModuleType = FsrmStorageModuleType(1i32);
pub const FsrmStorageModuleType_InFile: FsrmStorageModuleType = FsrmStorageModuleType(2i32);
pub const FsrmStorageModuleType_Database: FsrmStorageModuleType = FsrmStorageModuleType(3i32);
pub const FsrmStorageModuleType_System: FsrmStorageModuleType = FsrmStorageModuleType(100i32);
impl ::std::convert::From<i32> for FsrmStorageModuleType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmStorageModuleType {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FsrmTemplateApplyOptions(pub i32);
pub const FsrmTemplateApplyOptions_ApplyToDerivedMatching: FsrmTemplateApplyOptions = FsrmTemplateApplyOptions(1i32);
pub const FsrmTemplateApplyOptions_ApplyToDerivedAll: FsrmTemplateApplyOptions = FsrmTemplateApplyOptions(2i32);
impl ::std::convert::From<i32> for FsrmTemplateApplyOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FsrmTemplateApplyOptions {
    type Abi = Self;
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmAccessDeniedRemediationClient(::windows::runtime::IUnknown);
impl IFsrmAccessDeniedRemediationClient {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, parentwnd: usize, accesspath: Param1, errortype: AdrClientErrorType, flags: i32, windowtitle: Param4, windowmessage: Param5) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(parentwnd), accesspath.into_param().abi(), ::std::mem::transmute(errortype), ::std::mem::transmute(flags), windowtitle.into_param().abi(), windowmessage.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmAccessDeniedRemediationClient {
    type Vtable = IFsrmAccessDeniedRemediationClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1073750804, 22795, 17829, [142, 27, 140, 5, 218, 82, 126, 82]);
}
impl ::std::convert::From<IFsrmAccessDeniedRemediationClient> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmAccessDeniedRemediationClient) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmAccessDeniedRemediationClient> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmAccessDeniedRemediationClient) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmAccessDeniedRemediationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmAccessDeniedRemediationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmAccessDeniedRemediationClient> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmAccessDeniedRemediationClient) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmAccessDeniedRemediationClient> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmAccessDeniedRemediationClient) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmAccessDeniedRemediationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmAccessDeniedRemediationClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAccessDeniedRemediationClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parentwnd: usize, accesspath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, errortype: AdrClientErrorType, flags: i32, windowtitle: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, windowmessage: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, result: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmAction(::windows::runtime::IUnknown);
impl IFsrmAction {
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::runtime::Result<FsrmActionType> {
        let mut result__: <FsrmActionType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmAction {
    type Vtable = IFsrmAction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1825980554, 44640, 17979, [158, 241, 225, 23, 83, 77, 105, 220]);
}
impl ::std::convert::From<IFsrmAction> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmAction) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmAction> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmAction) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmAction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmAction {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmAction> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmAction) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmAction> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmAction) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmAction {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmAction {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAction_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actiontype: *mut FsrmActionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minutes: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minutes: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmActionCommand(::windows::runtime::IUnknown);
impl IFsrmActionCommand {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::runtime::Result<FsrmActionType> {
        let mut result__: <FsrmActionType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExecutablePath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExecutablePath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, executablepath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), executablepath.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Arguments(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetArguments<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, arguments: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), arguments.into_param().abi()).ok()
    }
    pub unsafe fn Account(&self) -> ::windows::runtime::Result<FsrmAccountType> {
        let mut result__: <FsrmAccountType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmAccountType>(result__)
    }
    pub unsafe fn SetAccount(&self, account: FsrmAccountType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(account)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WorkingDirectory(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWorkingDirectory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, workingdirectory: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), workingdirectory.into_param().abi()).ok()
    }
    pub unsafe fn MonitorCommand(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetMonitorCommand(&self, monitorcommand: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(monitorcommand)).ok()
    }
    pub unsafe fn KillTimeOut(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetKillTimeOut(&self, minutes: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(minutes)).ok()
    }
    pub unsafe fn LogResult(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetLogResult(&self, logresults: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(logresults)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmActionCommand {
    type Vtable = IFsrmActionCommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(311654281, 57927, 18711, [156, 32, 243, 238, 156, 126, 231, 131]);
}
impl ::std::convert::From<IFsrmActionCommand> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmActionCommand) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmActionCommand> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmActionCommand) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmActionCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmActionCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmActionCommand> for IFsrmAction {
    fn from(value: IFsrmActionCommand) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmActionCommand> for IFsrmAction {
    fn from(value: &IFsrmActionCommand) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmAction> for IFsrmActionCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmAction>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmAction> for &IFsrmActionCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmAction>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmActionCommand> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmActionCommand) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmActionCommand> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmActionCommand) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmActionCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmActionCommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionCommand_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actiontype: *mut FsrmActionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minutes: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minutes: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, executablepath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, executablepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, arguments: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, arguments: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, account: *mut FsrmAccountType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, account: FsrmAccountType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, workingdirectory: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, workingdirectory: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, monitorcommand: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, monitorcommand: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minutes: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minutes: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, logresults: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, logresults: i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmActionEmail(::windows::runtime::IUnknown);
impl IFsrmActionEmail {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::runtime::Result<FsrmActionType> {
        let mut result__: <FsrmActionType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailFrom(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailFrom<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailfrom: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), mailfrom.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailReplyTo(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailReplyTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailreplyto: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), mailreplyto.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailTo(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailCc(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailCc<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailcc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), mailcc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailBcc(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailBcc<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailbcc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), mailbcc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailSubject(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailSubject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailsubject: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), mailsubject.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageText(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMessageText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, messagetext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), messagetext.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmActionEmail {
    type Vtable = IFsrmActionEmail_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3594933885, 9902, 19626, [159, 132, 78, 10, 173, 32, 127, 202]);
}
impl ::std::convert::From<IFsrmActionEmail> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmActionEmail) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmActionEmail> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmActionEmail) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmActionEmail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmActionEmail {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmActionEmail> for IFsrmAction {
    fn from(value: IFsrmActionEmail) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmActionEmail> for IFsrmAction {
    fn from(value: &IFsrmActionEmail) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmAction> for IFsrmActionEmail {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmAction>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmAction> for &IFsrmActionEmail {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmAction>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmActionEmail> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmActionEmail) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmActionEmail> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmActionEmail) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmActionEmail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmActionEmail {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmail_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actiontype: *mut FsrmActionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minutes: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minutes: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailfrom: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailfrom: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailreplyto: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailreplyto: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailto: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailto: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailcc: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailcc: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailbcc: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailbcc: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailsubject: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailsubject: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetext: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetext: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmActionEmail2(::windows::runtime::IUnknown);
impl IFsrmActionEmail2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::runtime::Result<FsrmActionType> {
        let mut result__: <FsrmActionType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailFrom(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailFrom<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailfrom: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), mailfrom.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailReplyTo(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailReplyTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailreplyto: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), mailreplyto.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailTo(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailCc(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailCc<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailcc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), mailcc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailBcc(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailBcc<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailbcc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), mailbcc.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailSubject(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailSubject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailsubject: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), mailsubject.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageText(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMessageText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, messagetext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), messagetext.into_param().abi()).ok()
    }
    pub unsafe fn AttachmentFileListSize(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetAttachmentFileListSize(&self, attachmentfilelistsize: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(attachmentfilelistsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmActionEmail2 {
    type Vtable = IFsrmActionEmail2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2188800047, 9522, 18489, [137, 191, 72, 114, 96, 154, 46, 164]);
}
impl ::std::convert::From<IFsrmActionEmail2> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmActionEmail2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmActionEmail2> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmActionEmail2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmActionEmail2> for IFsrmActionEmail {
    fn from(value: IFsrmActionEmail2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmActionEmail2> for IFsrmActionEmail {
    fn from(value: &IFsrmActionEmail2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmActionEmail> for IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmActionEmail> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmActionEmail>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmActionEmail> for &IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmActionEmail> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmActionEmail>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmActionEmail2> for IFsrmAction {
    fn from(value: IFsrmActionEmail2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmActionEmail2> for IFsrmAction {
    fn from(value: &IFsrmActionEmail2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmAction> for IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmAction>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmAction> for &IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmAction>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmActionEmail2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmActionEmail2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmActionEmail2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmActionEmail2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmActionEmail2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEmail2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actiontype: *mut FsrmActionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minutes: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minutes: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailfrom: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailfrom: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailreplyto: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailreplyto: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailto: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailto: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailcc: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailcc: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailbcc: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailbcc: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailsubject: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailsubject: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetext: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetext: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attachmentfilelistsize: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attachmentfilelistsize: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmActionEventLog(::windows::runtime::IUnknown);
impl IFsrmActionEventLog {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::runtime::Result<FsrmActionType> {
        let mut result__: <FsrmActionType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EventType(&self) -> ::windows::runtime::Result<FsrmEventType> {
        let mut result__: <FsrmEventType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmEventType>(result__)
    }
    pub unsafe fn SetEventType(&self, eventtype: FsrmEventType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(eventtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MessageText(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMessageText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, messagetext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), messagetext.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmActionEventLog {
    type Vtable = IFsrmActionEventLog_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1284478659, 23956, 20279, [164, 244, 245, 106, 180, 99, 84, 111]);
}
impl ::std::convert::From<IFsrmActionEventLog> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmActionEventLog) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmActionEventLog> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmActionEventLog) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmActionEventLog {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmActionEventLog {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmActionEventLog> for IFsrmAction {
    fn from(value: IFsrmActionEventLog) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmActionEventLog> for IFsrmAction {
    fn from(value: &IFsrmActionEventLog) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmAction> for IFsrmActionEventLog {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmAction>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmAction> for &IFsrmActionEventLog {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmAction>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmActionEventLog> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmActionEventLog) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmActionEventLog> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmActionEventLog) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmActionEventLog {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmActionEventLog {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionEventLog_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actiontype: *mut FsrmActionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minutes: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minutes: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventtype: *mut FsrmEventType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventtype: FsrmEventType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetext: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetext: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmActionReport(::windows::runtime::IUnknown);
impl IFsrmActionReport {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    pub unsafe fn ActionType(&self) -> ::windows::runtime::Result<FsrmActionType> {
        let mut result__: <FsrmActionType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmActionType>(result__)
    }
    pub unsafe fn RunLimitInterval(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRunLimitInterval(&self, minutes: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(minutes)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ReportTypes(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetReportTypes(&self, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(reporttypes)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailTo(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmActionReport {
    type Vtable = IFsrmActionReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(767452100, 45888, 18592, [165, 176, 21, 142, 7, 252, 86, 126]);
}
impl ::std::convert::From<IFsrmActionReport> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmActionReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmActionReport> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmActionReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmActionReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmActionReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmActionReport> for IFsrmAction {
    fn from(value: IFsrmActionReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmActionReport> for IFsrmAction {
    fn from(value: &IFsrmActionReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmAction> for IFsrmActionReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmAction>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmAction> for &IFsrmActionReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmAction>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmActionReport> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmActionReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmActionReport> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmActionReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmActionReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmActionReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmActionReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actiontype: *mut FsrmActionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minutes: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, minutes: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reporttypes: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reporttypes: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailto: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailto: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmAutoApplyQuota(::windows::runtime::IUnknown);
impl IFsrmAutoApplyQuota {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), ::std::mem::transmute(newthreshold)).ok()
    }
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::runtime::Result<IFsrmAction> {
        let mut result__: <IFsrmAction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), ::std::mem::transmute(actiontype), &mut result__).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserSid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceTemplateName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), quotatemplatename.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExcludeFolders(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetExcludeFolders(&self, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(folders)).ok()
    }
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::runtime::Result<IFsrmDerivedObjectsResult> {
        let mut result__: <IFsrmDerivedObjectsResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(commitoptions), ::std::mem::transmute(applyoptions), &mut result__).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmAutoApplyQuota {
    type Vtable = IFsrmAutoApplyQuota_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4163786537, 27322, 18240, [191, 199, 199, 245, 143, 117, 251, 123]);
}
impl ::std::convert::From<IFsrmAutoApplyQuota> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmAutoApplyQuota> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmAutoApplyQuota> for IFsrmQuotaObject {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmAutoApplyQuota> for IFsrmQuotaObject {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaObject> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaObject> for &IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaObject>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmAutoApplyQuota> for IFsrmQuotaBase {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmAutoApplyQuota> for IFsrmQuotaBase {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaBase> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaBase>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaBase> for &IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaBase>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmAutoApplyQuota> for IFsrmObject {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmAutoApplyQuota> for IFsrmObject {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmAutoApplyQuota> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmAutoApplyQuota) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmAutoApplyQuota> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmAutoApplyQuota) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmAutoApplyQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmAutoApplyQuota_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotalimit: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotalimit: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotaflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotaflags: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, newthreshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, actions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, useraccount: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotatemplatename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matches: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotatemplatename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, folders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, folders: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmClassificationManager(::windows::runtime::IUnknown);
impl IFsrmClassificationManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassificationReportFormats(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetClassificationReportFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(formats)).ok()
    }
    pub unsafe fn Logging(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLogging(&self, logging: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(logging)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationReportMailTo(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClassificationReportMailTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn ClassificationReportEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetClassificationReportEnabled(&self, reportenabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(reportenabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationLastReportPathWithoutExtension(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationLastError(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn ClassificationRunningStatus(&self) -> ::windows::runtime::Result<FsrmReportRunningStatus> {
        let mut result__: <FsrmReportRunningStatus as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmReportRunningStatus>(result__)
    }
    pub unsafe fn EnumPropertyDefinitions(&self, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreatePropertyDefinition(&self) -> ::windows::runtime::Result<IFsrmPropertyDefinition> {
        let mut result__: <IFsrmPropertyDefinition as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyDefinition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertyname: Param0) -> ::windows::runtime::Result<IFsrmPropertyDefinition> {
        let mut result__: <IFsrmPropertyDefinition as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), propertyname.into_param().abi(), &mut result__).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    pub unsafe fn EnumRules(&self, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(ruletype), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreateRule(&self, ruletype: FsrmRuleType) -> ::windows::runtime::Result<IFsrmRule> {
        let mut result__: <IFsrmRule as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(ruletype), &mut result__).from_abi::<IFsrmRule>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRule<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, rulename: Param0, ruletype: FsrmRuleType) -> ::windows::runtime::Result<IFsrmRule> {
        let mut result__: <IFsrmRule as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), rulename.into_param().abi(), ::std::mem::transmute(ruletype), &mut result__).from_abi::<IFsrmRule>(result__)
    }
    pub unsafe fn EnumModuleDefinitions(&self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(moduletype), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreateModuleDefinition(&self, moduletype: FsrmPipelineModuleType) -> ::windows::runtime::Result<IFsrmPipelineModuleDefinition> {
        let mut result__: <IFsrmPipelineModuleDefinition as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(moduletype), &mut result__).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetModuleDefinition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, modulename: Param0, moduletype: FsrmPipelineModuleType) -> ::windows::runtime::Result<IFsrmPipelineModuleDefinition> {
        let mut result__: <IFsrmPipelineModuleDefinition as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), modulename.into_param().abi(), ::std::mem::transmute(moduletype), &mut result__).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunClassification<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: FsrmReportGenerationContext, reserved: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(context), reserved.into_param().abi()).ok()
    }
    pub unsafe fn WaitForClassificationCompletion(&self, waitseconds: i32) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(waitseconds), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn CancelClassification(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumFileProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, options: FsrmGetFilePropertyOptions) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), filepath.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, options: FsrmGetFilePropertyOptions) -> ::windows::runtime::Result<IFsrmProperty> {
        let mut result__: <IFsrmProperty as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), filepath.into_param().abi(), propertyname.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmProperty>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, propertyvalue: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), filepath.into_param().abi(), propertyname.into_param().abi(), propertyvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearFileProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, property: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), filepath.into_param().abi(), property.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmClassificationManager {
    type Vtable = IFsrmClassificationManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3537668570, 61073, 18592, [133, 216, 204, 114, 165, 111, 125, 4]);
}
impl ::std::convert::From<IFsrmClassificationManager> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmClassificationManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmClassificationManager> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmClassificationManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmClassificationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmClassificationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmClassificationManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmClassificationManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmClassificationManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmClassificationManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmClassificationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmClassificationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, logging: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, logging: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailto: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailto: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reportenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reportenabled: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastreportpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lasterror: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: FsrmEnumOptions, propertydefinitions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertydefinition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertydefinition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ruletype: FsrmRuleType, options: FsrmEnumOptions, rules: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ruletype: FsrmRuleType, rule: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rulename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ruletype: FsrmRuleType, rule: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions, moduledefinitions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modulename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: FsrmReportGenerationContext, reserved: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, waitseconds: i32, completed: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, fileproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, property: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmClassificationManager2(::windows::runtime::IUnknown);
impl IFsrmClassificationManager2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassificationReportFormats(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetClassificationReportFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(formats)).ok()
    }
    pub unsafe fn Logging(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLogging(&self, logging: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(logging)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationReportMailTo(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClassificationReportMailTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn ClassificationReportEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetClassificationReportEnabled(&self, reportenabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(reportenabled)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationLastReportPathWithoutExtension(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClassificationLastError(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn ClassificationRunningStatus(&self) -> ::windows::runtime::Result<FsrmReportRunningStatus> {
        let mut result__: <FsrmReportRunningStatus as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmReportRunningStatus>(result__)
    }
    pub unsafe fn EnumPropertyDefinitions(&self, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreatePropertyDefinition(&self) -> ::windows::runtime::Result<IFsrmPropertyDefinition> {
        let mut result__: <IFsrmPropertyDefinition as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyDefinition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertyname: Param0) -> ::windows::runtime::Result<IFsrmPropertyDefinition> {
        let mut result__: <IFsrmPropertyDefinition as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), propertyname.into_param().abi(), &mut result__).from_abi::<IFsrmPropertyDefinition>(result__)
    }
    pub unsafe fn EnumRules(&self, ruletype: FsrmRuleType, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(ruletype), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreateRule(&self, ruletype: FsrmRuleType) -> ::windows::runtime::Result<IFsrmRule> {
        let mut result__: <IFsrmRule as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(ruletype), &mut result__).from_abi::<IFsrmRule>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRule<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, rulename: Param0, ruletype: FsrmRuleType) -> ::windows::runtime::Result<IFsrmRule> {
        let mut result__: <IFsrmRule as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), rulename.into_param().abi(), ::std::mem::transmute(ruletype), &mut result__).from_abi::<IFsrmRule>(result__)
    }
    pub unsafe fn EnumModuleDefinitions(&self, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(moduletype), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreateModuleDefinition(&self, moduletype: FsrmPipelineModuleType) -> ::windows::runtime::Result<IFsrmPipelineModuleDefinition> {
        let mut result__: <IFsrmPipelineModuleDefinition as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(moduletype), &mut result__).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetModuleDefinition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, modulename: Param0, moduletype: FsrmPipelineModuleType) -> ::windows::runtime::Result<IFsrmPipelineModuleDefinition> {
        let mut result__: <IFsrmPipelineModuleDefinition as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), modulename.into_param().abi(), ::std::mem::transmute(moduletype), &mut result__).from_abi::<IFsrmPipelineModuleDefinition>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunClassification<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: FsrmReportGenerationContext, reserved: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(context), reserved.into_param().abi()).ok()
    }
    pub unsafe fn WaitForClassificationCompletion(&self, waitseconds: i32) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(waitseconds), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn CancelClassification(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumFileProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, options: FsrmGetFilePropertyOptions) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), filepath.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, options: FsrmGetFilePropertyOptions) -> ::windows::runtime::Result<IFsrmProperty> {
        let mut result__: <IFsrmProperty as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), filepath.into_param().abi(), propertyname.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmProperty>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, propertyname: Param1, propertyvalue: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), filepath.into_param().abi(), propertyname.into_param().abi(), propertyvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearFileProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, property: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), filepath.into_param().abi(), property.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ClassifyFiles(&self, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(filepaths), ::std::mem::transmute(propertynames), ::std::mem::transmute(propertyvalues), ::std::mem::transmute(options)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmClassificationManager2 {
    type Vtable = IFsrmClassificationManager2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(311753, 4734, 18277, [186, 7, 106, 49, 71, 188, 161, 18]);
}
impl ::std::convert::From<IFsrmClassificationManager2> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmClassificationManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmClassificationManager2> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmClassificationManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmClassificationManager2> for IFsrmClassificationManager {
    fn from(value: IFsrmClassificationManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmClassificationManager2> for IFsrmClassificationManager {
    fn from(value: &IFsrmClassificationManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmClassificationManager> for IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmClassificationManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmClassificationManager>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmClassificationManager> for &IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmClassificationManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmClassificationManager>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmClassificationManager2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmClassificationManager2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmClassificationManager2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmClassificationManager2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmClassificationManager2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationManager2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, logging: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, logging: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailto: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailto: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reportenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reportenabled: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastreportpath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lasterror: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: FsrmEnumOptions, propertydefinitions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertydefinition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertydefinition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ruletype: FsrmRuleType, options: FsrmEnumOptions, rules: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ruletype: FsrmRuleType, rule: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rulename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, ruletype: FsrmRuleType, rule: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduletype: FsrmPipelineModuleType, options: FsrmEnumOptions, moduledefinitions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modulename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, moduletype: FsrmPipelineModuleType, moduledefinition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: FsrmReportGenerationContext, reserved: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, waitseconds: i32, completed: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, fileproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmGetFilePropertyOptions, property: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepaths: *const super::super::System::Com::SAFEARRAY, propertynames: *const super::super::System::Com::SAFEARRAY, propertyvalues: *const super::super::System::Com::SAFEARRAY, options: FsrmGetFilePropertyOptions) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmClassificationRule(::windows::runtime::IUnknown);
impl IFsrmClassificationRule {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn RuleType(&self) -> ::windows::runtime::Result<FsrmRuleType> {
        let mut result__: <FsrmRuleType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmRuleType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleDefinitionName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModuleDefinitionName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, moduledefinitionname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), moduledefinitionname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(namespaceroots)).ok()
    }
    pub unsafe fn RuleFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRuleFlags(&self, ruleflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(ruleflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(parameters)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn LastModified(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn ExecutionOption(&self) -> ::windows::runtime::Result<FsrmExecutionOption> {
        let mut result__: <FsrmExecutionOption as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmExecutionOption>(result__)
    }
    pub unsafe fn SetExecutionOption(&self, executionoption: FsrmExecutionOption) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(executionoption)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PropertyAffected(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPropertyAffected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, property: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), property.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Value(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmClassificationRule {
    type Vtable = IFsrmClassificationRule_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2948616898, 21269, 17835, [132, 27, 198, 219, 14, 18, 1, 72]);
}
impl ::std::convert::From<IFsrmClassificationRule> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmClassificationRule) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmClassificationRule> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmClassificationRule) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmClassificationRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmClassificationRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmClassificationRule> for IFsrmRule {
    fn from(value: IFsrmClassificationRule) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmClassificationRule> for IFsrmRule {
    fn from(value: &IFsrmClassificationRule) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmRule> for IFsrmClassificationRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmRule> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmRule>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmRule> for &IFsrmClassificationRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmRule> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmRule>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmClassificationRule> for IFsrmObject {
    fn from(value: IFsrmClassificationRule) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmClassificationRule> for IFsrmObject {
    fn from(value: &IFsrmClassificationRule) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmClassificationRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmClassificationRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmClassificationRule> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmClassificationRule) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmClassificationRule> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmClassificationRule) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmClassificationRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmClassificationRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassificationRule_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ruletype: *mut FsrmRuleType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduledefinitionname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduledefinitionname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ruleflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ruleflags: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastmodified: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, executionoption: *mut FsrmExecutionOption) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, executionoption: FsrmExecutionOption) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, property: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, property: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmClassifierModuleDefinition(::windows::runtime::IUnknown);
impl IFsrmClassifierModuleDefinition {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleClsid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModuleClsid<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, moduleclsid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), moduleclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Company(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCompany<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, company: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), company.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Version(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVersion<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, version: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), version.into_param().abi()).ok()
    }
    pub unsafe fn ModuleType(&self) -> ::windows::runtime::Result<FsrmPipelineModuleType> {
        let mut result__: <FsrmPipelineModuleType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmPipelineModuleType>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    pub unsafe fn NeedsFileContent(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetNeedsFileContent(&self, needsfilecontent: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(needsfilecontent)).ok()
    }
    pub unsafe fn Account(&self) -> ::windows::runtime::Result<FsrmAccountType> {
        let mut result__: <FsrmAccountType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmAccountType>(result__)
    }
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(retrievalaccount)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedExtensions(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(supportedextensions)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(parameters)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PropertiesAffected(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPropertiesAffected(&self, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(propertiesaffected)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PropertiesUsed(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPropertiesUsed(&self, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(propertiesused)).ok()
    }
    pub unsafe fn NeedsExplicitValue(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetNeedsExplicitValue(&self, needsexplicitvalue: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), ::std::mem::transmute(needsexplicitvalue)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmClassifierModuleDefinition {
    type Vtable = IFsrmClassifierModuleDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3140938278, 25368, 19340, [133, 146, 247, 45, 214, 2, 231, 165]);
}
impl ::std::convert::From<IFsrmClassifierModuleDefinition> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmClassifierModuleDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmClassifierModuleDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmClassifierModuleDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmClassifierModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: IFsrmClassifierModuleDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmClassifierModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: &IFsrmClassifierModuleDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmPipelineModuleDefinition> for IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmPipelineModuleDefinition>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmPipelineModuleDefinition> for &IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmPipelineModuleDefinition>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmClassifierModuleDefinition> for IFsrmObject {
    fn from(value: IFsrmClassifierModuleDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmClassifierModuleDefinition> for IFsrmObject {
    fn from(value: &IFsrmClassifierModuleDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmClassifierModuleDefinition> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmClassifierModuleDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmClassifierModuleDefinition> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmClassifierModuleDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmClassifierModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassifierModuleDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduleclsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduleclsid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, company: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, company: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, version: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, version: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduletype: *mut FsrmPipelineModuleType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, needsfilecontent: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, needsfilecontent: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, retrievalaccount: *mut FsrmAccountType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, retrievalaccount: FsrmAccountType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedextensions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertiesaffected: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertiesaffected: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertiesused: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertiesused: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, needsexplicitvalue: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, needsexplicitvalue: i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmClassifierModuleImplementation(::windows::runtime::IUnknown);
impl IFsrmClassifierModuleImplementation {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn OnLoad<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmPipelineModuleDefinition>>(&self, moduledefinition: Param0) -> ::windows::runtime::Result<IFsrmPipelineModuleConnector> {
        let mut result__: <IFsrmPipelineModuleConnector as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), moduledefinition.into_param().abi(), &mut result__).from_abi::<IFsrmPipelineModuleConnector>(result__)
    }
    pub unsafe fn OnUnload(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn LastModified(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn UseRulesAndDefinitions<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmCollection>, Param1: ::windows::runtime::IntoParam<'a, IFsrmCollection>>(&self, rules: Param0, propertydefinitions: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), rules.into_param().abi(), propertydefinitions.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnBeginFile<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmPropertyBag>>(&self, propertybag: Param0, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), propertybag.into_param().abi(), ::std::mem::transmute(arrayruleids)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesPropertyValueApply<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, property: Param0, value: Param1, applyvalue: *mut i16, idrule: Param3, idpropdef: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), property.into_param().abi(), value.into_param().abi(), ::std::mem::transmute(applyvalue), idrule.into_param().abi(), idpropdef.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValueToApply<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, property: Param0, value: *mut super::super::Foundation::BSTR, idrule: Param2, idpropdef: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), property.into_param().abi(), ::std::mem::transmute(value), idrule.into_param().abi(), idpropdef.into_param().abi()).ok()
    }
    pub unsafe fn OnEndFile(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmClassifierModuleImplementation {
    type Vtable = IFsrmClassifierModuleImplementation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1284935622, 28379, 16465, [156, 24, 115, 183, 41, 26, 225, 6]);
}
impl ::std::convert::From<IFsrmClassifierModuleImplementation> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmClassifierModuleImplementation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmClassifierModuleImplementation> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmClassifierModuleImplementation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmClassifierModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: IFsrmClassifierModuleImplementation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmClassifierModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: &IFsrmClassifierModuleImplementation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmPipelineModuleImplementation> for IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmPipelineModuleImplementation>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmPipelineModuleImplementation> for &IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmPipelineModuleImplementation>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmClassifierModuleImplementation> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmClassifierModuleImplementation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmClassifierModuleImplementation> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmClassifierModuleImplementation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmClassifierModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmClassifierModuleImplementation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduledefinition: ::windows::runtime::RawPtr, moduleconnector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastmodified: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rules: ::windows::runtime::RawPtr, propertydefinitions: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertybag: ::windows::runtime::RawPtr, arrayruleids: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, property: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, applyvalue: *mut i16, idrule: ::windows::runtime::GUID, idpropdef: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, property: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, idrule: ::windows::runtime::GUID, idpropdef: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmCollection(::windows::runtime::IUnknown);
impl IFsrmCollection {
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::runtime::Result<FsrmCollectionState> {
        let mut result__: <FsrmCollectionState as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmCollectionState>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(waitseconds), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn GetById<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, id: Param0) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), id.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmCollection {
    type Vtable = IFsrmCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4151295803, 36317, 19266, [176, 90, 203, 28, 63, 241, 254, 232]);
}
impl ::std::convert::From<IFsrmCollection> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unknown: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, item: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, state: *mut FsrmCollectionState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, waitseconds: i32, completed: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::windows::runtime::GUID, entry: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmCommittableCollection(::windows::runtime::IUnknown);
impl IFsrmCommittableCollection {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::runtime::Result<FsrmCollectionState> {
        let mut result__: <FsrmCollectionState as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmCollectionState>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(waitseconds), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn GetById<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, id: Param0) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), id.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, item: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), item.into_param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(index)).ok()
    }
    pub unsafe fn RemoveById<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, id: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), id.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IFsrmMutableCollection> {
        let mut result__: <IFsrmMutableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn Commit(&self, options: FsrmCommitOptions) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmCommittableCollection {
    type Vtable = IFsrmCommittableCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2531177397, 35729, 18986, [157, 147, 128, 163, 93, 138, 168, 71]);
}
impl ::std::convert::From<IFsrmCommittableCollection> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmCommittableCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmCommittableCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmCommittableCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmCommittableCollection> for IFsrmMutableCollection {
    fn from(value: IFsrmCommittableCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmCommittableCollection> for IFsrmMutableCollection {
    fn from(value: &IFsrmCommittableCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmMutableCollection> for IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmMutableCollection> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmMutableCollection>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmMutableCollection> for &IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmMutableCollection> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmMutableCollection>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmCommittableCollection> for IFsrmCollection {
    fn from(value: IFsrmCommittableCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmCommittableCollection> for IFsrmCollection {
    fn from(value: &IFsrmCommittableCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmCollection> for IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmCollection> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmCollection>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmCollection> for &IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmCollection> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmCollection>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmCommittableCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmCommittableCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmCommittableCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmCommittableCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmCommittableCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmCommittableCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unknown: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, item: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, state: *mut FsrmCollectionState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, waitseconds: i32, completed: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::windows::runtime::GUID, entry: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, item: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, collection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: FsrmCommitOptions, results: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmDerivedObjectsResult(::windows::runtime::IUnknown);
impl IFsrmDerivedObjectsResult {
    pub unsafe fn DerivedObjects(&self) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn Results(&self) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmDerivedObjectsResult {
    type Vtable = IFsrmDerivedObjectsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(959588909, 14574, 19725, [128, 149, 66, 26, 128, 132, 154, 130]);
}
impl ::std::convert::From<IFsrmDerivedObjectsResult> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmDerivedObjectsResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmDerivedObjectsResult> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmDerivedObjectsResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmDerivedObjectsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmDerivedObjectsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmDerivedObjectsResult> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmDerivedObjectsResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmDerivedObjectsResult> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmDerivedObjectsResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmDerivedObjectsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmDerivedObjectsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmDerivedObjectsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, derivedobjects: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, results: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmExportImport(::windows::runtime::IUnknown);
impl IFsrmExportImport {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ExportFileGroups<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), filepath.into_param().abi(), ::std::mem::transmute(filegroupnamessafearray), remotehost.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ImportFileGroups<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, filegroupnamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), filepath.into_param().abi(), ::std::mem::transmute(filegroupnamessafearray), remotehost.into_param().abi(), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ExportFileScreenTemplates<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), filepath.into_param().abi(), ::std::mem::transmute(templatenamessafearray), remotehost.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ImportFileScreenTemplates<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), filepath.into_param().abi(), ::std::mem::transmute(templatenamessafearray), remotehost.into_param().abi(), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ExportQuotaTemplates<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), filepath.into_param().abi(), ::std::mem::transmute(templatenamessafearray), remotehost.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ImportQuotaTemplates<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filepath: Param0, templatenamessafearray: *const super::super::System::Com::VARIANT, remotehost: Param2) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), filepath.into_param().abi(), ::std::mem::transmute(templatenamessafearray), remotehost.into_param().abi(), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmExportImport {
    type Vtable = IFsrmExportImport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4023061169, 5828, 19065, [129, 44, 114, 86, 20, 195, 48, 107]);
}
impl ::std::convert::From<IFsrmExportImport> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmExportImport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmExportImport> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmExportImport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmExportImport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmExportImport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmExportImport> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmExportImport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmExportImport> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmExportImport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmExportImport {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmExportImport {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmExportImport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamessafearray: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, remotehost: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamessafearray: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, remotehost: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroups: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, remotehost: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, remotehost: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, templates: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, remotehost: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, templatenamessafearray: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, remotehost: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, templates: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmFileCondition(::windows::runtime::IUnknown);
impl IFsrmFileCondition {
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<FsrmFileConditionType> {
        let mut result__: <FsrmFileConditionType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmFileConditionType>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmFileCondition {
    type Vtable = IFsrmFileCondition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1885884412, 26906, 18970, [185, 34, 151, 117, 46, 19, 140, 193]);
}
impl ::std::convert::From<IFsrmFileCondition> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmFileCondition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileCondition> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmFileCondition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmFileCondition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmFileCondition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmFileCondition> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmFileCondition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmFileCondition> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmFileCondition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmFileCondition {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmFileCondition {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileCondition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut FsrmFileConditionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmFileConditionProperty(::windows::runtime::IUnknown);
impl IFsrmFileConditionProperty {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<FsrmFileConditionType> {
        let mut result__: <FsrmFileConditionType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmFileConditionType>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PropertyName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPropertyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows::runtime::Result<FsrmFileSystemPropertyId> {
        let mut result__: <FsrmFileSystemPropertyId as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmFileSystemPropertyId>(result__)
    }
    pub unsafe fn SetPropertyId(&self, newval: FsrmFileSystemPropertyId) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(newval)).ok()
    }
    pub unsafe fn Operator(&self) -> ::windows::runtime::Result<FsrmPropertyConditionType> {
        let mut result__: <FsrmPropertyConditionType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmPropertyConditionType>(result__)
    }
    pub unsafe fn SetOperator(&self, newval: FsrmPropertyConditionType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(newval)).ok()
    }
    pub unsafe fn ValueType(&self) -> ::windows::runtime::Result<FsrmPropertyValueType> {
        let mut result__: <FsrmPropertyValueType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmPropertyValueType>(result__)
    }
    pub unsafe fn SetValueType(&self, newval: FsrmPropertyValueType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(newval)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Value(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmFileConditionProperty {
    type Vtable = IFsrmFileConditionProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2173855605, 47489, 17529, [152, 143, 218, 23, 29, 98, 115, 96]);
}
impl ::std::convert::From<IFsrmFileConditionProperty> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmFileConditionProperty) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileConditionProperty> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmFileConditionProperty) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmFileConditionProperty> for IFsrmFileCondition {
    fn from(value: IFsrmFileConditionProperty) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileConditionProperty> for IFsrmFileCondition {
    fn from(value: &IFsrmFileConditionProperty) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmFileCondition> for IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmFileCondition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmFileCondition>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmFileCondition> for &IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmFileCondition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmFileCondition>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmFileConditionProperty> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmFileConditionProperty) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmFileConditionProperty> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmFileConditionProperty) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmFileConditionProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileConditionProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut FsrmFileConditionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut FsrmFileSystemPropertyId) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: FsrmFileSystemPropertyId) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut FsrmPropertyConditionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: FsrmPropertyConditionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut FsrmPropertyValueType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: FsrmPropertyValueType) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmFileGroup(::windows::runtime::IUnknown);
impl IFsrmFileGroup {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Members(&self) -> ::windows::runtime::Result<IFsrmMutableCollection> {
        let mut result__: <IFsrmMutableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetMembers<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmMutableCollection>>(&self, members: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), members.into_param().abi()).ok()
    }
    pub unsafe fn NonMembers(&self) -> ::windows::runtime::Result<IFsrmMutableCollection> {
        let mut result__: <IFsrmMutableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetNonMembers<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmMutableCollection>>(&self, nonmembers: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), nonmembers.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmFileGroup {
    type Vtable = IFsrmFileGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2379237641, 3636, 19797, [175, 170, 137, 225, 241, 161, 187, 185]);
}
impl ::std::convert::From<IFsrmFileGroup> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmFileGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileGroup> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmFileGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmFileGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmFileGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmFileGroup> for IFsrmObject {
    fn from(value: IFsrmFileGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileGroup> for IFsrmObject {
    fn from(value: &IFsrmFileGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmFileGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmFileGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmFileGroup> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmFileGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmFileGroup> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmFileGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmFileGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmFileGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, members: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, members: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nonmembers: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nonmembers: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmFileGroupImported(::windows::runtime::IUnknown);
impl IFsrmFileGroupImported {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Members(&self) -> ::windows::runtime::Result<IFsrmMutableCollection> {
        let mut result__: <IFsrmMutableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetMembers<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmMutableCollection>>(&self, members: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), members.into_param().abi()).ok()
    }
    pub unsafe fn NonMembers(&self) -> ::windows::runtime::Result<IFsrmMutableCollection> {
        let mut result__: <IFsrmMutableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetNonMembers<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmMutableCollection>>(&self, nonmembers: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), nonmembers.into_param().abi()).ok()
    }
    pub unsafe fn OverwriteOnCommit(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetOverwriteOnCommit(&self, overwrite: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(overwrite)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmFileGroupImported {
    type Vtable = IFsrmFileGroupImported_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2908090635, 24337, 19431, [148, 239, 217, 238, 46, 71, 13, 237]);
}
impl ::std::convert::From<IFsrmFileGroupImported> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmFileGroupImported) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileGroupImported> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmFileGroupImported) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmFileGroupImported> for IFsrmFileGroup {
    fn from(value: IFsrmFileGroupImported) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileGroupImported> for IFsrmFileGroup {
    fn from(value: &IFsrmFileGroupImported) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmFileGroup> for IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmFileGroup> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmFileGroup>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmFileGroup> for &IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmFileGroup> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmFileGroup>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmFileGroupImported> for IFsrmObject {
    fn from(value: IFsrmFileGroupImported) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileGroupImported> for IFsrmObject {
    fn from(value: &IFsrmFileGroupImported) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmFileGroupImported> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmFileGroupImported) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmFileGroupImported> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmFileGroupImported) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmFileGroupImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroupImported_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, members: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, members: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nonmembers: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nonmembers: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, overwrite: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, overwrite: i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmFileGroupManager(::windows::runtime::IUnknown);
impl IFsrmFileGroupManager {
    pub unsafe fn CreateFileGroup(&self) -> ::windows::runtime::Result<IFsrmFileGroup> {
        let mut result__: <IFsrmFileGroup as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmFileGroup>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<IFsrmFileGroup> {
        let mut result__: <IFsrmFileGroup as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IFsrmFileGroup>(result__)
    }
    pub unsafe fn EnumFileGroups(&self, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ExportFileGroups(&self, filegroupnamesarray: *const super::super::System::Com::VARIANT) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(filegroupnamesarray), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ImportFileGroups<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serializedfilegroups: Param0, filegroupnamesarray: *const super::super::System::Com::VARIANT) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), serializedfilegroups.into_param().abi(), ::std::mem::transmute(filegroupnamesarray), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmFileGroupManager {
    type Vtable = IFsrmFileGroupManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1114011605, 396, 18524, [138, 81, 32, 184, 109, 0, 189, 196]);
}
impl ::std::convert::From<IFsrmFileGroupManager> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmFileGroupManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileGroupManager> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmFileGroupManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmFileGroupManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmFileGroupManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmFileGroupManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmFileGroupManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmFileGroupManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmFileGroupManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmFileGroupManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmFileGroupManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileGroupManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filegroup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: FsrmEnumOptions, filegroups: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filegroupnamesarray: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, serializedfilegroups: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, serializedfilegroups: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, filegroupnamesarray: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, filegroups: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmFileManagementJob(::windows::runtime::IUnknown);
impl IFsrmFileManagementJob {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(namespaceroots)).ok()
    }
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    pub unsafe fn OperationType(&self) -> ::windows::runtime::Result<FsrmFileManagementType> {
        let mut result__: <FsrmFileManagementType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmFileManagementType>(result__)
    }
    pub unsafe fn SetOperationType(&self, operationtype: FsrmFileManagementType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(operationtype)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ExpirationDirectory(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExpirationDirectory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, expirationdirectory: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), expirationdirectory.into_param().abi()).ok()
    }
    pub unsafe fn CustomAction(&self) -> ::windows::runtime::Result<IFsrmActionCommand> {
        let mut result__: <IFsrmActionCommand as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmActionCommand>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Notifications(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn Logging(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLogging(&self, loggingflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(loggingflags)).ok()
    }
    pub unsafe fn ReportEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetReportEnabled(&self, reportenabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(reportenabled)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Formats(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(formats)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailTo(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn DaysSinceFileCreated(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDaysSinceFileCreated(&self, dayssincecreation: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(dayssincecreation)).ok()
    }
    pub unsafe fn DaysSinceFileLastAccessed(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDaysSinceFileLastAccessed(&self, dayssinceaccess: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), ::std::mem::transmute(dayssinceaccess)).ok()
    }
    pub unsafe fn DaysSinceFileLastModified(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDaysSinceFileLastModified(&self, dayssincemodify: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), ::std::mem::transmute(dayssincemodify)).ok()
    }
    pub unsafe fn PropertyConditions(&self) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn FromDate(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn SetFromDate(&self, fromdate: f64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), ::std::mem::transmute(fromdate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Task(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTask<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), taskname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), ::std::mem::transmute(parameters)).ok()
    }
    pub unsafe fn RunningStatus(&self) -> ::windows::runtime::Result<FsrmReportRunningStatus> {
        let mut result__: <FsrmReportRunningStatus as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmReportRunningStatus>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastError(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastReportPathWithoutExtension(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn LastRun(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FileNamePattern(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileNamePattern<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filenamepattern: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::std::mem::transmute_copy(self), filenamepattern.into_param().abi()).ok()
    }
    pub unsafe fn Run(&self, context: FsrmReportGenerationContext) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).51)(::std::mem::transmute_copy(self), ::std::mem::transmute(context)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).52)(::std::mem::transmute_copy(self), ::std::mem::transmute(waitseconds), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn AddNotification(&self, days: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::std::mem::transmute_copy(self), ::std::mem::transmute(days)).ok()
    }
    pub unsafe fn DeleteNotification(&self, days: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self), ::std::mem::transmute(days)).ok()
    }
    pub unsafe fn ModifyNotification(&self, days: i32, newdays: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::std::mem::transmute_copy(self), ::std::mem::transmute(days), ::std::mem::transmute(newdays)).ok()
    }
    pub unsafe fn CreateNotificationAction(&self, days: i32, actiontype: FsrmActionType) -> ::windows::runtime::Result<IFsrmAction> {
        let mut result__: <IFsrmAction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).57)(::std::mem::transmute_copy(self), ::std::mem::transmute(days), ::std::mem::transmute(actiontype), &mut result__).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumNotificationActions(&self, days: i32) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).58)(::std::mem::transmute_copy(self), ::std::mem::transmute(days), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePropertyCondition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<IFsrmPropertyCondition> {
        let mut result__: <IFsrmPropertyCondition as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).59)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IFsrmPropertyCondition>(result__)
    }
    pub unsafe fn CreateCustomAction(&self) -> ::windows::runtime::Result<IFsrmActionCommand> {
        let mut result__: <IFsrmActionCommand as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmActionCommand>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmFileManagementJob {
    type Vtable = IFsrmFileManagementJob_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(124807294, 40758, 19823, [135, 120, 89, 157, 24, 132, 97, 201]);
}
impl ::std::convert::From<IFsrmFileManagementJob> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmFileManagementJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileManagementJob> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmFileManagementJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmFileManagementJob> for IFsrmObject {
    fn from(value: IFsrmFileManagementJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileManagementJob> for IFsrmObject {
    fn from(value: &IFsrmFileManagementJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmFileManagementJob> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmFileManagementJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmFileManagementJob> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmFileManagementJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmFileManagementJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileManagementJob_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operationtype: *mut FsrmFileManagementType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operationtype: FsrmFileManagementType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, expirationdirectory: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, expirationdirectory: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, action: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, notifications: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loggingflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loggingflags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reportenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reportenabled: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailto: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailto: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dayssincecreation: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dayssincecreation: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dayssinceaccess: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dayssinceaccess: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dayssincemodify: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dayssincemodify: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyconditions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fromdate: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fromdate: f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, taskname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, taskname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lasterror: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastrun: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filenamepattern: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filenamepattern: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: FsrmReportGenerationContext) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, waitseconds: i32, completed: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, days: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, days: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, days: i32, newdays: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, days: i32, actiontype: FsrmActionType, action: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, days: i32, actions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertycondition: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customaction: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmFileManagementJobManager(::windows::runtime::IUnknown);
impl IFsrmFileManagementJobManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn EnumFileManagementJobs(&self, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreateFileManagementJob(&self) -> ::windows::runtime::Result<IFsrmFileManagementJob> {
        let mut result__: <IFsrmFileManagementJob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmFileManagementJob>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileManagementJob<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<IFsrmFileManagementJob> {
        let mut result__: <IFsrmFileManagementJob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IFsrmFileManagementJob>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmFileManagementJobManager {
    type Vtable = IFsrmFileManagementJobManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3996262091, 55646, 18665, [144, 124, 199, 104, 90, 1, 50, 53]);
}
impl ::std::convert::From<IFsrmFileManagementJobManager> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmFileManagementJobManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileManagementJobManager> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmFileManagementJobManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmFileManagementJobManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmFileManagementJobManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmFileManagementJobManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmFileManagementJobManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmFileManagementJobManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmFileManagementJobManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmFileManagementJobManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmFileManagementJobManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileManagementJobManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: FsrmEnumOptions, filemanagementjobs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filemanagementjob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, filemanagementjob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmFileScreen(::windows::runtime::IUnknown);
impl IFsrmFileScreen {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::runtime::Result<IFsrmMutableCollection> {
        let mut result__: <IFsrmMutableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), blocklist.into_param().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(filescreenflags)).ok()
    }
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::runtime::Result<IFsrmAction> {
        let mut result__: <IFsrmAction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(actiontype), &mut result__).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumActions(&self) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceTemplateName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserSid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filescreentemplatename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), filescreentemplatename.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmFileScreen {
    type Vtable = IFsrmFileScreen_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1600333267, 52872, 18227, [132, 193, 45, 106, 239, 197, 234, 7]);
}
impl ::std::convert::From<IFsrmFileScreen> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmFileScreen) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreen> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmFileScreen) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmFileScreen {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmFileScreen {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmFileScreen> for IFsrmFileScreenBase {
    fn from(value: IFsrmFileScreen) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreen> for IFsrmFileScreenBase {
    fn from(value: &IFsrmFileScreen) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmFileScreenBase> for IFsrmFileScreen {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmFileScreenBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmFileScreenBase>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmFileScreenBase> for &IFsrmFileScreen {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmFileScreenBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmFileScreenBase>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmFileScreen> for IFsrmObject {
    fn from(value: IFsrmFileScreen) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreen> for IFsrmObject {
    fn from(value: &IFsrmFileScreen) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmFileScreen {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmFileScreen {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmFileScreen> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmFileScreen) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmFileScreen> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmFileScreen) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmFileScreen {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmFileScreen {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreen_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blocklist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blocklist: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filescreenflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filescreenflags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actiontype: FsrmActionType, action: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filescreentemplatename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matches: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, useraccount: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filescreentemplatename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmFileScreenBase(::windows::runtime::IUnknown);
impl IFsrmFileScreenBase {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::runtime::Result<IFsrmMutableCollection> {
        let mut result__: <IFsrmMutableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), blocklist.into_param().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(filescreenflags)).ok()
    }
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::runtime::Result<IFsrmAction> {
        let mut result__: <IFsrmAction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(actiontype), &mut result__).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumActions(&self) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmFileScreenBase {
    type Vtable = IFsrmFileScreenBase_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4083383936, 23330, 18987, [166, 55, 187, 182, 66, 180, 28, 252]);
}
impl ::std::convert::From<IFsrmFileScreenBase> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmFileScreenBase) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreenBase> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmFileScreenBase) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmFileScreenBase> for IFsrmObject {
    fn from(value: IFsrmFileScreenBase) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreenBase> for IFsrmObject {
    fn from(value: &IFsrmFileScreenBase) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmFileScreenBase> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmFileScreenBase) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmFileScreenBase> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmFileScreenBase) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmFileScreenBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenBase_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blocklist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blocklist: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filescreenflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filescreenflags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actiontype: FsrmActionType, action: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmFileScreenException(::windows::runtime::IUnknown);
impl IFsrmFileScreenException {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn AllowedFileGroups(&self) -> ::windows::runtime::Result<IFsrmMutableCollection> {
        let mut result__: <IFsrmMutableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetAllowedFileGroups<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmMutableCollection>>(&self, allowlist: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), allowlist.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmFileScreenException {
    type Vtable = IFsrmFileScreenException_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3202862594, 57207, 17685, [147, 137, 120, 240, 28, 90, 252, 26]);
}
impl ::std::convert::From<IFsrmFileScreenException> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmFileScreenException) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreenException> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmFileScreenException) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmFileScreenException {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmFileScreenException {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmFileScreenException> for IFsrmObject {
    fn from(value: IFsrmFileScreenException) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreenException> for IFsrmObject {
    fn from(value: &IFsrmFileScreenException) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmFileScreenException {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmFileScreenException {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmFileScreenException> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmFileScreenException) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmFileScreenException> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmFileScreenException) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmFileScreenException {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmFileScreenException {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenException_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allowlist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, allowlist: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmFileScreenManager(::windows::runtime::IUnknown);
impl IFsrmFileScreenManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFileScreen<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<IFsrmFileScreen> {
        let mut result__: <IFsrmFileScreen as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IFsrmFileScreen>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileScreen<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<IFsrmFileScreen> {
        let mut result__: <IFsrmFileScreen as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IFsrmFileScreen>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumFileScreens<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), path.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateFileScreenException<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<IFsrmFileScreenException> {
        let mut result__: <IFsrmFileScreenException as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IFsrmFileScreenException>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileScreenException<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<IFsrmFileScreenException> {
        let mut result__: <IFsrmFileScreenException as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IFsrmFileScreenException>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumFileScreenExceptions<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), path.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
    pub unsafe fn CreateFileScreenCollection(&self) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmFileScreenManager {
    type Vtable = IFsrmFileScreenManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4283408462, 23188, 19418, [163, 160, 213, 180, 211, 197, 46, 186]);
}
impl ::std::convert::From<IFsrmFileScreenManager> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmFileScreenManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreenManager> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmFileScreenManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmFileScreenManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmFileScreenManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmFileScreenManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmFileScreenManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmFileScreenManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmFileScreenManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmFileScreenManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmFileScreenManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreen: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreen: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, filescreens: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreenexception: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreenexception: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, filescreenexceptions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, collection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmFileScreenTemplate(::windows::runtime::IUnknown);
impl IFsrmFileScreenTemplate {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::runtime::Result<IFsrmMutableCollection> {
        let mut result__: <IFsrmMutableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), blocklist.into_param().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(filescreenflags)).ok()
    }
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::runtime::Result<IFsrmAction> {
        let mut result__: <IFsrmAction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(actiontype), &mut result__).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumActions(&self) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTemplate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filescreentemplatename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), filescreentemplatename.into_param().abi()).ok()
    }
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::runtime::Result<IFsrmDerivedObjectsResult> {
        let mut result__: <IFsrmDerivedObjectsResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(commitoptions), ::std::mem::transmute(applyoptions), &mut result__).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmFileScreenTemplate {
    type Vtable = IFsrmFileScreenTemplate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(542895096, 56723, 17706, [149, 166, 50, 181, 102, 179, 88, 40]);
}
impl ::std::convert::From<IFsrmFileScreenTemplate> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmFileScreenTemplate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreenTemplate> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmFileScreenTemplate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmFileScreenTemplate> for IFsrmFileScreenBase {
    fn from(value: IFsrmFileScreenTemplate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreenTemplate> for IFsrmFileScreenBase {
    fn from(value: &IFsrmFileScreenTemplate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmFileScreenBase> for IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmFileScreenBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmFileScreenBase>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmFileScreenBase> for &IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmFileScreenBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmFileScreenBase>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmFileScreenTemplate> for IFsrmObject {
    fn from(value: IFsrmFileScreenTemplate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreenTemplate> for IFsrmObject {
    fn from(value: &IFsrmFileScreenTemplate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmFileScreenTemplate> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmFileScreenTemplate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmFileScreenTemplate> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmFileScreenTemplate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmFileScreenTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blocklist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blocklist: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filescreenflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filescreenflags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actiontype: FsrmActionType, action: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filescreentemplatename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmFileScreenTemplateImported(::windows::runtime::IUnknown);
impl IFsrmFileScreenTemplateImported {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn BlockedFileGroups(&self) -> ::windows::runtime::Result<IFsrmMutableCollection> {
        let mut result__: <IFsrmMutableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmMutableCollection>(result__)
    }
    pub unsafe fn SetBlockedFileGroups<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmMutableCollection>>(&self, blocklist: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), blocklist.into_param().abi()).ok()
    }
    pub unsafe fn FileScreenFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetFileScreenFlags(&self, filescreenflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(filescreenflags)).ok()
    }
    pub unsafe fn CreateAction(&self, actiontype: FsrmActionType) -> ::windows::runtime::Result<IFsrmAction> {
        let mut result__: <IFsrmAction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(actiontype), &mut result__).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumActions(&self) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTemplate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, filescreentemplatename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), filescreentemplatename.into_param().abi()).ok()
    }
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::runtime::Result<IFsrmDerivedObjectsResult> {
        let mut result__: <IFsrmDerivedObjectsResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(commitoptions), ::std::mem::transmute(applyoptions), &mut result__).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
    pub unsafe fn OverwriteOnCommit(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetOverwriteOnCommit(&self, overwrite: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(overwrite)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmFileScreenTemplateImported {
    type Vtable = IFsrmFileScreenTemplateImported_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3774939993, 15965, 20173, [159, 228, 239, 72, 98, 47, 223, 48]);
}
impl ::std::convert::From<IFsrmFileScreenTemplateImported> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreenTemplateImported> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmFileScreenTemplateImported> for IFsrmFileScreenTemplate {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreenTemplateImported> for IFsrmFileScreenTemplate {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmFileScreenTemplate> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmFileScreenTemplate> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmFileScreenTemplate>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmFileScreenTemplate> for &IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmFileScreenTemplate> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmFileScreenTemplate>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmFileScreenTemplateImported> for IFsrmFileScreenBase {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreenTemplateImported> for IFsrmFileScreenBase {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmFileScreenBase> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmFileScreenBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmFileScreenBase>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmFileScreenBase> for &IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmFileScreenBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmFileScreenBase>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmFileScreenTemplateImported> for IFsrmObject {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreenTemplateImported> for IFsrmObject {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmFileScreenTemplateImported> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmFileScreenTemplateImported) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmFileScreenTemplateImported> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmFileScreenTemplateImported) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmFileScreenTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplateImported_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blocklist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, blocklist: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filescreenflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filescreenflags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actiontype: FsrmActionType, action: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filescreentemplatename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, overwrite: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, overwrite: i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmFileScreenTemplateManager(::windows::runtime::IUnknown);
impl IFsrmFileScreenTemplateManager {
    pub unsafe fn CreateTemplate(&self) -> ::windows::runtime::Result<IFsrmFileScreenTemplate> {
        let mut result__: <IFsrmFileScreenTemplate as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmFileScreenTemplate>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTemplate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<IFsrmFileScreenTemplate> {
        let mut result__: <IFsrmFileScreenTemplate as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IFsrmFileScreenTemplate>(result__)
    }
    pub unsafe fn EnumTemplates(&self, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ExportTemplates(&self, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(filescreentemplatenamesarray), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ImportTemplates<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serializedfilescreentemplates: Param0, filescreentemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), serializedfilescreentemplates.into_param().abi(), ::std::mem::transmute(filescreentemplatenamesarray), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmFileScreenTemplateManager {
    type Vtable = IFsrmFileScreenTemplateManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3487788218, 6473, 20084, [161, 79, 241, 213, 128, 206, 175, 19]);
}
impl ::std::convert::From<IFsrmFileScreenTemplateManager> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmFileScreenTemplateManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmFileScreenTemplateManager> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmFileScreenTemplateManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmFileScreenTemplateManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmFileScreenTemplateManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmFileScreenTemplateManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmFileScreenTemplateManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmFileScreenTemplateManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmFileScreenTemplateManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmFileScreenTemplateManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmFileScreenTemplateManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmFileScreenTemplateManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filescreentemplate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreentemplate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: FsrmEnumOptions, filescreentemplates: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filescreentemplatenamesarray: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, serializedfilescreentemplates: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, serializedfilescreentemplates: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, filescreentemplatenamesarray: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, filescreentemplates: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmMutableCollection(::windows::runtime::IUnknown);
impl IFsrmMutableCollection {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn State(&self) -> ::windows::runtime::Result<FsrmCollectionState> {
        let mut result__: <FsrmCollectionState as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmCollectionState>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(waitseconds), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn GetById<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, id: Param0) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), id.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, item: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), item.into_param().abi()).ok()
    }
    pub unsafe fn Remove(&self, index: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(index)).ok()
    }
    pub unsafe fn RemoveById<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, id: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), id.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IFsrmMutableCollection> {
        let mut result__: <IFsrmMutableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmMutableCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmMutableCollection {
    type Vtable = IFsrmMutableCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(464918456, 14470, 18908, [175, 130, 166, 201, 15, 163, 93, 218]);
}
impl ::std::convert::From<IFsrmMutableCollection> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmMutableCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmMutableCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmMutableCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmMutableCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmMutableCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmMutableCollection> for IFsrmCollection {
    fn from(value: IFsrmMutableCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmMutableCollection> for IFsrmCollection {
    fn from(value: &IFsrmMutableCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmCollection> for IFsrmMutableCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmCollection> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmCollection>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmCollection> for &IFsrmMutableCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmCollection> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmCollection>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmMutableCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmMutableCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmMutableCollection> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmMutableCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmMutableCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmMutableCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmMutableCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unknown: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, item: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, state: *mut FsrmCollectionState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, waitseconds: i32, completed: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::windows::runtime::GUID, entry: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, item: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, collection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmObject(::windows::runtime::IUnknown);
impl IFsrmObject {
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmObject {
    type Vtable = IFsrmObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(582807443, 19007, 16771, [137, 249, 47, 139, 138, 98, 138, 238]);
}
impl ::std::convert::From<IFsrmObject> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmObject> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmObject> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmObject> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmPathMapper(::windows::runtime::IUnknown);
impl IFsrmPathMapper {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetSharePathsForLocalPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, localpath: Param0) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), localpath.into_param().abi(), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmPathMapper {
    type Vtable = IFsrmPathMapper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1867366399, 26912, 18465, [166, 195, 183, 233, 76, 31, 214, 12]);
}
impl ::std::convert::From<IFsrmPathMapper> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmPathMapper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmPathMapper> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmPathMapper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmPathMapper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmPathMapper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmPathMapper> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmPathMapper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmPathMapper> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmPathMapper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmPathMapper {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmPathMapper {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPathMapper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, localpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, sharepaths: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmPipelineModuleConnector(::windows::runtime::IUnknown);
impl IFsrmPipelineModuleConnector {
    pub unsafe fn ModuleImplementation(&self) -> ::windows::runtime::Result<IFsrmPipelineModuleImplementation> {
        let mut result__: <IFsrmPipelineModuleImplementation as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmPipelineModuleImplementation>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HostingUserAccount(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn HostingProcessPid(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn Bind<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmPipelineModuleDefinition>, Param1: ::windows::runtime::IntoParam<'a, IFsrmPipelineModuleImplementation>>(&self, moduledefinition: Param0, moduleimplementation: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), moduledefinition.into_param().abi(), moduleimplementation.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmPipelineModuleConnector {
    type Vtable = IFsrmPipelineModuleConnector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3244299507, 39585, 18099, [176, 167, 171, 20, 110, 178, 5, 242]);
}
impl ::std::convert::From<IFsrmPipelineModuleConnector> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmPipelineModuleConnector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmPipelineModuleConnector> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmPipelineModuleConnector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmPipelineModuleConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmPipelineModuleConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmPipelineModuleConnector> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmPipelineModuleConnector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmPipelineModuleConnector> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmPipelineModuleConnector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmPipelineModuleConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmPipelineModuleConnector {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleConnector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pipelinemoduleimplementation: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, username: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, useraccount: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pid: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduledefinition: ::windows::runtime::RawPtr, moduleimplementation: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmPipelineModuleDefinition(::windows::runtime::IUnknown);
impl IFsrmPipelineModuleDefinition {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleClsid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModuleClsid<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, moduleclsid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), moduleclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Company(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCompany<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, company: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), company.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Version(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVersion<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, version: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), version.into_param().abi()).ok()
    }
    pub unsafe fn ModuleType(&self) -> ::windows::runtime::Result<FsrmPipelineModuleType> {
        let mut result__: <FsrmPipelineModuleType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmPipelineModuleType>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    pub unsafe fn NeedsFileContent(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetNeedsFileContent(&self, needsfilecontent: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(needsfilecontent)).ok()
    }
    pub unsafe fn Account(&self) -> ::windows::runtime::Result<FsrmAccountType> {
        let mut result__: <FsrmAccountType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmAccountType>(result__)
    }
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(retrievalaccount)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedExtensions(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(supportedextensions)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(parameters)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmPipelineModuleDefinition {
    type Vtable = IFsrmPipelineModuleDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1364988535, 11393, 17422, [143, 207, 54, 121, 33, 237, 79, 89]);
}
impl ::std::convert::From<IFsrmPipelineModuleDefinition> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmPipelineModuleDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmPipelineModuleDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmPipelineModuleDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmPipelineModuleDefinition> for IFsrmObject {
    fn from(value: IFsrmPipelineModuleDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmPipelineModuleDefinition> for IFsrmObject {
    fn from(value: &IFsrmPipelineModuleDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmPipelineModuleDefinition> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmPipelineModuleDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmPipelineModuleDefinition> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmPipelineModuleDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmPipelineModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduleclsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduleclsid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, company: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, company: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, version: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, version: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduletype: *mut FsrmPipelineModuleType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, needsfilecontent: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, needsfilecontent: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, retrievalaccount: *mut FsrmAccountType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, retrievalaccount: FsrmAccountType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedextensions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmPipelineModuleImplementation(::windows::runtime::IUnknown);
impl IFsrmPipelineModuleImplementation {
    pub unsafe fn OnLoad<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmPipelineModuleDefinition>>(&self, moduledefinition: Param0) -> ::windows::runtime::Result<IFsrmPipelineModuleConnector> {
        let mut result__: <IFsrmPipelineModuleConnector as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), moduledefinition.into_param().abi(), &mut result__).from_abi::<IFsrmPipelineModuleConnector>(result__)
    }
    pub unsafe fn OnUnload(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmPipelineModuleImplementation {
    type Vtable = IFsrmPipelineModuleImplementation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3079698694, 11010, 19637, [132, 169, 253, 245, 70, 19, 214, 205]);
}
impl ::std::convert::From<IFsrmPipelineModuleImplementation> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmPipelineModuleImplementation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmPipelineModuleImplementation> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmPipelineModuleImplementation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmPipelineModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmPipelineModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmPipelineModuleImplementation> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmPipelineModuleImplementation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmPipelineModuleImplementation> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmPipelineModuleImplementation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmPipelineModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmPipelineModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPipelineModuleImplementation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduledefinition: ::windows::runtime::RawPtr, moduleconnector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmProperty(::windows::runtime::IUnknown);
impl IFsrmProperty {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Value(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Sources(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn PropertyFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmProperty {
    type Vtable = IFsrmProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1249115876, 16642, 20428, [159, 251, 56, 97, 79, 158, 231, 104]);
}
impl ::std::convert::From<IFsrmProperty> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmProperty) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmProperty> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmProperty) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmProperty> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmProperty) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmProperty> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmProperty) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sources: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmPropertyBag(::windows::runtime::IUnknown);
impl IFsrmPropertyBag {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelativePath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelativeNamespaceRoot(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn VolumeIndex(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn FileId(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ParentDirectoryId(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Size(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SizeAllocated(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn CreationTime(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn LastAccessTime(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn LastModificationTime(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn Attributes(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OwnerSid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FilePropertyNames(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Messages(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn PropertyBagFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<IFsrmProperty> {
        let mut result__: <IFsrmProperty as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IFsrmProperty>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), name.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddMessage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, message: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), message.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn GetFileStreamInterface(&self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(accessmode), ::std::mem::transmute(interfacetype), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmPropertyBag {
    type Vtable = IFsrmPropertyBag_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2001045969, 54016, 20346, [154, 36, 247, 183, 102, 128, 2, 80]);
}
impl ::std::convert::From<IFsrmPropertyBag> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmPropertyBag) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmPropertyBag> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmPropertyBag) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmPropertyBag {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmPropertyBag {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmPropertyBag> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmPropertyBag) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmPropertyBag> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmPropertyBag) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmPropertyBag {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmPropertyBag {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyBag_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, volumename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relativenamespaceroot: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, volumeid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fileid: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parentdirectoryid: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, size: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sizeallocated: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, creationtime: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastaccesstime: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastmodificationtime: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributes: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ownersid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepropertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messages: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, fileproperty: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType, pstreaminterface: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmPropertyBag2(::windows::runtime::IUnknown);
impl IFsrmPropertyBag2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelativePath(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn VolumeName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RelativeNamespaceRoot(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn VolumeIndex(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn FileId(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ParentDirectoryId(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Size(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SizeAllocated(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn CreationTime(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn LastAccessTime(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn LastModificationTime(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn Attributes(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OwnerSid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FilePropertyNames(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Messages(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn PropertyBagFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFileProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<IFsrmProperty> {
        let mut result__: <IFsrmProperty as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IFsrmProperty>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFileProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), name.into_param().abi(), value.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddMessage<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, message: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), message.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn GetFileStreamInterface(&self, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(accessmode), ::std::mem::transmute(interfacetype), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn GetFieldValue(&self, field: FsrmPropertyBagField) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(field), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn GetUntrustedInFileProperties(&self) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmPropertyBag2 {
    type Vtable = IFsrmPropertyBag2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(239517117, 9218, 20461, [156, 48, 146, 102, 230, 235, 44, 201]);
}
impl ::std::convert::From<IFsrmPropertyBag2> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmPropertyBag2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmPropertyBag2> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmPropertyBag2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmPropertyBag2> for IFsrmPropertyBag {
    fn from(value: IFsrmPropertyBag2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmPropertyBag2> for IFsrmPropertyBag {
    fn from(value: &IFsrmPropertyBag2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmPropertyBag> for IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmPropertyBag> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmPropertyBag>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmPropertyBag> for &IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmPropertyBag> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmPropertyBag>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmPropertyBag2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmPropertyBag2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmPropertyBag2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmPropertyBag2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmPropertyBag2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyBag2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, volumename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relativenamespaceroot: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, volumeid: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fileid: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parentdirectoryid: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, size: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sizeallocated: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, creationtime: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastaccesstime: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastmodificationtime: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributes: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ownersid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filepropertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messages: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, fileproperty: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, value: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accessmode: FsrmFileStreamingMode, interfacetype: FsrmFileStreamingInterfaceType, pstreaminterface: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, field: FsrmPropertyBagField, value: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, props: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmPropertyCondition(::windows::runtime::IUnknown);
impl IFsrmPropertyCondition {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<FsrmPropertyConditionType> {
        let mut result__: <FsrmPropertyConditionType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmPropertyConditionType>(result__)
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyConditionType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Value(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmPropertyCondition {
    type Vtable = IFsrmPropertyCondition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(845870703, 10944, 20328, [191, 140, 71, 89, 240, 84, 250, 41]);
}
impl ::std::convert::From<IFsrmPropertyCondition> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmPropertyCondition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmPropertyCondition> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmPropertyCondition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmPropertyCondition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmPropertyCondition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmPropertyCondition> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmPropertyCondition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmPropertyCondition> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmPropertyCondition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmPropertyCondition {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmPropertyCondition {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyCondition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut FsrmPropertyConditionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: FsrmPropertyConditionType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmPropertyDefinition(::windows::runtime::IUnknown);
impl IFsrmPropertyDefinition {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<FsrmPropertyDefinitionType> {
        let mut result__: <FsrmPropertyDefinitionType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmPropertyDefinitionType>(result__)
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyDefinitionType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PossibleValues(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPossibleValues(&self, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(possiblevalues)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValueDescriptions(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetValueDescriptions(&self, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(valuedescriptions)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(parameters)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmPropertyDefinition {
    type Vtable = IFsrmPropertyDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3990885647, 59811, 16796, [135, 124, 1, 254, 93, 36, 197, 211]);
}
impl ::std::convert::From<IFsrmPropertyDefinition> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmPropertyDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmPropertyDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmPropertyDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmPropertyDefinition> for IFsrmObject {
    fn from(value: IFsrmPropertyDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmPropertyDefinition> for IFsrmObject {
    fn from(value: &IFsrmPropertyDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmPropertyDefinition> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmPropertyDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmPropertyDefinition> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmPropertyDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmPropertyDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut FsrmPropertyDefinitionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: FsrmPropertyDefinitionType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, possiblevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, valuedescriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmPropertyDefinition2(::windows::runtime::IUnknown);
impl IFsrmPropertyDefinition2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<FsrmPropertyDefinitionType> {
        let mut result__: <FsrmPropertyDefinitionType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmPropertyDefinitionType>(result__)
    }
    pub unsafe fn SetType(&self, r#type: FsrmPropertyDefinitionType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PossibleValues(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetPossibleValues(&self, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(possiblevalues)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ValueDescriptions(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetValueDescriptions(&self, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(valuedescriptions)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(parameters)).ok()
    }
    pub unsafe fn PropertyDefinitionFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn AppliesTo(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn ValueDefinitions(&self) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmPropertyDefinition2 {
    type Vtable = IFsrmPropertyDefinition2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1199055186, 53612, 16937, [180, 225, 13, 223, 227, 8, 185, 246]);
}
impl ::std::convert::From<IFsrmPropertyDefinition2> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmPropertyDefinition2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmPropertyDefinition2> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmPropertyDefinition2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmPropertyDefinition2> for IFsrmPropertyDefinition {
    fn from(value: IFsrmPropertyDefinition2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmPropertyDefinition2> for IFsrmPropertyDefinition {
    fn from(value: &IFsrmPropertyDefinition2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmPropertyDefinition> for IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmPropertyDefinition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmPropertyDefinition>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmPropertyDefinition> for &IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmPropertyDefinition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmPropertyDefinition>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmPropertyDefinition2> for IFsrmObject {
    fn from(value: IFsrmPropertyDefinition2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmPropertyDefinition2> for IFsrmObject {
    fn from(value: &IFsrmPropertyDefinition2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmPropertyDefinition2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmPropertyDefinition2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmPropertyDefinition2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmPropertyDefinition2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmPropertyDefinition2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinition2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut FsrmPropertyDefinitionType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: FsrmPropertyDefinitionType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, possiblevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, possiblevalues: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, valuedescriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, valuedescriptions: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertydefinitionflags: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appliesto: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, valuedefinitions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmPropertyDefinitionValue(::windows::runtime::IUnknown);
impl IFsrmPropertyDefinitionValue {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DisplayName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UniqueID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmPropertyDefinitionValue {
    type Vtable = IFsrmPropertyDefinitionValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3913732424, 48487, 16760, [142, 34, 28, 68, 146, 94, 215, 16]);
}
impl ::std::convert::From<IFsrmPropertyDefinitionValue> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmPropertyDefinitionValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmPropertyDefinitionValue> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmPropertyDefinitionValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmPropertyDefinitionValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmPropertyDefinitionValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmPropertyDefinitionValue> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmPropertyDefinitionValue) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmPropertyDefinitionValue> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmPropertyDefinitionValue) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmPropertyDefinitionValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmPropertyDefinitionValue {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmPropertyDefinitionValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displayname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uniqueid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmQuota(::windows::runtime::IUnknown);
impl IFsrmQuota {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), ::std::mem::transmute(newthreshold)).ok()
    }
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::runtime::Result<IFsrmAction> {
        let mut result__: <IFsrmAction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), ::std::mem::transmute(actiontype), &mut result__).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserSid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceTemplateName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), quotatemplatename.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn QuotaUsed(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn QuotaPeakUsage(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    pub unsafe fn QuotaPeakUsageTime(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    pub unsafe fn ResetPeakUsage(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RefreshUsageProperties(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmQuota {
    type Vtable = IFsrmQuota_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(931099549, 38471, 19342, [151, 210, 95, 252, 230, 215, 89, 205]);
}
impl ::std::convert::From<IFsrmQuota> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuota> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmQuota) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmQuota> for IFsrmQuotaObject {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuota> for IFsrmQuotaObject {
    fn from(value: &IFsrmQuota) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaObject> for IFsrmQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaObject> for &IFsrmQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaObject>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmQuota> for IFsrmQuotaBase {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuota> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuota) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaBase>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaBase> for &IFsrmQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaBase>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmQuota> for IFsrmObject {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuota> for IFsrmObject {
    fn from(value: &IFsrmQuota) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmQuota> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmQuota) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmQuota> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmQuota) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmQuota {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuota_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotalimit: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotalimit: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotaflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotaflags: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, newthreshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, actions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, useraccount: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotatemplatename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matches: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotatemplatename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, used: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, peakusage: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, peakusagedatetime: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmQuotaBase(::windows::runtime::IUnknown);
impl IFsrmQuotaBase {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), ::std::mem::transmute(newthreshold)).ok()
    }
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::runtime::Result<IFsrmAction> {
        let mut result__: <IFsrmAction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), ::std::mem::transmute(actiontype), &mut result__).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmQuotaBase {
    type Vtable = IFsrmQuotaBase_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(359180181, 14628, 16664, [183, 75, 104, 216, 240, 250, 93, 175]);
}
impl ::std::convert::From<IFsrmQuotaBase> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmQuotaBase) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaBase> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmQuotaBase) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmQuotaBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmQuotaBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmQuotaBase> for IFsrmObject {
    fn from(value: IFsrmQuotaBase) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaBase> for IFsrmObject {
    fn from(value: &IFsrmQuotaBase) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmQuotaBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmQuotaBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmQuotaBase> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmQuotaBase) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmQuotaBase> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmQuotaBase) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmQuotaBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmQuotaBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaBase_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotalimit: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotalimit: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotaflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotaflags: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, newthreshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, actions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmQuotaManager(::windows::runtime::IUnknown);
impl IFsrmQuotaManager {
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateQuota<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<IFsrmQuota> {
        let mut result__: <IFsrmQuota as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateAutoApplyQuota<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0, path: Param1) -> ::windows::runtime::Result<IFsrmAutoApplyQuota> {
        let mut result__: <IFsrmAutoApplyQuota as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), quotatemplatename.into_param().abi(), path.into_param().abi(), &mut result__).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQuota<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<IFsrmQuota> {
        let mut result__: <IFsrmQuota as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAutoApplyQuota<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<IFsrmAutoApplyQuota> {
        let mut result__: <IFsrmAutoApplyQuota as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestrictiveQuota<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<IFsrmQuota> {
        let mut result__: <IFsrmQuota as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumQuotas<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), path.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAutoApplyQuotas<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), path.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffectiveQuotas<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), path.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scan<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), strpath.into_param().abi()).ok()
    }
    pub unsafe fn CreateQuotaCollection(&self) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmQuotaManager {
    type Vtable = IFsrmQuotaManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2343996541, 6616, 20475, [128, 158, 190, 79, 193, 115, 64, 20]);
}
impl ::std::convert::From<IFsrmQuotaManager> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmQuotaManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaManager> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmQuotaManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmQuotaManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmQuotaManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmQuotaManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmQuotaManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmQuotaManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmQuotaManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmQuotaManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmQuotaManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotatemplatename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, collection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmQuotaManagerEx(::windows::runtime::IUnknown);
impl IFsrmQuotaManagerEx {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariables(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActionVariableDescriptions(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateQuota<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<IFsrmQuota> {
        let mut result__: <IFsrmQuota as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateAutoApplyQuota<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0, path: Param1) -> ::windows::runtime::Result<IFsrmAutoApplyQuota> {
        let mut result__: <IFsrmAutoApplyQuota as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), quotatemplatename.into_param().abi(), path.into_param().abi(), &mut result__).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetQuota<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<IFsrmQuota> {
        let mut result__: <IFsrmQuota as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAutoApplyQuota<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<IFsrmAutoApplyQuota> {
        let mut result__: <IFsrmAutoApplyQuota as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IFsrmAutoApplyQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestrictiveQuota<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0) -> ::windows::runtime::Result<IFsrmQuota> {
        let mut result__: <IFsrmQuota as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), path.into_param().abi(), &mut result__).from_abi::<IFsrmQuota>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumQuotas<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), path.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAutoApplyQuotas<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), path.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffectiveQuotas<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), path.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scan<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), strpath.into_param().abi()).ok()
    }
    pub unsafe fn CreateQuotaCollection(&self) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsAffectedByQuota<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, path: Param0, options: FsrmEnumOptions) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), path.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmQuotaManagerEx {
    type Vtable = IFsrmQuotaManagerEx_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1212599041, 54320, 18767, [171, 180, 177, 5, 73, 153, 251, 9]);
}
impl ::std::convert::From<IFsrmQuotaManagerEx> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmQuotaManagerEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaManagerEx> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmQuotaManagerEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmQuotaManagerEx> for IFsrmQuotaManager {
    fn from(value: IFsrmQuotaManagerEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaManagerEx> for IFsrmQuotaManager {
    fn from(value: &IFsrmQuotaManagerEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaManager> for IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaManager>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaManager> for &IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaManager> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaManager>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmQuotaManagerEx> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmQuotaManagerEx) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmQuotaManagerEx> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmQuotaManagerEx) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmQuotaManagerEx {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaManagerEx_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, variables: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, descriptions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotatemplatename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, quota: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, quotas: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, collection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, options: FsrmEnumOptions, affected: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmQuotaObject(::windows::runtime::IUnknown);
impl IFsrmQuotaObject {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), ::std::mem::transmute(newthreshold)).ok()
    }
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::runtime::Result<IFsrmAction> {
        let mut result__: <IFsrmAction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), ::std::mem::transmute(actiontype), &mut result__).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserSid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserAccount(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SourceTemplateName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn MatchesSourceTemplate(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyTemplate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), quotatemplatename.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmQuotaObject {
    type Vtable = IFsrmQuotaObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1121727761, 25045, 18606, [182, 220, 89, 252, 0, 192, 168, 214]);
}
impl ::std::convert::From<IFsrmQuotaObject> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmQuotaObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaObject> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmQuotaObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmQuotaObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmQuotaObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmQuotaObject> for IFsrmQuotaBase {
    fn from(value: IFsrmQuotaObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaObject> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuotaObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuotaObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaBase>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaBase> for &IFsrmQuotaObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaBase>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmQuotaObject> for IFsrmObject {
    fn from(value: IFsrmQuotaObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaObject> for IFsrmObject {
    fn from(value: &IFsrmQuotaObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmQuotaObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmQuotaObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmQuotaObject> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmQuotaObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmQuotaObject> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmQuotaObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmQuotaObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmQuotaObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotalimit: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotalimit: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotaflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotaflags: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, newthreshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, actions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usersid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, useraccount: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotatemplatename: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, matches: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotatemplatename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmQuotaTemplate(::windows::runtime::IUnknown);
impl IFsrmQuotaTemplate {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), ::std::mem::transmute(newthreshold)).ok()
    }
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::runtime::Result<IFsrmAction> {
        let mut result__: <IFsrmAction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), ::std::mem::transmute(actiontype), &mut result__).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTemplate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), quotatemplatename.into_param().abi()).ok()
    }
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::runtime::Result<IFsrmDerivedObjectsResult> {
        let mut result__: <IFsrmDerivedObjectsResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(commitoptions), ::std::mem::transmute(applyoptions), &mut result__).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmQuotaTemplate {
    type Vtable = IFsrmQuotaTemplate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2733615921, 10590, 18107, [185, 118, 232, 109, 88, 181, 46, 139]);
}
impl ::std::convert::From<IFsrmQuotaTemplate> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmQuotaTemplate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaTemplate> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmQuotaTemplate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmQuotaTemplate> for IFsrmQuotaBase {
    fn from(value: IFsrmQuotaTemplate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaTemplate> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuotaTemplate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaBase>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaBase> for &IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaBase>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmQuotaTemplate> for IFsrmObject {
    fn from(value: IFsrmQuotaTemplate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaTemplate> for IFsrmObject {
    fn from(value: &IFsrmQuotaTemplate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmQuotaTemplate> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmQuotaTemplate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmQuotaTemplate> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmQuotaTemplate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmQuotaTemplate {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotalimit: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotalimit: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotaflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotaflags: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, newthreshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, actions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotatemplatename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmQuotaTemplateImported(::windows::runtime::IUnknown);
impl IFsrmQuotaTemplateImported {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn QuotaLimit(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetQuotaLimit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, quotalimit: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), quotalimit.into_param().abi()).ok()
    }
    pub unsafe fn QuotaFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetQuotaFlags(&self, quotaflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(quotaflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Thresholds(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    pub unsafe fn AddThreshold(&self, threshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold)).ok()
    }
    pub unsafe fn DeleteThreshold(&self, threshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold)).ok()
    }
    pub unsafe fn ModifyThreshold(&self, threshold: i32, newthreshold: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), ::std::mem::transmute(newthreshold)).ok()
    }
    pub unsafe fn CreateThresholdAction(&self, threshold: i32, actiontype: FsrmActionType) -> ::windows::runtime::Result<IFsrmAction> {
        let mut result__: <IFsrmAction as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), ::std::mem::transmute(actiontype), &mut result__).from_abi::<IFsrmAction>(result__)
    }
    pub unsafe fn EnumThresholdActions(&self, threshold: i32) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(threshold), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTemplate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, quotatemplatename: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), quotatemplatename.into_param().abi()).ok()
    }
    pub unsafe fn CommitAndUpdateDerived(&self, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions) -> ::windows::runtime::Result<IFsrmDerivedObjectsResult> {
        let mut result__: <IFsrmDerivedObjectsResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(commitoptions), ::std::mem::transmute(applyoptions), &mut result__).from_abi::<IFsrmDerivedObjectsResult>(result__)
    }
    pub unsafe fn OverwriteOnCommit(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetOverwriteOnCommit(&self, overwrite: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(overwrite)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmQuotaTemplateImported {
    type Vtable = IFsrmQuotaTemplateImported_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2586571027, 41769, 17612, [128, 154, 92, 0, 252, 232, 218, 64]);
}
impl ::std::convert::From<IFsrmQuotaTemplateImported> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaTemplateImported> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmQuotaTemplateImported> for IFsrmQuotaTemplate {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaTemplateImported> for IFsrmQuotaTemplate {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaTemplate> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaTemplate> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaTemplate>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaTemplate> for &IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaTemplate> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaTemplate>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmQuotaTemplateImported> for IFsrmQuotaBase {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaTemplateImported> for IFsrmQuotaBase {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaBase> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaBase>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmQuotaBase> for &IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmQuotaBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmQuotaBase>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmQuotaTemplateImported> for IFsrmObject {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaTemplateImported> for IFsrmObject {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmQuotaTemplateImported> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmQuotaTemplateImported) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmQuotaTemplateImported> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmQuotaTemplateImported) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmQuotaTemplateImported {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplateImported_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotalimit: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotalimit: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotaflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotaflags: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, thresholds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, newthreshold: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, actiontype: FsrmActionType, action: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threshold: i32, actions: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotatemplatename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, commitoptions: FsrmCommitOptions, applyoptions: FsrmTemplateApplyOptions, derivedobjectsresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, overwrite: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, overwrite: i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmQuotaTemplateManager(::windows::runtime::IUnknown);
impl IFsrmQuotaTemplateManager {
    pub unsafe fn CreateTemplate(&self) -> ::windows::runtime::Result<IFsrmQuotaTemplate> {
        let mut result__: <IFsrmQuotaTemplate as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmQuotaTemplate>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTemplate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<IFsrmQuotaTemplate> {
        let mut result__: <IFsrmQuotaTemplate as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), name.into_param().abi(), &mut result__).from_abi::<IFsrmQuotaTemplate>(result__)
    }
    pub unsafe fn EnumTemplates(&self, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ExportTemplates(&self, quotatemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(quotatemplatenamesarray), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ImportTemplates<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, serializedquotatemplates: Param0, quotatemplatenamesarray: *const super::super::System::Com::VARIANT) -> ::windows::runtime::Result<IFsrmCommittableCollection> {
        let mut result__: <IFsrmCommittableCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), serializedquotatemplates.into_param().abi(), ::std::mem::transmute(quotatemplatenamesarray), &mut result__).from_abi::<IFsrmCommittableCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmQuotaTemplateManager {
    type Vtable = IFsrmQuotaTemplateManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1098099777, 5933, 19794, [150, 60, 253, 199, 228, 21, 247, 23]);
}
impl ::std::convert::From<IFsrmQuotaTemplateManager> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmQuotaTemplateManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmQuotaTemplateManager> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmQuotaTemplateManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmQuotaTemplateManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmQuotaTemplateManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmQuotaTemplateManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmQuotaTemplateManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmQuotaTemplateManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmQuotaTemplateManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmQuotaTemplateManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmQuotaTemplateManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmQuotaTemplateManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotatemplate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, quotatemplate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: FsrmEnumOptions, quotatemplates: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, quotatemplatenamesarray: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, serializedquotatemplates: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, serializedquotatemplates: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, quotatemplatenamesarray: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, quotatemplates: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmReport(::windows::runtime::IUnknown);
impl IFsrmReport {
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<FsrmReportType> {
        let mut result__: <FsrmReportType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmReportType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastGeneratedFileNamePrefix(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn GetFilter(&self, filter: FsrmReportFilter) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(filter), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetFilter<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, filter: FsrmReportFilter, filtervalue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(filter), filtervalue.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmReport {
    type Vtable = IFsrmReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3637281241, 18104, 20388, [191, 165, 74, 169, 222, 201, 182, 56]);
}
impl ::std::convert::From<IFsrmReport> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmReport> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmReport> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmReport> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reporttype: *mut FsrmReportType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prefix: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filter: FsrmReportFilter, filtervalue: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filter: FsrmReportFilter, filtervalue: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmReportJob(::windows::runtime::IUnknown);
impl IFsrmReportJob {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Task(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTask<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), taskname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(namespaceroots)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Formats(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetFormats(&self, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(formats)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailTo(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailTo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn RunningStatus(&self) -> ::windows::runtime::Result<FsrmReportRunningStatus> {
        let mut result__: <FsrmReportRunningStatus as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmReportRunningStatus>(result__)
    }
    pub unsafe fn LastRun(&self) -> ::windows::runtime::Result<f64> {
        let mut result__: <f64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f64>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastError(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LastGeneratedInDirectory(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn EnumReports(&self) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreateReport(&self, reporttype: FsrmReportType) -> ::windows::runtime::Result<IFsrmReport> {
        let mut result__: <IFsrmReport as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(reporttype), &mut result__).from_abi::<IFsrmReport>(result__)
    }
    pub unsafe fn Run(&self, context: FsrmReportGenerationContext) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(context)).ok()
    }
    pub unsafe fn WaitForCompletion(&self, waitseconds: i32) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(waitseconds), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmReportJob {
    type Vtable = IFsrmReportJob_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(954757760, 29020, 19581, [162, 128, 234, 22, 81, 161, 159, 239]);
}
impl ::std::convert::From<IFsrmReportJob> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmReportJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmReportJob> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmReportJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmReportJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmReportJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmReportJob> for IFsrmObject {
    fn from(value: IFsrmReportJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmReportJob> for IFsrmObject {
    fn from(value: &IFsrmReportJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmReportJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmReportJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmReportJob> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmReportJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmReportJob> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmReportJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmReportJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmReportJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportJob_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, taskname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, taskname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formats: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, formats: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailto: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailto: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, runningstatus: *mut FsrmReportRunningStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastrun: *mut f64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lasterror: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reports: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reporttype: FsrmReportType, report: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: FsrmReportGenerationContext) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, waitseconds: i32, completed: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmReportManager(::windows::runtime::IUnknown);
impl IFsrmReportManager {
    pub unsafe fn EnumReportJobs(&self, options: FsrmEnumOptions) -> ::windows::runtime::Result<IFsrmCollection> {
        let mut result__: <IFsrmCollection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(options), &mut result__).from_abi::<IFsrmCollection>(result__)
    }
    pub unsafe fn CreateReportJob(&self) -> ::windows::runtime::Result<IFsrmReportJob> {
        let mut result__: <IFsrmReportJob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFsrmReportJob>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReportJob<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0) -> ::windows::runtime::Result<IFsrmReportJob> {
        let mut result__: <IFsrmReportJob as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), taskname.into_param().abi(), &mut result__).from_abi::<IFsrmReportJob>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputDirectory(&self, context: FsrmReportGenerationContext) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(context), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOutputDirectory<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, context: FsrmReportGenerationContext, path: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(context), path.into_param().abi()).ok()
    }
    pub unsafe fn IsFilterValidForReportType(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(reporttype), ::std::mem::transmute(filter), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn GetDefaultFilter(&self, reporttype: FsrmReportType, filter: FsrmReportFilter) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(reporttype), ::std::mem::transmute(filter), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetDefaultFilter<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(reporttype), ::std::mem::transmute(filter), filtervalue.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn GetReportSizeLimit(&self, limit: FsrmReportLimit) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(limit), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetReportSizeLimit<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, limit: FsrmReportLimit, limitvalue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(limit), limitvalue.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmReportManager {
    type Vtable = IFsrmReportManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(666409470, 28666, 17537, [161, 132, 211, 218, 173, 232, 160, 43]);
}
impl ::std::convert::From<IFsrmReportManager> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmReportManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmReportManager> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmReportManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmReportManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmReportManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmReportManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmReportManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmReportManager> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmReportManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmReportManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmReportManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: FsrmEnumOptions, reportjobs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reportjob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, taskname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, reportjob: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: FsrmReportGenerationContext, path: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: FsrmReportGenerationContext, path: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reporttype: FsrmReportType, filter: FsrmReportFilter, valid: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reporttype: FsrmReportType, filter: FsrmReportFilter, filtervalue: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, limit: FsrmReportLimit, limitvalue: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, limit: FsrmReportLimit, limitvalue: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmReportScheduler(::windows::runtime::IUnknown);
impl IFsrmReportScheduler {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn VerifyNamespaces(&self, namespacessafearray: *const super::super::System::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(namespacessafearray)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn CreateScheduleTask<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), taskname.into_param().abi(), ::std::mem::transmute(namespacessafearray), serializedtask.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ModifyScheduleTask<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0, namespacessafearray: *const super::super::System::Com::VARIANT, serializedtask: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), taskname.into_param().abi(), ::std::mem::transmute(namespacessafearray), serializedtask.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteScheduleTask<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, taskname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), taskname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmReportScheduler {
    type Vtable = IFsrmReportScheduler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1752812281, 26135, 17540, [135, 25, 113, 195, 216, 100, 95, 148]);
}
impl ::std::convert::From<IFsrmReportScheduler> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmReportScheduler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmReportScheduler> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmReportScheduler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmReportScheduler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmReportScheduler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmReportScheduler> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmReportScheduler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmReportScheduler> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmReportScheduler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmReportScheduler {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmReportScheduler {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmReportScheduler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespacessafearray: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, taskname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespacessafearray: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, serializedtask: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, taskname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, namespacessafearray: *const ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, serializedtask: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, taskname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmRule(::windows::runtime::IUnknown);
impl IFsrmRule {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    pub unsafe fn RuleType(&self) -> ::windows::runtime::Result<FsrmRuleType> {
        let mut result__: <FsrmRuleType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmRuleType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleDefinitionName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModuleDefinitionName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, moduledefinitionname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), moduledefinitionname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NamespaceRoots(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetNamespaceRoots(&self, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(namespaceroots)).ok()
    }
    pub unsafe fn RuleFlags(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetRuleFlags(&self, ruleflags: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(ruleflags)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(parameters)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn LastModified(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmRule {
    type Vtable = IFsrmRule_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3406690656, 5877, 17557, [144, 121, 63, 147, 96, 216, 49, 223]);
}
impl ::std::convert::From<IFsrmRule> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmRule) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmRule> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmRule) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmRule> for IFsrmObject {
    fn from(value: IFsrmRule) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmRule> for IFsrmObject {
    fn from(value: &IFsrmRule) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmRule> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmRule) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmRule> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmRule) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmRule_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ruletype: *mut FsrmRuleType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduledefinitionname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduledefinitionname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceroots: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, namespaceroots: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ruleflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ruleflags: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lastmodified: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmSetting(::windows::runtime::IUnknown);
impl IFsrmSetting {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SmtpServer(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSmtpServer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, smtpserver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), smtpserver.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MailFrom(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMailFrom<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailfrom: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), mailfrom.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdminEmail(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAdminEmail<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, adminemail: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), adminemail.into_param().abi()).ok()
    }
    pub unsafe fn DisableCommandLine(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetDisableCommandLine(&self, disablecommandline: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(disablecommandline)).ok()
    }
    pub unsafe fn EnableScreeningAudit(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnableScreeningAudit(&self, enablescreeningaudit: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(enablescreeningaudit)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EmailTest<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, mailto: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), mailto.into_param().abi()).ok()
    }
    pub unsafe fn SetActionRunLimitInterval(&self, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(actiontype), ::std::mem::transmute(delaytimeminutes)).ok()
    }
    pub unsafe fn GetActionRunLimitInterval(&self, actiontype: FsrmActionType) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(actiontype), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmSetting {
    type Vtable = IFsrmSetting_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4094809341, 5310, 16992, [140, 64, 3, 183, 201, 94, 96, 138]);
}
impl ::std::convert::From<IFsrmSetting> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmSetting) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmSetting> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmSetting) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmSetting {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmSetting {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmSetting> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmSetting) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmSetting> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmSetting) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmSetting {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmSetting {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmSetting_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, smtpserver: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, smtpserver: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailfrom: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailfrom: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, adminemail: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, adminemail: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, disablecommandline: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, disablecommandline: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enablescreeningaudit: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enablescreeningaudit: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mailto: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actiontype: FsrmActionType, delaytimeminutes: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actiontype: FsrmActionType, delaytimeminutes: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmStorageModuleDefinition(::windows::runtime::IUnknown);
impl IFsrmStorageModuleDefinition {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, description: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), description.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ModuleClsid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModuleClsid<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, moduleclsid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), moduleclsid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), name.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Company(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCompany<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, company: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), company.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Version(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVersion<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, version: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), version.into_param().abi()).ok()
    }
    pub unsafe fn ModuleType(&self) -> ::windows::runtime::Result<FsrmPipelineModuleType> {
        let mut result__: <FsrmPipelineModuleType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmPipelineModuleType>(result__)
    }
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetEnabled(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    pub unsafe fn NeedsFileContent(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetNeedsFileContent(&self, needsfilecontent: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(needsfilecontent)).ok()
    }
    pub unsafe fn Account(&self) -> ::windows::runtime::Result<FsrmAccountType> {
        let mut result__: <FsrmAccountType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmAccountType>(result__)
    }
    pub unsafe fn SetAccount(&self, retrievalaccount: FsrmAccountType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(retrievalaccount)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SupportedExtensions(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSupportedExtensions(&self, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(supportedextensions)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Parameters(&self) -> ::windows::runtime::Result<*mut super::super::System::Com::SAFEARRAY> {
        let mut result__: <*mut super::super::System::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::super::System::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetParameters(&self, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(parameters)).ok()
    }
    pub unsafe fn Capabilities(&self) -> ::windows::runtime::Result<FsrmStorageModuleCaps> {
        let mut result__: <FsrmStorageModuleCaps as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmStorageModuleCaps>(result__)
    }
    pub unsafe fn SetCapabilities(&self, capabilities: FsrmStorageModuleCaps) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(capabilities)).ok()
    }
    pub unsafe fn StorageType(&self) -> ::windows::runtime::Result<FsrmStorageModuleType> {
        let mut result__: <FsrmStorageModuleType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), &mut result__).from_abi::<FsrmStorageModuleType>(result__)
    }
    pub unsafe fn SetStorageType(&self, storagetype: FsrmStorageModuleType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(storagetype)).ok()
    }
    pub unsafe fn UpdatesFileContent(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetUpdatesFileContent(&self, updatesfilecontent: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), ::std::mem::transmute(updatesfilecontent)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmStorageModuleDefinition {
    type Vtable = IFsrmStorageModuleDefinition_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(363336528, 18813, 19130, [128, 233, 212, 219, 204, 85, 33, 254]);
}
impl ::std::convert::From<IFsrmStorageModuleDefinition> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmStorageModuleDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmStorageModuleDefinition> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmStorageModuleDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmStorageModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: IFsrmStorageModuleDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmStorageModuleDefinition> for IFsrmPipelineModuleDefinition {
    fn from(value: &IFsrmStorageModuleDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmPipelineModuleDefinition> for IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmPipelineModuleDefinition>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmPipelineModuleDefinition> for &IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmPipelineModuleDefinition> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmPipelineModuleDefinition>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmStorageModuleDefinition> for IFsrmObject {
    fn from(value: IFsrmStorageModuleDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmStorageModuleDefinition> for IFsrmObject {
    fn from(value: &IFsrmStorageModuleDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmObject> for &IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmObject> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmObject>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmStorageModuleDefinition> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmStorageModuleDefinition) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmStorageModuleDefinition> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmStorageModuleDefinition) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmStorageModuleDefinition {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmStorageModuleDefinition_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduleclsid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduleclsid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, company: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, company: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, version: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, version: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduletype: *mut FsrmPipelineModuleType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, needsfilecontent: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, needsfilecontent: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, retrievalaccount: *mut FsrmAccountType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, retrievalaccount: FsrmAccountType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedextensions: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedextensions: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameters: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capabilities: *mut FsrmStorageModuleCaps) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capabilities: FsrmStorageModuleCaps) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storagetype: *mut FsrmStorageModuleType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, storagetype: FsrmStorageModuleType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updatesfilecontent: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updatesfilecontent: i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFsrmStorageModuleImplementation(::windows::runtime::IUnknown);
impl IFsrmStorageModuleImplementation {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::super::System::Ole::Automation::ITypeInfo> {
        let mut result__: <super::super::System::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::super::System::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut super::super::System::Com::VARIANT, pexcepinfo: *mut super::super::System::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn OnLoad<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmPipelineModuleDefinition>>(&self, moduledefinition: Param0) -> ::windows::runtime::Result<IFsrmPipelineModuleConnector> {
        let mut result__: <IFsrmPipelineModuleConnector as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), moduledefinition.into_param().abi(), &mut result__).from_abi::<IFsrmPipelineModuleConnector>(result__)
    }
    pub unsafe fn OnUnload(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn UseDefinitions<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmCollection>>(&self, propertydefinitions: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), propertydefinitions.into_param().abi()).ok()
    }
    pub unsafe fn LoadProperties<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmPropertyBag>>(&self, propertybag: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), propertybag.into_param().abi()).ok()
    }
    pub unsafe fn SaveProperties<'a, Param0: ::windows::runtime::IntoParam<'a, IFsrmPropertyBag>>(&self, propertybag: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), propertybag.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFsrmStorageModuleImplementation {
    type Vtable = IFsrmStorageModuleImplementation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(183804122, 35162, 20048, [135, 18, 169, 103, 36, 188, 236, 100]);
}
impl ::std::convert::From<IFsrmStorageModuleImplementation> for ::windows::runtime::IUnknown {
    fn from(value: IFsrmStorageModuleImplementation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmStorageModuleImplementation> for ::windows::runtime::IUnknown {
    fn from(value: &IFsrmStorageModuleImplementation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IFsrmStorageModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: IFsrmStorageModuleImplementation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFsrmStorageModuleImplementation> for IFsrmPipelineModuleImplementation {
    fn from(value: &IFsrmStorageModuleImplementation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmPipelineModuleImplementation> for IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmPipelineModuleImplementation>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFsrmPipelineModuleImplementation> for &IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFsrmPipelineModuleImplementation> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFsrmPipelineModuleImplementation>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IFsrmStorageModuleImplementation> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IFsrmStorageModuleImplementation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IFsrmStorageModuleImplementation> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IFsrmStorageModuleImplementation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IFsrmStorageModuleImplementation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFsrmStorageModuleImplementation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, moduledefinition: ::windows::runtime::RawPtr, moduleconnector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertydefinitions: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertybag: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertybag: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
pub const MessageSizeLimit: u32 = 4096u32;
