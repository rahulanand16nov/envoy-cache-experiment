use log::debug;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
mod cache;

type URC<'a> = cache::UsageReportCache<'a>;

//static mut cache_pt: *mut URC = ptr::null_mut();

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Debug);
    debug!("In START!");
    proxy_wasm::set_http_context(|context_id, _| -> Box<dyn HttpContext> {
        Box::new(CacheAuthorizer { context_id })
    });
}

struct CacheAuthorizer {
    context_id: u32,
}

impl Context for CacheAuthorizer {}

impl HttpContext for CacheAuthorizer {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        // Need to find a better way for this to be globally available or be able to pass around after first initialization
        //unsafe {if cache_pt.is_null() { cache_pt = &mut URC::new();}}
        let cache_pt = &mut URC::new();
        match self.get_http_request_header("key") {
            Some(key) => match URC::get(cache_pt,&key) { // check the key is correct first 
                Some(_report) => debug!("Got some report for you :)"),
                None => {
                    let mut test_reports = cache::UsageReports::new();
                    test_reports.insert_report("hits".to_string(),cache::UsageReport::sample_report());
                    URC::set(cache_pt,key,test_reports);
                    debug!("Found nothing! I even checked the host :(")
                },
            },
            None => debug!("Provide some key, sir!"),
        }
        Action::Continue
    }
}
