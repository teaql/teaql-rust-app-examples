use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, SelectQuery, SmartList};
use teaql_runtime::RuntimeError;

use crate::request_support::*;

impl EntityReference for crate::SampleRecord {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::SampleRecord {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/sample_record
#[derive(Debug)]
pub struct SampleRecordRequest<R = crate::SampleRecord> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for SampleRecordRequest<R> {
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

impl<R> SampleRecordRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("SampleRecord")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> SampleRecordRequest<T> {
        SampleRecordRequest {
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
                "SampleRecord",
                query,
                relation_aggregates,
            ).await?;
            (rows, std::collections::BTreeMap::new())
        } else {
            let rows = context.fetch_entity_smart_list::<R>(
                "SampleRecord",
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
        context.fetch_compact_smart_list("SampleRecord", &query).await
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
        Ok(context.fetch_entity_stream("SampleRecord", query))
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
        let rows = context.fetch_compact_rows("SampleRecord", &query).await?;
        rows.first()
            .and_then(|row| row.get(COUNT_ALIAS))
            .and_then(teaql_core::Value::try_u64)
            .ok_or_else(|| RuntimeError::Graph(format!("count result for SampleRecord is missing or not numeric")))
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
        let rows = context.fetch_compact_rows("SampleRecord", &query).await?;
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
            "sample_time" => Some("sample_time"),
            "gas" => Some("gas"),
            "lref" => Some("lref"),
            "impurity1" => Some("impurity1"),
            "impurity2" => Some("impurity2"),
            "impurity3" => Some("impurity3"),
            "impurity4" => Some("impurity4"),
            "impurity5" => Some("impurity5"),
            "impurity6" => Some("impurity6"),
            "impurity7" => Some("impurity7"),
            "impurity8" => Some("impurity8"),
            "create_time" => Some("create_time"),
            "version" => Some("version"),
            "device_system" | "device_system_id" => Some("device_system_id"),
            "system_status" | "system_status_id" => Some("system_status_id"),
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
            "system_status" => {
                self.with_system_status_matching(
                    crate::Q::system_statuses_minimal()
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
        self.query = self.query.project("sample_time");
        self.query = self.query.project("gas");
        self.query = self.query.project("lref");
        self.query = self.query.project("impurity1");
        self.query = self.query.project("impurity2");
        self.query = self.query.project("impurity3");
        self.query = self.query.project("impurity4");
        self.query = self.query.project("impurity5");
        self.query = self.query.project("impurity6");
        self.query = self.query.project("impurity7");
        self.query = self.query.project("impurity8");
        self.query = self.query.project("create_time");
        self.query = self.query.project("version");
        self.query = self.query.project("device_system_id");
        self.query = self.query.project("system_status_id");
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
        request = request.select_system_status();
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


    pub fn select_sample_time(mut self) -> Self {
        self.query = self.query.project("sample_time");
        self
    }

    pub fn project_sample_time(self) -> Self {
        self.select_sample_time()
    }

    pub fn select_sample_time_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_sample_time_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_sample_time_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("sample_time", raw_sql_segment));
        self
    }

    pub fn group_by_sample_time(self) -> Self {
        self.group_by("sample_time")
    }

    pub fn group_by_sample_time_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("sample_time");
        request.query = request
            .query
            .project_expr(alias, Expr::column("sample_time"));
        request
    }

    pub fn group_by_sample_time_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("sample_time")
            .aggregate_with_function("sample_time", alias, function)
    }

    pub fn count_sample_time(self) -> Self {
        self.count_sample_time_as("sample_time_count")
    }

    pub fn count_sample_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("sample_time", alias)
    }

    pub fn sum_sample_time(self) -> Self {
        self.sum_sample_time_as("sum_sample_time")
    }

    pub fn sum_sample_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("sample_time", alias)
    }

    pub fn avg_sample_time(self) -> Self {
        self.avg_sample_time_as("avg_sample_time")
    }

    pub fn avg_sample_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("sample_time", alias)
    }

    pub fn min_sample_time(self) -> Self {
        self.min_sample_time_as("min_sample_time")
    }

    pub fn min_sample_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("sample_time", alias)
    }

    pub fn max_sample_time(self) -> Self {
        self.max_sample_time_as("max_sample_time")
    }

    pub fn max_sample_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("sample_time", alias)
    }

    pub fn unselect_sample_time(mut self) -> Self {
        self.query.projection.retain(|field| field != "sample_time");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "sample_time");
        self
    }


    pub fn with_sample_time(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "sample_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_sample_time_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "sample_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_sample_time_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("sample_time", value));
        self
    }



    pub fn with_sample_time_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("sample_time", value));
        self
    }

    pub fn with_sample_time_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("sample_time", value));
        self
    }

    pub fn with_sample_time_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("sample_time", value));
        self
    }

    pub fn with_sample_time_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("sample_time", value));
        self
    }

    pub fn with_sample_time_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("sample_time", value));
        self
    }

    pub fn with_sample_time_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("sample_time", lower, upper));
        self
    }

    pub fn with_sample_time_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "sample_time",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_sample_time_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "sample_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_sample_time_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "sample_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_sample_time_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("sample_time", value));
        self
    }

    pub fn with_sample_time_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("sample_time", value));
        self
    }

    pub fn with_sample_time_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("sample_time"));
        self
    }



    pub fn with_sample_time_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("sample_time"));
        self
    }


    pub fn order_by_sample_time_asc(mut self) -> Self {
        self.query = self.query.order_asc("sample_time");
        self
    }

    pub fn order_by_sample_time_desc(mut self) -> Self {
        self.query = self.query.order_desc("sample_time");
        self
    }

    pub fn order_by_sample_time_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("sample_time");
        self
    }

    pub fn order_by_sample_time_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("sample_time");
        self
    }


    pub fn select_gas(mut self) -> Self {
        self.query = self.query.project("gas");
        self
    }

    pub fn project_gas(self) -> Self {
        self.select_gas()
    }

    pub fn select_gas_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_gas_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_gas_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("gas", raw_sql_segment));
        self
    }

    pub fn select_gas_with_function(self, function: AggregateFunction) -> Self {
        self.select_gas_as_with_function("gas", function)
    }

    pub fn select_gas_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("gas", alias, function)
    }

    pub fn group_by_gas(self) -> Self {
        self.group_by("gas")
    }

    pub fn group_by_gas_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("gas");
        request.query = request
            .query
            .project_expr(alias, Expr::column("gas"));
        request
    }

    pub fn group_by_gas_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("gas")
            .aggregate_with_function("gas", alias, function)
    }

    pub fn count_gas(self) -> Self {
        self.count_gas_as("gas_count")
    }

    pub fn count_gas_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("gas", alias)
    }

    pub fn sum_gas(self) -> Self {
        self.sum_gas_as("sum_gas")
    }

    pub fn sum_gas_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("gas", alias)
    }

    pub fn avg_gas(self) -> Self {
        self.avg_gas_as("avg_gas")
    }

    pub fn avg_gas_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("gas", alias)
    }

    pub fn min_gas(self) -> Self {
        self.min_gas_as("min_gas")
    }

    pub fn min_gas_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("gas", alias)
    }

    pub fn max_gas(self) -> Self {
        self.max_gas_as("max_gas")
    }

    pub fn max_gas_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("gas", alias)
    }

    pub fn standard_deviation_gas(self) -> Self {
        self.standard_deviation_gas_as("stdDev_gas")
    }

    pub fn standard_deviation_gas_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("gas", alias)
    }

    pub fn square_root_of_population_standard_deviation_gas(self) -> Self {
        self.square_root_of_population_standard_deviation_gas_as("stdDevPop_gas")
    }

    pub fn square_root_of_population_standard_deviation_gas_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("gas", alias)
    }

    pub fn sample_variance_gas(self) -> Self {
        self.sample_variance_gas_as("varSamp_gas")
    }

    pub fn sample_variance_gas_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("gas", alias)
    }

    pub fn sample_population_variance_gas(self) -> Self {
        self.sample_population_variance_gas_as("varPop_gas")
    }

    pub fn sample_population_variance_gas_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("gas", alias)
    }

    pub fn unselect_gas(mut self) -> Self {
        self.query.projection.retain(|field| field != "gas");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "gas");
        self
    }


    pub fn with_gas(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "gas",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_gas_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "gas",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_gas_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("gas", value));
        self
    }



    pub fn with_gas_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("gas", value));
        self
    }

    pub fn with_gas_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("gas", value));
        self
    }

    pub fn with_gas_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("gas", value));
        self
    }

    pub fn with_gas_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("gas", value));
        self
    }

    pub fn with_gas_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("gas", value));
        self
    }

    pub fn with_gas_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("gas", lower, upper));
        self
    }

    pub fn with_gas_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "gas",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_gas_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "gas",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_gas_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "gas",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_gas_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("gas", value));
        self
    }

    pub fn with_gas_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("gas", value));
        self
    }

    pub fn with_gas_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("gas"));
        self
    }



    pub fn with_gas_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("gas"));
        self
    }


    pub fn order_by_gas_asc(mut self) -> Self {
        self.query = self.query.order_asc("gas");
        self
    }

    pub fn order_by_gas_desc(mut self) -> Self {
        self.query = self.query.order_desc("gas");
        self
    }

    pub fn order_by_gas_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("gas");
        self
    }

    pub fn order_by_gas_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("gas");
        self
    }


    pub fn select_lref(mut self) -> Self {
        self.query = self.query.project("lref");
        self
    }

    pub fn project_lref(self) -> Self {
        self.select_lref()
    }

    pub fn select_lref_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_lref_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_lref_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("lref", raw_sql_segment));
        self
    }

    pub fn select_lref_with_function(self, function: AggregateFunction) -> Self {
        self.select_lref_as_with_function("lref", function)
    }

    pub fn select_lref_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("lref", alias, function)
    }

    pub fn group_by_lref(self) -> Self {
        self.group_by("lref")
    }

    pub fn group_by_lref_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("lref");
        request.query = request
            .query
            .project_expr(alias, Expr::column("lref"));
        request
    }

    pub fn group_by_lref_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("lref")
            .aggregate_with_function("lref", alias, function)
    }

    pub fn count_lref(self) -> Self {
        self.count_lref_as("lref_count")
    }

    pub fn count_lref_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("lref", alias)
    }

    pub fn sum_lref(self) -> Self {
        self.sum_lref_as("sum_lref")
    }

    pub fn sum_lref_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("lref", alias)
    }

    pub fn avg_lref(self) -> Self {
        self.avg_lref_as("avg_lref")
    }

    pub fn avg_lref_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("lref", alias)
    }

    pub fn min_lref(self) -> Self {
        self.min_lref_as("min_lref")
    }

    pub fn min_lref_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("lref", alias)
    }

    pub fn max_lref(self) -> Self {
        self.max_lref_as("max_lref")
    }

    pub fn max_lref_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("lref", alias)
    }

    pub fn standard_deviation_lref(self) -> Self {
        self.standard_deviation_lref_as("stdDev_lref")
    }

    pub fn standard_deviation_lref_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("lref", alias)
    }

    pub fn square_root_of_population_standard_deviation_lref(self) -> Self {
        self.square_root_of_population_standard_deviation_lref_as("stdDevPop_lref")
    }

    pub fn square_root_of_population_standard_deviation_lref_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("lref", alias)
    }

    pub fn sample_variance_lref(self) -> Self {
        self.sample_variance_lref_as("varSamp_lref")
    }

    pub fn sample_variance_lref_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("lref", alias)
    }

    pub fn sample_population_variance_lref(self) -> Self {
        self.sample_population_variance_lref_as("varPop_lref")
    }

    pub fn sample_population_variance_lref_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("lref", alias)
    }

    pub fn unselect_lref(mut self) -> Self {
        self.query.projection.retain(|field| field != "lref");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "lref");
        self
    }


    pub fn with_lref(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "lref",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_lref_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "lref",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_lref_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("lref", value));
        self
    }



    pub fn with_lref_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("lref", value));
        self
    }

    pub fn with_lref_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("lref", value));
        self
    }

    pub fn with_lref_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("lref", value));
        self
    }

    pub fn with_lref_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("lref", value));
        self
    }

    pub fn with_lref_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("lref", value));
        self
    }

    pub fn with_lref_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("lref", lower, upper));
        self
    }

    pub fn with_lref_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "lref",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_lref_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "lref",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_lref_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "lref",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_lref_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("lref", value));
        self
    }

    pub fn with_lref_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("lref", value));
        self
    }

    pub fn with_lref_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("lref"));
        self
    }



    pub fn with_lref_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("lref"));
        self
    }


    pub fn order_by_lref_asc(mut self) -> Self {
        self.query = self.query.order_asc("lref");
        self
    }

    pub fn order_by_lref_desc(mut self) -> Self {
        self.query = self.query.order_desc("lref");
        self
    }

    pub fn order_by_lref_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("lref");
        self
    }

    pub fn order_by_lref_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("lref");
        self
    }


    pub fn select_impurity1(mut self) -> Self {
        self.query = self.query.project("impurity1");
        self
    }

    pub fn project_impurity1(self) -> Self {
        self.select_impurity1()
    }

    pub fn select_impurity1_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_impurity1_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_impurity1_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("impurity1", raw_sql_segment));
        self
    }

    pub fn select_impurity1_with_function(self, function: AggregateFunction) -> Self {
        self.select_impurity1_as_with_function("impurity1", function)
    }

    pub fn select_impurity1_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("impurity1", alias, function)
    }

    pub fn group_by_impurity1(self) -> Self {
        self.group_by("impurity1")
    }

    pub fn group_by_impurity1_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("impurity1");
        request.query = request
            .query
            .project_expr(alias, Expr::column("impurity1"));
        request
    }

    pub fn group_by_impurity1_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("impurity1")
            .aggregate_with_function("impurity1", alias, function)
    }

    pub fn count_impurity1(self) -> Self {
        self.count_impurity1_as("impurity1_count")
    }

    pub fn count_impurity1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("impurity1", alias)
    }

    pub fn sum_impurity1(self) -> Self {
        self.sum_impurity1_as("sum_impurity1")
    }

    pub fn sum_impurity1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("impurity1", alias)
    }

    pub fn avg_impurity1(self) -> Self {
        self.avg_impurity1_as("avg_impurity1")
    }

    pub fn avg_impurity1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("impurity1", alias)
    }

    pub fn min_impurity1(self) -> Self {
        self.min_impurity1_as("min_impurity1")
    }

    pub fn min_impurity1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("impurity1", alias)
    }

    pub fn max_impurity1(self) -> Self {
        self.max_impurity1_as("max_impurity1")
    }

    pub fn max_impurity1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("impurity1", alias)
    }

    pub fn standard_deviation_impurity1(self) -> Self {
        self.standard_deviation_impurity1_as("stdDev_impurity1")
    }

    pub fn standard_deviation_impurity1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("impurity1", alias)
    }

    pub fn square_root_of_population_standard_deviation_impurity1(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity1_as("stdDevPop_impurity1")
    }

    pub fn square_root_of_population_standard_deviation_impurity1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("impurity1", alias)
    }

    pub fn sample_variance_impurity1(self) -> Self {
        self.sample_variance_impurity1_as("varSamp_impurity1")
    }

    pub fn sample_variance_impurity1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("impurity1", alias)
    }

    pub fn sample_population_variance_impurity1(self) -> Self {
        self.sample_population_variance_impurity1_as("varPop_impurity1")
    }

    pub fn sample_population_variance_impurity1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("impurity1", alias)
    }

    pub fn unselect_impurity1(mut self) -> Self {
        self.query.projection.retain(|field| field != "impurity1");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "impurity1");
        self
    }


    pub fn with_impurity1(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "impurity1",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_impurity1_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "impurity1",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_impurity1_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("impurity1", value));
        self
    }



    pub fn with_impurity1_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("impurity1", value));
        self
    }

    pub fn with_impurity1_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity1", value));
        self
    }

    pub fn with_impurity1_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("impurity1", value));
        self
    }

    pub fn with_impurity1_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity1", value));
        self
    }

    pub fn with_impurity1_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("impurity1", value));
        self
    }

    pub fn with_impurity1_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("impurity1", lower, upper));
        self
    }

    pub fn with_impurity1_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "impurity1",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_impurity1_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "impurity1",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity1_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "impurity1",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity1_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity1", value));
        self
    }

    pub fn with_impurity1_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity1", value));
        self
    }

    pub fn with_impurity1_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("impurity1"));
        self
    }



    pub fn with_impurity1_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("impurity1"));
        self
    }


    pub fn order_by_impurity1_asc(mut self) -> Self {
        self.query = self.query.order_asc("impurity1");
        self
    }

    pub fn order_by_impurity1_desc(mut self) -> Self {
        self.query = self.query.order_desc("impurity1");
        self
    }

    pub fn order_by_impurity1_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("impurity1");
        self
    }

    pub fn order_by_impurity1_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("impurity1");
        self
    }


    pub fn select_impurity2(mut self) -> Self {
        self.query = self.query.project("impurity2");
        self
    }

    pub fn project_impurity2(self) -> Self {
        self.select_impurity2()
    }

    pub fn select_impurity2_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_impurity2_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_impurity2_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("impurity2", raw_sql_segment));
        self
    }

    pub fn select_impurity2_with_function(self, function: AggregateFunction) -> Self {
        self.select_impurity2_as_with_function("impurity2", function)
    }

    pub fn select_impurity2_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("impurity2", alias, function)
    }

    pub fn group_by_impurity2(self) -> Self {
        self.group_by("impurity2")
    }

    pub fn group_by_impurity2_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("impurity2");
        request.query = request
            .query
            .project_expr(alias, Expr::column("impurity2"));
        request
    }

    pub fn group_by_impurity2_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("impurity2")
            .aggregate_with_function("impurity2", alias, function)
    }

    pub fn count_impurity2(self) -> Self {
        self.count_impurity2_as("impurity2_count")
    }

    pub fn count_impurity2_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("impurity2", alias)
    }

    pub fn sum_impurity2(self) -> Self {
        self.sum_impurity2_as("sum_impurity2")
    }

    pub fn sum_impurity2_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("impurity2", alias)
    }

    pub fn avg_impurity2(self) -> Self {
        self.avg_impurity2_as("avg_impurity2")
    }

    pub fn avg_impurity2_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("impurity2", alias)
    }

    pub fn min_impurity2(self) -> Self {
        self.min_impurity2_as("min_impurity2")
    }

    pub fn min_impurity2_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("impurity2", alias)
    }

    pub fn max_impurity2(self) -> Self {
        self.max_impurity2_as("max_impurity2")
    }

    pub fn max_impurity2_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("impurity2", alias)
    }

    pub fn standard_deviation_impurity2(self) -> Self {
        self.standard_deviation_impurity2_as("stdDev_impurity2")
    }

    pub fn standard_deviation_impurity2_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("impurity2", alias)
    }

    pub fn square_root_of_population_standard_deviation_impurity2(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity2_as("stdDevPop_impurity2")
    }

    pub fn square_root_of_population_standard_deviation_impurity2_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("impurity2", alias)
    }

    pub fn sample_variance_impurity2(self) -> Self {
        self.sample_variance_impurity2_as("varSamp_impurity2")
    }

    pub fn sample_variance_impurity2_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("impurity2", alias)
    }

    pub fn sample_population_variance_impurity2(self) -> Self {
        self.sample_population_variance_impurity2_as("varPop_impurity2")
    }

    pub fn sample_population_variance_impurity2_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("impurity2", alias)
    }

    pub fn unselect_impurity2(mut self) -> Self {
        self.query.projection.retain(|field| field != "impurity2");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "impurity2");
        self
    }


    pub fn with_impurity2(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "impurity2",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_impurity2_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "impurity2",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_impurity2_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("impurity2", value));
        self
    }



    pub fn with_impurity2_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("impurity2", value));
        self
    }

    pub fn with_impurity2_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity2", value));
        self
    }

    pub fn with_impurity2_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("impurity2", value));
        self
    }

    pub fn with_impurity2_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity2", value));
        self
    }

    pub fn with_impurity2_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("impurity2", value));
        self
    }

    pub fn with_impurity2_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("impurity2", lower, upper));
        self
    }

    pub fn with_impurity2_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "impurity2",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_impurity2_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "impurity2",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity2_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "impurity2",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity2_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity2", value));
        self
    }

    pub fn with_impurity2_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity2", value));
        self
    }

    pub fn with_impurity2_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("impurity2"));
        self
    }



    pub fn with_impurity2_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("impurity2"));
        self
    }


    pub fn order_by_impurity2_asc(mut self) -> Self {
        self.query = self.query.order_asc("impurity2");
        self
    }

    pub fn order_by_impurity2_desc(mut self) -> Self {
        self.query = self.query.order_desc("impurity2");
        self
    }

    pub fn order_by_impurity2_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("impurity2");
        self
    }

    pub fn order_by_impurity2_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("impurity2");
        self
    }


    pub fn select_impurity3(mut self) -> Self {
        self.query = self.query.project("impurity3");
        self
    }

    pub fn project_impurity3(self) -> Self {
        self.select_impurity3()
    }

    pub fn select_impurity3_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_impurity3_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_impurity3_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("impurity3", raw_sql_segment));
        self
    }

    pub fn select_impurity3_with_function(self, function: AggregateFunction) -> Self {
        self.select_impurity3_as_with_function("impurity3", function)
    }

    pub fn select_impurity3_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("impurity3", alias, function)
    }

    pub fn group_by_impurity3(self) -> Self {
        self.group_by("impurity3")
    }

    pub fn group_by_impurity3_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("impurity3");
        request.query = request
            .query
            .project_expr(alias, Expr::column("impurity3"));
        request
    }

    pub fn group_by_impurity3_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("impurity3")
            .aggregate_with_function("impurity3", alias, function)
    }

    pub fn count_impurity3(self) -> Self {
        self.count_impurity3_as("impurity3_count")
    }

    pub fn count_impurity3_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("impurity3", alias)
    }

    pub fn sum_impurity3(self) -> Self {
        self.sum_impurity3_as("sum_impurity3")
    }

    pub fn sum_impurity3_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("impurity3", alias)
    }

    pub fn avg_impurity3(self) -> Self {
        self.avg_impurity3_as("avg_impurity3")
    }

    pub fn avg_impurity3_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("impurity3", alias)
    }

    pub fn min_impurity3(self) -> Self {
        self.min_impurity3_as("min_impurity3")
    }

    pub fn min_impurity3_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("impurity3", alias)
    }

    pub fn max_impurity3(self) -> Self {
        self.max_impurity3_as("max_impurity3")
    }

    pub fn max_impurity3_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("impurity3", alias)
    }

    pub fn standard_deviation_impurity3(self) -> Self {
        self.standard_deviation_impurity3_as("stdDev_impurity3")
    }

    pub fn standard_deviation_impurity3_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("impurity3", alias)
    }

    pub fn square_root_of_population_standard_deviation_impurity3(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity3_as("stdDevPop_impurity3")
    }

    pub fn square_root_of_population_standard_deviation_impurity3_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("impurity3", alias)
    }

    pub fn sample_variance_impurity3(self) -> Self {
        self.sample_variance_impurity3_as("varSamp_impurity3")
    }

    pub fn sample_variance_impurity3_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("impurity3", alias)
    }

    pub fn sample_population_variance_impurity3(self) -> Self {
        self.sample_population_variance_impurity3_as("varPop_impurity3")
    }

    pub fn sample_population_variance_impurity3_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("impurity3", alias)
    }

    pub fn unselect_impurity3(mut self) -> Self {
        self.query.projection.retain(|field| field != "impurity3");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "impurity3");
        self
    }


    pub fn with_impurity3(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "impurity3",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_impurity3_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "impurity3",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_impurity3_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("impurity3", value));
        self
    }



    pub fn with_impurity3_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("impurity3", value));
        self
    }

    pub fn with_impurity3_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity3", value));
        self
    }

    pub fn with_impurity3_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("impurity3", value));
        self
    }

    pub fn with_impurity3_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity3", value));
        self
    }

    pub fn with_impurity3_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("impurity3", value));
        self
    }

    pub fn with_impurity3_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("impurity3", lower, upper));
        self
    }

    pub fn with_impurity3_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "impurity3",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_impurity3_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "impurity3",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity3_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "impurity3",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity3_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity3", value));
        self
    }

    pub fn with_impurity3_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity3", value));
        self
    }

    pub fn with_impurity3_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("impurity3"));
        self
    }



    pub fn with_impurity3_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("impurity3"));
        self
    }


    pub fn order_by_impurity3_asc(mut self) -> Self {
        self.query = self.query.order_asc("impurity3");
        self
    }

    pub fn order_by_impurity3_desc(mut self) -> Self {
        self.query = self.query.order_desc("impurity3");
        self
    }

    pub fn order_by_impurity3_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("impurity3");
        self
    }

    pub fn order_by_impurity3_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("impurity3");
        self
    }


    pub fn select_impurity4(mut self) -> Self {
        self.query = self.query.project("impurity4");
        self
    }

    pub fn project_impurity4(self) -> Self {
        self.select_impurity4()
    }

    pub fn select_impurity4_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_impurity4_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_impurity4_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("impurity4", raw_sql_segment));
        self
    }

    pub fn select_impurity4_with_function(self, function: AggregateFunction) -> Self {
        self.select_impurity4_as_with_function("impurity4", function)
    }

    pub fn select_impurity4_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("impurity4", alias, function)
    }

    pub fn group_by_impurity4(self) -> Self {
        self.group_by("impurity4")
    }

    pub fn group_by_impurity4_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("impurity4");
        request.query = request
            .query
            .project_expr(alias, Expr::column("impurity4"));
        request
    }

    pub fn group_by_impurity4_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("impurity4")
            .aggregate_with_function("impurity4", alias, function)
    }

    pub fn count_impurity4(self) -> Self {
        self.count_impurity4_as("impurity4_count")
    }

    pub fn count_impurity4_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("impurity4", alias)
    }

    pub fn sum_impurity4(self) -> Self {
        self.sum_impurity4_as("sum_impurity4")
    }

    pub fn sum_impurity4_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("impurity4", alias)
    }

    pub fn avg_impurity4(self) -> Self {
        self.avg_impurity4_as("avg_impurity4")
    }

    pub fn avg_impurity4_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("impurity4", alias)
    }

    pub fn min_impurity4(self) -> Self {
        self.min_impurity4_as("min_impurity4")
    }

    pub fn min_impurity4_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("impurity4", alias)
    }

    pub fn max_impurity4(self) -> Self {
        self.max_impurity4_as("max_impurity4")
    }

    pub fn max_impurity4_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("impurity4", alias)
    }

    pub fn standard_deviation_impurity4(self) -> Self {
        self.standard_deviation_impurity4_as("stdDev_impurity4")
    }

    pub fn standard_deviation_impurity4_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("impurity4", alias)
    }

    pub fn square_root_of_population_standard_deviation_impurity4(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity4_as("stdDevPop_impurity4")
    }

    pub fn square_root_of_population_standard_deviation_impurity4_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("impurity4", alias)
    }

    pub fn sample_variance_impurity4(self) -> Self {
        self.sample_variance_impurity4_as("varSamp_impurity4")
    }

    pub fn sample_variance_impurity4_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("impurity4", alias)
    }

    pub fn sample_population_variance_impurity4(self) -> Self {
        self.sample_population_variance_impurity4_as("varPop_impurity4")
    }

    pub fn sample_population_variance_impurity4_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("impurity4", alias)
    }

    pub fn unselect_impurity4(mut self) -> Self {
        self.query.projection.retain(|field| field != "impurity4");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "impurity4");
        self
    }


    pub fn with_impurity4(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "impurity4",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_impurity4_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "impurity4",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_impurity4_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("impurity4", value));
        self
    }



    pub fn with_impurity4_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("impurity4", value));
        self
    }

    pub fn with_impurity4_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity4", value));
        self
    }

    pub fn with_impurity4_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("impurity4", value));
        self
    }

    pub fn with_impurity4_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity4", value));
        self
    }

    pub fn with_impurity4_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("impurity4", value));
        self
    }

    pub fn with_impurity4_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("impurity4", lower, upper));
        self
    }

    pub fn with_impurity4_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "impurity4",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_impurity4_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "impurity4",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity4_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "impurity4",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity4_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity4", value));
        self
    }

    pub fn with_impurity4_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity4", value));
        self
    }

    pub fn with_impurity4_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("impurity4"));
        self
    }



    pub fn with_impurity4_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("impurity4"));
        self
    }


    pub fn order_by_impurity4_asc(mut self) -> Self {
        self.query = self.query.order_asc("impurity4");
        self
    }

    pub fn order_by_impurity4_desc(mut self) -> Self {
        self.query = self.query.order_desc("impurity4");
        self
    }

    pub fn order_by_impurity4_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("impurity4");
        self
    }

    pub fn order_by_impurity4_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("impurity4");
        self
    }


    pub fn select_impurity5(mut self) -> Self {
        self.query = self.query.project("impurity5");
        self
    }

    pub fn project_impurity5(self) -> Self {
        self.select_impurity5()
    }

    pub fn select_impurity5_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_impurity5_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_impurity5_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("impurity5", raw_sql_segment));
        self
    }

    pub fn select_impurity5_with_function(self, function: AggregateFunction) -> Self {
        self.select_impurity5_as_with_function("impurity5", function)
    }

    pub fn select_impurity5_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("impurity5", alias, function)
    }

    pub fn group_by_impurity5(self) -> Self {
        self.group_by("impurity5")
    }

    pub fn group_by_impurity5_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("impurity5");
        request.query = request
            .query
            .project_expr(alias, Expr::column("impurity5"));
        request
    }

    pub fn group_by_impurity5_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("impurity5")
            .aggregate_with_function("impurity5", alias, function)
    }

    pub fn count_impurity5(self) -> Self {
        self.count_impurity5_as("impurity5_count")
    }

    pub fn count_impurity5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("impurity5", alias)
    }

    pub fn sum_impurity5(self) -> Self {
        self.sum_impurity5_as("sum_impurity5")
    }

    pub fn sum_impurity5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("impurity5", alias)
    }

    pub fn avg_impurity5(self) -> Self {
        self.avg_impurity5_as("avg_impurity5")
    }

    pub fn avg_impurity5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("impurity5", alias)
    }

    pub fn min_impurity5(self) -> Self {
        self.min_impurity5_as("min_impurity5")
    }

    pub fn min_impurity5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("impurity5", alias)
    }

    pub fn max_impurity5(self) -> Self {
        self.max_impurity5_as("max_impurity5")
    }

    pub fn max_impurity5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("impurity5", alias)
    }

    pub fn standard_deviation_impurity5(self) -> Self {
        self.standard_deviation_impurity5_as("stdDev_impurity5")
    }

    pub fn standard_deviation_impurity5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("impurity5", alias)
    }

    pub fn square_root_of_population_standard_deviation_impurity5(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity5_as("stdDevPop_impurity5")
    }

    pub fn square_root_of_population_standard_deviation_impurity5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("impurity5", alias)
    }

    pub fn sample_variance_impurity5(self) -> Self {
        self.sample_variance_impurity5_as("varSamp_impurity5")
    }

    pub fn sample_variance_impurity5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("impurity5", alias)
    }

    pub fn sample_population_variance_impurity5(self) -> Self {
        self.sample_population_variance_impurity5_as("varPop_impurity5")
    }

    pub fn sample_population_variance_impurity5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("impurity5", alias)
    }

    pub fn unselect_impurity5(mut self) -> Self {
        self.query.projection.retain(|field| field != "impurity5");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "impurity5");
        self
    }


    pub fn with_impurity5(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "impurity5",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_impurity5_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "impurity5",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_impurity5_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("impurity5", value));
        self
    }



    pub fn with_impurity5_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("impurity5", value));
        self
    }

    pub fn with_impurity5_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity5", value));
        self
    }

    pub fn with_impurity5_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("impurity5", value));
        self
    }

    pub fn with_impurity5_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity5", value));
        self
    }

    pub fn with_impurity5_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("impurity5", value));
        self
    }

    pub fn with_impurity5_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("impurity5", lower, upper));
        self
    }

    pub fn with_impurity5_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "impurity5",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_impurity5_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "impurity5",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity5_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "impurity5",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity5_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity5", value));
        self
    }

    pub fn with_impurity5_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity5", value));
        self
    }

    pub fn with_impurity5_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("impurity5"));
        self
    }



    pub fn with_impurity5_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("impurity5"));
        self
    }


    pub fn order_by_impurity5_asc(mut self) -> Self {
        self.query = self.query.order_asc("impurity5");
        self
    }

    pub fn order_by_impurity5_desc(mut self) -> Self {
        self.query = self.query.order_desc("impurity5");
        self
    }

    pub fn order_by_impurity5_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("impurity5");
        self
    }

    pub fn order_by_impurity5_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("impurity5");
        self
    }


    pub fn select_impurity6(mut self) -> Self {
        self.query = self.query.project("impurity6");
        self
    }

    pub fn project_impurity6(self) -> Self {
        self.select_impurity6()
    }

    pub fn select_impurity6_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_impurity6_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_impurity6_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("impurity6", raw_sql_segment));
        self
    }

    pub fn select_impurity6_with_function(self, function: AggregateFunction) -> Self {
        self.select_impurity6_as_with_function("impurity6", function)
    }

    pub fn select_impurity6_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("impurity6", alias, function)
    }

    pub fn group_by_impurity6(self) -> Self {
        self.group_by("impurity6")
    }

    pub fn group_by_impurity6_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("impurity6");
        request.query = request
            .query
            .project_expr(alias, Expr::column("impurity6"));
        request
    }

    pub fn group_by_impurity6_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("impurity6")
            .aggregate_with_function("impurity6", alias, function)
    }

    pub fn count_impurity6(self) -> Self {
        self.count_impurity6_as("impurity6_count")
    }

    pub fn count_impurity6_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("impurity6", alias)
    }

    pub fn sum_impurity6(self) -> Self {
        self.sum_impurity6_as("sum_impurity6")
    }

    pub fn sum_impurity6_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("impurity6", alias)
    }

    pub fn avg_impurity6(self) -> Self {
        self.avg_impurity6_as("avg_impurity6")
    }

    pub fn avg_impurity6_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("impurity6", alias)
    }

    pub fn min_impurity6(self) -> Self {
        self.min_impurity6_as("min_impurity6")
    }

    pub fn min_impurity6_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("impurity6", alias)
    }

    pub fn max_impurity6(self) -> Self {
        self.max_impurity6_as("max_impurity6")
    }

    pub fn max_impurity6_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("impurity6", alias)
    }

    pub fn standard_deviation_impurity6(self) -> Self {
        self.standard_deviation_impurity6_as("stdDev_impurity6")
    }

    pub fn standard_deviation_impurity6_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("impurity6", alias)
    }

    pub fn square_root_of_population_standard_deviation_impurity6(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity6_as("stdDevPop_impurity6")
    }

    pub fn square_root_of_population_standard_deviation_impurity6_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("impurity6", alias)
    }

    pub fn sample_variance_impurity6(self) -> Self {
        self.sample_variance_impurity6_as("varSamp_impurity6")
    }

    pub fn sample_variance_impurity6_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("impurity6", alias)
    }

    pub fn sample_population_variance_impurity6(self) -> Self {
        self.sample_population_variance_impurity6_as("varPop_impurity6")
    }

    pub fn sample_population_variance_impurity6_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("impurity6", alias)
    }

    pub fn unselect_impurity6(mut self) -> Self {
        self.query.projection.retain(|field| field != "impurity6");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "impurity6");
        self
    }


    pub fn with_impurity6(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "impurity6",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_impurity6_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "impurity6",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_impurity6_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("impurity6", value));
        self
    }



    pub fn with_impurity6_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("impurity6", value));
        self
    }

    pub fn with_impurity6_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity6", value));
        self
    }

    pub fn with_impurity6_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("impurity6", value));
        self
    }

    pub fn with_impurity6_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity6", value));
        self
    }

    pub fn with_impurity6_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("impurity6", value));
        self
    }

    pub fn with_impurity6_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("impurity6", lower, upper));
        self
    }

    pub fn with_impurity6_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "impurity6",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_impurity6_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "impurity6",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity6_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "impurity6",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity6_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity6", value));
        self
    }

    pub fn with_impurity6_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity6", value));
        self
    }

    pub fn with_impurity6_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("impurity6"));
        self
    }



    pub fn with_impurity6_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("impurity6"));
        self
    }


    pub fn order_by_impurity6_asc(mut self) -> Self {
        self.query = self.query.order_asc("impurity6");
        self
    }

    pub fn order_by_impurity6_desc(mut self) -> Self {
        self.query = self.query.order_desc("impurity6");
        self
    }

    pub fn order_by_impurity6_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("impurity6");
        self
    }

    pub fn order_by_impurity6_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("impurity6");
        self
    }


    pub fn select_impurity7(mut self) -> Self {
        self.query = self.query.project("impurity7");
        self
    }

    pub fn project_impurity7(self) -> Self {
        self.select_impurity7()
    }

    pub fn select_impurity7_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_impurity7_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_impurity7_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("impurity7", raw_sql_segment));
        self
    }

    pub fn select_impurity7_with_function(self, function: AggregateFunction) -> Self {
        self.select_impurity7_as_with_function("impurity7", function)
    }

    pub fn select_impurity7_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("impurity7", alias, function)
    }

    pub fn group_by_impurity7(self) -> Self {
        self.group_by("impurity7")
    }

    pub fn group_by_impurity7_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("impurity7");
        request.query = request
            .query
            .project_expr(alias, Expr::column("impurity7"));
        request
    }

    pub fn group_by_impurity7_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("impurity7")
            .aggregate_with_function("impurity7", alias, function)
    }

    pub fn count_impurity7(self) -> Self {
        self.count_impurity7_as("impurity7_count")
    }

    pub fn count_impurity7_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("impurity7", alias)
    }

    pub fn sum_impurity7(self) -> Self {
        self.sum_impurity7_as("sum_impurity7")
    }

    pub fn sum_impurity7_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("impurity7", alias)
    }

    pub fn avg_impurity7(self) -> Self {
        self.avg_impurity7_as("avg_impurity7")
    }

    pub fn avg_impurity7_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("impurity7", alias)
    }

    pub fn min_impurity7(self) -> Self {
        self.min_impurity7_as("min_impurity7")
    }

    pub fn min_impurity7_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("impurity7", alias)
    }

    pub fn max_impurity7(self) -> Self {
        self.max_impurity7_as("max_impurity7")
    }

    pub fn max_impurity7_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("impurity7", alias)
    }

    pub fn standard_deviation_impurity7(self) -> Self {
        self.standard_deviation_impurity7_as("stdDev_impurity7")
    }

    pub fn standard_deviation_impurity7_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("impurity7", alias)
    }

    pub fn square_root_of_population_standard_deviation_impurity7(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity7_as("stdDevPop_impurity7")
    }

    pub fn square_root_of_population_standard_deviation_impurity7_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("impurity7", alias)
    }

    pub fn sample_variance_impurity7(self) -> Self {
        self.sample_variance_impurity7_as("varSamp_impurity7")
    }

    pub fn sample_variance_impurity7_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("impurity7", alias)
    }

    pub fn sample_population_variance_impurity7(self) -> Self {
        self.sample_population_variance_impurity7_as("varPop_impurity7")
    }

    pub fn sample_population_variance_impurity7_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("impurity7", alias)
    }

    pub fn unselect_impurity7(mut self) -> Self {
        self.query.projection.retain(|field| field != "impurity7");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "impurity7");
        self
    }


    pub fn with_impurity7(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "impurity7",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_impurity7_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "impurity7",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_impurity7_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("impurity7", value));
        self
    }



    pub fn with_impurity7_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("impurity7", value));
        self
    }

    pub fn with_impurity7_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity7", value));
        self
    }

    pub fn with_impurity7_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("impurity7", value));
        self
    }

    pub fn with_impurity7_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity7", value));
        self
    }

    pub fn with_impurity7_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("impurity7", value));
        self
    }

    pub fn with_impurity7_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("impurity7", lower, upper));
        self
    }

    pub fn with_impurity7_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "impurity7",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_impurity7_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "impurity7",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity7_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "impurity7",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity7_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity7", value));
        self
    }

    pub fn with_impurity7_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity7", value));
        self
    }

    pub fn with_impurity7_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("impurity7"));
        self
    }



    pub fn with_impurity7_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("impurity7"));
        self
    }


    pub fn order_by_impurity7_asc(mut self) -> Self {
        self.query = self.query.order_asc("impurity7");
        self
    }

    pub fn order_by_impurity7_desc(mut self) -> Self {
        self.query = self.query.order_desc("impurity7");
        self
    }

    pub fn order_by_impurity7_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("impurity7");
        self
    }

    pub fn order_by_impurity7_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("impurity7");
        self
    }


    pub fn select_impurity8(mut self) -> Self {
        self.query = self.query.project("impurity8");
        self
    }

    pub fn project_impurity8(self) -> Self {
        self.select_impurity8()
    }

    pub fn select_impurity8_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_impurity8_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_impurity8_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("impurity8", raw_sql_segment));
        self
    }

    pub fn select_impurity8_with_function(self, function: AggregateFunction) -> Self {
        self.select_impurity8_as_with_function("impurity8", function)
    }

    pub fn select_impurity8_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("impurity8", alias, function)
    }

    pub fn group_by_impurity8(self) -> Self {
        self.group_by("impurity8")
    }

    pub fn group_by_impurity8_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("impurity8");
        request.query = request
            .query
            .project_expr(alias, Expr::column("impurity8"));
        request
    }

    pub fn group_by_impurity8_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("impurity8")
            .aggregate_with_function("impurity8", alias, function)
    }

    pub fn count_impurity8(self) -> Self {
        self.count_impurity8_as("impurity8_count")
    }

    pub fn count_impurity8_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("impurity8", alias)
    }

    pub fn sum_impurity8(self) -> Self {
        self.sum_impurity8_as("sum_impurity8")
    }

    pub fn sum_impurity8_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("impurity8", alias)
    }

    pub fn avg_impurity8(self) -> Self {
        self.avg_impurity8_as("avg_impurity8")
    }

    pub fn avg_impurity8_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("impurity8", alias)
    }

    pub fn min_impurity8(self) -> Self {
        self.min_impurity8_as("min_impurity8")
    }

    pub fn min_impurity8_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("impurity8", alias)
    }

    pub fn max_impurity8(self) -> Self {
        self.max_impurity8_as("max_impurity8")
    }

    pub fn max_impurity8_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("impurity8", alias)
    }

    pub fn standard_deviation_impurity8(self) -> Self {
        self.standard_deviation_impurity8_as("stdDev_impurity8")
    }

    pub fn standard_deviation_impurity8_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("impurity8", alias)
    }

    pub fn square_root_of_population_standard_deviation_impurity8(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity8_as("stdDevPop_impurity8")
    }

    pub fn square_root_of_population_standard_deviation_impurity8_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("impurity8", alias)
    }

    pub fn sample_variance_impurity8(self) -> Self {
        self.sample_variance_impurity8_as("varSamp_impurity8")
    }

    pub fn sample_variance_impurity8_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("impurity8", alias)
    }

    pub fn sample_population_variance_impurity8(self) -> Self {
        self.sample_population_variance_impurity8_as("varPop_impurity8")
    }

    pub fn sample_population_variance_impurity8_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("impurity8", alias)
    }

    pub fn unselect_impurity8(mut self) -> Self {
        self.query.projection.retain(|field| field != "impurity8");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "impurity8");
        self
    }


    pub fn with_impurity8(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "impurity8",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_impurity8_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "impurity8",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_impurity8_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("impurity8", value));
        self
    }



    pub fn with_impurity8_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("impurity8", value));
        self
    }

    pub fn with_impurity8_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity8", value));
        self
    }

    pub fn with_impurity8_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("impurity8", value));
        self
    }

    pub fn with_impurity8_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity8", value));
        self
    }

    pub fn with_impurity8_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("impurity8", value));
        self
    }

    pub fn with_impurity8_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("impurity8", lower, upper));
        self
    }

    pub fn with_impurity8_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "impurity8",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_impurity8_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "impurity8",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity8_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "impurity8",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_impurity8_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("impurity8", value));
        self
    }

    pub fn with_impurity8_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("impurity8", value));
        self
    }

    pub fn with_impurity8_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("impurity8"));
        self
    }



    pub fn with_impurity8_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("impurity8"));
        self
    }


    pub fn order_by_impurity8_asc(mut self) -> Self {
        self.query = self.query.order_asc("impurity8");
        self
    }

    pub fn order_by_impurity8_desc(mut self) -> Self {
        self.query = self.query.order_desc("impurity8");
        self
    }

    pub fn order_by_impurity8_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("impurity8");
        self
    }

    pub fn order_by_impurity8_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("impurity8");
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


    /// Please use `with_system_status_is` instead
    pub(crate) fn filter_by_system_status(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("system_status_id", value.entity_id_value()));
        self
    }
    /// Complex relation filter for `system_status`.
    ///
    /// **Usage Priority:**
    ///
    /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
    ///    - [`Self::with_system_status_is_xxx`]
    ///
    ///    This gives the best code readability.
    ///
    /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
    ///
    /// # Example
    /// ```text
    /// // Only use when building dynamic queries
    /// let dynamic_query = crate::Q::system_statuses_minimal().filter(...);
    /// let request = crate::Q::sample_records().with_system_status_matching(dynamic_query);
    /// ```
    pub fn with_system_status_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "system_status_id",
            <crate::SystemStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("system_status", selection));
        self
    }


    /// Complex relation filter for `system_status`.
    ///
    /// **Usage Priority:**
    ///
    /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
    ///    - [`Self::with_system_status_is_not_xxx`]
    ///
    ///    This gives the best code readability.
    ///
    /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
    ///
    /// # Example
    /// ```text
    /// // Only use when building dynamic queries
    /// let dynamic_query = crate::Q::system_statuses_minimal().filter(...);
    /// let request = crate::Q::sample_records().without_system_status_matching(dynamic_query);
    /// ```
    pub fn without_system_status_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "system_status_id",
            <crate::SystemStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("system_status", selection));
        self
    }


    pub fn have_system_status(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("system_status_id"));
        self
    }

    pub fn have_no_system_status(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("system_status_id"));
        self
    }


    pub fn group_by_system_status(self) -> Self {
        self.group_by("system_status_id")
    }

    pub fn group_by_system_status_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("system_status_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("system_status_id"));
        request
    }

    pub fn group_by_system_status_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("system_status_id")
            .aggregate_with_function("system_status_id", alias, function)
    }

    pub fn group_by_system_status_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("system_status_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "system_status",
            "system_status_id",
            request,
        ));
        self
    }

    pub fn group_by_system_status_with_details(self) -> Self {
        self.group_by_system_status_with_details_from(crate::Q::system_statuses().unlimited())
    }

    pub fn group_by_system_status_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_system_status_with(request)
    }


    pub fn roll_up_to_system_status(self) -> Self {
        self.roll_up_to_system_status_with(crate::Q::system_statuses().unlimited())
    }

    pub fn roll_up_to_system_status_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_system_status_matching(selection.clone())
            .group_by_system_status_with(selection)
    }

    pub fn count_system_status(self) -> Self {
        self.count_system_status_as("system_status_count")
    }

    pub fn count_system_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("system_status_id", alias)
    }

    pub fn unselect_system_status(mut self) -> Self {
        self.query.projection.retain(|field| field != "system_status_id");
        self.query.relations.retain(|relation| relation.name != "system_status");
        self
    }
    pub fn with_system_status_is_online(self) -> Self {
        self.filter_by_system_status(1001_u64)
    }



    pub fn with_system_status_is_not_online(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("system_status_id", 1001_u64));
        self
    }


    pub fn with_system_status_is_offline(self) -> Self {
        self.filter_by_system_status(1002_u64)
    }



    pub fn with_system_status_is_not_offline(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("system_status_id", 1002_u64));
        self
    }


    pub fn with_system_status_is_sampling(self) -> Self {
        self.filter_by_system_status(1003_u64)
    }



    pub fn with_system_status_is_not_sampling(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("system_status_id", 1003_u64));
        self
    }


    pub fn with_system_status_is_calibrating(self) -> Self {
        self.filter_by_system_status(1004_u64)
    }



    pub fn with_system_status_is_not_calibrating(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("system_status_id", 1004_u64));
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

    pub fn select_system_status(mut self) -> Self {
        self.query = self.query.relation("system_status");
        self
    }

    pub fn select_system_status_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("system_status", selection.into_query());
        self
}

    pub fn facet_by_system_status_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_system_status_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_system_status_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "system_status",
            request,
            include_all_facets,
        ));
        self
    }
}

impl<R> Default for SampleRecordRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< SampleRecordRequest<R> > for SelectQuery {
    fn from(request: SampleRecordRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< SampleRecordRequest<R> > for QuerySelection {
    fn from(request: SampleRecordRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::SampleRecord> 
where C: crate::TeaqlRuntime + ?Sized + 'a
{
    type Error = teaql_runtime::RuntimeError;
    type Entity = crate::SampleRecord;
    fn save(self, context: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Entity, Self::Error>> + Send + '_>> {
        Box::pin(async move {
            context.save_audited_entity(self).await
        })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<SampleRecordRequest<R>> {
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.inner.query_options.comment = Some(comment.into());
        self
    }

    pub fn new_entity<C>(&self, context: &C) -> crate::SampleRecord
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        self.require_comment();
        let mut entity = crate::SampleRecord::runtime_new(context.user_context().entity_runtime_state());
        if let Ok(id) = context.user_context().next_id(crate::SampleRecord::ENTITY_NAME) {
            entity.update_id(id);
        }
        teaql_core::Entity::mark_as_new(&mut entity);
        entity
    }

    fn into_inner_with_trace(mut self) -> SampleRecordRequest<R> {
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
