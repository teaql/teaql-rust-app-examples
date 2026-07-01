use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::Thread {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::Thread {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/thread
#[derive(Debug)]
pub struct ThreadRequest<R = crate::Thread> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for ThreadRequest<R> {
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

impl<R> ThreadRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("Thread")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> ThreadRequest<T> {
        ThreadRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::ThreadRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .thread_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::ThreadRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .thread_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::ThreadRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::ThreadRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::ThreadRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::ThreadRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .thread_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for Thread is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::ThreadRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .thread_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::ThreadRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .thread_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::ThreadRepository<'a>>>
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
            "tid" => Some("tid"),
            "name" => Some("name"),
            "state" => Some("state"),
            "process_pid" => Some("process_pid"),
            "cpu_user_ticks" => Some("cpu_user_ticks"),
            "cpu_system_ticks" => Some("cpu_system_ticks"),
            "create_time" => Some("create_time"),
            "update_time" => Some("update_time"),
            "version" => Some("version"),
            "process" | "process_id" => Some("process_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "process" => {
                self.with_process_matching(
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
        self.query = self.query.project("tid");
        self.query = self.query.project("name");
        self.query = self.query.project("state");
        self.query = self.query.project("process_pid");
        self.query = self.query.project("cpu_user_ticks");
        self.query = self.query.project("cpu_system_ticks");
        self.query = self.query.project("create_time");
        self.query = self.query.project("update_time");
        self.query = self.query.project("version");
        self.query = self.query.project("process_id");
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
        request = request.select_process();
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


    pub fn select_tid(mut self) -> Self {
        self.query = self.query.project("tid");
        self
    }

    pub fn project_tid(self) -> Self {
        self.select_tid()
    }

    pub fn select_tid_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_tid_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_tid_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("tid", raw_sql_segment));
        self
    }

    pub fn select_tid_with_function(self, function: AggregateFunction) -> Self {
        self.select_tid_as_with_function("tid", function)
    }

    pub fn select_tid_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("tid", alias, function)
    }

    pub fn group_by_tid(self) -> Self {
        self.group_by("tid")
    }

    pub fn group_by_tid_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("tid");
        request.query = request
            .query
            .project_expr(alias, Expr::column("tid"));
        request
    }

    pub fn group_by_tid_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("tid")
            .aggregate_with_function("tid", alias, function)
    }

    pub fn count_tid(self) -> Self {
        self.count_tid_as("tid_count")
    }

    pub fn count_tid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("tid", alias)
    }

    pub fn sum_tid(self) -> Self {
        self.sum_tid_as("sum_tid")
    }

    pub fn sum_tid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("tid", alias)
    }

    pub fn avg_tid(self) -> Self {
        self.avg_tid_as("avg_tid")
    }

    pub fn avg_tid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("tid", alias)
    }

    pub fn min_tid(self) -> Self {
        self.min_tid_as("min_tid")
    }

    pub fn min_tid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("tid", alias)
    }

    pub fn max_tid(self) -> Self {
        self.max_tid_as("max_tid")
    }

    pub fn max_tid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("tid", alias)
    }

    pub fn standard_deviation_tid(self) -> Self {
        self.standard_deviation_tid_as("stdDev_tid")
    }

    pub fn standard_deviation_tid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("tid", alias)
    }

    pub fn square_root_of_population_standard_deviation_tid(self) -> Self {
        self.square_root_of_population_standard_deviation_tid_as("stdDevPop_tid")
    }

    pub fn square_root_of_population_standard_deviation_tid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("tid", alias)
    }

    pub fn sample_variance_tid(self) -> Self {
        self.sample_variance_tid_as("varSamp_tid")
    }

    pub fn sample_variance_tid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("tid", alias)
    }

    pub fn sample_population_variance_tid(self) -> Self {
        self.sample_population_variance_tid_as("varPop_tid")
    }

    pub fn sample_population_variance_tid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("tid", alias)
    }

    pub fn unselect_tid(mut self) -> Self {
        self.query.projection.retain(|field| field != "tid");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "tid");
        self
    }


    pub fn with_tid(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "tid",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_tid_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "tid",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_tid_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("tid", value));
        self
    }



    pub fn with_tid_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("tid", value));
        self
    }

    pub fn with_tid_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("tid", value));
        self
    }

    pub fn with_tid_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("tid", value));
        self
    }

    pub fn with_tid_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("tid", value));
        self
    }

    pub fn with_tid_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("tid", value));
        self
    }

    pub fn with_tid_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("tid", lower, upper));
        self
    }

    pub fn with_tid_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "tid",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_tid_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "tid",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_tid_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "tid",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_tid_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("tid", value));
        self
    }

    pub fn with_tid_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("tid", value));
        self
    }

    pub fn with_tid_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("tid"));
        self
    }



    pub fn with_tid_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("tid"));
        self
    }


    pub fn order_by_tid_asc(mut self) -> Self {
        self.query = self.query.order_asc("tid");
        self
    }

    pub fn order_by_tid_desc(mut self) -> Self {
        self.query = self.query.order_desc("tid");
        self
    }

    pub fn order_by_tid_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("tid");
        self
    }

    pub fn order_by_tid_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("tid");
        self
    }


    pub fn select_name(mut self) -> Self {
        self.query = self.query.project("name");
        self
    }

    pub fn project_name(self) -> Self {
        self.select_name()
    }

    pub fn select_name_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_name_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_name_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("name", raw_sql_segment));
        self
    }

    pub fn group_by_name(self) -> Self {
        self.group_by("name")
    }

    pub fn group_by_name_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("name");
        request.query = request
            .query
            .project_expr(alias, Expr::column("name"));
        request
    }

    pub fn group_by_name_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("name")
            .aggregate_with_function("name", alias, function)
    }

    pub fn count_name(self) -> Self {
        self.count_name_as("name_count")
    }

    pub fn count_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("name", alias)
    }

    pub fn sum_name(self) -> Self {
        self.sum_name_as("sum_name")
    }

    pub fn sum_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("name", alias)
    }

    pub fn avg_name(self) -> Self {
        self.avg_name_as("avg_name")
    }

    pub fn avg_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("name", alias)
    }

    pub fn min_name(self) -> Self {
        self.min_name_as("min_name")
    }

    pub fn min_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("name", alias)
    }

    pub fn max_name(self) -> Self {
        self.max_name_as("max_name")
    }

    pub fn max_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("name", alias)
    }

    pub fn unselect_name(mut self) -> Self {
        self.query.projection.retain(|field| field != "name");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "name");
        self
    }


    pub fn with_name(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "name",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_name_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "name",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_name_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("name", value));
        self
    }



    pub fn with_name_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("name", value));
        self
    }

    pub fn with_name_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("name", value));
        self
    }

    pub fn with_name_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("name", value));
        self
    }

    pub fn with_name_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("name", value));
        self
    }

    pub fn with_name_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("name", value));
        self
    }

    pub fn with_name_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("name", lower, upper));
        self
    }

    pub fn with_name_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "name",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_name_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_name_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_name_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("name", value));
        self
    }

    pub fn with_name_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("name", value));
        self
    }

    pub fn with_name_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("name", value));
        self
    }

    pub fn with_name_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("name", value));
        self
    }

    pub fn with_name_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("name", value));
        self
    }

    pub fn with_name_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("name", value));
        self
    }

    pub fn with_name_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("name", value));
        self
    }
    pub fn with_name_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("name", value));
        self
    }

    pub fn with_name_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("name", value));
        self
    }

    pub fn with_name_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("name"));
        self
    }



    pub fn with_name_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("name"));
        self
    }


    pub fn order_by_name_asc(mut self) -> Self {
        self.query = self.query.order_asc("name");
        self
    }

    pub fn order_by_name_desc(mut self) -> Self {
        self.query = self.query.order_desc("name");
        self
    }

    pub fn order_by_name_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("name");
        self
    }

    pub fn order_by_name_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("name");
        self
    }


    pub fn select_state(mut self) -> Self {
        self.query = self.query.project("state");
        self
    }

    pub fn project_state(self) -> Self {
        self.select_state()
    }

    pub fn select_state_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_state_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_state_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("state", raw_sql_segment));
        self
    }

    pub fn group_by_state(self) -> Self {
        self.group_by("state")
    }

    pub fn group_by_state_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("state");
        request.query = request
            .query
            .project_expr(alias, Expr::column("state"));
        request
    }

    pub fn group_by_state_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("state")
            .aggregate_with_function("state", alias, function)
    }

    pub fn count_state(self) -> Self {
        self.count_state_as("state_count")
    }

    pub fn count_state_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("state", alias)
    }

    pub fn sum_state(self) -> Self {
        self.sum_state_as("sum_state")
    }

    pub fn sum_state_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("state", alias)
    }

    pub fn avg_state(self) -> Self {
        self.avg_state_as("avg_state")
    }

    pub fn avg_state_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("state", alias)
    }

    pub fn min_state(self) -> Self {
        self.min_state_as("min_state")
    }

    pub fn min_state_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("state", alias)
    }

    pub fn max_state(self) -> Self {
        self.max_state_as("max_state")
    }

    pub fn max_state_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("state", alias)
    }

    pub fn unselect_state(mut self) -> Self {
        self.query.projection.retain(|field| field != "state");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "state");
        self
    }


    pub fn with_state(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "state",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_state_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "state",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_state_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("state", value));
        self
    }



    pub fn with_state_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("state", value));
        self
    }

    pub fn with_state_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("state", value));
        self
    }

    pub fn with_state_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("state", value));
        self
    }

    pub fn with_state_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("state", value));
        self
    }

    pub fn with_state_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("state", value));
        self
    }

    pub fn with_state_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("state", lower, upper));
        self
    }

    pub fn with_state_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "state",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_state_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "state",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_state_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "state",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_state_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("state", value));
        self
    }

    pub fn with_state_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("state", value));
        self
    }

    pub fn with_state_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("state", value));
        self
    }

    pub fn with_state_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("state", value));
        self
    }

    pub fn with_state_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("state", value));
        self
    }

    pub fn with_state_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("state", value));
        self
    }

    pub fn with_state_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("state", value));
        self
    }
    pub fn with_state_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("state", value));
        self
    }

    pub fn with_state_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("state", value));
        self
    }

    pub fn with_state_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("state"));
        self
    }



    pub fn with_state_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("state"));
        self
    }


    pub fn order_by_state_asc(mut self) -> Self {
        self.query = self.query.order_asc("state");
        self
    }

    pub fn order_by_state_desc(mut self) -> Self {
        self.query = self.query.order_desc("state");
        self
    }

    pub fn order_by_state_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("state");
        self
    }

    pub fn order_by_state_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("state");
        self
    }


    pub fn select_process_pid(mut self) -> Self {
        self.query = self.query.project("process_pid");
        self
    }

    pub fn project_process_pid(self) -> Self {
        self.select_process_pid()
    }

    pub fn select_process_pid_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_process_pid_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_process_pid_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("process_pid", raw_sql_segment));
        self
    }

    pub fn select_process_pid_with_function(self, function: AggregateFunction) -> Self {
        self.select_process_pid_as_with_function("process_pid", function)
    }

    pub fn select_process_pid_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("process_pid", alias, function)
    }

    pub fn group_by_process_pid(self) -> Self {
        self.group_by("process_pid")
    }

    pub fn group_by_process_pid_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("process_pid");
        request.query = request
            .query
            .project_expr(alias, Expr::column("process_pid"));
        request
    }

    pub fn group_by_process_pid_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("process_pid")
            .aggregate_with_function("process_pid", alias, function)
    }

    pub fn count_process_pid(self) -> Self {
        self.count_process_pid_as("process_pid_count")
    }

    pub fn count_process_pid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("process_pid", alias)
    }

    pub fn sum_process_pid(self) -> Self {
        self.sum_process_pid_as("sum_process_pid")
    }

    pub fn sum_process_pid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("process_pid", alias)
    }

    pub fn avg_process_pid(self) -> Self {
        self.avg_process_pid_as("avg_process_pid")
    }

    pub fn avg_process_pid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("process_pid", alias)
    }

    pub fn min_process_pid(self) -> Self {
        self.min_process_pid_as("min_process_pid")
    }

    pub fn min_process_pid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("process_pid", alias)
    }

    pub fn max_process_pid(self) -> Self {
        self.max_process_pid_as("max_process_pid")
    }

    pub fn max_process_pid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("process_pid", alias)
    }

    pub fn standard_deviation_process_pid(self) -> Self {
        self.standard_deviation_process_pid_as("stdDev_process_pid")
    }

    pub fn standard_deviation_process_pid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("process_pid", alias)
    }

    pub fn square_root_of_population_standard_deviation_process_pid(self) -> Self {
        self.square_root_of_population_standard_deviation_process_pid_as("stdDevPop_process_pid")
    }

    pub fn square_root_of_population_standard_deviation_process_pid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("process_pid", alias)
    }

    pub fn sample_variance_process_pid(self) -> Self {
        self.sample_variance_process_pid_as("varSamp_process_pid")
    }

    pub fn sample_variance_process_pid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("process_pid", alias)
    }

    pub fn sample_population_variance_process_pid(self) -> Self {
        self.sample_population_variance_process_pid_as("varPop_process_pid")
    }

    pub fn sample_population_variance_process_pid_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("process_pid", alias)
    }

    pub fn unselect_process_pid(mut self) -> Self {
        self.query.projection.retain(|field| field != "process_pid");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "process_pid");
        self
    }


    pub fn with_process_pid(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "process_pid",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_process_pid_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "process_pid",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_process_pid_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("process_pid", value));
        self
    }



    pub fn with_process_pid_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("process_pid", value));
        self
    }

    pub fn with_process_pid_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("process_pid", value));
        self
    }

    pub fn with_process_pid_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("process_pid", value));
        self
    }

    pub fn with_process_pid_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("process_pid", value));
        self
    }

    pub fn with_process_pid_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("process_pid", value));
        self
    }

    pub fn with_process_pid_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("process_pid", lower, upper));
        self
    }

    pub fn with_process_pid_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "process_pid",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_process_pid_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "process_pid",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_process_pid_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "process_pid",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_process_pid_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("process_pid", value));
        self
    }

    pub fn with_process_pid_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("process_pid", value));
        self
    }

    pub fn with_process_pid_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("process_pid"));
        self
    }



    pub fn with_process_pid_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("process_pid"));
        self
    }


    pub fn order_by_process_pid_asc(mut self) -> Self {
        self.query = self.query.order_asc("process_pid");
        self
    }

    pub fn order_by_process_pid_desc(mut self) -> Self {
        self.query = self.query.order_desc("process_pid");
        self
    }

    pub fn order_by_process_pid_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("process_pid");
        self
    }

    pub fn order_by_process_pid_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("process_pid");
        self
    }


    pub fn select_cpu_user_ticks(mut self) -> Self {
        self.query = self.query.project("cpu_user_ticks");
        self
    }

    pub fn project_cpu_user_ticks(self) -> Self {
        self.select_cpu_user_ticks()
    }

    pub fn select_cpu_user_ticks_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_cpu_user_ticks_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_cpu_user_ticks_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("cpu_user_ticks", raw_sql_segment));
        self
    }

    pub fn select_cpu_user_ticks_with_function(self, function: AggregateFunction) -> Self {
        self.select_cpu_user_ticks_as_with_function("cpu_user_ticks", function)
    }

    pub fn select_cpu_user_ticks_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("cpu_user_ticks", alias, function)
    }

    pub fn group_by_cpu_user_ticks(self) -> Self {
        self.group_by("cpu_user_ticks")
    }

    pub fn group_by_cpu_user_ticks_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("cpu_user_ticks");
        request.query = request
            .query
            .project_expr(alias, Expr::column("cpu_user_ticks"));
        request
    }

    pub fn group_by_cpu_user_ticks_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("cpu_user_ticks")
            .aggregate_with_function("cpu_user_ticks", alias, function)
    }

    pub fn count_cpu_user_ticks(self) -> Self {
        self.count_cpu_user_ticks_as("cpu_user_ticks_count")
    }

    pub fn count_cpu_user_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("cpu_user_ticks", alias)
    }

    pub fn sum_cpu_user_ticks(self) -> Self {
        self.sum_cpu_user_ticks_as("sum_cpu_user_ticks")
    }

    pub fn sum_cpu_user_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("cpu_user_ticks", alias)
    }

    pub fn avg_cpu_user_ticks(self) -> Self {
        self.avg_cpu_user_ticks_as("avg_cpu_user_ticks")
    }

    pub fn avg_cpu_user_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("cpu_user_ticks", alias)
    }

    pub fn min_cpu_user_ticks(self) -> Self {
        self.min_cpu_user_ticks_as("min_cpu_user_ticks")
    }

    pub fn min_cpu_user_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("cpu_user_ticks", alias)
    }

    pub fn max_cpu_user_ticks(self) -> Self {
        self.max_cpu_user_ticks_as("max_cpu_user_ticks")
    }

    pub fn max_cpu_user_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("cpu_user_ticks", alias)
    }

    pub fn standard_deviation_cpu_user_ticks(self) -> Self {
        self.standard_deviation_cpu_user_ticks_as("stdDev_cpu_user_ticks")
    }

    pub fn standard_deviation_cpu_user_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("cpu_user_ticks", alias)
    }

    pub fn square_root_of_population_standard_deviation_cpu_user_ticks(self) -> Self {
        self.square_root_of_population_standard_deviation_cpu_user_ticks_as("stdDevPop_cpu_user_ticks")
    }

    pub fn square_root_of_population_standard_deviation_cpu_user_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("cpu_user_ticks", alias)
    }

    pub fn sample_variance_cpu_user_ticks(self) -> Self {
        self.sample_variance_cpu_user_ticks_as("varSamp_cpu_user_ticks")
    }

    pub fn sample_variance_cpu_user_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("cpu_user_ticks", alias)
    }

    pub fn sample_population_variance_cpu_user_ticks(self) -> Self {
        self.sample_population_variance_cpu_user_ticks_as("varPop_cpu_user_ticks")
    }

    pub fn sample_population_variance_cpu_user_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("cpu_user_ticks", alias)
    }

    pub fn unselect_cpu_user_ticks(mut self) -> Self {
        self.query.projection.retain(|field| field != "cpu_user_ticks");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "cpu_user_ticks");
        self
    }


    pub fn with_cpu_user_ticks(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "cpu_user_ticks",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_cpu_user_ticks_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "cpu_user_ticks",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_cpu_user_ticks_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("cpu_user_ticks", value));
        self
    }



    pub fn with_cpu_user_ticks_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("cpu_user_ticks", value));
        self
    }

    pub fn with_cpu_user_ticks_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("cpu_user_ticks", value));
        self
    }

    pub fn with_cpu_user_ticks_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("cpu_user_ticks", value));
        self
    }

    pub fn with_cpu_user_ticks_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("cpu_user_ticks", value));
        self
    }

    pub fn with_cpu_user_ticks_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("cpu_user_ticks", value));
        self
    }

    pub fn with_cpu_user_ticks_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("cpu_user_ticks", lower, upper));
        self
    }

    pub fn with_cpu_user_ticks_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "cpu_user_ticks",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_cpu_user_ticks_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "cpu_user_ticks",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_cpu_user_ticks_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "cpu_user_ticks",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_cpu_user_ticks_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("cpu_user_ticks", value));
        self
    }

    pub fn with_cpu_user_ticks_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("cpu_user_ticks", value));
        self
    }

    pub fn with_cpu_user_ticks_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("cpu_user_ticks"));
        self
    }



    pub fn with_cpu_user_ticks_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("cpu_user_ticks"));
        self
    }


    pub fn order_by_cpu_user_ticks_asc(mut self) -> Self {
        self.query = self.query.order_asc("cpu_user_ticks");
        self
    }

    pub fn order_by_cpu_user_ticks_desc(mut self) -> Self {
        self.query = self.query.order_desc("cpu_user_ticks");
        self
    }

    pub fn order_by_cpu_user_ticks_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("cpu_user_ticks");
        self
    }

    pub fn order_by_cpu_user_ticks_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("cpu_user_ticks");
        self
    }


    pub fn select_cpu_system_ticks(mut self) -> Self {
        self.query = self.query.project("cpu_system_ticks");
        self
    }

    pub fn project_cpu_system_ticks(self) -> Self {
        self.select_cpu_system_ticks()
    }

    pub fn select_cpu_system_ticks_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_cpu_system_ticks_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_cpu_system_ticks_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("cpu_system_ticks", raw_sql_segment));
        self
    }

    pub fn select_cpu_system_ticks_with_function(self, function: AggregateFunction) -> Self {
        self.select_cpu_system_ticks_as_with_function("cpu_system_ticks", function)
    }

    pub fn select_cpu_system_ticks_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("cpu_system_ticks", alias, function)
    }

    pub fn group_by_cpu_system_ticks(self) -> Self {
        self.group_by("cpu_system_ticks")
    }

    pub fn group_by_cpu_system_ticks_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("cpu_system_ticks");
        request.query = request
            .query
            .project_expr(alias, Expr::column("cpu_system_ticks"));
        request
    }

    pub fn group_by_cpu_system_ticks_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("cpu_system_ticks")
            .aggregate_with_function("cpu_system_ticks", alias, function)
    }

    pub fn count_cpu_system_ticks(self) -> Self {
        self.count_cpu_system_ticks_as("cpu_system_ticks_count")
    }

    pub fn count_cpu_system_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("cpu_system_ticks", alias)
    }

    pub fn sum_cpu_system_ticks(self) -> Self {
        self.sum_cpu_system_ticks_as("sum_cpu_system_ticks")
    }

    pub fn sum_cpu_system_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("cpu_system_ticks", alias)
    }

    pub fn avg_cpu_system_ticks(self) -> Self {
        self.avg_cpu_system_ticks_as("avg_cpu_system_ticks")
    }

    pub fn avg_cpu_system_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("cpu_system_ticks", alias)
    }

    pub fn min_cpu_system_ticks(self) -> Self {
        self.min_cpu_system_ticks_as("min_cpu_system_ticks")
    }

    pub fn min_cpu_system_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("cpu_system_ticks", alias)
    }

    pub fn max_cpu_system_ticks(self) -> Self {
        self.max_cpu_system_ticks_as("max_cpu_system_ticks")
    }

    pub fn max_cpu_system_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("cpu_system_ticks", alias)
    }

    pub fn standard_deviation_cpu_system_ticks(self) -> Self {
        self.standard_deviation_cpu_system_ticks_as("stdDev_cpu_system_ticks")
    }

    pub fn standard_deviation_cpu_system_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("cpu_system_ticks", alias)
    }

    pub fn square_root_of_population_standard_deviation_cpu_system_ticks(self) -> Self {
        self.square_root_of_population_standard_deviation_cpu_system_ticks_as("stdDevPop_cpu_system_ticks")
    }

    pub fn square_root_of_population_standard_deviation_cpu_system_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("cpu_system_ticks", alias)
    }

    pub fn sample_variance_cpu_system_ticks(self) -> Self {
        self.sample_variance_cpu_system_ticks_as("varSamp_cpu_system_ticks")
    }

    pub fn sample_variance_cpu_system_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("cpu_system_ticks", alias)
    }

    pub fn sample_population_variance_cpu_system_ticks(self) -> Self {
        self.sample_population_variance_cpu_system_ticks_as("varPop_cpu_system_ticks")
    }

    pub fn sample_population_variance_cpu_system_ticks_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("cpu_system_ticks", alias)
    }

    pub fn unselect_cpu_system_ticks(mut self) -> Self {
        self.query.projection.retain(|field| field != "cpu_system_ticks");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "cpu_system_ticks");
        self
    }


    pub fn with_cpu_system_ticks(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "cpu_system_ticks",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_cpu_system_ticks_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "cpu_system_ticks",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_cpu_system_ticks_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("cpu_system_ticks", value));
        self
    }



    pub fn with_cpu_system_ticks_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("cpu_system_ticks", value));
        self
    }

    pub fn with_cpu_system_ticks_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("cpu_system_ticks", value));
        self
    }

    pub fn with_cpu_system_ticks_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("cpu_system_ticks", value));
        self
    }

    pub fn with_cpu_system_ticks_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("cpu_system_ticks", value));
        self
    }

    pub fn with_cpu_system_ticks_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("cpu_system_ticks", value));
        self
    }

    pub fn with_cpu_system_ticks_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("cpu_system_ticks", lower, upper));
        self
    }

    pub fn with_cpu_system_ticks_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "cpu_system_ticks",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_cpu_system_ticks_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "cpu_system_ticks",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_cpu_system_ticks_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "cpu_system_ticks",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_cpu_system_ticks_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("cpu_system_ticks", value));
        self
    }

    pub fn with_cpu_system_ticks_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("cpu_system_ticks", value));
        self
    }

    pub fn with_cpu_system_ticks_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("cpu_system_ticks"));
        self
    }



    pub fn with_cpu_system_ticks_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("cpu_system_ticks"));
        self
    }


    pub fn order_by_cpu_system_ticks_asc(mut self) -> Self {
        self.query = self.query.order_asc("cpu_system_ticks");
        self
    }

    pub fn order_by_cpu_system_ticks_desc(mut self) -> Self {
        self.query = self.query.order_desc("cpu_system_ticks");
        self
    }

    pub fn order_by_cpu_system_ticks_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("cpu_system_ticks");
        self
    }

    pub fn order_by_cpu_system_ticks_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("cpu_system_ticks");
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
    pub fn filter_by_process(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("process_id", value.entity_id_value()));
        self
    }

    pub fn with_process_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "process_id",
            <crate::Process as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("process", selection));
        self
    }


    pub fn without_process_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "process_id",
            <crate::Process as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("process", selection));
        self
    }


    pub fn have_process(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("process_id"));
        self
    }

    pub fn have_no_process(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("process_id"));
        self
    }


    pub fn group_by_process(self) -> Self {
        self.group_by("process_id")
    }

    pub fn group_by_process_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("process_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("process_id"));
        request
    }

    pub fn group_by_process_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("process_id")
            .aggregate_with_function("process_id", alias, function)
    }

    pub fn group_by_process_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("process_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "process",
            "process_id",
            request,
        ));
        self
    }

    pub fn group_by_process_with_details(self) -> Self {
        self.group_by_process_with_details_from(crate::Q::processes().unlimited())
    }

    pub fn group_by_process_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_process_with(request)
    }


    pub fn roll_up_to_process(self) -> Self {
        self.roll_up_to_process_with(crate::Q::processes().unlimited())
    }

    pub fn roll_up_to_process_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_process_matching(selection.clone())
            .group_by_process_with(selection)
    }

    pub fn count_process(self) -> Self {
        self.count_process_as("process_count")
    }

    pub fn count_process_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("process_id", alias)
    }

    pub fn unselect_process(mut self) -> Self {
        self.query.projection.retain(|field| field != "process_id");
        self.query.relations.retain(|relation| relation.name != "process");
        self
    }
    pub fn select_process(mut self) -> Self {
        self.query = self.query.relation("process");
        self
    }

    pub fn select_process_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("process", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("process", selection));
        self
}

    pub fn facet_by_process_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_process_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_process_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "process",
            request,
            include_all_facets,
        ));
        self
    }
}

impl<R> Default for ThreadRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< ThreadRequest<R> > for SelectQuery {
    fn from(request: ThreadRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< ThreadRequest<R> > for QuerySelection {
    fn from(request: ThreadRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::Thread> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::ThreadRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<ThreadRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::Thread
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::Thread::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> ThreadRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::ThreadRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::ThreadRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::ThreadRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::ThreadRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::ThreadRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::ThreadRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::ThreadRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::ThreadRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::ThreadRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
