use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, SelectQuery, SmartList};
use teaql_runtime::RuntimeError;

use crate::request_support::*;

impl EntityReference for crate::DeviceSetting {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::DeviceSetting {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/device_setting
#[derive(Debug)]
pub struct DeviceSettingRequest<R = crate::DeviceSetting> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for DeviceSettingRequest<R> {
    fn clone(&self) -> Self {
        Self {
            query: self.query.clone(),
            relation_selections: self.relation_selections.clone(),
            relation_filters: self.relation_filters.clone(),
            child_enhancements: self.child_enhancements.clone(),
            query_options: self.query_options.clone(),
            marker: PhantomData,
        }
    }
}

impl<R> DeviceSettingRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("DeviceSetting")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> DeviceSettingRequest<T> {
        DeviceSettingRequest {
            query: self.query,
            relation_selections: self.relation_selections,
            relation_filters: self.relation_filters,
            child_enhancements: self.child_enhancements,
            query_options: self.query_options,
            marker: PhantomData,
        }
    }

    pub fn query(&self) -> &SelectQuery {
        &self.query
    }

    pub fn relation_selections(&self) -> &[RelationSelection] {
        &self.relation_selections
    }

    pub fn relation_filters(&self) -> &[RelationFilter] {
        &self.relation_filters
    }

    pub fn child_enhancements(&self) -> &[QuerySelection] {
        &self.child_enhancements
    }

    pub fn query_options(&self) -> &QueryOptions {
        &self.query_options
    }

    pub fn into_query(self) -> SelectQuery {
        self.query
    }


    pub fn purpose(self, purpose: impl Into<String>) -> crate::PurposedQuery<Self> {
        crate::PurposedQuery::new(self, purpose)
    }

    pub(crate) async fn _execute_for_list<'a, C>(
        self,
        context: &'a C,
    ) -> Result<SmartList<R>, RuntimeError>
    where
        C: TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        let query_options = self.query_options.clone();
        let relation_aggregates = runtime_relation_aggregates(&query_options);
        let query = authorize_query(apply_runtime_metadata(
            self.query,
            &query_options,
            &self.child_enhancements,
        ))?;
        let (mut rows, facets) = if query_options.facets.is_empty() {
            let rows = context.fetch_entity_smart_list::<R>(
                "DeviceSetting",
                query,
                relation_aggregates,
            ).await?;
            (rows, std::collections::BTreeMap::new())
        } else {
            let rows = context.fetch_entity_smart_list::<R>(
                "DeviceSetting",
                query.clone(),
                relation_aggregates,
            ).await?;
            let facets = execute_facets(context, query.as_query(), &query_options)
                .await?;
            (rows, facets)
        };
        attach_facets(&mut rows, facets);
        Ok(rows)
    }

    pub(crate) async fn _execute_for_rows<'a, C>(
        self,
        context: &'a C,
    ) -> Result<SmartList<teaql_core::CompactRow>, RuntimeError>
    where
        C: TeaqlRuntime + ?Sized,
    {
        let query = authorize_query(apply_runtime_metadata(
            self.query,
            &self.query_options,
            &self.child_enhancements,
        ))?;
        context.fetch_compact_smart_list("DeviceSetting", &query).await
    }

    pub(crate) async fn _execute_for_stream<'a, C>(
        self,
        context: &'a C,
    ) -> Result<TeaqlEntityStream<'a, R, RuntimeError>, RuntimeError>
    where
        C: TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        let query = authorize_query(apply_runtime_metadata(
            self.query,
            &self.query_options,
            &self.child_enhancements,
        ))?;
        Ok(context.fetch_entity_stream("DeviceSetting", query))
    }

    pub(crate) async fn _execute_for_first<'a, C>(
        self,
        context: &'a C,
    ) -> Result<Option<R>, RuntimeError>
    where
        C: TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        let rows = self.limit(1)._execute_for_list(context).await?;
        Ok(rows.into_iter().next())
    }

    pub(crate) async fn _execute_for_one<'a, C>(
        self,
        context: &'a C,
    ) -> Result<Option<R>, RuntimeError>
    where
        C: TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        self._execute_for_first(context).await
    }


    pub(crate) async fn _execute_for_page<'a, C>(
        self,
        context: &'a C,
        offset: u64,
        limit: u64,
    ) -> Result<SmartList<R>, RuntimeError>
    where
        C: TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        if self.query.id_set_pagination.is_some() {
            let mut rows = self
                .clone()
                .page_offset(offset, limit)
                ._execute_for_list(context)
                .await?;
            if rows.total_count.is_none() {
                rows.total_count = Some(self._execute_for_count(context).await?);
            }
            return Ok(rows);
        }
        let total_count = self.clone()._execute_for_count(context).await?;
        let mut rows = self.page_offset(offset, limit)._execute_for_list(context).await?;
        rows.total_count = Some(total_count);
        Ok(rows)
    }

    pub(crate) async fn _execute_for_count<'a, C>(
        self,
        context: &'a C,
    ) -> Result<u64, RuntimeError>
    where
        C: TeaqlRuntime + ?Sized,
    {
        let query_options = self.query_options.clone();
        let mut query = apply_runtime_metadata(
            self.query,
            &query_options,
            &self.child_enhancements,
        );
        query.projection.clear();
        query.expr_projection.clear();
        query.order_by.clear();
        query.slice = None;
        query.relations.clear();
        query = query.count(COUNT_ALIAS);
        let query = authorize_query(query)?;
        let rows = context.fetch_compact_rows("DeviceSetting", &query).await?;
        rows.first()
            .and_then(|row| row.get(COUNT_ALIAS))
            .and_then(teaql_core::Value::try_u64)
            .ok_or_else(|| RuntimeError::Graph(format!("count result for DeviceSetting is missing or not numeric")))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        context: &'a C,
    ) -> Result<bool, RuntimeError>
    where
        C: TeaqlRuntime + ?Sized,
    {
        let mut query = apply_runtime_metadata(
            self.query,
            &self.query_options,
            &self.child_enhancements,
        ).limit(1);
        query.relations.clear();
        let query = authorize_query(query)?;
        let rows = context.fetch_compact_rows("DeviceSetting", &query).await?;
        Ok(!rows.is_empty())
    }

    pub fn search_with_text(mut self, text: impl Into<String>) -> Self {
        self.query = self.query.search_with_text(text);
        self
    }

    pub fn filter(mut self, filter: Expr) -> Self {
        self.query = self.query.filter(filter);
        self
    }

    pub fn and_filter(mut self, filter: Expr) -> Self {
        self.query = self.query.and_filter(filter);
        self
    }

    pub fn or_filter(mut self, filter: Expr) -> Self {
        self.query = self.query.or_filter(filter);
        self
    }

    pub fn append_search_criteria(self, criteria: Expr) -> Self {
        self.and_filter(criteria)
    }

    pub fn filter_property(
        mut self,
        property1: impl AsRef<str>,
        operator: FieldOperator,
        property2: impl AsRef<str>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_column_expr(
            property1.as_ref(),
            operator,
            property2.as_ref(),
        ));
        self
    }

    pub fn with_deleted_rows(mut self) -> Self {
        self.query.filter = remove_default_live_filter(self.query.filter);
        self
    }

    pub fn deleted_rows_only(mut self) -> Self {
        self.query.filter = remove_default_live_filter(self.query.filter);
        self.query = self.query.and_filter(Expr::lte("version", 0_i64));
        self
    }

    pub fn match_types(
        mut self,
        types: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(TYPE_FIELD, types.into_iter().map(Into::into)));
        self
    }


    pub fn with_type_group(mut self) -> Self {
        self.query = self.query.project(TYPE_GROUP_FIELD);
        self
    }

    pub fn matching_any_of(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        let entity = EntityDescriptor::new(selection.query.entity.clone());
        self.query = self.query.and_filter(Expr::in_subquery("id", entity, selection.query.clone(), "id"));
        self
    }

    pub fn match_any_of(self, request: impl Into<QuerySelection>) -> Self {
        self.matching_any_of(request)
    }

    pub fn enhance_child(mut self, request: impl Into<QuerySelection>) -> Self {
        self.child_enhancements.push(request.into());
        self
    }

    pub fn enhance_children_if_needed(self) -> Self {
        let request = self;
        request
    }


    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.query_options.comment = Some(comment.into());
        self
    }

    pub fn raw_sql(self, raw_sql: impl Into<String>) -> Self {
        self.unsafe_raw_sql(UnsafeRawSqlSegment::trusted(raw_sql))
    }

    pub fn unsafe_raw_sql(mut self, raw_sql: UnsafeRawSqlSegment) -> Self {
        self.query_options.raw_sql = Some(raw_sql.into_sql());
        self
    }

    pub fn raw_sql_filter(self, raw_sql: impl Into<String>) -> Self {
        self.unsafe_raw_sql_filter(UnsafeRawSqlSegment::trusted(raw_sql))
    }

    pub fn unsafe_raw_sql_filter(mut self, raw_sql: UnsafeRawSqlSegment) -> Self {
        self.query_options.raw_sql_search_criteria.push(raw_sql.into_sql());
        self
    }
    pub fn filter_with_json(self, json_expr: impl Into<String>) -> Self {
        self.merge_dynamic_json_expr(json_expr.into())
    }

    fn merge_dynamic_json_expr(self, json_expr: String) -> Self {
        let json = serde_json::from_str::<JsonValue>(&json_expr)
            .unwrap_or_else(|_| panic!("Input JSON format error: {json_expr}"));
        self.merge_dynamic_json(&json)
    }

    fn merge_dynamic_json(mut self, json: &JsonValue) -> Self {
        let Some(object) = json.as_object() else {
            return self;
        };

        for (field, value) in object {
            if field.starts_with('_') {
                continue;
            }
            self = self.apply_dynamic_json_filter(field, value);
        }

        self = self.apply_dynamic_json_order_by(object.get("_orderBy"));

        if let Some(offset) = dynamic_json_u64_field(object, "_start") {
            self = self.skip(offset);
        }
        if let Some(size) = dynamic_json_u64_field(object, "_size") {
            self = self.limit(size);
        }

        if let Some(page_size) = dynamic_json_u64_field(object, "_pageSize") {
            self = self.limit(page_size);
        }
        if let Some(page_number) = dynamic_json_u64_field(object, "_page") {
            if page_number > 0 {
                let size = dynamic_json_u64_field(object, "_pageSize")
                    .or_else(|| self.query.slice.as_ref().and_then(|slice| slice.limit))
                    .unwrap_or(10);
                let offset = page_number.saturating_sub(1).saturating_mul(size);
                self = self.page_offset(offset, size);
            }
        }

        self
    }

    pub(crate) fn apply_dynamic_json_filter(self, field: &str, value: &JsonValue) -> Self {
        if let Some((head, tail)) = field.split_once('.') {
            self.apply_dynamic_json_chain_filter(head, tail, value)
        } else if let Some(storage_field) = Self::dynamic_json_self_field(field) {
            self.and_filter(dynamic_json_filter_expr(storage_field, value))
        } else {
            self
        }
    }

    fn apply_dynamic_json_order_by(mut self, order_by: Option<&JsonValue>) -> Self {
        match order_by {
            Some(JsonValue::String(field)) => {
                if let Some(storage_field) = Self::dynamic_json_self_field(field) {
                    self.query = self.query.order_desc(storage_field);
                }
            }
            Some(JsonValue::Object(order_by)) => {
                self = self.apply_dynamic_json_single_order_by(order_by);
            }
            Some(JsonValue::Array(order_bys)) => {
                for order_by in order_bys {
                    if let Some(order_by) = order_by.as_object() {
                        self = self.apply_dynamic_json_single_order_by(order_by);
                    }
                }
            }
            _ => {}
        }
        self
    }

    fn apply_dynamic_json_single_order_by(
        mut self,
        order_by: &serde_json::Map<String, JsonValue>,
    ) -> Self {
        let Some(field) = order_by.get("field").and_then(JsonValue::as_str) else {
            return self;
        };
        let Some(storage_field) = Self::dynamic_json_self_field(field) else {
            return self;
        };
        if order_by
            .get("useAsc")
            .and_then(JsonValue::as_bool)
            .unwrap_or(false)
        {
            self.query = self.query.order_asc(storage_field);
        } else {
            self.query = self.query.order_desc(storage_field);
        }
        self
    }

    fn dynamic_json_self_field(field: &str) -> Option<&'static str> {
        match field {
            "id" => Some("id"),
            "calibration_point" => Some("calibration_point"),
            "data_keep_days" => Some("data_keep_days"),
            "sampling_frequency" => Some("sampling_frequency"),
            "password_enabled" => Some("password_enabled"),
            "password_hash" => Some("password_hash"),
            "super_password_hash" => Some("super_password_hash"),
            "create_time" => Some("create_time"),
            "update_time" => Some("update_time"),
            "version" => Some("version"),
            "device_system" | "device_system_id" => Some("device_system_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "device_system" => {
                self.with_device_system_matching(
                    crate::Q::device_systems_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            _ => self,
        }
    }

    pub fn create_property_as(
        self,
        property_name: impl Into<String>,
        raw_sql_segment: impl Into<String>,
    ) -> Self {
        self.unsafe_create_property_as(property_name, UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn unsafe_create_property_as(
        mut self,
        property_name: impl Into<String>,
        raw_sql_segment: UnsafeRawSqlSegment,
    ) -> Self {
        self.query_options
            .dynamic_properties
            .push(RawDynamicProperty::new(property_name, raw_sql_segment));
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.query = self.query.limit(limit);
        self
    }

    pub fn stream(mut self, chunk_size: usize) -> Self {
        assert!(chunk_size > 0, "stream chunk size must be positive");
        self.query = self.query.stream(chunk_size);
        self
    }

    pub fn stream_default(mut self) -> Self {
        self.query = self.query.stream_default();
        self
    }

    pub fn skip(mut self, offset: u64) -> Self {
        self.query = self.query.offset(offset);
        self
    }

    pub fn offset_only(self, offset: u64) -> Self {
        self.skip(offset)
    }

    pub fn offset(self, offset: u64, size: u64) -> Self {
        self.page_offset(offset, size)
    }

    pub fn page_offset(mut self, offset: u64, limit: u64) -> Self {
        self.query = self.query.page(offset, limit);
        self
    }

    pub fn optimize_for_continuous_page_fetch(mut self) -> Self {
        self.query = self.query.optimize_for_continuous_page_fetch();
        self
    }

    pub fn optimize_for_continuous_page_fetch_with(
        mut self,
        namespace: impl Into<String>,
        ttl_seconds: u64,
    ) -> Self {
        self.query = self
            .query
            .optimize_for_continuous_page_fetch_with(namespace, ttl_seconds);
        self
    }

    pub fn optimize_pagination_with_id_set(mut self) -> Self {
        self.query = self.query.optimize_pagination_with_id_set();
        self
    }

    pub fn optimize_pagination_with_id_set_config(
        mut self,
        namespace: impl Into<String>,
        ttl_seconds: u64,
        max_ids: u64,
    ) -> Self {
        self.query = self
            .query
            .optimize_pagination_with_id_set_config(namespace, ttl_seconds, max_ids);
        self
    }

    /// Select bounded indexed probes for a per-parent Top-N relation only
    /// when the already-loaded parent count is at or below `threshold`.
    /// Passing zero explicitly selects the provider window plan.
    pub fn top_n_probe_parent_threshold(mut self, threshold: usize) -> Self {
        self.query = self.query.top_n_probe_parent_threshold(threshold);
        self
    }

    pub fn top(self, top_n: u64) -> Self {
        self.limit(top_n)
    }

    pub fn offset_size(self, offset: u64, size: u64) -> Self {
        self.offset(offset, size)
    }

    pub fn unlimited(mut self) -> Self {
        self.query.slice = None;
        self
    }

    pub fn page_number(self, page_number: u64, page_size: u64) -> Self {
        let offset = page_number.saturating_sub(1).saturating_mul(page_size);
        self.page_offset(offset, page_size)
    }

    pub fn page_number_default(self, page_number: u64) -> Self {
        self.page_number(page_number, 10)
    }

    pub fn page(self, page_number: u64, page_size: u64) -> Self {
        self.page_number(page_number, page_size)
    }

    pub fn page_default(self, page_number: u64) -> Self {
        self.page_number_default(page_number)
    }

    pub fn select_self(mut self) -> Self {
        self.query = self.query.project("id");
        self.query = self.query.project("calibration_point");
        self.query = self.query.project("data_keep_days");
        self.query = self.query.project("sampling_frequency");
        self.query = self.query.project("password_enabled");
        self.query = self.query.project("password_hash");
        self.query = self.query.project("super_password_hash");
        self.query = self.query.project("create_time");
        self.query = self.query.project("update_time");
        self.query = self.query.project("version");
        self.query = self.query.project("device_system_id");
        self
    }

    pub fn select_self_fields(self) -> Self {
        self.select_self()
    }

    pub fn select_self_without_parent(self) -> Self {
        self.select_self_fields()
    }

    pub fn select_all(self) -> Self {
        let mut request = self.select_self();
        request = request.select_device_system();
        request
    }

    pub fn select_children(self) -> Self {
        self.select_all()
    }

    pub fn select_any(self) -> Self {
        self.select_children()
    }

    pub fn group_by(mut self, field: impl Into<String>) -> Self {
        self.query = self.query.group_by(field);
        self
    }

    pub fn count(self) -> Self {
        self.count_as("count")
    }

    pub fn count_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count(alias)
    }

    pub fn aggregate_count(mut self, alias: impl Into<String>) -> Self {
        self.query = self.query.count(alias);
        self
    }

    pub fn aggregate_count_field(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.count_field(field, alias);
        self
    }

    pub fn aggregate_with_function(
        mut self,
        field: impl Into<String>,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.query = self.query.aggregate(Aggregate::new(function, field, alias));
        self
    }

    pub fn aggregate_sum(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.sum(field, alias);
        self
    }

    pub fn aggregate_avg(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.avg(field, alias);
        self
    }

    pub fn aggregate_min(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.min(field, alias);
        self
    }

    pub fn aggregate_max(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.max(field, alias);
        self
    }

    pub fn aggregate_stddev(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.stddev(field, alias);
        self
    }

    pub fn aggregate_stddev_pop(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.stddev_pop(field, alias);
        self
    }

    pub fn aggregate_var_samp(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.var_samp(field, alias);
        self
    }

    pub fn aggregate_var_pop(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.var_pop(field, alias);
        self
    }

    pub fn aggregate_bit_and(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.bit_and(field, alias);
        self
    }

    pub fn aggregate_bit_or(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.bit_or(field, alias);
        self
    }

    pub fn aggregate_bit_xor(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.bit_xor(field, alias);
        self
    }

    pub fn enable_aggregation_cache(mut self) -> Self {
        self.query = self.query.enable_aggregation_cache();
        self
    }

    pub fn enable_aggregation_cache_for(mut self, cache_expired_millis: u64) -> Self {
        self.query = self.query.enable_aggregation_cache_for(cache_expired_millis);
        self
    }

    pub fn propagate_aggregation_cache(mut self, cache_expired_millis: u64) -> Self {
        self.query = self.query.propagate_aggregation_cache(cache_expired_millis);
        self
    }

    pub fn group_by_id(self) -> Self {
        self.group_by("id")
    }

    pub fn group_by_id_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("id"));
        request
    }

    pub fn group_by_id_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("id")
            .aggregate_with_function("id", alias, function)
    }

    pub fn count_id(self) -> Self {
        self.count_id_as("id_count")
    }

    pub fn count_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("id", alias)
    }

    pub fn sum_id(self) -> Self {
        self.sum_id_as("sum_id")
    }

    pub fn sum_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("id", alias)
    }

    pub fn avg_id(self) -> Self {
        self.avg_id_as("avg_id")
    }

    pub fn avg_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("id", alias)
    }

    pub fn min_id(self) -> Self {
        self.min_id_as("min_id")
    }

    pub fn min_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("id", alias)
    }

    pub fn max_id(self) -> Self {
        self.max_id_as("max_id")
    }

    pub fn max_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("id", alias)
    }


    pub fn with_id(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "id",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_id_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "id",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_id_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("id", value));
        self
    }



    pub fn with_id_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("id", value));
        self
    }

    pub fn with_id_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_id_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn order_by_id_asc(mut self) -> Self {
        self.query = self.query.order_asc("id");
        self
    }

    pub fn order_by_id_desc(mut self) -> Self {
        self.query = self.query.order_desc("id");
        self
    }

    pub fn order_by_id_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("id");
        self
    }

    pub fn order_by_id_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("id");
        self
    }


    pub fn select_calibration_point(mut self) -> Self {
        self.query = self.query.project("calibration_point");
        self
    }

    pub fn project_calibration_point(self) -> Self {
        self.select_calibration_point()
    }

    pub fn select_calibration_point_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_calibration_point_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_calibration_point_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("calibration_point", raw_sql_segment));
        self
    }

    pub fn select_calibration_point_with_function(self, function: AggregateFunction) -> Self {
        self.select_calibration_point_as_with_function("calibration_point", function)
    }

    pub fn select_calibration_point_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("calibration_point", alias, function)
    }

    pub fn group_by_calibration_point(self) -> Self {
        self.group_by("calibration_point")
    }

    pub fn group_by_calibration_point_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("calibration_point");
        request.query = request
            .query
            .project_expr(alias, Expr::column("calibration_point"));
        request
    }

    pub fn group_by_calibration_point_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("calibration_point")
            .aggregate_with_function("calibration_point", alias, function)
    }

    pub fn count_calibration_point(self) -> Self {
        self.count_calibration_point_as("calibration_point_count")
    }

    pub fn count_calibration_point_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("calibration_point", alias)
    }

    pub fn sum_calibration_point(self) -> Self {
        self.sum_calibration_point_as("sum_calibration_point")
    }

    pub fn sum_calibration_point_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("calibration_point", alias)
    }

    pub fn avg_calibration_point(self) -> Self {
        self.avg_calibration_point_as("avg_calibration_point")
    }

    pub fn avg_calibration_point_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("calibration_point", alias)
    }

    pub fn min_calibration_point(self) -> Self {
        self.min_calibration_point_as("min_calibration_point")
    }

    pub fn min_calibration_point_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("calibration_point", alias)
    }

    pub fn max_calibration_point(self) -> Self {
        self.max_calibration_point_as("max_calibration_point")
    }

    pub fn max_calibration_point_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("calibration_point", alias)
    }

    pub fn standard_deviation_calibration_point(self) -> Self {
        self.standard_deviation_calibration_point_as("stdDev_calibration_point")
    }

    pub fn standard_deviation_calibration_point_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("calibration_point", alias)
    }

    pub fn square_root_of_population_standard_deviation_calibration_point(self) -> Self {
        self.square_root_of_population_standard_deviation_calibration_point_as("stdDevPop_calibration_point")
    }

    pub fn square_root_of_population_standard_deviation_calibration_point_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("calibration_point", alias)
    }

    pub fn sample_variance_calibration_point(self) -> Self {
        self.sample_variance_calibration_point_as("varSamp_calibration_point")
    }

    pub fn sample_variance_calibration_point_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("calibration_point", alias)
    }

    pub fn sample_population_variance_calibration_point(self) -> Self {
        self.sample_population_variance_calibration_point_as("varPop_calibration_point")
    }

    pub fn sample_population_variance_calibration_point_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("calibration_point", alias)
    }

    pub fn unselect_calibration_point(mut self) -> Self {
        self.query.projection.retain(|field| field != "calibration_point");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "calibration_point");
        self
    }


    pub fn with_calibration_point(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "calibration_point",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_calibration_point_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "calibration_point",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_calibration_point_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("calibration_point", value));
        self
    }



    pub fn with_calibration_point_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("calibration_point", value));
        self
    }

    pub fn with_calibration_point_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("calibration_point", value));
        self
    }

    pub fn with_calibration_point_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("calibration_point", value));
        self
    }

    pub fn with_calibration_point_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("calibration_point", value));
        self
    }

    pub fn with_calibration_point_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("calibration_point", value));
        self
    }

    pub fn with_calibration_point_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("calibration_point", lower, upper));
        self
    }

    pub fn with_calibration_point_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "calibration_point",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_calibration_point_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "calibration_point",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_calibration_point_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "calibration_point",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_calibration_point_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("calibration_point", value));
        self
    }

    pub fn with_calibration_point_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("calibration_point", value));
        self
    }

    pub fn with_calibration_point_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("calibration_point"));
        self
    }



    pub fn with_calibration_point_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("calibration_point"));
        self
    }


    pub fn order_by_calibration_point_asc(mut self) -> Self {
        self.query = self.query.order_asc("calibration_point");
        self
    }

    pub fn order_by_calibration_point_desc(mut self) -> Self {
        self.query = self.query.order_desc("calibration_point");
        self
    }

    pub fn order_by_calibration_point_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("calibration_point");
        self
    }

    pub fn order_by_calibration_point_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("calibration_point");
        self
    }


    pub fn select_data_keep_days(mut self) -> Self {
        self.query = self.query.project("data_keep_days");
        self
    }

    pub fn project_data_keep_days(self) -> Self {
        self.select_data_keep_days()
    }

    pub fn select_data_keep_days_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_data_keep_days_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_data_keep_days_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("data_keep_days", raw_sql_segment));
        self
    }

    pub fn select_data_keep_days_with_function(self, function: AggregateFunction) -> Self {
        self.select_data_keep_days_as_with_function("data_keep_days", function)
    }

    pub fn select_data_keep_days_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("data_keep_days", alias, function)
    }

    pub fn group_by_data_keep_days(self) -> Self {
        self.group_by("data_keep_days")
    }

    pub fn group_by_data_keep_days_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("data_keep_days");
        request.query = request
            .query
            .project_expr(alias, Expr::column("data_keep_days"));
        request
    }

    pub fn group_by_data_keep_days_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("data_keep_days")
            .aggregate_with_function("data_keep_days", alias, function)
    }

    pub fn count_data_keep_days(self) -> Self {
        self.count_data_keep_days_as("data_keep_days_count")
    }

    pub fn count_data_keep_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("data_keep_days", alias)
    }

    pub fn sum_data_keep_days(self) -> Self {
        self.sum_data_keep_days_as("sum_data_keep_days")
    }

    pub fn sum_data_keep_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("data_keep_days", alias)
    }

    pub fn avg_data_keep_days(self) -> Self {
        self.avg_data_keep_days_as("avg_data_keep_days")
    }

    pub fn avg_data_keep_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("data_keep_days", alias)
    }

    pub fn min_data_keep_days(self) -> Self {
        self.min_data_keep_days_as("min_data_keep_days")
    }

    pub fn min_data_keep_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("data_keep_days", alias)
    }

    pub fn max_data_keep_days(self) -> Self {
        self.max_data_keep_days_as("max_data_keep_days")
    }

    pub fn max_data_keep_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("data_keep_days", alias)
    }

    pub fn standard_deviation_data_keep_days(self) -> Self {
        self.standard_deviation_data_keep_days_as("stdDev_data_keep_days")
    }

    pub fn standard_deviation_data_keep_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("data_keep_days", alias)
    }

    pub fn square_root_of_population_standard_deviation_data_keep_days(self) -> Self {
        self.square_root_of_population_standard_deviation_data_keep_days_as("stdDevPop_data_keep_days")
    }

    pub fn square_root_of_population_standard_deviation_data_keep_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("data_keep_days", alias)
    }

    pub fn sample_variance_data_keep_days(self) -> Self {
        self.sample_variance_data_keep_days_as("varSamp_data_keep_days")
    }

    pub fn sample_variance_data_keep_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("data_keep_days", alias)
    }

    pub fn sample_population_variance_data_keep_days(self) -> Self {
        self.sample_population_variance_data_keep_days_as("varPop_data_keep_days")
    }

    pub fn sample_population_variance_data_keep_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("data_keep_days", alias)
    }

    pub fn unselect_data_keep_days(mut self) -> Self {
        self.query.projection.retain(|field| field != "data_keep_days");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "data_keep_days");
        self
    }


    pub fn with_data_keep_days(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "data_keep_days",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_data_keep_days_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "data_keep_days",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_data_keep_days_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("data_keep_days", value));
        self
    }



    pub fn with_data_keep_days_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("data_keep_days", value));
        self
    }

    pub fn with_data_keep_days_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("data_keep_days", value));
        self
    }

    pub fn with_data_keep_days_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("data_keep_days", value));
        self
    }

    pub fn with_data_keep_days_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("data_keep_days", value));
        self
    }

    pub fn with_data_keep_days_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("data_keep_days", value));
        self
    }

    pub fn with_data_keep_days_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("data_keep_days", lower, upper));
        self
    }

    pub fn with_data_keep_days_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "data_keep_days",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_data_keep_days_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "data_keep_days",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_data_keep_days_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "data_keep_days",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_data_keep_days_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("data_keep_days", value));
        self
    }

    pub fn with_data_keep_days_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("data_keep_days", value));
        self
    }

    pub fn with_data_keep_days_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("data_keep_days"));
        self
    }



    pub fn with_data_keep_days_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("data_keep_days"));
        self
    }


    pub fn order_by_data_keep_days_asc(mut self) -> Self {
        self.query = self.query.order_asc("data_keep_days");
        self
    }

    pub fn order_by_data_keep_days_desc(mut self) -> Self {
        self.query = self.query.order_desc("data_keep_days");
        self
    }

    pub fn order_by_data_keep_days_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("data_keep_days");
        self
    }

    pub fn order_by_data_keep_days_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("data_keep_days");
        self
    }


    pub fn select_sampling_frequency(mut self) -> Self {
        self.query = self.query.project("sampling_frequency");
        self
    }

    pub fn project_sampling_frequency(self) -> Self {
        self.select_sampling_frequency()
    }

    pub fn select_sampling_frequency_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_sampling_frequency_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_sampling_frequency_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("sampling_frequency", raw_sql_segment));
        self
    }

    pub fn select_sampling_frequency_with_function(self, function: AggregateFunction) -> Self {
        self.select_sampling_frequency_as_with_function("sampling_frequency", function)
    }

    pub fn select_sampling_frequency_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("sampling_frequency", alias, function)
    }

    pub fn group_by_sampling_frequency(self) -> Self {
        self.group_by("sampling_frequency")
    }

    pub fn group_by_sampling_frequency_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("sampling_frequency");
        request.query = request
            .query
            .project_expr(alias, Expr::column("sampling_frequency"));
        request
    }

    pub fn group_by_sampling_frequency_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("sampling_frequency")
            .aggregate_with_function("sampling_frequency", alias, function)
    }

    pub fn count_sampling_frequency(self) -> Self {
        self.count_sampling_frequency_as("sampling_frequency_count")
    }

    pub fn count_sampling_frequency_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("sampling_frequency", alias)
    }

    pub fn sum_sampling_frequency(self) -> Self {
        self.sum_sampling_frequency_as("sum_sampling_frequency")
    }

    pub fn sum_sampling_frequency_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("sampling_frequency", alias)
    }

    pub fn avg_sampling_frequency(self) -> Self {
        self.avg_sampling_frequency_as("avg_sampling_frequency")
    }

    pub fn avg_sampling_frequency_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("sampling_frequency", alias)
    }

    pub fn min_sampling_frequency(self) -> Self {
        self.min_sampling_frequency_as("min_sampling_frequency")
    }

    pub fn min_sampling_frequency_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("sampling_frequency", alias)
    }

    pub fn max_sampling_frequency(self) -> Self {
        self.max_sampling_frequency_as("max_sampling_frequency")
    }

    pub fn max_sampling_frequency_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("sampling_frequency", alias)
    }

    pub fn standard_deviation_sampling_frequency(self) -> Self {
        self.standard_deviation_sampling_frequency_as("stdDev_sampling_frequency")
    }

    pub fn standard_deviation_sampling_frequency_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("sampling_frequency", alias)
    }

    pub fn square_root_of_population_standard_deviation_sampling_frequency(self) -> Self {
        self.square_root_of_population_standard_deviation_sampling_frequency_as("stdDevPop_sampling_frequency")
    }

    pub fn square_root_of_population_standard_deviation_sampling_frequency_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("sampling_frequency", alias)
    }

    pub fn sample_variance_sampling_frequency(self) -> Self {
        self.sample_variance_sampling_frequency_as("varSamp_sampling_frequency")
    }

    pub fn sample_variance_sampling_frequency_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("sampling_frequency", alias)
    }

    pub fn sample_population_variance_sampling_frequency(self) -> Self {
        self.sample_population_variance_sampling_frequency_as("varPop_sampling_frequency")
    }

    pub fn sample_population_variance_sampling_frequency_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("sampling_frequency", alias)
    }

    pub fn unselect_sampling_frequency(mut self) -> Self {
        self.query.projection.retain(|field| field != "sampling_frequency");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "sampling_frequency");
        self
    }


    pub fn with_sampling_frequency(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "sampling_frequency",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_sampling_frequency_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "sampling_frequency",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_sampling_frequency_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("sampling_frequency", value));
        self
    }



    pub fn with_sampling_frequency_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("sampling_frequency", value));
        self
    }

    pub fn with_sampling_frequency_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("sampling_frequency", value));
        self
    }

    pub fn with_sampling_frequency_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("sampling_frequency", value));
        self
    }

    pub fn with_sampling_frequency_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("sampling_frequency", value));
        self
    }

    pub fn with_sampling_frequency_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("sampling_frequency", value));
        self
    }

    pub fn with_sampling_frequency_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("sampling_frequency", lower, upper));
        self
    }

    pub fn with_sampling_frequency_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "sampling_frequency",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_sampling_frequency_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "sampling_frequency",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_sampling_frequency_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "sampling_frequency",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_sampling_frequency_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("sampling_frequency", value));
        self
    }

    pub fn with_sampling_frequency_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("sampling_frequency", value));
        self
    }

    pub fn with_sampling_frequency_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("sampling_frequency"));
        self
    }



    pub fn with_sampling_frequency_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("sampling_frequency"));
        self
    }


    pub fn order_by_sampling_frequency_asc(mut self) -> Self {
        self.query = self.query.order_asc("sampling_frequency");
        self
    }

    pub fn order_by_sampling_frequency_desc(mut self) -> Self {
        self.query = self.query.order_desc("sampling_frequency");
        self
    }

    pub fn order_by_sampling_frequency_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("sampling_frequency");
        self
    }

    pub fn order_by_sampling_frequency_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("sampling_frequency");
        self
    }


    pub fn select_password_enabled(mut self) -> Self {
        self.query = self.query.project("password_enabled");
        self
    }

    pub fn project_password_enabled(self) -> Self {
        self.select_password_enabled()
    }

    pub fn select_password_enabled_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_password_enabled_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_password_enabled_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("password_enabled", raw_sql_segment));
        self
    }

    pub fn select_password_enabled_with_function(self, function: AggregateFunction) -> Self {
        self.select_password_enabled_as_with_function("password_enabled", function)
    }

    pub fn select_password_enabled_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("password_enabled", alias, function)
    }

    pub fn group_by_password_enabled(self) -> Self {
        self.group_by("password_enabled")
    }

    pub fn group_by_password_enabled_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("password_enabled");
        request.query = request
            .query
            .project_expr(alias, Expr::column("password_enabled"));
        request
    }

    pub fn group_by_password_enabled_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("password_enabled")
            .aggregate_with_function("password_enabled", alias, function)
    }

    pub fn count_password_enabled(self) -> Self {
        self.count_password_enabled_as("password_enabled_count")
    }

    pub fn count_password_enabled_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("password_enabled", alias)
    }

    pub fn sum_password_enabled(self) -> Self {
        self.sum_password_enabled_as("sum_password_enabled")
    }

    pub fn sum_password_enabled_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("password_enabled", alias)
    }

    pub fn avg_password_enabled(self) -> Self {
        self.avg_password_enabled_as("avg_password_enabled")
    }

    pub fn avg_password_enabled_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("password_enabled", alias)
    }

    pub fn min_password_enabled(self) -> Self {
        self.min_password_enabled_as("min_password_enabled")
    }

    pub fn min_password_enabled_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("password_enabled", alias)
    }

    pub fn max_password_enabled(self) -> Self {
        self.max_password_enabled_as("max_password_enabled")
    }

    pub fn max_password_enabled_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("password_enabled", alias)
    }

    pub fn standard_deviation_password_enabled(self) -> Self {
        self.standard_deviation_password_enabled_as("stdDev_password_enabled")
    }

    pub fn standard_deviation_password_enabled_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("password_enabled", alias)
    }

    pub fn square_root_of_population_standard_deviation_password_enabled(self) -> Self {
        self.square_root_of_population_standard_deviation_password_enabled_as("stdDevPop_password_enabled")
    }

    pub fn square_root_of_population_standard_deviation_password_enabled_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("password_enabled", alias)
    }

    pub fn sample_variance_password_enabled(self) -> Self {
        self.sample_variance_password_enabled_as("varSamp_password_enabled")
    }

    pub fn sample_variance_password_enabled_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("password_enabled", alias)
    }

    pub fn sample_population_variance_password_enabled(self) -> Self {
        self.sample_population_variance_password_enabled_as("varPop_password_enabled")
    }

    pub fn sample_population_variance_password_enabled_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("password_enabled", alias)
    }

    pub fn unselect_password_enabled(mut self) -> Self {
        self.query.projection.retain(|field| field != "password_enabled");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "password_enabled");
        self
    }


    pub fn with_password_enabled(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "password_enabled",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_password_enabled_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "password_enabled",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_password_enabled_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("password_enabled", value));
        self
    }



    pub fn with_password_enabled_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("password_enabled", value));
        self
    }

    pub fn with_password_enabled_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("password_enabled", value));
        self
    }

    pub fn with_password_enabled_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("password_enabled", value));
        self
    }

    pub fn with_password_enabled_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("password_enabled", value));
        self
    }

    pub fn with_password_enabled_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("password_enabled", value));
        self
    }

    pub fn with_password_enabled_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("password_enabled", lower, upper));
        self
    }

    pub fn with_password_enabled_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "password_enabled",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_password_enabled_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "password_enabled",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_password_enabled_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "password_enabled",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_password_enabled_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("password_enabled", value));
        self
    }

    pub fn with_password_enabled_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("password_enabled", value));
        self
    }

    pub fn with_password_enabled_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("password_enabled"));
        self
    }



    pub fn with_password_enabled_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("password_enabled"));
        self
    }


    pub fn order_by_password_enabled_asc(mut self) -> Self {
        self.query = self.query.order_asc("password_enabled");
        self
    }

    pub fn order_by_password_enabled_desc(mut self) -> Self {
        self.query = self.query.order_desc("password_enabled");
        self
    }

    pub fn order_by_password_enabled_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("password_enabled");
        self
    }

    pub fn order_by_password_enabled_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("password_enabled");
        self
    }


    pub fn select_password_hash(mut self) -> Self {
        self.query = self.query.project("password_hash");
        self
    }

    pub fn project_password_hash(self) -> Self {
        self.select_password_hash()
    }

    pub fn select_password_hash_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_password_hash_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_password_hash_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("password_hash", raw_sql_segment));
        self
    }

    pub fn group_by_password_hash(self) -> Self {
        self.group_by("password_hash")
    }

    pub fn group_by_password_hash_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("password_hash");
        request.query = request
            .query
            .project_expr(alias, Expr::column("password_hash"));
        request
    }

    pub fn group_by_password_hash_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("password_hash")
            .aggregate_with_function("password_hash", alias, function)
    }

    pub fn count_password_hash(self) -> Self {
        self.count_password_hash_as("password_hash_count")
    }

    pub fn count_password_hash_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("password_hash", alias)
    }

    pub fn sum_password_hash(self) -> Self {
        self.sum_password_hash_as("sum_password_hash")
    }

    pub fn sum_password_hash_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("password_hash", alias)
    }

    pub fn avg_password_hash(self) -> Self {
        self.avg_password_hash_as("avg_password_hash")
    }

    pub fn avg_password_hash_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("password_hash", alias)
    }

    pub fn min_password_hash(self) -> Self {
        self.min_password_hash_as("min_password_hash")
    }

    pub fn min_password_hash_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("password_hash", alias)
    }

    pub fn max_password_hash(self) -> Self {
        self.max_password_hash_as("max_password_hash")
    }

    pub fn max_password_hash_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("password_hash", alias)
    }

    pub fn unselect_password_hash(mut self) -> Self {
        self.query.projection.retain(|field| field != "password_hash");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "password_hash");
        self
    }


    pub fn with_password_hash(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "password_hash",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_password_hash_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "password_hash",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_password_hash_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("password_hash", value));
        self
    }



    pub fn with_password_hash_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("password_hash", value));
        self
    }

    pub fn with_password_hash_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("password_hash", value));
        self
    }

    pub fn with_password_hash_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("password_hash", value));
        self
    }

    pub fn with_password_hash_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("password_hash", value));
        self
    }

    pub fn with_password_hash_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("password_hash", value));
        self
    }

    pub fn with_password_hash_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("password_hash", lower, upper));
        self
    }

    pub fn with_password_hash_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "password_hash",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_password_hash_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "password_hash",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_password_hash_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "password_hash",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_password_hash_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("password_hash", value));
        self
    }

    pub fn with_password_hash_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("password_hash", value));
        self
    }

    pub fn with_password_hash_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("password_hash", value));
        self
    }

    pub fn with_password_hash_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("password_hash", value));
        self
    }

    pub fn with_password_hash_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("password_hash", value));
        self
    }

    pub fn with_password_hash_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("password_hash", value));
        self
    }

    pub fn with_password_hash_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("password_hash", value));
        self
    }
    pub fn with_password_hash_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("password_hash", value));
        self
    }

    pub fn with_password_hash_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("password_hash", value));
        self
    }

    pub fn with_password_hash_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("password_hash"));
        self
    }



    pub fn with_password_hash_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("password_hash"));
        self
    }


    pub fn order_by_password_hash_asc(mut self) -> Self {
        self.query = self.query.order_asc("password_hash");
        self
    }

    pub fn order_by_password_hash_desc(mut self) -> Self {
        self.query = self.query.order_desc("password_hash");
        self
    }

    pub fn order_by_password_hash_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("password_hash");
        self
    }

    pub fn order_by_password_hash_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("password_hash");
        self
    }


    pub fn select_super_password_hash(mut self) -> Self {
        self.query = self.query.project("super_password_hash");
        self
    }

    pub fn project_super_password_hash(self) -> Self {
        self.select_super_password_hash()
    }

    pub fn select_super_password_hash_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_super_password_hash_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_super_password_hash_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("super_password_hash", raw_sql_segment));
        self
    }

    pub fn group_by_super_password_hash(self) -> Self {
        self.group_by("super_password_hash")
    }

    pub fn group_by_super_password_hash_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("super_password_hash");
        request.query = request
            .query
            .project_expr(alias, Expr::column("super_password_hash"));
        request
    }

    pub fn group_by_super_password_hash_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("super_password_hash")
            .aggregate_with_function("super_password_hash", alias, function)
    }

    pub fn count_super_password_hash(self) -> Self {
        self.count_super_password_hash_as("super_password_hash_count")
    }

    pub fn count_super_password_hash_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("super_password_hash", alias)
    }

    pub fn sum_super_password_hash(self) -> Self {
        self.sum_super_password_hash_as("sum_super_password_hash")
    }

    pub fn sum_super_password_hash_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("super_password_hash", alias)
    }

    pub fn avg_super_password_hash(self) -> Self {
        self.avg_super_password_hash_as("avg_super_password_hash")
    }

    pub fn avg_super_password_hash_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("super_password_hash", alias)
    }

    pub fn min_super_password_hash(self) -> Self {
        self.min_super_password_hash_as("min_super_password_hash")
    }

    pub fn min_super_password_hash_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("super_password_hash", alias)
    }

    pub fn max_super_password_hash(self) -> Self {
        self.max_super_password_hash_as("max_super_password_hash")
    }

    pub fn max_super_password_hash_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("super_password_hash", alias)
    }

    pub fn unselect_super_password_hash(mut self) -> Self {
        self.query.projection.retain(|field| field != "super_password_hash");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "super_password_hash");
        self
    }


    pub fn with_super_password_hash(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "super_password_hash",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_super_password_hash_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "super_password_hash",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_super_password_hash_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("super_password_hash", value));
        self
    }



    pub fn with_super_password_hash_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("super_password_hash", value));
        self
    }

    pub fn with_super_password_hash_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("super_password_hash", value));
        self
    }

    pub fn with_super_password_hash_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("super_password_hash", value));
        self
    }

    pub fn with_super_password_hash_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("super_password_hash", value));
        self
    }

    pub fn with_super_password_hash_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("super_password_hash", value));
        self
    }

    pub fn with_super_password_hash_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("super_password_hash", lower, upper));
        self
    }

    pub fn with_super_password_hash_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "super_password_hash",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_super_password_hash_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "super_password_hash",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_super_password_hash_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "super_password_hash",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_super_password_hash_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("super_password_hash", value));
        self
    }

    pub fn with_super_password_hash_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("super_password_hash", value));
        self
    }

    pub fn with_super_password_hash_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("super_password_hash", value));
        self
    }

    pub fn with_super_password_hash_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("super_password_hash", value));
        self
    }

    pub fn with_super_password_hash_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("super_password_hash", value));
        self
    }

    pub fn with_super_password_hash_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("super_password_hash", value));
        self
    }

    pub fn with_super_password_hash_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("super_password_hash", value));
        self
    }
    pub fn with_super_password_hash_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("super_password_hash", value));
        self
    }

    pub fn with_super_password_hash_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("super_password_hash", value));
        self
    }

    pub fn with_super_password_hash_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("super_password_hash"));
        self
    }



    pub fn with_super_password_hash_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("super_password_hash"));
        self
    }


    pub fn order_by_super_password_hash_asc(mut self) -> Self {
        self.query = self.query.order_asc("super_password_hash");
        self
    }

    pub fn order_by_super_password_hash_desc(mut self) -> Self {
        self.query = self.query.order_desc("super_password_hash");
        self
    }

    pub fn order_by_super_password_hash_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("super_password_hash");
        self
    }

    pub fn order_by_super_password_hash_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("super_password_hash");
        self
    }


    pub fn select_create_time(mut self) -> Self {
        self.query = self.query.project("create_time");
        self
    }

    pub fn project_create_time(self) -> Self {
        self.select_create_time()
    }

    pub fn select_create_time_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_create_time_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_create_time_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("create_time", raw_sql_segment));
        self
    }

    pub fn group_by_create_time(self) -> Self {
        self.group_by("create_time")
    }

    pub fn group_by_create_time_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("create_time");
        request.query = request
            .query
            .project_expr(alias, Expr::column("create_time"));
        request
    }

    pub fn group_by_create_time_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("create_time")
            .aggregate_with_function("create_time", alias, function)
    }

    pub fn count_create_time(self) -> Self {
        self.count_create_time_as("create_time_count")
    }

    pub fn count_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("create_time", alias)
    }

    pub fn sum_create_time(self) -> Self {
        self.sum_create_time_as("sum_create_time")
    }

    pub fn sum_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("create_time", alias)
    }

    pub fn avg_create_time(self) -> Self {
        self.avg_create_time_as("avg_create_time")
    }

    pub fn avg_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("create_time", alias)
    }

    pub fn min_create_time(self) -> Self {
        self.min_create_time_as("min_create_time")
    }

    pub fn min_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("create_time", alias)
    }

    pub fn max_create_time(self) -> Self {
        self.max_create_time_as("max_create_time")
    }

    pub fn max_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("create_time", alias)
    }

    pub fn unselect_create_time(mut self) -> Self {
        self.query.projection.retain(|field| field != "create_time");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "create_time");
        self
    }


    pub fn with_create_time(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "create_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_create_time_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "create_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_create_time_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("create_time", value));
        self
    }



    pub fn with_create_time_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("create_time", value));
        self
    }

    pub fn with_create_time_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("create_time", value));
        self
    }

    pub fn with_create_time_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("create_time", value));
        self
    }

    pub fn with_create_time_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("create_time", value));
        self
    }

    pub fn with_create_time_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("create_time", value));
        self
    }

    pub fn with_create_time_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("create_time", lower, upper));
        self
    }

    pub fn with_create_time_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "create_time",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_create_time_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "create_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_create_time_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "create_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_create_time_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("create_time", value));
        self
    }

    pub fn with_create_time_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("create_time", value));
        self
    }

    pub fn with_create_time_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("create_time"));
        self
    }



    pub fn with_create_time_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("create_time"));
        self
    }


    pub fn order_by_create_time_asc(mut self) -> Self {
        self.query = self.query.order_asc("create_time");
        self
    }

    pub fn order_by_create_time_desc(mut self) -> Self {
        self.query = self.query.order_desc("create_time");
        self
    }

    pub fn order_by_create_time_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("create_time");
        self
    }

    pub fn order_by_create_time_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("create_time");
        self
    }


    pub fn select_update_time(mut self) -> Self {
        self.query = self.query.project("update_time");
        self
    }

    pub fn project_update_time(self) -> Self {
        self.select_update_time()
    }

    pub fn select_update_time_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_update_time_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_update_time_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("update_time", raw_sql_segment));
        self
    }

    pub fn group_by_update_time(self) -> Self {
        self.group_by("update_time")
    }

    pub fn group_by_update_time_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("update_time");
        request.query = request
            .query
            .project_expr(alias, Expr::column("update_time"));
        request
    }

    pub fn group_by_update_time_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("update_time")
            .aggregate_with_function("update_time", alias, function)
    }

    pub fn count_update_time(self) -> Self {
        self.count_update_time_as("update_time_count")
    }

    pub fn count_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("update_time", alias)
    }

    pub fn sum_update_time(self) -> Self {
        self.sum_update_time_as("sum_update_time")
    }

    pub fn sum_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("update_time", alias)
    }

    pub fn avg_update_time(self) -> Self {
        self.avg_update_time_as("avg_update_time")
    }

    pub fn avg_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("update_time", alias)
    }

    pub fn min_update_time(self) -> Self {
        self.min_update_time_as("min_update_time")
    }

    pub fn min_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("update_time", alias)
    }

    pub fn max_update_time(self) -> Self {
        self.max_update_time_as("max_update_time")
    }

    pub fn max_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("update_time", alias)
    }

    pub fn unselect_update_time(mut self) -> Self {
        self.query.projection.retain(|field| field != "update_time");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "update_time");
        self
    }


    pub fn with_update_time(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "update_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_update_time_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "update_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_update_time_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("update_time", value));
        self
    }



    pub fn with_update_time_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("update_time", value));
        self
    }

    pub fn with_update_time_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("update_time", value));
        self
    }

    pub fn with_update_time_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("update_time", value));
        self
    }

    pub fn with_update_time_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("update_time", value));
        self
    }

    pub fn with_update_time_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("update_time", value));
        self
    }

    pub fn with_update_time_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("update_time", lower, upper));
        self
    }

    pub fn with_update_time_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "update_time",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_update_time_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "update_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_update_time_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "update_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_update_time_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("update_time", value));
        self
    }

    pub fn with_update_time_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("update_time", value));
        self
    }

    pub fn with_update_time_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("update_time"));
        self
    }



    pub fn with_update_time_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("update_time"));
        self
    }


    pub fn order_by_update_time_asc(mut self) -> Self {
        self.query = self.query.order_asc("update_time");
        self
    }

    pub fn order_by_update_time_desc(mut self) -> Self {
        self.query = self.query.order_desc("update_time");
        self
    }

    pub fn order_by_update_time_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("update_time");
        self
    }

    pub fn order_by_update_time_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("update_time");
        self
    }

    pub fn group_by_version(self) -> Self {
        self.group_by("version")
    }

    pub fn group_by_version_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("version");
        request.query = request
            .query
            .project_expr(alias, Expr::column("version"));
        request
    }

    pub fn group_by_version_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("version")
            .aggregate_with_function("version", alias, function)
    }

    pub fn count_version(self) -> Self {
        self.count_version_as("version_count")
    }

    pub fn count_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("version", alias)
    }

    pub fn sum_version(self) -> Self {
        self.sum_version_as("sum_version")
    }

    pub fn sum_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("version", alias)
    }

    pub fn avg_version(self) -> Self {
        self.avg_version_as("avg_version")
    }

    pub fn avg_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("version", alias)
    }

    pub fn min_version(self) -> Self {
        self.min_version_as("min_version")
    }

    pub fn min_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("version", alias)
    }

    pub fn max_version(self) -> Self {
        self.max_version_as("max_version")
    }

    pub fn max_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("version", alias)
    }

    pub fn order_by_version_asc(mut self) -> Self {
        self.query = self.query.order_asc("version");
        self
    }

    pub fn order_by_version_desc(mut self) -> Self {
        self.query = self.query.order_desc("version");
        self
    }

    pub fn order_by_version_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("version");
        self
    }

    pub fn order_by_version_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("version");
        self
    }
    pub fn filter_by_device_system(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("device_system_id", value.entity_id_value()));
        self
    }

    pub fn with_device_system_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "device_system_id",
            <crate::DeviceSystem as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("device_system", selection));
        self
    }


    pub fn without_device_system_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "device_system_id",
            <crate::DeviceSystem as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("device_system", selection));
        self
    }


    pub fn have_device_system(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("device_system_id"));
        self
    }

    pub fn have_no_device_system(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("device_system_id"));
        self
    }


    pub fn group_by_device_system(self) -> Self {
        self.group_by("device_system_id")
    }

    pub fn group_by_device_system_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("device_system_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("device_system_id"));
        request
    }

    pub fn group_by_device_system_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("device_system_id")
            .aggregate_with_function("device_system_id", alias, function)
    }

    pub fn group_by_device_system_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("device_system_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "device_system",
            "device_system_id",
            request,
        ));
        self
    }

    pub fn group_by_device_system_with_details(self) -> Self {
        self.group_by_device_system_with_details_from(crate::Q::device_systems().unlimited())
    }

    pub fn group_by_device_system_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_device_system_with(request)
    }


    pub fn roll_up_to_device_system(self) -> Self {
        self.roll_up_to_device_system_with(crate::Q::device_systems().unlimited())
    }

    pub fn roll_up_to_device_system_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_device_system_matching(selection.clone())
            .group_by_device_system_with(selection)
    }

    pub fn count_device_system(self) -> Self {
        self.count_device_system_as("device_system_count")
    }

    pub fn count_device_system_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("device_system_id", alias)
    }

    pub fn unselect_device_system(mut self) -> Self {
        self.query.projection.retain(|field| field != "device_system_id");
        self.query.relations.retain(|relation| relation.name != "device_system");
        self
    }
    pub fn select_device_system(mut self) -> Self {
        self.query = self.query.relation("device_system");
        self
    }

    pub fn select_device_system_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("device_system", selection.into_query());
        self
}

    pub fn facet_by_device_system_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_device_system_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_device_system_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "device_system",
            request,
            include_all_facets,
        ));
        self
    }
}

impl<R> Default for DeviceSettingRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< DeviceSettingRequest<R> > for SelectQuery {
    fn from(request: DeviceSettingRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< DeviceSettingRequest<R> > for QuerySelection {
    fn from(request: DeviceSettingRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::DeviceSetting> 
where C: crate::TeaqlRuntime + ?Sized + 'a
{
    type Error = teaql_runtime::RuntimeError;
    type Entity = crate::DeviceSetting;
    fn save(self, context: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Entity, Self::Error>> + Send + '_>> {
        Box::pin(async move {
            context.save_audited_entity(self).await
        })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<DeviceSettingRequest<R>> {
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.inner.query_options.comment = Some(comment.into());
        self
    }

    pub fn new_entity<C>(&self, context: &C) -> crate::DeviceSetting
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        self.require_comment();
        let mut entity = crate::DeviceSetting::runtime_new(context.user_context().entity_runtime_state());
        if let Ok(id) = context.user_context().next_id(crate::DeviceSetting::ENTITY_NAME) {
            entity.update_id(id);
        }
        teaql_core::Entity::mark_as_new(&mut entity);
        entity
    }

    fn into_inner_with_trace(mut self) -> DeviceSettingRequest<R> {
        self.require_comment();
        self.inner.query.trace_chain.push(teaql_core::TraceNode::typed(
            teaql_core::TraceKind::Purpose,
            self.inner.query.entity.clone(),
            None,
            self.purpose,
        ));
        self.inner
    }

    fn require_comment(&self) {
        assert!(
            self.inner
                .query_options
                .comment
                .as_deref()
                .is_some_and(|comment| !comment.trim().is_empty()),
            "query comment must not be empty"
        );
    }

    pub async fn execute_for_page<'a, C>(
        self,
        context: &'a C,
        offset: u64,
        limit: u64,
    ) -> Result<teaql_core::SmartList<R>, teaql_runtime::RuntimeError>
    where
        C: crate::TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        self.into_inner_with_trace()._execute_for_page(context, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        context: &'a C,
    ) -> Result<bool, teaql_runtime::RuntimeError>
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(context).await
    }

    pub async fn execute_for_list<'a, C>(self, context: &'a C) -> Result<teaql_core::SmartList<R>, teaql_runtime::RuntimeError>
    where
        C: crate::TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        self.into_inner_with_trace()._execute_for_list(context).await
    }

    pub async fn execute_for_rows<'a, C>(self, context: &'a C) -> Result<teaql_core::SmartList<teaql_core::CompactRow>, teaql_runtime::RuntimeError>
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_rows(context).await
    }

    /// Execute query as a lazy entity stream without materializing the result set.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, context: &'a C) -> Result<crate::request_support::TeaqlEntityStream<'a, R, teaql_runtime::RuntimeError>, teaql_runtime::RuntimeError>
    where
        C: crate::TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        self.into_inner_with_trace()._execute_for_stream(context).await
    }

    pub async fn execute_for_first<'a, C>(self, context: &'a C) -> Result<Option<R>, teaql_runtime::RuntimeError>
    where
        C: crate::TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        self.into_inner_with_trace()._execute_for_first(context).await
    }

    pub async fn execute_for_one<'a, C>(self, context: &'a C) -> Result<Option<R>, teaql_runtime::RuntimeError>
    where
        C: crate::TeaqlRuntime + ?Sized,
        R: teaql_core::Entity + Send + 'a,
    {
        self.into_inner_with_trace()._execute_for_one(context).await
    }


    pub async fn execute_for_count<'a, C>(self, context: &'a C) -> Result<u64, teaql_runtime::RuntimeError>
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(context).await
    }
}
