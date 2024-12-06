use raylib_ffi::{enums::TraceLogLevel, rl_str, SetTraceLogLevel, TraceLog};

pub fn info(msg: &str) {
    unsafe {
        SetTraceLogLevel(TraceLogLevel::Info as i32);
        TraceLog(TraceLogLevel::Info as i32, rl_str!(msg));
        SetTraceLogLevel(TraceLogLevel::Error as i32);
    }
}
