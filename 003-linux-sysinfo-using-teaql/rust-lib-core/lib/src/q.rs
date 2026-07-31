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
    pub fn system_info() -> SystemInfoRequest {
        SystemInfoRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn system_info_minimal() -> SystemInfoRequest {
        SystemInfoRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn system_info_with_children() -> SystemInfoRequest {
        SystemInfoRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn processes() -> ProcessRequest {
        ProcessRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn processes_minimal() -> ProcessRequest {
        ProcessRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn processes_with_children() -> ProcessRequest {
        ProcessRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn threads() -> ThreadRequest {
        ThreadRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn threads_minimal() -> ThreadRequest {
        ThreadRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn threads_with_children() -> ThreadRequest {
        ThreadRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}