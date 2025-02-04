#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const AADBE_ADD_ENTRY: u32 = 1u32;
pub const AADBE_DEL_ENTRY: u32 = 2u32;
pub const ACTCTX_FLAG_APPLICATION_NAME_VALID: u32 = 32u32;
pub const ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID: u32 = 4u32;
pub const ACTCTX_FLAG_HMODULE_VALID: u32 = 128u32;
pub const ACTCTX_FLAG_LANGID_VALID: u32 = 2u32;
pub const ACTCTX_FLAG_PROCESSOR_ARCHITECTURE_VALID: u32 = 1u32;
pub const ACTCTX_FLAG_RESOURCE_NAME_VALID: u32 = 8u32;
pub const ACTCTX_FLAG_SET_PROCESS_DEFAULT: u32 = 16u32;
pub const ACTCTX_FLAG_SOURCE_IS_ASSEMBLYREF: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTCTX_SECTION_KEYED_DATA_2600 {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: *mut ::std::ffi::c_void,
    pub ulLength: u32,
    pub lpSectionGlobalData: *mut ::std::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: *mut ::std::ffi::c_void,
    pub ulSectionTotalLength: u32,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub ulAssemblyRosterIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ACTCTX_SECTION_KEYED_DATA_2600 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACTCTX_SECTION_KEYED_DATA_2600")
            .field("cbSize", &self.cbSize)
            .field("ulDataFormatVersion", &self.ulDataFormatVersion)
            .field("lpData", &self.lpData)
            .field("ulLength", &self.ulLength)
            .field("lpSectionGlobalData", &self.lpSectionGlobalData)
            .field("ulSectionGlobalDataLength", &self.ulSectionGlobalDataLength)
            .field("lpSectionBase", &self.lpSectionBase)
            .field("ulSectionTotalLength", &self.ulSectionTotalLength)
            .field("hActCtx", &self.hActCtx)
            .field("ulAssemblyRosterIndex", &self.ulAssemblyRosterIndex)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulDataFormatVersion == other.ulDataFormatVersion && self.lpData == other.lpData && self.ulLength == other.ulLength && self.lpSectionGlobalData == other.lpSectionGlobalData && self.ulSectionGlobalDataLength == other.ulSectionGlobalDataLength && self.lpSectionBase == other.lpSectionBase && self.ulSectionTotalLength == other.ulSectionTotalLength && self.hActCtx == other.hActCtx && self.ulAssemblyRosterIndex == other.ulAssemblyRosterIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ACTCTX_SECTION_KEYED_DATA_2600 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ACTCTX_SECTION_KEYED_DATA_2600 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    pub lpInformation: *mut ::std::ffi::c_void,
    pub lpSectionBase: *mut ::std::ffi::c_void,
    pub ulSectionLength: u32,
    pub lpSectionGlobalDataBase: *mut ::std::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
}
impl ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {}
impl ::std::default::Default for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA")
            .field("lpInformation", &self.lpInformation)
            .field("lpSectionBase", &self.lpSectionBase)
            .field("ulSectionLength", &self.ulSectionLength)
            .field("lpSectionGlobalDataBase", &self.lpSectionGlobalDataBase)
            .field("ulSectionGlobalDataLength", &self.ulSectionGlobalDataLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpInformation == other.lpInformation && self.lpSectionBase == other.lpSectionBase && self.ulSectionLength == other.ulSectionLength && self.lpSectionGlobalDataBase == other.lpSectionGlobalDataBase && self.ulSectionGlobalDataLength == other.ulSectionGlobalDataLength
    }
}
impl ::std::cmp::Eq for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {}
unsafe impl ::windows::runtime::Abi for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTIVATION_CONTEXT_BASIC_INFORMATION {
    pub hActCtx: super::super::Foundation::HANDLE,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ACTIVATION_CONTEXT_BASIC_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ACTIVATION_CONTEXT_BASIC_INFORMATION").field("hActCtx", &self.hActCtx).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.hActCtx == other.hActCtx && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ACTIVATION_CONTEXT_BASIC_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ACTIVATION_CONTEXT_BASIC_INFORMATION_DEFINED: u32 = 1u32;
pub const AC_LINE_BACKUP_POWER: u32 = 2u32;
pub const AC_LINE_OFFLINE: u32 = 0u32;
pub const AC_LINE_ONLINE: u32 = 1u32;
pub const AC_LINE_UNKNOWN: u32 = 255u32;
pub const ADN_DEL_IF_EMPTY: u32 = 1u32;
pub const ADN_DEL_UNC_PATHS: u32 = 8u32;
pub const ADN_DONT_DEL_DIR: u32 = 4u32;
pub const ADN_DONT_DEL_SUBDIRS: u32 = 2u32;
pub const AFSR_BACKNEW: u32 = 2u32;
pub const AFSR_EXTRAINCREFCNT: u32 = 2048u32;
pub const AFSR_NODELETENEW: u32 = 4u32;
pub const AFSR_NOMESSAGES: u32 = 8u32;
pub const AFSR_NOPROGRESS: u32 = 16u32;
pub const AFSR_RESTORE: u32 = 1u32;
pub const AFSR_UPDREFCNT: u32 = 512u32;
pub const AFSR_USEREFCNT: u32 = 1024u32;
pub const AIF_FORCE_FILE_IN_USE: u32 = 8u32;
pub const AIF_NOLANGUAGECHECK: u32 = 268435456u32;
pub const AIF_NOOVERWRITE: u32 = 16u32;
pub const AIF_NOSKIP: u32 = 2u32;
pub const AIF_NOVERSIONCHECK: u32 = 4u32;
pub const AIF_NO_VERSION_DIALOG: u32 = 32u32;
pub const AIF_QUIET: u32 = 536870912u32;
pub const AIF_REPLACEONLY: u32 = 1024u32;
pub const AIF_WARNIFSKIP: u32 = 1u32;
pub const ALINF_BKINSTALL: u32 = 32u32;
pub const ALINF_CHECKBKDATA: u32 = 128u32;
pub const ALINF_DELAYREGISTEROCX: u32 = 512u32;
pub const ALINF_NGCONV: u32 = 8u32;
pub const ALINF_QUIET: u32 = 4u32;
pub const ALINF_ROLLBACK: u32 = 64u32;
pub const ALINF_ROLLBKDOALL: u32 = 256u32;
pub const ALINF_UPDHLPDLLS: u32 = 16u32;
pub type APPLICATION_RECOVERY_CALLBACK = unsafe extern "system" fn(pvparameter: *mut ::std::ffi::c_void) -> u32;
pub const ARSR_NOMESSAGES: u32 = 8u32;
pub const ARSR_REGSECTION: u32 = 128u32;
pub const ARSR_REMOVREGBKDATA: u32 = 4096u32;
pub const ARSR_RESTORE: u32 = 1u32;
pub const ATOM_FLAG_GLOBAL: u32 = 2u32;
pub const AT_ARP: u32 = 640u32;
pub const AT_NULL: u32 = 642u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddDelBackupEntryA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpcszfilelist: Param0, lpcszbackupdir: Param1, lpcszbasename: Param2, dwflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddDelBackupEntryA(lpcszfilelist: super::super::Foundation::PSTR, lpcszbackupdir: super::super::Foundation::PSTR, lpcszbasename: super::super::Foundation::PSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        AddDelBackupEntryA(lpcszfilelist.into_param().abi(), lpcszbackupdir.into_param().abi(), lpcszbasename.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddDelBackupEntryW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpcszfilelist: Param0, lpcszbackupdir: Param1, lpcszbasename: Param2, dwflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddDelBackupEntryW(lpcszfilelist: super::super::Foundation::PWSTR, lpcszbackupdir: super::super::Foundation::PWSTR, lpcszbasename: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        AddDelBackupEntryW(lpcszfilelist.into_param().abi(), lpcszbackupdir.into_param().abi(), lpcszbasename.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AdvInstallFileA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(
    hwnd: Param0,
    lpszsourcedir: Param1,
    lpszsourcefile: Param2,
    lpszdestdir: Param3,
    lpszdestfile: Param4,
    dwflags: u32,
    dwreserved: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AdvInstallFileA(hwnd: super::super::Foundation::HWND, lpszsourcedir: super::super::Foundation::PSTR, lpszsourcefile: super::super::Foundation::PSTR, lpszdestdir: super::super::Foundation::PSTR, lpszdestfile: super::super::Foundation::PSTR, dwflags: u32, dwreserved: u32) -> ::windows::runtime::HRESULT;
        }
        AdvInstallFileA(hwnd.into_param().abi(), lpszsourcedir.into_param().abi(), lpszsourcefile.into_param().abi(), lpszdestdir.into_param().abi(), lpszdestfile.into_param().abi(), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AdvInstallFileW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    hwnd: Param0,
    lpszsourcedir: Param1,
    lpszsourcefile: Param2,
    lpszdestdir: Param3,
    lpszdestfile: Param4,
    dwflags: u32,
    dwreserved: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AdvInstallFileW(hwnd: super::super::Foundation::HWND, lpszsourcedir: super::super::Foundation::PWSTR, lpszsourcefile: super::super::Foundation::PWSTR, lpszdestdir: super::super::Foundation::PWSTR, lpszdestfile: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: u32) -> ::windows::runtime::HRESULT;
        }
        AdvInstallFileW(hwnd.into_param().abi(), lpszsourcedir.into_param().abi(), lpszsourcefile.into_param().abi(), lpszdestdir.into_param().abi(), lpszdestfile.into_param().abi(), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApphelpCheckShellObject<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(objectclsid: *const ::windows::runtime::GUID, bshimifnecessary: Param1, pullflags: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApphelpCheckShellObject(objectclsid: *const ::windows::runtime::GUID, bshimifnecessary: super::super::Foundation::BOOL, pullflags: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ApphelpCheckShellObject(::std::mem::transmute(objectclsid), bshimifnecessary.into_param().abi(), ::std::mem::transmute(pullflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const BACKUP_GHOSTED_FILE_EXTENTS: u32 = 11u32;
pub const BACKUP_INVALID: u32 = 0u32;
pub const BASE_SEARCH_PATH_DISABLE_SAFE_SEARCHMODE: u32 = 65536u32;
pub const BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE: u32 = 1u32;
pub const BASE_SEARCH_PATH_PERMANENT: u32 = 32768u32;
pub const BATTERY_FLAG_CHARGING: u32 = 8u32;
pub const BATTERY_FLAG_CRITICAL: u32 = 4u32;
pub const BATTERY_FLAG_HIGH: u32 = 1u32;
pub const BATTERY_FLAG_LOW: u32 = 2u32;
pub const BATTERY_FLAG_NO_BATTERY: u32 = 128u32;
pub const BATTERY_FLAG_UNKNOWN: u32 = 255u32;
pub const BATTERY_LIFE_UNKNOWN: u32 = 4294967295u32;
pub const BATTERY_PERCENTAGE_UNKNOWN: u32 = 255u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BrowserNavConstants(pub i32);
pub const navOpenInNewWindow: BrowserNavConstants = BrowserNavConstants(1i32);
pub const navNoHistory: BrowserNavConstants = BrowserNavConstants(2i32);
pub const navNoReadFromCache: BrowserNavConstants = BrowserNavConstants(4i32);
pub const navNoWriteToCache: BrowserNavConstants = BrowserNavConstants(8i32);
pub const navAllowAutosearch: BrowserNavConstants = BrowserNavConstants(16i32);
pub const navBrowserBar: BrowserNavConstants = BrowserNavConstants(32i32);
pub const navHyperlink: BrowserNavConstants = BrowserNavConstants(64i32);
pub const navEnforceRestricted: BrowserNavConstants = BrowserNavConstants(128i32);
pub const navNewWindowsManaged: BrowserNavConstants = BrowserNavConstants(256i32);
pub const navUntrustedForDownload: BrowserNavConstants = BrowserNavConstants(512i32);
pub const navTrustedForActiveX: BrowserNavConstants = BrowserNavConstants(1024i32);
pub const navOpenInNewTab: BrowserNavConstants = BrowserNavConstants(2048i32);
pub const navOpenInBackgroundTab: BrowserNavConstants = BrowserNavConstants(4096i32);
pub const navKeepWordWheelText: BrowserNavConstants = BrowserNavConstants(8192i32);
pub const navVirtualTab: BrowserNavConstants = BrowserNavConstants(16384i32);
pub const navBlockRedirectsXDomain: BrowserNavConstants = BrowserNavConstants(32768i32);
pub const navOpenNewForegroundTab: BrowserNavConstants = BrowserNavConstants(65536i32);
pub const navTravelLogScreenshot: BrowserNavConstants = BrowserNavConstants(131072i32);
pub const navDeferUnload: BrowserNavConstants = BrowserNavConstants(262144i32);
pub const navSpeculative: BrowserNavConstants = BrowserNavConstants(524288i32);
pub const navSuggestNewWindow: BrowserNavConstants = BrowserNavConstants(1048576i32);
pub const navSuggestNewTab: BrowserNavConstants = BrowserNavConstants(2097152i32);
pub const navReserved1: BrowserNavConstants = BrowserNavConstants(4194304i32);
pub const navHomepageNavigate: BrowserNavConstants = BrowserNavConstants(8388608i32);
pub const navRefresh: BrowserNavConstants = BrowserNavConstants(16777216i32);
pub const navHostNavigation: BrowserNavConstants = BrowserNavConstants(33554432i32);
pub const navReserved2: BrowserNavConstants = BrowserNavConstants(67108864i32);
pub const navReserved3: BrowserNavConstants = BrowserNavConstants(134217728i32);
pub const navReserved4: BrowserNavConstants = BrowserNavConstants(268435456i32);
pub const navReserved5: BrowserNavConstants = BrowserNavConstants(536870912i32);
pub const navReserved6: BrowserNavConstants = BrowserNavConstants(1073741824i32);
pub const navReserved7: BrowserNavConstants = BrowserNavConstants(-2147483648i32);
impl ::std::convert::From<i32> for BrowserNavConstants {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BrowserNavConstants {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CABINFOA {
    pub pszCab: super::super::Foundation::PSTR,
    pub pszInf: super::super::Foundation::PSTR,
    pub pszSection: super::super::Foundation::PSTR,
    pub szSrcPath: [super::super::Foundation::CHAR; 260],
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CABINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CABINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CABINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CABINFOA").field("pszCab", &self.pszCab).field("pszInf", &self.pszInf).field("pszSection", &self.pszSection).field("szSrcPath", &self.szSrcPath).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CABINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.pszCab == other.pszCab && self.pszInf == other.pszInf && self.pszSection == other.pszSection && self.szSrcPath == other.szSrcPath && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CABINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CABINFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CABINFOW {
    pub pszCab: super::super::Foundation::PWSTR,
    pub pszInf: super::super::Foundation::PWSTR,
    pub pszSection: super::super::Foundation::PWSTR,
    pub szSrcPath: [u16; 260],
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CABINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CABINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CABINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CABINFOW").field("pszCab", &self.pszCab).field("pszInf", &self.pszInf).field("pszSection", &self.pszSection).field("szSrcPath", &self.szSrcPath).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CABINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.pszCab == other.pszCab && self.pszInf == other.pszInf && self.pszSection == other.pszSection && self.szSrcPath == other.szSrcPath && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CABINFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CABINFOW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CATID_DeleteBrowsingHistory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(835385060, 54954, 16528, [160, 80, 165, 172, 137, 114, 233, 239]);
pub const CBR_110: u32 = 110u32;
pub const CBR_115200: u32 = 115200u32;
pub const CBR_1200: u32 = 1200u32;
pub const CBR_128000: u32 = 128000u32;
pub const CBR_14400: u32 = 14400u32;
pub const CBR_19200: u32 = 19200u32;
pub const CBR_2400: u32 = 2400u32;
pub const CBR_256000: u32 = 256000u32;
pub const CBR_300: u32 = 300u32;
pub const CBR_38400: u32 = 38400u32;
pub const CBR_4800: u32 = 4800u32;
pub const CBR_56000: u32 = 56000u32;
pub const CBR_57600: u32 = 57600u32;
pub const CBR_600: u32 = 600u32;
pub const CBR_9600: u32 = 9600u32;
pub const CE_DNS: u32 = 2048u32;
pub const CE_IOE: u32 = 1024u32;
pub const CE_MODE: u32 = 32768u32;
pub const CE_OOP: u32 = 4096u32;
pub const CE_PTO: u32 = 512u32;
pub const CE_TXFULL: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CLIENT_ID {
    pub UniqueProcess: super::super::Foundation::HANDLE,
    pub UniqueThread: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl CLIENT_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CLIENT_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CLIENT_ID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CLIENT_ID").field("UniqueProcess", &self.UniqueProcess).field("UniqueThread", &self.UniqueThread).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CLIENT_ID {
    fn eq(&self, other: &Self) -> bool {
        self.UniqueProcess == other.UniqueProcess && self.UniqueThread == other.UniqueThread
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CLIENT_ID {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CLIENT_ID {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CL_NL_IP: u32 = 771u32;
pub const CL_NL_IPX: u32 = 769u32;
pub const CL_TL_NBF: u32 = 1025u32;
pub const CL_TL_UDP: u32 = 1027u32;
pub const CODEINTEGRITY_OPTION_DEBUGMODE_ENABLED: u32 = 128u32;
pub const CODEINTEGRITY_OPTION_ENABLED: u32 = 1u32;
pub const CODEINTEGRITY_OPTION_FLIGHTING_ENABLED: u32 = 512u32;
pub const CODEINTEGRITY_OPTION_FLIGHT_BUILD: u32 = 256u32;
pub const CODEINTEGRITY_OPTION_HVCI_IUM_ENABLED: u32 = 8192u32;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_AUDITMODE_ENABLED: u32 = 2048u32;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_ENABLED: u32 = 1024u32;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_STRICTMODE_ENABLED: u32 = 4096u32;
pub const CODEINTEGRITY_OPTION_PREPRODUCTION_BUILD: u32 = 64u32;
pub const CODEINTEGRITY_OPTION_TESTSIGN: u32 = 2u32;
pub const CODEINTEGRITY_OPTION_TEST_BUILD: u32 = 32u32;
pub const CODEINTEGRITY_OPTION_UMCI_AUDITMODE_ENABLED: u32 = 8u32;
pub const CODEINTEGRITY_OPTION_UMCI_ENABLED: u32 = 4u32;
pub const CODEINTEGRITY_OPTION_UMCI_EXCLUSIONPATHS_ENABLED: u32 = 16u32;
pub const CONTEXT_SIZE: u32 = 16u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
pub struct COPYFILE2_EXTENDED_PARAMETERS_V2 {
    pub dwSize: u32,
    pub dwCopyFlags: u32,
    pub pfCancel: *mut super::super::Foundation::BOOL,
    pub pProgressRoutine: ::std::option::Option<super::super::Storage::FileSystem::PCOPYFILE2_PROGRESS_ROUTINE>,
    pub pvCallbackContext: *mut ::std::ffi::c_void,
    pub dwCopyFlagsV2: u32,
    pub ioDesiredSize: u32,
    pub ioDesiredRate: u32,
    pub reserved: [*mut ::std::ffi::c_void; 8],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl COPYFILE2_EXTENDED_PARAMETERS_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::default::Default for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::fmt::Debug for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COPYFILE2_EXTENDED_PARAMETERS_V2")
            .field("dwSize", &self.dwSize)
            .field("dwCopyFlags", &self.dwCopyFlags)
            .field("pfCancel", &self.pfCancel)
            .field("pvCallbackContext", &self.pvCallbackContext)
            .field("dwCopyFlagsV2", &self.dwCopyFlagsV2)
            .field("ioDesiredSize", &self.ioDesiredSize)
            .field("ioDesiredRate", &self.ioDesiredRate)
            .field("reserved", &self.reserved)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::cmp::PartialEq for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCopyFlags == other.dwCopyFlags && self.pfCancel == other.pfCancel && self.pProgressRoutine.map(|f| f as usize) == other.pProgressRoutine.map(|f| f as usize) && self.pvCallbackContext == other.pvCallbackContext && self.dwCopyFlagsV2 == other.dwCopyFlagsV2 && self.ioDesiredSize == other.ioDesiredSize && self.ioDesiredRate == other.ioDesiredRate && self.reserved == other.reserved
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::std::cmp::Eq for COPYFILE2_EXTENDED_PARAMETERS_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
unsafe impl ::windows::runtime::Abi for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const COPYFILE2_IO_CYCLE_SIZE_MAX: u32 = 1073741824u32;
pub const COPYFILE2_IO_CYCLE_SIZE_MIN: u32 = 4096u32;
pub const COPYFILE2_IO_RATE_MIN: u32 = 512u32;
pub const COPYFILE2_MESSAGE_COPY_OFFLOAD: i32 = 1i32;
pub const COPY_FILE_ALLOW_DECRYPTED_DESTINATION: u32 = 8u32;
pub const COPY_FILE_COPY_SYMLINK: u32 = 2048u32;
pub const COPY_FILE_DIRECTORY: u32 = 128u32;
pub const COPY_FILE_DISABLE_PRE_ALLOCATION: u32 = 67108864u32;
pub const COPY_FILE_DONT_REQUEST_DEST_WRITE_DAC: u32 = 33554432u32;
pub const COPY_FILE_ENABLE_LOW_FREE_SPACE_MODE: u32 = 134217728u32;
pub const COPY_FILE_FAIL_IF_EXISTS: u32 = 1u32;
pub const COPY_FILE_IGNORE_EDP_BLOCK: u32 = 4194304u32;
pub const COPY_FILE_IGNORE_SOURCE_ENCRYPTION: u32 = 8388608u32;
pub const COPY_FILE_NO_BUFFERING: u32 = 4096u32;
pub const COPY_FILE_NO_OFFLOAD: u32 = 262144u32;
pub const COPY_FILE_OPEN_AND_COPY_REPARSE_POINT: u32 = 2097152u32;
pub const COPY_FILE_OPEN_SOURCE_FOR_WRITE: u32 = 4u32;
pub const COPY_FILE_REQUEST_COMPRESSED_TRAFFIC: u32 = 268435456u32;
pub const COPY_FILE_REQUEST_SECURITY_PRIVILEGES: u32 = 8192u32;
pub const COPY_FILE_RESTARTABLE: u32 = 2u32;
pub const COPY_FILE_RESUME_FROM_PAUSE: u32 = 16384u32;
pub const COPY_FILE_SKIP_ALTERNATE_STREAMS: u32 = 32768u32;
pub const CO_TL_NBF: u32 = 1024u32;
pub const CO_TL_SPP: u32 = 1030u32;
pub const CO_TL_SPX: u32 = 1026u32;
pub const CO_TL_TCP: u32 = 1028u32;
pub const CP_DIRECT: u32 = 2u32;
pub const CP_HWND: u32 = 0u32;
pub const CP_LEVEL: u32 = 3u32;
pub const CP_OPEN: u32 = 1u32;
pub const CREATE_FOR_DIR: u32 = 2u32;
pub const CREATE_FOR_IMPORT: u32 = 1u32;
pub const CRITICAL_SECTION_NO_DEBUG_INFO: u32 = 16777216u32;
pub const CScriptErrorList: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4023390976, 5647, 4562, [187, 46, 0, 128, 95, 247, 239, 202]);
pub const CameraUIControl: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(383099582, 45509, 18355, [142, 174, 204, 188, 244, 82, 199, 232]);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraUIControlCaptureMode(pub i32);
impl CameraUIControlCaptureMode {
    pub const PhotoOrVideo: CameraUIControlCaptureMode = CameraUIControlCaptureMode(0i32);
    pub const Photo: CameraUIControlCaptureMode = CameraUIControlCaptureMode(1i32);
    pub const Video: CameraUIControlCaptureMode = CameraUIControlCaptureMode(2i32);
}
impl ::std::convert::From<i32> for CameraUIControlCaptureMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CameraUIControlCaptureMode {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraUIControlLinearSelectionMode(pub i32);
impl CameraUIControlLinearSelectionMode {
    pub const Single: CameraUIControlLinearSelectionMode = CameraUIControlLinearSelectionMode(0i32);
    pub const Multiple: CameraUIControlLinearSelectionMode = CameraUIControlLinearSelectionMode(1i32);
}
impl ::std::convert::From<i32> for CameraUIControlLinearSelectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CameraUIControlLinearSelectionMode {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraUIControlMode(pub i32);
impl CameraUIControlMode {
    pub const Browse: CameraUIControlMode = CameraUIControlMode(0i32);
    pub const Linear: CameraUIControlMode = CameraUIControlMode(1i32);
}
impl ::std::convert::From<i32> for CameraUIControlMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CameraUIControlMode {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraUIControlPhotoFormat(pub i32);
impl CameraUIControlPhotoFormat {
    pub const Jpeg: CameraUIControlPhotoFormat = CameraUIControlPhotoFormat(0i32);
    pub const Png: CameraUIControlPhotoFormat = CameraUIControlPhotoFormat(1i32);
    pub const JpegXR: CameraUIControlPhotoFormat = CameraUIControlPhotoFormat(2i32);
}
impl ::std::convert::From<i32> for CameraUIControlPhotoFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CameraUIControlPhotoFormat {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraUIControlVideoFormat(pub i32);
impl CameraUIControlVideoFormat {
    pub const Mp4: CameraUIControlVideoFormat = CameraUIControlVideoFormat(0i32);
    pub const Wmv: CameraUIControlVideoFormat = CameraUIControlVideoFormat(1i32);
}
impl ::std::convert::From<i32> for CameraUIControlVideoFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CameraUIControlVideoFormat {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CameraUIControlViewType(pub i32);
impl CameraUIControlViewType {
    pub const SingleItem: CameraUIControlViewType = CameraUIControlViewType(0i32);
    pub const ItemList: CameraUIControlViewType = CameraUIControlViewType(1i32);
}
impl ::std::convert::From<i32> for CameraUIControlViewType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CameraUIControlViewType {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelDeviceWakeupRequest<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hdevice: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelDeviceWakeupRequest(hdevice: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CancelDeviceWakeupRequest(hdevice.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelTimerQueueTimer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(timerqueue: Param0, timer: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelTimerQueueTimer(timerqueue: super::super::Foundation::HANDLE, timer: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CancelTimerQueueTimer(timerqueue.into_param().abi(), timer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn CloseINFEngine(hinf: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseINFEngine(hinf: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CloseINFEngine(::std::mem::transmute(hinf)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CommandStateChangeConstants(pub i32);
pub const CSC_UPDATECOMMANDS: CommandStateChangeConstants = CommandStateChangeConstants(-1i32);
pub const CSC_NAVIGATEFORWARD: CommandStateChangeConstants = CommandStateChangeConstants(1i32);
pub const CSC_NAVIGATEBACK: CommandStateChangeConstants = CommandStateChangeConstants(2i32);
impl ::std::convert::From<i32> for CommandStateChangeConstants {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CommandStateChangeConstants {
    type Abi = Self;
    type DefaultType = Self;
}
#[inline]
pub unsafe fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue: u64, lpperformancecountervalue: *mut u64, lpconversionerror: *mut u64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertAuxiliaryCounterToPerformanceCounter(ullauxiliarycountervalue: u64, lpperformancecountervalue: *mut u64, lpconversionerror: *mut u64) -> ::windows::runtime::HRESULT;
        }
        ConvertAuxiliaryCounterToPerformanceCounter(::std::mem::transmute(ullauxiliarycountervalue), ::std::mem::transmute(lpperformancecountervalue), ::std::mem::transmute(lpconversionerror)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue: u64, lpauxiliarycountervalue: *mut u64, lpconversionerror: *mut u64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertPerformanceCounterToAuxiliaryCounter(ullperformancecountervalue: u64, lpauxiliarycountervalue: *mut u64, lpconversionerror: *mut u64) -> ::windows::runtime::HRESULT;
        }
        ConvertPerformanceCounterToAuxiliaryCounter(::std::mem::transmute(ullperformancecountervalue), ::std::mem::transmute(lpauxiliarycountervalue), ::std::mem::transmute(lpconversionerror)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn CreateJobSet(numjob: u32, userjobset: *const super::SystemServices::JOB_SET_ARRAY, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateJobSet(numjob: u32, userjobset: *const super::SystemServices::JOB_SET_ARRAY, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CreateJobSet(::std::mem::transmute(numjob), ::std::mem::transmute(userjobset), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWaitableTimerA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: Param1, lptimername: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateWaitableTimerA(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, lptimername: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateWaitableTimerA(::std::mem::transmute(lptimerattributes), bmanualreset.into_param().abi(), lptimername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWaitableTimerExA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lptimername: Param1, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateWaitableTimerExA(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lptimername: super::super::Foundation::PSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateWaitableTimerExA(::std::mem::transmute(lptimerattributes), lptimername.into_param().abi(), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DATETIME {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub min: u16,
    pub sec: u16,
}
impl DATETIME {}
impl ::std::default::Default for DATETIME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DATETIME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DATETIME").field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("min", &self.min).field("sec", &self.sec).finish()
    }
}
impl ::std::cmp::PartialEq for DATETIME {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day && self.hour == other.hour && self.min == other.min && self.sec == other.sec
    }
}
impl ::std::cmp::Eq for DATETIME {}
unsafe impl ::windows::runtime::Abi for DATETIME {
    type Abi = Self;
    type DefaultType = Self;
}
#[inline]
pub unsafe fn DCIBeginAccess(pdci: *mut DCISURFACEINFO, x: i32, y: i32, dx: i32, dy: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCIBeginAccess(pdci: *mut DCISURFACEINFO, x: i32, y: i32, dx: i32, dy: i32) -> i32;
        }
        ::std::mem::transmute(DCIBeginAccess(::std::mem::transmute(pdci), ::std::mem::transmute(x), ::std::mem::transmute(y), ::std::mem::transmute(dx), ::std::mem::transmute(dy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DCICMD {
    pub dwCommand: u32,
    pub dwParam1: u32,
    pub dwParam2: u32,
    pub dwVersion: u32,
    pub dwReserved: u32,
}
impl DCICMD {}
impl ::std::default::Default for DCICMD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DCICMD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DCICMD").field("dwCommand", &self.dwCommand).field("dwParam1", &self.dwParam1).field("dwParam2", &self.dwParam2).field("dwVersion", &self.dwVersion).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::std::cmp::PartialEq for DCICMD {
    fn eq(&self, other: &Self) -> bool {
        self.dwCommand == other.dwCommand && self.dwParam1 == other.dwParam1 && self.dwParam2 == other.dwParam2 && self.dwVersion == other.dwVersion && self.dwReserved == other.dwReserved
    }
}
impl ::std::cmp::Eq for DCICMD {}
unsafe impl ::windows::runtime::Abi for DCICMD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DCICREATEINPUT {
    pub cmd: DCICMD,
    pub dwCompression: u32,
    pub dwMask: [u32; 3],
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwDCICaps: u32,
    pub dwBitCount: u32,
    pub lpSurface: *mut ::std::ffi::c_void,
}
impl DCICREATEINPUT {}
impl ::std::default::Default for DCICREATEINPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DCICREATEINPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DCICREATEINPUT")
            .field("cmd", &self.cmd)
            .field("dwCompression", &self.dwCompression)
            .field("dwMask", &self.dwMask)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("dwDCICaps", &self.dwDCICaps)
            .field("dwBitCount", &self.dwBitCount)
            .field("lpSurface", &self.lpSurface)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DCICREATEINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.cmd == other.cmd && self.dwCompression == other.dwCompression && self.dwMask == other.dwMask && self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.dwDCICaps == other.dwDCICaps && self.dwBitCount == other.dwBitCount && self.lpSurface == other.lpSurface
    }
}
impl ::std::cmp::Eq for DCICREATEINPUT {}
unsafe impl ::windows::runtime::Abi for DCICREATEINPUT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DCICREATEOFFSCREENSURFACE: u32 = 2u32;
pub const DCICREATEOVERLAYSURFACE: u32 = 3u32;
pub const DCICREATEPRIMARYSURFACE: u32 = 1u32;
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICloseProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCICloseProvider(hdc: super::super::Graphics::Gdi::HDC);
        }
        ::std::mem::transmute(DCICloseProvider(hdc.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICreateOffscreen<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, dwcompression: u32, dwredmask: u32, dwgreenmask: u32, dwbluemask: u32, dwwidth: u32, dwheight: u32, dwdcicaps: u32, dwbitcount: u32, lplpsurface: *mut *mut DCIOFFSCREEN) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCICreateOffscreen(hdc: super::super::Graphics::Gdi::HDC, dwcompression: u32, dwredmask: u32, dwgreenmask: u32, dwbluemask: u32, dwwidth: u32, dwheight: u32, dwdcicaps: u32, dwbitcount: u32, lplpsurface: *mut *mut DCIOFFSCREEN) -> i32;
        }
        ::std::mem::transmute(DCICreateOffscreen(
            hdc.into_param().abi(),
            ::std::mem::transmute(dwcompression),
            ::std::mem::transmute(dwredmask),
            ::std::mem::transmute(dwgreenmask),
            ::std::mem::transmute(dwbluemask),
            ::std::mem::transmute(dwwidth),
            ::std::mem::transmute(dwheight),
            ::std::mem::transmute(dwdcicaps),
            ::std::mem::transmute(dwbitcount),
            ::std::mem::transmute(lplpsurface),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICreateOverlay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, lpoffscreensurf: *mut ::std::ffi::c_void, lplpsurface: *mut *mut DCIOVERLAY) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCICreateOverlay(hdc: super::super::Graphics::Gdi::HDC, lpoffscreensurf: *mut ::std::ffi::c_void, lplpsurface: *mut *mut DCIOVERLAY) -> i32;
        }
        ::std::mem::transmute(DCICreateOverlay(hdc.into_param().abi(), ::std::mem::transmute(lpoffscreensurf), ::std::mem::transmute(lplpsurface)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCICreatePrimary<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, lplpsurface: *mut *mut DCISURFACEINFO) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCICreatePrimary(hdc: super::super::Graphics::Gdi::HDC, lplpsurface: *mut *mut DCISURFACEINFO) -> i32;
        }
        ::std::mem::transmute(DCICreatePrimary(hdc.into_param().abi(), ::std::mem::transmute(lplpsurface)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DCIDestroy(pdci: *mut DCISURFACEINFO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCIDestroy(pdci: *mut DCISURFACEINFO);
        }
        ::std::mem::transmute(DCIDestroy(::std::mem::transmute(pdci)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn DCIDraw(pdci: *mut DCIOFFSCREEN) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCIDraw(pdci: *mut DCIOFFSCREEN) -> i32;
        }
        ::std::mem::transmute(DCIDraw(::std::mem::transmute(pdci)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DCIENUMINPUT {
    pub cmd: DCICMD,
    pub rSrc: super::super::Foundation::RECT,
    pub rDst: super::super::Foundation::RECT,
    pub EnumCallback: isize,
    pub lpContext: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl DCIENUMINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DCIENUMINPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DCIENUMINPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DCIENUMINPUT").field("cmd", &self.cmd).field("rSrc", &self.rSrc).field("rDst", &self.rDst).field("EnumCallback", &self.EnumCallback).field("lpContext", &self.lpContext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DCIENUMINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.cmd == other.cmd && self.rSrc == other.rSrc && self.rDst == other.rDst && self.EnumCallback == other.EnumCallback && self.lpContext == other.lpContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DCIENUMINPUT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DCIENUMINPUT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DCIENUMSURFACE: u32 = 4u32;
pub const DCIESCAPE: u32 = 5u32;
#[inline]
pub unsafe fn DCIEndAccess(pdci: *mut DCISURFACEINFO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCIEndAccess(pdci: *mut DCISURFACEINFO);
        }
        ::std::mem::transmute(DCIEndAccess(::std::mem::transmute(pdci)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DCIEnum<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, lprdst: *mut super::super::Foundation::RECT, lprsrc: *mut super::super::Foundation::RECT, lpfncallback: *mut ::std::ffi::c_void, lpcontext: *mut ::std::ffi::c_void) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCIEnum(hdc: super::super::Graphics::Gdi::HDC, lprdst: *mut super::super::Foundation::RECT, lprsrc: *mut super::super::Foundation::RECT, lpfncallback: *mut ::std::ffi::c_void, lpcontext: *mut ::std::ffi::c_void) -> i32;
        }
        ::std::mem::transmute(DCIEnum(hdc.into_param().abi(), ::std::mem::transmute(lprdst), ::std::mem::transmute(lprsrc), ::std::mem::transmute(lpfncallback), ::std::mem::transmute(lpcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DCIOFFSCREEN {
    pub dciInfo: DCISURFACEINFO,
    pub Draw: isize,
    pub SetClipList: isize,
    pub SetDestination: isize,
}
impl DCIOFFSCREEN {}
impl ::std::default::Default for DCIOFFSCREEN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DCIOFFSCREEN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DCIOFFSCREEN").field("dciInfo", &self.dciInfo).field("Draw", &self.Draw).field("SetClipList", &self.SetClipList).field("SetDestination", &self.SetDestination).finish()
    }
}
impl ::std::cmp::PartialEq for DCIOFFSCREEN {
    fn eq(&self, other: &Self) -> bool {
        self.dciInfo == other.dciInfo && self.Draw == other.Draw && self.SetClipList == other.SetClipList && self.SetDestination == other.SetDestination
    }
}
impl ::std::cmp::Eq for DCIOFFSCREEN {}
unsafe impl ::windows::runtime::Abi for DCIOFFSCREEN {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DCIOVERLAY {
    pub dciInfo: DCISURFACEINFO,
    pub dwChromakeyValue: u32,
    pub dwChromakeyMask: u32,
}
impl DCIOVERLAY {}
impl ::std::default::Default for DCIOVERLAY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DCIOVERLAY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DCIOVERLAY").field("dciInfo", &self.dciInfo).field("dwChromakeyValue", &self.dwChromakeyValue).field("dwChromakeyMask", &self.dwChromakeyMask).finish()
    }
}
impl ::std::cmp::PartialEq for DCIOVERLAY {
    fn eq(&self, other: &Self) -> bool {
        self.dciInfo == other.dciInfo && self.dwChromakeyValue == other.dwChromakeyValue && self.dwChromakeyMask == other.dwChromakeyMask
    }
}
impl ::std::cmp::Eq for DCIOVERLAY {}
unsafe impl ::windows::runtime::Abi for DCIOVERLAY {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn DCIOpenProvider() -> super::super::Graphics::Gdi::HDC {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCIOpenProvider() -> super::super::Graphics::Gdi::HDC;
        }
        ::std::mem::transmute(DCIOpenProvider())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DCISURFACEINFO {
    pub dwSize: u32,
    pub dwDCICaps: u32,
    pub dwCompression: u32,
    pub dwMask: [u32; 3],
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub lStride: i32,
    pub dwBitCount: u32,
    pub dwOffSurface: usize,
    pub wSelSurface: u16,
    pub wReserved: u16,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub BeginAccess: isize,
    pub EndAccess: isize,
    pub DestroySurface: isize,
}
impl DCISURFACEINFO {}
impl ::std::default::Default for DCISURFACEINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DCISURFACEINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DCISURFACEINFO")
            .field("dwSize", &self.dwSize)
            .field("dwDCICaps", &self.dwDCICaps)
            .field("dwCompression", &self.dwCompression)
            .field("dwMask", &self.dwMask)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("lStride", &self.lStride)
            .field("dwBitCount", &self.dwBitCount)
            .field("dwOffSurface", &self.dwOffSurface)
            .field("wSelSurface", &self.wSelSurface)
            .field("wReserved", &self.wReserved)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("BeginAccess", &self.BeginAccess)
            .field("EndAccess", &self.EndAccess)
            .field("DestroySurface", &self.DestroySurface)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DCISURFACEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwDCICaps == other.dwDCICaps
            && self.dwCompression == other.dwCompression
            && self.dwMask == other.dwMask
            && self.dwWidth == other.dwWidth
            && self.dwHeight == other.dwHeight
            && self.lStride == other.lStride
            && self.dwBitCount == other.dwBitCount
            && self.dwOffSurface == other.dwOffSurface
            && self.wSelSurface == other.wSelSurface
            && self.wReserved == other.wReserved
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwReserved3 == other.dwReserved3
            && self.BeginAccess == other.BeginAccess
            && self.EndAccess == other.EndAccess
            && self.DestroySurface == other.DestroySurface
    }
}
impl ::std::cmp::Eq for DCISURFACEINFO {}
unsafe impl ::windows::runtime::Abi for DCISURFACEINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DCISetClipList(pdci: *mut DCIOFFSCREEN, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCISetClipList(pdci: *mut DCIOFFSCREEN, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32;
        }
        ::std::mem::transmute(DCISetClipList(::std::mem::transmute(pdci), ::std::mem::transmute(prd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCISetDestination(pdci: *mut DCIOFFSCREEN, dst: *mut super::super::Foundation::RECT, src: *mut super::super::Foundation::RECT) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCISetDestination(pdci: *mut DCIOFFSCREEN, dst: *mut super::super::Foundation::RECT, src: *mut super::super::Foundation::RECT) -> i32;
        }
        ::std::mem::transmute(DCISetDestination(::std::mem::transmute(pdci), ::std::mem::transmute(dst), ::std::mem::transmute(src)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DCISetSrcDestClip(pdci: *mut DCIOFFSCREEN, srcrc: *mut super::super::Foundation::RECT, destrc: *mut super::super::Foundation::RECT, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCISetSrcDestClip(pdci: *mut DCIOFFSCREEN, srcrc: *mut super::super::Foundation::RECT, destrc: *mut super::super::Foundation::RECT, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> i32;
        }
        ::std::mem::transmute(DCISetSrcDestClip(::std::mem::transmute(pdci), ::std::mem::transmute(srcrc), ::std::mem::transmute(destrc), ::std::mem::transmute(prd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DCI_1632_ACCESS: u32 = 64u32;
pub const DCI_ASYNC: u32 = 1024u32;
pub const DCI_CANOVERLAY: u32 = 65536u32;
pub const DCI_CAN_STRETCHX: u32 = 4096u32;
pub const DCI_CAN_STRETCHXN: u32 = 16384u32;
pub const DCI_CAN_STRETCHY: u32 = 8192u32;
pub const DCI_CAN_STRETCHYN: u32 = 32768u32;
pub const DCI_CHROMAKEY: u32 = 32u32;
pub const DCI_DWORDALIGN: u32 = 256u32;
pub const DCI_DWORDSIZE: u32 = 128u32;
pub const DCI_ERR_CURRENTLYNOTAVAIL: i32 = -5i32;
pub const DCI_ERR_HEIGHTALIGN: i32 = -21i32;
pub const DCI_ERR_INVALIDCLIPLIST: i32 = -15i32;
pub const DCI_ERR_INVALIDPOSITION: i32 = -13i32;
pub const DCI_ERR_INVALIDRECT: i32 = -6i32;
pub const DCI_ERR_INVALIDSTRETCH: i32 = -14i32;
pub const DCI_ERR_OUTOFMEMORY: i32 = -12i32;
pub const DCI_ERR_SURFACEISOBSCURED: i32 = -16i32;
pub const DCI_ERR_TOOBIGHEIGHT: i32 = -9i32;
pub const DCI_ERR_TOOBIGSIZE: i32 = -11i32;
pub const DCI_ERR_TOOBIGWIDTH: i32 = -10i32;
pub const DCI_ERR_UNSUPPORTEDFORMAT: i32 = -7i32;
pub const DCI_ERR_UNSUPPORTEDMASK: i32 = -8i32;
pub const DCI_ERR_WIDTHALIGN: i32 = -20i32;
pub const DCI_ERR_XALIGN: i32 = -17i32;
pub const DCI_ERR_XYALIGN: i32 = -19i32;
pub const DCI_ERR_YALIGN: i32 = -18i32;
pub const DCI_FAIL_GENERIC: i32 = -1i32;
pub const DCI_FAIL_INVALIDSURFACE: i32 = -3i32;
pub const DCI_FAIL_UNSUPPORTED: i32 = -4i32;
pub const DCI_FAIL_UNSUPPORTEDVERSION: i32 = -2i32;
pub const DCI_OFFSCREEN: u32 = 1u32;
pub const DCI_OK: u32 = 0u32;
pub const DCI_OVERLAY: u32 = 2u32;
pub const DCI_PRIMARY: u32 = 0u32;
pub const DCI_STATUS_CHROMAKEYCHANGED: u32 = 16u32;
pub const DCI_STATUS_FORMATCHANGED: u32 = 4u32;
pub const DCI_STATUS_POINTERCHANGED: u32 = 1u32;
pub const DCI_STATUS_STRIDECHANGED: u32 = 2u32;
pub const DCI_STATUS_SURFACEINFOCHANGED: u32 = 8u32;
pub const DCI_STATUS_WASSTILLDRAWING: u32 = 32u32;
pub const DCI_SURFACE_TYPE: u32 = 15u32;
pub const DCI_VERSION: u32 = 256u32;
pub const DCI_VISIBLE: u32 = 16u32;
pub const DCI_WRITEONLY: u32 = 512u32;
pub const DDIRQ_RESERVED1: i32 = 2i32;
pub const DDKERNELCAPS_AUTOFLIP: i32 = 2i32;
pub const DDKERNELCAPS_CAPTURE_INVERTED: i32 = 512i32;
pub const DDKERNELCAPS_CAPTURE_NONLOCALVIDMEM: i32 = 128i32;
pub const DDKERNELCAPS_CAPTURE_SYSMEM: i32 = 64i32;
pub const DDKERNELCAPS_FIELDPOLARITY: i32 = 256i32;
pub const DDKERNELCAPS_FLIPOVERLAY: i32 = 32i32;
pub const DDKERNELCAPS_FLIPVIDEOPORT: i32 = 16i32;
pub const DDKERNELCAPS_LOCK: i32 = 8i32;
pub const DDKERNELCAPS_SETSTATE: i32 = 4i32;
pub const DDKERNELCAPS_SKIPFIELDS: i32 = 1i32;
pub const DEACTIVATE_ACTCTX_FLAG_FORCE_EARLY_DEACTIVATION: u32 = 1u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DECISION_LOCATION(pub i32);
pub const DECISION_LOCATION_REFRESH_GLOBAL_DATA: DECISION_LOCATION = DECISION_LOCATION(0i32);
pub const DECISION_LOCATION_PARAMETER_VALIDATION: DECISION_LOCATION = DECISION_LOCATION(1i32);
pub const DECISION_LOCATION_AUDIT: DECISION_LOCATION = DECISION_LOCATION(2i32);
pub const DECISION_LOCATION_FAILED_CONVERT_GUID: DECISION_LOCATION = DECISION_LOCATION(3i32);
pub const DECISION_LOCATION_ENTERPRISE_DEFINED_CLASS_ID: DECISION_LOCATION = DECISION_LOCATION(4i32);
pub const DECISION_LOCATION_GLOBAL_BUILT_IN_LIST: DECISION_LOCATION = DECISION_LOCATION(5i32);
pub const DECISION_LOCATION_PROVIDER_BUILT_IN_LIST: DECISION_LOCATION = DECISION_LOCATION(6i32);
pub const DECISION_LOCATION_ENFORCE_STATE_LIST: DECISION_LOCATION = DECISION_LOCATION(7i32);
pub const DECISION_LOCATION_NOT_FOUND: DECISION_LOCATION = DECISION_LOCATION(8i32);
pub const DECISION_LOCATION_UNKNOWN: DECISION_LOCATION = DECISION_LOCATION(9i32);
impl ::std::convert::From<i32> for DECISION_LOCATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DECISION_LOCATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DELAYLOAD_GPA_FAILURE: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct DELAYLOAD_INFO {
    pub Size: u32,
    pub DelayloadDescriptor: *mut super::SystemServices::IMAGE_DELAYLOAD_DESCRIPTOR,
    pub ThunkAddress: *mut super::SystemServices::IMAGE_THUNK_DATA64,
    pub TargetDllName: super::super::Foundation::PSTR,
    pub TargetApiDescriptor: DELAYLOAD_PROC_DESCRIPTOR,
    pub TargetModuleBase: *mut ::std::ffi::c_void,
    pub Unused: *mut ::std::ffi::c_void,
    pub LastError: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl DELAYLOAD_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for DELAYLOAD_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for DELAYLOAD_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for DELAYLOAD_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for DELAYLOAD_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DELAYLOAD_PROC_DESCRIPTOR {
    pub ImportDescribedByName: u32,
    pub Description: DELAYLOAD_PROC_DESCRIPTOR_0,
}
#[cfg(feature = "Win32_Foundation")]
impl DELAYLOAD_PROC_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DELAYLOAD_PROC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DELAYLOAD_PROC_DESCRIPTOR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DELAYLOAD_PROC_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DELAYLOAD_PROC_DESCRIPTOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union DELAYLOAD_PROC_DESCRIPTOR_0 {
    pub Name: super::super::Foundation::PSTR,
    pub Ordinal: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl DELAYLOAD_PROC_DESCRIPTOR_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DELAYLOAD_PROC_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DELAYLOAD_PROC_DESCRIPTOR_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DELAYLOAD_PROC_DESCRIPTOR_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DELAYLOAD_PROC_DESCRIPTOR_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DELETE_BROWSING_HISTORY_COOKIES: u32 = 2u32;
pub const DELETE_BROWSING_HISTORY_DOWNLOADHISTORY: u32 = 64u32;
pub const DELETE_BROWSING_HISTORY_FORMDATA: u32 = 8u32;
pub const DELETE_BROWSING_HISTORY_HISTORY: u32 = 1u32;
pub const DELETE_BROWSING_HISTORY_PASSWORDS: u32 = 16u32;
pub const DELETE_BROWSING_HISTORY_PRESERVEFAVORITES: u32 = 32u32;
pub const DELETE_BROWSING_HISTORY_TIF: u32 = 4u32;
pub const DOCKINFO_DOCKED: u32 = 2u32;
pub const DOCKINFO_UNDOCKED: u32 = 1u32;
pub const DOCKINFO_USER_SUPPLIED: u32 = 4u32;
pub const DRIVE_CDROM: u32 = 5u32;
pub const DRIVE_FIXED: u32 = 3u32;
pub const DRIVE_NO_ROOT_DIR: u32 = 1u32;
pub const DRIVE_RAMDISK: u32 = 6u32;
pub const DRIVE_REMOTE: u32 = 4u32;
pub const DRIVE_REMOVABLE: u32 = 2u32;
pub const DRIVE_UNKNOWN: u32 = 0u32;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DShellNameSpaceEvents(::windows::runtime::IUnknown);
impl DShellNameSpaceEvents {}
unsafe impl ::windows::runtime::Interface for DShellNameSpaceEvents {
    type Vtable = DShellNameSpaceEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1427335174, 45790, 4561, [185, 242, 0, 160, 201, 139, 197, 71]);
}
impl ::std::convert::From<DShellNameSpaceEvents> for ::windows::runtime::IUnknown {
    fn from(value: DShellNameSpaceEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DShellNameSpaceEvents> for ::windows::runtime::IUnknown {
    fn from(value: &DShellNameSpaceEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DShellNameSpaceEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DShellNameSpaceEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<DShellNameSpaceEvents> for super::Ole::Automation::IDispatch {
    fn from(value: DShellNameSpaceEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&DShellNameSpaceEvents> for super::Ole::Automation::IDispatch {
    fn from(value: &DShellNameSpaceEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for DShellNameSpaceEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &DShellNameSpaceEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DShellNameSpaceEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DShellWindowsEvents(::windows::runtime::IUnknown);
impl DShellWindowsEvents {}
unsafe impl ::windows::runtime::Interface for DShellWindowsEvents {
    type Vtable = DShellWindowsEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4265674464, 14746, 4560, [164, 140, 0, 160, 201, 10, 143, 57]);
}
impl ::std::convert::From<DShellWindowsEvents> for ::windows::runtime::IUnknown {
    fn from(value: DShellWindowsEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DShellWindowsEvents> for ::windows::runtime::IUnknown {
    fn from(value: &DShellWindowsEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DShellWindowsEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DShellWindowsEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<DShellWindowsEvents> for super::Ole::Automation::IDispatch {
    fn from(value: DShellWindowsEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&DShellWindowsEvents> for super::Ole::Automation::IDispatch {
    fn from(value: &DShellWindowsEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for DShellWindowsEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &DShellWindowsEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DShellWindowsEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
pub const DTR_CONTROL_DISABLE: u32 = 0u32;
pub const DTR_CONTROL_ENABLE: u32 = 1u32;
pub const DTR_CONTROL_HANDSHAKE: u32 = 2u32;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DWebBrowserEvents(::windows::runtime::IUnknown);
impl DWebBrowserEvents {}
unsafe impl ::windows::runtime::Interface for DWebBrowserEvents {
    type Vtable = DWebBrowserEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3937544898, 12481, 4559, [167, 235, 0, 0, 192, 91, 174, 11]);
}
impl ::std::convert::From<DWebBrowserEvents> for ::windows::runtime::IUnknown {
    fn from(value: DWebBrowserEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DWebBrowserEvents> for ::windows::runtime::IUnknown {
    fn from(value: &DWebBrowserEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DWebBrowserEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DWebBrowserEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<DWebBrowserEvents> for super::Ole::Automation::IDispatch {
    fn from(value: DWebBrowserEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&DWebBrowserEvents> for super::Ole::Automation::IDispatch {
    fn from(value: &DWebBrowserEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for DWebBrowserEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &DWebBrowserEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DWebBrowserEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DWebBrowserEvents2(::windows::runtime::IUnknown);
impl DWebBrowserEvents2 {}
unsafe impl ::windows::runtime::Interface for DWebBrowserEvents2 {
    type Vtable = DWebBrowserEvents2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(883365280, 25991, 4560, [146, 74, 0, 32, 175, 199, 172, 77]);
}
impl ::std::convert::From<DWebBrowserEvents2> for ::windows::runtime::IUnknown {
    fn from(value: DWebBrowserEvents2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DWebBrowserEvents2> for ::windows::runtime::IUnknown {
    fn from(value: &DWebBrowserEvents2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DWebBrowserEvents2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DWebBrowserEvents2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<DWebBrowserEvents2> for super::Ole::Automation::IDispatch {
    fn from(value: DWebBrowserEvents2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&DWebBrowserEvents2> for super::Ole::Automation::IDispatch {
    fn from(value: &DWebBrowserEvents2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for DWebBrowserEvents2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &DWebBrowserEvents2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DWebBrowserEvents2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
pub const DefaultBrowserSyncSettings: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(986199075, 12562, 19110, [155, 91, 31, 235, 35, 208, 197, 249]);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DelNodeA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pszfileordirname: Param0, dwflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DelNodeA(pszfileordirname: super::super::Foundation::PSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        DelNodeA(pszfileordirname.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DelNodeRunDLL32W<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hwnd: Param0, hinstance: Param1, pszparms: Param2, nshow: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DelNodeRunDLL32W(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PWSTR, nshow: i32) -> ::windows::runtime::HRESULT;
        }
        DelNodeRunDLL32W(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), ::std::mem::transmute(nshow)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DelNodeW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszfileordirname: Param0, dwflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DelNodeW(pszfileordirname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        DelNodeW(pszfileordirname.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DnsHostnameToComputerNameA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hostname: Param0, computername: super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DnsHostnameToComputerNameA(hostname: super::super::Foundation::PSTR, computername: super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DnsHostnameToComputerNameA(hostname.into_param().abi(), ::std::mem::transmute(computername), ::std::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DnsHostnameToComputerNameW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hostname: Param0, computername: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DnsHostnameToComputerNameW(hostname: super::super::Foundation::PWSTR, computername: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DnsHostnameToComputerNameW(hostname.into_param().abi(), ::std::mem::transmute(computername), ::std::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DosDateTimeToFileTime(wfatdate: u16, wfattime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DosDateTimeToFileTime(wfatdate: u16, wfattime: u16, lpfiletime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DosDateTimeToFileTime(::std::mem::transmute(wfatdate), ::std::mem::transmute(wfattime), ::std::mem::transmute(lpfiletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const EFSRPC_SECURE_ONLY: u32 = 8u32;
pub const EFS_DROP_ALTERNATE_STREAMS: u32 = 16u32;
pub const EFS_USE_RECOVERY_KEYS: u32 = 1u32;
pub const ENTITY_LIST_ID: u32 = 0u32;
pub const ENTITY_TYPE_ID: u32 = 1u32;
pub type ENUM_CALLBACK = unsafe extern "system" fn(lpsurfaceinfo: *mut DCISURFACEINFO, lpcontext: *mut ::std::ffi::c_void);
pub const ER_ICMP: u32 = 896u32;
pub const EVENPARITY: u32 = 2u32;
pub const EVENTLOG_FULL_INFO: u32 = 0u32;
pub const EditionUpgradeBroker: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3290892327, 20281, 17887, [146, 136, 18, 255, 107, 133, 169, 33]);
pub const EditionUpgradeHelper: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(24604147, 47535, 20048, [155, 28, 86, 233, 49, 22, 215, 4]);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableProcessOptionalXStateFeatures(features: u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableProcessOptionalXStateFeatures(features: u64) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnableProcessOptionalXStateFeatures(::std::mem::transmute(features)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExecuteCabA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, pcab: *mut CABINFOA, preserved: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExecuteCabA(hwnd: super::super::Foundation::HWND, pcab: *mut CABINFOA, preserved: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ExecuteCabA(hwnd.into_param().abi(), ::std::mem::transmute(pcab), ::std::mem::transmute(preserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExecuteCabW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, pcab: *mut CABINFOW, preserved: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExecuteCabW(hwnd: super::super::Foundation::HWND, pcab: *mut CABINFOW, preserved: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        ExecuteCabW(hwnd.into_param().abi(), ::std::mem::transmute(pcab), ::std::mem::transmute(preserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtractFilesA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pszcabname: Param0, pszexpanddir: Param1, dwflags: u32, pszfilelist: Param3, lpreserved: *mut ::std::ffi::c_void, dwreserved: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExtractFilesA(pszcabname: super::super::Foundation::PSTR, pszexpanddir: super::super::Foundation::PSTR, dwflags: u32, pszfilelist: super::super::Foundation::PSTR, lpreserved: *mut ::std::ffi::c_void, dwreserved: u32) -> ::windows::runtime::HRESULT;
        }
        ExtractFilesA(pszcabname.into_param().abi(), pszexpanddir.into_param().abi(), ::std::mem::transmute(dwflags), pszfilelist.into_param().abi(), ::std::mem::transmute(lpreserved), ::std::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExtractFilesW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszcabname: Param0, pszexpanddir: Param1, dwflags: u32, pszfilelist: Param3, lpreserved: *mut ::std::ffi::c_void, dwreserved: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExtractFilesW(pszcabname: super::super::Foundation::PWSTR, pszexpanddir: super::super::Foundation::PWSTR, dwflags: u32, pszfilelist: super::super::Foundation::PWSTR, lpreserved: *mut ::std::ffi::c_void, dwreserved: u32) -> ::windows::runtime::HRESULT;
        }
        ExtractFilesW(pszcabname.into_param().abi(), pszexpanddir.into_param().abi(), ::std::mem::transmute(dwflags), pszfilelist.into_param().abi(), ::std::mem::transmute(lpreserved), ::std::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FAIL_FAST_GENERATE_EXCEPTION_ADDRESS: u32 = 1u32;
pub const FAIL_FAST_NO_HARD_ERROR_DLG: u32 = 2u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FEATURE_CHANGE_TIME(pub i32);
pub const FEATURE_CHANGE_TIME_READ: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(0i32);
pub const FEATURE_CHANGE_TIME_MODULE_RELOAD: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(1i32);
pub const FEATURE_CHANGE_TIME_SESSION: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(2i32);
pub const FEATURE_CHANGE_TIME_REBOOT: FEATURE_CHANGE_TIME = FEATURE_CHANGE_TIME(3i32);
impl ::std::convert::From<i32> for FEATURE_CHANGE_TIME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FEATURE_CHANGE_TIME {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FEATURE_ENABLED_STATE(pub i32);
pub const FEATURE_ENABLED_STATE_DEFAULT: FEATURE_ENABLED_STATE = FEATURE_ENABLED_STATE(0i32);
pub const FEATURE_ENABLED_STATE_DISABLED: FEATURE_ENABLED_STATE = FEATURE_ENABLED_STATE(1i32);
pub const FEATURE_ENABLED_STATE_ENABLED: FEATURE_ENABLED_STATE = FEATURE_ENABLED_STATE(2i32);
impl ::std::convert::From<i32> for FEATURE_ENABLED_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FEATURE_ENABLED_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FEATURE_ERROR {
    pub hr: ::windows::runtime::HRESULT,
    pub lineNumber: u16,
    pub file: super::super::Foundation::PSTR,
    pub process: super::super::Foundation::PSTR,
    pub module: super::super::Foundation::PSTR,
    pub callerReturnAddressOffset: u32,
    pub callerModule: super::super::Foundation::PSTR,
    pub message: super::super::Foundation::PSTR,
    pub originLineNumber: u16,
    pub originFile: super::super::Foundation::PSTR,
    pub originModule: super::super::Foundation::PSTR,
    pub originCallerReturnAddressOffset: u32,
    pub originCallerModule: super::super::Foundation::PSTR,
    pub originName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FEATURE_ERROR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FEATURE_ERROR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FEATURE_ERROR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FEATURE_ERROR")
            .field("hr", &self.hr)
            .field("lineNumber", &self.lineNumber)
            .field("file", &self.file)
            .field("process", &self.process)
            .field("module", &self.module)
            .field("callerReturnAddressOffset", &self.callerReturnAddressOffset)
            .field("callerModule", &self.callerModule)
            .field("message", &self.message)
            .field("originLineNumber", &self.originLineNumber)
            .field("originFile", &self.originFile)
            .field("originModule", &self.originModule)
            .field("originCallerReturnAddressOffset", &self.originCallerReturnAddressOffset)
            .field("originCallerModule", &self.originCallerModule)
            .field("originName", &self.originName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FEATURE_ERROR {
    fn eq(&self, other: &Self) -> bool {
        self.hr == other.hr
            && self.lineNumber == other.lineNumber
            && self.file == other.file
            && self.process == other.process
            && self.module == other.module
            && self.callerReturnAddressOffset == other.callerReturnAddressOffset
            && self.callerModule == other.callerModule
            && self.message == other.message
            && self.originLineNumber == other.originLineNumber
            && self.originFile == other.originFile
            && self.originModule == other.originModule
            && self.originCallerReturnAddressOffset == other.originCallerReturnAddressOffset
            && self.originCallerModule == other.originCallerModule
            && self.originName == other.originName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FEATURE_ERROR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FEATURE_ERROR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct FEATURE_STATE_CHANGE_SUBSCRIPTION(pub isize);
impl ::std::default::Default for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for FEATURE_STATE_CHANGE_SUBSCRIPTION {}
unsafe impl ::windows::runtime::Abi for FEATURE_STATE_CHANGE_SUBSCRIPTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct FH_SERVICE_PIPE_HANDLE(pub isize);
impl ::std::default::Default for FH_SERVICE_PIPE_HANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for FH_SERVICE_PIPE_HANDLE {}
unsafe impl ::windows::runtime::Abi for FH_SERVICE_PIPE_HANDLE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FIBER_FLAG_FLOAT_SWITCH: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_CASE_SENSITIVE_INFO {
    pub Flags: u32,
}
impl FILE_CASE_SENSITIVE_INFO {}
impl ::std::default::Default for FILE_CASE_SENSITIVE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_CASE_SENSITIVE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_CASE_SENSITIVE_INFO").field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for FILE_CASE_SENSITIVE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for FILE_CASE_SENSITIVE_INFO {}
unsafe impl ::windows::runtime::Abi for FILE_CASE_SENSITIVE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FILE_COMPLETE_IF_OPLOCKED: u32 = 256u32;
pub const FILE_CREATED: u32 = 2u32;
pub const FILE_CREATE_TREE_CONNECTION: u32 = 128u32;
pub const FILE_DELETE_ON_CLOSE: u32 = 4096u32;
pub const FILE_DIRECTORY_FILE: u32 = 1u32;
pub const FILE_DIR_DISALLOWED: u32 = 9u32;
pub const FILE_DISPOSITION_FLAG_DELETE: u32 = 1u32;
pub const FILE_DISPOSITION_FLAG_DO_NOT_DELETE: u32 = 0u32;
pub const FILE_DISPOSITION_FLAG_FORCE_IMAGE_SECTION_CHECK: u32 = 4u32;
pub const FILE_DISPOSITION_FLAG_IGNORE_READONLY_ATTRIBUTE: u32 = 16u32;
pub const FILE_DISPOSITION_FLAG_ON_CLOSE: u32 = 8u32;
pub const FILE_DISPOSITION_FLAG_POSIX_SEMANTICS: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_DISPOSITION_INFO_EX {
    pub Flags: u32,
}
impl FILE_DISPOSITION_INFO_EX {}
impl ::std::default::Default for FILE_DISPOSITION_INFO_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_DISPOSITION_INFO_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_DISPOSITION_INFO_EX").field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for FILE_DISPOSITION_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for FILE_DISPOSITION_INFO_EX {}
unsafe impl ::windows::runtime::Abi for FILE_DISPOSITION_INFO_EX {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FILE_DOES_NOT_EXIST: u32 = 5u32;
pub const FILE_ENCRYPTABLE: u32 = 0u32;
pub const FILE_EXISTS: u32 = 4u32;
pub const FILE_FLAG_OPEN_REQUIRING_OPLOCK: u32 = 262144u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FILE_INFORMATION_CLASS(pub i32);
pub const FileDirectoryInformation: FILE_INFORMATION_CLASS = FILE_INFORMATION_CLASS(1i32);
impl ::std::convert::From<i32> for FILE_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILE_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const FILE_IS_ENCRYPTED: u32 = 1u32;
pub const FILE_MAXIMUM_DISPOSITION: u32 = 5u32;
pub const FILE_NON_DIRECTORY_FILE: u32 = 64u32;
pub const FILE_NO_COMPRESSION: u32 = 32768u32;
pub const FILE_NO_EA_KNOWLEDGE: u32 = 512u32;
pub const FILE_NO_INTERMEDIATE_BUFFERING: u32 = 8u32;
pub const FILE_OPENED: u32 = 1u32;
pub const FILE_OPEN_BY_FILE_ID: u32 = 8192u32;
pub const FILE_OPEN_FOR_BACKUP_INTENT: u32 = 16384u32;
pub const FILE_OPEN_FOR_FREE_SPACE_QUERY: u32 = 8388608u32;
pub const FILE_OPEN_NO_RECALL: u32 = 4194304u32;
pub const FILE_OPEN_REMOTE_INSTANCE: u32 = 1024u32;
pub const FILE_OPEN_REPARSE_POINT: u32 = 2097152u32;
pub const FILE_OPEN_REQUIRING_OPLOCK: u32 = 65536u32;
pub const FILE_OVERWRITTEN: u32 = 3u32;
pub const FILE_RANDOM_ACCESS: u32 = 2048u32;
pub const FILE_READ_ONLY: u32 = 8u32;
pub const FILE_RENAME_FLAG_POSIX_SEMANTICS: u32 = 2u32;
pub const FILE_RENAME_FLAG_REPLACE_IF_EXISTS: u32 = 1u32;
pub const FILE_RENAME_FLAG_SUPPRESS_PIN_STATE_INHERITANCE: u32 = 4u32;
pub const FILE_RESERVE_OPFILTER: u32 = 1048576u32;
pub const FILE_ROOT_DIR: u32 = 3u32;
pub const FILE_SEQUENTIAL_ONLY: u32 = 4u32;
pub const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: u32 = 1u32;
pub const FILE_SKIP_SET_EVENT_ON_HANDLE: u32 = 2u32;
pub const FILE_SUPERSEDED: u32 = 0u32;
pub const FILE_SYNCHRONOUS_IO_ALERT: u32 = 16u32;
pub const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 32u32;
pub const FILE_SYSTEM_ATTR: u32 = 2u32;
pub const FILE_SYSTEM_DIR: u32 = 4u32;
pub const FILE_SYSTEM_NOT_SUPPORT: u32 = 6u32;
pub const FILE_TYPE_CHAR: u32 = 2u32;
pub const FILE_TYPE_DISK: u32 = 1u32;
pub const FILE_TYPE_PIPE: u32 = 3u32;
pub const FILE_TYPE_REMOTE: u32 = 32768u32;
pub const FILE_TYPE_UNKNOWN: u32 = 0u32;
pub const FILE_UNKNOWN: u32 = 5u32;
pub const FILE_USER_DISALLOWED: u32 = 7u32;
pub const FILE_VALID_MAILSLOT_OPTION_FLAGS: u32 = 50u32;
pub const FILE_VALID_OPTION_FLAGS: u32 = 16777215u32;
pub const FILE_VALID_PIPE_OPTION_FLAGS: u32 = 50u32;
pub const FILE_VALID_SET_FLAGS: u32 = 54u32;
pub const FILE_WRITE_THROUGH: u32 = 2u32;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_ASSEMBLY_METADATA: u32 = 4u32;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_FLAGS: u32 = 2u32;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_HACTCTX: u32 = 1u32;
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: u32 = 255u32;
pub const FS_CASE_IS_PRESERVED: u32 = 2u32;
pub const FS_CASE_SENSITIVE: u32 = 1u32;
pub const FS_FILE_COMPRESSION: u32 = 16u32;
pub const FS_FILE_ENCRYPTION: u32 = 131072u32;
pub const FS_PERSISTENT_ACLS: u32 = 8u32;
pub const FS_UNICODE_STORED_ON_DISK: u32 = 4u32;
pub const FS_VOL_IS_COMPRESSED: u32 = 32768u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileSaveMarkNotExistA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpfilelist: Param0, lpdir: Param1, lpbasename: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FileSaveMarkNotExistA(lpfilelist: super::super::Foundation::PSTR, lpdir: super::super::Foundation::PSTR, lpbasename: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
        }
        FileSaveMarkNotExistA(lpfilelist.into_param().abi(), lpdir.into_param().abi(), lpbasename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileSaveMarkNotExistW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpfilelist: Param0, lpdir: Param1, lpbasename: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FileSaveMarkNotExistW(lpfilelist: super::super::Foundation::PWSTR, lpdir: super::super::Foundation::PWSTR, lpbasename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        FileSaveMarkNotExistW(lpfilelist.into_param().abi(), lpdir.into_param().abi(), lpbasename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileSaveRestoreOnINFA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(
    hwnd: Param0,
    psztitle: Param1,
    pszinf: Param2,
    pszsection: Param3,
    pszbackupdir: Param4,
    pszbasebackupfile: Param5,
    dwflags: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FileSaveRestoreOnINFA(hwnd: super::super::Foundation::HWND, psztitle: super::super::Foundation::PSTR, pszinf: super::super::Foundation::PSTR, pszsection: super::super::Foundation::PSTR, pszbackupdir: super::super::Foundation::PSTR, pszbasebackupfile: super::super::Foundation::PSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        FileSaveRestoreOnINFA(hwnd.into_param().abi(), psztitle.into_param().abi(), pszinf.into_param().abi(), pszsection.into_param().abi(), pszbackupdir.into_param().abi(), pszbasebackupfile.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileSaveRestoreOnINFW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    hwnd: Param0,
    psztitle: Param1,
    pszinf: Param2,
    pszsection: Param3,
    pszbackupdir: Param4,
    pszbasebackupfile: Param5,
    dwflags: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FileSaveRestoreOnINFW(hwnd: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, pszinf: super::super::Foundation::PWSTR, pszsection: super::super::Foundation::PWSTR, pszbackupdir: super::super::Foundation::PWSTR, pszbasebackupfile: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        FileSaveRestoreOnINFW(hwnd.into_param().abi(), psztitle.into_param().abi(), pszinf.into_param().abi(), pszsection.into_param().abi(), pszbackupdir.into_param().abi(), pszbasebackupfile.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileSaveRestoreW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hdlg: Param0, lpfilelist: Param1, lpdir: Param2, lpbasename: Param3, dwflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FileSaveRestoreW(hdlg: super::super::Foundation::HWND, lpfilelist: super::super::Foundation::PWSTR, lpdir: super::super::Foundation::PWSTR, lpbasename: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        FileSaveRestoreW(hdlg.into_param().abi(), lpfilelist.into_param().abi(), lpdir.into_param().abi(), lpbasename.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpfatdate: *mut u16, lpfattime: *mut u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FileTimeToDosDateTime(lpfiletime: *const super::super::Foundation::FILETIME, lpfatdate: *mut u16, lpfattime: *mut u16) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(FileTimeToDosDateTime(::std::mem::transmute(lpfiletime), ::std::mem::transmute(lpfatdate), ::std::mem::transmute(lpfattime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const GMEM_DDESHARE: u32 = 8192u32;
pub const GMEM_DISCARDABLE: u32 = 256u32;
pub const GMEM_DISCARDED: u32 = 16384u32;
pub const GMEM_INVALID_HANDLE: u32 = 32768u32;
pub const GMEM_LOCKCOUNT: u32 = 255u32;
pub const GMEM_LOWER: u32 = 4096u32;
pub const GMEM_MODIFY: u32 = 128u32;
pub const GMEM_NOCOMPACT: u32 = 16u32;
pub const GMEM_NODISCARD: u32 = 32u32;
pub const GMEM_NOTIFY: u32 = 16384u32;
pub const GMEM_NOT_BANKED: u32 = 4096u32;
pub const GMEM_SHARE: u32 = 8192u32;
pub const GMEM_VALID_FLAGS: u32 = 32626u32;
#[inline]
pub unsafe fn GdiEntry13() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GdiEntry13() -> u32;
        }
        ::std::mem::transmute(GdiEntry13())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComputerNameA(lpbuffer: super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetComputerNameA(lpbuffer: super::super::Foundation::PSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetComputerNameA(::std::mem::transmute(lpbuffer), ::std::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetComputerNameW(lpbuffer: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetComputerNameW(lpbuffer: super::super::Foundation::PWSTR, nsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetComputerNameW(::std::mem::transmute(lpbuffer), ::std::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentHwProfileA(lphwprofileinfo: *mut HW_PROFILE_INFOA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentHwProfileA(lphwprofileinfo: *mut HW_PROFILE_INFOA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCurrentHwProfileA(::std::mem::transmute(lphwprofileinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentHwProfileW(lphwprofileinfo: *mut HW_PROFILE_INFOW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentHwProfileW(lphwprofileinfo: *mut HW_PROFILE_INFOW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetCurrentHwProfileW(::std::mem::transmute(lphwprofileinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetDCRegionData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HDC>>(hdc: Param0, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDCRegionData(hdc: super::super::Graphics::Gdi::HDC, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32;
        }
        ::std::mem::transmute(GetDCRegionData(hdc.into_param().abi(), ::std::mem::transmute(size), ::std::mem::transmute(prd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetFeatureEnabledState(featureid: u32, changetime: FEATURE_CHANGE_TIME) -> FEATURE_ENABLED_STATE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFeatureEnabledState(featureid: u32, changetime: FEATURE_CHANGE_TIME) -> FEATURE_ENABLED_STATE;
        }
        ::std::mem::transmute(GetFeatureEnabledState(::std::mem::transmute(featureid), ::std::mem::transmute(changetime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFeatureVariant(featureid: u32, changetime: FEATURE_CHANGE_TIME, payloadid: *mut u32, hasnotification: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFeatureVariant(featureid: u32, changetime: FEATURE_CHANGE_TIME, payloadid: *mut u32, hasnotification: *mut super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(GetFeatureVariant(::std::mem::transmute(featureid), ::std::mem::transmute(changetime), ::std::mem::transmute(payloadid), ::std::mem::transmute(hasnotification)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpname: Param0, lpguid: Param1, pbuffer: *mut ::std::ffi::c_void, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFirmwareEnvironmentVariableA(lpname: super::super::Foundation::PSTR, lpguid: super::super::Foundation::PSTR, pbuffer: *mut ::std::ffi::c_void, nsize: u32) -> u32;
        }
        ::std::mem::transmute(GetFirmwareEnvironmentVariableA(lpname.into_param().abi(), lpguid.into_param().abi(), ::std::mem::transmute(pbuffer), ::std::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpname: Param0, lpguid: Param1, pbuffer: *mut ::std::ffi::c_void, nsize: u32, pdwattribubutes: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFirmwareEnvironmentVariableExA(lpname: super::super::Foundation::PSTR, lpguid: super::super::Foundation::PSTR, pbuffer: *mut ::std::ffi::c_void, nsize: u32, pdwattribubutes: *mut u32) -> u32;
        }
        ::std::mem::transmute(GetFirmwareEnvironmentVariableExA(lpname.into_param().abi(), lpguid.into_param().abi(), ::std::mem::transmute(pbuffer), ::std::mem::transmute(nsize), ::std::mem::transmute(pdwattribubutes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpname: Param0, lpguid: Param1, pbuffer: *mut ::std::ffi::c_void, nsize: u32, pdwattribubutes: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFirmwareEnvironmentVariableExW(lpname: super::super::Foundation::PWSTR, lpguid: super::super::Foundation::PWSTR, pbuffer: *mut ::std::ffi::c_void, nsize: u32, pdwattribubutes: *mut u32) -> u32;
        }
        ::std::mem::transmute(GetFirmwareEnvironmentVariableExW(lpname.into_param().abi(), lpguid.into_param().abi(), ::std::mem::transmute(pbuffer), ::std::mem::transmute(nsize), ::std::mem::transmute(pdwattribubutes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpname: Param0, lpguid: Param1, pbuffer: *mut ::std::ffi::c_void, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFirmwareEnvironmentVariableW(lpname: super::super::Foundation::PWSTR, lpguid: super::super::Foundation::PWSTR, pbuffer: *mut ::std::ffi::c_void, nsize: u32) -> u32;
        }
        ::std::mem::transmute(GetFirmwareEnvironmentVariableW(lpname.into_param().abi(), lpguid.into_param().abi(), ::std::mem::transmute(pbuffer), ::std::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateProfileIntA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpappname: Param0, lpkeyname: Param1, ndefault: i32, lpfilename: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileIntA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, ndefault: i32, lpfilename: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(GetPrivateProfileIntA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), ::std::mem::transmute(ndefault), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateProfileIntW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpappname: Param0, lpkeyname: Param1, ndefault: i32, lpfilename: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileIntW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, ndefault: i32, lpfilename: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(GetPrivateProfileIntW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), ::std::mem::transmute(ndefault), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateProfileSectionA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpappname: Param0, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32, lpfilename: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileSectionA(lpappname: super::super::Foundation::PSTR, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32, lpfilename: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(GetPrivateProfileSectionA(lpappname.into_param().abi(), ::std::mem::transmute(lpreturnedstring), ::std::mem::transmute(nsize), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateProfileSectionNamesA<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpszreturnbuffer: super::super::Foundation::PSTR, nsize: u32, lpfilename: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileSectionNamesA(lpszreturnbuffer: super::super::Foundation::PSTR, nsize: u32, lpfilename: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(GetPrivateProfileSectionNamesA(::std::mem::transmute(lpszreturnbuffer), ::std::mem::transmute(nsize), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateProfileSectionNamesW<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszreturnbuffer: super::super::Foundation::PWSTR, nsize: u32, lpfilename: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileSectionNamesW(lpszreturnbuffer: super::super::Foundation::PWSTR, nsize: u32, lpfilename: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(GetPrivateProfileSectionNamesW(::std::mem::transmute(lpszreturnbuffer), ::std::mem::transmute(nsize), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateProfileSectionW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpappname: Param0, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32, lpfilename: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileSectionW(lpappname: super::super::Foundation::PWSTR, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32, lpfilename: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(GetPrivateProfileSectionW(lpappname.into_param().abi(), ::std::mem::transmute(lpreturnedstring), ::std::mem::transmute(nsize), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateProfileStringA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpappname: Param0, lpkeyname: Param1, lpdefault: Param2, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32, lpfilename: Param5) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileStringA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, lpdefault: super::super::Foundation::PSTR, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32, lpfilename: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(GetPrivateProfileStringA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpdefault.into_param().abi(), ::std::mem::transmute(lpreturnedstring), ::std::mem::transmute(nsize), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateProfileStringW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpappname: Param0, lpkeyname: Param1, lpdefault: Param2, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32, lpfilename: Param5) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileStringW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, lpdefault: super::super::Foundation::PWSTR, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32, lpfilename: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(GetPrivateProfileStringW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpdefault.into_param().abi(), ::std::mem::transmute(lpreturnedstring), ::std::mem::transmute(nsize), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateProfileStructA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpszsection: Param0, lpszkey: Param1, lpstruct: *mut ::std::ffi::c_void, usizestruct: u32, szfile: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileStructA(lpszsection: super::super::Foundation::PSTR, lpszkey: super::super::Foundation::PSTR, lpstruct: *mut ::std::ffi::c_void, usizestruct: u32, szfile: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPrivateProfileStructA(lpszsection.into_param().abi(), lpszkey.into_param().abi(), ::std::mem::transmute(lpstruct), ::std::mem::transmute(usizestruct), szfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPrivateProfileStructW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszsection: Param0, lpszkey: Param1, lpstruct: *mut ::std::ffi::c_void, usizestruct: u32, szfile: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPrivateProfileStructW(lpszsection: super::super::Foundation::PWSTR, lpszkey: super::super::Foundation::PWSTR, lpstruct: *mut ::std::ffi::c_void, usizestruct: u32, szfile: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetPrivateProfileStructW(lpszsection.into_param().abi(), lpszkey.into_param().abi(), ::std::mem::transmute(lpstruct), ::std::mem::transmute(usizestruct), szfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProfileIntA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpappname: Param0, lpkeyname: Param1, ndefault: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProfileIntA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, ndefault: i32) -> u32;
        }
        ::std::mem::transmute(GetProfileIntA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), ::std::mem::transmute(ndefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProfileIntW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpappname: Param0, lpkeyname: Param1, ndefault: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProfileIntW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, ndefault: i32) -> u32;
        }
        ::std::mem::transmute(GetProfileIntW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), ::std::mem::transmute(ndefault)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProfileSectionA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpappname: Param0, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProfileSectionA(lpappname: super::super::Foundation::PSTR, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32) -> u32;
        }
        ::std::mem::transmute(GetProfileSectionA(lpappname.into_param().abi(), ::std::mem::transmute(lpreturnedstring), ::std::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProfileSectionW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpappname: Param0, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProfileSectionW(lpappname: super::super::Foundation::PWSTR, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32) -> u32;
        }
        ::std::mem::transmute(GetProfileSectionW(lpappname.into_param().abi(), ::std::mem::transmute(lpreturnedstring), ::std::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProfileStringA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpappname: Param0, lpkeyname: Param1, lpdefault: Param2, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProfileStringA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, lpdefault: super::super::Foundation::PSTR, lpreturnedstring: super::super::Foundation::PSTR, nsize: u32) -> u32;
        }
        ::std::mem::transmute(GetProfileStringA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpdefault.into_param().abi(), ::std::mem::transmute(lpreturnedstring), ::std::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProfileStringW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpappname: Param0, lpkeyname: Param1, lpdefault: Param2, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProfileStringW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, lpdefault: super::super::Foundation::PWSTR, lpreturnedstring: super::super::Foundation::PWSTR, nsize: u32) -> u32;
        }
        ::std::mem::transmute(GetProfileStringW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpdefault.into_param().abi(), ::std::mem::transmute(lpreturnedstring), ::std::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
#[inline]
pub unsafe fn GetStartupInfoA(lpstartupinfo: *mut super::Threading::STARTUPINFOA) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStartupInfoA(lpstartupinfo: *mut super::Threading::STARTUPINFOA);
        }
        ::std::mem::transmute(GetStartupInfoA(::std::mem::transmute(lpstartupinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemRegistryQuota(pdwquotaallowed: *mut u32, pdwquotaused: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemRegistryQuota(pdwquotaallowed: *mut u32, pdwquotaused: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetSystemRegistryQuota(::std::mem::transmute(pdwquotaallowed), ::std::mem::transmute(pdwquotaused)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetThreadEnabledXStateFeatures() -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadEnabledXStateFeatures() -> u64;
        }
        ::std::mem::transmute(GetThreadEnabledXStateFeatures())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserNameA(lpbuffer: super::super::Foundation::PSTR, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUserNameA(lpbuffer: super::super::Foundation::PSTR, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetUserNameA(::std::mem::transmute(lpbuffer), ::std::mem::transmute(pcbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUserNameW(lpbuffer: super::super::Foundation::PWSTR, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUserNameW(lpbuffer: super::super::Foundation::PWSTR, pcbbuffer: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetUserNameW(::std::mem::transmute(lpbuffer), ::std::mem::transmute(pcbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionFromFileA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpszfilename: Param0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: Param3) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVersionFromFileA(lpszfilename: super::super::Foundation::PSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        GetVersionFromFileA(lpszfilename.into_param().abi(), ::std::mem::transmute(pdwmsver), ::std::mem::transmute(pdwlsver), bversion.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionFromFileExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpszfilename: Param0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: Param3) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVersionFromFileExA(lpszfilename: super::super::Foundation::PSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        GetVersionFromFileExA(lpszfilename.into_param().abi(), ::std::mem::transmute(pdwmsver), ::std::mem::transmute(pdwlsver), bversion.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionFromFileExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpszfilename: Param0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: Param3) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVersionFromFileExW(lpszfilename: super::super::Foundation::PWSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        GetVersionFromFileExW(lpszfilename.into_param().abi(), ::std::mem::transmute(pdwmsver), ::std::mem::transmute(pdwlsver), bversion.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVersionFromFileW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(lpszfilename: Param0, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: Param3) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVersionFromFileW(lpszfilename: super::super::Foundation::PWSTR, pdwmsver: *mut u32, pdwlsver: *mut u32, bversion: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        GetVersionFromFileW(lpszfilename.into_param().abi(), ::std::mem::transmute(pdwmsver), ::std::mem::transmute(pdwlsver), bversion.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn GetWindowRegionData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetWindowRegionData(hwnd: super::super::Foundation::HWND, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32;
        }
        ::std::mem::transmute(GetWindowRegionData(hwnd.into_param().abi(), ::std::mem::transmute(size), ::std::mem::transmute(prd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GlobalCompact(dwminfree: u32) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalCompact(dwminfree: u32) -> usize;
        }
        ::std::mem::transmute(GlobalCompact(::std::mem::transmute(dwminfree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GlobalFix(hmem: isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalFix(hmem: isize);
        }
        ::std::mem::transmute(GlobalFix(::std::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GlobalUnWire(hmem: isize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalUnWire(hmem: isize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GlobalUnWire(::std::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GlobalUnfix(hmem: isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalUnfix(hmem: isize);
        }
        ::std::mem::transmute(GlobalUnfix(::std::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GlobalWire(hmem: isize) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GlobalWire(hmem: isize) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(GlobalWire(::std::mem::transmute(hmem)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const HANJA_WINDOW: u32 = 2u32;
pub const HINSTANCE_ERROR: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HWINWATCH(pub isize);
impl ::std::default::Default for HWINWATCH {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HWINWATCH {}
unsafe impl ::windows::runtime::Abi for HWINWATCH {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HW_PROFILE_GUIDLEN: u32 = 39u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HW_PROFILE_INFOA {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [super::super::Foundation::CHAR; 39],
    pub szHwProfileName: [super::super::Foundation::CHAR; 80],
}
#[cfg(feature = "Win32_Foundation")]
impl HW_PROFILE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HW_PROFILE_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HW_PROFILE_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HW_PROFILE_INFOA").field("dwDockInfo", &self.dwDockInfo).field("szHwProfileGuid", &self.szHwProfileGuid).field("szHwProfileName", &self.szHwProfileName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HW_PROFILE_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwDockInfo == other.dwDockInfo && self.szHwProfileGuid == other.szHwProfileGuid && self.szHwProfileName == other.szHwProfileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HW_PROFILE_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HW_PROFILE_INFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HW_PROFILE_INFOW {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [u16; 39],
    pub szHwProfileName: [u16; 80],
}
impl HW_PROFILE_INFOW {}
impl ::std::default::Default for HW_PROFILE_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HW_PROFILE_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HW_PROFILE_INFOW").field("dwDockInfo", &self.dwDockInfo).field("szHwProfileGuid", &self.szHwProfileGuid).field("szHwProfileName", &self.szHwProfileName).finish()
    }
}
impl ::std::cmp::PartialEq for HW_PROFILE_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwDockInfo == other.dwDockInfo && self.szHwProfileGuid == other.szHwProfileGuid && self.szHwProfileName == other.szHwProfileName
    }
}
impl ::std::cmp::Eq for HW_PROFILE_INFOW {}
unsafe impl ::windows::runtime::Abi for HW_PROFILE_INFOW {
    type Abi = Self;
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICameraUIControl(::windows::runtime::IUnknown);
impl ICameraUIControl {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param7: ::windows::runtime::IntoParam<'a, ICameraUIControlEventCallback>>(
        &self,
        pwindow: Param0,
        mode: CameraUIControlMode,
        selectionmode: CameraUIControlLinearSelectionMode,
        capturemode: CameraUIControlCaptureMode,
        photoformat: CameraUIControlPhotoFormat,
        videoformat: CameraUIControlVideoFormat,
        bhasclosebutton: Param6,
        peventcallback: Param7,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            pwindow.into_param().abi(),
            ::std::mem::transmute(mode),
            ::std::mem::transmute(selectionmode),
            ::std::mem::transmute(capturemode),
            ::std::mem::transmute(photoformat),
            ::std::mem::transmute(videoformat),
            bhasclosebutton.into_param().abi(),
            peventcallback.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Suspend(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn Resume(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetCurrentViewType(&self) -> ::windows::runtime::Result<CameraUIControlViewType> {
        let mut result__: <CameraUIControlViewType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<CameraUIControlViewType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetActiveItem(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSelectedItems(&self) -> ::windows::runtime::Result<*mut super::Com::SAFEARRAY> {
        let mut result__: <*mut super::Com::SAFEARRAY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut super::Com::SAFEARRAY>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveCapturedItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pszpath.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICameraUIControl {
    type Vtable = ICameraUIControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3094559455, 15720, 19343, [187, 8, 226, 138, 11, 237, 3, 118]);
}
impl ::std::convert::From<ICameraUIControl> for ::windows::runtime::IUnknown {
    fn from(value: ICameraUIControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICameraUIControl> for ::windows::runtime::IUnknown {
    fn from(value: &ICameraUIControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICameraUIControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICameraUIControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraUIControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwindow: ::windows::runtime::RawPtr, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: super::super::Foundation::BOOL, peventcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbdeferralrequired: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pviewtype: *mut CameraUIControlViewType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstractiveitempath: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppselecteditempaths: *mut *mut super::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICameraUIControlEventCallback(::windows::runtime::IUnknown);
impl ICameraUIControlEventCallback {
    pub unsafe fn OnStartupComplete(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
    pub unsafe fn OnSuspendComplete(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnItemCaptured<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pszpath.into_param().abi()))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnItemDeleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pszpath.into_param().abi()))
    }
    pub unsafe fn OnClosed(&self) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for ICameraUIControlEventCallback {
    type Vtable = ICameraUIControlEventCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(469371948, 64461, 18294, [189, 164, 136, 191, 151, 78, 116, 244]);
}
impl ::std::convert::From<ICameraUIControlEventCallback> for ::windows::runtime::IUnknown {
    fn from(value: ICameraUIControlEventCallback) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICameraUIControlEventCallback> for ::windows::runtime::IUnknown {
    fn from(value: &ICameraUIControlEventCallback) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICameraUIControlEventCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ICameraUIControlEventCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICameraUIControlEventCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr),
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IClipServiceNotificationHelper(::windows::runtime::IUnknown);
impl IClipServiceNotificationHelper {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowToast<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(
        &self,
        titletext: Param0,
        bodytext: Param1,
        packagename: Param2,
        appid: Param3,
        launchcommand: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), titletext.into_param().abi(), bodytext.into_param().abi(), packagename.into_param().abi(), appid.into_param().abi(), launchcommand.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IClipServiceNotificationHelper {
    type Vtable = IClipServiceNotificationHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3281602800, 24898, 17661, [152, 202, 225, 104, 26, 141, 104, 181]);
}
impl ::std::convert::From<IClipServiceNotificationHelper> for ::windows::runtime::IUnknown {
    fn from(value: IClipServiceNotificationHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IClipServiceNotificationHelper> for ::windows::runtime::IUnknown {
    fn from(value: &IClipServiceNotificationHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IClipServiceNotificationHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IClipServiceNotificationHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClipServiceNotificationHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, titletext: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bodytext: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, packagename: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, appid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, launchcommand: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IContainerActivationHelper(::windows::runtime::IUnknown);
impl IContainerActivationHelper {
    pub unsafe fn CanActivateClientVM(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IContainerActivationHelper {
    type Vtable = IContainerActivationHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3039099199, 32981, 20167, [174, 158, 214, 110, 147, 173, 225, 250]);
}
impl ::std::convert::From<IContainerActivationHelper> for ::windows::runtime::IUnknown {
    fn from(value: IContainerActivationHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IContainerActivationHelper> for ::windows::runtime::IUnknown {
    fn from(value: &IContainerActivationHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContainerActivationHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IContainerActivationHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContainerActivationHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isallowed: *mut i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDefaultBrowserSyncSettings(::windows::runtime::IUnknown);
impl IDefaultBrowserSyncSettings {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsEnabled(&self) -> super::super::Foundation::BOOL {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IDefaultBrowserSyncSettings {
    type Vtable = IDefaultBrowserSyncSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2049440429, 23270, 16981, [144, 48, 197, 48, 147, 98, 146, 227]);
}
impl ::std::convert::From<IDefaultBrowserSyncSettings> for ::windows::runtime::IUnknown {
    fn from(value: IDefaultBrowserSyncSettings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDefaultBrowserSyncSettings> for ::windows::runtime::IUnknown {
    fn from(value: &IDefaultBrowserSyncSettings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDefaultBrowserSyncSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDefaultBrowserSyncSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDefaultBrowserSyncSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDeleteBrowsingHistory(::windows::runtime::IUnknown);
impl IDeleteBrowsingHistory {
    pub unsafe fn DeleteBrowsingHistory(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDeleteBrowsingHistory {
    type Vtable = IDeleteBrowsingHistory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3476614475, 11239, 17505, [139, 94, 154, 70, 109, 200, 42, 227]);
}
impl ::std::convert::From<IDeleteBrowsingHistory> for ::windows::runtime::IUnknown {
    fn from(value: IDeleteBrowsingHistory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDeleteBrowsingHistory> for ::windows::runtime::IUnknown {
    fn from(value: &IDeleteBrowsingHistory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDeleteBrowsingHistory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDeleteBrowsingHistory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeleteBrowsingHistory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
);
pub const IE4_BACKNEW: u32 = 2u32;
pub const IE4_EXTRAINCREFCNT: u32 = 2048u32;
pub const IE4_FRDOALL: u32 = 256u32;
pub const IE4_NODELETENEW: u32 = 4u32;
pub const IE4_NOENUMKEY: u32 = 32u32;
pub const IE4_NOMESSAGES: u32 = 8u32;
pub const IE4_NOPROGRESS: u32 = 16u32;
pub const IE4_NO_CRC_MAPPING: u32 = 64u32;
pub const IE4_REGSECTION: u32 = 128u32;
pub const IE4_REMOVREGBKDATA: u32 = 4096u32;
pub const IE4_RESTORE: u32 = 1u32;
pub const IE4_UPDREFCNT: u32 = 512u32;
pub const IE4_USEREFCNT: u32 = 1024u32;
pub const IE_BADID: i32 = -1i32;
pub const IE_BAUDRATE: i32 = -12i32;
pub const IE_BYTESIZE: i32 = -11i32;
pub const IE_DEFAULT: i32 = -5i32;
pub const IE_HARDWARE: i32 = -10i32;
pub const IE_MEMORY: i32 = -4i32;
pub const IE_NOPEN: i32 = -3i32;
pub const IE_OPEN: i32 = -2i32;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEditionUpgradeBroker(::windows::runtime::IUnknown);
impl IEditionUpgradeBroker {
    pub unsafe fn InitializeParentWindow(&self, parenthandle: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(parenthandle)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateOperatingSystem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, parameter: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), parameter.into_param().abi()).ok()
    }
    pub unsafe fn ShowProductKeyUI(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CanUpgrade(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEditionUpgradeBroker {
    type Vtable = IEditionUpgradeBroker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4279880655, 37973, 18743, [184, 114, 107, 121, 41, 164, 96, 175]);
}
impl ::std::convert::From<IEditionUpgradeBroker> for ::windows::runtime::IUnknown {
    fn from(value: IEditionUpgradeBroker) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEditionUpgradeBroker> for ::windows::runtime::IUnknown {
    fn from(value: &IEditionUpgradeBroker) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEditionUpgradeBroker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEditionUpgradeBroker {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEditionUpgradeBroker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parenthandle: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parameter: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEditionUpgradeHelper(::windows::runtime::IUnknown);
impl IEditionUpgradeHelper {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanUpgrade(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateOperatingSystem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, contentid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), contentid.into_param().abi()).ok()
    }
    pub unsafe fn ShowProductKeyUI(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOsProductContentId(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetGenuineLocalStatus(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEditionUpgradeHelper {
    type Vtable = IEditionUpgradeHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3555320642, 24043, 17334, [132, 158, 105, 19, 184, 93, 80, 58]);
}
impl ::std::convert::From<IEditionUpgradeHelper> for ::windows::runtime::IUnknown {
    fn from(value: IEditionUpgradeHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEditionUpgradeHelper> for ::windows::runtime::IUnknown {
    fn from(value: &IEditionUpgradeHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEditionUpgradeHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEditionUpgradeHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEditionUpgradeHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isallowed: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, contentid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isgenuine: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const IF_GENERIC: u32 = 512u32;
pub const IF_MIB: u32 = 514u32;
pub const IGNORE: u32 = 0u32;
pub const IMEA_INIT: u32 = 1u32;
pub const IMEA_NEXT: u32 = 2u32;
pub const IMEA_PREV: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEPROA {
    pub hWnd: super::super::Foundation::HWND,
    pub InstDate: DATETIME,
    pub wVersion: u32,
    pub szDescription: [u8; 50],
    pub szName: [u8; 80],
    pub szOptions: [u8; 30],
}
#[cfg(feature = "Win32_Foundation")]
impl IMEPROA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IMEPROA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IMEPROA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IMEPROA").field("hWnd", &self.hWnd).field("InstDate", &self.InstDate).field("wVersion", &self.wVersion).field("szDescription", &self.szDescription).field("szName", &self.szName).field("szOptions", &self.szOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IMEPROA {
    fn eq(&self, other: &Self) -> bool {
        self.hWnd == other.hWnd && self.InstDate == other.InstDate && self.wVersion == other.wVersion && self.szDescription == other.szDescription && self.szName == other.szName && self.szOptions == other.szOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IMEPROA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IMEPROA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IMEPROW {
    pub hWnd: super::super::Foundation::HWND,
    pub InstDate: DATETIME,
    pub wVersion: u32,
    pub szDescription: [u16; 50],
    pub szName: [u16; 80],
    pub szOptions: [u16; 30],
}
#[cfg(feature = "Win32_Foundation")]
impl IMEPROW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IMEPROW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IMEPROW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IMEPROW").field("hWnd", &self.hWnd).field("InstDate", &self.InstDate).field("wVersion", &self.wVersion).field("szDescription", &self.szDescription).field("szName", &self.szName).field("szOptions", &self.szOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IMEPROW {
    fn eq(&self, other: &Self) -> bool {
        self.hWnd == other.hWnd && self.InstDate == other.InstDate && self.wVersion == other.wVersion && self.szDescription == other.szDescription && self.szName == other.szName && self.szOptions == other.szOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IMEPROW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IMEPROW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IMESTRUCT {
    pub fnc: u32,
    pub wParam: super::super::Foundation::WPARAM,
    pub wCount: u32,
    pub dchSource: u32,
    pub dchDest: u32,
    pub lParam1: super::super::Foundation::LPARAM,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lParam3: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl IMESTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IMESTRUCT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IMESTRUCT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IMESTRUCT").field("fnc", &self.fnc).field("wParam", &self.wParam).field("wCount", &self.wCount).field("dchSource", &self.dchSource).field("dchDest", &self.dchDest).field("lParam1", &self.lParam1).field("lParam2", &self.lParam2).field("lParam3", &self.lParam3).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IMESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.fnc == other.fnc && self.wParam == other.wParam && self.wCount == other.wCount && self.dchSource == other.dchSource && self.dchDest == other.dchDest && self.lParam1 == other.lParam1 && self.lParam2 == other.lParam2 && self.lParam3 == other.lParam3
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IMESTRUCT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IMESTRUCT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IME_BANJAtoJUNJA: u32 = 19u32;
pub const IME_ENABLE_CONVERT: u32 = 2u32;
pub const IME_ENTERWORDREGISTERMODE: u32 = 24u32;
pub const IME_GETCONVERSIONMODE: u32 = 17u32;
pub const IME_GETIMECAPS: u32 = 3u32;
pub const IME_GETOPEN: u32 = 5u32;
pub const IME_GETVERSION: u32 = 7u32;
pub const IME_JOHABtoKS: u32 = 21u32;
pub const IME_JUNJAtoBANJA: u32 = 20u32;
pub const IME_KStoJOHAB: u32 = 22u32;
pub const IME_MAXPROCESS: u32 = 32u32;
pub const IME_MODE_ALPHANUMERIC: u32 = 1u32;
pub const IME_MODE_CODEINPUT: u32 = 128u32;
pub const IME_MODE_DBCSCHAR: u32 = 16u32;
pub const IME_MODE_HANJACONVERT: u32 = 4u32;
pub const IME_MODE_HIRAGANA: u32 = 4u32;
pub const IME_MODE_KATAKANA: u32 = 2u32;
pub const IME_MODE_NOCODEINPUT: u32 = 256u32;
pub const IME_MODE_NOROMAN: u32 = 64u32;
pub const IME_MODE_ROMAN: u32 = 32u32;
pub const IME_MODE_SBCSCHAR: u32 = 2u32;
pub const IME_MOVEIMEWINDOW: u32 = 8u32;
pub const IME_REQUEST_CONVERT: u32 = 1u32;
pub const IME_RS_DISKERROR: u32 = 14u32;
pub const IME_RS_ERROR: u32 = 1u32;
pub const IME_RS_ILLEGAL: u32 = 6u32;
pub const IME_RS_INVALID: u32 = 17u32;
pub const IME_RS_NEST: u32 = 18u32;
pub const IME_RS_NOIME: u32 = 2u32;
pub const IME_RS_NOROOM: u32 = 10u32;
pub const IME_RS_NOTFOUND: u32 = 7u32;
pub const IME_RS_SYSTEMMODAL: u32 = 19u32;
pub const IME_RS_TOOLONG: u32 = 5u32;
pub const IME_SENDVKEY: u32 = 19u32;
pub const IME_SETCONVERSIONFONTEX: u32 = 25u32;
pub const IME_SETCONVERSIONMODE: u32 = 16u32;
pub const IME_SETCONVERSIONWINDOW: u32 = 8u32;
pub const IME_SETOPEN: u32 = 4u32;
pub const IME_SET_MODE: u32 = 18u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPGetIMEA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0, param1: *mut IMEPROA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IMPGetIMEA(param0: super::super::Foundation::HWND, param1: *mut IMEPROA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IMPGetIMEA(param0.into_param().abi(), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPGetIMEW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0, param1: *mut IMEPROW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IMPGetIMEW(param0: super::super::Foundation::HWND, param1: *mut IMEPROW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IMPGetIMEW(param0.into_param().abi(), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPQueryIMEA(param0: *mut IMEPROA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IMPQueryIMEA(param0: *mut IMEPROA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IMPQueryIMEA(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPQueryIMEW(param0: *mut IMEPROW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IMPQueryIMEW(param0: *mut IMEPROW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IMPQueryIMEW(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPSetIMEA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0, param1: *mut IMEPROA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IMPSetIMEA(param0: super::super::Foundation::HWND, param1: *mut IMEPROA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IMPSetIMEA(param0.into_param().abi(), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IMPSetIMEW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0, param1: *mut IMEPROW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IMPSetIMEW(param0: super::super::Foundation::HWND, param1: *mut IMEPROW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IMPSetIMEW(param0.into_param().abi(), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const INFINITE: u32 = 4294967295u32;
pub const INFO_CLASS_GENERIC: u32 = 256u32;
pub const INFO_CLASS_IMPLEMENTATION: u32 = 768u32;
pub const INFO_CLASS_PROTOCOL: u32 = 512u32;
pub const INFO_TYPE_ADDRESS_OBJECT: u32 = 512u32;
pub const INFO_TYPE_CONNECTION: u32 = 768u32;
pub const INFO_TYPE_PROVIDER: u32 = 256u32;
pub const INTERIM_WINDOW: u32 = 0u32;
pub const INVALID_ENTITY_INSTANCE: i32 = -1i32;
pub const IOCTL_TDI_TL_IO_CONTROL_ENDPOINT: u32 = 2162744u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IO_STATUS_BLOCK {
    pub Anonymous: IO_STATUS_BLOCK_0,
    pub Information: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl IO_STATUS_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IO_STATUS_BLOCK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IO_STATUS_BLOCK {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IO_STATUS_BLOCK {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IO_STATUS_BLOCK {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union IO_STATUS_BLOCK_0 {
    pub Status: super::super::Foundation::NTSTATUS,
    pub Pointer: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl IO_STATUS_BLOCK_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IO_STATUS_BLOCK_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IO_STATUS_BLOCK_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IO_STATUS_BLOCK_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IO_STATUS_BLOCK_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IR_CHANGECONVERT: u32 = 289u32;
pub const IR_CLOSECONVERT: u32 = 290u32;
pub const IR_DBCSCHAR: u32 = 352u32;
pub const IR_FULLCONVERT: u32 = 291u32;
pub const IR_IMESELECT: u32 = 304u32;
pub const IR_MODEINFO: u32 = 400u32;
pub const IR_OPENCONVERT: u32 = 288u32;
pub const IR_STRING: u32 = 320u32;
pub const IR_STRINGEND: u32 = 257u32;
pub const IR_STRINGEX: u32 = 384u32;
pub const IR_STRINGSTART: u32 = 256u32;
pub const IR_UNDETERMINE: u32 = 368u32;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IScriptErrorList(::windows::runtime::IUnknown);
impl IScriptErrorList {
    pub unsafe fn advanceError(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn retreatError(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn canAdvanceError(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn canRetreatError(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    pub unsafe fn getErrorLine(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn getErrorChar(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn getErrorCode(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getErrorMsg(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getErrorUrl(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getAlwaysShowLockState(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getDetailsPaneOpen(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn setDetailsPaneOpen<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fdetailspaneopen: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), fdetailspaneopen.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getPerErrorDisplay(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn setPerErrorDisplay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fpererrordisplay: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), fpererrordisplay.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IScriptErrorList {
    type Vtable = IScriptErrorList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4081520420, 5629, 4562, [187, 46, 0, 128, 95, 247, 239, 202]);
}
impl ::std::convert::From<IScriptErrorList> for ::windows::runtime::IUnknown {
    fn from(value: IScriptErrorList) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IScriptErrorList> for ::windows::runtime::IUnknown {
    fn from(value: &IScriptErrorList) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IScriptErrorList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IScriptErrorList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IScriptErrorList> for super::Ole::Automation::IDispatch {
    fn from(value: IScriptErrorList) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IScriptErrorList> for super::Ole::Automation::IDispatch {
    fn from(value: &IScriptErrorList) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IScriptErrorList {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IScriptErrorList {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScriptErrorList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfcanadvance: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfcanretreat: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plline: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plchar: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcode: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstr: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstr: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfalwaysshowlocked: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfdetailspaneopen: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fdetailspaneopen: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfpererrordisplay: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fpererrordisplay: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IShellFavoritesNameSpace(::windows::runtime::IUnknown);
impl IShellFavoritesNameSpace {
    pub unsafe fn MoveSelectionUp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn MoveSelectionDown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResetSort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn NewFolder(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Synchronize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Import(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Export(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeContextMenuCommand<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strcommand: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), strcommand.into_param().abi()).ok()
    }
    pub unsafe fn MoveSelectionTo(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SubscriptionsEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn CreateSubscriptionForSelection(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn DeleteSubscriptionForSelection(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfullpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), bstrfullpath.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IShellFavoritesNameSpace {
    type Vtable = IShellFavoritesNameSpace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1427335172, 45790, 4561, [185, 242, 0, 160, 201, 139, 197, 71]);
}
impl ::std::convert::From<IShellFavoritesNameSpace> for ::windows::runtime::IUnknown {
    fn from(value: IShellFavoritesNameSpace) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellFavoritesNameSpace> for ::windows::runtime::IUnknown {
    fn from(value: &IShellFavoritesNameSpace) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IShellFavoritesNameSpace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IShellFavoritesNameSpace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IShellFavoritesNameSpace> for super::Ole::Automation::IDispatch {
    fn from(value: IShellFavoritesNameSpace) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IShellFavoritesNameSpace> for super::Ole::Automation::IDispatch {
    fn from(value: &IShellFavoritesNameSpace) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IShellFavoritesNameSpace {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IShellFavoritesNameSpace {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellFavoritesNameSpace_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strcommand: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrfullpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IShellNameSpace(::windows::runtime::IUnknown);
impl IShellNameSpace {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn MoveSelectionUp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn MoveSelectionDown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResetSort(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn NewFolder(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Synchronize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Import(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Export(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InvokeContextMenuCommand<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strcommand: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), strcommand.into_param().abi()).ok()
    }
    pub unsafe fn MoveSelectionTo(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SubscriptionsEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn CreateSubscriptionForSelection(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn DeleteSubscriptionForSelection(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRoot<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrfullpath: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), bstrfullpath.into_param().abi()).ok()
    }
    pub unsafe fn EnumOptions(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetEnumOptions(&self, lval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(lval)).ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn SelectedItem(&self) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn SetSelectedItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch>>(&self, pitem: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), pitem.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Root(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn SetRoot2<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, var: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), var.into_param().abi()).ok()
    }
    pub unsafe fn Depth(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetDepth(&self, idepth: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(idepth)).ok()
    }
    pub unsafe fn Mode(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetMode(&self, umode: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(umode)).ok()
    }
    pub unsafe fn Flags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn SetFlags(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn SetTVFlags(&self, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwflags)).ok()
    }
    pub unsafe fn TVFlags(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Columns(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetColumns<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcolumns: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), bstrcolumns.into_param().abi()).ok()
    }
    pub unsafe fn CountViewTypes(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetViewType(&self, itype: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), ::std::mem::transmute(itype)).ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn SelectedItems(&self) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Expand<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, var: Param0, idepth: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), var.into_param().abi(), ::std::mem::transmute(idepth)).ok()
    }
    pub unsafe fn UnselectAll(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IShellNameSpace {
    type Vtable = IShellNameSpace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3849507785, 14270, 19170, [130, 93, 213, 33, 118, 62, 49, 8]);
}
impl ::std::convert::From<IShellNameSpace> for ::windows::runtime::IUnknown {
    fn from(value: IShellNameSpace) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellNameSpace> for ::windows::runtime::IUnknown {
    fn from(value: &IShellNameSpace) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IShellNameSpace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IShellNameSpace {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellNameSpace> for IShellFavoritesNameSpace {
    fn from(value: IShellNameSpace) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellNameSpace> for IShellFavoritesNameSpace {
    fn from(value: &IShellNameSpace) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellFavoritesNameSpace> for IShellNameSpace {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellFavoritesNameSpace> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellFavoritesNameSpace>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellFavoritesNameSpace> for &IShellNameSpace {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellFavoritesNameSpace> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellFavoritesNameSpace>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IShellNameSpace> for super::Ole::Automation::IDispatch {
    fn from(value: IShellNameSpace) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IShellNameSpace> for super::Ole::Automation::IDispatch {
    fn from(value: &IShellNameSpace) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IShellNameSpace {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IShellNameSpace {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellNameSpace_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strcommand: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrfullpath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgrfenumflags: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lval: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitem: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvar: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, var: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidepth: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, idepth: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pumode: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, umode: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwflags: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcolumns: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrcolumns: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitypes: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itype: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppid: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, var: ::std::mem::ManuallyDrop<super::Com::VARIANT>, idepth: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IShellUIHelper(::windows::runtime::IUnknown);
impl IShellUIHelper {
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddFavorite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), url.into_param().abi(), ::std::mem::transmute(title)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddChannel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddDesktopComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, r#type: Param1, left: *const super::Com::VARIANT, top: *const super::Com::VARIANT, width: *const super::Com::VARIANT, height: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), url.into_param().abi(), r#type.into_param().abi(), ::std::mem::transmute(left), ::std::mem::transmute(top), ::std::mem::transmute(width), ::std::mem::transmute(height)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn NavigateAndFind<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, strquery: Param1, vartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), url.into_param().abi(), strquery.into_param().abi(), ::std::mem::transmute(vartargetframe)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fimport: i16, strimpexppath: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(fimport), strimpexppath.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(form)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoScan<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strsearch: Param0, strfailureurl: Param1, pvartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), strsearch.into_param().abi(), strfailureurl.into_param().abi(), ::std::mem::transmute(pvartargetframe)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(reserved)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ShowBrowserUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, pvarin: *const super::Com::VARIANT) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), ::std::mem::transmute(pvarin), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IShellUIHelper {
    type Vtable = IShellUIHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1923080952, 7848, 4561, [143, 133, 0, 192, 79, 194, 251, 225]);
}
impl ::std::convert::From<IShellUIHelper> for ::windows::runtime::IUnknown {
    fn from(value: IShellUIHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper> for ::windows::runtime::IUnknown {
    fn from(value: &IShellUIHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IShellUIHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IShellUIHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IShellUIHelper> for super::Ole::Automation::IDispatch {
    fn from(value: IShellUIHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IShellUIHelper> for super::Ole::Automation::IDispatch {
    fn from(value: &IShellUIHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IShellUIHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IShellUIHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, left: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, top: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, width: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, height: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fimport: i16, strimpexppath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, form: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strsearch: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strfailureurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pvarout: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IShellUIHelper2(::windows::runtime::IUnknown);
impl IShellUIHelper2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddFavorite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), url.into_param().abi(), ::std::mem::transmute(title)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddChannel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddDesktopComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, r#type: Param1, left: *const super::Com::VARIANT, top: *const super::Com::VARIANT, width: *const super::Com::VARIANT, height: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), url.into_param().abi(), r#type.into_param().abi(), ::std::mem::transmute(left), ::std::mem::transmute(top), ::std::mem::transmute(width), ::std::mem::transmute(height)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn NavigateAndFind<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, strquery: Param1, vartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), url.into_param().abi(), strquery.into_param().abi(), ::std::mem::transmute(vartargetframe)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fimport: i16, strimpexppath: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(fimport), strimpexppath.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(form)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoScan<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strsearch: Param0, strfailureurl: Param1, pvartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), strsearch.into_param().abi(), strfailureurl.into_param().abi(), ::std::mem::transmute(pvartargetframe)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(reserved)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ShowBrowserUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, pvarin: *const super::Com::VARIANT) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), ::std::mem::transmute(pvarin), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddSearchProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    pub unsafe fn RunOnceShown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SkipRunOnce(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeSettings<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fsqm: i16, fphishing: i16, bstrlocale: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(fsqm), ::std::mem::transmute(fphishing), bstrlocale.into_param().abi()).ok()
    }
    pub unsafe fn SqmEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn PhishingEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BrandImageUri(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SkipTabsWelcome(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DiagnoseConnection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CustomizeClearType(&self, fset: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(fset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSearchProviderInstalled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn IsSearchMigrated(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DefaultSearchProvider(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RunOnceRequiredSettingsComplete(&self, fcomplete: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(fcomplete)).ok()
    }
    pub unsafe fn RunOnceHasShown(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchGuideUrl(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IShellUIHelper2 {
    type Vtable = IShellUIHelper2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2818469594, 6450, 17025, [184, 129, 135, 179, 27, 139, 197, 44]);
}
impl ::std::convert::From<IShellUIHelper2> for ::windows::runtime::IUnknown {
    fn from(value: IShellUIHelper2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper2> for ::windows::runtime::IUnknown {
    fn from(value: &IShellUIHelper2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IShellUIHelper2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IShellUIHelper2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper2> for IShellUIHelper {
    fn from(value: IShellUIHelper2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper2> for IShellUIHelper {
    fn from(value: &IShellUIHelper2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for IShellUIHelper2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for &IShellUIHelper2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IShellUIHelper2> for super::Ole::Automation::IDispatch {
    fn from(value: IShellUIHelper2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IShellUIHelper2> for super::Ole::Automation::IDispatch {
    fn from(value: &IShellUIHelper2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IShellUIHelper2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IShellUIHelper2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, left: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, top: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, width: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, height: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fimport: i16, strimpexppath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, form: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strsearch: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strfailureurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pvarout: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fsqm: i16, fphishing: i16, bstrlocale: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstruri: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fset: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfmigrated: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fcomplete: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfshown: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrurl: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IShellUIHelper3(::windows::runtime::IUnknown);
impl IShellUIHelper3 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddFavorite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), url.into_param().abi(), ::std::mem::transmute(title)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddChannel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddDesktopComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, r#type: Param1, left: *const super::Com::VARIANT, top: *const super::Com::VARIANT, width: *const super::Com::VARIANT, height: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), url.into_param().abi(), r#type.into_param().abi(), ::std::mem::transmute(left), ::std::mem::transmute(top), ::std::mem::transmute(width), ::std::mem::transmute(height)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn NavigateAndFind<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, strquery: Param1, vartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), url.into_param().abi(), strquery.into_param().abi(), ::std::mem::transmute(vartargetframe)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fimport: i16, strimpexppath: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(fimport), strimpexppath.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(form)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoScan<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strsearch: Param0, strfailureurl: Param1, pvartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), strsearch.into_param().abi(), strfailureurl.into_param().abi(), ::std::mem::transmute(pvartargetframe)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(reserved)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ShowBrowserUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, pvarin: *const super::Com::VARIANT) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), ::std::mem::transmute(pvarin), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddSearchProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    pub unsafe fn RunOnceShown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SkipRunOnce(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeSettings<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fsqm: i16, fphishing: i16, bstrlocale: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(fsqm), ::std::mem::transmute(fphishing), bstrlocale.into_param().abi()).ok()
    }
    pub unsafe fn SqmEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn PhishingEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BrandImageUri(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SkipTabsWelcome(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DiagnoseConnection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CustomizeClearType(&self, fset: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(fset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSearchProviderInstalled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn IsSearchMigrated(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DefaultSearchProvider(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RunOnceRequiredSettingsComplete(&self, fcomplete: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(fcomplete)).ok()
    }
    pub unsafe fn RunOnceHasShown(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchGuideUrl(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsServiceInstalled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, verb: Param1) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), url.into_param().abi(), verb.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn InPrivateFilteringEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddToFavoritesBar<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: Param1, r#type: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), url.into_param().abi(), title.into_param().abi(), ::std::mem::transmute(r#type)).ok()
    }
    pub unsafe fn BuildNewTabPage(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetRecentlyClosedVisible(&self, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(fvisible)).ok()
    }
    pub unsafe fn SetActivitiesVisible(&self, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), ::std::mem::transmute(fvisible)).ok()
    }
    pub unsafe fn ContentDiscoveryReset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsSuggestedSitesEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn EnableSuggestedSites(&self, fenable: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), ::std::mem::transmute(fenable)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NavigateToSuggestedSites<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrelativeurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), bstrrelativeurl.into_param().abi()).ok()
    }
    pub unsafe fn ShowTabsHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ShowInPrivateHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IShellUIHelper3 {
    type Vtable = IShellUIHelper3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1385034476, 54297, 16572, [155, 109, 220, 219, 249, 193, 178, 93]);
}
impl ::std::convert::From<IShellUIHelper3> for ::windows::runtime::IUnknown {
    fn from(value: IShellUIHelper3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper3> for ::windows::runtime::IUnknown {
    fn from(value: &IShellUIHelper3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IShellUIHelper3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IShellUIHelper3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper3> for IShellUIHelper2 {
    fn from(value: IShellUIHelper3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper3> for IShellUIHelper2 {
    fn from(value: &IShellUIHelper3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper2> for IShellUIHelper3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper2> for &IShellUIHelper3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper3> for IShellUIHelper {
    fn from(value: IShellUIHelper3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper3> for IShellUIHelper {
    fn from(value: &IShellUIHelper3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for IShellUIHelper3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for &IShellUIHelper3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IShellUIHelper3> for super::Ole::Automation::IDispatch {
    fn from(value: IShellUIHelper3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IShellUIHelper3> for super::Ole::Automation::IDispatch {
    fn from(value: &IShellUIHelper3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IShellUIHelper3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IShellUIHelper3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, left: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, top: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, width: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, height: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fimport: i16, strimpexppath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, form: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strsearch: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strfailureurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pvarout: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fsqm: i16, fphishing: i16, bstrlocale: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstruri: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fset: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfmigrated: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fcomplete: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfshown: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrurl: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, verb: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fvisible: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fvisible: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrelativeurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IShellUIHelper4(::windows::runtime::IUnknown);
impl IShellUIHelper4 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddFavorite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), url.into_param().abi(), ::std::mem::transmute(title)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddChannel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddDesktopComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, r#type: Param1, left: *const super::Com::VARIANT, top: *const super::Com::VARIANT, width: *const super::Com::VARIANT, height: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), url.into_param().abi(), r#type.into_param().abi(), ::std::mem::transmute(left), ::std::mem::transmute(top), ::std::mem::transmute(width), ::std::mem::transmute(height)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn NavigateAndFind<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, strquery: Param1, vartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), url.into_param().abi(), strquery.into_param().abi(), ::std::mem::transmute(vartargetframe)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fimport: i16, strimpexppath: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(fimport), strimpexppath.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(form)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoScan<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strsearch: Param0, strfailureurl: Param1, pvartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), strsearch.into_param().abi(), strfailureurl.into_param().abi(), ::std::mem::transmute(pvartargetframe)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(reserved)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ShowBrowserUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, pvarin: *const super::Com::VARIANT) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), ::std::mem::transmute(pvarin), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddSearchProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    pub unsafe fn RunOnceShown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SkipRunOnce(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeSettings<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fsqm: i16, fphishing: i16, bstrlocale: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(fsqm), ::std::mem::transmute(fphishing), bstrlocale.into_param().abi()).ok()
    }
    pub unsafe fn SqmEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn PhishingEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BrandImageUri(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SkipTabsWelcome(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DiagnoseConnection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CustomizeClearType(&self, fset: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(fset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSearchProviderInstalled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn IsSearchMigrated(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DefaultSearchProvider(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RunOnceRequiredSettingsComplete(&self, fcomplete: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(fcomplete)).ok()
    }
    pub unsafe fn RunOnceHasShown(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchGuideUrl(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsServiceInstalled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, verb: Param1) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), url.into_param().abi(), verb.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn InPrivateFilteringEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddToFavoritesBar<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: Param1, r#type: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), url.into_param().abi(), title.into_param().abi(), ::std::mem::transmute(r#type)).ok()
    }
    pub unsafe fn BuildNewTabPage(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetRecentlyClosedVisible(&self, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(fvisible)).ok()
    }
    pub unsafe fn SetActivitiesVisible(&self, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), ::std::mem::transmute(fvisible)).ok()
    }
    pub unsafe fn ContentDiscoveryReset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsSuggestedSitesEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn EnableSuggestedSites(&self, fenable: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), ::std::mem::transmute(fenable)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NavigateToSuggestedSites<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrelativeurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), bstrrelativeurl.into_param().abi()).ok()
    }
    pub unsafe fn ShowTabsHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ShowInPrivateHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msIsSiteMode(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn msSiteModeShowThumbBar(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddThumbBarButton<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstriconurl: Param0, bstrtooltip: Param1) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::std::mem::transmute_copy(self), bstriconurl.into_param().abi(), bstrtooltip.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeUpdateThumbBarButton<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, buttonid: Param0, fenabled: i16, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::std::mem::transmute_copy(self), buttonid.into_param().abi(), ::std::mem::transmute(fenabled), ::std::mem::transmute(fvisible)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeSetIconOverlay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, iconurl: Param0, pvardescription: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::std::mem::transmute_copy(self), iconurl.into_param().abi(), ::std::mem::transmute(pvardescription)).ok()
    }
    pub unsafe fn msSiteModeClearIconOverlay(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msAddSiteMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msSiteModeCreateJumpList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrheader: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::std::mem::transmute_copy(self), bstrheader.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddJumpListItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, bstractionuri: Param1, bstriconuri: Param2, pvarwindowtype: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).57)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), bstractionuri.into_param().abi(), bstriconuri.into_param().abi(), ::std::mem::transmute(pvarwindowtype)).ok()
    }
    pub unsafe fn msSiteModeClearJumpList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeShowJumpList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).59)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddButtonStyle<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, uibuttonid: Param0, bstriconurl: Param1, bstrtooltip: Param2) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::std::mem::transmute_copy(self), uibuttonid.into_param().abi(), bstriconurl.into_param().abi(), bstrtooltip.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeShowButtonStyle<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, uibuttonid: Param0, uistyleid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).61)(::std::mem::transmute_copy(self), uibuttonid.into_param().abi(), uistyleid.into_param().abi()).ok()
    }
    pub unsafe fn msSiteModeActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).62)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msIsSiteModeFirstRun(&self, fpreservestate: i16) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::std::mem::transmute_copy(self), ::std::mem::transmute(fpreservestate), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msAddTrackingProtectionList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, bstrfiltername: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).64)(::std::mem::transmute_copy(self), url.into_param().abi(), bstrfiltername.into_param().abi()).ok()
    }
    pub unsafe fn msTrackingProtectionEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).65)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn msActiveXFilteringEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).66)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IShellUIHelper4 {
    type Vtable = IShellUIHelper4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3010357843, 32883, 18846, [130, 76, 215, 118, 51, 10, 51, 62]);
}
impl ::std::convert::From<IShellUIHelper4> for ::windows::runtime::IUnknown {
    fn from(value: IShellUIHelper4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper4> for ::windows::runtime::IUnknown {
    fn from(value: &IShellUIHelper4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IShellUIHelper4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IShellUIHelper4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper4> for IShellUIHelper3 {
    fn from(value: IShellUIHelper4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper4> for IShellUIHelper3 {
    fn from(value: &IShellUIHelper4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper3> for IShellUIHelper4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper3> for &IShellUIHelper4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper3>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper4> for IShellUIHelper2 {
    fn from(value: IShellUIHelper4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper4> for IShellUIHelper2 {
    fn from(value: &IShellUIHelper4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper2> for IShellUIHelper4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper2> for &IShellUIHelper4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper4> for IShellUIHelper {
    fn from(value: IShellUIHelper4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper4> for IShellUIHelper {
    fn from(value: &IShellUIHelper4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for IShellUIHelper4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for &IShellUIHelper4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IShellUIHelper4> for super::Ole::Automation::IDispatch {
    fn from(value: IShellUIHelper4) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IShellUIHelper4> for super::Ole::Automation::IDispatch {
    fn from(value: &IShellUIHelper4) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IShellUIHelper4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IShellUIHelper4 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, left: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, top: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, width: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, height: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fimport: i16, strimpexppath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, form: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strsearch: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strfailureurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pvarout: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fsqm: i16, fphishing: i16, bstrlocale: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstruri: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fset: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfmigrated: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fcomplete: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfshown: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrurl: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, verb: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fvisible: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fvisible: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrelativeurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfsitemode: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstriconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtooltip: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarbuttonid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, fenabled: i16, fvisible: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardescription: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrheader: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstractionuri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstriconuri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarwindowtype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uibuttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, bstriconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtooltip: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarstyleid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uibuttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uistyleid: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fpreservestate: i16, puifirstrun: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfiltername: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IShellUIHelper5(::windows::runtime::IUnknown);
impl IShellUIHelper5 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddFavorite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), url.into_param().abi(), ::std::mem::transmute(title)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddChannel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddDesktopComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, r#type: Param1, left: *const super::Com::VARIANT, top: *const super::Com::VARIANT, width: *const super::Com::VARIANT, height: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), url.into_param().abi(), r#type.into_param().abi(), ::std::mem::transmute(left), ::std::mem::transmute(top), ::std::mem::transmute(width), ::std::mem::transmute(height)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn NavigateAndFind<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, strquery: Param1, vartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), url.into_param().abi(), strquery.into_param().abi(), ::std::mem::transmute(vartargetframe)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fimport: i16, strimpexppath: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(fimport), strimpexppath.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(form)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoScan<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strsearch: Param0, strfailureurl: Param1, pvartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), strsearch.into_param().abi(), strfailureurl.into_param().abi(), ::std::mem::transmute(pvartargetframe)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(reserved)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ShowBrowserUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, pvarin: *const super::Com::VARIANT) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), ::std::mem::transmute(pvarin), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddSearchProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    pub unsafe fn RunOnceShown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SkipRunOnce(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeSettings<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fsqm: i16, fphishing: i16, bstrlocale: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(fsqm), ::std::mem::transmute(fphishing), bstrlocale.into_param().abi()).ok()
    }
    pub unsafe fn SqmEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn PhishingEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BrandImageUri(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SkipTabsWelcome(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DiagnoseConnection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CustomizeClearType(&self, fset: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(fset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSearchProviderInstalled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn IsSearchMigrated(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DefaultSearchProvider(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RunOnceRequiredSettingsComplete(&self, fcomplete: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(fcomplete)).ok()
    }
    pub unsafe fn RunOnceHasShown(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchGuideUrl(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsServiceInstalled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, verb: Param1) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), url.into_param().abi(), verb.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn InPrivateFilteringEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddToFavoritesBar<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: Param1, r#type: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), url.into_param().abi(), title.into_param().abi(), ::std::mem::transmute(r#type)).ok()
    }
    pub unsafe fn BuildNewTabPage(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetRecentlyClosedVisible(&self, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(fvisible)).ok()
    }
    pub unsafe fn SetActivitiesVisible(&self, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), ::std::mem::transmute(fvisible)).ok()
    }
    pub unsafe fn ContentDiscoveryReset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsSuggestedSitesEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn EnableSuggestedSites(&self, fenable: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), ::std::mem::transmute(fenable)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NavigateToSuggestedSites<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrelativeurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), bstrrelativeurl.into_param().abi()).ok()
    }
    pub unsafe fn ShowTabsHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ShowInPrivateHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msIsSiteMode(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn msSiteModeShowThumbBar(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddThumbBarButton<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstriconurl: Param0, bstrtooltip: Param1) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::std::mem::transmute_copy(self), bstriconurl.into_param().abi(), bstrtooltip.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeUpdateThumbBarButton<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, buttonid: Param0, fenabled: i16, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::std::mem::transmute_copy(self), buttonid.into_param().abi(), ::std::mem::transmute(fenabled), ::std::mem::transmute(fvisible)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeSetIconOverlay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, iconurl: Param0, pvardescription: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::std::mem::transmute_copy(self), iconurl.into_param().abi(), ::std::mem::transmute(pvardescription)).ok()
    }
    pub unsafe fn msSiteModeClearIconOverlay(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msAddSiteMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msSiteModeCreateJumpList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrheader: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::std::mem::transmute_copy(self), bstrheader.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddJumpListItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, bstractionuri: Param1, bstriconuri: Param2, pvarwindowtype: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).57)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), bstractionuri.into_param().abi(), bstriconuri.into_param().abi(), ::std::mem::transmute(pvarwindowtype)).ok()
    }
    pub unsafe fn msSiteModeClearJumpList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeShowJumpList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).59)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddButtonStyle<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, uibuttonid: Param0, bstriconurl: Param1, bstrtooltip: Param2) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::std::mem::transmute_copy(self), uibuttonid.into_param().abi(), bstriconurl.into_param().abi(), bstrtooltip.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeShowButtonStyle<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, uibuttonid: Param0, uistyleid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).61)(::std::mem::transmute_copy(self), uibuttonid.into_param().abi(), uistyleid.into_param().abi()).ok()
    }
    pub unsafe fn msSiteModeActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).62)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msIsSiteModeFirstRun(&self, fpreservestate: i16) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::std::mem::transmute_copy(self), ::std::mem::transmute(fpreservestate), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msAddTrackingProtectionList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, bstrfiltername: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).64)(::std::mem::transmute_copy(self), url.into_param().abi(), bstrfiltername.into_param().abi()).ok()
    }
    pub unsafe fn msTrackingProtectionEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).65)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn msActiveXFilteringEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).66)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msProvisionNetworks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprovisioningxml: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).67)(::std::mem::transmute_copy(self), bstrprovisioningxml.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn msReportSafeUrl(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).68)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeRefreshBadge(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).69)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeClearBadge(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).70)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msDiagnoseConnectionUILess(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).71)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msLaunchNetworkClientHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).72)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msChangeDefaultBrowser(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).73)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IShellUIHelper5 {
    type Vtable = IShellUIHelper5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2728430345, 4157, 19775, [185, 28, 234, 69, 92, 168, 46, 250]);
}
impl ::std::convert::From<IShellUIHelper5> for ::windows::runtime::IUnknown {
    fn from(value: IShellUIHelper5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper5> for ::windows::runtime::IUnknown {
    fn from(value: &IShellUIHelper5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IShellUIHelper5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IShellUIHelper5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper5> for IShellUIHelper4 {
    fn from(value: IShellUIHelper5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper5> for IShellUIHelper4 {
    fn from(value: &IShellUIHelper5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper4> for IShellUIHelper5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper4>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper4> for &IShellUIHelper5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper4>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper5> for IShellUIHelper3 {
    fn from(value: IShellUIHelper5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper5> for IShellUIHelper3 {
    fn from(value: &IShellUIHelper5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper3> for IShellUIHelper5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper3> for &IShellUIHelper5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper3>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper5> for IShellUIHelper2 {
    fn from(value: IShellUIHelper5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper5> for IShellUIHelper2 {
    fn from(value: &IShellUIHelper5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper2> for IShellUIHelper5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper2> for &IShellUIHelper5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper5> for IShellUIHelper {
    fn from(value: IShellUIHelper5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper5> for IShellUIHelper {
    fn from(value: &IShellUIHelper5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for IShellUIHelper5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for &IShellUIHelper5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IShellUIHelper5> for super::Ole::Automation::IDispatch {
    fn from(value: IShellUIHelper5) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IShellUIHelper5> for super::Ole::Automation::IDispatch {
    fn from(value: &IShellUIHelper5) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IShellUIHelper5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IShellUIHelper5 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, left: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, top: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, width: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, height: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fimport: i16, strimpexppath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, form: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strsearch: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strfailureurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pvarout: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fsqm: i16, fphishing: i16, bstrlocale: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstruri: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fset: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfmigrated: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fcomplete: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfshown: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrurl: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, verb: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fvisible: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fvisible: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrelativeurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfsitemode: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstriconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtooltip: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarbuttonid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, fenabled: i16, fvisible: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardescription: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrheader: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstractionuri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstriconuri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarwindowtype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uibuttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, bstriconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtooltip: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarstyleid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uibuttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uistyleid: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fpreservestate: i16, puifirstrun: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfiltername: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrprovisioningxml: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, puiresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IShellUIHelper6(::windows::runtime::IUnknown);
impl IShellUIHelper6 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddFavorite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), url.into_param().abi(), ::std::mem::transmute(title)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddChannel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddDesktopComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, r#type: Param1, left: *const super::Com::VARIANT, top: *const super::Com::VARIANT, width: *const super::Com::VARIANT, height: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), url.into_param().abi(), r#type.into_param().abi(), ::std::mem::transmute(left), ::std::mem::transmute(top), ::std::mem::transmute(width), ::std::mem::transmute(height)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn NavigateAndFind<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, strquery: Param1, vartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), url.into_param().abi(), strquery.into_param().abi(), ::std::mem::transmute(vartargetframe)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fimport: i16, strimpexppath: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(fimport), strimpexppath.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(form)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoScan<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strsearch: Param0, strfailureurl: Param1, pvartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), strsearch.into_param().abi(), strfailureurl.into_param().abi(), ::std::mem::transmute(pvartargetframe)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(reserved)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ShowBrowserUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, pvarin: *const super::Com::VARIANT) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), ::std::mem::transmute(pvarin), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddSearchProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    pub unsafe fn RunOnceShown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SkipRunOnce(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeSettings<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fsqm: i16, fphishing: i16, bstrlocale: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(fsqm), ::std::mem::transmute(fphishing), bstrlocale.into_param().abi()).ok()
    }
    pub unsafe fn SqmEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn PhishingEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BrandImageUri(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SkipTabsWelcome(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DiagnoseConnection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CustomizeClearType(&self, fset: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(fset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSearchProviderInstalled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn IsSearchMigrated(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DefaultSearchProvider(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RunOnceRequiredSettingsComplete(&self, fcomplete: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(fcomplete)).ok()
    }
    pub unsafe fn RunOnceHasShown(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchGuideUrl(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsServiceInstalled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, verb: Param1) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), url.into_param().abi(), verb.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn InPrivateFilteringEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddToFavoritesBar<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: Param1, r#type: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), url.into_param().abi(), title.into_param().abi(), ::std::mem::transmute(r#type)).ok()
    }
    pub unsafe fn BuildNewTabPage(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetRecentlyClosedVisible(&self, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(fvisible)).ok()
    }
    pub unsafe fn SetActivitiesVisible(&self, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), ::std::mem::transmute(fvisible)).ok()
    }
    pub unsafe fn ContentDiscoveryReset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsSuggestedSitesEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn EnableSuggestedSites(&self, fenable: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), ::std::mem::transmute(fenable)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NavigateToSuggestedSites<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrelativeurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), bstrrelativeurl.into_param().abi()).ok()
    }
    pub unsafe fn ShowTabsHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ShowInPrivateHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msIsSiteMode(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn msSiteModeShowThumbBar(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddThumbBarButton<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstriconurl: Param0, bstrtooltip: Param1) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::std::mem::transmute_copy(self), bstriconurl.into_param().abi(), bstrtooltip.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeUpdateThumbBarButton<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, buttonid: Param0, fenabled: i16, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::std::mem::transmute_copy(self), buttonid.into_param().abi(), ::std::mem::transmute(fenabled), ::std::mem::transmute(fvisible)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeSetIconOverlay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, iconurl: Param0, pvardescription: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::std::mem::transmute_copy(self), iconurl.into_param().abi(), ::std::mem::transmute(pvardescription)).ok()
    }
    pub unsafe fn msSiteModeClearIconOverlay(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msAddSiteMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msSiteModeCreateJumpList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrheader: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::std::mem::transmute_copy(self), bstrheader.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddJumpListItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, bstractionuri: Param1, bstriconuri: Param2, pvarwindowtype: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).57)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), bstractionuri.into_param().abi(), bstriconuri.into_param().abi(), ::std::mem::transmute(pvarwindowtype)).ok()
    }
    pub unsafe fn msSiteModeClearJumpList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeShowJumpList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).59)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddButtonStyle<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, uibuttonid: Param0, bstriconurl: Param1, bstrtooltip: Param2) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::std::mem::transmute_copy(self), uibuttonid.into_param().abi(), bstriconurl.into_param().abi(), bstrtooltip.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeShowButtonStyle<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, uibuttonid: Param0, uistyleid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).61)(::std::mem::transmute_copy(self), uibuttonid.into_param().abi(), uistyleid.into_param().abi()).ok()
    }
    pub unsafe fn msSiteModeActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).62)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msIsSiteModeFirstRun(&self, fpreservestate: i16) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::std::mem::transmute_copy(self), ::std::mem::transmute(fpreservestate), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msAddTrackingProtectionList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, bstrfiltername: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).64)(::std::mem::transmute_copy(self), url.into_param().abi(), bstrfiltername.into_param().abi()).ok()
    }
    pub unsafe fn msTrackingProtectionEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).65)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn msActiveXFilteringEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).66)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msProvisionNetworks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprovisioningxml: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).67)(::std::mem::transmute_copy(self), bstrprovisioningxml.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn msReportSafeUrl(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).68)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeRefreshBadge(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).69)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeClearBadge(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).70)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msDiagnoseConnectionUILess(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).71)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msLaunchNetworkClientHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).72)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msChangeDefaultBrowser(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).73)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    pub unsafe fn msStopPeriodicTileUpdate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).74)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msStartPeriodicTileUpdate<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param2: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, pollinguris: Param0, starttime: Param1, uiupdaterecurrence: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).75)(::std::mem::transmute_copy(self), pollinguris.into_param().abi(), starttime.into_param().abi(), uiupdaterecurrence.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msStartPeriodicTileUpdateBatch<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param2: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, pollinguris: Param0, starttime: Param1, uiupdaterecurrence: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).76)(::std::mem::transmute_copy(self), pollinguris.into_param().abi(), starttime.into_param().abi(), uiupdaterecurrence.into_param().abi()).ok()
    }
    pub unsafe fn msClearTile(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).77)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msEnableTileNotificationQueue(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).78)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msPinnedSiteState(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).79)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn msEnableTileNotificationQueueForSquare150x150(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).80)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    pub unsafe fn msEnableTileNotificationQueueForWide310x150(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).81)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    pub unsafe fn msEnableTileNotificationQueueForSquare310x310(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).82)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msScheduledTileNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param4: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(
        &self,
        bstrnotificationxml: Param0,
        bstrnotificationid: Param1,
        bstrnotificationtag: Param2,
        starttime: Param3,
        expirationtime: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).83)(::std::mem::transmute_copy(self), bstrnotificationxml.into_param().abi(), bstrnotificationid.into_param().abi(), bstrnotificationtag.into_param().abi(), starttime.into_param().abi(), expirationtime.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msRemoveScheduledTileNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrnotificationid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).84)(::std::mem::transmute_copy(self), bstrnotificationid.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msStartPeriodicBadgeUpdate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param2: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, pollinguri: Param0, starttime: Param1, uiupdaterecurrence: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).85)(::std::mem::transmute_copy(self), pollinguri.into_param().abi(), starttime.into_param().abi(), uiupdaterecurrence.into_param().abi()).ok()
    }
    pub unsafe fn msStopPeriodicBadgeUpdate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).86)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msLaunchInternetOptions(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).87)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IShellUIHelper6 {
    type Vtable = IShellUIHelper6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2558154558, 18158, 20105, [150, 171, 221, 247, 248, 253, 201, 140]);
}
impl ::std::convert::From<IShellUIHelper6> for ::windows::runtime::IUnknown {
    fn from(value: IShellUIHelper6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper6> for ::windows::runtime::IUnknown {
    fn from(value: &IShellUIHelper6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IShellUIHelper6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IShellUIHelper6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper6> for IShellUIHelper5 {
    fn from(value: IShellUIHelper6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper6> for IShellUIHelper5 {
    fn from(value: &IShellUIHelper6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper5> for IShellUIHelper6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper5>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper5> for &IShellUIHelper6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper5>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper6> for IShellUIHelper4 {
    fn from(value: IShellUIHelper6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper6> for IShellUIHelper4 {
    fn from(value: &IShellUIHelper6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper4> for IShellUIHelper6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper4>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper4> for &IShellUIHelper6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper4>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper6> for IShellUIHelper3 {
    fn from(value: IShellUIHelper6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper6> for IShellUIHelper3 {
    fn from(value: &IShellUIHelper6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper3> for IShellUIHelper6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper3> for &IShellUIHelper6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper3>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper6> for IShellUIHelper2 {
    fn from(value: IShellUIHelper6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper6> for IShellUIHelper2 {
    fn from(value: &IShellUIHelper6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper2> for IShellUIHelper6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper2> for &IShellUIHelper6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper6> for IShellUIHelper {
    fn from(value: IShellUIHelper6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper6> for IShellUIHelper {
    fn from(value: &IShellUIHelper6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for IShellUIHelper6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for &IShellUIHelper6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IShellUIHelper6> for super::Ole::Automation::IDispatch {
    fn from(value: IShellUIHelper6) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IShellUIHelper6> for super::Ole::Automation::IDispatch {
    fn from(value: &IShellUIHelper6) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IShellUIHelper6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IShellUIHelper6 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper6_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, left: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, top: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, width: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, height: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fimport: i16, strimpexppath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, form: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strsearch: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strfailureurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pvarout: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fsqm: i16, fphishing: i16, bstrlocale: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstruri: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fset: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfmigrated: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fcomplete: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfshown: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrurl: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, verb: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fvisible: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fvisible: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrelativeurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfsitemode: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstriconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtooltip: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarbuttonid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, fenabled: i16, fvisible: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardescription: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrheader: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstractionuri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstriconuri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarwindowtype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uibuttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, bstriconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtooltip: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarstyleid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uibuttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uistyleid: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fpreservestate: i16, puifirstrun: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfiltername: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrprovisioningxml: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, puiresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pollinguris: ::std::mem::ManuallyDrop<super::Com::VARIANT>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uiupdaterecurrence: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pollinguris: ::std::mem::ManuallyDrop<super::Com::VARIANT>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uiupdaterecurrence: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarsitestate: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrnotificationxml: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnotificationid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnotificationtag: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, expirationtime: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrnotificationid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pollinguri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uiupdaterecurrence: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IShellUIHelper7(::windows::runtime::IUnknown);
impl IShellUIHelper7 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddFavorite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), url.into_param().abi(), ::std::mem::transmute(title)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddChannel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddDesktopComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, r#type: Param1, left: *const super::Com::VARIANT, top: *const super::Com::VARIANT, width: *const super::Com::VARIANT, height: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), url.into_param().abi(), r#type.into_param().abi(), ::std::mem::transmute(left), ::std::mem::transmute(top), ::std::mem::transmute(width), ::std::mem::transmute(height)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn NavigateAndFind<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, strquery: Param1, vartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), url.into_param().abi(), strquery.into_param().abi(), ::std::mem::transmute(vartargetframe)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fimport: i16, strimpexppath: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(fimport), strimpexppath.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(form)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoScan<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strsearch: Param0, strfailureurl: Param1, pvartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), strsearch.into_param().abi(), strfailureurl.into_param().abi(), ::std::mem::transmute(pvartargetframe)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(reserved)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ShowBrowserUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, pvarin: *const super::Com::VARIANT) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), ::std::mem::transmute(pvarin), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddSearchProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    pub unsafe fn RunOnceShown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SkipRunOnce(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeSettings<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fsqm: i16, fphishing: i16, bstrlocale: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(fsqm), ::std::mem::transmute(fphishing), bstrlocale.into_param().abi()).ok()
    }
    pub unsafe fn SqmEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn PhishingEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BrandImageUri(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SkipTabsWelcome(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DiagnoseConnection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CustomizeClearType(&self, fset: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(fset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSearchProviderInstalled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn IsSearchMigrated(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DefaultSearchProvider(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RunOnceRequiredSettingsComplete(&self, fcomplete: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(fcomplete)).ok()
    }
    pub unsafe fn RunOnceHasShown(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchGuideUrl(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsServiceInstalled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, verb: Param1) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), url.into_param().abi(), verb.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn InPrivateFilteringEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddToFavoritesBar<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: Param1, r#type: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), url.into_param().abi(), title.into_param().abi(), ::std::mem::transmute(r#type)).ok()
    }
    pub unsafe fn BuildNewTabPage(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetRecentlyClosedVisible(&self, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(fvisible)).ok()
    }
    pub unsafe fn SetActivitiesVisible(&self, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), ::std::mem::transmute(fvisible)).ok()
    }
    pub unsafe fn ContentDiscoveryReset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsSuggestedSitesEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn EnableSuggestedSites(&self, fenable: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), ::std::mem::transmute(fenable)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NavigateToSuggestedSites<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrelativeurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), bstrrelativeurl.into_param().abi()).ok()
    }
    pub unsafe fn ShowTabsHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ShowInPrivateHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msIsSiteMode(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn msSiteModeShowThumbBar(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddThumbBarButton<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstriconurl: Param0, bstrtooltip: Param1) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::std::mem::transmute_copy(self), bstriconurl.into_param().abi(), bstrtooltip.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeUpdateThumbBarButton<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, buttonid: Param0, fenabled: i16, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::std::mem::transmute_copy(self), buttonid.into_param().abi(), ::std::mem::transmute(fenabled), ::std::mem::transmute(fvisible)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeSetIconOverlay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, iconurl: Param0, pvardescription: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::std::mem::transmute_copy(self), iconurl.into_param().abi(), ::std::mem::transmute(pvardescription)).ok()
    }
    pub unsafe fn msSiteModeClearIconOverlay(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msAddSiteMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msSiteModeCreateJumpList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrheader: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::std::mem::transmute_copy(self), bstrheader.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddJumpListItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, bstractionuri: Param1, bstriconuri: Param2, pvarwindowtype: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).57)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), bstractionuri.into_param().abi(), bstriconuri.into_param().abi(), ::std::mem::transmute(pvarwindowtype)).ok()
    }
    pub unsafe fn msSiteModeClearJumpList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeShowJumpList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).59)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddButtonStyle<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, uibuttonid: Param0, bstriconurl: Param1, bstrtooltip: Param2) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::std::mem::transmute_copy(self), uibuttonid.into_param().abi(), bstriconurl.into_param().abi(), bstrtooltip.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeShowButtonStyle<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, uibuttonid: Param0, uistyleid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).61)(::std::mem::transmute_copy(self), uibuttonid.into_param().abi(), uistyleid.into_param().abi()).ok()
    }
    pub unsafe fn msSiteModeActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).62)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msIsSiteModeFirstRun(&self, fpreservestate: i16) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::std::mem::transmute_copy(self), ::std::mem::transmute(fpreservestate), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msAddTrackingProtectionList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, bstrfiltername: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).64)(::std::mem::transmute_copy(self), url.into_param().abi(), bstrfiltername.into_param().abi()).ok()
    }
    pub unsafe fn msTrackingProtectionEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).65)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn msActiveXFilteringEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).66)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msProvisionNetworks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprovisioningxml: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).67)(::std::mem::transmute_copy(self), bstrprovisioningxml.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn msReportSafeUrl(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).68)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeRefreshBadge(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).69)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeClearBadge(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).70)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msDiagnoseConnectionUILess(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).71)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msLaunchNetworkClientHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).72)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msChangeDefaultBrowser(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).73)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    pub unsafe fn msStopPeriodicTileUpdate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).74)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msStartPeriodicTileUpdate<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param2: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, pollinguris: Param0, starttime: Param1, uiupdaterecurrence: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).75)(::std::mem::transmute_copy(self), pollinguris.into_param().abi(), starttime.into_param().abi(), uiupdaterecurrence.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msStartPeriodicTileUpdateBatch<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param2: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, pollinguris: Param0, starttime: Param1, uiupdaterecurrence: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).76)(::std::mem::transmute_copy(self), pollinguris.into_param().abi(), starttime.into_param().abi(), uiupdaterecurrence.into_param().abi()).ok()
    }
    pub unsafe fn msClearTile(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).77)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msEnableTileNotificationQueue(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).78)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msPinnedSiteState(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).79)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn msEnableTileNotificationQueueForSquare150x150(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).80)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    pub unsafe fn msEnableTileNotificationQueueForWide310x150(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).81)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    pub unsafe fn msEnableTileNotificationQueueForSquare310x310(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).82)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msScheduledTileNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param4: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(
        &self,
        bstrnotificationxml: Param0,
        bstrnotificationid: Param1,
        bstrnotificationtag: Param2,
        starttime: Param3,
        expirationtime: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).83)(::std::mem::transmute_copy(self), bstrnotificationxml.into_param().abi(), bstrnotificationid.into_param().abi(), bstrnotificationtag.into_param().abi(), starttime.into_param().abi(), expirationtime.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msRemoveScheduledTileNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrnotificationid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).84)(::std::mem::transmute_copy(self), bstrnotificationid.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msStartPeriodicBadgeUpdate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param2: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, pollinguri: Param0, starttime: Param1, uiupdaterecurrence: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).85)(::std::mem::transmute_copy(self), pollinguri.into_param().abi(), starttime.into_param().abi(), uiupdaterecurrence.into_param().abi()).ok()
    }
    pub unsafe fn msStopPeriodicBadgeUpdate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).86)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msLaunchInternetOptions(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).87)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExperimentalFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrflagstring: Param0, vfflag: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).88)(::std::mem::transmute_copy(self), bstrflagstring.into_param().abi(), ::std::mem::transmute(vfflag)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExperimentalFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrflagstring: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).89)(::std::mem::transmute_copy(self), bstrflagstring.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExperimentalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrvaluestring: Param0, dwvalue: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).90)(::std::mem::transmute_copy(self), bstrvaluestring.into_param().abi(), ::std::mem::transmute(dwvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExperimentalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrvaluestring: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).91)(::std::mem::transmute_copy(self), bstrvaluestring.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn ResetAllExperimentalFlagsAndValues(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).92)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNeedIEAutoLaunchFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrurl: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).93)(::std::mem::transmute_copy(self), bstrurl.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNeedIEAutoLaunchFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrurl: Param0, flag: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).94)(::std::mem::transmute_copy(self), bstrurl.into_param().abi(), ::std::mem::transmute(flag)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasNeedIEAutoLaunchFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrurl: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).95)(::std::mem::transmute_copy(self), bstrurl.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LaunchIE<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrurl: Param0, automated: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).96)(::std::mem::transmute_copy(self), bstrurl.into_param().abi(), ::std::mem::transmute(automated)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IShellUIHelper7 {
    type Vtable = IShellUIHelper7_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1625647048, 38259, 19122, [162, 100, 99, 124, 108, 22, 28, 177]);
}
impl ::std::convert::From<IShellUIHelper7> for ::windows::runtime::IUnknown {
    fn from(value: IShellUIHelper7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper7> for ::windows::runtime::IUnknown {
    fn from(value: &IShellUIHelper7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper7> for IShellUIHelper6 {
    fn from(value: IShellUIHelper7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper7> for IShellUIHelper6 {
    fn from(value: &IShellUIHelper7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper6> for IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper6> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper6>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper6> for &IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper6> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper6>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper7> for IShellUIHelper5 {
    fn from(value: IShellUIHelper7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper7> for IShellUIHelper5 {
    fn from(value: &IShellUIHelper7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper5> for IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper5>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper5> for &IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper5>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper7> for IShellUIHelper4 {
    fn from(value: IShellUIHelper7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper7> for IShellUIHelper4 {
    fn from(value: &IShellUIHelper7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper4> for IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper4>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper4> for &IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper4>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper7> for IShellUIHelper3 {
    fn from(value: IShellUIHelper7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper7> for IShellUIHelper3 {
    fn from(value: &IShellUIHelper7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper3> for IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper3> for &IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper3>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper7> for IShellUIHelper2 {
    fn from(value: IShellUIHelper7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper7> for IShellUIHelper2 {
    fn from(value: &IShellUIHelper7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper2> for IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper2> for &IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper7> for IShellUIHelper {
    fn from(value: IShellUIHelper7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper7> for IShellUIHelper {
    fn from(value: &IShellUIHelper7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for &IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IShellUIHelper7> for super::Ole::Automation::IDispatch {
    fn from(value: IShellUIHelper7) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IShellUIHelper7> for super::Ole::Automation::IDispatch {
    fn from(value: &IShellUIHelper7) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IShellUIHelper7 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper7_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, left: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, top: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, width: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, height: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fimport: i16, strimpexppath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, form: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strsearch: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strfailureurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pvarout: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fsqm: i16, fphishing: i16, bstrlocale: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstruri: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fset: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfmigrated: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fcomplete: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfshown: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrurl: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, verb: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fvisible: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fvisible: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrelativeurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfsitemode: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstriconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtooltip: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarbuttonid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, fenabled: i16, fvisible: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardescription: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrheader: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstractionuri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstriconuri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarwindowtype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uibuttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, bstriconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtooltip: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarstyleid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uibuttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uistyleid: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fpreservestate: i16, puifirstrun: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfiltername: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrprovisioningxml: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, puiresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pollinguris: ::std::mem::ManuallyDrop<super::Com::VARIANT>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uiupdaterecurrence: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pollinguris: ::std::mem::ManuallyDrop<super::Com::VARIANT>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uiupdaterecurrence: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarsitestate: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrnotificationxml: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnotificationid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnotificationtag: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, expirationtime: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrnotificationid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pollinguri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uiupdaterecurrence: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrflagstring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vfflag: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrflagstring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vfflag: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvaluestring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwvalue: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvaluestring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwvalue: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, flag: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, flag: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, automated: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IShellUIHelper8(::windows::runtime::IUnknown);
impl IShellUIHelper8 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddFavorite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), url.into_param().abi(), ::std::mem::transmute(title)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddChannel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddDesktopComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, r#type: Param1, left: *const super::Com::VARIANT, top: *const super::Com::VARIANT, width: *const super::Com::VARIANT, height: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), url.into_param().abi(), r#type.into_param().abi(), ::std::mem::transmute(left), ::std::mem::transmute(top), ::std::mem::transmute(width), ::std::mem::transmute(height)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn NavigateAndFind<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, strquery: Param1, vartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), url.into_param().abi(), strquery.into_param().abi(), ::std::mem::transmute(vartargetframe)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fimport: i16, strimpexppath: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(fimport), strimpexppath.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(form)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoScan<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strsearch: Param0, strfailureurl: Param1, pvartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), strsearch.into_param().abi(), strfailureurl.into_param().abi(), ::std::mem::transmute(pvartargetframe)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(reserved)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ShowBrowserUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, pvarin: *const super::Com::VARIANT) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), ::std::mem::transmute(pvarin), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddSearchProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    pub unsafe fn RunOnceShown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SkipRunOnce(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeSettings<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fsqm: i16, fphishing: i16, bstrlocale: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(fsqm), ::std::mem::transmute(fphishing), bstrlocale.into_param().abi()).ok()
    }
    pub unsafe fn SqmEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn PhishingEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BrandImageUri(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SkipTabsWelcome(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DiagnoseConnection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CustomizeClearType(&self, fset: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(fset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSearchProviderInstalled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn IsSearchMigrated(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DefaultSearchProvider(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RunOnceRequiredSettingsComplete(&self, fcomplete: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(fcomplete)).ok()
    }
    pub unsafe fn RunOnceHasShown(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchGuideUrl(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsServiceInstalled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, verb: Param1) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), url.into_param().abi(), verb.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn InPrivateFilteringEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddToFavoritesBar<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: Param1, r#type: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), url.into_param().abi(), title.into_param().abi(), ::std::mem::transmute(r#type)).ok()
    }
    pub unsafe fn BuildNewTabPage(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetRecentlyClosedVisible(&self, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(fvisible)).ok()
    }
    pub unsafe fn SetActivitiesVisible(&self, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), ::std::mem::transmute(fvisible)).ok()
    }
    pub unsafe fn ContentDiscoveryReset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsSuggestedSitesEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn EnableSuggestedSites(&self, fenable: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), ::std::mem::transmute(fenable)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NavigateToSuggestedSites<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrelativeurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), bstrrelativeurl.into_param().abi()).ok()
    }
    pub unsafe fn ShowTabsHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ShowInPrivateHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msIsSiteMode(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn msSiteModeShowThumbBar(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddThumbBarButton<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstriconurl: Param0, bstrtooltip: Param1) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::std::mem::transmute_copy(self), bstriconurl.into_param().abi(), bstrtooltip.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeUpdateThumbBarButton<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, buttonid: Param0, fenabled: i16, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::std::mem::transmute_copy(self), buttonid.into_param().abi(), ::std::mem::transmute(fenabled), ::std::mem::transmute(fvisible)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeSetIconOverlay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, iconurl: Param0, pvardescription: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::std::mem::transmute_copy(self), iconurl.into_param().abi(), ::std::mem::transmute(pvardescription)).ok()
    }
    pub unsafe fn msSiteModeClearIconOverlay(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msAddSiteMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msSiteModeCreateJumpList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrheader: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::std::mem::transmute_copy(self), bstrheader.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddJumpListItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, bstractionuri: Param1, bstriconuri: Param2, pvarwindowtype: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).57)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), bstractionuri.into_param().abi(), bstriconuri.into_param().abi(), ::std::mem::transmute(pvarwindowtype)).ok()
    }
    pub unsafe fn msSiteModeClearJumpList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeShowJumpList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).59)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddButtonStyle<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, uibuttonid: Param0, bstriconurl: Param1, bstrtooltip: Param2) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::std::mem::transmute_copy(self), uibuttonid.into_param().abi(), bstriconurl.into_param().abi(), bstrtooltip.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeShowButtonStyle<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, uibuttonid: Param0, uistyleid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).61)(::std::mem::transmute_copy(self), uibuttonid.into_param().abi(), uistyleid.into_param().abi()).ok()
    }
    pub unsafe fn msSiteModeActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).62)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msIsSiteModeFirstRun(&self, fpreservestate: i16) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::std::mem::transmute_copy(self), ::std::mem::transmute(fpreservestate), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msAddTrackingProtectionList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, bstrfiltername: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).64)(::std::mem::transmute_copy(self), url.into_param().abi(), bstrfiltername.into_param().abi()).ok()
    }
    pub unsafe fn msTrackingProtectionEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).65)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn msActiveXFilteringEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).66)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msProvisionNetworks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprovisioningxml: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).67)(::std::mem::transmute_copy(self), bstrprovisioningxml.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn msReportSafeUrl(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).68)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeRefreshBadge(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).69)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeClearBadge(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).70)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msDiagnoseConnectionUILess(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).71)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msLaunchNetworkClientHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).72)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msChangeDefaultBrowser(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).73)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    pub unsafe fn msStopPeriodicTileUpdate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).74)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msStartPeriodicTileUpdate<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param2: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, pollinguris: Param0, starttime: Param1, uiupdaterecurrence: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).75)(::std::mem::transmute_copy(self), pollinguris.into_param().abi(), starttime.into_param().abi(), uiupdaterecurrence.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msStartPeriodicTileUpdateBatch<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param2: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, pollinguris: Param0, starttime: Param1, uiupdaterecurrence: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).76)(::std::mem::transmute_copy(self), pollinguris.into_param().abi(), starttime.into_param().abi(), uiupdaterecurrence.into_param().abi()).ok()
    }
    pub unsafe fn msClearTile(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).77)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msEnableTileNotificationQueue(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).78)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msPinnedSiteState(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).79)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn msEnableTileNotificationQueueForSquare150x150(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).80)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    pub unsafe fn msEnableTileNotificationQueueForWide310x150(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).81)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    pub unsafe fn msEnableTileNotificationQueueForSquare310x310(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).82)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msScheduledTileNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param4: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(
        &self,
        bstrnotificationxml: Param0,
        bstrnotificationid: Param1,
        bstrnotificationtag: Param2,
        starttime: Param3,
        expirationtime: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).83)(::std::mem::transmute_copy(self), bstrnotificationxml.into_param().abi(), bstrnotificationid.into_param().abi(), bstrnotificationtag.into_param().abi(), starttime.into_param().abi(), expirationtime.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msRemoveScheduledTileNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrnotificationid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).84)(::std::mem::transmute_copy(self), bstrnotificationid.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msStartPeriodicBadgeUpdate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param2: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, pollinguri: Param0, starttime: Param1, uiupdaterecurrence: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).85)(::std::mem::transmute_copy(self), pollinguri.into_param().abi(), starttime.into_param().abi(), uiupdaterecurrence.into_param().abi()).ok()
    }
    pub unsafe fn msStopPeriodicBadgeUpdate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).86)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msLaunchInternetOptions(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).87)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExperimentalFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrflagstring: Param0, vfflag: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).88)(::std::mem::transmute_copy(self), bstrflagstring.into_param().abi(), ::std::mem::transmute(vfflag)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExperimentalFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrflagstring: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).89)(::std::mem::transmute_copy(self), bstrflagstring.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExperimentalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrvaluestring: Param0, dwvalue: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).90)(::std::mem::transmute_copy(self), bstrvaluestring.into_param().abi(), ::std::mem::transmute(dwvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExperimentalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrvaluestring: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).91)(::std::mem::transmute_copy(self), bstrvaluestring.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn ResetAllExperimentalFlagsAndValues(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).92)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNeedIEAutoLaunchFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrurl: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).93)(::std::mem::transmute_copy(self), bstrurl.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNeedIEAutoLaunchFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrurl: Param0, flag: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).94)(::std::mem::transmute_copy(self), bstrurl.into_param().abi(), ::std::mem::transmute(flag)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasNeedIEAutoLaunchFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrurl: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).95)(::std::mem::transmute_copy(self), bstrurl.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LaunchIE<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrurl: Param0, automated: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).96)(::std::mem::transmute_copy(self), bstrurl.into_param().abi(), ::std::mem::transmute(automated)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCVListData(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).97)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCVListLocalData(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).98)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEMIEListData(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).99)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEMIEListLocalData(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).100)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn OpenFavoritesPane(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).101)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OpenFavoritesSettings(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).102)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LaunchInHVSI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).103)(::std::mem::transmute_copy(self), bstrurl.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IShellUIHelper8 {
    type Vtable = IShellUIHelper8_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1725873394, 1456, 20231, [180, 155, 185, 98, 65, 166, 93, 178]);
}
impl ::std::convert::From<IShellUIHelper8> for ::windows::runtime::IUnknown {
    fn from(value: IShellUIHelper8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper8> for ::windows::runtime::IUnknown {
    fn from(value: &IShellUIHelper8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper8> for IShellUIHelper7 {
    fn from(value: IShellUIHelper8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper8> for IShellUIHelper7 {
    fn from(value: &IShellUIHelper8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper7> for IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper7> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper7>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper7> for &IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper7> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper7>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper8> for IShellUIHelper6 {
    fn from(value: IShellUIHelper8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper8> for IShellUIHelper6 {
    fn from(value: &IShellUIHelper8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper6> for IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper6> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper6>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper6> for &IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper6> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper6>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper8> for IShellUIHelper5 {
    fn from(value: IShellUIHelper8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper8> for IShellUIHelper5 {
    fn from(value: &IShellUIHelper8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper5> for IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper5>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper5> for &IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper5>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper8> for IShellUIHelper4 {
    fn from(value: IShellUIHelper8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper8> for IShellUIHelper4 {
    fn from(value: &IShellUIHelper8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper4> for IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper4>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper4> for &IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper4>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper8> for IShellUIHelper3 {
    fn from(value: IShellUIHelper8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper8> for IShellUIHelper3 {
    fn from(value: &IShellUIHelper8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper3> for IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper3> for &IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper3>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper8> for IShellUIHelper2 {
    fn from(value: IShellUIHelper8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper8> for IShellUIHelper2 {
    fn from(value: &IShellUIHelper8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper2> for IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper2> for &IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper8> for IShellUIHelper {
    fn from(value: IShellUIHelper8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper8> for IShellUIHelper {
    fn from(value: &IShellUIHelper8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for &IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IShellUIHelper8> for super::Ole::Automation::IDispatch {
    fn from(value: IShellUIHelper8) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IShellUIHelper8> for super::Ole::Automation::IDispatch {
    fn from(value: &IShellUIHelper8) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IShellUIHelper8 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper8_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, left: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, top: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, width: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, height: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fimport: i16, strimpexppath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, form: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strsearch: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strfailureurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pvarout: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fsqm: i16, fphishing: i16, bstrlocale: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstruri: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fset: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfmigrated: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fcomplete: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfshown: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrurl: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, verb: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fvisible: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fvisible: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrelativeurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfsitemode: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstriconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtooltip: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarbuttonid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, fenabled: i16, fvisible: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardescription: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrheader: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstractionuri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstriconuri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarwindowtype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uibuttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, bstriconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtooltip: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarstyleid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uibuttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uistyleid: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fpreservestate: i16, puifirstrun: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfiltername: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrprovisioningxml: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, puiresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pollinguris: ::std::mem::ManuallyDrop<super::Com::VARIANT>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uiupdaterecurrence: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pollinguris: ::std::mem::ManuallyDrop<super::Com::VARIANT>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uiupdaterecurrence: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarsitestate: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrnotificationxml: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnotificationid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnotificationtag: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, expirationtime: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrnotificationid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pollinguri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uiupdaterecurrence: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrflagstring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vfflag: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrflagstring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vfflag: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvaluestring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwvalue: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvaluestring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwvalue: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, flag: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, flag: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, automated: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrresult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrresult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrresult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrresult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IShellUIHelper9(::windows::runtime::IUnknown);
impl IShellUIHelper9 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn ResetFirstBootMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResetSafeMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn RefreshOfflineDesktop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddFavorite<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), url.into_param().abi(), ::std::mem::transmute(title)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddChannel<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddDesktopComponent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, r#type: Param1, left: *const super::Com::VARIANT, top: *const super::Com::VARIANT, width: *const super::Com::VARIANT, height: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), url.into_param().abi(), r#type.into_param().abi(), ::std::mem::transmute(left), ::std::mem::transmute(top), ::std::mem::transmute(width), ::std::mem::transmute(height)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn NavigateAndFind<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, strquery: Param1, vartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), url.into_param().abi(), strquery.into_param().abi(), ::std::mem::transmute(vartargetframe)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportExportFavorites<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fimport: i16, strimpexppath: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(fimport), strimpexppath.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteSaveForm(&self, form: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(form)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoScan<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, strsearch: Param0, strfailureurl: Param1, pvartargetframe: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), strsearch.into_param().abi(), strfailureurl.into_param().abi(), ::std::mem::transmute(pvartargetframe)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AutoCompleteAttach(&self, reserved: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(reserved)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ShowBrowserUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, pvarin: *const super::Com::VARIANT) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), ::std::mem::transmute(pvarin), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddSearchProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    pub unsafe fn RunOnceShown(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SkipRunOnce(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomizeSettings<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, fsqm: i16, fphishing: i16, bstrlocale: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(fsqm), ::std::mem::transmute(fphishing), bstrlocale.into_param().abi()).ok()
    }
    pub unsafe fn SqmEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn PhishingEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BrandImageUri(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SkipTabsWelcome(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DiagnoseConnection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn CustomizeClearType(&self, fset: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(fset)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSearchProviderInstalled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), url.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn IsSearchMigrated(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DefaultSearchProvider(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn RunOnceRequiredSettingsComplete(&self, fcomplete: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(fcomplete)).ok()
    }
    pub unsafe fn RunOnceHasShown(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SearchGuideUrl(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), url.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsServiceInstalled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, verb: Param1) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), url.into_param().abi(), verb.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn InPrivateFilteringEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn AddToFavoritesBar<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, title: Param1, r#type: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), url.into_param().abi(), title.into_param().abi(), ::std::mem::transmute(r#type)).ok()
    }
    pub unsafe fn BuildNewTabPage(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetRecentlyClosedVisible(&self, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(fvisible)).ok()
    }
    pub unsafe fn SetActivitiesVisible(&self, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), ::std::mem::transmute(fvisible)).ok()
    }
    pub unsafe fn ContentDiscoveryReset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn IsSuggestedSitesEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn EnableSuggestedSites(&self, fenable: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), ::std::mem::transmute(fenable)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NavigateToSuggestedSites<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrrelativeurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), bstrrelativeurl.into_param().abi()).ok()
    }
    pub unsafe fn ShowTabsHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ShowInPrivateHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msIsSiteMode(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn msSiteModeShowThumbBar(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddThumbBarButton<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstriconurl: Param0, bstrtooltip: Param1) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::std::mem::transmute_copy(self), bstriconurl.into_param().abi(), bstrtooltip.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeUpdateThumbBarButton<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, buttonid: Param0, fenabled: i16, fvisible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::std::mem::transmute_copy(self), buttonid.into_param().abi(), ::std::mem::transmute(fenabled), ::std::mem::transmute(fvisible)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeSetIconOverlay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, iconurl: Param0, pvardescription: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::std::mem::transmute_copy(self), iconurl.into_param().abi(), ::std::mem::transmute(pvardescription)).ok()
    }
    pub unsafe fn msSiteModeClearIconOverlay(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msAddSiteMode(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msSiteModeCreateJumpList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrheader: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::std::mem::transmute_copy(self), bstrheader.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddJumpListItem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, bstractionuri: Param1, bstriconuri: Param2, pvarwindowtype: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).57)(::std::mem::transmute_copy(self), bstrname.into_param().abi(), bstractionuri.into_param().abi(), bstriconuri.into_param().abi(), ::std::mem::transmute(pvarwindowtype)).ok()
    }
    pub unsafe fn msSiteModeClearJumpList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeShowJumpList(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).59)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeAddButtonStyle<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, uibuttonid: Param0, bstriconurl: Param1, bstrtooltip: Param2) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::std::mem::transmute_copy(self), uibuttonid.into_param().abi(), bstriconurl.into_param().abi(), bstrtooltip.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msSiteModeShowButtonStyle<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, uibuttonid: Param0, uistyleid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).61)(::std::mem::transmute_copy(self), uibuttonid.into_param().abi(), uistyleid.into_param().abi()).ok()
    }
    pub unsafe fn msSiteModeActivate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).62)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msIsSiteModeFirstRun(&self, fpreservestate: i16) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::std::mem::transmute_copy(self), ::std::mem::transmute(fpreservestate), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msAddTrackingProtectionList<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, bstrfiltername: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).64)(::std::mem::transmute_copy(self), url.into_param().abi(), bstrfiltername.into_param().abi()).ok()
    }
    pub unsafe fn msTrackingProtectionEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).65)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn msActiveXFilteringEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).66)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msProvisionNetworks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrprovisioningxml: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).67)(::std::mem::transmute_copy(self), bstrprovisioningxml.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn msReportSafeUrl(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).68)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeRefreshBadge(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).69)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msSiteModeClearBadge(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).70)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msDiagnoseConnectionUILess(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).71)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msLaunchNetworkClientHelp(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).72)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msChangeDefaultBrowser(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).73)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    pub unsafe fn msStopPeriodicTileUpdate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).74)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msStartPeriodicTileUpdate<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param2: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, pollinguris: Param0, starttime: Param1, uiupdaterecurrence: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).75)(::std::mem::transmute_copy(self), pollinguris.into_param().abi(), starttime.into_param().abi(), uiupdaterecurrence.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msStartPeriodicTileUpdateBatch<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param2: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, pollinguris: Param0, starttime: Param1, uiupdaterecurrence: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).76)(::std::mem::transmute_copy(self), pollinguris.into_param().abi(), starttime.into_param().abi(), uiupdaterecurrence.into_param().abi()).ok()
    }
    pub unsafe fn msClearTile(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).77)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msEnableTileNotificationQueue(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).78)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msPinnedSiteState(&self) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).79)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    pub unsafe fn msEnableTileNotificationQueueForSquare150x150(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).80)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    pub unsafe fn msEnableTileNotificationQueueForWide310x150(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).81)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    pub unsafe fn msEnableTileNotificationQueueForSquare310x310(&self, fchange: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).82)(::std::mem::transmute_copy(self), ::std::mem::transmute(fchange)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msScheduledTileNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param3: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param4: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(
        &self,
        bstrnotificationxml: Param0,
        bstrnotificationid: Param1,
        bstrnotificationtag: Param2,
        starttime: Param3,
        expirationtime: Param4,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).83)(::std::mem::transmute_copy(self), bstrnotificationxml.into_param().abi(), bstrnotificationid.into_param().abi(), bstrnotificationtag.into_param().abi(), starttime.into_param().abi(), expirationtime.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn msRemoveScheduledTileNotification<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrnotificationid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).84)(::std::mem::transmute_copy(self), bstrnotificationid.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn msStartPeriodicBadgeUpdate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>, Param2: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, pollinguri: Param0, starttime: Param1, uiupdaterecurrence: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).85)(::std::mem::transmute_copy(self), pollinguri.into_param().abi(), starttime.into_param().abi(), uiupdaterecurrence.into_param().abi()).ok()
    }
    pub unsafe fn msStopPeriodicBadgeUpdate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).86)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn msLaunchInternetOptions(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).87)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExperimentalFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrflagstring: Param0, vfflag: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).88)(::std::mem::transmute_copy(self), bstrflagstring.into_param().abi(), ::std::mem::transmute(vfflag)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExperimentalFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrflagstring: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).89)(::std::mem::transmute_copy(self), bstrflagstring.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExperimentalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrvaluestring: Param0, dwvalue: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).90)(::std::mem::transmute_copy(self), bstrvaluestring.into_param().abi(), ::std::mem::transmute(dwvalue)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetExperimentalValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrvaluestring: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).91)(::std::mem::transmute_copy(self), bstrvaluestring.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    pub unsafe fn ResetAllExperimentalFlagsAndValues(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).92)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNeedIEAutoLaunchFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrurl: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).93)(::std::mem::transmute_copy(self), bstrurl.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetNeedIEAutoLaunchFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrurl: Param0, flag: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).94)(::std::mem::transmute_copy(self), bstrurl.into_param().abi(), ::std::mem::transmute(flag)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasNeedIEAutoLaunchFlag<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrurl: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).95)(::std::mem::transmute_copy(self), bstrurl.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LaunchIE<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrurl: Param0, automated: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).96)(::std::mem::transmute_copy(self), bstrurl.into_param().abi(), ::std::mem::transmute(automated)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCVListData(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).97)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCVListLocalData(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).98)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEMIEListData(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).99)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEMIEListLocalData(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).100)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn OpenFavoritesPane(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).101)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn OpenFavoritesSettings(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).102)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LaunchInHVSI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrurl: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).103)(::std::mem::transmute_copy(self), bstrurl.into_param().abi()).ok()
    }
    pub unsafe fn GetOSSku(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).104)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IShellUIHelper9 {
    type Vtable = IShellUIHelper9_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1826583472, 32559, 17695, [188, 15, 99, 224, 243, 40, 78, 84]);
}
impl ::std::convert::From<IShellUIHelper9> for ::windows::runtime::IUnknown {
    fn from(value: IShellUIHelper9) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper9> for ::windows::runtime::IUnknown {
    fn from(value: &IShellUIHelper9) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper9> for IShellUIHelper8 {
    fn from(value: IShellUIHelper9) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper9> for IShellUIHelper8 {
    fn from(value: &IShellUIHelper9) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper8> for IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper8> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper8>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper8> for &IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper8> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper8>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper9> for IShellUIHelper7 {
    fn from(value: IShellUIHelper9) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper9> for IShellUIHelper7 {
    fn from(value: &IShellUIHelper9) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper7> for IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper7> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper7>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper7> for &IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper7> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper7>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper9> for IShellUIHelper6 {
    fn from(value: IShellUIHelper9) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper9> for IShellUIHelper6 {
    fn from(value: &IShellUIHelper9) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper6> for IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper6> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper6>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper6> for &IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper6> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper6>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper9> for IShellUIHelper5 {
    fn from(value: IShellUIHelper9) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper9> for IShellUIHelper5 {
    fn from(value: &IShellUIHelper9) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper5> for IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper5>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper5> for &IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper5> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper5>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper9> for IShellUIHelper4 {
    fn from(value: IShellUIHelper9) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper9> for IShellUIHelper4 {
    fn from(value: &IShellUIHelper9) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper4> for IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper4>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper4> for &IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper4> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper4>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper9> for IShellUIHelper3 {
    fn from(value: IShellUIHelper9) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper9> for IShellUIHelper3 {
    fn from(value: &IShellUIHelper9) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper3> for IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper3>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper3> for &IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper3> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper3>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper9> for IShellUIHelper2 {
    fn from(value: IShellUIHelper9) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper9> for IShellUIHelper2 {
    fn from(value: &IShellUIHelper9) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper2> for IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper2> for &IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper2>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IShellUIHelper9> for IShellUIHelper {
    fn from(value: IShellUIHelper9) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IShellUIHelper9> for IShellUIHelper {
    fn from(value: &IShellUIHelper9) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IShellUIHelper> for &IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IShellUIHelper> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IShellUIHelper>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IShellUIHelper9> for super::Ole::Automation::IDispatch {
    fn from(value: IShellUIHelper9) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IShellUIHelper9> for super::Ole::Automation::IDispatch {
    fn from(value: &IShellUIHelper9) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IShellUIHelper9 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShellUIHelper9_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, left: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, top: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, width: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, height: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strquery: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fimport: i16, strimpexppath: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, form: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strsearch: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strfailureurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvartargetframe: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reserved: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarin: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pvarout: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fsqm: i16, fphishing: i16, bstrlocale: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstruri: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fset: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfmigrated: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fcomplete: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfshown: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrurl: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, verb: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, title: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, r#type: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fvisible: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fvisible: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrrelativeurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfsitemode: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstriconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtooltip: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarbuttonid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, fenabled: i16, fvisible: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvardescription: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrheader: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstractionuri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstriconuri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarwindowtype: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uibuttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, bstriconurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtooltip: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarstyleid: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uibuttonid: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uistyleid: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fpreservestate: i16, puifirstrun: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrfiltername: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrprovisioningxml: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, puiresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pollinguris: ::std::mem::ManuallyDrop<super::Com::VARIANT>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uiupdaterecurrence: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pollinguris: ::std::mem::ManuallyDrop<super::Com::VARIANT>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uiupdaterecurrence: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarsitestate: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fchange: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrnotificationxml: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnotificationid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnotificationtag: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, expirationtime: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrnotificationid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pollinguri: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, starttime: ::std::mem::ManuallyDrop<super::Com::VARIANT>, uiupdaterecurrence: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrflagstring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vfflag: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrflagstring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vfflag: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvaluestring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwvalue: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrvaluestring: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pdwvalue: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, flag: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, flag: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, exists: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, automated: i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrresult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrresult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrresult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrresult: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrurl: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwresult: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWebBrowser(::windows::runtime::IUnknown);
impl IWebBrowser {
    pub unsafe fn GoBack(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GoForward(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GoHome(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GoSearch(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Navigate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, flags: *const super::Com::VARIANT, targetframename: *const super::Com::VARIANT, postdata: *const super::Com::VARIANT, headers: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), url.into_param().abi(), ::std::mem::transmute(flags), ::std::mem::transmute(targetframename), ::std::mem::transmute(postdata), ::std::mem::transmute(headers)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Refresh2(&self, level: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(level)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Application(&self) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Parent(&self) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Container(&self) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Document(&self) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    pub unsafe fn TopLevelContainer(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Left(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLeft(&self, left: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(left)).ok()
    }
    pub unsafe fn Top(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTop(&self, top: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(top)).ok()
    }
    pub unsafe fn Width(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetWidth(&self, width: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(width)).ok()
    }
    pub unsafe fn Height(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHeight(&self, height: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(height)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocationName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocationURL(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Busy(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWebBrowser {
    type Vtable = IWebBrowser_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3937544897, 12481, 4559, [167, 235, 0, 0, 192, 91, 174, 11]);
}
impl ::std::convert::From<IWebBrowser> for ::windows::runtime::IUnknown {
    fn from(value: IWebBrowser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWebBrowser> for ::windows::runtime::IUnknown {
    fn from(value: &IWebBrowser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWebBrowser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWebBrowser {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWebBrowser> for super::Ole::Automation::IDispatch {
    fn from(value: IWebBrowser) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWebBrowser> for super::Ole::Automation::IDispatch {
    fn from(value: &IWebBrowser) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWebBrowser {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWebBrowser {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebBrowser_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, targetframename: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, postdata: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, headers: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, level: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdisp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdisp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdisp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdisp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pl: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, left: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pl: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, top: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pl: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, width: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pl: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, height: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, locationname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, locationurl: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWebBrowser2(::windows::runtime::IUnknown);
impl IWebBrowser2 {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn GoBack(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GoForward(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GoHome(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GoSearch(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Navigate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, flags: *const super::Com::VARIANT, targetframename: *const super::Com::VARIANT, postdata: *const super::Com::VARIANT, headers: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), url.into_param().abi(), ::std::mem::transmute(flags), ::std::mem::transmute(targetframename), ::std::mem::transmute(postdata), ::std::mem::transmute(headers)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Refresh2(&self, level: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(level)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Application(&self) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Parent(&self) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Container(&self) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Document(&self) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    pub unsafe fn TopLevelContainer(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Left(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLeft(&self, left: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(left)).ok()
    }
    pub unsafe fn Top(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTop(&self, top: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(top)).ok()
    }
    pub unsafe fn Width(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetWidth(&self, width: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(width)).ok()
    }
    pub unsafe fn Height(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHeight(&self, height: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(height)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocationName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocationURL(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Busy(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Quit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ClientToWindow(&self, pcx: *mut i32, pcy: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcx), ::std::mem::transmute(pcy)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn PutProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, property: Param0, vtvalue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), property.into_param().abi(), vtvalue.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn GetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, property: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), property.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HWND(&self) -> ::windows::runtime::Result<super::super::Foundation::SHANDLE_PTR> {
        let mut result__: <super::super::Foundation::SHANDLE_PTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::SHANDLE_PTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FullName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Visible(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetVisible(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    pub unsafe fn StatusBar(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetStatusBar(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StatusText(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatusText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, statustext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), statustext.into_param().abi()).ok()
    }
    pub unsafe fn ToolBar(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetToolBar(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    pub unsafe fn MenuBar(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetMenuBar(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).49)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    pub unsafe fn FullScreen(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetFullScreen(&self, bfullscreen: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).51)(::std::mem::transmute_copy(self), ::std::mem::transmute(bfullscreen)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Navigate2(&self, url: *const super::Com::VARIANT, flags: *const super::Com::VARIANT, targetframename: *const super::Com::VARIANT, postdata: *const super::Com::VARIANT, headers: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::std::mem::transmute_copy(self), ::std::mem::transmute(url), ::std::mem::transmute(flags), ::std::mem::transmute(targetframename), ::std::mem::transmute(postdata), ::std::mem::transmute(headers)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn QueryStatusWB(&self, cmdid: super::Ole::OLECMDID) -> ::windows::runtime::Result<super::Ole::OLECMDF> {
        let mut result__: <super::Ole::OLECMDF as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).53)(::std::mem::transmute_copy(self), ::std::mem::transmute(cmdid), &mut result__).from_abi::<super::Ole::OLECMDF>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ExecWB(&self, cmdid: super::Ole::OLECMDID, cmdexecopt: super::Ole::OLECMDEXECOPT, pvain: *const super::Com::VARIANT, pvaout: *mut super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::std::mem::transmute_copy(self), ::std::mem::transmute(cmdid), ::std::mem::transmute(cmdexecopt), ::std::mem::transmute(pvain), ::std::mem::transmute(pvaout)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn ShowBrowserBar(&self, pvaclsid: *const super::Com::VARIANT, pvarshow: *const super::Com::VARIANT, pvarsize: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self), ::std::mem::transmute(pvaclsid), ::std::mem::transmute(pvarshow), ::std::mem::transmute(pvarsize)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn ReadyState(&self) -> ::windows::runtime::Result<super::Ole::READYSTATE> {
        let mut result__: <super::Ole::READYSTATE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).56)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::READYSTATE>(result__)
    }
    pub unsafe fn Offline(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).57)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetOffline(&self, boffline: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(::std::mem::transmute_copy(self), ::std::mem::transmute(boffline)).ok()
    }
    pub unsafe fn Silent(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).59)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetSilent(&self, bsilent: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).60)(::std::mem::transmute_copy(self), ::std::mem::transmute(bsilent)).ok()
    }
    pub unsafe fn RegisterAsBrowser(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).61)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetRegisterAsBrowser(&self, bregister: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).62)(::std::mem::transmute_copy(self), ::std::mem::transmute(bregister)).ok()
    }
    pub unsafe fn RegisterAsDropTarget(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetRegisterAsDropTarget(&self, bregister: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).64)(::std::mem::transmute_copy(self), ::std::mem::transmute(bregister)).ok()
    }
    pub unsafe fn TheaterMode(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).65)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetTheaterMode(&self, bregister: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).66)(::std::mem::transmute_copy(self), ::std::mem::transmute(bregister)).ok()
    }
    pub unsafe fn AddressBar(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).67)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetAddressBar(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).68)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    pub unsafe fn Resizable(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).69)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetResizable(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).70)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWebBrowser2 {
    type Vtable = IWebBrowser2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3540784737, 52655, 4560, [138, 62, 0, 192, 79, 201, 226, 110]);
}
impl ::std::convert::From<IWebBrowser2> for ::windows::runtime::IUnknown {
    fn from(value: IWebBrowser2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWebBrowser2> for ::windows::runtime::IUnknown {
    fn from(value: &IWebBrowser2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWebBrowser2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWebBrowser2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWebBrowser2> for IWebBrowserApp {
    fn from(value: IWebBrowser2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWebBrowser2> for IWebBrowserApp {
    fn from(value: &IWebBrowser2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebBrowserApp> for IWebBrowser2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebBrowserApp> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWebBrowserApp>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebBrowserApp> for &IWebBrowser2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebBrowserApp> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWebBrowserApp>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWebBrowser2> for IWebBrowser {
    fn from(value: IWebBrowser2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWebBrowser2> for IWebBrowser {
    fn from(value: &IWebBrowser2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebBrowser> for IWebBrowser2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebBrowser> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWebBrowser>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebBrowser> for &IWebBrowser2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebBrowser> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWebBrowser>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWebBrowser2> for super::Ole::Automation::IDispatch {
    fn from(value: IWebBrowser2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWebBrowser2> for super::Ole::Automation::IDispatch {
    fn from(value: &IWebBrowser2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWebBrowser2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWebBrowser2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebBrowser2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, targetframename: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, postdata: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, headers: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, level: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdisp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdisp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdisp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdisp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pl: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, left: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pl: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, top: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pl: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, width: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pl: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, height: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, locationname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, locationurl: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcx: *mut i32, pcy: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, property: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vtvalue: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, property: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvtvalue: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::SHANDLE_PTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fullname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statustext: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statustext: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbfullscreen: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bfullscreen: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, flags: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, targetframename: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, postdata: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, headers: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cmdid: super::Ole::OLECMDID, pcmdf: *mut super::Ole::OLECMDF) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cmdid: super::Ole::OLECMDID, cmdexecopt: super::Ole::OLECMDEXECOPT, pvain: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pvaout: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvaclsid: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pvarshow: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, pvarsize: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plreadystate: *mut super::Ole::READYSTATE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pboffline: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, boffline: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsilent: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bsilent: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbregister: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bregister: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbregister: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bregister: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbregister: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bregister: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWebBrowserApp(::windows::runtime::IUnknown);
impl IWebBrowserApp {
    pub unsafe fn GetTypeInfoCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::runtime::Result<super::Ole::Automation::ITypeInfo> {
        let mut result__: <super::Ole::Automation::ITypeInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(itinfo), ::std::mem::transmute(lcid), &mut result__).from_abi::<super::Ole::Automation::ITypeInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIDsOfNames(&self, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(rgsznames), ::std::mem::transmute(cnames), ::std::mem::transmute(lcid), ::std::mem::transmute(rgdispid)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Invoke(&self, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut super::Com::VARIANT, pexcepinfo: *mut super::Ole::Automation::EXCEPINFO, puargerr: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dispidmember), ::std::mem::transmute(riid), ::std::mem::transmute(lcid), ::std::mem::transmute(wflags), ::std::mem::transmute(pdispparams), ::std::mem::transmute(pvarresult), ::std::mem::transmute(pexcepinfo), ::std::mem::transmute(puargerr)).ok()
    }
    pub unsafe fn GoBack(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GoForward(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GoHome(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GoSearch(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Navigate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, url: Param0, flags: *const super::Com::VARIANT, targetframename: *const super::Com::VARIANT, postdata: *const super::Com::VARIANT, headers: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), url.into_param().abi(), ::std::mem::transmute(flags), ::std::mem::transmute(targetframename), ::std::mem::transmute(postdata), ::std::mem::transmute(headers)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn Refresh2(&self, level: *const super::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(level)).ok()
    }
    pub unsafe fn Stop(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Application(&self) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Parent(&self) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Container(&self) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub unsafe fn Document(&self) -> ::windows::runtime::Result<super::Ole::Automation::IDispatch> {
        let mut result__: <super::Ole::Automation::IDispatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::Ole::Automation::IDispatch>(result__)
    }
    pub unsafe fn TopLevelContainer(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Type(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Left(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetLeft(&self, left: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(left)).ok()
    }
    pub unsafe fn Top(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetTop(&self, top: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(top)).ok()
    }
    pub unsafe fn Width(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetWidth(&self, width: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(width)).ok()
    }
    pub unsafe fn Height(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetHeight(&self, height: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(height)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocationName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocationURL(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Busy(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn Quit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ClientToWindow(&self, pcx: *mut i32, pcy: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcx), ::std::mem::transmute(pcy)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn PutProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::Com::VARIANT>>(&self, property: Param0, vtvalue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), property.into_param().abi(), vtvalue.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub unsafe fn GetProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, property: Param0) -> ::windows::runtime::Result<super::Com::VARIANT> {
        let mut result__: <super::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), property.into_param().abi(), &mut result__).from_abi::<super::Com::VARIANT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HWND(&self) -> ::windows::runtime::Result<super::super::Foundation::SHANDLE_PTR> {
        let mut result__: <super::super::Foundation::SHANDLE_PTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::SHANDLE_PTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FullName(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Path(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Visible(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetVisible(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    pub unsafe fn StatusBar(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetStatusBar(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StatusText(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStatusText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, statustext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), statustext.into_param().abi()).ok()
    }
    pub unsafe fn ToolBar(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    pub unsafe fn SetToolBar(&self, value: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    pub unsafe fn MenuBar(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetMenuBar(&self, value: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).49)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    pub unsafe fn FullScreen(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).50)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    pub unsafe fn SetFullScreen(&self, bfullscreen: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).51)(::std::mem::transmute_copy(self), ::std::mem::transmute(bfullscreen)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWebBrowserApp {
    type Vtable = IWebBrowserApp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(188165, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl ::std::convert::From<IWebBrowserApp> for ::windows::runtime::IUnknown {
    fn from(value: IWebBrowserApp) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWebBrowserApp> for ::windows::runtime::IUnknown {
    fn from(value: &IWebBrowserApp) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWebBrowserApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWebBrowserApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IWebBrowserApp> for IWebBrowser {
    fn from(value: IWebBrowserApp) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWebBrowserApp> for IWebBrowser {
    fn from(value: &IWebBrowserApp) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebBrowser> for IWebBrowserApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebBrowser> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWebBrowser>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebBrowser> for &IWebBrowserApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebBrowser> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWebBrowser>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWebBrowserApp> for super::Ole::Automation::IDispatch {
    fn from(value: IWebBrowserApp) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWebBrowserApp> for super::Ole::Automation::IDispatch {
    fn from(value: &IWebBrowserApp) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for IWebBrowserApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Ole::Automation::IDispatch> for &IWebBrowserApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebBrowserApp_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, url: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, targetframename: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, postdata: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>, headers: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, level: *const ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdisp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdisp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdisp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppdisp: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pl: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, left: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pl: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, top: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pl: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, width: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pl: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, height: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, locationname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, locationurl: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcx: *mut i32, pcy: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, property: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, vtvalue: ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, property: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvtvalue: *mut ::std::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::SHANDLE_PTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fullname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, path: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statustext: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statustext: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbfullscreen: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bfullscreen: i16) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IWindowsLockModeHelper(::windows::runtime::IUnknown);
impl IWindowsLockModeHelper {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSMode(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWindowsLockModeHelper {
    type Vtable = IWindowsLockModeHelper_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4081242526, 52258, 17992, [187, 93, 3, 204, 247, 91, 71, 197]);
}
impl ::std::convert::From<IWindowsLockModeHelper> for ::windows::runtime::IUnknown {
    fn from(value: IWindowsLockModeHelper) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWindowsLockModeHelper> for ::windows::runtime::IUnknown {
    fn from(value: &IWindowsLockModeHelper) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWindowsLockModeHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IWindowsLockModeHelper {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsLockModeHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, issmode: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn InitializeContext2(buffer: *mut ::std::ffi::c_void, contextflags: u32, context: *mut *mut super::Diagnostics::Debug::CONTEXT, contextlength: *mut u32, xstatecompactionmask: u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeContext2(buffer: *mut ::std::ffi::c_void, contextflags: u32, context: *mut *mut super::Diagnostics::Debug::CONTEXT, contextlength: *mut u32, xstatecompactionmask: u64) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(InitializeContext2(::std::mem::transmute(buffer), ::std::mem::transmute(contextflags), ::std::mem::transmute(context), ::std::mem::transmute(contextlength), ::std::mem::transmute(xstatecompactionmask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const InternetExplorer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(188161, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
pub const InternetExplorerMedium: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3588752413, 37391, 17897, [184, 251, 177, 222, 184, 44, 110, 94]);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsApiSetImplemented<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(contract: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsApiSetImplemented(contract: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsApiSetImplemented(contract.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadHugeReadPtr(lp: *const ::std::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsBadHugeReadPtr(lp: *const ::std::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsBadHugeReadPtr(::std::mem::transmute(lp), ::std::mem::transmute(ucb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsBadHugeWritePtr(lp: *const ::std::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsBadHugeWritePtr(lp: *const ::std::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsBadHugeWritePtr(::std::mem::transmute(lp), ::std::mem::transmute(ucb)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsNTAdmin(dwreserved: u32, lpdwreserved: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsNTAdmin(dwreserved: u32, lpdwreserved: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsNTAdmin(::std::mem::transmute(dwreserved), ::std::mem::transmute(lpdwreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsNativeVhdBoot(nativevhdboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsNativeVhdBoot(nativevhdboot: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsNativeVhdBoot(::std::mem::transmute(nativevhdboot)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsTokenUntrusted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(tokenhandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsTokenUntrusted(tokenhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsTokenUntrusted(tokenhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct JAVA_TRUST {
    pub cbSize: u32,
    pub flag: u32,
    pub fAllActiveXPermissions: super::super::Foundation::BOOL,
    pub fAllPermissions: super::super::Foundation::BOOL,
    pub dwEncodingType: u32,
    pub pbJavaPermissions: *mut u8,
    pub cbJavaPermissions: u32,
    pub pbSigner: *mut u8,
    pub cbSigner: u32,
    pub pwszZone: super::super::Foundation::PWSTR,
    pub guidZone: ::windows::runtime::GUID,
    pub hVerify: ::windows::runtime::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl JAVA_TRUST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for JAVA_TRUST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for JAVA_TRUST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JAVA_TRUST")
            .field("cbSize", &self.cbSize)
            .field("flag", &self.flag)
            .field("fAllActiveXPermissions", &self.fAllActiveXPermissions)
            .field("fAllPermissions", &self.fAllPermissions)
            .field("dwEncodingType", &self.dwEncodingType)
            .field("pbJavaPermissions", &self.pbJavaPermissions)
            .field("cbJavaPermissions", &self.cbJavaPermissions)
            .field("pbSigner", &self.pbSigner)
            .field("cbSigner", &self.cbSigner)
            .field("pwszZone", &self.pwszZone)
            .field("guidZone", &self.guidZone)
            .field("hVerify", &self.hVerify)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for JAVA_TRUST {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.flag == other.flag && self.fAllActiveXPermissions == other.fAllActiveXPermissions && self.fAllPermissions == other.fAllPermissions && self.dwEncodingType == other.dwEncodingType && self.pbJavaPermissions == other.pbJavaPermissions && self.cbJavaPermissions == other.cbJavaPermissions && self.pbSigner == other.pbSigner && self.cbSigner == other.cbSigner && self.pwszZone == other.pwszZone && self.guidZone == other.guidZone && self.hVerify == other.hVerify
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for JAVA_TRUST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for JAVA_TRUST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct JIT_DEBUG_INFO {
    pub dwSize: u32,
    pub dwProcessorArchitecture: u32,
    pub dwThreadID: u32,
    pub dwReserved0: u32,
    pub lpExceptionAddress: u64,
    pub lpExceptionRecord: u64,
    pub lpContextRecord: u64,
}
impl JIT_DEBUG_INFO {}
impl ::std::default::Default for JIT_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for JIT_DEBUG_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("JIT_DEBUG_INFO")
            .field("dwSize", &self.dwSize)
            .field("dwProcessorArchitecture", &self.dwProcessorArchitecture)
            .field("dwThreadID", &self.dwThreadID)
            .field("dwReserved0", &self.dwReserved0)
            .field("lpExceptionAddress", &self.lpExceptionAddress)
            .field("lpExceptionRecord", &self.lpExceptionRecord)
            .field("lpContextRecord", &self.lpContextRecord)
            .finish()
    }
}
impl ::std::cmp::PartialEq for JIT_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwProcessorArchitecture == other.dwProcessorArchitecture && self.dwThreadID == other.dwThreadID && self.dwReserved0 == other.dwReserved0 && self.lpExceptionAddress == other.lpExceptionAddress && self.lpExceptionRecord == other.lpExceptionRecord && self.lpContextRecord == other.lpContextRecord
    }
}
impl ::std::cmp::Eq for JIT_DEBUG_INFO {}
unsafe impl ::windows::runtime::Abi for JIT_DEBUG_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct KEY_SET_INFORMATION_CLASS(pub i32);
pub const KeyWriteTimeInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(0i32);
pub const KeyWow64FlagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(1i32);
pub const KeyControlFlagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(2i32);
pub const KeySetVirtualizationInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(3i32);
pub const KeySetDebugInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(4i32);
pub const KeySetHandleTagsInformation: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(5i32);
pub const MaxKeySetInfoClass: KEY_SET_INFORMATION_CLASS = KEY_SET_INFORMATION_CLASS(6i32);
impl ::std::convert::From<i32> for KEY_SET_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KEY_SET_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct KEY_VALUE_ENTRY {
    pub ValueName: *mut super::super::Foundation::UNICODE_STRING,
    pub DataLength: u32,
    pub DataOffset: u32,
    pub Type: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl KEY_VALUE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for KEY_VALUE_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for KEY_VALUE_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("KEY_VALUE_ENTRY").field("ValueName", &self.ValueName).field("DataLength", &self.DataLength).field("DataOffset", &self.DataOffset).field("Type", &self.Type).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for KEY_VALUE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ValueName == other.ValueName && self.DataLength == other.DataLength && self.DataOffset == other.DataOffset && self.Type == other.Type
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for KEY_VALUE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for KEY_VALUE_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct LDR_DATA_TABLE_ENTRY {
    pub Reserved1: [*mut ::std::ffi::c_void; 2],
    pub InMemoryOrderLinks: super::Kernel::LIST_ENTRY,
    pub Reserved2: [*mut ::std::ffi::c_void; 2],
    pub DllBase: *mut ::std::ffi::c_void,
    pub Reserved3: [*mut ::std::ffi::c_void; 2],
    pub FullDllName: super::super::Foundation::UNICODE_STRING,
    pub Reserved4: [u8; 8],
    pub Reserved5: [*mut ::std::ffi::c_void; 3],
    pub Anonymous: LDR_DATA_TABLE_ENTRY_0,
    pub TimeDateStamp: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl LDR_DATA_TABLE_ENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::std::default::Default for LDR_DATA_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::std::cmp::PartialEq for LDR_DATA_TABLE_ENTRY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::std::cmp::Eq for LDR_DATA_TABLE_ENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::runtime::Abi for LDR_DATA_TABLE_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union LDR_DATA_TABLE_ENTRY_0 {
    pub CheckSum: u32,
    pub Reserved6: *mut ::std::ffi::c_void,
}
impl LDR_DATA_TABLE_ENTRY_0 {}
impl ::std::default::Default for LDR_DATA_TABLE_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for LDR_DATA_TABLE_ENTRY_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for LDR_DATA_TABLE_ENTRY_0 {}
unsafe impl ::windows::runtime::Abi for LDR_DATA_TABLE_ENTRY_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const LIS_NOGRPCONV: u32 = 2u32;
pub const LIS_QUIET: u32 = 1u32;
pub const LOGON32_PROVIDER_VIRTUAL: u32 = 4u32;
pub const LOGON32_PROVIDER_WINNT35: u32 = 1u32;
pub const LOGON_ZERO_PASSWORD_BUFFER: u32 = 2147483648u32;
pub type LPFIBER_START_ROUTINE = unsafe extern "system" fn(lpfiberparameter: *mut ::std::ffi::c_void);
pub const LPTx: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LaunchINFSectionExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hwnd: Param0, hinstance: Param1, pszparms: Param2, nshow: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LaunchINFSectionExW(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PWSTR, nshow: i32) -> ::windows::runtime::HRESULT;
        }
        LaunchINFSectionExW(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), ::std::mem::transmute(nshow)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LaunchINFSectionW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hwndowner: Param0, hinstance: Param1, pszparams: Param2, nshow: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LaunchINFSectionW(hwndowner: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparams: super::super::Foundation::PWSTR, nshow: i32) -> i32;
        }
        ::std::mem::transmute(LaunchINFSectionW(hwndowner.into_param().abi(), hinstance.into_param().abi(), pszparams.into_param().abi(), ::std::mem::transmute(nshow)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LocalCompact(uminfree: u32) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalCompact(uminfree: u32) -> usize;
        }
        ::std::mem::transmute(LocalCompact(::std::mem::transmute(uminfree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn LocalShrink(hmem: isize, cbnewsize: u32) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LocalShrink(hmem: isize, cbnewsize: u32) -> usize;
        }
        ::std::mem::transmute(LocalShrink(::std::mem::transmute(hmem), ::std::mem::transmute(cbnewsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MARKPARITY: u32 = 3u32;
pub const MAXINTATOM: u32 = 49152u32;
pub const MAX_COMPUTERNAME_LENGTH: u32 = 15u32;
pub const MAX_TDI_ENTITIES: u32 = 4096u32;
pub const MCW_DEFAULT: u32 = 0u32;
pub const MCW_HIDDEN: u32 = 16u32;
pub const MCW_RECT: u32 = 1u32;
pub const MCW_SCREEN: u32 = 4u32;
pub const MCW_VERTICAL: u32 = 8u32;
pub const MCW_WINDOW: u32 = 2u32;
pub const MICROSOFT_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 0u32;
pub const MICROSOFT_WINDOWS_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 0u32;
pub const MODE_WINDOW: u32 = 1u32;
#[inline]
pub unsafe fn MulDiv(nnumber: i32, nnumerator: i32, ndenominator: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MulDiv(nnumber: i32, nnumerator: i32, ndenominator: i32) -> i32;
        }
        ::std::mem::transmute(MulDiv(::std::mem::transmute(nnumber), ::std::mem::transmute(nnumerator), ::std::mem::transmute(ndenominator)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const NOPARITY: u32 = 0u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NT_CREATE_FILE_DISPOSITION(pub u32);
pub const FILE_SUPERSEDE: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(0u32);
pub const FILE_CREATE: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(2u32);
pub const FILE_OPEN: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(1u32);
pub const FILE_OPEN_IF: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(3u32);
pub const FILE_OVERWRITE: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(4u32);
pub const FILE_OVERWRITE_IF: NT_CREATE_FILE_DISPOSITION = NT_CREATE_FILE_DISPOSITION(5u32);
impl ::std::convert::From<u32> for NT_CREATE_FILE_DISPOSITION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NT_CREATE_FILE_DISPOSITION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for NT_CREATE_FILE_DISPOSITION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NT_CREATE_FILE_DISPOSITION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NT_CREATE_FILE_DISPOSITION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NT_CREATE_FILE_DISPOSITION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NT_CREATE_FILE_DISPOSITION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NeedReboot(dwrebootcheck: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NeedReboot(dwrebootcheck: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(NeedReboot(::std::mem::transmute(dwrebootcheck)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn NeedRebootInit() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NeedRebootInit() -> u32;
        }
        ::std::mem::transmute(NeedRebootInit())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct NewProcessCauseConstants(pub i32);
pub const ProtectedModeRedirect: NewProcessCauseConstants = NewProcessCauseConstants(1i32);
impl ::std::convert::From<i32> for NewProcessCauseConstants {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NewProcessCauseConstants {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtClose<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(handle: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtClose(handle: super::super::Foundation::HANDLE) -> super::super::Foundation::NTSTATUS;
        }
        NtClose(handle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn NtCreateFile(filehandle: *mut super::super::Foundation::HANDLE, desiredaccess: u32, objectattributes: *mut OBJECT_ATTRIBUTES, iostatusblock: *mut IO_STATUS_BLOCK, allocationsize: *mut i64, fileattributes: u32, shareaccess: super::super::Storage::FileSystem::FILE_SHARE_MODE, createdisposition: NT_CREATE_FILE_DISPOSITION, createoptions: u32, eabuffer: *mut ::std::ffi::c_void, ealength: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtCreateFile(filehandle: *mut super::super::Foundation::HANDLE, desiredaccess: u32, objectattributes: *mut OBJECT_ATTRIBUTES, iostatusblock: *mut IO_STATUS_BLOCK, allocationsize: *mut i64, fileattributes: u32, shareaccess: super::super::Storage::FileSystem::FILE_SHARE_MODE, createdisposition: NT_CREATE_FILE_DISPOSITION, createoptions: u32, eabuffer: *mut ::std::ffi::c_void, ealength: u32) -> super::super::Foundation::NTSTATUS;
        }
        NtCreateFile(
            ::std::mem::transmute(filehandle),
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(objectattributes),
            ::std::mem::transmute(iostatusblock),
            ::std::mem::transmute(allocationsize),
            ::std::mem::transmute(fileattributes),
            ::std::mem::transmute(shareaccess),
            ::std::mem::transmute(createdisposition),
            ::std::mem::transmute(createoptions),
            ::std::mem::transmute(eabuffer),
            ::std::mem::transmute(ealength),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtDeviceIoControlFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(
    filehandle: Param0,
    event: Param1,
    apcroutine: ::std::option::Option<PIO_APC_ROUTINE>,
    apccontext: *mut ::std::ffi::c_void,
    iostatusblock: *mut IO_STATUS_BLOCK,
    iocontrolcode: u32,
    inputbuffer: *mut ::std::ffi::c_void,
    inputbufferlength: u32,
    outputbuffer: *mut ::std::ffi::c_void,
    outputbufferlength: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtDeviceIoControlFile(filehandle: super::super::Foundation::HANDLE, event: super::super::Foundation::HANDLE, apcroutine: ::windows::runtime::RawPtr, apccontext: *mut ::std::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, iocontrolcode: u32, inputbuffer: *mut ::std::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut ::std::ffi::c_void, outputbufferlength: u32) -> super::super::Foundation::NTSTATUS;
        }
        NtDeviceIoControlFile(
            filehandle.into_param().abi(),
            event.into_param().abi(),
            ::std::mem::transmute(apcroutine),
            ::std::mem::transmute(apccontext),
            ::std::mem::transmute(iostatusblock),
            ::std::mem::transmute(iocontrolcode),
            ::std::mem::transmute(inputbuffer),
            ::std::mem::transmute(inputbufferlength),
            ::std::mem::transmute(outputbuffer),
            ::std::mem::transmute(outputbufferlength),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtNotifyChangeMultipleKeys<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>, Param11: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(
    masterkeyhandle: Param0,
    count: u32,
    subordinateobjects: *const OBJECT_ATTRIBUTES,
    event: Param3,
    apcroutine: ::std::option::Option<PIO_APC_ROUTINE>,
    apccontext: *const ::std::ffi::c_void,
    iostatusblock: *mut IO_STATUS_BLOCK,
    completionfilter: u32,
    watchtree: Param8,
    buffer: *mut ::std::ffi::c_void,
    buffersize: u32,
    asynchronous: Param11,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtNotifyChangeMultipleKeys(
                masterkeyhandle: super::super::Foundation::HANDLE,
                count: u32,
                subordinateobjects: *const OBJECT_ATTRIBUTES,
                event: super::super::Foundation::HANDLE,
                apcroutine: ::windows::runtime::RawPtr,
                apccontext: *const ::std::ffi::c_void,
                iostatusblock: *mut IO_STATUS_BLOCK,
                completionfilter: u32,
                watchtree: super::super::Foundation::BOOLEAN,
                buffer: *mut ::std::ffi::c_void,
                buffersize: u32,
                asynchronous: super::super::Foundation::BOOLEAN,
            ) -> super::super::Foundation::NTSTATUS;
        }
        NtNotifyChangeMultipleKeys(
            masterkeyhandle.into_param().abi(),
            ::std::mem::transmute(count),
            ::std::mem::transmute(subordinateobjects),
            event.into_param().abi(),
            ::std::mem::transmute(apcroutine),
            ::std::mem::transmute(apccontext),
            ::std::mem::transmute(iostatusblock),
            ::std::mem::transmute(completionfilter),
            watchtree.into_param().abi(),
            ::std::mem::transmute(buffer),
            ::std::mem::transmute(buffersize),
            asynchronous.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtOpenFile(filehandle: *mut super::super::Foundation::HANDLE, desiredaccess: u32, objectattributes: *mut OBJECT_ATTRIBUTES, iostatusblock: *mut IO_STATUS_BLOCK, shareaccess: u32, openoptions: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtOpenFile(filehandle: *mut super::super::Foundation::HANDLE, desiredaccess: u32, objectattributes: *mut OBJECT_ATTRIBUTES, iostatusblock: *mut IO_STATUS_BLOCK, shareaccess: u32, openoptions: u32) -> super::super::Foundation::NTSTATUS;
        }
        NtOpenFile(::std::mem::transmute(filehandle), ::std::mem::transmute(desiredaccess), ::std::mem::transmute(objectattributes), ::std::mem::transmute(iostatusblock), ::std::mem::transmute(shareaccess), ::std::mem::transmute(openoptions)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryMultipleValueKey<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(keyhandle: Param0, valueentries: *mut KEY_VALUE_ENTRY, entrycount: u32, valuebuffer: *mut ::std::ffi::c_void, bufferlength: *mut u32, requiredbufferlength: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtQueryMultipleValueKey(keyhandle: super::super::Foundation::HANDLE, valueentries: *mut KEY_VALUE_ENTRY, entrycount: u32, valuebuffer: *mut ::std::ffi::c_void, bufferlength: *mut u32, requiredbufferlength: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        NtQueryMultipleValueKey(keyhandle.into_param().abi(), ::std::mem::transmute(valueentries), ::std::mem::transmute(entrycount), ::std::mem::transmute(valuebuffer), ::std::mem::transmute(bufferlength), ::std::mem::transmute(requiredbufferlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(handle: Param0, objectinformationclass: OBJECT_INFORMATION_CLASS, objectinformation: *mut ::std::ffi::c_void, objectinformationlength: u32, returnlength: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtQueryObject(handle: super::super::Foundation::HANDLE, objectinformationclass: OBJECT_INFORMATION_CLASS, objectinformation: *mut ::std::ffi::c_void, objectinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        NtQueryObject(handle.into_param().abi(), ::std::mem::transmute(objectinformationclass), ::std::mem::transmute(objectinformation), ::std::mem::transmute(objectinformationlength), ::std::mem::transmute(returnlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQuerySystemInformation(systeminformationclass: SYSTEM_INFORMATION_CLASS, systeminformation: *mut ::std::ffi::c_void, systeminformationlength: u32, returnlength: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtQuerySystemInformation(systeminformationclass: SYSTEM_INFORMATION_CLASS, systeminformation: *mut ::std::ffi::c_void, systeminformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        NtQuerySystemInformation(::std::mem::transmute(systeminformationclass), ::std::mem::transmute(systeminformation), ::std::mem::transmute(systeminformationlength), ::std::mem::transmute(returnlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQuerySystemTime(systemtime: *mut i64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtQuerySystemTime(systemtime: *mut i64) -> super::super::Foundation::NTSTATUS;
        }
        NtQuerySystemTime(::std::mem::transmute(systemtime)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryTimerResolution(maximumtime: *mut u32, minimumtime: *mut u32, currenttime: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtQueryTimerResolution(maximumtime: *mut u32, minimumtime: *mut u32, currenttime: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        NtQueryTimerResolution(::std::mem::transmute(maximumtime), ::std::mem::transmute(minimumtime), ::std::mem::transmute(currenttime)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtRenameKey<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(keyhandle: Param0, newname: *const super::super::Foundation::UNICODE_STRING) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtRenameKey(keyhandle: super::super::Foundation::HANDLE, newname: *const super::super::Foundation::UNICODE_STRING) -> super::super::Foundation::NTSTATUS;
        }
        NtRenameKey(keyhandle.into_param().abi(), ::std::mem::transmute(newname)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtSetInformationKey<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(keyhandle: Param0, keysetinformationclass: KEY_SET_INFORMATION_CLASS, keysetinformation: *const ::std::ffi::c_void, keysetinformationlength: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtSetInformationKey(keyhandle: super::super::Foundation::HANDLE, keysetinformationclass: KEY_SET_INFORMATION_CLASS, keysetinformation: *const ::std::ffi::c_void, keysetinformationlength: u32) -> super::super::Foundation::NTSTATUS;
        }
        NtSetInformationKey(keyhandle.into_param().abi(), ::std::mem::transmute(keysetinformationclass), ::std::mem::transmute(keysetinformation), ::std::mem::transmute(keysetinformationlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
#[inline]
pub unsafe fn NtSetInformationThread<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(threadhandle: Param0, threadinformationclass: super::Threading::THREADINFOCLASS, threadinformation: *const ::std::ffi::c_void, threadinformationlength: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtSetInformationThread(threadhandle: super::super::Foundation::HANDLE, threadinformationclass: super::Threading::THREADINFOCLASS, threadinformation: *const ::std::ffi::c_void, threadinformationlength: u32) -> super::super::Foundation::NTSTATUS;
        }
        NtSetInformationThread(threadhandle.into_param().abi(), ::std::mem::transmute(threadinformationclass), ::std::mem::transmute(threadinformation), ::std::mem::transmute(threadinformationlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtWaitForSingleObject<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(handle: Param0, alertable: Param1, timeout: *mut i64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtWaitForSingleObject(handle: super::super::Foundation::HANDLE, alertable: super::super::Foundation::BOOLEAN, timeout: *mut i64) -> super::super::Foundation::NTSTATUS;
        }
        NtWaitForSingleObject(handle.into_param().abi(), alertable.into_param().abi(), ::std::mem::transmute(timeout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OBJECT_ATTRIBUTES {
    pub Length: u32,
    pub RootDirectory: super::super::Foundation::HANDLE,
    pub ObjectName: *mut super::super::Foundation::UNICODE_STRING,
    pub Attributes: u32,
    pub SecurityDescriptor: *mut ::std::ffi::c_void,
    pub SecurityQualityOfService: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl OBJECT_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OBJECT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OBJECT_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OBJECT_ATTRIBUTES")
            .field("Length", &self.Length)
            .field("RootDirectory", &self.RootDirectory)
            .field("ObjectName", &self.ObjectName)
            .field("Attributes", &self.Attributes)
            .field("SecurityDescriptor", &self.SecurityDescriptor)
            .field("SecurityQualityOfService", &self.SecurityQualityOfService)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OBJECT_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.RootDirectory == other.RootDirectory && self.ObjectName == other.ObjectName && self.Attributes == other.Attributes && self.SecurityDescriptor == other.SecurityDescriptor && self.SecurityQualityOfService == other.SecurityQualityOfService
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OBJECT_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OBJECT_ATTRIBUTES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct OBJECT_INFORMATION_CLASS(pub i32);
pub const ObjectBasicInformation: OBJECT_INFORMATION_CLASS = OBJECT_INFORMATION_CLASS(0i32);
pub const ObjectTypeInformation: OBJECT_INFORMATION_CLASS = OBJECT_INFORMATION_CLASS(2i32);
impl ::std::convert::From<i32> for OBJECT_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OBJECT_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const ODDPARITY: u32 = 1u32;
pub const OFS_MAXPATHNAME: u32 = 128u32;
pub const ONE5STOPBITS: u32 = 1u32;
pub const ONESTOPBIT: u32 = 0u32;
pub const OPERATION_API_VERSION: u32 = 1u32;
pub const OVERWRITE_HIDDEN: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenINFEngineA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pszinffilename: Param0, pszinstallsection: Param1, dwflags: u32, phinf: *mut *mut ::std::ffi::c_void, pvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenINFEngineA(pszinffilename: super::super::Foundation::PSTR, pszinstallsection: super::super::Foundation::PSTR, dwflags: u32, phinf: *mut *mut ::std::ffi::c_void, pvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OpenINFEngineA(pszinffilename.into_param().abi(), pszinstallsection.into_param().abi(), ::std::mem::transmute(dwflags), ::std::mem::transmute(phinf), ::std::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenINFEngineW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszinffilename: Param0, pszinstallsection: Param1, dwflags: u32, phinf: *mut *mut ::std::ffi::c_void, pvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenINFEngineW(pszinffilename: super::super::Foundation::PWSTR, pszinstallsection: super::super::Foundation::PWSTR, dwflags: u32, phinf: *mut *mut ::std::ffi::c_void, pvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        OpenINFEngineW(pszinffilename.into_param().abi(), pszinstallsection.into_param().abi(), ::std::mem::transmute(dwflags), ::std::mem::transmute(phinf), ::std::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenMutexA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenMutexA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenMutexA(::std::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenSemaphoreA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenSemaphoreA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenSemaphoreA(::std::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenWaitableTimerA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lptimername: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenWaitableTimerA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lptimername: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(OpenWaitableTimerA(::std::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lptimername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub type PDELAYLOAD_FAILURE_DLL_CALLBACK = unsafe extern "system" fn(notificationreason: u32, delayloadinfo: *const DELAYLOAD_INFO) -> *mut ::std::ffi::c_void;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PERUSERSECTIONA {
    pub szGUID: [super::super::Foundation::CHAR; 59],
    pub szDispName: [super::super::Foundation::CHAR; 128],
    pub szLocale: [super::super::Foundation::CHAR; 10],
    pub szStub: [super::super::Foundation::CHAR; 1040],
    pub szVersion: [super::super::Foundation::CHAR; 32],
    pub szCompID: [super::super::Foundation::CHAR; 128],
    pub dwIsInstalled: u32,
    pub bRollback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl PERUSERSECTIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PERUSERSECTIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PERUSERSECTIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PERUSERSECTIONA")
            .field("szGUID", &self.szGUID)
            .field("szDispName", &self.szDispName)
            .field("szLocale", &self.szLocale)
            .field("szStub", &self.szStub)
            .field("szVersion", &self.szVersion)
            .field("szCompID", &self.szCompID)
            .field("dwIsInstalled", &self.dwIsInstalled)
            .field("bRollback", &self.bRollback)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PERUSERSECTIONA {
    fn eq(&self, other: &Self) -> bool {
        self.szGUID == other.szGUID && self.szDispName == other.szDispName && self.szLocale == other.szLocale && self.szStub == other.szStub && self.szVersion == other.szVersion && self.szCompID == other.szCompID && self.dwIsInstalled == other.dwIsInstalled && self.bRollback == other.bRollback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PERUSERSECTIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PERUSERSECTIONA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PERUSERSECTIONW {
    pub szGUID: [u16; 59],
    pub szDispName: [u16; 128],
    pub szLocale: [u16; 10],
    pub szStub: [u16; 1040],
    pub szVersion: [u16; 32],
    pub szCompID: [u16; 128],
    pub dwIsInstalled: u32,
    pub bRollback: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl PERUSERSECTIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PERUSERSECTIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PERUSERSECTIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PERUSERSECTIONW")
            .field("szGUID", &self.szGUID)
            .field("szDispName", &self.szDispName)
            .field("szLocale", &self.szLocale)
            .field("szStub", &self.szStub)
            .field("szVersion", &self.szVersion)
            .field("szCompID", &self.szCompID)
            .field("dwIsInstalled", &self.dwIsInstalled)
            .field("bRollback", &self.bRollback)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PERUSERSECTIONW {
    fn eq(&self, other: &Self) -> bool {
        self.szGUID == other.szGUID && self.szDispName == other.szDispName && self.szLocale == other.szLocale && self.szStub == other.szStub && self.szVersion == other.szVersion && self.szCompID == other.szCompID && self.dwIsInstalled == other.dwIsInstalled && self.bRollback == other.bRollback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PERUSERSECTIONW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PERUSERSECTIONW {
    type Abi = Self;
    type DefaultType = Self;
}
pub type PFEATURE_STATE_CHANGE_CALLBACK = unsafe extern "system" fn(context: *const ::std::ffi::c_void);
pub type PFIBER_CALLOUT_ROUTINE = unsafe extern "system" fn(lpparameter: *mut ::std::ffi::c_void) -> *mut ::std::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type PIO_APC_ROUTINE = unsafe extern "system" fn(apccontext: *mut ::std::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, reserved: u32);
pub type PPS_POST_PROCESS_INIT_ROUTINE = unsafe extern "system" fn();
#[cfg(feature = "Win32_Foundation")]
pub type PQUERYACTCTXW_FUNC = unsafe extern "system" fn(dwflags: u32, hactctx: super::super::Foundation::HANDLE, pvsubinstance: *const ::std::ffi::c_void, ulinfoclass: u32, pvbuffer: *mut ::std::ffi::c_void, cbbuffer: usize, pcbwrittenorrequired: *mut usize) -> super::super::Foundation::BOOL;
pub const PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT: u32 = 1u32;
pub const PROCESS_CREATION_CHILD_PROCESS_OVERRIDE: u32 = 2u32;
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED: u32 = 1u32;
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED_UNLESS_SECURE: u32 = 4u32;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_DISABLE_PROCESS_TREE: u32 = 2u32;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_ENABLE_PROCESS_TREE: u32 = 1u32;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_OVERRIDE: u32 = 4u32;
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ATL_THUNK_ENABLE: u32 = 2u32;
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ENABLE: u32 = 1u32;
pub const PROCESS_CREATION_MITIGATION_POLICY_SEHOP_ENABLE: u32 = 4u32;
pub const PROC_THREAD_ATTRIBUTE_ADDITIVE: u32 = 262144u32;
pub const PROC_THREAD_ATTRIBUTE_INPUT: u32 = 131072u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROC_THREAD_ATTRIBUTE_NUM(pub i32);
pub const ProcThreadAttributeParentProcess: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(0i32);
pub const ProcThreadAttributeHandleList: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(2i32);
pub const ProcThreadAttributeGroupAffinity: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(3i32);
pub const ProcThreadAttributePreferredNode: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(4i32);
pub const ProcThreadAttributeIdealProcessor: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(5i32);
pub const ProcThreadAttributeUmsThread: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(6i32);
pub const ProcThreadAttributeMitigationPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(7i32);
pub const ProcThreadAttributeSecurityCapabilities: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(9i32);
pub const ProcThreadAttributeProtectionLevel: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(11i32);
pub const ProcThreadAttributeJobList: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(13i32);
pub const ProcThreadAttributeChildProcessPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(14i32);
pub const ProcThreadAttributeAllApplicationPackagesPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(15i32);
pub const ProcThreadAttributeWin32kFilter: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(16i32);
pub const ProcThreadAttributeSafeOpenPromptOriginClaim: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(17i32);
pub const ProcThreadAttributeDesktopAppPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(18i32);
pub const ProcThreadAttributePseudoConsole: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(22i32);
pub const ProcThreadAttributeMitigationAuditPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(24i32);
pub const ProcThreadAttributeMachineType: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(25i32);
pub const ProcThreadAttributeComponentFilter: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(26i32);
pub const ProcThreadAttributeEnableOptionalXStateFeatures: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(27i32);
impl ::std::convert::From<i32> for PROC_THREAD_ATTRIBUTE_NUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROC_THREAD_ATTRIBUTE_NUM {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PROC_THREAD_ATTRIBUTE_NUMBER: u32 = 65535u32;
pub const PROC_THREAD_ATTRIBUTE_THREAD: u32 = 65536u32;
pub const PROGRESS_CANCEL: u32 = 1u32;
pub const PROGRESS_CONTINUE: u32 = 0u32;
pub const PROGRESS_QUIET: u32 = 3u32;
pub const PROGRESS_STOP: u32 = 2u32;
pub const PROTECTION_LEVEL_SAME: u32 = 4294967295u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PUBLIC_OBJECT_BASIC_INFORMATION {
    pub Attributes: u32,
    pub GrantedAccess: u32,
    pub HandleCount: u32,
    pub PointerCount: u32,
    pub Reserved: [u32; 10],
}
impl PUBLIC_OBJECT_BASIC_INFORMATION {}
impl ::std::default::Default for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PUBLIC_OBJECT_BASIC_INFORMATION").field("Attributes", &self.Attributes).field("GrantedAccess", &self.GrantedAccess).field("HandleCount", &self.HandleCount).field("PointerCount", &self.PointerCount).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Attributes == other.Attributes && self.GrantedAccess == other.GrantedAccess && self.HandleCount == other.HandleCount && self.PointerCount == other.PointerCount && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for PUBLIC_OBJECT_BASIC_INFORMATION {}
unsafe impl ::windows::runtime::Abi for PUBLIC_OBJECT_BASIC_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct PUBLIC_OBJECT_TYPE_INFORMATION {
    pub TypeName: super::super::Foundation::UNICODE_STRING,
    pub Reserved: [u32; 22],
}
#[cfg(feature = "Win32_Foundation")]
impl PUBLIC_OBJECT_TYPE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PUBLIC_OBJECT_TYPE_INFORMATION").field("TypeName", &self.TypeName).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TypeName == other.TypeName && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for PUBLIC_OBJECT_TYPE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for PUBLIC_OBJECT_TYPE_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type PWINSTATIONQUERYINFORMATIONW = unsafe extern "system" fn(param0: super::super::Foundation::HANDLE, param1: u32, param2: WINSTATIONINFOCLASS, param3: *mut ::std::ffi::c_void, param4: u32, param5: *mut u32) -> super::super::Foundation::BOOLEAN;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_ISAPPAPPROVEDBYPOLICY_API = unsafe extern "system" fn(packagefamilyname: super::super::Foundation::PWSTR, packageversion: u64) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_ISDYNAMICCODEPOLICYENABLED_API = unsafe extern "system" fn(pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_ISPRODUCTIONCONFIGURATION_API = unsafe extern "system" fn(isproductionconfiguration: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_ISWCOSPRODUCTIONCONFIGURATION_API = unsafe extern "system" fn(isproductionconfiguration: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_QUERYDEVICESECURITYINFORMATION_API = unsafe extern "system" fn(information: *mut WLDP_DEVICE_SECURITY_INFORMATION, informationlength: u32, returnlength: *mut u32) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_QUERYDYNAMICODETRUST_API = unsafe extern "system" fn(filehandle: super::super::Foundation::HANDLE, baseimage: *const ::std::ffi::c_void, imagesize: u32) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_QUERYPOLICYSETTINGENABLED2_API = unsafe extern "system" fn(setting: super::super::Foundation::PWSTR, enabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_QUERYPOLICYSETTINGENABLED_API = unsafe extern "system" fn(setting: WLDP_POLICY_SETTING, enabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
pub type PWLDP_QUERYWINDOWSLOCKDOWNMODE_API = unsafe extern "system" fn(lockdownmode: *mut WLDP_WINDOWS_LOCKDOWN_MODE) -> ::windows::runtime::HRESULT;
pub type PWLDP_QUERYWINDOWSLOCKDOWNRESTRICTION_API = unsafe extern "system" fn(lockdownrestriction: *mut WLDP_WINDOWS_LOCKDOWN_RESTRICTION) -> ::windows::runtime::HRESULT;
pub type PWLDP_RESETPRODUCTIONCONFIGURATION_API = unsafe extern "system" fn() -> ::windows::runtime::HRESULT;
pub type PWLDP_RESETWCOSPRODUCTIONCONFIGURATION_API = unsafe extern "system" fn() -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PWLDP_SETDYNAMICCODETRUST_API = unsafe extern "system" fn(hfilehandle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
pub type PWLDP_SETWINDOWSLOCKDOWNRESTRICTION_API = unsafe extern "system" fn(lockdownrestriction: WLDP_WINDOWS_LOCKDOWN_RESTRICTION) -> ::windows::runtime::HRESULT;
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_ADDRESS: u32 = 16u32;
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_HMODULE: u32 = 8u32;
pub const QUERY_ACTCTX_FLAG_NO_ADDREF: u32 = 2147483648u32;
pub const QUERY_ACTCTX_FLAG_USE_ACTIVE_ACTCTX: u32 = 4u32;
#[inline]
pub unsafe fn QueryAuxiliaryCounterFrequency() -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryAuxiliaryCounterFrequency(lpauxiliarycounterfrequency: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        QueryAuxiliaryCounterFrequency(&mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryIdleProcessorCycleTime(bufferlength: *mut u32, processoridlecycletime: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryIdleProcessorCycleTime(bufferlength: *mut u32, processoridlecycletime: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryIdleProcessorCycleTime(::std::mem::transmute(bufferlength), ::std::mem::transmute(processoridlecycletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryIdleProcessorCycleTimeEx(group: u16, bufferlength: *mut u32, processoridlecycletime: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryIdleProcessorCycleTimeEx(group: u16, bufferlength: *mut u32, processoridlecycletime: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryIdleProcessorCycleTimeEx(::std::mem::transmute(group), ::std::mem::transmute(bufferlength), ::std::mem::transmute(processoridlecycletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn QueryInterruptTime(lpinterrupttime: *mut u64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryInterruptTime(lpinterrupttime: *mut u64);
        }
        ::std::mem::transmute(QueryInterruptTime(::std::mem::transmute(lpinterrupttime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn QueryInterruptTimePrecise(lpinterrupttimeprecise: *mut u64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryInterruptTimePrecise(lpinterrupttimeprecise: *mut u64);
        }
        ::std::mem::transmute(QueryInterruptTimePrecise(::std::mem::transmute(lpinterrupttimeprecise)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryProcessCycleTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(processhandle: Param0, cycletime: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryProcessCycleTime(processhandle: super::super::Foundation::HANDLE, cycletime: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryProcessCycleTime(processhandle.into_param().abi(), ::std::mem::transmute(cycletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryThreadCycleTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(threadhandle: Param0, cycletime: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryThreadCycleTime(threadhandle: super::super::Foundation::HANDLE, cycletime: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryThreadCycleTime(threadhandle.into_param().abi(), ::std::mem::transmute(cycletime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryUnbiasedInterruptTime(unbiasedtime: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryUnbiasedInterruptTime(unbiasedtime: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryUnbiasedInterruptTime(::std::mem::transmute(unbiasedtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn QueryUnbiasedInterruptTimePrecise(lpunbiasedinterrupttimeprecise: *mut u64) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryUnbiasedInterruptTimePrecise(lpunbiasedinterrupttimeprecise: *mut u64);
        }
        ::std::mem::transmute(QueryUnbiasedInterruptTimePrecise(::std::mem::transmute(lpunbiasedinterrupttimeprecise)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const RECOVERY_DEFAULT_PING_INTERVAL: u32 = 5000u32;
#[cfg(feature = "Win32_Foundation")]
pub type REGINSTALLA = unsafe extern "system" fn(hm: super::super::Foundation::HINSTANCE, pszsection: super::super::Foundation::PSTR, psttable: *mut STRTABLEA) -> ::windows::runtime::HRESULT;
pub const REMOTE_PROTOCOL_INFO_FLAG_LOOPBACK: u32 = 1u32;
pub const REMOTE_PROTOCOL_INFO_FLAG_OFFLINE: u32 = 2u32;
pub const REMOTE_PROTOCOL_INFO_FLAG_PERSISTENT_HANDLE: u32 = 4u32;
pub const RESETDEV: u32 = 7u32;
pub const RESTART_MAX_CMD_LINE: u32 = 1024u32;
pub const RPI_FLAG_SMB2_SHARECAP_CLUSTER: u32 = 64u32;
pub const RPI_FLAG_SMB2_SHARECAP_CONTINUOUS_AVAILABILITY: u32 = 16u32;
pub const RPI_FLAG_SMB2_SHARECAP_DFS: u32 = 8u32;
pub const RPI_FLAG_SMB2_SHARECAP_SCALEOUT: u32 = 32u32;
pub const RPI_FLAG_SMB2_SHARECAP_TIMEWARP: u32 = 2u32;
pub const RPI_SMB2_FLAG_SERVERCAP_DFS: u32 = 1u32;
pub const RPI_SMB2_FLAG_SERVERCAP_DIRECTORY_LEASING: u32 = 32u32;
pub const RPI_SMB2_FLAG_SERVERCAP_LARGEMTU: u32 = 4u32;
pub const RPI_SMB2_FLAG_SERVERCAP_LEASING: u32 = 2u32;
pub const RPI_SMB2_FLAG_SERVERCAP_MULTICHANNEL: u32 = 8u32;
pub const RPI_SMB2_FLAG_SERVERCAP_PERSISTENT_HANDLES: u32 = 16u32;
pub const RSC_FLAG_DELAYREGISTEROCX: u32 = 512u32;
pub const RSC_FLAG_INF: u32 = 1u32;
pub const RSC_FLAG_NGCONV: u32 = 8u32;
pub const RSC_FLAG_QUIET: u32 = 4u32;
pub const RSC_FLAG_SETUPAPI: u32 = 1024u32;
pub const RSC_FLAG_SKIPDISKSPACECHECK: u32 = 2u32;
pub const RSC_FLAG_UPDHLPDLLS: u32 = 16u32;
pub const RTS_CONTROL_DISABLE: u32 = 0u32;
pub const RTS_CONTROL_ENABLE: u32 = 1u32;
pub const RTS_CONTROL_HANDSHAKE: u32 = 2u32;
pub const RTS_CONTROL_TOGGLE: u32 = 3u32;
pub const RUNCMDS_DELAYPOSTCMD: u32 = 4u32;
pub const RUNCMDS_NOWAIT: u32 = 2u32;
pub const RUNCMDS_QUIET: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig: *const super::SystemServices::CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig: *const super::SystemServices::CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32;
        }
        ::std::mem::transmute(RaiseCustomSystemEventTrigger(::std::mem::transmute(customsystemeventtriggerconfig)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RebootCheckOnInstallA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hwnd: Param0, pszinf: Param1, pszsec: Param2, dwreserved: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RebootCheckOnInstallA(hwnd: super::super::Foundation::HWND, pszinf: super::super::Foundation::PSTR, pszsec: super::super::Foundation::PSTR, dwreserved: u32) -> ::windows::runtime::HRESULT;
        }
        RebootCheckOnInstallA(hwnd.into_param().abi(), pszinf.into_param().abi(), pszsec.into_param().abi(), ::std::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RebootCheckOnInstallW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hwnd: Param0, pszinf: Param1, pszsec: Param2, dwreserved: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RebootCheckOnInstallW(hwnd: super::super::Foundation::HWND, pszinf: super::super::Foundation::PWSTR, pszsec: super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::runtime::HRESULT;
        }
        RebootCheckOnInstallW(hwnd.into_param().abi(), pszinf.into_param().abi(), pszsec.into_param().abi(), ::std::mem::transmute(dwreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RecordFeatureError(featureid: u32, error: *const FEATURE_ERROR) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RecordFeatureError(featureid: u32, error: *const FEATURE_ERROR);
        }
        ::std::mem::transmute(RecordFeatureError(::std::mem::transmute(featureid), ::std::mem::transmute(error)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RecordFeatureUsage<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(featureid: u32, kind: u32, addend: u32, originname: Param3) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RecordFeatureUsage(featureid: u32, kind: u32, addend: u32, originname: super::super::Foundation::PSTR);
        }
        ::std::mem::transmute(RecordFeatureUsage(::std::mem::transmute(featureid), ::std::mem::transmute(kind), ::std::mem::transmute(addend), originname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RefreshConstants(pub i32);
pub const REFRESH_NORMAL: RefreshConstants = RefreshConstants(0i32);
pub const REFRESH_IFEXPIRED: RefreshConstants = RefreshConstants(1i32);
pub const REFRESH_COMPLETELY: RefreshConstants = RefreshConstants(3i32);
impl ::std::convert::From<i32> for RefreshConstants {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RefreshConstants {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegInstallA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hmod: Param0, pszsection: Param1, psttable: *const STRTABLEA) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegInstallA(hmod: super::super::Foundation::HINSTANCE, pszsection: super::super::Foundation::PSTR, psttable: *const STRTABLEA) -> ::windows::runtime::HRESULT;
        }
        RegInstallA(hmod.into_param().abi(), pszsection.into_param().abi(), ::std::mem::transmute(psttable)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegInstallW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hmod: Param0, pszsection: Param1, psttable: *const STRTABLEW) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegInstallW(hmod: super::super::Foundation::HINSTANCE, pszsection: super::super::Foundation::PWSTR, psttable: *const STRTABLEW) -> ::windows::runtime::HRESULT;
        }
        RegInstallW(hmod.into_param().abi(), pszsection.into_param().abi(), ::std::mem::transmute(psttable)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegRestoreAllA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>>(hwnd: Param0, psztitlestring: Param1, hkbckupkey: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegRestoreAllA(hwnd: super::super::Foundation::HWND, psztitlestring: super::super::Foundation::PSTR, hkbckupkey: super::Registry::HKEY) -> ::windows::runtime::HRESULT;
        }
        RegRestoreAllA(hwnd.into_param().abi(), psztitlestring.into_param().abi(), hkbckupkey.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegRestoreAllW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>>(hwnd: Param0, psztitlestring: Param1, hkbckupkey: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegRestoreAllW(hwnd: super::super::Foundation::HWND, psztitlestring: super::super::Foundation::PWSTR, hkbckupkey: super::Registry::HKEY) -> ::windows::runtime::HRESULT;
        }
        RegRestoreAllW(hwnd.into_param().abi(), psztitlestring.into_param().abi(), hkbckupkey.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegSaveRestoreA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(
    hwnd: Param0,
    psztitlestring: Param1,
    hkbckupkey: Param2,
    pcszrootkey: Param3,
    pcszsubkey: Param4,
    pcszvaluename: Param5,
    dwflags: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSaveRestoreA(hwnd: super::super::Foundation::HWND, psztitlestring: super::super::Foundation::PSTR, hkbckupkey: super::Registry::HKEY, pcszrootkey: super::super::Foundation::PSTR, pcszsubkey: super::super::Foundation::PSTR, pcszvaluename: super::super::Foundation::PSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        RegSaveRestoreA(hwnd.into_param().abi(), psztitlestring.into_param().abi(), hkbckupkey.into_param().abi(), pcszrootkey.into_param().abi(), pcszsubkey.into_param().abi(), pcszvaluename.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegSaveRestoreOnINFA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>, Param5: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>>(
    hwnd: Param0,
    psztitle: Param1,
    pszinf: Param2,
    pszsection: Param3,
    hhklmbackkey: Param4,
    hhkcubackkey: Param5,
    dwflags: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSaveRestoreOnINFA(hwnd: super::super::Foundation::HWND, psztitle: super::super::Foundation::PSTR, pszinf: super::super::Foundation::PSTR, pszsection: super::super::Foundation::PSTR, hhklmbackkey: super::Registry::HKEY, hhkcubackkey: super::Registry::HKEY, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        RegSaveRestoreOnINFA(hwnd.into_param().abi(), psztitle.into_param().abi(), pszinf.into_param().abi(), pszsection.into_param().abi(), hhklmbackkey.into_param().abi(), hhkcubackkey.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegSaveRestoreOnINFW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>, Param5: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>>(
    hwnd: Param0,
    psztitle: Param1,
    pszinf: Param2,
    pszsection: Param3,
    hhklmbackkey: Param4,
    hhkcubackkey: Param5,
    dwflags: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSaveRestoreOnINFW(hwnd: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, pszinf: super::super::Foundation::PWSTR, pszsection: super::super::Foundation::PWSTR, hhklmbackkey: super::Registry::HKEY, hhkcubackkey: super::Registry::HKEY, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        RegSaveRestoreOnINFW(hwnd.into_param().abi(), psztitle.into_param().abi(), pszinf.into_param().abi(), pszsection.into_param().abi(), hhklmbackkey.into_param().abi(), hhkcubackkey.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn RegSaveRestoreW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::Registry::HKEY>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    hwnd: Param0,
    psztitlestring: Param1,
    hkbckupkey: Param2,
    pcszrootkey: Param3,
    pcszsubkey: Param4,
    pcszvaluename: Param5,
    dwflags: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegSaveRestoreW(hwnd: super::super::Foundation::HWND, psztitlestring: super::super::Foundation::PWSTR, hkbckupkey: super::Registry::HKEY, pcszrootkey: super::super::Foundation::PWSTR, pcszsubkey: super::super::Foundation::PWSTR, pcszvaluename: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        RegSaveRestoreW(hwnd.into_param().abi(), psztitlestring.into_param().abi(), hkbckupkey.into_param().abi(), pcszrootkey.into_param().abi(), pcszsubkey.into_param().abi(), pcszvaluename.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplacePartitionUnit<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(targetpartition: Param0, sparepartition: Param1, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReplacePartitionUnit(targetpartition: super::super::Foundation::PWSTR, sparepartition: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReplacePartitionUnit(targetpartition.into_param().abi(), sparepartition.into_param().abi(), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RequestDeviceWakeup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hdevice: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RequestDeviceWakeup(hdevice: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RequestDeviceWakeup(hdevice.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlAnsiStringToUnicodeString<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: *mut super::Kernel::STRING, allocatedestinationstring: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlAnsiStringToUnicodeString(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: *mut super::Kernel::STRING, allocatedestinationstring: super::super::Foundation::BOOLEAN) -> super::super::Foundation::NTSTATUS;
        }
        RtlAnsiStringToUnicodeString(::std::mem::transmute(destinationstring), ::std::mem::transmute(sourcestring), allocatedestinationstring.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlCharToInteger(string: *mut i8, base: u32, value: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlCharToInteger(string: *mut i8, base: u32, value: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        RtlCharToInteger(::std::mem::transmute(string), ::std::mem::transmute(base), ::std::mem::transmute(value)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlFreeAnsiString(ansistring: *mut super::Kernel::STRING) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlFreeAnsiString(ansistring: *mut super::Kernel::STRING);
        }
        ::std::mem::transmute(RtlFreeAnsiString(::std::mem::transmute(ansistring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlFreeOemString(oemstring: *mut super::Kernel::STRING) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlFreeOemString(oemstring: *mut super::Kernel::STRING);
        }
        ::std::mem::transmute(RtlFreeOemString(::std::mem::transmute(oemstring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlFreeUnicodeString(unicodestring: *mut super::super::Foundation::UNICODE_STRING) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlFreeUnicodeString(unicodestring: *mut super::super::Foundation::UNICODE_STRING);
        }
        ::std::mem::transmute(RtlFreeUnicodeString(::std::mem::transmute(unicodestring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlInitAnsiString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInitAnsiString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8);
        }
        ::std::mem::transmute(RtlInitAnsiString(::std::mem::transmute(destinationstring), ::std::mem::transmute(sourcestring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlInitAnsiStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInitAnsiStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> super::super::Foundation::NTSTATUS;
        }
        RtlInitAnsiStringEx(::std::mem::transmute(destinationstring), ::std::mem::transmute(sourcestring)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlInitString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInitString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8);
        }
        ::std::mem::transmute(RtlInitString(::std::mem::transmute(destinationstring), ::std::mem::transmute(sourcestring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlInitStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInitStringEx(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut i8) -> super::super::Foundation::NTSTATUS;
        }
        RtlInitStringEx(::std::mem::transmute(destinationstring), ::std::mem::transmute(sourcestring)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlInitUnicodeString<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlInitUnicodeString(destinationstring: *mut super::super::Foundation::UNICODE_STRING, sourcestring: super::super::Foundation::PWSTR);
        }
        ::std::mem::transmute(RtlInitUnicodeString(::std::mem::transmute(destinationstring), sourcestring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlIsNameLegalDOS8Dot3(name: *mut super::super::Foundation::UNICODE_STRING, oemname: *mut super::Kernel::STRING, namecontainsspaces: *mut super::super::Foundation::BOOLEAN) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlIsNameLegalDOS8Dot3(name: *mut super::super::Foundation::UNICODE_STRING, oemname: *mut super::Kernel::STRING, namecontainsspaces: *mut super::super::Foundation::BOOLEAN) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(RtlIsNameLegalDOS8Dot3(::std::mem::transmute(name), ::std::mem::transmute(oemname), ::std::mem::transmute(namecontainsspaces)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlLocalTimeToSystemTime(localtime: *mut i64, systemtime: *mut i64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlLocalTimeToSystemTime(localtime: *mut i64, systemtime: *mut i64) -> super::super::Foundation::NTSTATUS;
        }
        RtlLocalTimeToSystemTime(::std::mem::transmute(localtime), ::std::mem::transmute(systemtime)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlTimeToSecondsSince1970(time: *mut i64, elapsedseconds: *mut u32) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlTimeToSecondsSince1970(time: *mut i64, elapsedseconds: *mut u32) -> super::super::Foundation::BOOLEAN;
        }
        ::std::mem::transmute(RtlTimeToSecondsSince1970(::std::mem::transmute(time), ::std::mem::transmute(elapsedseconds)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlUnicodeStringToAnsiString<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlUnicodeStringToAnsiString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: super::super::Foundation::BOOLEAN) -> super::super::Foundation::NTSTATUS;
        }
        RtlUnicodeStringToAnsiString(::std::mem::transmute(destinationstring), ::std::mem::transmute(sourcestring), allocatedestinationstring.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn RtlUnicodeStringToOemString<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOLEAN>>(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlUnicodeStringToOemString(destinationstring: *mut super::Kernel::STRING, sourcestring: *mut super::super::Foundation::UNICODE_STRING, allocatedestinationstring: super::super::Foundation::BOOLEAN) -> super::super::Foundation::NTSTATUS;
        }
        RtlUnicodeStringToOemString(::std::mem::transmute(destinationstring), ::std::mem::transmute(sourcestring), allocatedestinationstring.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtlUnicodeToMultiByteSize<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(bytesinmultibytestring: *mut u32, unicodestring: Param1, bytesinunicodestring: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlUnicodeToMultiByteSize(bytesinmultibytestring: *mut u32, unicodestring: super::super::Foundation::PWSTR, bytesinunicodestring: u32) -> super::super::Foundation::NTSTATUS;
        }
        RtlUnicodeToMultiByteSize(::std::mem::transmute(bytesinmultibytestring), unicodestring.into_param().abi(), ::std::mem::transmute(bytesinunicodestring)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtlUniform(seed: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtlUniform(seed: *mut u32) -> u32;
        }
        ::std::mem::transmute(RtlUniform(::std::mem::transmute(seed)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RunSetupCommandA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(
    hwnd: Param0,
    szcmdname: Param1,
    szinfsection: Param2,
    szdir: Param3,
    lpsztitle: Param4,
    phexe: *mut super::super::Foundation::HANDLE,
    dwflags: u32,
    pvreserved: *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RunSetupCommandA(hwnd: super::super::Foundation::HWND, szcmdname: super::super::Foundation::PSTR, szinfsection: super::super::Foundation::PSTR, szdir: super::super::Foundation::PSTR, lpsztitle: super::super::Foundation::PSTR, phexe: *mut super::super::Foundation::HANDLE, dwflags: u32, pvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        RunSetupCommandA(hwnd.into_param().abi(), szcmdname.into_param().abi(), szinfsection.into_param().abi(), szdir.into_param().abi(), lpsztitle.into_param().abi(), ::std::mem::transmute(phexe), ::std::mem::transmute(dwflags), ::std::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RunSetupCommandW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    hwnd: Param0,
    szcmdname: Param1,
    szinfsection: Param2,
    szdir: Param3,
    lpsztitle: Param4,
    phexe: *mut super::super::Foundation::HANDLE,
    dwflags: u32,
    pvreserved: *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RunSetupCommandW(hwnd: super::super::Foundation::HWND, szcmdname: super::super::Foundation::PWSTR, szinfsection: super::super::Foundation::PWSTR, szdir: super::super::Foundation::PWSTR, lpsztitle: super::super::Foundation::PWSTR, phexe: *mut super::super::Foundation::HANDLE, dwflags: u32, pvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        RunSetupCommandW(hwnd.into_param().abi(), szcmdname.into_param().abi(), szinfsection.into_param().abi(), szdir.into_param().abi(), lpsztitle.into_param().abi(), ::std::mem::transmute(phexe), ::std::mem::transmute(dwflags), ::std::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SCS_32BIT_BINARY: u32 = 0u32;
pub const SCS_64BIT_BINARY: u32 = 6u32;
pub const SCS_DOS_BINARY: u32 = 1u32;
pub const SCS_OS216_BINARY: u32 = 5u32;
pub const SCS_PIF_BINARY: u32 = 3u32;
pub const SCS_POSIX_BINARY: u32 = 4u32;
pub const SCS_WOW_BINARY: u32 = 2u32;
pub const SHUTDOWN_NORETRY: u32 = 1u32;
pub const SPACEPARITY: u32 = 4u32;
pub const STARTF_HOLOGRAPHIC: u32 = 262144u32;
pub const STORAGE_INFO_FLAGS_ALIGNED_DEVICE: u32 = 1u32;
pub const STORAGE_INFO_FLAGS_PARTITION_ALIGNED_ON_DEVICE: u32 = 2u32;
pub const STORAGE_INFO_OFFSET_UNKNOWN: u32 = 4294967295u32;
pub const STREAM_CONTAINS_GHOSTED_FILE_EXTENTS: u32 = 16u32;
pub const STREAM_CONTAINS_PROPERTIES: u32 = 4u32;
pub const STREAM_CONTAINS_SECURITY: u32 = 2u32;
pub const STREAM_MODIFIED_WHEN_READ: u32 = 1u32;
pub const STREAM_NORMAL_ATTRIBUTE: u32 = 0u32;
pub const STREAM_SPARSE_ATTRIBUTE: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STRENTRYA {
    pub pszName: super::super::Foundation::PSTR,
    pub pszValue: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl STRENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STRENTRYA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STRENTRYA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STRENTRYA").field("pszName", &self.pszName).field("pszValue", &self.pszValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STRENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.pszValue == other.pszValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STRENTRYA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STRENTRYA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STRENTRYW {
    pub pszName: super::super::Foundation::PWSTR,
    pub pszValue: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl STRENTRYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STRENTRYW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STRENTRYW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STRENTRYW").field("pszName", &self.pszName).field("pszValue", &self.pszValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STRENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.pszName == other.pszName && self.pszValue == other.pszValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STRENTRYW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STRENTRYW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STRINGEXSTRUCT {
    pub dwSize: u32,
    pub uDeterminePos: u32,
    pub uDetermineDelimPos: u32,
    pub uYomiPos: u32,
    pub uYomiDelimPos: u32,
}
impl STRINGEXSTRUCT {}
impl ::std::default::Default for STRINGEXSTRUCT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STRINGEXSTRUCT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STRINGEXSTRUCT").field("dwSize", &self.dwSize).field("uDeterminePos", &self.uDeterminePos).field("uDetermineDelimPos", &self.uDetermineDelimPos).field("uYomiPos", &self.uYomiPos).field("uYomiDelimPos", &self.uYomiDelimPos).finish()
    }
}
impl ::std::cmp::PartialEq for STRINGEXSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.uDeterminePos == other.uDeterminePos && self.uDetermineDelimPos == other.uDetermineDelimPos && self.uYomiPos == other.uYomiPos && self.uYomiDelimPos == other.uYomiDelimPos
    }
}
impl ::std::cmp::Eq for STRINGEXSTRUCT {}
unsafe impl ::windows::runtime::Abi for STRINGEXSTRUCT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STRTABLEA {
    pub cEntries: u32,
    pub pse: *mut STRENTRYA,
}
#[cfg(feature = "Win32_Foundation")]
impl STRTABLEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STRTABLEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STRTABLEA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STRTABLEA").field("cEntries", &self.cEntries).field("pse", &self.pse).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STRTABLEA {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pse == other.pse
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STRTABLEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STRTABLEA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STRTABLEW {
    pub cEntries: u32,
    pub pse: *mut STRENTRYW,
}
#[cfg(feature = "Win32_Foundation")]
impl STRTABLEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STRTABLEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STRTABLEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STRTABLEW").field("cEntries", &self.cEntries).field("pse", &self.pse).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STRTABLEW {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.pse == other.pse
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STRTABLEW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STRTABLEW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_BASIC_INFORMATION {
    pub Reserved1: [u8; 24],
    pub Reserved2: [*mut ::std::ffi::c_void; 4],
    pub NumberOfProcessors: i8,
}
impl SYSTEM_BASIC_INFORMATION {}
impl ::std::default::Default for SYSTEM_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_BASIC_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_BASIC_INFORMATION").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("NumberOfProcessors", &self.NumberOfProcessors).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.NumberOfProcessors == other.NumberOfProcessors
    }
}
impl ::std::cmp::Eq for SYSTEM_BASIC_INFORMATION {}
unsafe impl ::windows::runtime::Abi for SYSTEM_BASIC_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_CODEINTEGRITY_INFORMATION {
    pub Length: u32,
    pub CodeIntegrityOptions: u32,
}
impl SYSTEM_CODEINTEGRITY_INFORMATION {}
impl ::std::default::Default for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_CODEINTEGRITY_INFORMATION").field("Length", &self.Length).field("CodeIntegrityOptions", &self.CodeIntegrityOptions).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_CODEINTEGRITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.CodeIntegrityOptions == other.CodeIntegrityOptions
    }
}
impl ::std::cmp::Eq for SYSTEM_CODEINTEGRITY_INFORMATION {}
unsafe impl ::windows::runtime::Abi for SYSTEM_CODEINTEGRITY_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_EXCEPTION_INFORMATION {
    pub Reserved1: [u8; 16],
}
impl SYSTEM_EXCEPTION_INFORMATION {}
impl ::std::default::Default for SYSTEM_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_EXCEPTION_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_EXCEPTION_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_EXCEPTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::std::cmp::Eq for SYSTEM_EXCEPTION_INFORMATION {}
unsafe impl ::windows::runtime::Abi for SYSTEM_EXCEPTION_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SYSTEM_INFORMATION_CLASS(pub i32);
pub const SystemBasicInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(0i32);
pub const SystemPerformanceInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(2i32);
pub const SystemTimeOfDayInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(3i32);
pub const SystemProcessInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(5i32);
pub const SystemProcessorPerformanceInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(8i32);
pub const SystemInterruptInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(23i32);
pub const SystemExceptionInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(33i32);
pub const SystemRegistryQuotaInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(37i32);
pub const SystemLookasideInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(45i32);
pub const SystemCodeIntegrityInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(103i32);
pub const SystemPolicyInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(134i32);
impl ::std::convert::From<i32> for SYSTEM_INFORMATION_CLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYSTEM_INFORMATION_CLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_INTERRUPT_INFORMATION {
    pub Reserved1: [u8; 24],
}
impl SYSTEM_INTERRUPT_INFORMATION {}
impl ::std::default::Default for SYSTEM_INTERRUPT_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_INTERRUPT_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_INTERRUPT_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_INTERRUPT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::std::cmp::Eq for SYSTEM_INTERRUPT_INFORMATION {}
unsafe impl ::windows::runtime::Abi for SYSTEM_INTERRUPT_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_LOOKASIDE_INFORMATION {
    pub Reserved1: [u8; 32],
}
impl SYSTEM_LOOKASIDE_INFORMATION {}
impl ::std::default::Default for SYSTEM_LOOKASIDE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_LOOKASIDE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_LOOKASIDE_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_LOOKASIDE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::std::cmp::Eq for SYSTEM_LOOKASIDE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for SYSTEM_LOOKASIDE_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_PERFORMANCE_INFORMATION {
    pub Reserved1: [u8; 312],
}
impl SYSTEM_PERFORMANCE_INFORMATION {}
impl ::std::default::Default for SYSTEM_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_PERFORMANCE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_PERFORMANCE_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_PERFORMANCE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::std::cmp::Eq for SYSTEM_PERFORMANCE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for SYSTEM_PERFORMANCE_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_POLICY_INFORMATION {
    pub Reserved1: [*mut ::std::ffi::c_void; 2],
    pub Reserved2: [u32; 3],
}
impl SYSTEM_POLICY_INFORMATION {}
impl ::std::default::Default for SYSTEM_POLICY_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_POLICY_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_POLICY_INFORMATION").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_POLICY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::std::cmp::Eq for SYSTEM_POLICY_INFORMATION {}
unsafe impl ::windows::runtime::Abi for SYSTEM_POLICY_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    pub IdleTime: i64,
    pub KernelTime: i64,
    pub UserTime: i64,
    pub Reserved1: [i64; 2],
    pub Reserved2: u32,
}
impl SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {}
impl ::std::default::Default for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION").field("IdleTime", &self.IdleTime).field("KernelTime", &self.KernelTime).field("UserTime", &self.UserTime).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.IdleTime == other.IdleTime && self.KernelTime == other.KernelTime && self.UserTime == other.UserTime && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::std::cmp::Eq for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {}
unsafe impl ::windows::runtime::Abi for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SYSTEM_PROCESS_INFORMATION {
    pub NextEntryOffset: u32,
    pub NumberOfThreads: u32,
    pub Reserved1: [u8; 48],
    pub ImageName: super::super::Foundation::UNICODE_STRING,
    pub BasePriority: i32,
    pub UniqueProcessId: super::super::Foundation::HANDLE,
    pub Reserved2: *mut ::std::ffi::c_void,
    pub HandleCount: u32,
    pub SessionId: u32,
    pub Reserved3: *mut ::std::ffi::c_void,
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub Reserved4: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub Reserved5: *mut ::std::ffi::c_void,
    pub QuotaPagedPoolUsage: usize,
    pub Reserved6: *mut ::std::ffi::c_void,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivatePageCount: usize,
    pub Reserved7: [i64; 6],
}
#[cfg(feature = "Win32_Foundation")]
impl SYSTEM_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SYSTEM_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SYSTEM_PROCESS_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_PROCESS_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("NumberOfThreads", &self.NumberOfThreads)
            .field("Reserved1", &self.Reserved1)
            .field("ImageName", &self.ImageName)
            .field("BasePriority", &self.BasePriority)
            .field("UniqueProcessId", &self.UniqueProcessId)
            .field("Reserved2", &self.Reserved2)
            .field("HandleCount", &self.HandleCount)
            .field("SessionId", &self.SessionId)
            .field("Reserved3", &self.Reserved3)
            .field("PeakVirtualSize", &self.PeakVirtualSize)
            .field("VirtualSize", &self.VirtualSize)
            .field("Reserved4", &self.Reserved4)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("Reserved5", &self.Reserved5)
            .field("QuotaPagedPoolUsage", &self.QuotaPagedPoolUsage)
            .field("Reserved6", &self.Reserved6)
            .field("QuotaNonPagedPoolUsage", &self.QuotaNonPagedPoolUsage)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("PrivatePageCount", &self.PrivatePageCount)
            .field("Reserved7", &self.Reserved7)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SYSTEM_PROCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset
            && self.NumberOfThreads == other.NumberOfThreads
            && self.Reserved1 == other.Reserved1
            && self.ImageName == other.ImageName
            && self.BasePriority == other.BasePriority
            && self.UniqueProcessId == other.UniqueProcessId
            && self.Reserved2 == other.Reserved2
            && self.HandleCount == other.HandleCount
            && self.SessionId == other.SessionId
            && self.Reserved3 == other.Reserved3
            && self.PeakVirtualSize == other.PeakVirtualSize
            && self.VirtualSize == other.VirtualSize
            && self.Reserved4 == other.Reserved4
            && self.PeakWorkingSetSize == other.PeakWorkingSetSize
            && self.WorkingSetSize == other.WorkingSetSize
            && self.Reserved5 == other.Reserved5
            && self.QuotaPagedPoolUsage == other.QuotaPagedPoolUsage
            && self.Reserved6 == other.Reserved6
            && self.QuotaNonPagedPoolUsage == other.QuotaNonPagedPoolUsage
            && self.PagefileUsage == other.PagefileUsage
            && self.PeakPagefileUsage == other.PeakPagefileUsage
            && self.PrivatePageCount == other.PrivatePageCount
            && self.Reserved7 == other.Reserved7
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SYSTEM_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SYSTEM_PROCESS_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_REGISTRY_QUOTA_INFORMATION {
    pub RegistryQuotaAllowed: u32,
    pub RegistryQuotaUsed: u32,
    pub Reserved1: *mut ::std::ffi::c_void,
}
impl SYSTEM_REGISTRY_QUOTA_INFORMATION {}
impl ::std::default::Default for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_REGISTRY_QUOTA_INFORMATION").field("RegistryQuotaAllowed", &self.RegistryQuotaAllowed).field("RegistryQuotaUsed", &self.RegistryQuotaUsed).field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.RegistryQuotaAllowed == other.RegistryQuotaAllowed && self.RegistryQuotaUsed == other.RegistryQuotaUsed && self.Reserved1 == other.Reserved1
    }
}
impl ::std::cmp::Eq for SYSTEM_REGISTRY_QUOTA_INFORMATION {}
unsafe impl ::windows::runtime::Abi for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const SYSTEM_STATUS_FLAG_POWER_SAVING_ON: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SYSTEM_THREAD_INFORMATION {
    pub Reserved1: [i64; 3],
    pub Reserved2: u32,
    pub StartAddress: *mut ::std::ffi::c_void,
    pub ClientId: CLIENT_ID,
    pub Priority: i32,
    pub BasePriority: i32,
    pub Reserved3: u32,
    pub ThreadState: u32,
    pub WaitReason: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl SYSTEM_THREAD_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SYSTEM_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SYSTEM_THREAD_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_THREAD_INFORMATION")
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .field("StartAddress", &self.StartAddress)
            .field("ClientId", &self.ClientId)
            .field("Priority", &self.Priority)
            .field("BasePriority", &self.BasePriority)
            .field("Reserved3", &self.Reserved3)
            .field("ThreadState", &self.ThreadState)
            .field("WaitReason", &self.WaitReason)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SYSTEM_THREAD_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.StartAddress == other.StartAddress && self.ClientId == other.ClientId && self.Priority == other.Priority && self.BasePriority == other.BasePriority && self.Reserved3 == other.Reserved3 && self.ThreadState == other.ThreadState && self.WaitReason == other.WaitReason
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SYSTEM_THREAD_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SYSTEM_THREAD_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SYSTEM_TIMEOFDAY_INFORMATION {
    pub Reserved1: [u8; 48],
}
impl SYSTEM_TIMEOFDAY_INFORMATION {}
impl ::std::default::Default for SYSTEM_TIMEOFDAY_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_TIMEOFDAY_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_TIMEOFDAY_INFORMATION").field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_TIMEOFDAY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1
    }
}
impl ::std::cmp::Eq for SYSTEM_TIMEOFDAY_INFORMATION {}
unsafe impl ::windows::runtime::Abi for SYSTEM_TIMEOFDAY_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const S_ALLTHRESHOLD: u32 = 2u32;
pub const S_LEGATO: u32 = 1u32;
pub const S_NORMAL: u32 = 0u32;
pub const S_PERIOD1024: u32 = 1u32;
pub const S_PERIOD2048: u32 = 2u32;
pub const S_PERIOD512: u32 = 0u32;
pub const S_PERIODVOICE: u32 = 3u32;
pub const S_QUEUEEMPTY: u32 = 0u32;
pub const S_SERBDNT: i32 = -5i32;
pub const S_SERDCC: i32 = -7i32;
pub const S_SERDDR: i32 = -14i32;
pub const S_SERDFQ: i32 = -13i32;
pub const S_SERDLN: i32 = -6i32;
pub const S_SERDMD: i32 = -10i32;
pub const S_SERDPT: i32 = -12i32;
pub const S_SERDSH: i32 = -11i32;
pub const S_SERDSR: i32 = -15i32;
pub const S_SERDST: i32 = -16i32;
pub const S_SERDTP: i32 = -8i32;
pub const S_SERDVL: i32 = -9i32;
pub const S_SERDVNA: i32 = -1i32;
pub const S_SERMACT: i32 = -3i32;
pub const S_SEROFM: i32 = -2i32;
pub const S_SERQFUL: i32 = -4i32;
pub const S_STACCATO: u32 = 2u32;
pub const S_THRESHOLD: u32 = 1u32;
pub const S_WHITE1024: u32 = 5u32;
pub const S_WHITE2048: u32 = 6u32;
pub const S_WHITE512: u32 = 4u32;
pub const S_WHITEVOICE: u32 = 7u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SecureLockIconConstants(pub i32);
pub const secureLockIconUnsecure: SecureLockIconConstants = SecureLockIconConstants(0i32);
pub const secureLockIconMixed: SecureLockIconConstants = SecureLockIconConstants(1i32);
pub const secureLockIconSecureUnknownBits: SecureLockIconConstants = SecureLockIconConstants(2i32);
pub const secureLockIconSecure40Bit: SecureLockIconConstants = SecureLockIconConstants(3i32);
pub const secureLockIconSecure56Bit: SecureLockIconConstants = SecureLockIconConstants(4i32);
pub const secureLockIconSecureFortezza: SecureLockIconConstants = SecureLockIconConstants(5i32);
pub const secureLockIconSecure128Bit: SecureLockIconConstants = SecureLockIconConstants(6i32);
impl ::std::convert::From<i32> for SecureLockIconConstants {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SecureLockIconConstants {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendIMEMessageExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(param0: Param0, param1: Param1) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SendIMEMessageExA(param0: super::super::Foundation::HWND, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
        }
        ::std::mem::transmute(SendIMEMessageExA(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SendIMEMessageExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(param0: Param0, param1: Param1) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SendIMEMessageExW(param0: super::super::Foundation::HWND, param1: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
        }
        ::std::mem::transmute(SendIMEMessageExW(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEnvironmentStringsA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(newenvironment: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEnvironmentStringsA(newenvironment: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetEnvironmentStringsA(newenvironment.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpname: Param0, lpguid: Param1, pvalue: *const ::std::ffi::c_void, nsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFirmwareEnvironmentVariableA(lpname: super::super::Foundation::PSTR, lpguid: super::super::Foundation::PSTR, pvalue: *const ::std::ffi::c_void, nsize: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFirmwareEnvironmentVariableA(lpname.into_param().abi(), lpguid.into_param().abi(), ::std::mem::transmute(pvalue), ::std::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpname: Param0, lpguid: Param1, pvalue: *const ::std::ffi::c_void, nsize: u32, dwattributes: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFirmwareEnvironmentVariableExA(lpname: super::super::Foundation::PSTR, lpguid: super::super::Foundation::PSTR, pvalue: *const ::std::ffi::c_void, nsize: u32, dwattributes: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFirmwareEnvironmentVariableExA(lpname.into_param().abi(), lpguid.into_param().abi(), ::std::mem::transmute(pvalue), ::std::mem::transmute(nsize), ::std::mem::transmute(dwattributes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpname: Param0, lpguid: Param1, pvalue: *const ::std::ffi::c_void, nsize: u32, dwattributes: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFirmwareEnvironmentVariableExW(lpname: super::super::Foundation::PWSTR, lpguid: super::super::Foundation::PWSTR, pvalue: *const ::std::ffi::c_void, nsize: u32, dwattributes: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFirmwareEnvironmentVariableExW(lpname.into_param().abi(), lpguid.into_param().abi(), ::std::mem::transmute(pvalue), ::std::mem::transmute(nsize), ::std::mem::transmute(dwattributes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpname: Param0, lpguid: Param1, pvalue: *const ::std::ffi::c_void, nsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFirmwareEnvironmentVariableW(lpname: super::super::Foundation::PWSTR, lpguid: super::super::Foundation::PWSTR, pvalue: *const ::std::ffi::c_void, nsize: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetFirmwareEnvironmentVariableW(lpname.into_param().abi(), lpguid.into_param().abi(), ::std::mem::transmute(pvalue), ::std::mem::transmute(nsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SetHandleCount(unumber: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetHandleCount(unumber: u32) -> u32;
        }
        ::std::mem::transmute(SetHandleCount(::std::mem::transmute(unumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetMessageWaitingIndicator<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmsgindicator: Param0, ulmsgcount: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetMessageWaitingIndicator(hmsgindicator: super::super::Foundation::HANDLE, ulmsgcount: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetMessageWaitingIndicator(hmsgindicator.into_param().abi(), ::std::mem::transmute(ulmsgcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPerUserSecValuesA(pperuser: *mut PERUSERSECTIONA) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPerUserSecValuesA(pperuser: *mut PERUSERSECTIONA) -> ::windows::runtime::HRESULT;
        }
        SetPerUserSecValuesA(::std::mem::transmute(pperuser)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPerUserSecValuesW(pperuser: *mut PERUSERSECTIONW) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPerUserSecValuesW(pperuser: *mut PERUSERSECTIONW) -> ::windows::runtime::HRESULT;
        }
        SetPerUserSecValuesW(::std::mem::transmute(pperuser)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn SetTimerQueueTimer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(timerqueue: Param0, callback: ::std::option::Option<super::SystemServices::WAITORTIMERCALLBACK>, parameter: *const ::std::ffi::c_void, duetime: u32, period: u32, preferio: Param5) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTimerQueueTimer(timerqueue: super::super::Foundation::HANDLE, callback: ::windows::runtime::RawPtr, parameter: *const ::std::ffi::c_void, duetime: u32, period: u32, preferio: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(SetTimerQueueTimer(timerqueue.into_param().abi(), ::std::mem::transmute(callback), ::std::mem::transmute(parameter), ::std::mem::transmute(duetime), ::std::mem::transmute(period), preferio.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const ShellBrowserWindow: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3230334352, 62113, 4561, [132, 85, 0, 160, 201, 31, 56, 128]);
pub const ShellNameSpace: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1427335173, 45790, 4561, [185, 242, 0, 160, 201, 139, 197, 71]);
pub const ShellUIHelper: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1688947639, 4382, 4561, [143, 121, 0, 192, 79, 194, 251, 225]);
pub const ShellWindows: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2610977138, 63144, 4559, [164, 66, 0, 160, 201, 10, 143, 57]);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SignalObjectAndWait<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hobjecttosignal: Param0, hobjecttowaiton: Param1, dwmilliseconds: u32, balertable: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SignalObjectAndWait(hobjecttosignal: super::super::Foundation::HANDLE, hobjecttowaiton: super::super::Foundation::HANDLE, dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(SignalObjectAndWait(hobjecttosignal.into_param().abi(), hobjecttowaiton.into_param().abi(), ::std::mem::transmute(dwmilliseconds), balertable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SubscribeFeatureStateChangeNotification(subscription: *mut FEATURE_STATE_CHANGE_SUBSCRIPTION, callback: ::std::option::Option<PFEATURE_STATE_CHANGE_CALLBACK>, context: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SubscribeFeatureStateChangeNotification(subscription: *mut FEATURE_STATE_CHANGE_SUBSCRIPTION, callback: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(SubscribeFeatureStateChangeNotification(::std::mem::transmute(subscription), ::std::mem::transmute(callback), ::std::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const TC_GP_TRAP: u32 = 2u32;
pub const TC_HARDERR: u32 = 1u32;
pub const TC_NORMAL: u32 = 0u32;
pub const TC_SIGNAL: u32 = 3u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TDIENTITY_ENTITY_TYPE(pub u32);
pub const GENERIC_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(0u32);
pub const AT_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(640u32);
pub const CL_NL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(769u32);
pub const CO_NL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(768u32);
pub const CL_TL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(1025u32);
pub const CO_TL_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(1024u32);
pub const ER_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(896u32);
pub const IF_ENTITY: TDIENTITY_ENTITY_TYPE = TDIENTITY_ENTITY_TYPE(512u32);
impl ::std::convert::From<u32> for TDIENTITY_ENTITY_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TDIENTITY_ENTITY_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for TDIENTITY_ENTITY_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TDIENTITY_ENTITY_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TDIENTITY_ENTITY_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TDIENTITY_ENTITY_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TDIENTITY_ENTITY_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TDIEntityID {
    pub tei_entity: TDIENTITY_ENTITY_TYPE,
    pub tei_instance: u32,
}
impl TDIEntityID {}
impl ::std::default::Default for TDIEntityID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TDIEntityID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TDIEntityID").field("tei_entity", &self.tei_entity).field("tei_instance", &self.tei_instance).finish()
    }
}
impl ::std::cmp::PartialEq for TDIEntityID {
    fn eq(&self, other: &Self) -> bool {
        self.tei_entity == other.tei_entity && self.tei_instance == other.tei_instance
    }
}
impl ::std::cmp::Eq for TDIEntityID {}
unsafe impl ::windows::runtime::Abi for TDIEntityID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TDIObjectID {
    pub toi_entity: TDIEntityID,
    pub toi_class: u32,
    pub toi_type: u32,
    pub toi_id: u32,
}
impl TDIObjectID {}
impl ::std::default::Default for TDIObjectID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TDIObjectID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TDIObjectID").field("toi_entity", &self.toi_entity).field("toi_class", &self.toi_class).field("toi_type", &self.toi_type).field("toi_id", &self.toi_id).finish()
    }
}
impl ::std::cmp::PartialEq for TDIObjectID {
    fn eq(&self, other: &Self) -> bool {
        self.toi_entity == other.toi_entity && self.toi_class == other.toi_class && self.toi_type == other.toi_type && self.toi_id == other.toi_id
    }
}
impl ::std::cmp::Eq for TDIObjectID {}
unsafe impl ::windows::runtime::Abi for TDIObjectID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TDI_TL_IO_CONTROL_ENDPOINT {
    pub Type: TDI_TL_IO_CONTROL_TYPE,
    pub Level: u32,
    pub Anonymous: TDI_TL_IO_CONTROL_ENDPOINT_0,
    pub InputBuffer: *mut ::std::ffi::c_void,
    pub InputBufferLength: u32,
    pub OutputBuffer: *mut ::std::ffi::c_void,
    pub OutputBufferLength: u32,
}
impl TDI_TL_IO_CONTROL_ENDPOINT {}
impl ::std::default::Default for TDI_TL_IO_CONTROL_ENDPOINT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TDI_TL_IO_CONTROL_ENDPOINT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TDI_TL_IO_CONTROL_ENDPOINT {}
unsafe impl ::windows::runtime::Abi for TDI_TL_IO_CONTROL_ENDPOINT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union TDI_TL_IO_CONTROL_ENDPOINT_0 {
    pub IoControlCode: u32,
    pub OptionName: u32,
}
impl TDI_TL_IO_CONTROL_ENDPOINT_0 {}
impl ::std::default::Default for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TDI_TL_IO_CONTROL_ENDPOINT_0 {}
unsafe impl ::windows::runtime::Abi for TDI_TL_IO_CONTROL_ENDPOINT_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TDI_TL_IO_CONTROL_TYPE(pub i32);
pub const EndpointIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(0i32);
pub const SetSockOptIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(1i32);
pub const GetSockOptIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(2i32);
pub const SocketIoControlType: TDI_TL_IO_CONTROL_TYPE = TDI_TL_IO_CONTROL_TYPE(3i32);
impl ::std::convert::From<i32> for TDI_TL_IO_CONTROL_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TDI_TL_IO_CONTROL_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct THREAD_NAME_INFORMATION {
    pub ThreadName: super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl THREAD_NAME_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for THREAD_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for THREAD_NAME_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("THREAD_NAME_INFORMATION").field("ThreadName", &self.ThreadName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for THREAD_NAME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadName == other.ThreadName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for THREAD_NAME_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for THREAD_NAME_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const THREAD_PRIORITY_ERROR_RETURN: u32 = 2147483647u32;
pub const TWOSTOPBITS: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TranslateInfStringA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(
    pszinffilename: Param0,
    pszinstallsection: Param1,
    psztranslatesection: Param2,
    psztranslatekey: Param3,
    pszbuffer: super::super::Foundation::PSTR,
    cchbuffer: u32,
    pdwrequiredsize: *mut u32,
    pvreserved: *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TranslateInfStringA(pszinffilename: super::super::Foundation::PSTR, pszinstallsection: super::super::Foundation::PSTR, psztranslatesection: super::super::Foundation::PSTR, psztranslatekey: super::super::Foundation::PSTR, pszbuffer: super::super::Foundation::PSTR, cchbuffer: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        TranslateInfStringA(pszinffilename.into_param().abi(), pszinstallsection.into_param().abi(), psztranslatesection.into_param().abi(), psztranslatekey.into_param().abi(), ::std::mem::transmute(pszbuffer), ::std::mem::transmute(cchbuffer), ::std::mem::transmute(pdwrequiredsize), ::std::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TranslateInfStringExA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(
    hinf: *mut ::std::ffi::c_void,
    pszinffilename: Param1,
    psztranslatesection: Param2,
    psztranslatekey: Param3,
    pszbuffer: super::super::Foundation::PSTR,
    dwbuffersize: u32,
    pdwrequiredsize: *mut u32,
    pvreserved: *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TranslateInfStringExA(hinf: *mut ::std::ffi::c_void, pszinffilename: super::super::Foundation::PSTR, psztranslatesection: super::super::Foundation::PSTR, psztranslatekey: super::super::Foundation::PSTR, pszbuffer: super::super::Foundation::PSTR, dwbuffersize: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        TranslateInfStringExA(::std::mem::transmute(hinf), pszinffilename.into_param().abi(), psztranslatesection.into_param().abi(), psztranslatekey.into_param().abi(), ::std::mem::transmute(pszbuffer), ::std::mem::transmute(dwbuffersize), ::std::mem::transmute(pdwrequiredsize), ::std::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TranslateInfStringExW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    hinf: *mut ::std::ffi::c_void,
    pszinffilename: Param1,
    psztranslatesection: Param2,
    psztranslatekey: Param3,
    pszbuffer: super::super::Foundation::PWSTR,
    dwbuffersize: u32,
    pdwrequiredsize: *mut u32,
    pvreserved: *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TranslateInfStringExW(hinf: *mut ::std::ffi::c_void, pszinffilename: super::super::Foundation::PWSTR, psztranslatesection: super::super::Foundation::PWSTR, psztranslatekey: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, dwbuffersize: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        TranslateInfStringExW(::std::mem::transmute(hinf), pszinffilename.into_param().abi(), psztranslatesection.into_param().abi(), psztranslatekey.into_param().abi(), ::std::mem::transmute(pszbuffer), ::std::mem::transmute(dwbuffersize), ::std::mem::transmute(pdwrequiredsize), ::std::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TranslateInfStringW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    pszinffilename: Param0,
    pszinstallsection: Param1,
    psztranslatesection: Param2,
    psztranslatekey: Param3,
    pszbuffer: super::super::Foundation::PWSTR,
    cchbuffer: u32,
    pdwrequiredsize: *mut u32,
    pvreserved: *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TranslateInfStringW(pszinffilename: super::super::Foundation::PWSTR, pszinstallsection: super::super::Foundation::PWSTR, psztranslatesection: super::super::Foundation::PWSTR, psztranslatekey: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, cchbuffer: u32, pdwrequiredsize: *mut u32, pvreserved: *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        TranslateInfStringW(pszinffilename.into_param().abi(), pszinstallsection.into_param().abi(), psztranslatesection.into_param().abi(), psztranslatekey.into_param().abi(), ::std::mem::transmute(pszbuffer), ::std::mem::transmute(cchbuffer), ::std::mem::transmute(pdwrequiredsize), ::std::mem::transmute(pvreserved)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const UMS_VERSION: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct UNDETERMINESTRUCT {
    pub dwSize: u32,
    pub uDefIMESize: u32,
    pub uDefIMEPos: u32,
    pub uUndetTextLen: u32,
    pub uUndetTextPos: u32,
    pub uUndetAttrPos: u32,
    pub uCursorPos: u32,
    pub uDeltaStart: u32,
    pub uDetermineTextLen: u32,
    pub uDetermineTextPos: u32,
    pub uDetermineDelimPos: u32,
    pub uYomiTextLen: u32,
    pub uYomiTextPos: u32,
    pub uYomiDelimPos: u32,
}
impl UNDETERMINESTRUCT {}
impl ::std::default::Default for UNDETERMINESTRUCT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for UNDETERMINESTRUCT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("UNDETERMINESTRUCT")
            .field("dwSize", &self.dwSize)
            .field("uDefIMESize", &self.uDefIMESize)
            .field("uDefIMEPos", &self.uDefIMEPos)
            .field("uUndetTextLen", &self.uUndetTextLen)
            .field("uUndetTextPos", &self.uUndetTextPos)
            .field("uUndetAttrPos", &self.uUndetAttrPos)
            .field("uCursorPos", &self.uCursorPos)
            .field("uDeltaStart", &self.uDeltaStart)
            .field("uDetermineTextLen", &self.uDetermineTextLen)
            .field("uDetermineTextPos", &self.uDetermineTextPos)
            .field("uDetermineDelimPos", &self.uDetermineDelimPos)
            .field("uYomiTextLen", &self.uYomiTextLen)
            .field("uYomiTextPos", &self.uYomiTextPos)
            .field("uYomiDelimPos", &self.uYomiDelimPos)
            .finish()
    }
}
impl ::std::cmp::PartialEq for UNDETERMINESTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.uDefIMESize == other.uDefIMESize
            && self.uDefIMEPos == other.uDefIMEPos
            && self.uUndetTextLen == other.uUndetTextLen
            && self.uUndetTextPos == other.uUndetTextPos
            && self.uUndetAttrPos == other.uUndetAttrPos
            && self.uCursorPos == other.uCursorPos
            && self.uDeltaStart == other.uDeltaStart
            && self.uDetermineTextLen == other.uDetermineTextLen
            && self.uDetermineTextPos == other.uDetermineTextPos
            && self.uDetermineDelimPos == other.uDetermineDelimPos
            && self.uYomiTextLen == other.uYomiTextLen
            && self.uYomiTextPos == other.uYomiTextPos
            && self.uYomiDelimPos == other.uYomiDelimPos
    }
}
impl ::std::cmp::Eq for UNDETERMINESTRUCT {}
unsafe impl ::windows::runtime::Abi for UNDETERMINESTRUCT {
    type Abi = Self;
    type DefaultType = Self;
}
#[inline]
pub unsafe fn UnsubscribeFeatureStateChangeNotification<'a, Param0: ::windows::runtime::IntoParam<'a, FEATURE_STATE_CHANGE_SUBSCRIPTION>>(subscription: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnsubscribeFeatureStateChangeNotification(subscription: FEATURE_STATE_CHANGE_SUBSCRIPTION);
        }
        ::std::mem::transmute(UnsubscribeFeatureStateChangeNotification(subscription.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UserInstStubWrapperA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hwnd: Param0, hinstance: Param1, pszparms: Param2, nshow: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UserInstStubWrapperA(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PSTR, nshow: i32) -> ::windows::runtime::HRESULT;
        }
        UserInstStubWrapperA(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), ::std::mem::transmute(nshow)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UserInstStubWrapperW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hwnd: Param0, hinstance: Param1, pszparms: Param2, nshow: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UserInstStubWrapperW(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PWSTR, nshow: i32) -> ::windows::runtime::HRESULT;
        }
        UserInstStubWrapperW(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), ::std::mem::transmute(nshow)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UserUnInstStubWrapperA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hwnd: Param0, hinstance: Param1, pszparms: Param2, nshow: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UserUnInstStubWrapperA(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PSTR, nshow: i32) -> ::windows::runtime::HRESULT;
        }
        UserUnInstStubWrapperA(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), ::std::mem::transmute(nshow)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UserUnInstStubWrapperW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HINSTANCE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hwnd: Param0, hinstance: Param1, pszparms: Param2, nshow: i32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UserUnInstStubWrapperW(hwnd: super::super::Foundation::HWND, hinstance: super::super::Foundation::HINSTANCE, pszparms: super::super::Foundation::PWSTR, nshow: i32) -> ::windows::runtime::HRESULT;
        }
        UserUnInstStubWrapperW(hwnd.into_param().abi(), hinstance.into_param().abi(), pszparms.into_param().abi(), ::std::mem::transmute(nshow)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VALUENAME(pub i32);
pub const VALUENAME_UNKNOWN: VALUENAME = VALUENAME(0i32);
pub const VALUENAME_ENTERPRISE_DEFINED_CLASS_ID: VALUENAME = VALUENAME(1i32);
pub const VALUENAME_BUILT_IN_LIST: VALUENAME = VALUENAME(2i32);
impl ::std::convert::From<i32> for VALUENAME {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VALUENAME {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VOLUME_NAME_DOS: u32 = 0u32;
pub const VOLUME_NAME_GUID: u32 = 1u32;
pub const VOLUME_NAME_NONE: u32 = 4u32;
pub const VOLUME_NAME_NT: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WINNLSEnableIME<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(param0: Param0, param1: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WINNLSEnableIME(param0: super::super::Foundation::HWND, param1: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WINNLSEnableIME(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WINNLSGetEnableStatus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WINNLSGetEnableStatus(param0: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WINNLSGetEnableStatus(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WINNLSGetIMEHotkey<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WINNLSGetIMEHotkey(param0: super::super::Foundation::HWND) -> u32;
        }
        ::std::mem::transmute(WINNLSGetIMEHotkey(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WINSTATIONINFOCLASS(pub i32);
pub const WinStationInformation: WINSTATIONINFOCLASS = WINSTATIONINFOCLASS(8i32);
impl ::std::convert::From<i32> for WINSTATIONINFOCLASS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WINSTATIONINFOCLASS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct WINSTATIONINFORMATIONW {
    pub Reserved2: [u8; 70],
    pub LogonId: u32,
    pub Reserved3: [u8; 1140],
}
impl WINSTATIONINFORMATIONW {}
impl ::std::default::Default for WINSTATIONINFORMATIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for WINSTATIONINFORMATIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WINSTATIONINFORMATIONW").field("Reserved2", &self.Reserved2).field("LogonId", &self.LogonId).field("Reserved3", &self.Reserved3).finish()
    }
}
impl ::std::cmp::PartialEq for WINSTATIONINFORMATIONW {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved2 == other.Reserved2 && self.LogonId == other.LogonId && self.Reserved3 == other.Reserved3
    }
}
impl ::std::cmp::Eq for WINSTATIONINFORMATIONW {}
unsafe impl ::windows::runtime::Abi for WINSTATIONINFORMATIONW {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type WINWATCHNOTIFYPROC = unsafe extern "system" fn(hww: HWINWATCH, hwnd: super::super::Foundation::HWND, code: u32, lparam: super::super::Foundation::LPARAM);
pub const WINWATCHNOTIFY_CHANGED: u32 = 4u32;
pub const WINWATCHNOTIFY_CHANGING: u32 = 3u32;
pub const WINWATCHNOTIFY_DESTROY: u32 = 2u32;
pub const WINWATCHNOTIFY_START: u32 = 0u32;
pub const WINWATCHNOTIFY_STOP: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLDP_DEVICE_SECURITY_INFORMATION {
    pub UnlockIdSize: u32,
    pub UnlockId: *mut u8,
    pub ManufacturerIDLength: u32,
    pub ManufacturerID: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl WLDP_DEVICE_SECURITY_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WLDP_DEVICE_SECURITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WLDP_DEVICE_SECURITY_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLDP_DEVICE_SECURITY_INFORMATION").field("UnlockIdSize", &self.UnlockIdSize).field("UnlockId", &self.UnlockId).field("ManufacturerIDLength", &self.ManufacturerIDLength).field("ManufacturerID", &self.ManufacturerID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WLDP_DEVICE_SECURITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.UnlockIdSize == other.UnlockIdSize && self.UnlockId == other.UnlockId && self.ManufacturerIDLength == other.ManufacturerIDLength && self.ManufacturerID == other.ManufacturerID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WLDP_DEVICE_SECURITY_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WLDP_DEVICE_SECURITY_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WLDP_FLAGS_SKIPSIGNATUREVALIDATION: u32 = 256u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WLDP_HOST(pub i32);
pub const WLDP_HOST_RUNDLL32: WLDP_HOST = WLDP_HOST(0i32);
pub const WLDP_HOST_SVCHOST: WLDP_HOST = WLDP_HOST(1i32);
pub const WLDP_HOST_MAX: WLDP_HOST = WLDP_HOST(2i32);
impl ::std::convert::From<i32> for WLDP_HOST {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WLDP_HOST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WLDP_HOST_ID(pub i32);
pub const WLDP_HOST_ID_UNKNOWN: WLDP_HOST_ID = WLDP_HOST_ID(0i32);
pub const WLDP_HOST_ID_GLOBAL: WLDP_HOST_ID = WLDP_HOST_ID(1i32);
pub const WLDP_HOST_ID_VBA: WLDP_HOST_ID = WLDP_HOST_ID(2i32);
pub const WLDP_HOST_ID_WSH: WLDP_HOST_ID = WLDP_HOST_ID(3i32);
pub const WLDP_HOST_ID_POWERSHELL: WLDP_HOST_ID = WLDP_HOST_ID(4i32);
pub const WLDP_HOST_ID_IE: WLDP_HOST_ID = WLDP_HOST_ID(5i32);
pub const WLDP_HOST_ID_MSI: WLDP_HOST_ID = WLDP_HOST_ID(6i32);
pub const WLDP_HOST_ID_ALL: WLDP_HOST_ID = WLDP_HOST_ID(7i32);
pub const WLDP_HOST_ID_MAX: WLDP_HOST_ID = WLDP_HOST_ID(8i32);
impl ::std::convert::From<i32> for WLDP_HOST_ID {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WLDP_HOST_ID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLDP_HOST_INFORMATION {
    pub dwRevision: u32,
    pub dwHostId: WLDP_HOST_ID,
    pub szSource: super::super::Foundation::PWSTR,
    pub hSource: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl WLDP_HOST_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for WLDP_HOST_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for WLDP_HOST_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("WLDP_HOST_INFORMATION").field("dwRevision", &self.dwRevision).field("dwHostId", &self.dwHostId).field("szSource", &self.szSource).field("hSource", &self.hSource).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for WLDP_HOST_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.dwRevision == other.dwRevision && self.dwHostId == other.dwHostId && self.szSource == other.szSource && self.hSource == other.hSource
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for WLDP_HOST_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for WLDP_HOST_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WLDP_HOST_INFORMATION_REVISION: u32 = 1u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WLDP_KEY(pub i32);
pub const KEY_UNKNOWN: WLDP_KEY = WLDP_KEY(0i32);
pub const KEY_OVERRIDE: WLDP_KEY = WLDP_KEY(1i32);
pub const KEY_ALL_KEYS: WLDP_KEY = WLDP_KEY(2i32);
impl ::std::convert::From<i32> for WLDP_KEY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WLDP_KEY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WLDP_LOCKDOWN_AUDIT_FLAG: u32 = 8u32;
pub const WLDP_LOCKDOWN_CONFIG_CI_AUDIT_FLAG: u32 = 2u32;
pub const WLDP_LOCKDOWN_CONFIG_CI_FLAG: u32 = 1u32;
pub const WLDP_LOCKDOWN_DEFINED_FLAG: u32 = 2147483648u32;
pub const WLDP_LOCKDOWN_EXCLUSION_FLAG: u32 = 16u32;
pub const WLDP_LOCKDOWN_OFF: u32 = 2147483648u32;
pub const WLDP_LOCKDOWN_UMCIENFORCE_FLAG: u32 = 4u32;
pub const WLDP_LOCKDOWN_UNDEFINED: u32 = 0u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WLDP_POLICY_SETTING(pub i32);
pub const WLDP_POLICY_SETTING_AV_PERF_MODE: WLDP_POLICY_SETTING = WLDP_POLICY_SETTING(1000i32);
impl ::std::convert::From<i32> for WLDP_POLICY_SETTING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WLDP_POLICY_SETTING {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WLDP_WINDOWS_LOCKDOWN_MODE(pub i32);
pub const WLDP_WINDOWS_LOCKDOWN_MODE_UNLOCKED: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(0i32);
pub const WLDP_WINDOWS_LOCKDOWN_MODE_TRIAL: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(1i32);
pub const WLDP_WINDOWS_LOCKDOWN_MODE_LOCKED: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(2i32);
pub const WLDP_WINDOWS_LOCKDOWN_MODE_MAX: WLDP_WINDOWS_LOCKDOWN_MODE = WLDP_WINDOWS_LOCKDOWN_MODE(3i32);
impl ::std::convert::From<i32> for WLDP_WINDOWS_LOCKDOWN_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WLDP_WINDOWS_LOCKDOWN_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WLDP_WINDOWS_LOCKDOWN_RESTRICTION(pub i32);
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NONE: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(0i32);
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(1i32);
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_NOUNLOCK_PERMANENT: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(2i32);
pub const WLDP_WINDOWS_LOCKDOWN_RESTRICTION_MAX: WLDP_WINDOWS_LOCKDOWN_RESTRICTION = WLDP_WINDOWS_LOCKDOWN_RESTRICTION(3i32);
impl ::std::convert::From<i32> for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WLDP_WINDOWS_LOCKDOWN_RESTRICTION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WM_CONVERTREQUEST: u32 = 266u32;
pub const WM_CONVERTRESULT: u32 = 267u32;
pub const WM_IMEKEYDOWN: u32 = 656u32;
pub const WM_IMEKEYUP: u32 = 657u32;
pub const WM_IME_REPORT: u32 = 640u32;
pub const WM_INTERIM: u32 = 268u32;
pub const WM_WNT_CONVERTREQUESTEX: u32 = 265u32;
pub const WebBrowser: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2287401313, 13322, 4560, [169, 107, 0, 192, 79, 215, 5, 162]);
pub const WebBrowser_V1: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3937544899, 12481, 4559, [167, 235, 0, 0, 192, 91, 174, 11]);
#[inline]
pub unsafe fn WinWatchClose<'a, Param0: ::windows::runtime::IntoParam<'a, HWINWATCH>>(hww: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinWatchClose(hww: HWINWATCH);
        }
        ::std::mem::transmute(WinWatchClose(hww.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinWatchDidStatusChange<'a, Param0: ::windows::runtime::IntoParam<'a, HWINWATCH>>(hww: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinWatchDidStatusChange(hww: HWINWATCH) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WinWatchDidStatusChange(hww.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn WinWatchGetClipList<'a, Param0: ::windows::runtime::IntoParam<'a, HWINWATCH>>(hww: Param0, prc: *mut super::super::Foundation::RECT, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinWatchGetClipList(hww: HWINWATCH, prc: *mut super::super::Foundation::RECT, size: u32, prd: *mut super::super::Graphics::Gdi::RGNDATA) -> u32;
        }
        ::std::mem::transmute(WinWatchGetClipList(hww.into_param().abi(), ::std::mem::transmute(prc), ::std::mem::transmute(size), ::std::mem::transmute(prd)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinWatchNotify<'a, Param0: ::windows::runtime::IntoParam<'a, HWINWATCH>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::LPARAM>>(hww: Param0, notifycallback: ::std::option::Option<WINWATCHNOTIFYPROC>, notifyparam: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinWatchNotify(hww: HWINWATCH, notifycallback: ::windows::runtime::RawPtr, notifyparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WinWatchNotify(hww.into_param().abi(), ::std::mem::transmute(notifycallback), notifyparam.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WinWatchOpen<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0) -> HWINWATCH {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinWatchOpen(hwnd: super::super::Foundation::HWND) -> HWINWATCH;
        }
        ::std::mem::transmute(WinWatchOpen(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpGetLockdownPolicy(hostinformation: *const WLDP_HOST_INFORMATION, lockdownstate: *mut u32, lockdownflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WldpGetLockdownPolicy(hostinformation: *const WLDP_HOST_INFORMATION, lockdownstate: *mut u32, lockdownflags: u32) -> ::windows::runtime::HRESULT;
        }
        WldpGetLockdownPolicy(::std::mem::transmute(hostinformation), ::std::mem::transmute(lockdownstate), ::std::mem::transmute(lockdownflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpIsClassInApprovedList(classid: *const ::windows::runtime::GUID, hostinformation: *const WLDP_HOST_INFORMATION, isapproved: *mut super::super::Foundation::BOOL, optionalflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WldpIsClassInApprovedList(classid: *const ::windows::runtime::GUID, hostinformation: *const WLDP_HOST_INFORMATION, isapproved: *mut super::super::Foundation::BOOL, optionalflags: u32) -> ::windows::runtime::HRESULT;
        }
        WldpIsClassInApprovedList(::std::mem::transmute(classid), ::std::mem::transmute(hostinformation), ::std::mem::transmute(isapproved), ::std::mem::transmute(optionalflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpIsDynamicCodePolicyEnabled() -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WldpIsDynamicCodePolicyEnabled(isenabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WldpIsDynamicCodePolicyEnabled(&mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpQueryDeviceSecurityInformation(information: *mut WLDP_DEVICE_SECURITY_INFORMATION, informationlength: u32, returnlength: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WldpQueryDeviceSecurityInformation(information: *mut WLDP_DEVICE_SECURITY_INFORMATION, informationlength: u32, returnlength: *mut u32) -> ::windows::runtime::HRESULT;
        }
        WldpQueryDeviceSecurityInformation(::std::mem::transmute(information), ::std::mem::transmute(informationlength), ::std::mem::transmute(returnlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpQueryDynamicCodeTrust<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, baseimage: *const ::std::ffi::c_void, imagesize: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WldpQueryDynamicCodeTrust(filehandle: super::super::Foundation::HANDLE, baseimage: *const ::std::ffi::c_void, imagesize: u32) -> ::windows::runtime::HRESULT;
        }
        WldpQueryDynamicCodeTrust(filehandle.into_param().abi(), ::std::mem::transmute(baseimage), ::std::mem::transmute(imagesize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WldpSetDynamicCodeTrust<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WldpSetDynamicCodeTrust(filehandle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        WldpSetDynamicCodeTrust(filehandle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileSectionA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpappname: Param0, lpstring: Param1, lpfilename: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WritePrivateProfileSectionA(lpappname: super::super::Foundation::PSTR, lpstring: super::super::Foundation::PSTR, lpfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WritePrivateProfileSectionA(lpappname.into_param().abi(), lpstring.into_param().abi(), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileSectionW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpappname: Param0, lpstring: Param1, lpfilename: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WritePrivateProfileSectionW(lpappname: super::super::Foundation::PWSTR, lpstring: super::super::Foundation::PWSTR, lpfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WritePrivateProfileSectionW(lpappname.into_param().abi(), lpstring.into_param().abi(), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileStringA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpappname: Param0, lpkeyname: Param1, lpstring: Param2, lpfilename: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WritePrivateProfileStringA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, lpstring: super::super::Foundation::PSTR, lpfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WritePrivateProfileStringA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpstring.into_param().abi(), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileStringW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpappname: Param0, lpkeyname: Param1, lpstring: Param2, lpfilename: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WritePrivateProfileStringW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, lpstring: super::super::Foundation::PWSTR, lpfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WritePrivateProfileStringW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpstring.into_param().abi(), lpfilename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileStructA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpszsection: Param0, lpszkey: Param1, lpstruct: *const ::std::ffi::c_void, usizestruct: u32, szfile: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WritePrivateProfileStructA(lpszsection: super::super::Foundation::PSTR, lpszkey: super::super::Foundation::PSTR, lpstruct: *const ::std::ffi::c_void, usizestruct: u32, szfile: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WritePrivateProfileStructA(lpszsection.into_param().abi(), lpszkey.into_param().abi(), ::std::mem::transmute(lpstruct), ::std::mem::transmute(usizestruct), szfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WritePrivateProfileStructW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszsection: Param0, lpszkey: Param1, lpstruct: *const ::std::ffi::c_void, usizestruct: u32, szfile: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WritePrivateProfileStructW(lpszsection: super::super::Foundation::PWSTR, lpszkey: super::super::Foundation::PWSTR, lpstruct: *const ::std::ffi::c_void, usizestruct: u32, szfile: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WritePrivateProfileStructW(lpszsection.into_param().abi(), lpszkey.into_param().abi(), ::std::mem::transmute(lpstruct), ::std::mem::transmute(usizestruct), szfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteProfileSectionA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpappname: Param0, lpstring: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteProfileSectionA(lpappname: super::super::Foundation::PSTR, lpstring: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteProfileSectionA(lpappname.into_param().abi(), lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteProfileSectionW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpappname: Param0, lpstring: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteProfileSectionW(lpappname: super::super::Foundation::PWSTR, lpstring: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteProfileSectionW(lpappname.into_param().abi(), lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteProfileStringA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpappname: Param0, lpkeyname: Param1, lpstring: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteProfileStringA(lpappname: super::super::Foundation::PSTR, lpkeyname: super::super::Foundation::PSTR, lpstring: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteProfileStringA(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WriteProfileStringW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpappname: Param0, lpkeyname: Param1, lpstring: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WriteProfileStringW(lpappname: super::super::Foundation::PWSTR, lpkeyname: super::super::Foundation::PWSTR, lpstring: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WriteProfileStringW(lpappname.into_param().abi(), lpkeyname.into_param().abi(), lpstring.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _D3DHAL_CALLBACKS(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _D3DHAL_GLOBALDRIVERDATA(pub u8);
#[inline]
pub unsafe fn _hread(hfile: i32, lpbuffer: *mut ::std::ffi::c_void, lbytes: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _hread(hfile: i32, lpbuffer: *mut ::std::ffi::c_void, lbytes: i32) -> i32;
        }
        ::std::mem::transmute(_hread(::std::mem::transmute(hfile), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(lbytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn _hwrite<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hfile: i32, lpbuffer: Param1, lbytes: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _hwrite(hfile: i32, lpbuffer: super::super::Foundation::PSTR, lbytes: i32) -> i32;
        }
        ::std::mem::transmute(_hwrite(::std::mem::transmute(hfile), lpbuffer.into_param().abi(), ::std::mem::transmute(lbytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn _lclose(hfile: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _lclose(hfile: i32) -> i32;
        }
        ::std::mem::transmute(_lclose(::std::mem::transmute(hfile)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn _lcreat<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lppathname: Param0, iattribute: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _lcreat(lppathname: super::super::Foundation::PSTR, iattribute: i32) -> i32;
        }
        ::std::mem::transmute(_lcreat(lppathname.into_param().abi(), ::std::mem::transmute(iattribute)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn _llseek(hfile: i32, loffset: i32, iorigin: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _llseek(hfile: i32, loffset: i32, iorigin: i32) -> i32;
        }
        ::std::mem::transmute(_llseek(::std::mem::transmute(hfile), ::std::mem::transmute(loffset), ::std::mem::transmute(iorigin)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn _lopen<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lppathname: Param0, ireadwrite: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _lopen(lppathname: super::super::Foundation::PSTR, ireadwrite: i32) -> i32;
        }
        ::std::mem::transmute(_lopen(lppathname.into_param().abi(), ::std::mem::transmute(ireadwrite)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn _lread(hfile: i32, lpbuffer: *mut ::std::ffi::c_void, ubytes: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _lread(hfile: i32, lpbuffer: *mut ::std::ffi::c_void, ubytes: u32) -> u32;
        }
        ::std::mem::transmute(_lread(::std::mem::transmute(hfile), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(ubytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn _lwrite<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hfile: i32, lpbuffer: Param1, ubytes: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _lwrite(hfile: i32, lpbuffer: super::super::Foundation::PSTR, ubytes: u32) -> u32;
        }
        ::std::mem::transmute(_lwrite(::std::mem::transmute(hfile), lpbuffer.into_param().abi(), ::std::mem::transmute(ubytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct tcp_request_query_information_ex32_xp {
    pub ID: TDIObjectID,
    pub Context: [u32; 4],
}
impl tcp_request_query_information_ex32_xp {}
impl ::std::default::Default for tcp_request_query_information_ex32_xp {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for tcp_request_query_information_ex32_xp {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("tcp_request_query_information_ex32_xp").field("ID", &self.ID).field("Context", &self.Context).finish()
    }
}
impl ::std::cmp::PartialEq for tcp_request_query_information_ex32_xp {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.Context == other.Context
    }
}
impl ::std::cmp::Eq for tcp_request_query_information_ex32_xp {}
unsafe impl ::windows::runtime::Abi for tcp_request_query_information_ex32_xp {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct tcp_request_query_information_ex_w2k {
    pub ID: TDIObjectID,
    pub Context: [u8; 16],
}
impl tcp_request_query_information_ex_w2k {}
impl ::std::default::Default for tcp_request_query_information_ex_w2k {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for tcp_request_query_information_ex_w2k {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("tcp_request_query_information_ex_w2k").field("ID", &self.ID).field("Context", &self.Context).finish()
    }
}
impl ::std::cmp::PartialEq for tcp_request_query_information_ex_w2k {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.Context == other.Context
    }
}
impl ::std::cmp::Eq for tcp_request_query_information_ex_w2k {}
unsafe impl ::windows::runtime::Abi for tcp_request_query_information_ex_w2k {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct tcp_request_query_information_ex_xp {
    pub ID: TDIObjectID,
    pub Context: [usize; 2],
}
impl tcp_request_query_information_ex_xp {}
impl ::std::default::Default for tcp_request_query_information_ex_xp {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for tcp_request_query_information_ex_xp {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("tcp_request_query_information_ex_xp").field("ID", &self.ID).field("Context", &self.Context).finish()
    }
}
impl ::std::cmp::PartialEq for tcp_request_query_information_ex_xp {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.Context == other.Context
    }
}
impl ::std::cmp::Eq for tcp_request_query_information_ex_xp {}
unsafe impl ::windows::runtime::Abi for tcp_request_query_information_ex_xp {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct tcp_request_set_information_ex {
    pub ID: TDIObjectID,
    pub BufferSize: u32,
    pub Buffer: [u8; 1],
}
impl tcp_request_set_information_ex {}
impl ::std::default::Default for tcp_request_set_information_ex {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for tcp_request_set_information_ex {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("tcp_request_set_information_ex").field("ID", &self.ID).field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
impl ::std::cmp::PartialEq for tcp_request_set_information_ex {
    fn eq(&self, other: &Self) -> bool {
        self.ID == other.ID && self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
impl ::std::cmp::Eq for tcp_request_set_information_ex {}
unsafe impl ::windows::runtime::Abi for tcp_request_set_information_ex {
    type Abi = Self;
    type DefaultType = Self;
}
#[inline]
pub unsafe fn uaw_lstrcmpW(string1: *const u16, string2: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_lstrcmpW(string1: *const u16, string2: *const u16) -> i32;
        }
        ::std::mem::transmute(uaw_lstrcmpW(::std::mem::transmute(string1), ::std::mem::transmute(string2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn uaw_lstrcmpiW(string1: *const u16, string2: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_lstrcmpiW(string1: *const u16, string2: *const u16) -> i32;
        }
        ::std::mem::transmute(uaw_lstrcmpiW(::std::mem::transmute(string1), ::std::mem::transmute(string2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn uaw_lstrlenW(string: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_lstrlenW(string: *const u16) -> i32;
        }
        ::std::mem::transmute(uaw_lstrlenW(::std::mem::transmute(string)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn uaw_wcschr(string: *const u16, character: u16) -> *mut u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_wcschr(string: *const u16, character: u16) -> *mut u16;
        }
        ::std::mem::transmute(uaw_wcschr(::std::mem::transmute(string), ::std::mem::transmute(character)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn uaw_wcscpy(destination: *mut u16, source: *const u16) -> *mut u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_wcscpy(destination: *mut u16, source: *const u16) -> *mut u16;
        }
        ::std::mem::transmute(uaw_wcscpy(::std::mem::transmute(destination), ::std::mem::transmute(source)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn uaw_wcsicmp(string1: *const u16, string2: *const u16) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_wcsicmp(string1: *const u16, string2: *const u16) -> i32;
        }
        ::std::mem::transmute(uaw_wcsicmp(::std::mem::transmute(string1), ::std::mem::transmute(string2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn uaw_wcslen(string: *const u16) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_wcslen(string: *const u16) -> usize;
        }
        ::std::mem::transmute(uaw_wcslen(::std::mem::transmute(string)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn uaw_wcsrchr(string: *const u16, character: u16) -> *mut u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn uaw_wcsrchr(string: *const u16, character: u16) -> *mut u16;
        }
        ::std::mem::transmute(uaw_wcsrchr(::std::mem::transmute(string), ::std::mem::transmute(character)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
