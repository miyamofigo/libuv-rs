extern crate core;
extern crate libc;

use self::core::option::Option;
use libc::*;

pub type uv_mutex_t = pthread_mutex_t;
pub type uv_rwlock_t = pthread_rwlock_t;

pub type uv_timer_t = Struct_uv_timer_s;
pub type uv_loop_t = Struct_uv_loop_s;
pub type uv_async_t = Struct_uv_async_s;
pub type uv_handle_t = Struct_uv_handle_s;
pub type uv_signal_t = Struct_uv_signal_s;
pub type uv_handle_type = Enum_Unnamed95;
pub type uv__io_t = Struct_uv__io_s;

pub type uv_close_cb =
    Option<unsafe extern "C" fn(handle: *mut uv_handle_t)>;

pub type uv__async_cb =
    Option<unsafe extern "C" fn(_loop: *mut Struct_uv_loop_s,
                                w: *mut Struct_uv__async,
                                nevents: c_uint)>;
pub type uv__io_cb =
    Option<unsafe extern "C" fn(_loop: *mut Struct_uv_loop_s,
                                w: *mut Struct_uv__io_s,
                                events: c_uint)>;

pub type uv_async_cb =
    Option<unsafe extern "C" fn(handle: *mut uv_async_t)>;

pub type uv_signal_cb =
    Option<unsafe extern "C" fn(handle: *mut uv_signal_t,
                                signum: c_int)>;

pub type uv_timer_cb =
    Option<unsafe extern "C" fn(handle: *mut uv_timer_t)>;

#[repr(C)]
#[derive(Copy)]
pub struct Struct_uv_timer_s {
    pub data: *mut c_void,
    pub _loop: *mut uv_loop_t,
    pub _type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut c_void; 2usize],
    pub u: Union_Unnamed114,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::std::os::raw::c_uint,
    pub timer_cb: uv_timer_cb,
    pub heap_node: [*mut c_void; 3usize],
    pub timeout: uint64_t,
    pub repeat: uint64_t,
    pub start_id: uint64_t,
} 

impl ::std::clone::Clone for Struct_uv_timer_s {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for Struct_uv_timer_s {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_uv_loop_s {
    pub data: *mut c_void,
    pub active_handles: c_uint,
    pub handle_queue: [*mut c_void; 2usize],
    pub active_reqs: [*mut c_void; 2usize],
    pub stop_flag: c_uint,
    pub flags: c_ulong,
    pub backend_fd: c_int,
    pub pending_queue: [*mut c_void; 2usize],
    pub watcher_queue: [*mut c_void; 2usize],
    pub watchers: *mut *mut uv__io_t,
    pub nwatchers: c_uint,
    pub nfds: c_uint,
    pub wq: [*mut c_void; 2usize],
    pub wq_mutex: uv_mutex_t,
    pub wq_async: uv_async_t,
    pub cloexec_lock: uv_rwlock_t,
    pub closing_handles: *mut uv_handle_t,
    pub process_handles: [*mut c_void; 2usize],
    pub prepare_handles: [*mut c_void; 2usize],
    pub check_handles: [*mut c_void; 2usize],
    pub idle_handles: [*mut c_void; 2usize],
    pub async_handles: [*mut c_void; 2usize],
    pub async_watcher: Struct_uv__async,
    pub timer_heap: Struct_Unnamed128,
    pub timer_counter: uint64_t,
    pub time: uint64_t,
    pub signal_pipefd: [c_int; 2usize],
    pub signal_io_watcher: uv__io_t,
    pub child_watcher: uv_signal_t,
    pub emfile_fd: c_int,
    pub inotify_read_watcher: uv__io_t,
    pub inotify_watchers: *mut c_void,
    pub inotify_fd: c_int,
}

impl ::std::clone::Clone for Struct_uv_loop_s {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for Struct_uv_loop_s {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_uv__io_s {
    pub cb: uv__io_cb,
    pub pending_queue: [*mut c_void; 2usize],
    pub watcher_queue: [*mut c_void; 2usize],
    pub pevents: c_uint,
    pub events: c_uint,
    pub fd: c_int,
}

impl ::std::clone::Clone for Struct_uv__io_s {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for Struct_uv__io_s {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_uv_async_s {
    pub data: *mut c_void,
    pub _loop: *mut uv_loop_t,
    pub _type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut c_void; 2usize],
    pub u: Union_Unnamed113,
    pub next_closing: *mut uv_handle_t,
    pub flags: c_uint,
    pub async_cb: uv_async_cb,
    pub queue: [*mut c_void; 2usize],
    pub pending: c_int,
}

impl ::std::clone::Clone for Struct_uv_async_s {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for Struct_uv_async_s {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_uv_handle_s {
    pub data: *mut c_void,
    pub _loop: *mut uv_loop_t,
    pub _type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut c_void; 2usize],
    pub u: Union_Unnamed102,
    pub next_closing: *mut uv_handle_t,
    pub flags: c_uint,
}

impl ::std::clone::Clone for Struct_uv_handle_s {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for Struct_uv_handle_s {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
} 

#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed102 {
    pub _bindgen_data_: [u32; 4usize],
}

impl Union_Unnamed102 {
    pub unsafe fn fd(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn reserved(&mut self)
     -> *mut [*mut ::std::os::raw::c_void; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}

impl ::std::clone::Clone for Union_Unnamed102 {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for Union_Unnamed102 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_uv__async {
    pub cb: uv__async_cb,
    pub io_watcher: uv__io_t,
    pub wfd: c_int,
}

impl ::std::clone::Clone for Struct_uv__async {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for Struct_uv__async {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_uv_signal_s {
    pub data: *mut c_void,
    pub _loop: *mut uv_loop_t,
    pub _type: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: [*mut c_void; 2usize],
    pub u: Union_Unnamed126,
    pub next_closing: *mut uv_handle_t,
    pub flags: c_uint,
    pub signal_cb: uv_signal_cb,
    pub signum: c_int,
    pub tree_entry: Struct_Unnamed127,
    pub caught_signals: c_uint,
    pub dispatched_signals: c_uint,
}

impl ::std::clone::Clone for Struct_uv_signal_s {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for Struct_uv_signal_s {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed126 {
    pub _bindgen_data_: [u32; 4usize],
}

impl Union_Unnamed126 {
    pub unsafe fn fd(&mut self) -> *mut c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn reserved(&mut self)
     -> *mut [*mut c_void; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}

impl ::std::clone::Clone for Union_Unnamed126 {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for Union_Unnamed126 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed127 {
    pub rbe_left: *mut Struct_uv_signal_s,
    pub rbe_right: *mut Struct_uv_signal_s,
    pub rbe_parent: *mut Struct_uv_signal_s,
    pub rbe_color: c_int,
}

impl ::std::clone::Clone for Struct_Unnamed127 {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for Struct_Unnamed127 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Enum_Unnamed95 {
    UV_UNKNOWN_HANDLE = 0,
    UV_ASYNC = 1,
    UV_CHECK = 2,
    UV_FS_EVENT = 3,
    UV_FS_POLL = 4,
    UV_HANDLE = 5,
    UV_IDLE = 6,
    UV_NAMED_PIPE = 7,
    UV_POLL = 8,
    UV_PREPARE = 9,
    UV_PROCESS = 10,
    UV_STREAM = 11,
    UV_TCP = 12,
    UV_TIMER = 13,
    UV_TTY = 14,
    UV_UDP = 15,
    UV_SIGNAL = 16,
    UV_FILE = 17,
    UV_HANDLE_TYPE_MAX = 18,
}

#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed114 {
    pub _bindgen_data_: [u32; 4usize],
}

impl Union_Unnamed114 {
    pub unsafe fn fd(&mut self) -> *mut c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn reserved(&mut self)
     -> *mut [*mut ::std::os::raw::c_void; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}

impl ::std::clone::Clone for Union_Unnamed114 {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for Union_Unnamed114 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed113 {
    pub _bindgen_data_: [u32; 4usize],
}

impl Union_Unnamed113 {
    pub unsafe fn fd(&mut self) -> *mut c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn reserved(&mut self)
     -> *mut [*mut ::std::os::raw::c_void; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}

impl ::std::clone::Clone for Union_Unnamed113 {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for Union_Unnamed113 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed128 {
    pub min: *mut c_void,
    pub nelts: c_uint,
}

impl ::std::clone::Clone for Struct_Unnamed128 {
    fn clone(&self) -> Self { *self }
}

impl ::std::default::Default for Struct_Unnamed128 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
