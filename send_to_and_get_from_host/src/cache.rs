use std::collections::HashMap;
use serde_json;
use log::debug;
use serde::{Serialize, Deserialize};
use std::marker::PhantomData;

#[derive(Serialize, Deserialize)]
pub struct UsageReportCache<'urc> {
    // Key will of type: ServiceID_AppID
    cache: HashMap<String,UsageReports>,
    a: PhantomData<&'urc str>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct UsageReports {
    // Key will of type: Metric
    reports: HashMap<String, UsageReport>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct UsageReport {
    current_value: i32,
    max_value: i32,
    period_type: String, // Change this to an enum
    period_start: i64,
    period_end: i64,
}

impl UsageReports {
    pub fn new() -> UsageReports {
        UsageReports {reports: HashMap::new()}
    }

    pub fn insert_report(self: &mut Self, metric: String, report: UsageReport){
        self.reports.insert(metric,report);
    }
}


impl UsageReport {
    pub fn sample_report() -> UsageReport {
        return UsageReport {
            current_value: 0, max_value: 3, period_type: "minute".to_string(),
            period_start:0, period_end:2
        };
    }
}


impl <'urc> UsageReportCache<'urc> {
    pub fn get(this: *mut Self, key: &str) -> Option<&'urc UsageReports> {
        unsafe {
            if let Some(_report) = (*this).cache.get(key) {
                return (*this).cache.get(key);
            }
        }
        return Self::get_data_from_host(key)
    }

    fn get_data_from_host(key: &str) -> Option<&'urc UsageReports> {
        match proxy_wasm::hostcalls::get_shared_data(key) {
            Ok((data_option,_cas)) => {
                match data_option {
                    Some(ref data) => {
                        debug!("This is the data from host: {:?}", std::str::from_utf8(data).unwrap());
                        // convert the shared data to UsageReport and return it back
                    },
                    None => debug!("Data option is empty!")
                }
            },
            Err(_e) => debug!("Error when getting trying to data from host"),
        };
        None
    }

    pub fn set(this: *mut Self, key: String, report: UsageReports){
        unsafe {
            let key_dup = key.clone(); // Because of some weird "move" error
            (*this).cache.insert(key,report);
            Self::send_data_to_host(this,key_dup);
        }
    }

    fn send_data_to_host(this: *const Self, key: String){
        unsafe {
            let contents_of_struct = Self::to_string(&(*this)).unwrap_or("NOTHING!".to_string());
            debug!("Sending {} to host", contents_of_struct);

            match proxy_wasm::hostcalls::set_shared_data(&key,Some(Self::to_string(&(*this)).unwrap().as_bytes()),None) {
                Err(status) => debug!("Status recieved from the host: {:?}", status),
                Ok(_) => { debug!("No problem from the host!")},
            }
        }
    }

    fn to_string(self: &Self) -> Result<String,serde_json::Error> {
        return serde_json::to_string(self);
    }

    pub fn new() -> UsageReportCache<'urc> {
        UsageReportCache {cache: HashMap::new(), a: PhantomData}
    }
}
