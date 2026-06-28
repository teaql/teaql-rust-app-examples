use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::SystemInfo {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::SystemInfo {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/system_info
#[derive(Debug)]
pub struct SystemInfoRequest<R = crate::SystemInfo> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for SystemInfoRequest<R> {
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

impl<R> SystemInfoRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("SystemInfo")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> SystemInfoRequest<T> {
        SystemInfoRequest {
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
        ctx: &'a C,
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .system_info_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let query_options = self.query_options.clone();
        let relation_aggregates = runtime_relation_aggregates(&query_options);
        let query = apply_runtime_metadata(self.query, &query_options, &self.child_enhancements);
        let mut rows = repository.fetch_enhanced_entities_with_relation_aggregates::<R>(
            &query,
            &relation_aggregates,
        ).await?;
        let facets = execute_facets(ctx, &query, &query_options)
            .await
            .map_err(DataServiceError::Runtime)?;
        attach_facets(&mut rows, facets);
        Ok(rows)
    }

    pub(crate) async fn _execute_for_stream<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .system_info_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let query_options = self.query_options.clone();
        let query = apply_runtime_metadata(self.query, &query_options, &self.child_enhancements);
        let chunks = repository.fetch_stream(&query)
            .await?;
        Ok(chunks)
    }

    pub(crate) async fn _execute_for_first<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Option<R>, TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let rows = self.limit(1)._execute_for_list(ctx).await?;
        Ok(rows.into_iter().next())
    }

    pub(crate) async fn _execute_for_one<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Option<R>, TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        self._execute_for_first(ctx).await
    }


    pub(crate) async fn _execute_for_page<'a, C>(
        self,
        ctx: &'a C,
        offset: u64,
        limit: u64,
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let total_count = self.clone()._execute_for_count(ctx).await?;
        let mut rows = self.page_offset(offset, limit)._execute_for_list(ctx).await?;
        rows.total_count = Some(total_count);
        Ok(rows)
    }

    pub(crate) async fn _execute_for_count<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<u64, TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .system_info_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query;
        query.projection.clear();
        query.expr_projection.clear();
        query.order_by.clear();
        query.slice = None;
        query.relations.clear();
        query = query.count(COUNT_ALIAS);
        let rows = repository.fetch_all(&query).await?;
        rows.first()
            .and_then(|row| row.get(COUNT_ALIAS))
            .and_then(teaql_core::Value::try_u64)
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for SystemInfo is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .system_info_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .system_info_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let query_options = self.query_options.clone();
        let outer_query = self.query.clone();
        let relation_aggregates = runtime_relation_aggregates(&query_options);
        let query = apply_runtime_metadata(self.query, &query_options, &self.child_enhancements);
        let mut rows = repository.fetch_smart_list_with_relation_aggregates(&query, &relation_aggregates).await?;
        let facets = execute_facets(ctx, &outer_query, &query_options)
            .await
            .map_err(DataServiceError::Runtime)?;
        attach_facets(&mut rows, facets);
        Ok(rows)
    }

    pub(crate) async fn _execute_for_record<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let records = self.limit(1)._execute_for_records(ctx).await?;
        Ok(records.into_iter().next())
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
            "hostname" => Some("hostname"),
            "cpu_count" => Some("cpu_count"),
            "memory_total_bytes" => Some("memory_total_bytes"),
            "memory_available_bytes" => Some("memory_available_bytes"),
            "load_avg_1" => Some("load_avg_1"),
            "load_avg_5" => Some("load_avg_5"),
            "load_avg_15" => Some("load_avg_15"),
            "uptime_seconds" => Some("uptime_seconds"),
            "create_time" => Some("create_time"),
            "update_time" => Some("update_time"),
            "version" => Some("version"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "process_list" => {
                self.with_process_list_matching(
                    crate::Q::processes_minimal()
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
        self.query = self.query.project("hostname");
        self.query = self.query.project("cpu_count");
        self.query = self.query.project("memory_total_bytes");
        self.query = self.query.project("memory_available_bytes");
        self.query = self.query.project("load_avg_1");
        self.query = self.query.project("load_avg_5");
        self.query = self.query.project("load_avg_15");
        self.query = self.query.project("uptime_seconds");
        self.query = self.query.project("create_time");
        self.query = self.query.project("update_time");
        self.query = self.query.project("version");
        self
    }

    pub fn select_self_fields(self) -> Self {
        self.select_self()
    }

    pub fn select_self_without_parent(self) -> Self {
        self.select_self_fields()
    }

    pub fn select_all(self) -> Self {
        self.select_self()
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_process_list();
        request
    }

    pub fn select_any(self) -> Self {
        self.select_children()
    }

    pub fn group_by(mut self, field: impl Into<String>) -> Self {
        self.query = self.query.group_by(field);
        self
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


    pub fn select_hostname(mut self) -> Self {
        self.query = self.query.project("hostname");
        self
    }

    pub fn project_hostname(self) -> Self {
        self.select_hostname()
    }

    pub fn select_hostname_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_hostname_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_hostname_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("hostname", raw_sql_segment));
        self
    }

    pub fn group_by_hostname(self) -> Self {
        self.group_by("hostname")
    }

    pub fn group_by_hostname_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("hostname");
        request.query = request
            .query
            .project_expr(alias, Expr::column("hostname"));
        request
    }

    pub fn group_by_hostname_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("hostname")
            .aggregate_with_function("hostname", alias, function)
    }

    pub fn count_hostname(self) -> Self {
        self.count_hostname_as("hostname_count")
    }

    pub fn count_hostname_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("hostname", alias)
    }

    pub fn sum_hostname(self) -> Self {
        self.sum_hostname_as("sum_hostname")
    }

    pub fn sum_hostname_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("hostname", alias)
    }

    pub fn avg_hostname(self) -> Self {
        self.avg_hostname_as("avg_hostname")
    }

    pub fn avg_hostname_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("hostname", alias)
    }

    pub fn min_hostname(self) -> Self {
        self.min_hostname_as("min_hostname")
    }

    pub fn min_hostname_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("hostname", alias)
    }

    pub fn max_hostname(self) -> Self {
        self.max_hostname_as("max_hostname")
    }

    pub fn max_hostname_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("hostname", alias)
    }

    pub fn unselect_hostname(mut self) -> Self {
        self.query.projection.retain(|field| field != "hostname");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "hostname");
        self
    }


    pub fn with_hostname(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "hostname",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_hostname_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "hostname",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_hostname_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("hostname", value));
        self
    }



    pub fn with_hostname_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("hostname", value));
        self
    }

    pub fn with_hostname_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("hostname", value));
        self
    }

    pub fn with_hostname_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("hostname", value));
        self
    }

    pub fn with_hostname_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("hostname", value));
        self
    }

    pub fn with_hostname_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("hostname", value));
        self
    }

    pub fn with_hostname_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("hostname", lower, upper));
        self
    }

    pub fn with_hostname_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "hostname",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_hostname_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "hostname",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_hostname_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "hostname",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_hostname_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("hostname", value));
        self
    }

    pub fn with_hostname_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("hostname", value));
        self
    }

    pub fn with_hostname_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("hostname", value));
        self
    }

    pub fn with_hostname_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("hostname", value));
        self
    }

    pub fn with_hostname_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("hostname", value));
        self
    }

    pub fn with_hostname_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("hostname", value));
        self
    }

    pub fn with_hostname_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("hostname", value));
        self
    }
    pub fn with_hostname_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("hostname", value));
        self
    }

    pub fn with_hostname_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("hostname", value));
        self
    }

    pub fn with_hostname_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("hostname"));
        self
    }



    pub fn with_hostname_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("hostname"));
        self
    }


    pub fn order_by_hostname_asc(mut self) -> Self {
        self.query = self.query.order_asc("hostname");
        self
    }

    pub fn order_by_hostname_desc(mut self) -> Self {
        self.query = self.query.order_desc("hostname");
        self
    }

    pub fn order_by_hostname_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("hostname");
        self
    }

    pub fn order_by_hostname_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("hostname");
        self
    }


    pub fn select_cpu_count(mut self) -> Self {
        self.query = self.query.project("cpu_count");
        self
    }

    pub fn project_cpu_count(self) -> Self {
        self.select_cpu_count()
    }

    pub fn select_cpu_count_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_cpu_count_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_cpu_count_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("cpu_count", raw_sql_segment));
        self
    }

    pub fn select_cpu_count_with_function(self, function: AggregateFunction) -> Self {
        self.select_cpu_count_as_with_function("cpu_count", function)
    }

    pub fn select_cpu_count_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("cpu_count", alias, function)
    }

    pub fn group_by_cpu_count(self) -> Self {
        self.group_by("cpu_count")
    }

    pub fn group_by_cpu_count_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("cpu_count");
        request.query = request
            .query
            .project_expr(alias, Expr::column("cpu_count"));
        request
    }

    pub fn group_by_cpu_count_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("cpu_count")
            .aggregate_with_function("cpu_count", alias, function)
    }

    pub fn count_cpu_count(self) -> Self {
        self.count_cpu_count_as("cpu_count_count")
    }

    pub fn count_cpu_count_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("cpu_count", alias)
    }

    pub fn sum_cpu_count(self) -> Self {
        self.sum_cpu_count_as("sum_cpu_count")
    }

    pub fn sum_cpu_count_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("cpu_count", alias)
    }

    pub fn avg_cpu_count(self) -> Self {
        self.avg_cpu_count_as("avg_cpu_count")
    }

    pub fn avg_cpu_count_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("cpu_count", alias)
    }

    pub fn min_cpu_count(self) -> Self {
        self.min_cpu_count_as("min_cpu_count")
    }

    pub fn min_cpu_count_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("cpu_count", alias)
    }

    pub fn max_cpu_count(self) -> Self {
        self.max_cpu_count_as("max_cpu_count")
    }

    pub fn max_cpu_count_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("cpu_count", alias)
    }

    pub fn standard_deviation_cpu_count(self) -> Self {
        self.standard_deviation_cpu_count_as("stdDev_cpu_count")
    }

    pub fn standard_deviation_cpu_count_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("cpu_count", alias)
    }

    pub fn square_root_of_population_standard_deviation_cpu_count(self) -> Self {
        self.square_root_of_population_standard_deviation_cpu_count_as("stdDevPop_cpu_count")
    }

    pub fn square_root_of_population_standard_deviation_cpu_count_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("cpu_count", alias)
    }

    pub fn sample_variance_cpu_count(self) -> Self {
        self.sample_variance_cpu_count_as("varSamp_cpu_count")
    }

    pub fn sample_variance_cpu_count_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("cpu_count", alias)
    }

    pub fn sample_population_variance_cpu_count(self) -> Self {
        self.sample_population_variance_cpu_count_as("varPop_cpu_count")
    }

    pub fn sample_population_variance_cpu_count_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("cpu_count", alias)
    }

    pub fn unselect_cpu_count(mut self) -> Self {
        self.query.projection.retain(|field| field != "cpu_count");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "cpu_count");
        self
    }


    pub fn with_cpu_count(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "cpu_count",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_cpu_count_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "cpu_count",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_cpu_count_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("cpu_count", value));
        self
    }



    pub fn with_cpu_count_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("cpu_count", value));
        self
    }

    pub fn with_cpu_count_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("cpu_count", value));
        self
    }

    pub fn with_cpu_count_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("cpu_count", value));
        self
    }

    pub fn with_cpu_count_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("cpu_count", value));
        self
    }

    pub fn with_cpu_count_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("cpu_count", value));
        self
    }

    pub fn with_cpu_count_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("cpu_count", lower, upper));
        self
    }

    pub fn with_cpu_count_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "cpu_count",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_cpu_count_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "cpu_count",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_cpu_count_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "cpu_count",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_cpu_count_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("cpu_count", value));
        self
    }

    pub fn with_cpu_count_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("cpu_count", value));
        self
    }

    pub fn with_cpu_count_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("cpu_count"));
        self
    }



    pub fn with_cpu_count_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("cpu_count"));
        self
    }


    pub fn order_by_cpu_count_asc(mut self) -> Self {
        self.query = self.query.order_asc("cpu_count");
        self
    }

    pub fn order_by_cpu_count_desc(mut self) -> Self {
        self.query = self.query.order_desc("cpu_count");
        self
    }

    pub fn order_by_cpu_count_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("cpu_count");
        self
    }

    pub fn order_by_cpu_count_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("cpu_count");
        self
    }


    pub fn select_memory_total_bytes(mut self) -> Self {
        self.query = self.query.project("memory_total_bytes");
        self
    }

    pub fn project_memory_total_bytes(self) -> Self {
        self.select_memory_total_bytes()
    }

    pub fn select_memory_total_bytes_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_memory_total_bytes_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_memory_total_bytes_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("memory_total_bytes", raw_sql_segment));
        self
    }

    pub fn select_memory_total_bytes_with_function(self, function: AggregateFunction) -> Self {
        self.select_memory_total_bytes_as_with_function("memory_total_bytes", function)
    }

    pub fn select_memory_total_bytes_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("memory_total_bytes", alias, function)
    }

    pub fn group_by_memory_total_bytes(self) -> Self {
        self.group_by("memory_total_bytes")
    }

    pub fn group_by_memory_total_bytes_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("memory_total_bytes");
        request.query = request
            .query
            .project_expr(alias, Expr::column("memory_total_bytes"));
        request
    }

    pub fn group_by_memory_total_bytes_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("memory_total_bytes")
            .aggregate_with_function("memory_total_bytes", alias, function)
    }

    pub fn count_memory_total_bytes(self) -> Self {
        self.count_memory_total_bytes_as("memory_total_bytes_count")
    }

    pub fn count_memory_total_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("memory_total_bytes", alias)
    }

    pub fn sum_memory_total_bytes(self) -> Self {
        self.sum_memory_total_bytes_as("sum_memory_total_bytes")
    }

    pub fn sum_memory_total_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("memory_total_bytes", alias)
    }

    pub fn avg_memory_total_bytes(self) -> Self {
        self.avg_memory_total_bytes_as("avg_memory_total_bytes")
    }

    pub fn avg_memory_total_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("memory_total_bytes", alias)
    }

    pub fn min_memory_total_bytes(self) -> Self {
        self.min_memory_total_bytes_as("min_memory_total_bytes")
    }

    pub fn min_memory_total_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("memory_total_bytes", alias)
    }

    pub fn max_memory_total_bytes(self) -> Self {
        self.max_memory_total_bytes_as("max_memory_total_bytes")
    }

    pub fn max_memory_total_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("memory_total_bytes", alias)
    }

    pub fn standard_deviation_memory_total_bytes(self) -> Self {
        self.standard_deviation_memory_total_bytes_as("stdDev_memory_total_bytes")
    }

    pub fn standard_deviation_memory_total_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("memory_total_bytes", alias)
    }

    pub fn square_root_of_population_standard_deviation_memory_total_bytes(self) -> Self {
        self.square_root_of_population_standard_deviation_memory_total_bytes_as("stdDevPop_memory_total_bytes")
    }

    pub fn square_root_of_population_standard_deviation_memory_total_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("memory_total_bytes", alias)
    }

    pub fn sample_variance_memory_total_bytes(self) -> Self {
        self.sample_variance_memory_total_bytes_as("varSamp_memory_total_bytes")
    }

    pub fn sample_variance_memory_total_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("memory_total_bytes", alias)
    }

    pub fn sample_population_variance_memory_total_bytes(self) -> Self {
        self.sample_population_variance_memory_total_bytes_as("varPop_memory_total_bytes")
    }

    pub fn sample_population_variance_memory_total_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("memory_total_bytes", alias)
    }

    pub fn unselect_memory_total_bytes(mut self) -> Self {
        self.query.projection.retain(|field| field != "memory_total_bytes");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "memory_total_bytes");
        self
    }


    pub fn with_memory_total_bytes(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "memory_total_bytes",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_memory_total_bytes_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "memory_total_bytes",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_memory_total_bytes_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("memory_total_bytes", value));
        self
    }



    pub fn with_memory_total_bytes_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("memory_total_bytes", value));
        self
    }

    pub fn with_memory_total_bytes_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("memory_total_bytes", value));
        self
    }

    pub fn with_memory_total_bytes_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("memory_total_bytes", value));
        self
    }

    pub fn with_memory_total_bytes_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("memory_total_bytes", value));
        self
    }

    pub fn with_memory_total_bytes_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("memory_total_bytes", value));
        self
    }

    pub fn with_memory_total_bytes_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("memory_total_bytes", lower, upper));
        self
    }

    pub fn with_memory_total_bytes_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "memory_total_bytes",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_memory_total_bytes_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "memory_total_bytes",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_memory_total_bytes_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "memory_total_bytes",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_memory_total_bytes_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("memory_total_bytes", value));
        self
    }

    pub fn with_memory_total_bytes_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("memory_total_bytes", value));
        self
    }

    pub fn with_memory_total_bytes_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("memory_total_bytes"));
        self
    }



    pub fn with_memory_total_bytes_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("memory_total_bytes"));
        self
    }


    pub fn order_by_memory_total_bytes_asc(mut self) -> Self {
        self.query = self.query.order_asc("memory_total_bytes");
        self
    }

    pub fn order_by_memory_total_bytes_desc(mut self) -> Self {
        self.query = self.query.order_desc("memory_total_bytes");
        self
    }

    pub fn order_by_memory_total_bytes_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("memory_total_bytes");
        self
    }

    pub fn order_by_memory_total_bytes_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("memory_total_bytes");
        self
    }


    pub fn select_memory_available_bytes(mut self) -> Self {
        self.query = self.query.project("memory_available_bytes");
        self
    }

    pub fn project_memory_available_bytes(self) -> Self {
        self.select_memory_available_bytes()
    }

    pub fn select_memory_available_bytes_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_memory_available_bytes_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_memory_available_bytes_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("memory_available_bytes", raw_sql_segment));
        self
    }

    pub fn select_memory_available_bytes_with_function(self, function: AggregateFunction) -> Self {
        self.select_memory_available_bytes_as_with_function("memory_available_bytes", function)
    }

    pub fn select_memory_available_bytes_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("memory_available_bytes", alias, function)
    }

    pub fn group_by_memory_available_bytes(self) -> Self {
        self.group_by("memory_available_bytes")
    }

    pub fn group_by_memory_available_bytes_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("memory_available_bytes");
        request.query = request
            .query
            .project_expr(alias, Expr::column("memory_available_bytes"));
        request
    }

    pub fn group_by_memory_available_bytes_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("memory_available_bytes")
            .aggregate_with_function("memory_available_bytes", alias, function)
    }

    pub fn count_memory_available_bytes(self) -> Self {
        self.count_memory_available_bytes_as("memory_available_bytes_count")
    }

    pub fn count_memory_available_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("memory_available_bytes", alias)
    }

    pub fn sum_memory_available_bytes(self) -> Self {
        self.sum_memory_available_bytes_as("sum_memory_available_bytes")
    }

    pub fn sum_memory_available_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("memory_available_bytes", alias)
    }

    pub fn avg_memory_available_bytes(self) -> Self {
        self.avg_memory_available_bytes_as("avg_memory_available_bytes")
    }

    pub fn avg_memory_available_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("memory_available_bytes", alias)
    }

    pub fn min_memory_available_bytes(self) -> Self {
        self.min_memory_available_bytes_as("min_memory_available_bytes")
    }

    pub fn min_memory_available_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("memory_available_bytes", alias)
    }

    pub fn max_memory_available_bytes(self) -> Self {
        self.max_memory_available_bytes_as("max_memory_available_bytes")
    }

    pub fn max_memory_available_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("memory_available_bytes", alias)
    }

    pub fn standard_deviation_memory_available_bytes(self) -> Self {
        self.standard_deviation_memory_available_bytes_as("stdDev_memory_available_bytes")
    }

    pub fn standard_deviation_memory_available_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("memory_available_bytes", alias)
    }

    pub fn square_root_of_population_standard_deviation_memory_available_bytes(self) -> Self {
        self.square_root_of_population_standard_deviation_memory_available_bytes_as("stdDevPop_memory_available_bytes")
    }

    pub fn square_root_of_population_standard_deviation_memory_available_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("memory_available_bytes", alias)
    }

    pub fn sample_variance_memory_available_bytes(self) -> Self {
        self.sample_variance_memory_available_bytes_as("varSamp_memory_available_bytes")
    }

    pub fn sample_variance_memory_available_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("memory_available_bytes", alias)
    }

    pub fn sample_population_variance_memory_available_bytes(self) -> Self {
        self.sample_population_variance_memory_available_bytes_as("varPop_memory_available_bytes")
    }

    pub fn sample_population_variance_memory_available_bytes_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("memory_available_bytes", alias)
    }

    pub fn unselect_memory_available_bytes(mut self) -> Self {
        self.query.projection.retain(|field| field != "memory_available_bytes");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "memory_available_bytes");
        self
    }


    pub fn with_memory_available_bytes(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "memory_available_bytes",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_memory_available_bytes_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "memory_available_bytes",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_memory_available_bytes_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("memory_available_bytes", value));
        self
    }



    pub fn with_memory_available_bytes_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("memory_available_bytes", value));
        self
    }

    pub fn with_memory_available_bytes_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("memory_available_bytes", value));
        self
    }

    pub fn with_memory_available_bytes_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("memory_available_bytes", value));
        self
    }

    pub fn with_memory_available_bytes_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("memory_available_bytes", value));
        self
    }

    pub fn with_memory_available_bytes_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("memory_available_bytes", value));
        self
    }

    pub fn with_memory_available_bytes_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("memory_available_bytes", lower, upper));
        self
    }

    pub fn with_memory_available_bytes_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "memory_available_bytes",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_memory_available_bytes_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "memory_available_bytes",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_memory_available_bytes_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "memory_available_bytes",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_memory_available_bytes_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("memory_available_bytes", value));
        self
    }

    pub fn with_memory_available_bytes_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("memory_available_bytes", value));
        self
    }

    pub fn with_memory_available_bytes_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("memory_available_bytes"));
        self
    }



    pub fn with_memory_available_bytes_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("memory_available_bytes"));
        self
    }


    pub fn order_by_memory_available_bytes_asc(mut self) -> Self {
        self.query = self.query.order_asc("memory_available_bytes");
        self
    }

    pub fn order_by_memory_available_bytes_desc(mut self) -> Self {
        self.query = self.query.order_desc("memory_available_bytes");
        self
    }

    pub fn order_by_memory_available_bytes_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("memory_available_bytes");
        self
    }

    pub fn order_by_memory_available_bytes_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("memory_available_bytes");
        self
    }


    pub fn select_load_avg_1(mut self) -> Self {
        self.query = self.query.project("load_avg_1");
        self
    }

    pub fn project_load_avg_1(self) -> Self {
        self.select_load_avg_1()
    }

    pub fn select_load_avg_1_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_load_avg_1_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_load_avg_1_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("load_avg_1", raw_sql_segment));
        self
    }

    pub fn select_load_avg_1_with_function(self, function: AggregateFunction) -> Self {
        self.select_load_avg_1_as_with_function("load_avg_1", function)
    }

    pub fn select_load_avg_1_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("load_avg_1", alias, function)
    }

    pub fn group_by_load_avg_1(self) -> Self {
        self.group_by("load_avg_1")
    }

    pub fn group_by_load_avg_1_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("load_avg_1");
        request.query = request
            .query
            .project_expr(alias, Expr::column("load_avg_1"));
        request
    }

    pub fn group_by_load_avg_1_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("load_avg_1")
            .aggregate_with_function("load_avg_1", alias, function)
    }

    pub fn count_load_avg_1(self) -> Self {
        self.count_load_avg_1_as("load_avg_1_count")
    }

    pub fn count_load_avg_1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("load_avg_1", alias)
    }

    pub fn sum_load_avg_1(self) -> Self {
        self.sum_load_avg_1_as("sum_load_avg_1")
    }

    pub fn sum_load_avg_1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("load_avg_1", alias)
    }

    pub fn avg_load_avg_1(self) -> Self {
        self.avg_load_avg_1_as("avg_load_avg_1")
    }

    pub fn avg_load_avg_1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("load_avg_1", alias)
    }

    pub fn min_load_avg_1(self) -> Self {
        self.min_load_avg_1_as("min_load_avg_1")
    }

    pub fn min_load_avg_1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("load_avg_1", alias)
    }

    pub fn max_load_avg_1(self) -> Self {
        self.max_load_avg_1_as("max_load_avg_1")
    }

    pub fn max_load_avg_1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("load_avg_1", alias)
    }

    pub fn standard_deviation_load_avg_1(self) -> Self {
        self.standard_deviation_load_avg_1_as("stdDev_load_avg_1")
    }

    pub fn standard_deviation_load_avg_1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("load_avg_1", alias)
    }

    pub fn square_root_of_population_standard_deviation_load_avg_1(self) -> Self {
        self.square_root_of_population_standard_deviation_load_avg_1_as("stdDevPop_load_avg_1")
    }

    pub fn square_root_of_population_standard_deviation_load_avg_1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("load_avg_1", alias)
    }

    pub fn sample_variance_load_avg_1(self) -> Self {
        self.sample_variance_load_avg_1_as("varSamp_load_avg_1")
    }

    pub fn sample_variance_load_avg_1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("load_avg_1", alias)
    }

    pub fn sample_population_variance_load_avg_1(self) -> Self {
        self.sample_population_variance_load_avg_1_as("varPop_load_avg_1")
    }

    pub fn sample_population_variance_load_avg_1_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("load_avg_1", alias)
    }

    pub fn unselect_load_avg_1(mut self) -> Self {
        self.query.projection.retain(|field| field != "load_avg_1");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "load_avg_1");
        self
    }


    pub fn with_load_avg1(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "load_avg_1",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_load_avg_1_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "load_avg_1",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_load_avg1_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("load_avg_1", value));
        self
    }



    pub fn with_load_avg1_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("load_avg_1", value));
        self
    }

    pub fn with_load_avg1_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("load_avg_1", value));
        self
    }

    pub fn with_load_avg1_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("load_avg_1", value));
        self
    }

    pub fn with_load_avg1_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("load_avg_1", value));
        self
    }

    pub fn with_load_avg1_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("load_avg_1", value));
        self
    }

    pub fn with_load_avg1_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("load_avg_1", lower, upper));
        self
    }

    pub fn with_load_avg1_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "load_avg_1",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_load_avg1_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "load_avg_1",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_load_avg1_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "load_avg_1",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_load_avg1_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("load_avg_1", value));
        self
    }

    pub fn with_load_avg1_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("load_avg_1", value));
        self
    }

    pub fn with_load_avg1_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("load_avg_1"));
        self
    }



    pub fn with_load_avg1_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("load_avg_1"));
        self
    }


    pub fn order_by_load_avg_1_asc(mut self) -> Self {
        self.query = self.query.order_asc("load_avg_1");
        self
    }

    pub fn order_by_load_avg_1_desc(mut self) -> Self {
        self.query = self.query.order_desc("load_avg_1");
        self
    }

    pub fn order_by_load_avg_1_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("load_avg_1");
        self
    }

    pub fn order_by_load_avg_1_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("load_avg_1");
        self
    }


    pub fn select_load_avg_5(mut self) -> Self {
        self.query = self.query.project("load_avg_5");
        self
    }

    pub fn project_load_avg_5(self) -> Self {
        self.select_load_avg_5()
    }

    pub fn select_load_avg_5_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_load_avg_5_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_load_avg_5_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("load_avg_5", raw_sql_segment));
        self
    }

    pub fn select_load_avg_5_with_function(self, function: AggregateFunction) -> Self {
        self.select_load_avg_5_as_with_function("load_avg_5", function)
    }

    pub fn select_load_avg_5_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("load_avg_5", alias, function)
    }

    pub fn group_by_load_avg_5(self) -> Self {
        self.group_by("load_avg_5")
    }

    pub fn group_by_load_avg_5_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("load_avg_5");
        request.query = request
            .query
            .project_expr(alias, Expr::column("load_avg_5"));
        request
    }

    pub fn group_by_load_avg_5_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("load_avg_5")
            .aggregate_with_function("load_avg_5", alias, function)
    }

    pub fn count_load_avg_5(self) -> Self {
        self.count_load_avg_5_as("load_avg_5_count")
    }

    pub fn count_load_avg_5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("load_avg_5", alias)
    }

    pub fn sum_load_avg_5(self) -> Self {
        self.sum_load_avg_5_as("sum_load_avg_5")
    }

    pub fn sum_load_avg_5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("load_avg_5", alias)
    }

    pub fn avg_load_avg_5(self) -> Self {
        self.avg_load_avg_5_as("avg_load_avg_5")
    }

    pub fn avg_load_avg_5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("load_avg_5", alias)
    }

    pub fn min_load_avg_5(self) -> Self {
        self.min_load_avg_5_as("min_load_avg_5")
    }

    pub fn min_load_avg_5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("load_avg_5", alias)
    }

    pub fn max_load_avg_5(self) -> Self {
        self.max_load_avg_5_as("max_load_avg_5")
    }

    pub fn max_load_avg_5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("load_avg_5", alias)
    }

    pub fn standard_deviation_load_avg_5(self) -> Self {
        self.standard_deviation_load_avg_5_as("stdDev_load_avg_5")
    }

    pub fn standard_deviation_load_avg_5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("load_avg_5", alias)
    }

    pub fn square_root_of_population_standard_deviation_load_avg_5(self) -> Self {
        self.square_root_of_population_standard_deviation_load_avg_5_as("stdDevPop_load_avg_5")
    }

    pub fn square_root_of_population_standard_deviation_load_avg_5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("load_avg_5", alias)
    }

    pub fn sample_variance_load_avg_5(self) -> Self {
        self.sample_variance_load_avg_5_as("varSamp_load_avg_5")
    }

    pub fn sample_variance_load_avg_5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("load_avg_5", alias)
    }

    pub fn sample_population_variance_load_avg_5(self) -> Self {
        self.sample_population_variance_load_avg_5_as("varPop_load_avg_5")
    }

    pub fn sample_population_variance_load_avg_5_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("load_avg_5", alias)
    }

    pub fn unselect_load_avg_5(mut self) -> Self {
        self.query.projection.retain(|field| field != "load_avg_5");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "load_avg_5");
        self
    }


    pub fn with_load_avg5(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "load_avg_5",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_load_avg_5_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "load_avg_5",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_load_avg5_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("load_avg_5", value));
        self
    }



    pub fn with_load_avg5_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("load_avg_5", value));
        self
    }

    pub fn with_load_avg5_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("load_avg_5", value));
        self
    }

    pub fn with_load_avg5_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("load_avg_5", value));
        self
    }

    pub fn with_load_avg5_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("load_avg_5", value));
        self
    }

    pub fn with_load_avg5_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("load_avg_5", value));
        self
    }

    pub fn with_load_avg5_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("load_avg_5", lower, upper));
        self
    }

    pub fn with_load_avg5_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "load_avg_5",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_load_avg5_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "load_avg_5",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_load_avg5_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "load_avg_5",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_load_avg5_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("load_avg_5", value));
        self
    }

    pub fn with_load_avg5_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("load_avg_5", value));
        self
    }

    pub fn with_load_avg5_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("load_avg_5"));
        self
    }



    pub fn with_load_avg5_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("load_avg_5"));
        self
    }


    pub fn order_by_load_avg_5_asc(mut self) -> Self {
        self.query = self.query.order_asc("load_avg_5");
        self
    }

    pub fn order_by_load_avg_5_desc(mut self) -> Self {
        self.query = self.query.order_desc("load_avg_5");
        self
    }

    pub fn order_by_load_avg_5_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("load_avg_5");
        self
    }

    pub fn order_by_load_avg_5_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("load_avg_5");
        self
    }


    pub fn select_load_avg_15(mut self) -> Self {
        self.query = self.query.project("load_avg_15");
        self
    }

    pub fn project_load_avg_15(self) -> Self {
        self.select_load_avg_15()
    }

    pub fn select_load_avg_15_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_load_avg_15_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_load_avg_15_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("load_avg_15", raw_sql_segment));
        self
    }

    pub fn select_load_avg_15_with_function(self, function: AggregateFunction) -> Self {
        self.select_load_avg_15_as_with_function("load_avg_15", function)
    }

    pub fn select_load_avg_15_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("load_avg_15", alias, function)
    }

    pub fn group_by_load_avg_15(self) -> Self {
        self.group_by("load_avg_15")
    }

    pub fn group_by_load_avg_15_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("load_avg_15");
        request.query = request
            .query
            .project_expr(alias, Expr::column("load_avg_15"));
        request
    }

    pub fn group_by_load_avg_15_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("load_avg_15")
            .aggregate_with_function("load_avg_15", alias, function)
    }

    pub fn count_load_avg_15(self) -> Self {
        self.count_load_avg_15_as("load_avg_15_count")
    }

    pub fn count_load_avg_15_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("load_avg_15", alias)
    }

    pub fn sum_load_avg_15(self) -> Self {
        self.sum_load_avg_15_as("sum_load_avg_15")
    }

    pub fn sum_load_avg_15_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("load_avg_15", alias)
    }

    pub fn avg_load_avg_15(self) -> Self {
        self.avg_load_avg_15_as("avg_load_avg_15")
    }

    pub fn avg_load_avg_15_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("load_avg_15", alias)
    }

    pub fn min_load_avg_15(self) -> Self {
        self.min_load_avg_15_as("min_load_avg_15")
    }

    pub fn min_load_avg_15_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("load_avg_15", alias)
    }

    pub fn max_load_avg_15(self) -> Self {
        self.max_load_avg_15_as("max_load_avg_15")
    }

    pub fn max_load_avg_15_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("load_avg_15", alias)
    }

    pub fn standard_deviation_load_avg_15(self) -> Self {
        self.standard_deviation_load_avg_15_as("stdDev_load_avg_15")
    }

    pub fn standard_deviation_load_avg_15_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("load_avg_15", alias)
    }

    pub fn square_root_of_population_standard_deviation_load_avg_15(self) -> Self {
        self.square_root_of_population_standard_deviation_load_avg_15_as("stdDevPop_load_avg_15")
    }

    pub fn square_root_of_population_standard_deviation_load_avg_15_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("load_avg_15", alias)
    }

    pub fn sample_variance_load_avg_15(self) -> Self {
        self.sample_variance_load_avg_15_as("varSamp_load_avg_15")
    }

    pub fn sample_variance_load_avg_15_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("load_avg_15", alias)
    }

    pub fn sample_population_variance_load_avg_15(self) -> Self {
        self.sample_population_variance_load_avg_15_as("varPop_load_avg_15")
    }

    pub fn sample_population_variance_load_avg_15_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("load_avg_15", alias)
    }

    pub fn unselect_load_avg_15(mut self) -> Self {
        self.query.projection.retain(|field| field != "load_avg_15");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "load_avg_15");
        self
    }


    pub fn with_load_avg15(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "load_avg_15",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_load_avg_15_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "load_avg_15",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_load_avg15_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("load_avg_15", value));
        self
    }



    pub fn with_load_avg15_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("load_avg_15", value));
        self
    }

    pub fn with_load_avg15_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("load_avg_15", value));
        self
    }

    pub fn with_load_avg15_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("load_avg_15", value));
        self
    }

    pub fn with_load_avg15_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("load_avg_15", value));
        self
    }

    pub fn with_load_avg15_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("load_avg_15", value));
        self
    }

    pub fn with_load_avg15_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("load_avg_15", lower, upper));
        self
    }

    pub fn with_load_avg15_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "load_avg_15",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_load_avg15_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "load_avg_15",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_load_avg15_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "load_avg_15",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_load_avg15_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("load_avg_15", value));
        self
    }

    pub fn with_load_avg15_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("load_avg_15", value));
        self
    }

    pub fn with_load_avg15_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("load_avg_15"));
        self
    }



    pub fn with_load_avg15_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("load_avg_15"));
        self
    }


    pub fn order_by_load_avg_15_asc(mut self) -> Self {
        self.query = self.query.order_asc("load_avg_15");
        self
    }

    pub fn order_by_load_avg_15_desc(mut self) -> Self {
        self.query = self.query.order_desc("load_avg_15");
        self
    }

    pub fn order_by_load_avg_15_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("load_avg_15");
        self
    }

    pub fn order_by_load_avg_15_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("load_avg_15");
        self
    }


    pub fn select_uptime_seconds(mut self) -> Self {
        self.query = self.query.project("uptime_seconds");
        self
    }

    pub fn project_uptime_seconds(self) -> Self {
        self.select_uptime_seconds()
    }

    pub fn select_uptime_seconds_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_uptime_seconds_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_uptime_seconds_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("uptime_seconds", raw_sql_segment));
        self
    }

    pub fn select_uptime_seconds_with_function(self, function: AggregateFunction) -> Self {
        self.select_uptime_seconds_as_with_function("uptime_seconds", function)
    }

    pub fn select_uptime_seconds_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("uptime_seconds", alias, function)
    }

    pub fn group_by_uptime_seconds(self) -> Self {
        self.group_by("uptime_seconds")
    }

    pub fn group_by_uptime_seconds_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("uptime_seconds");
        request.query = request
            .query
            .project_expr(alias, Expr::column("uptime_seconds"));
        request
    }

    pub fn group_by_uptime_seconds_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("uptime_seconds")
            .aggregate_with_function("uptime_seconds", alias, function)
    }

    pub fn count_uptime_seconds(self) -> Self {
        self.count_uptime_seconds_as("uptime_seconds_count")
    }

    pub fn count_uptime_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("uptime_seconds", alias)
    }

    pub fn sum_uptime_seconds(self) -> Self {
        self.sum_uptime_seconds_as("sum_uptime_seconds")
    }

    pub fn sum_uptime_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("uptime_seconds", alias)
    }

    pub fn avg_uptime_seconds(self) -> Self {
        self.avg_uptime_seconds_as("avg_uptime_seconds")
    }

    pub fn avg_uptime_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("uptime_seconds", alias)
    }

    pub fn min_uptime_seconds(self) -> Self {
        self.min_uptime_seconds_as("min_uptime_seconds")
    }

    pub fn min_uptime_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("uptime_seconds", alias)
    }

    pub fn max_uptime_seconds(self) -> Self {
        self.max_uptime_seconds_as("max_uptime_seconds")
    }

    pub fn max_uptime_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("uptime_seconds", alias)
    }

    pub fn standard_deviation_uptime_seconds(self) -> Self {
        self.standard_deviation_uptime_seconds_as("stdDev_uptime_seconds")
    }

    pub fn standard_deviation_uptime_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("uptime_seconds", alias)
    }

    pub fn square_root_of_population_standard_deviation_uptime_seconds(self) -> Self {
        self.square_root_of_population_standard_deviation_uptime_seconds_as("stdDevPop_uptime_seconds")
    }

    pub fn square_root_of_population_standard_deviation_uptime_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("uptime_seconds", alias)
    }

    pub fn sample_variance_uptime_seconds(self) -> Self {
        self.sample_variance_uptime_seconds_as("varSamp_uptime_seconds")
    }

    pub fn sample_variance_uptime_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("uptime_seconds", alias)
    }

    pub fn sample_population_variance_uptime_seconds(self) -> Self {
        self.sample_population_variance_uptime_seconds_as("varPop_uptime_seconds")
    }

    pub fn sample_population_variance_uptime_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("uptime_seconds", alias)
    }

    pub fn unselect_uptime_seconds(mut self) -> Self {
        self.query.projection.retain(|field| field != "uptime_seconds");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "uptime_seconds");
        self
    }


    pub fn with_uptime_seconds(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "uptime_seconds",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_uptime_seconds_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "uptime_seconds",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_uptime_seconds_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("uptime_seconds", value));
        self
    }



    pub fn with_uptime_seconds_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("uptime_seconds", value));
        self
    }

    pub fn with_uptime_seconds_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("uptime_seconds", value));
        self
    }

    pub fn with_uptime_seconds_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("uptime_seconds", value));
        self
    }

    pub fn with_uptime_seconds_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("uptime_seconds", value));
        self
    }

    pub fn with_uptime_seconds_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("uptime_seconds", value));
        self
    }

    pub fn with_uptime_seconds_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("uptime_seconds", lower, upper));
        self
    }

    pub fn with_uptime_seconds_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "uptime_seconds",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_uptime_seconds_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "uptime_seconds",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_uptime_seconds_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "uptime_seconds",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_uptime_seconds_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("uptime_seconds", value));
        self
    }

    pub fn with_uptime_seconds_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("uptime_seconds", value));
        self
    }

    pub fn with_uptime_seconds_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("uptime_seconds"));
        self
    }



    pub fn with_uptime_seconds_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("uptime_seconds"));
        self
    }


    pub fn order_by_uptime_seconds_asc(mut self) -> Self {
        self.query = self.query.order_asc("uptime_seconds");
        self
    }

    pub fn order_by_uptime_seconds_desc(mut self) -> Self {
        self.query = self.query.order_desc("uptime_seconds");
        self
    }

    pub fn order_by_uptime_seconds_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("uptime_seconds");
        self
    }

    pub fn order_by_uptime_seconds_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("uptime_seconds");
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
    pub fn hostname_is_localhost(self) -> Self {
        self.with_hostname_is("localhost")
    }

    pub fn with_hostname_is_localhost(self) -> Self {
        self.with_hostname_is("localhost")
    }



    pub fn with_hostname_is_not_localhost(self) -> Self {
        self.with_hostname_is_not("localhost")
    }



    pub fn cpu_count_is_value_16(self) -> Self {
        self.with_cpu_count_is("16")
    }

    pub fn with_cpu_count_is_value_16(self) -> Self {
        self.with_cpu_count_is("16")
    }



    pub fn with_cpu_count_is_not_value_16(self) -> Self {
        self.with_cpu_count_is_not("16")
    }



    pub fn memory_total_bytes_is_value_85899345920(self) -> Self {
        self.with_memory_total_bytes_is("85899345920")
    }

    pub fn with_memory_total_bytes_is_value_85899345920(self) -> Self {
        self.with_memory_total_bytes_is("85899345920")
    }



    pub fn with_memory_total_bytes_is_not_value_85899345920(self) -> Self {
        self.with_memory_total_bytes_is_not("85899345920")
    }



    pub fn memory_available_bytes_is_value_85899345920(self) -> Self {
        self.with_memory_available_bytes_is("85899345920")
    }

    pub fn with_memory_available_bytes_is_value_85899345920(self) -> Self {
        self.with_memory_available_bytes_is("85899345920")
    }



    pub fn with_memory_available_bytes_is_not_value_85899345920(self) -> Self {
        self.with_memory_available_bytes_is_not("85899345920")
    }



    pub fn load_avg_1_is_value_1_5(self) -> Self {
        self.with_load_avg1_is("1.5")
    }

    pub fn with_load_avg1_is_value_1_5(self) -> Self {
        self.with_load_avg1_is("1.5")
    }



    pub fn with_load_avg1_is_not_value_1_5(self) -> Self {
        self.with_load_avg1_is_not("1.5")
    }



    pub fn load_avg_5_is_value_1_2(self) -> Self {
        self.with_load_avg5_is("1.2")
    }

    pub fn with_load_avg5_is_value_1_2(self) -> Self {
        self.with_load_avg5_is("1.2")
    }



    pub fn with_load_avg5_is_not_value_1_2(self) -> Self {
        self.with_load_avg5_is_not("1.2")
    }



    pub fn load_avg_15_is_value_1_0(self) -> Self {
        self.with_load_avg15_is("1.0")
    }

    pub fn with_load_avg15_is_value_1_0(self) -> Self {
        self.with_load_avg15_is("1.0")
    }



    pub fn with_load_avg15_is_not_value_1_0(self) -> Self {
        self.with_load_avg15_is_not("1.0")
    }



    pub fn uptime_seconds_is_value_3600_0(self) -> Self {
        self.with_uptime_seconds_is("3600.0")
    }

    pub fn with_uptime_seconds_is_value_3600_0(self) -> Self {
        self.with_uptime_seconds_is("3600.0")
    }



    pub fn with_uptime_seconds_is_not_value_3600_0(self) -> Self {
        self.with_uptime_seconds_is_not("3600.0")
    }



    pub fn create_time_is_create_time(self) -> Self {
        self.with_create_time_is("createTime()")
    }

    pub fn with_create_time_is_create_time(self) -> Self {
        self.with_create_time_is("createTime()")
    }



    pub fn with_create_time_is_not_create_time(self) -> Self {
        self.with_create_time_is_not("createTime()")
    }



    pub fn update_time_is_update_time(self) -> Self {
        self.with_update_time_is("updateTime()")
    }

    pub fn with_update_time_is_update_time(self) -> Self {
        self.with_update_time_is("updateTime()")
    }



    pub fn with_update_time_is_not_update_time(self) -> Self {
        self.with_update_time_is_not("updateTime()")
    }




    pub fn have_processes(self) -> Self {
        self.with_process_list_matching(SelectQuery::new("Process"))
    }

    pub fn have_no_processes(self) -> Self {
        self.without_process_list_matching(SelectQuery::new("Process"))
    }

    pub fn with_process_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Process as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "system_info_id",
        ));
        self.relation_filters.push(RelationFilter::new("process_list", selection));
        self
    }

    pub fn without_process_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Process as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "system_info_id",
        ));
        self.relation_filters.push(RelationFilter::new("process_list", selection));
        self
    }

    pub fn select_process_list(mut self) -> Self {
        self.query = self.query.relation("process_list");
        self
    }

    pub fn select_process_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("process_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("process_list", selection));
        self
}
    pub fn count_processes(self) -> Self {
        self.count_processes_as("count_processes")
    }

    pub fn count_processes_as(self, alias: impl Into<String>) -> Self {
        self.count_processes_with(alias, crate::Q::processes().unlimited())
    }

    pub fn count_processes_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "process_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_processes(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as("refinements", request)
    }

    pub fn stats_from_processes_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "process_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_processes_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes(request)
    }


    pub fn sum_pid_of_processes(self) -> Self {
        self.sum_pid_of_processes_as("sum_pid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sum_pid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().sum("pid", "sum_pid"))
    }
    pub fn min_pid_of_processes(self) -> Self {
        self.min_pid_of_processes_as("min_pid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn min_pid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().min("pid", "min_pid"))
    }
    pub fn max_pid_of_processes(self) -> Self {
        self.max_pid_of_processes_as("max_pid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn max_pid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().max("pid", "max_pid"))
    }
    pub fn avg_pid_of_processes(self) -> Self {
        self.avg_pid_of_processes_as("avg_pid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn avg_pid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().avg("pid", "avg_pid"))
    }
    pub fn standard_deviation_pid_of_processes(self) -> Self {
        self.standard_deviation_pid_of_processes_as("standard_deviation_pid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn standard_deviation_pid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().stddev("pid", "stdDev_pid"))
    }
    pub fn square_root_of_population_standard_deviation_pid_of_processes(self) -> Self {
        self.square_root_of_population_standard_deviation_pid_of_processes_as("square_root_of_population_standard_deviation_pid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_pid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().stddev_pop("pid", "stdDevPop_pid"))
    }
    pub fn sample_variance_pid_of_processes(self) -> Self {
        self.sample_variance_pid_of_processes_as("sample_variance_pid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sample_variance_pid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().var_samp("pid", "varSamp_pid"))
    }
    pub fn sample_population_variance_pid_of_processes(self) -> Self {
        self.sample_population_variance_pid_of_processes_as("sample_population_variance_pid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sample_population_variance_pid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().var_pop("pid", "varPop_pid"))
    }
    pub fn sum_ppid_of_processes(self) -> Self {
        self.sum_ppid_of_processes_as("sum_ppid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sum_ppid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().sum("ppid", "sum_ppid"))
    }
    pub fn min_ppid_of_processes(self) -> Self {
        self.min_ppid_of_processes_as("min_ppid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn min_ppid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().min("ppid", "min_ppid"))
    }
    pub fn max_ppid_of_processes(self) -> Self {
        self.max_ppid_of_processes_as("max_ppid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn max_ppid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().max("ppid", "max_ppid"))
    }
    pub fn avg_ppid_of_processes(self) -> Self {
        self.avg_ppid_of_processes_as("avg_ppid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn avg_ppid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().avg("ppid", "avg_ppid"))
    }
    pub fn standard_deviation_ppid_of_processes(self) -> Self {
        self.standard_deviation_ppid_of_processes_as("standard_deviation_ppid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn standard_deviation_ppid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().stddev("ppid", "stdDev_ppid"))
    }
    pub fn square_root_of_population_standard_deviation_ppid_of_processes(self) -> Self {
        self.square_root_of_population_standard_deviation_ppid_of_processes_as("square_root_of_population_standard_deviation_ppid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_ppid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().stddev_pop("ppid", "stdDevPop_ppid"))
    }
    pub fn sample_variance_ppid_of_processes(self) -> Self {
        self.sample_variance_ppid_of_processes_as("sample_variance_ppid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sample_variance_ppid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().var_samp("ppid", "varSamp_ppid"))
    }
    pub fn sample_population_variance_ppid_of_processes(self) -> Self {
        self.sample_population_variance_ppid_of_processes_as("sample_population_variance_ppid_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sample_population_variance_ppid_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().var_pop("ppid", "varPop_ppid"))
    }
    pub fn sum_thread_count_of_processes(self) -> Self {
        self.sum_thread_count_of_processes_as("sum_thread_count_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sum_thread_count_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().sum("thread_count", "sum_thread_count"))
    }
    pub fn min_thread_count_of_processes(self) -> Self {
        self.min_thread_count_of_processes_as("min_thread_count_of_processes", crate::Q::processes().unlimited())
    }

    pub fn min_thread_count_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().min("thread_count", "min_thread_count"))
    }
    pub fn max_thread_count_of_processes(self) -> Self {
        self.max_thread_count_of_processes_as("max_thread_count_of_processes", crate::Q::processes().unlimited())
    }

    pub fn max_thread_count_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().max("thread_count", "max_thread_count"))
    }
    pub fn avg_thread_count_of_processes(self) -> Self {
        self.avg_thread_count_of_processes_as("avg_thread_count_of_processes", crate::Q::processes().unlimited())
    }

    pub fn avg_thread_count_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().avg("thread_count", "avg_thread_count"))
    }
    pub fn standard_deviation_thread_count_of_processes(self) -> Self {
        self.standard_deviation_thread_count_of_processes_as("standard_deviation_thread_count_of_processes", crate::Q::processes().unlimited())
    }

    pub fn standard_deviation_thread_count_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().stddev("thread_count", "stdDev_thread_count"))
    }
    pub fn square_root_of_population_standard_deviation_thread_count_of_processes(self) -> Self {
        self.square_root_of_population_standard_deviation_thread_count_of_processes_as("square_root_of_population_standard_deviation_thread_count_of_processes", crate::Q::processes().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_thread_count_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().stddev_pop("thread_count", "stdDevPop_thread_count"))
    }
    pub fn sample_variance_thread_count_of_processes(self) -> Self {
        self.sample_variance_thread_count_of_processes_as("sample_variance_thread_count_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sample_variance_thread_count_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().var_samp("thread_count", "varSamp_thread_count"))
    }
    pub fn sample_population_variance_thread_count_of_processes(self) -> Self {
        self.sample_population_variance_thread_count_of_processes_as("sample_population_variance_thread_count_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sample_population_variance_thread_count_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().var_pop("thread_count", "varPop_thread_count"))
    }
    pub fn sum_memory_rss_kb_of_processes(self) -> Self {
        self.sum_memory_rss_kb_of_processes_as("sum_memory_rss_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sum_memory_rss_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().sum("memory_rss_kb", "sum_memory_rss_kb"))
    }
    pub fn min_memory_rss_kb_of_processes(self) -> Self {
        self.min_memory_rss_kb_of_processes_as("min_memory_rss_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn min_memory_rss_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().min("memory_rss_kb", "min_memory_rss_kb"))
    }
    pub fn max_memory_rss_kb_of_processes(self) -> Self {
        self.max_memory_rss_kb_of_processes_as("max_memory_rss_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn max_memory_rss_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().max("memory_rss_kb", "max_memory_rss_kb"))
    }
    pub fn avg_memory_rss_kb_of_processes(self) -> Self {
        self.avg_memory_rss_kb_of_processes_as("avg_memory_rss_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn avg_memory_rss_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().avg("memory_rss_kb", "avg_memory_rss_kb"))
    }
    pub fn standard_deviation_memory_rss_kb_of_processes(self) -> Self {
        self.standard_deviation_memory_rss_kb_of_processes_as("standard_deviation_memory_rss_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn standard_deviation_memory_rss_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().stddev("memory_rss_kb", "stdDev_memory_rss_kb"))
    }
    pub fn square_root_of_population_standard_deviation_memory_rss_kb_of_processes(self) -> Self {
        self.square_root_of_population_standard_deviation_memory_rss_kb_of_processes_as("square_root_of_population_standard_deviation_memory_rss_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_memory_rss_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().stddev_pop("memory_rss_kb", "stdDevPop_memory_rss_kb"))
    }
    pub fn sample_variance_memory_rss_kb_of_processes(self) -> Self {
        self.sample_variance_memory_rss_kb_of_processes_as("sample_variance_memory_rss_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sample_variance_memory_rss_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().var_samp("memory_rss_kb", "varSamp_memory_rss_kb"))
    }
    pub fn sample_population_variance_memory_rss_kb_of_processes(self) -> Self {
        self.sample_population_variance_memory_rss_kb_of_processes_as("sample_population_variance_memory_rss_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sample_population_variance_memory_rss_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().var_pop("memory_rss_kb", "varPop_memory_rss_kb"))
    }
    pub fn sum_memory_vms_kb_of_processes(self) -> Self {
        self.sum_memory_vms_kb_of_processes_as("sum_memory_vms_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sum_memory_vms_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().sum("memory_vms_kb", "sum_memory_vms_kb"))
    }
    pub fn min_memory_vms_kb_of_processes(self) -> Self {
        self.min_memory_vms_kb_of_processes_as("min_memory_vms_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn min_memory_vms_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().min("memory_vms_kb", "min_memory_vms_kb"))
    }
    pub fn max_memory_vms_kb_of_processes(self) -> Self {
        self.max_memory_vms_kb_of_processes_as("max_memory_vms_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn max_memory_vms_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().max("memory_vms_kb", "max_memory_vms_kb"))
    }
    pub fn avg_memory_vms_kb_of_processes(self) -> Self {
        self.avg_memory_vms_kb_of_processes_as("avg_memory_vms_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn avg_memory_vms_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().avg("memory_vms_kb", "avg_memory_vms_kb"))
    }
    pub fn standard_deviation_memory_vms_kb_of_processes(self) -> Self {
        self.standard_deviation_memory_vms_kb_of_processes_as("standard_deviation_memory_vms_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn standard_deviation_memory_vms_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().stddev("memory_vms_kb", "stdDev_memory_vms_kb"))
    }
    pub fn square_root_of_population_standard_deviation_memory_vms_kb_of_processes(self) -> Self {
        self.square_root_of_population_standard_deviation_memory_vms_kb_of_processes_as("square_root_of_population_standard_deviation_memory_vms_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_memory_vms_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().stddev_pop("memory_vms_kb", "stdDevPop_memory_vms_kb"))
    }
    pub fn sample_variance_memory_vms_kb_of_processes(self) -> Self {
        self.sample_variance_memory_vms_kb_of_processes_as("sample_variance_memory_vms_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sample_variance_memory_vms_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().var_samp("memory_vms_kb", "varSamp_memory_vms_kb"))
    }
    pub fn sample_population_variance_memory_vms_kb_of_processes(self) -> Self {
        self.sample_population_variance_memory_vms_kb_of_processes_as("sample_population_variance_memory_vms_kb_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sample_population_variance_memory_vms_kb_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().var_pop("memory_vms_kb", "varPop_memory_vms_kb"))
    }
    pub fn sum_cpu_user_ticks_of_processes(self) -> Self {
        self.sum_cpu_user_ticks_of_processes_as("sum_cpu_user_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sum_cpu_user_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().sum("cpu_user_ticks", "sum_cpu_user_ticks"))
    }
    pub fn min_cpu_user_ticks_of_processes(self) -> Self {
        self.min_cpu_user_ticks_of_processes_as("min_cpu_user_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn min_cpu_user_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().min("cpu_user_ticks", "min_cpu_user_ticks"))
    }
    pub fn max_cpu_user_ticks_of_processes(self) -> Self {
        self.max_cpu_user_ticks_of_processes_as("max_cpu_user_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn max_cpu_user_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().max("cpu_user_ticks", "max_cpu_user_ticks"))
    }
    pub fn avg_cpu_user_ticks_of_processes(self) -> Self {
        self.avg_cpu_user_ticks_of_processes_as("avg_cpu_user_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn avg_cpu_user_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().avg("cpu_user_ticks", "avg_cpu_user_ticks"))
    }
    pub fn standard_deviation_cpu_user_ticks_of_processes(self) -> Self {
        self.standard_deviation_cpu_user_ticks_of_processes_as("standard_deviation_cpu_user_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn standard_deviation_cpu_user_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().stddev("cpu_user_ticks", "stdDev_cpu_user_ticks"))
    }
    pub fn square_root_of_population_standard_deviation_cpu_user_ticks_of_processes(self) -> Self {
        self.square_root_of_population_standard_deviation_cpu_user_ticks_of_processes_as("square_root_of_population_standard_deviation_cpu_user_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_cpu_user_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().stddev_pop("cpu_user_ticks", "stdDevPop_cpu_user_ticks"))
    }
    pub fn sample_variance_cpu_user_ticks_of_processes(self) -> Self {
        self.sample_variance_cpu_user_ticks_of_processes_as("sample_variance_cpu_user_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sample_variance_cpu_user_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().var_samp("cpu_user_ticks", "varSamp_cpu_user_ticks"))
    }
    pub fn sample_population_variance_cpu_user_ticks_of_processes(self) -> Self {
        self.sample_population_variance_cpu_user_ticks_of_processes_as("sample_population_variance_cpu_user_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sample_population_variance_cpu_user_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().var_pop("cpu_user_ticks", "varPop_cpu_user_ticks"))
    }
    pub fn sum_cpu_system_ticks_of_processes(self) -> Self {
        self.sum_cpu_system_ticks_of_processes_as("sum_cpu_system_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sum_cpu_system_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().sum("cpu_system_ticks", "sum_cpu_system_ticks"))
    }
    pub fn min_cpu_system_ticks_of_processes(self) -> Self {
        self.min_cpu_system_ticks_of_processes_as("min_cpu_system_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn min_cpu_system_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().min("cpu_system_ticks", "min_cpu_system_ticks"))
    }
    pub fn max_cpu_system_ticks_of_processes(self) -> Self {
        self.max_cpu_system_ticks_of_processes_as("max_cpu_system_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn max_cpu_system_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().max("cpu_system_ticks", "max_cpu_system_ticks"))
    }
    pub fn avg_cpu_system_ticks_of_processes(self) -> Self {
        self.avg_cpu_system_ticks_of_processes_as("avg_cpu_system_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn avg_cpu_system_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().avg("cpu_system_ticks", "avg_cpu_system_ticks"))
    }
    pub fn standard_deviation_cpu_system_ticks_of_processes(self) -> Self {
        self.standard_deviation_cpu_system_ticks_of_processes_as("standard_deviation_cpu_system_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn standard_deviation_cpu_system_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().stddev("cpu_system_ticks", "stdDev_cpu_system_ticks"))
    }
    pub fn square_root_of_population_standard_deviation_cpu_system_ticks_of_processes(self) -> Self {
        self.square_root_of_population_standard_deviation_cpu_system_ticks_of_processes_as("square_root_of_population_standard_deviation_cpu_system_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_cpu_system_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().stddev_pop("cpu_system_ticks", "stdDevPop_cpu_system_ticks"))
    }
    pub fn sample_variance_cpu_system_ticks_of_processes(self) -> Self {
        self.sample_variance_cpu_system_ticks_of_processes_as("sample_variance_cpu_system_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sample_variance_cpu_system_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().var_samp("cpu_system_ticks", "varSamp_cpu_system_ticks"))
    }
    pub fn sample_population_variance_cpu_system_ticks_of_processes(self) -> Self {
        self.sample_population_variance_cpu_system_ticks_of_processes_as("sample_population_variance_cpu_system_ticks_of_processes", crate::Q::processes().unlimited())
    }

    pub fn sample_population_variance_cpu_system_ticks_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().var_pop("cpu_system_ticks", "varPop_cpu_system_ticks"))
    }
    pub fn min_create_time_of_processes(self) -> Self {
        self.min_create_time_of_processes_as("min_create_time_of_processes", crate::Q::processes().unlimited())
    }

    pub fn min_create_time_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_processes(self) -> Self {
        self.max_create_time_of_processes_as("max_create_time_of_processes", crate::Q::processes().unlimited())
    }

    pub fn max_create_time_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_processes(self) -> Self {
        self.min_update_time_of_processes_as("min_update_time_of_processes", crate::Q::processes().unlimited())
    }

    pub fn min_update_time_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_processes(self) -> Self {
        self.max_update_time_of_processes_as("max_update_time_of_processes", crate::Q::processes().unlimited())
    }

    pub fn max_update_time_of_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_processes_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }
}

impl<R> Default for SystemInfoRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< SystemInfoRequest<R> > for SelectQuery {
    fn from(request: SystemInfoRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< SystemInfoRequest<R> > for QuerySelection {
    fn from(request: SystemInfoRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::SystemInfo> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::SystemInfoRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<SystemInfoRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::SystemInfo
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::SystemInfo::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> SystemInfoRequest<R> {
        self.inner.query.trace_chain.push(teaql_core::TraceNode {
            entity_type: self.inner.query.entity.clone(),
            entity_id: None,
            comment: self.purpose,
        });
        self.inner
    }

    pub async fn execute_for_page<'a, C>(
        self,
        ctx: &'a C,
        offset: u64,
        limit: u64,
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::SystemInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
