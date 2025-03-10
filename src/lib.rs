#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::collections::HashMap;
use std::convert::TryFrom;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt;
use std::mem;
use std::mem::zeroed;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::slice;
use std::time::Duration;

use libc;

/// Error.
#[derive(Copy, Clone)]
pub enum SrError {
    Ok = sr_error_t_SR_ERR_OK as isize,
    InvalArg = sr_error_t_SR_ERR_INVAL_ARG as isize,
    Ly = sr_error_t_SR_ERR_LY as isize,
    Sys = sr_error_t_SR_ERR_SYS as isize,
    NoMemory = sr_error_t_SR_ERR_NO_MEMORY as isize,
    NotFound = sr_error_t_SR_ERR_NOT_FOUND as isize,
    Exists = sr_error_t_SR_ERR_EXISTS as isize,
    Internal = sr_error_t_SR_ERR_INTERNAL as isize,
    Unsupported = sr_error_t_SR_ERR_UNSUPPORTED as isize,
    ValidationFailed = sr_error_t_SR_ERR_VALIDATION_FAILED as isize,
    OperationFailed = sr_error_t_SR_ERR_OPERATION_FAILED as isize,
    Unauthorized = sr_error_t_SR_ERR_UNAUTHORIZED as isize,
    Locked = sr_error_t_SR_ERR_LOCKED as isize,
    TimeOut = sr_error_t_SR_ERR_TIME_OUT as isize,
    CallbackFailed = sr_error_t_SR_ERR_CALLBACK_FAILED as isize,
    CallbackShelve = sr_error_t_SR_ERR_CALLBACK_SHELVE as isize,
}

/// Log level.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SrLogLevel {
    None = sr_log_level_t_SR_LL_NONE as isize,
    Error = sr_log_level_t_SR_LL_ERR as isize,
    Warn = sr_log_level_t_SR_LL_WRN as isize,
    Info = sr_log_level_t_SR_LL_INF as isize,
    Debug = sr_log_level_t_SR_LL_DBG as isize,
}

/// Conn Flag.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SrConnFlag {
    Default = sr_conn_flag_t_SR_CONN_DEFAULT as isize,
    CacheRunning = sr_conn_flag_t_SR_CONN_CACHE_RUNNING as isize,
}

/// Datastore.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SrDatastore {
    Startup = sr_datastore_t_SR_DS_STARTUP as isize,
    Running = sr_datastore_t_SR_DS_RUNNING as isize,
    Candidate = sr_datastore_t_SR_DS_CANDIDATE as isize,
    Operational = sr_datastore_t_SR_DS_OPERATIONAL as isize,
}

/// Sysrepo Type.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SrType {
    Unknown = sr_val_type_t_SR_UNKNOWN_T as isize,
    List = sr_val_type_t_SR_LIST_T as isize,
    Container = sr_val_type_t_SR_CONTAINER_T as isize,
    ContainerPresence = sr_val_type_t_SR_CONTAINER_PRESENCE_T as isize,
    LeafEmpty = sr_val_type_t_SR_LEAF_EMPTY_T as isize,
    Notification = sr_val_type_t_SR_NOTIFICATION_T as isize,
    Binary = sr_val_type_t_SR_BINARY_T as isize,
    Bits = sr_val_type_t_SR_BITS_T as isize,
    Bool = sr_val_type_t_SR_BOOL_T as isize,
    Decimal64 = sr_val_type_t_SR_DECIMAL64_T as isize,
    Enum = sr_val_type_t_SR_ENUM_T as isize,
    IdentityRef = sr_val_type_t_SR_IDENTITYREF_T as isize,
    InstanceId = sr_val_type_t_SR_INSTANCEID_T as isize,
    Int8 = sr_val_type_t_SR_INT8_T as isize,
    Int16 = sr_val_type_t_SR_INT16_T as isize,
    Int32 = sr_val_type_t_SR_INT32_T as isize,
    Int64 = sr_val_type_t_SR_INT64_T as isize,
    String = sr_val_type_t_SR_STRING_T as isize,
    UInt8 = sr_val_type_t_SR_UINT8_T as isize,
    UInt16 = sr_val_type_t_SR_UINT16_T as isize,
    UInt32 = sr_val_type_t_SR_UINT32_T as isize,
    UInt64 = sr_val_type_t_SR_UINT64_T as isize,
    AnyXml = sr_val_type_t_SR_ANYXML_T as isize,
    AnyData = sr_val_type_t_SR_ANYDATA_T as isize,
}

/// Get Oper Flag.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SrGetOperFlag {
    Default = sr_get_oper_flag_t_SR_OPER_DEFAULT as isize,
    NoState = sr_get_oper_flag_t_SR_OPER_NO_STATE as isize,
    NoConfig = sr_get_oper_flag_t_SR_OPER_NO_CONFIG as isize,
    NoSubs = sr_get_oper_flag_t_SR_OPER_NO_SUBS as isize,
    NoStored = sr_get_oper_flag_t_SR_OPER_NO_STORED as isize,
    WithOrigin = sr_get_oper_flag_t_SR_OPER_WITH_ORIGIN as isize,
}

/// Edit Flag.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SrEditFlag {
    Default = sr_edit_flag_t_SR_EDIT_DEFAULT as isize,
    NonRecursive = sr_edit_flag_t_SR_EDIT_NON_RECURSIVE as isize,
    Strict = sr_edit_flag_t_SR_EDIT_STRICT as isize,
    Isolate = sr_edit_flag_t_SR_EDIT_ISOLATE as isize,
}

/// Move Position.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SrMovePosition {
    Before = sr_move_position_t_SR_MOVE_BEFORE as isize,
    After = sr_move_position_t_SR_MOVE_AFTER as isize,
    First = sr_move_position_t_SR_MOVE_FIRST as isize,
    Last = sr_move_position_t_SR_MOVE_LAST as isize,
}

/// Subscribe Flag.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SrSubcribeFlag {
    Default = sr_subscr_flag_t_SR_SUBSCR_DEFAULT as isize,
    NoThread = sr_subscr_flag_t_SR_SUBSCR_NO_THREAD as isize,
    Passive = sr_subscr_flag_t_SR_SUBSCR_PASSIVE as isize,
    DoneOnly = sr_subscr_flag_t_SR_SUBSCR_DONE_ONLY as isize,
    Enabled = sr_subscr_flag_t_SR_SUBSCR_ENABLED as isize,
    Update = sr_subscr_flag_t_SR_SUBSCR_UPDATE as isize,
    OperMerge = sr_subscr_flag_t_SR_SUBSCR_OPER_MERGE as isize,
}

/// Event.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SrEvent {
    Update = sr_event_t_SR_EV_UPDATE as isize,
    Change = sr_event_t_SR_EV_CHANGE as isize,
    Done = sr_event_t_SR_EV_DONE as isize,
    Abort = sr_event_t_SR_EV_ABORT as isize,
    Enabled = sr_event_t_SR_EV_ENABLED as isize,
    Rpc = sr_event_t_SR_EV_RPC as isize,
}

impl TryFrom<u32> for SrEvent {
    type Error = &'static str;

    fn try_from(t: u32) -> Result<Self, Self::Error> {
        match t {
            sr_event_t_SR_EV_UPDATE => Ok(SrEvent::Update),
            sr_event_t_SR_EV_CHANGE => Ok(SrEvent::Change),
            sr_event_t_SR_EV_DONE => Ok(SrEvent::Done),
            sr_event_t_SR_EV_ABORT => Ok(SrEvent::Abort),
            sr_event_t_SR_EV_ENABLED => Ok(SrEvent::Enabled),
            sr_event_t_SR_EV_RPC => Ok(SrEvent::Rpc),
            _ => Err("Invalid SrEvent"),
        }
    }
}

impl fmt::Display for SrEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            SrEvent::Update => "Update",
            SrEvent::Change => "Change",
            SrEvent::Done => "Done",
            SrEvent::Abort => "Abort",
            SrEvent::Enabled => "Enabled",
            SrEvent::Rpc => "RPC",
        };
        write!(f, "{}", s)
    }
}

/// Change Oper.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SrChangeOper {
    Created = sr_change_oper_t_SR_OP_CREATED as isize,
    Modified = sr_change_oper_t_SR_OP_MODIFIED as isize,
    Deleted = sr_change_oper_t_SR_OP_DELETED as isize,
    Moved = sr_change_oper_t_SR_OP_MOVED as isize,
}

impl TryFrom<u32> for SrChangeOper {
    type Error = &'static str;

    fn try_from(t: u32) -> Result<Self, Self::Error> {
        match t {
            sr_change_oper_t_SR_OP_CREATED => Ok(SrChangeOper::Created),
            sr_change_oper_t_SR_OP_MODIFIED => Ok(SrChangeOper::Modified),
            sr_change_oper_t_SR_OP_DELETED => Ok(SrChangeOper::Deleted),
            sr_change_oper_t_SR_OP_MOVED => Ok(SrChangeOper::Moved),
            _ => Err("Invalid SrChangeOper"),
        }
    }
}

impl fmt::Display for SrChangeOper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            SrChangeOper::Created => "Created",
            SrChangeOper::Modified => "Modified",
            SrChangeOper::Deleted => "Deleted",
            SrChangeOper::Moved => "Moved",
        };
        write!(f, "{}", s)
    }
}

/// Notification Type.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SrNotifType {
    Realtime = sr_ev_notif_type_t_SR_EV_NOTIF_REALTIME as isize,
    Replay = sr_ev_notif_type_t_SR_EV_NOTIF_REPLAY as isize,
    ReplayComplete = sr_ev_notif_type_t_SR_EV_NOTIF_REPLAY_COMPLETE as isize,
    Terminated = sr_ev_notif_type_t_SR_EV_NOTIF_TERMINATED as isize,
    Modified = sr_ev_notif_type_t_SR_EV_NOTIF_MODIFIED as isize,
    Suspended = sr_ev_notif_type_t_SR_EV_NOTIF_SUSPENDED as isize,
    Resumed = sr_ev_notif_type_t_SR_EV_NOTIF_RESUMED as isize,
}

impl TryFrom<u32> for SrNotifType {
    type Error = &'static str;

    fn try_from(t: u32) -> Result<Self, Self::Error> {
        match t {
            sr_ev_notif_type_t_SR_EV_NOTIF_REALTIME => Ok(SrNotifType::Realtime),
            sr_ev_notif_type_t_SR_EV_NOTIF_REPLAY => Ok(SrNotifType::Replay),
            sr_ev_notif_type_t_SR_EV_NOTIF_REPLAY_COMPLETE => Ok(SrNotifType::ReplayComplete),
            sr_ev_notif_type_t_SR_EV_NOTIF_TERMINATED => Ok(SrNotifType::Terminated),
            sr_ev_notif_type_t_SR_EV_NOTIF_MODIFIED => Ok(SrNotifType::Modified),
            sr_ev_notif_type_t_SR_EV_NOTIF_SUSPENDED => Ok(SrNotifType::Suspended),
            sr_ev_notif_type_t_SR_EV_NOTIF_RESUMED => Ok(SrNotifType::Resumed),
            _ => Err("Invalid SrNotifType"),
        }
    }
}

/// Lyd Anydata Value Type.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum LydAnyDataValueType {
    String = LYD_ANYDATA_VALUETYPE_LYD_ANYDATA_STRING as isize,
    Json = LYD_ANYDATA_VALUETYPE_LYD_ANYDATA_JSON as isize,
    Xml = LYD_ANYDATA_VALUETYPE_LYD_ANYDATA_XML as isize,
    Datatree = LYD_ANYDATA_VALUETYPE_LYD_ANYDATA_DATATREE as isize,
    Lyb = LYD_ANYDATA_VALUETYPE_LYD_ANYDATA_LYB as isize,
}

/// Typedefs.
pub type SrSessionId = *const sr_session_ctx_t;
pub type SrSubscrId = *const sr_subscription_ctx_t;

/// Single Sysrepo Value.
pub struct SrValue {
    value: *mut sr_val_t,
}

impl SrValue {
    pub fn from(value: *mut sr_val_t) -> Self {
        Self { value: value }
    }

    pub fn value(&self) -> *mut sr_val_t {
        self.value
    }
}

impl Drop for SrValue {
    fn drop(&mut self) {
        unsafe {
            sr_free_val(self.value);
        }
    }
}

/// Slice of Sysrepo Value.
///  The size of slice cannot change.
pub struct SrValueSlice {
    /// Pointer to raw sr_val_t array.
    values: *mut sr_val_t,

    /// Length of this slice.
    len: u64,

    /// Owned flag.
    owned: bool,
}

impl SrValueSlice {
    pub fn new(capacity: u64, owned: bool) -> Self {
        Self {
            values: unsafe {
                libc::malloc(mem::size_of::<sr_val_t>() * capacity as usize) as *mut sr_val_t
            },
            len: capacity,
            owned: owned,
        }
    }

    pub fn from(values: *mut sr_val_t, len: u64, owned: bool) -> Self {
        Self {
            values: values,
            len: len,
            owned: owned,
        }
    }

    pub fn at_mut(&mut self, index: usize) -> &mut sr_val_t {
        let slice = unsafe { slice::from_raw_parts_mut(self.values, self.len as usize) };

        &mut slice[index]
    }

    pub fn as_slice(&mut self) -> &[sr_val_t] {
        unsafe { slice::from_raw_parts(self.values, self.len as usize) }
    }

    pub fn as_ptr(&self) -> *mut sr_val_t {
        self.values
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn set_owned(&mut self) {
        self.owned = true;
    }

    pub fn set_int64_value(&mut self, index: usize, dflt: bool, xpath: &str, value: i64) {
        let xpath = CString::new(xpath).unwrap();
        let xpath_ptr = xpath.as_ptr();

        let mut val = self.at_mut(index) as *mut sr_val_t;
        unsafe {
            (*val).xpath = libc::strdup(xpath_ptr);
            (*val).type_ = sr_val_type_t_SR_INT64_T;
            (*val).dflt = if dflt { 0 } else { 1 }; //TODO: It is really those values?
            (*val).data.int64_val = value;
        }
    }
}

impl Drop for SrValueSlice {
    fn drop(&mut self) {
        if self.owned {
            unsafe {
                sr_free_values(self.values, self.len);
            }
        }
    }
}

/// Set Log Stderr.
pub fn log_stderr(log_level: SrLogLevel) {
    unsafe {
        sr_log_stderr(log_level as u32);
    }
}

/// Set Log Syslog.
pub fn log_syslog(app_name: &str, log_level: SrLogLevel) {
    let app_name = app_name.as_ptr() as *const i8;
    unsafe {
        sr_log_syslog(app_name, log_level as u32);
    }
}

/// Sysrepo connection.
pub struct SrConn {
    /// Raw Pointer to Connection.
    conn: *mut sr_conn_ctx_t,

    /// Sessions.
    sessions: HashMap<SrSessionId, SrSession>,
}

impl SrConn {
    /// Constructor.
    pub fn new(opts: sr_conn_options_t) -> Result<SrConn, i32> {
        let mut conn = std::ptr::null_mut();

        let rc = unsafe { sr_connect(opts, &mut conn) };

        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            Ok(SrConn {
                conn: conn,
                sessions: HashMap::new(),
            })
        }
    }

    /// Disconnect.
    pub fn disconnect(&mut self) {
        unsafe {
            sr_disconnect(self.conn);
        }
        self.conn = std::ptr::null_mut();
    }

    /// Add session to map.
    pub fn insert_session(&mut self, id: SrSessionId, sess: SrSession) {
        self.sessions.insert(id, sess);
    }

    /// Add session to map.
    pub fn remove_session(&mut self, id: &SrSessionId) {
        self.sessions.remove(id);
    }

    /// Lookup session from map.
    pub fn lookup_session(&mut self, id: &SrSessionId) -> Option<&mut SrSession> {
        self.sessions.get_mut(id)
    }

    /// Start session.
    pub fn start_session(&mut self, ds: SrDatastore) -> Result<&mut SrSession, i32> {
        let mut sess = std::ptr::null_mut();
        let rc = unsafe { sr_session_start(self.conn, ds as u32, &mut sess) };
        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            let id = sess;
            self.insert_session(id, SrSession::from(sess, true));
            Ok(self.sessions.get_mut(&(id as SrSessionId)).unwrap())
        }
    }

    /// Get context.
    pub fn get_context(&mut self) -> LibYangCtx {
        LibYangCtx::from(unsafe { sr_acquire_context(self.conn) })
    }
}

impl Drop for SrConn {
    fn drop(&mut self) {
        self.sessions.drain();
        self.disconnect();
    }
}

/// Sysrepo session.
pub struct SrSession {
    /// Raw Pointer to session.
    sess: *mut sr_session_ctx_t,

    /// Owned flag.
    owned: bool,

    /// Map from raw pointer to subscription.
    subscrs: HashMap<SrSubscrId, SrSubscr>,
}

impl SrSession {
    /// Constructor.
    pub fn new() -> Self {
        Self {
            sess: std::ptr::null_mut(),
            owned: true,
            subscrs: HashMap::new(),
        }
    }

    /// Constructor.
    pub fn from(sess: *mut sr_session_ctx_t, owned: bool) -> Self {
        Self {
            sess: sess,
            owned: owned,
            subscrs: HashMap::new(),
        }
    }

    /// Create unowned clone.
    pub fn clone(&self) -> Self {
        Self {
            sess: self.sess,
            owned: false,
            subscrs: HashMap::new(),
        }
    }

    /// Get raw session context.
    pub unsafe fn get_ctx(&self) -> *mut sr_session_ctx_t {
        self.sess
    }

    /// Insert subscription.
    pub fn insert_subscription(&mut self, subscr: SrSubscr) -> SrSubscrId {
        let id = subscr.id();
        self.subscrs.insert(id, subscr);
        id
    }

    /// Remove subscription.
    pub fn remove_subscription(&mut self, subscr: &SrSubscr) {
        let id = subscr.id();
        self.subscrs.remove(&id);
    }

    /// Get items from given Xpath, anre return result in Value slice.
    pub fn get_items(
        &mut self,
        xpath: &str,
        timeout: Option<Duration>,
        opts: u32,
    ) -> Result<SrValueSlice, i32> {
        let xpath = xpath.as_ptr() as *const i8;
        let timeout_ms = timeout.map_or(0, |timeout| timeout.as_millis() as u32);
        let mut values_count: u64 = 0;
        let mut values: *mut sr_val_t = unsafe { zeroed::<*mut sr_val_t>() };

        let rc = unsafe {
            sr_get_items(
                self.sess,
                xpath,
                timeout_ms,
                opts,
                &mut values,
                &mut values_count,
            )
        };
        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            Ok(SrValueSlice::from(values, values_count, true))
        }
    }

    /// Set string item to given Xpath.
    pub fn set_item_str(
        &mut self,
        path: &str,
        value: &str,
        origin: Option<&str>,
        opts: u32,
    ) -> Result<(), i32> {
        let path = path.as_ptr() as *const i8;
        let value = value.as_ptr() as *const i8;
        let origin = match origin {
            Some(orig) => orig.as_ptr() as *const i8,
            None => std::ptr::null(),
        };

        let rc = unsafe { sr_set_item_str(self.sess, path, value, origin, opts) };
        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            Ok(())
        }
    }

    /// Apply changes for the session.
    pub fn apply_changes(&mut self, timeout: Option<Duration>) -> Result<(), i32> {
        let timeout_ms = timeout.map_or(0, |timeout| timeout.as_millis() as u32);

        let rc = unsafe { sr_apply_changes(self.sess, timeout_ms) };
        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            Ok(())
        }
    }

    /// Subscribe event notification.
    pub fn notif_subscribe<F>(
        &mut self,
        mod_name: &str,
        xpath: Option<String>,
        start_time: Option<*mut timespec>,
        stop_time: Option<*mut timespec>,
        callback: F,
        opts: sr_subscr_options_t,
    ) -> Result<&mut SrSubscr, i32>
    where
        F: FnMut(SrSession, u32, SrNotifType, &str, SrValueSlice, *mut timespec) + 'static,
    {
        let mod_name = mod_name.as_ptr() as *const i8;
        let xpath = xpath.map_or(std::ptr::null_mut(), |xpath| xpath.as_ptr() as *mut i8);
        let start_time = start_time.unwrap_or(std::ptr::null_mut());
        let stop_time = stop_time.unwrap_or(std::ptr::null_mut());

        let mut subscr: *mut sr_subscription_ctx_t =
            unsafe { zeroed::<*mut sr_subscription_ctx_t>() };
        let data = Box::into_raw(Box::new(callback));
        let rc = unsafe {
            sr_notif_subscribe(
                self.sess,
                mod_name,
                xpath,
                start_time,
                stop_time,
                Some(SrSession::call_event_notif::<F>),
                data as *mut _,
                opts,
                &mut subscr,
            )
        };

        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            let id = self.insert_subscription(SrSubscr::from(subscr));
            Ok(self.subscrs.get_mut(&id).unwrap())
        }
    }

    unsafe extern "C" fn call_event_notif<F>(
        sess: *mut sr_session_ctx_t,
        sub_id: u32,
        notif_type: sr_ev_notif_type_t,
        path: *const c_char,
        values: *const sr_val_t,
        values_cnt: size_t,
        timestamp: *mut timespec,
        private_data: *mut c_void,
    ) where
        F: FnMut(SrSession, u32, SrNotifType, &str, SrValueSlice, *mut timespec),
    {
        let callback_ptr = private_data as *mut F;
        let callback = &mut *callback_ptr;

        let path = CStr::from_ptr(path).to_str().unwrap();
        let sr_values = SrValueSlice::from(values as *mut sr_val_t, values_cnt, false);
        let sess = SrSession::from(sess, false);
        let notif_type = SrNotifType::try_from(notif_type).expect("Convert error");

        callback(sess, sub_id, notif_type, path, sr_values, timestamp);
    }

    /// Subscribe RPC.
    pub fn rpc_subscribe<F>(
        &mut self,
        xpath: Option<String>,
        callback: F,
        priority: u32,
        opts: sr_subscr_options_t,
    ) -> Result<&mut SrSubscr, i32>
    where
        F: FnMut(SrSession, u32, &str, SrValueSlice, SrEvent, u32) -> SrValueSlice + 'static,
    {
        let mut subscr: *mut sr_subscription_ctx_t =
            unsafe { zeroed::<*mut sr_subscription_ctx_t>() };
        let data = Box::into_raw(Box::new(callback));

        let rc = unsafe {
            match xpath {
                Some(xpath) => {
                    let xpath = xpath.as_ptr() as *mut i8;
                    sr_rpc_subscribe(
                        self.sess,
                        xpath,
                        Some(SrSession::call_rpc::<F>),
                        data as *mut _,
                        priority,
                        opts,
                        &mut subscr,
                    )
                }
                None => sr_rpc_subscribe(
                    self.sess,
                    std::ptr::null_mut(),
                    Some(SrSession::call_rpc::<F>),
                    data as *mut _,
                    priority,
                    opts,
                    &mut subscr,
                ),
            }
        };

        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            let id = self.insert_subscription(SrSubscr::from(subscr));
            Ok(self.subscrs.get_mut(&id).unwrap())
        }
    }

    unsafe extern "C" fn call_rpc<F>(
        sess: *mut sr_session_ctx_t,
        sub_id: u32,
        op_path: *const c_char,
        input: *const sr_val_t,
        input_cnt: size_t,
        event: sr_event_t,
        request_id: u32,
        output: *mut *mut sr_val_t,
        output_cnt: *mut u64,
        private_data: *mut c_void,
    ) -> i32
    where
        F: FnMut(SrSession, u32, &str, SrValueSlice, SrEvent, u32) -> SrValueSlice,
    {
        let callback_ptr = private_data as *mut F;
        let callback = &mut *callback_ptr;

        let op_path = CStr::from_ptr(op_path).to_str().unwrap();
        let inputs = SrValueSlice::from(input as *mut sr_val_t, input_cnt, false);
        let sess = SrSession::from(sess, false);
        let event = SrEvent::try_from(event).expect("Convert error");

        let sr_output = callback(sess, sub_id, op_path, inputs, event, request_id);
        *output = sr_output.as_ptr();
        *output_cnt = sr_output.len();

        sr_error_t_SR_ERR_OK as i32
    }

    /// Subscribe oper get items.
    pub fn oper_get_subscribe<F>(
        &mut self,
        mod_name: &str,
        path: &str,
        callback: F,
        opts: sr_subscr_options_t,
    ) -> Result<&mut SrSubscr, i32>
    where
        F: FnMut(&LibYangCtx, u32, &str, &str, Option<&str>, u32) -> Option<LydNode> + 'static,
    {
        let mut subscr: *mut sr_subscription_ctx_t =
            unsafe { zeroed::<*mut sr_subscription_ctx_t>() };
        let data = Box::into_raw(Box::new(callback));
        let mod_name = mod_name.as_ptr() as *mut i8;
        let path = path.as_ptr() as *mut i8;

        let rc = unsafe {
            sr_oper_get_subscribe(
                self.sess,
                mod_name,
                path,
                Some(SrSession::call_get_items::<F>),
                data as *mut _,
                opts,
                &mut subscr,
            )
        };

        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            let id = self.insert_subscription(SrSubscr::from(subscr));
            Ok(self.subscrs.get_mut(&id).unwrap())
        }
    }

    unsafe extern "C" fn call_get_items<F>(
        sess: *mut sr_session_ctx_t,
        sub_id: u32,
        mod_name: *const c_char,
        path: *const c_char,
        request_xpath: *const c_char,
        request_id: u32,
        parent: *mut *mut lyd_node,
        private_data: *mut c_void,
    ) -> i32
    where
        F: FnMut(&LibYangCtx, u32, &str, &str, Option<&str>, u32) -> Option<LydNode>,
    {
        let callback_ptr = private_data as *mut F;
        let callback = &mut *callback_ptr;

        let ctx = sr_acquire_context(sr_session_get_connection(sess));

        let mod_name = CStr::from_ptr(mod_name).to_str().unwrap();
        let path = CStr::from_ptr(path).to_str().unwrap();
        let request_xpath = if request_xpath == std::ptr::null_mut() {
            None
        } else {
            Some(CStr::from_ptr(request_xpath).to_str().unwrap())
        };

        let ctx = LibYangCtx::from(ctx);
        let node = callback(&ctx, sub_id, mod_name, path, request_xpath, request_id);

        match node {
            Some(node) => {
                *parent = node.get_node();
            }
            None => {}
        }

        sr_error_t_SR_ERR_OK as i32
    }

    /// Subscribe module change.
    pub fn module_change_subscribe<F>(
        &mut self,
        mod_name: &str,
        path: Option<&str>,
        callback: F,
        priority: u32,
        opts: sr_subscr_options_t,
    ) -> Result<&mut SrSubscr, i32>
    where
        F: FnMut(SrSession, u32, &str, Option<&str>, SrEvent, u32) -> () + 'static,
    {
        let mut subscr: *mut sr_subscription_ctx_t =
            unsafe { zeroed::<*mut sr_subscription_ctx_t>() };
        let data = Box::into_raw(Box::new(callback));
        let mod_name = mod_name.as_ptr() as *mut i8;
        let path = path.map_or(std::ptr::null_mut(), |path| path.as_ptr() as *mut i8);

        let rc = unsafe {
            sr_module_change_subscribe(
                self.sess,
                mod_name,
                path,
                Some(SrSession::call_module_change::<F>),
                data as *mut _,
                priority,
                opts,
                &mut subscr,
            )
        };

        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            let id = self.insert_subscription(SrSubscr::from(subscr));
            Ok(self.subscrs.get_mut(&id).unwrap())
        }
    }

    unsafe extern "C" fn call_module_change<F>(
        sess: *mut sr_session_ctx_t,
        sub_id: u32,
        mod_name: *const c_char,
        path: *const c_char,
        event: sr_event_t,
        request_id: u32,
        private_data: *mut c_void,
    ) -> i32
    where
        F: FnMut(SrSession, u32, &str, Option<&str>, SrEvent, u32) -> (),
    {
        let callback_ptr = private_data as *mut F;
        let callback = &mut *callback_ptr;

        let mod_name = CStr::from_ptr(mod_name).to_str().unwrap();
        let path = if path == std::ptr::null_mut() {
            None
        } else {
            Some(CStr::from_ptr(path).to_str().unwrap())
        };
        let event = SrEvent::try_from(event).expect("Convert error");
        let sess = SrSession::from(sess, false);

        callback(sess, sub_id, mod_name, path, event, request_id);

        sr_error_t_SR_ERR_OK as i32
    }

    /// Get changes iter.
    pub fn get_changes_iter(&self, path: &str) -> Result<SrChangeIter, i32> {
        let mut it = unsafe { zeroed::<*mut sr_change_iter_t>() };
        let rc = unsafe {
            let path = CString::new(path).unwrap();
            let path = path.as_ptr() as *const i8;

            sr_get_changes_iter(self.sess, path, &mut it)
        };

        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            Ok(SrChangeIter::from(it))
        }
    }

    /// Send event notify tree.
    pub fn notif_send_tree(
        &mut self,
        notif: &LydNode,
        timeout_ms: u32,
        wait: i32,
    ) -> Result<(), i32> {
        let rc = unsafe { sr_notif_send_tree(self.sess, notif.get_node(), timeout_ms, wait) };
        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            Ok(())
        }
    }

    /// Send RPC.
    pub fn rpc_send(
        &mut self,
        path: &str,
        input: Option<Vec<sr_val_t>>,
        timeout: Option<Duration>,
    ) -> Result<SrValueSlice, i32> {
        let path = path.as_ptr() as *mut i8;
        let (input, input_cnt) = match input {
            Some(mut input) => (input.as_mut_ptr(), input.len() as u64),
            None => (std::ptr::null_mut(), 0),
        };
        let timeout = timeout.map_or(0, |timeout| timeout.as_millis() as u32);

        let mut output: *mut sr_val_t = unsafe { zeroed::<*mut sr_val_t>() };
        let mut output_count: u64 = 0;

        let rc = unsafe {
            sr_rpc_send(
                self.sess,
                path,
                input,
                input_cnt,
                timeout,
                &mut output,
                &mut output_count,
            )
        };

        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            Ok(SrValueSlice::from(output, output_count, true))
        }
    }

    /// Return oper, old_value, new_value with next iter.
    pub fn get_change_next(
        &mut self,
        iter: &mut SrChangeIter,
    ) -> Option<(SrChangeOper, SrValue, SrValue)> {
        let mut oper: sr_change_oper_t = 0;
        let mut old_value: *mut sr_val_t = std::ptr::null_mut();
        let mut new_value: *mut sr_val_t = std::ptr::null_mut();

        let rc = unsafe {
            sr_get_change_next(
                self.sess,
                iter.iter(),
                &mut oper,
                &mut old_value,
                &mut new_value,
            )
        };

        if rc == SrError::Ok as i32 {
            match SrChangeOper::try_from(oper) {
                Ok(oper) => Some((oper, SrValue::from(old_value), SrValue::from(new_value))),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

impl Drop for SrSession {
    fn drop(&mut self) {
        if self.owned {
            self.subscrs.drain();

            unsafe {
                sr_session_stop(self.sess);
            }
        }
    }
}

/// Sysrepo Subscription.
pub struct SrSubscr {
    /// Raw Pointer to subscription.
    subscr: *mut sr_subscription_ctx_t,
}

impl SrSubscr {
    pub fn new() -> Self {
        Self {
            subscr: std::ptr::null_mut(),
        }
    }

    pub fn from(subscr: *mut sr_subscription_ctx_t) -> Self {
        Self { subscr: subscr }
    }

    pub fn id(&self) -> SrSubscrId {
        self.subscr
    }
}

impl Drop for SrSubscr {
    fn drop(&mut self) {
        unsafe {
            sr_unsubscribe(self.subscr);
        }
    }
}

/// Sysrepo Changes Iterator.
pub struct SrChangeIter {
    /// Raw pointer to iter.
    iter: *mut sr_change_iter_t,
}

impl SrChangeIter {
    pub fn from(iter: *mut sr_change_iter_t) -> Self {
        Self { iter: iter }
    }

    pub fn iter(&mut self) -> *mut sr_change_iter_t {
        self.iter
    }
}

impl Drop for SrChangeIter {
    fn drop(&mut self) {
        unsafe {
            sr_free_change_iter(self.iter);
        }
    }
}

/// Lib Yang Context.
///  It just holds raw pointer, but does not own the object.
pub struct LibYangCtx {
    /// Raw Pointer to Lib Yang Context.
    ly_ctx: *const ly_ctx,
}

impl LibYangCtx {
    /// Constructo from raw pointer.
    pub fn from(ly_ctx: *const ly_ctx) -> Self {
        Self { ly_ctx: ly_ctx }
    }

    pub fn get_ctx(&self) -> *const ly_ctx {
        self.ly_ctx
    }
}

/// LibYang data node.
pub struct LydNode {
    /// Raw pointer to LibYang data node.
    node: *mut lyd_node,

    // /// Value.
    // value: Option<LydValue>,
}

impl LydNode {
    pub fn from(node: *mut lyd_node) -> Self {
        Self {
            node: node,
            // value: None,
        }
    }

    pub fn get_node(&self) -> *mut lyd_node {
        self.node
    }

    pub fn free_all(&self) {
        unsafe {
            lyd_free_all(self.node);
        }
    }

    pub fn free_siblings(&self) {
        unsafe {
            lyd_free_siblings(self.node);
        }
    }

    pub fn free_tree(&self) {
        unsafe {
            lyd_free_tree(self.node);
        }
    }
}

/// LibYang data value.
pub struct LydValue {
    value_type: LydAnyDataValueType,

    /// TBD: It is string for now.
    ///      It has to be variable length of byte array.
    value: CString,
}

impl LydValue {
    pub fn from_string(s: String) -> Self {
        Self {
            value_type: LydAnyDataValueType::String,
            value: CString::new(s).unwrap(),
        }
    }

    pub fn get_value(&self) -> &CStr {
        &self.value
    }

    pub fn get_value_raw(&self) -> *mut c_void {
        self.get_value() as *const _ as *mut c_void
    }

    pub fn get_type(&self) -> LydAnyDataValueType {
        self.value_type
    }
}

/// Lib Yang Utilities.
pub struct LibYang {}

impl LibYang {
    pub fn lyd_new_path(
        parent: Option<&LydNode>,
        ly_ctx: Option<&LibYangCtx>,
        path: &str,
        value: Option<&LydValue>,
        options: u32,
    ) -> Result<LydNode, i32> {
        let parent = parent.map_or(std::ptr::null_mut(), |parent| parent.get_node());
        let ctx = ly_ctx.map_or(std::ptr::null_mut(), |ly_ctx| {
            ly_ctx.get_ctx() as *mut ly_ctx
        });
        let path = CString::new(path).unwrap();
        let path = path.as_ptr() as *const _ as *const i8;
        let mut node: *mut lyd_node = unsafe { zeroed::<*mut lyd_node>() };


        let val = match value {
            Some(value) => value.get_value().as_ptr(),
            None => std::ptr::null_mut(),
        };

        let rc = unsafe { lyd_new_path(parent, ctx, path, val, options, &mut node) };

        if rc != LY_ERR_LY_SUCCESS {
            Err(rc as i32) // FIXME: We should not cast like this
        } else {
            Ok(LydNode::from(node))
        }
    }
}
