use teaql_core::Expr;

use crate::*;

pub struct PurposedQuery<T> {
    pub inner: T,
    pub purpose: String,
}

impl<T> PurposedQuery<T> {
    pub fn new(inner: T, purpose: impl Into<String>) -> Self {
        Self { inner, purpose: purpose.into() }
    }
}

pub struct Q;

impl Q {
    pub fn device_systems() -> DeviceSystemRequest {
        DeviceSystemRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn device_systems_minimal() -> DeviceSystemRequest {
        DeviceSystemRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn device_systems_with_children() -> DeviceSystemRequest {
        DeviceSystemRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn system_statuses() -> SystemStatusRequest {
        SystemStatusRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn system_statuses_minimal() -> SystemStatusRequest {
        SystemStatusRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn system_statuses_with_children() -> SystemStatusRequest {
        SystemStatusRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn device_settings() -> DeviceSettingRequest {
        DeviceSettingRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn device_settings_minimal() -> DeviceSettingRequest {
        DeviceSettingRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn device_settings_with_children() -> DeviceSettingRequest {
        DeviceSettingRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn sample_records() -> SampleRecordRequest {
        SampleRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sample_records_minimal() -> SampleRecordRequest {
        SampleRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn sample_records_with_children() -> SampleRecordRequest {
        SampleRecordRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}