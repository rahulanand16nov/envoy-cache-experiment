use log::debug;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::collections::HashMap;
mod cache;

type URC<'a> = cache::UsageReportCache<'a>;

//static mut cache_pt: *mut URC = ptr::null_mut();

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Debug);
    debug!("In START!");
}

struct CacheRootContext {
    cache: HashMap<String,UsageReports>,
}


impl Context for CacheRootContext {}

impl RootContext for CacheRootContext {

    fn on_configure(&mut self, _plugin_configuration_size: usize) -> bool {
        self.cache = HashMap::new();
        return true;
    }

    // NOTE!!!!!!!!!!!!!!  ONLY WAY TO HAVE SOMETHING IN COMMON AMONG DIFFERENT THREADS IS ONLY THROUGH SHARED MEMORY !!!!!!!!!!!!!!!!!!

    fn create_http_context(&self, c_id: u32) -> Option<Box<dyn HttpContext>> {
        return Some(Box::new(CacheAuthorizer { context_id: c_id, cache_ref:}));
    }
}

struct CacheAuthorizer {
    context_id: u32,
    cache_ref: &mut HashMap<String,UsageReports>,
}

impl Context for CacheAuthorizer {}

impl HttpContext for CacheAuthorizer {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        // Need to find a better way for this to be globally available or be able to pass around after first initialization
        //unsafe {if cache_pt.is_null() { cache_pt = &mut URC::new();}}
        unsafe {
            if !self.cache_pt.is_null() {
                debug!("afksjafklajf: {:?}",(*self.cache_pt).cache.len());
            }
        }
        //let cache_pt = &mut URC::new();
        match self.get_http_request_header("key") {
            Some(key) => match URC::get(self.cache_pt,&key) { // check the key is correct first 
                Some(_report) => debug!("Got some report for you :)"),
                None => {
                    let mut test_reports = cache::UsageReports::new();
                    test_reports.insert_report("hits".to_string(),cache::UsageReport::sample_report());
                    URC::set(self.cache_pt,key,test_reports);
                    debug!("Found nothing! I even checked the host :(")
                },
            },
            None => debug!("Provide some key, sir!"),
        }
        Action::Continue
    }
}
