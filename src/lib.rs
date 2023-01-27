#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(test)]

pub mod EverythingSDK {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    use bitflags::bitflags;
    use num_enum::TryFromPrimitive;
    use serde::Deserialize;
    use ts_rs::TS;

    #[derive(Debug, TS, TryFromPrimitive, PartialEq)]
    #[repr(u32)]
    #[ts(export)]
    pub enum EverythingError {
        // no error detected
        Ok = EVERYTHING_OK,
        // out of memory.
        Memory = EVERYTHING_ERROR_MEMORY,
        // Everything search client is not running
        Ipc = EVERYTHING_ERROR_IPC,
        // unable to register window class.
        RegisterClassEx = EVERYTHING_ERROR_REGISTERCLASSEX,
        // unable to create listening window
        CreateWindow = EVERYTHING_ERROR_CREATEWINDOW,
        // unable to create listening thread
        CreateThread = EVERYTHING_ERROR_CREATETHREAD,
        // invalid index
        InvalidIndex = EVERYTHING_ERROR_INVALIDINDEX,
        // invalid call
        InvalidCall = EVERYTHING_ERROR_INVALIDCALL,
        // invalid request data, request data first.
        InvalidRequest = EVERYTHING_ERROR_INVALIDREQUEST,
        // bad parameter.
        InvalidParameter = EVERYTHING_ERROR_INVALIDPARAMETER,
    }

    #[derive(Deserialize, TS, Debug, TryFromPrimitive)]
    #[repr(u32)]
    #[ts(export)]
    pub enum EverythingSort {
        NameAsc = EVERYTHING_SORT_NAME_ASCENDING,
        NameDesc = EVERYTHING_SORT_NAME_DESCENDING,
        PathAsc = EVERYTHING_SORT_PATH_ASCENDING,
        PathDesc = EVERYTHING_SORT_PATH_DESCENDING,
        SizeAsc = EVERYTHING_SORT_SIZE_ASCENDING,
        SizeDesc = EVERYTHING_SORT_SIZE_DESCENDING,
        ExtensionAsc = EVERYTHING_SORT_EXTENSION_ASCENDING,
        ExtensionDesc = EVERYTHING_SORT_EXTENSION_DESCENDING,
        TypeNameAsc = EVERYTHING_SORT_TYPE_NAME_ASCENDING,
        TypeNameDesc = EVERYTHING_SORT_TYPE_NAME_DESCENDING,
        DateCreatedAsc = EVERYTHING_SORT_DATE_CREATED_ASCENDING,
        DateCreatedDesc = EVERYTHING_SORT_DATE_CREATED_DESCENDING,
        DateModifiedAsc = EVERYTHING_SORT_DATE_MODIFIED_ASCENDING,
        DateModifiedDesc = EVERYTHING_SORT_DATE_MODIFIED_DESCENDING,
        AttributesAsc = EVERYTHING_SORT_ATTRIBUTES_ASCENDING,
        AttributesDesc = EVERYTHING_SORT_ATTRIBUTES_DESCENDING,
        FileListFilenameAsc = EVERYTHING_SORT_FILE_LIST_FILENAME_ASCENDING,
        FileListFilenameDesc = EVERYTHING_SORT_FILE_LIST_FILENAME_DESCENDING,
        RunCountAsc = EVERYTHING_SORT_RUN_COUNT_ASCENDING,
        RunCountDesc = EVERYTHING_SORT_RUN_COUNT_DESCENDING,
        DateRecentlyChangedAsc = EVERYTHING_SORT_DATE_RECENTLY_CHANGED_ASCENDING,
        DateRecentlyChangedDesc = EVERYTHING_SORT_DATE_RECENTLY_CHANGED_DESCENDING,
        DateAccessedAsc = EVERYTHING_SORT_DATE_ACCESSED_ASCENDING,
        DateAccessedDesc = EVERYTHING_SORT_DATE_ACCESSED_DESCENDING,
        DateRunAsc = EVERYTHING_SORT_DATE_RUN_ASCENDING,
        DateRunDesc = EVERYTHING_SORT_DATE_RUN_DESCENDING,
    }

    bitflags! {
        #[derive(Deserialize, TS)]
        pub struct EverythingRequestFlags: u32 {
            const FileName = EVERYTHING_REQUEST_FILE_NAME;
            const Path = EVERYTHING_REQUEST_PATH;
            const FullPathAndFileName = EVERYTHING_REQUEST_FULL_PATH_AND_FILE_NAME;
            const Extension = EVERYTHING_REQUEST_EXTENSION;
            const Size = EVERYTHING_REQUEST_SIZE;
            const DateCreated = EVERYTHING_REQUEST_DATE_CREATED;
            const DateModified = EVERYTHING_REQUEST_DATE_MODIFIED;
            const DateAccessed = EVERYTHING_REQUEST_DATE_ACCESSED;
            const Attributes = EVERYTHING_REQUEST_ATTRIBUTES;
            const FileListFileName = EVERYTHING_REQUEST_FILE_LIST_FILE_NAME;
            const RunCount = EVERYTHING_REQUEST_RUN_COUNT;
            const DateRun = EVERYTHING_REQUEST_DATE_RUN;
            const DateRecentlyChanged = EVERYTHING_REQUEST_DATE_RECENTLY_CHANGED;
            const HighlightedFileName = EVERYTHING_REQUEST_HIGHLIGHTED_FILE_NAME;
            const HighlightedPath = EVERYTHING_REQUEST_HIGHLIGHTED_PATH;
            const HighlightedFullPathAndFileName = EVERYTHING_REQUEST_HIGHLIGHTED_FULL_PATH_AND_FILE_NAME;
        }
    }

    #[derive(Deserialize, TS, Debug, TryFromPrimitive)]
    #[ts(export)]
    #[repr(u32)]
    pub enum EverythingRequestFlagsEnum {
        FileName = EVERYTHING_REQUEST_FILE_NAME,
        Path = EVERYTHING_REQUEST_PATH,
        FullPathAndFileName = EVERYTHING_REQUEST_FULL_PATH_AND_FILE_NAME,
        Extension = EVERYTHING_REQUEST_EXTENSION,
        Size = EVERYTHING_REQUEST_SIZE,
        DateCreated = EVERYTHING_REQUEST_DATE_CREATED,
        DateModified = EVERYTHING_REQUEST_DATE_MODIFIED,
        DateAccessed = EVERYTHING_REQUEST_DATE_ACCESSED,
        Attributes = EVERYTHING_REQUEST_ATTRIBUTES,
        FileListFileName = EVERYTHING_REQUEST_FILE_LIST_FILE_NAME,
        RunCount = EVERYTHING_REQUEST_RUN_COUNT,
        DateRun = EVERYTHING_REQUEST_DATE_RUN,
        DateRecentlyChanged = EVERYTHING_REQUEST_DATE_RECENTLY_CHANGED,
        HighlightedFileName = EVERYTHING_REQUEST_HIGHLIGHTED_FILE_NAME,
        HighlightedPath = EVERYTHING_REQUEST_HIGHLIGHTED_PATH,
        HighlightedFullPathAndFileName = EVERYTHING_REQUEST_HIGHLIGHTED_FULL_PATH_AND_FILE_NAME,
    }

    #[derive(Deserialize, TS)]
    #[ts(export)]
    pub enum ResultType {
        File,
        Path,
        FullPath,
    }
}

use widestring::*;
use EverythingSDK::*;

pub struct Everything;

impl Everything {
    fn parse_string_ptr(ptr: *const u16) -> String {
        if ptr.is_null() {
            let error_code = Everything::get_last_error();
            panic!("Error code: {:?}", error_code);
        }
        unsafe { U16CStr::from_ptr_str(ptr).to_string_lossy() }
    }

    pub fn version() -> String {
        unsafe {
            let major = Everything_GetMajorVersion();
            let minor = Everything_GetMinorVersion();
            let revision = Everything_GetRevision();
            format!("{}.{}.{}", major, minor, revision)
        }
    }

    pub fn wait_db_loaded() {
        let sleep_duration = 300;
        let max_wait_time = 60 * 1000 * 2;

        unsafe {
            for _ in 0..(max_wait_time / sleep_duration) {
                if Everything_IsDBLoaded() != 0 {
                    return;
                } else {
                    if Everything::get_last_error() == EverythingError::Ok {
                        // Everything is still loading the database
                        std::thread::sleep(std::time::Duration::from_millis(sleep_duration));
                    }
                }
            }
            panic!("Timeout waiting for database to load");
        }
    }

    pub fn new() -> Self {
        Self::wait_db_loaded();
        Self
    }

    pub fn reset(self: &Self) {
        unsafe {
            Everything_Reset();
        }
    }

    pub fn get_last_error() -> EverythingError {
        match EverythingError::try_from(unsafe { Everything_GetLastError() }) {
            Ok(error_code) => error_code,
            Err(_) => panic!("Unknown error code"),
        }
    }

    pub fn set_search(self: &Self, search: &str) {
        let search_c_str = match U16CString::from_str(search) {
            Ok(res) => res,
            Err(_) => panic!("Could not convert search string to c string: {}", search),
        };

        unsafe {
            Everything_SetSearchW(search_c_str.as_ptr());
        }
    }

    pub fn get_search(self: &Self) -> String {
        unsafe { Self::parse_string_ptr(Everything_GetSearchW()) }
    }

    pub fn set_request_flags(self: &Self, flag: u32) {
        unsafe {
            Everything_SetRequestFlags(flag);
        }
    }

    pub fn set_sort(self: &Self, sort: EverythingSort) {
        unsafe {
            Everything_SetSort(sort as u32);
        }
    }

    pub fn get_sort(self: &Self) -> EverythingSort {
        match EverythingSort::try_from(unsafe { Everything_GetSort() }) {
            Ok(sort) => sort,
            Err(_) => panic!("Unknown sort code"),
        }
    }

    pub fn query(self: &Self) -> DWORD {
        unsafe {
            Self::wait_db_loaded();
            Everything_QueryW(true as BOOL);
            self.get_num_results()
        }
    }

    pub fn is_result_file(self: &Self, index: DWORD) -> bool {
        unsafe { Everything_IsFileResult(index) != 0 }
    }

    pub fn is_result_folder(self: &Self, index: DWORD) -> bool {
        unsafe { Everything_IsFolderResult(index) != 0 }
    }

    pub fn is_result_volume(self: &Self, index: DWORD) -> bool {
        unsafe { Everything_IsVolumeResult(index) != 0 }
    }

    pub fn get_num_results(self: &Self) -> DWORD {
        unsafe { Everything_GetNumResults() }
    }

    pub fn name_iter(self: &Self) -> impl Iterator<Item = String> + '_ {
        (0..self.get_num_results()).map(|i| self.get_result_file_name(i))
    }

    pub fn path_iter(
        self: &Self,
        start: u32,
        end: u32,
        result_type: EverythingSDK::ResultType,
    ) -> impl Iterator<Item = String> + '_ {
        (self.clamp_index(start)..self.clamp_index(end)).map(move |i| match result_type {
            EverythingSDK::ResultType::File => self.get_result_file_name(i),
            EverythingSDK::ResultType::Path => self.get_result_file_path(i),
            EverythingSDK::ResultType::FullPath => self.get_result_full_path(i),
        })
    }

    pub fn get_result_full_path(self: &Self, index: DWORD) -> String {
        let index = self.clamp_index(index);
        unsafe {
            let file_path_ptr = Everything_GetResultPathW(index);
            let file_ptr = Everything_GetResultFileNameW(index);

            let file_path = Self::parse_string_ptr(file_path_ptr);
            let file = Self::parse_string_ptr(file_ptr);

            if file_path.is_empty() {
                return file;
            }

            format!("{}\\{}", file_path, file)
        }
    }

    pub fn get_result_file_path(self: &Self, index: DWORD) -> String {
        let index = self.clamp_index(index);
        unsafe { Self::parse_string_ptr(Everything_GetResultPathW(index)) }
    }

    pub fn get_result_file_name(self: &Self, index: u32) -> String {
        let index = self.clamp_index(index as DWORD);
        unsafe { Self::parse_string_ptr(Everything_GetResultFileNameW(index)) }
    }

    pub fn cleanup(self: &Self) {
        unsafe {
            Everything_CleanUp();
        }
    }

    fn clamp_index(self: &Self, index: DWORD) -> DWORD {
        let num_results = self.get_num_results();

        if num_results == 0 {
            return 0;
        }
        if index >= num_results {
            num_results - 1
        } else {
            index
        }
    }
}

impl Drop for Everything {
    fn drop(self: &mut Self) {
        self.cleanup();
    }
}

extern crate test;

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::Everything;
    use crate::EverythingSDK::*;
    use lazy_static::lazy_static;
    use test::Bencher;

    static BENCH_SIZE: u32 = 1;

    lazy_static! {
        static ref EVERYTHING: Mutex<Everything> = Mutex::new(Everything::new());
    }

    #[test]
    fn reports_version() {
        let version = Everything::version();
        assert_eq!(version, "1.5.0");
    }

    #[test]
    fn waits_for_db_loaded() {
        Everything::wait_db_loaded();
    }

    #[test]
    fn search() {
        let everything = EVERYTHING.lock().unwrap();
        everything.reset();

        everything.set_sort(EverythingSort::ExtensionAsc);

        everything.set_search("test");
        assert_eq!(everything.get_search(), "test");

        everything.set_search(".mp4");
        assert_eq!(everything.get_search(), ".mp4");

        everything.set_request_flags(
            (EverythingRequestFlags::FileName | EverythingRequestFlags::Path).bits(),
        );

        let num_results = everything.query();
        assert!(num_results > 0);
        println!("Num results: {}", num_results);
        assert_eq!(everything.get_num_results(), num_results);

        for i in 0..num_results {
            let file_name = everything.get_result_file_name(i);
            // println!("File name: {}", file_name);
            assert!(file_name.len() > 0);

            let file_path = everything.get_result_file_path(i);
            // println!("File path: {}", file_path);
            assert!(file_path.len() > 0);

            let full_path = everything.get_result_full_path(i);
            // println!("Full path: {}", full_path);
            assert!(full_path.len() > 0);
        }
    }

    #[test]
    fn get_one_result() {
        let everything = EVERYTHING.lock().unwrap();
        everything.reset();

        everything.set_search("test");

        everything.set_request_flags(
            (EverythingRequestFlags::FileName | EverythingRequestFlags::Path).bits(),
        );

        let num_results = everything.query();

        println!("Num results: {}", num_results);

        assert!(num_results > 0);

        let file_name = everything.get_result_file_name(0);

        println!("File name: {}", file_name);

        assert!(file_name.len() > 0);

        let file_path = everything.get_result_file_path(0);

        println!("File path: {}", file_path);

        assert!(file_path.len() > 0);

        let full_path = everything.get_result_full_path(0);

        println!("Full path: {}", full_path);

        assert!(full_path.len() > 0);
    }

    fn init_search() -> u32 {
        let everything = EVERYTHING.lock().unwrap();
        everything.reset();

        let search_filter = ".mp4";

        everything.set_search(search_filter);
        assert_eq!(everything.get_search(), search_filter);

        everything.set_request_flags(
            (EverythingRequestFlags::FileName | EverythingRequestFlags::Path).bits(),
        );

        let num_results = everything.query();
        assert!(num_results > 0);
        num_results
    }

    #[bench]
    fn full_path(b: &mut Bencher) {
        let num_results = init_search();
        let everything = EVERYTHING.lock().unwrap();

        b.iter(|| {
            for _ in 1..=BENCH_SIZE {
                let file_names = (0..num_results)
                    .into_iter()
                    .map(|j| everything.get_result_full_path(j))
                    .collect::<Vec<String>>();
                assert!(file_names.len() == num_results as usize);
            }
        })
    }

    #[bench]
    fn file_name(b: &mut Bencher) {
        let num_results = init_search();
        let everything = EVERYTHING.lock().unwrap();

        b.iter(|| {
            for _ in 1..=BENCH_SIZE {
                let file_names = (0..num_results)
                    .into_iter()
                    .map(|j| everything.get_result_file_name(j))
                    .collect::<Vec<String>>();
                assert!(file_names.len() == num_results as usize);
            }
        })
    }

    #[bench]
    fn file_name_raw(b: &mut Bencher) {
        let num_results = init_search();

        b.iter(|| {
            for _ in 1..=BENCH_SIZE {
                let file_names = (0..num_results)
                    .into_iter()
                    .map(|j| unsafe { Everything_GetResultFileNameW(j) })
                    .collect::<Vec<LPCWSTR>>();
                assert!(file_names.len() == num_results as usize);
            }
        })
    }
}
