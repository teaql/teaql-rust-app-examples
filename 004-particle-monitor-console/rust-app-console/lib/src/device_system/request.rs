use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::DeviceSystem {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::DeviceSystem {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/device_system
#[derive(Debug)]
pub struct DeviceSystemRequest<R = crate::DeviceSystem> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for DeviceSystemRequest<R> {
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

impl<R> DeviceSystemRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("DeviceSystem")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> DeviceSystemRequest<T> {
        DeviceSystemRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .device_system_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let query_options = self.query_options.clone();
        let relation_aggregates = runtime_relation_aggregates(&query_options);
        let query = authorize_query(apply_runtime_metadata(
            self.query,
            &query_options,
            &self.child_enhancements,
        )).map_err(DataServiceError::Runtime)?;
        let mut rows = repository.fetch_enhanced_entities_with_relation_aggregates::<R>(
            &query,
            &relation_aggregates,
        ).await?;
        let facets = execute_facets(ctx, query.as_query(), &query_options)
            .await
            .map_err(DataServiceError::Runtime)?;
        attach_facets(&mut rows, facets);
        Ok(rows)
    }

    pub(crate) async fn _execute_for_stream<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .device_system_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let query_options = self.query_options.clone();
        let query = authorize_query(apply_runtime_metadata(
            self.query,
            &query_options,
            &self.child_enhancements,
        )).map_err(DataServiceError::Runtime)?;
        let chunks = repository.fetch_stream(&query)
            .await?;
        Ok(chunks)
    }

    pub(crate) async fn _execute_for_first<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Option<R>, TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .device_system_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query;
        query.projection.clear();
        query.expr_projection.clear();
        query.order_by.clear();
        query.slice = None;
        query.relations.clear();
        query = query.count(COUNT_ALIAS);
        let query = authorize_query(query).map_err(DataServiceError::Runtime)?;
        let rows = repository.fetch_all(&query).await?;
        rows.first()
            .and_then(|row| row.get(COUNT_ALIAS))
            .and_then(teaql_core::Value::try_u64)
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for DeviceSystem is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .device_system_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let query = authorize_query(query).map_err(DataServiceError::Runtime)?;
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .device_system_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let query_options = self.query_options.clone();
        let outer_query = self.query.clone();
        let relation_aggregates = runtime_relation_aggregates(&query_options);
        let query = authorize_query(apply_runtime_metadata(
            self.query,
            &query_options,
            &self.child_enhancements,
        )).map_err(DataServiceError::Runtime)?;
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
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
            "name" => Some("name"),
            "serial_number" => Some("serial_number"),
            "create_time" => Some("create_time"),
            "update_time" => Some("update_time"),
            "version" => Some("version"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "system_status_list" => {
                self.with_system_status_list_matching(
                    crate::Q::system_statuses_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "device_setting_list" => {
                self.with_device_setting_list_matching(
                    crate::Q::device_settings_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "sample_record_list" => {
                self.with_sample_record_list_matching(
                    crate::Q::sample_records_minimal()
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
        self.query = self.query.project("name");
        self.query = self.query.project("serial_number");
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
        request = request.select_system_status_list();
        request = request.select_device_setting_list();
        request = request.select_sample_record_list();
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


    pub fn select_serial_number(mut self) -> Self {
        self.query = self.query.project("serial_number");
        self
    }

    pub fn project_serial_number(self) -> Self {
        self.select_serial_number()
    }

    pub fn select_serial_number_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_serial_number_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_serial_number_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("serial_number", raw_sql_segment));
        self
    }

    pub fn group_by_serial_number(self) -> Self {
        self.group_by("serial_number")
    }

    pub fn group_by_serial_number_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("serial_number");
        request.query = request
            .query
            .project_expr(alias, Expr::column("serial_number"));
        request
    }

    pub fn group_by_serial_number_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("serial_number")
            .aggregate_with_function("serial_number", alias, function)
    }

    pub fn count_serial_number(self) -> Self {
        self.count_serial_number_as("serial_number_count")
    }

    pub fn count_serial_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("serial_number", alias)
    }

    pub fn sum_serial_number(self) -> Self {
        self.sum_serial_number_as("sum_serial_number")
    }

    pub fn sum_serial_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("serial_number", alias)
    }

    pub fn avg_serial_number(self) -> Self {
        self.avg_serial_number_as("avg_serial_number")
    }

    pub fn avg_serial_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("serial_number", alias)
    }

    pub fn min_serial_number(self) -> Self {
        self.min_serial_number_as("min_serial_number")
    }

    pub fn min_serial_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("serial_number", alias)
    }

    pub fn max_serial_number(self) -> Self {
        self.max_serial_number_as("max_serial_number")
    }

    pub fn max_serial_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("serial_number", alias)
    }

    pub fn unselect_serial_number(mut self) -> Self {
        self.query.projection.retain(|field| field != "serial_number");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "serial_number");
        self
    }


    pub fn with_serial_number(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "serial_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_serial_number_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "serial_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_serial_number_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("serial_number", value));
        self
    }



    pub fn with_serial_number_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("serial_number", value));
        self
    }

    pub fn with_serial_number_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("serial_number", value));
        self
    }

    pub fn with_serial_number_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("serial_number", value));
        self
    }

    pub fn with_serial_number_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("serial_number", value));
        self
    }

    pub fn with_serial_number_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("serial_number", value));
        self
    }

    pub fn with_serial_number_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("serial_number", lower, upper));
        self
    }

    pub fn with_serial_number_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "serial_number",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_serial_number_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "serial_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_serial_number_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "serial_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_serial_number_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("serial_number", value));
        self
    }

    pub fn with_serial_number_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("serial_number", value));
        self
    }

    pub fn with_serial_number_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("serial_number", value));
        self
    }

    pub fn with_serial_number_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("serial_number", value));
        self
    }

    pub fn with_serial_number_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("serial_number", value));
        self
    }

    pub fn with_serial_number_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("serial_number", value));
        self
    }

    pub fn with_serial_number_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("serial_number", value));
        self
    }
    pub fn with_serial_number_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("serial_number", value));
        self
    }

    pub fn with_serial_number_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("serial_number", value));
        self
    }

    pub fn with_serial_number_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("serial_number"));
        self
    }



    pub fn with_serial_number_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("serial_number"));
        self
    }


    pub fn order_by_serial_number_asc(mut self) -> Self {
        self.query = self.query.order_asc("serial_number");
        self
    }

    pub fn order_by_serial_number_desc(mut self) -> Self {
        self.query = self.query.order_desc("serial_number");
        self
    }

    pub fn order_by_serial_number_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("serial_number");
        self
    }

    pub fn order_by_serial_number_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("serial_number");
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
    pub fn name_is_pms_gt660x_terminal(self) -> Self {
        self.with_name_is("PMS-GT660X Terminal")
    }

    pub fn with_name_is_pms_gt660x_terminal(self) -> Self {
        self.with_name_is("PMS-GT660X Terminal")
    }



    pub fn with_name_is_not_pms_gt660x_terminal(self) -> Self {
        self.with_name_is_not("PMS-GT660X Terminal")
    }



    pub fn serial_number_is_pms_2026_0701(self) -> Self {
        self.with_serial_number_is("PMS-2026-0701")
    }

    pub fn with_serial_number_is_pms_2026_0701(self) -> Self {
        self.with_serial_number_is("PMS-2026-0701")
    }



    pub fn with_serial_number_is_not_pms_2026_0701(self) -> Self {
        self.with_serial_number_is_not("PMS-2026-0701")
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




    pub fn have_system_statuses(self) -> Self {
        self.with_system_status_list_matching(SelectQuery::new("SystemStatus"))
    }

    pub fn have_no_system_statuses(self) -> Self {
        self.without_system_status_list_matching(SelectQuery::new("SystemStatus"))
    }

    pub fn with_system_status_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::SystemStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "device_system_id",
        ));
        self.relation_filters.push(RelationFilter::new("system_status_list", selection));
        self
    }

    pub fn without_system_status_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::SystemStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "device_system_id",
        ));
        self.relation_filters.push(RelationFilter::new("system_status_list", selection));
        self
    }

    pub fn select_system_status_list(mut self) -> Self {
        self.query = self.query.relation("system_status_list");
        self
    }

    pub fn select_system_status_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("system_status_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("system_status_list", selection));
        self
}

    pub fn have_device_settings(self) -> Self {
        self.with_device_setting_list_matching(SelectQuery::new("DeviceSetting"))
    }

    pub fn have_no_device_settings(self) -> Self {
        self.without_device_setting_list_matching(SelectQuery::new("DeviceSetting"))
    }

    pub fn with_device_setting_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::DeviceSetting as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "device_system_id",
        ));
        self.relation_filters.push(RelationFilter::new("device_setting_list", selection));
        self
    }

    pub fn without_device_setting_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::DeviceSetting as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "device_system_id",
        ));
        self.relation_filters.push(RelationFilter::new("device_setting_list", selection));
        self
    }

    pub fn select_device_setting_list(mut self) -> Self {
        self.query = self.query.relation("device_setting_list");
        self
    }

    pub fn select_device_setting_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("device_setting_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("device_setting_list", selection));
        self
}

    pub fn have_sample_records(self) -> Self {
        self.with_sample_record_list_matching(SelectQuery::new("SampleRecord"))
    }

    pub fn have_no_sample_records(self) -> Self {
        self.without_sample_record_list_matching(SelectQuery::new("SampleRecord"))
    }

    pub fn with_sample_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::SampleRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "device_system_id",
        ));
        self.relation_filters.push(RelationFilter::new("sample_record_list", selection));
        self
    }

    pub fn without_sample_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::SampleRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "device_system_id",
        ));
        self.relation_filters.push(RelationFilter::new("sample_record_list", selection));
        self
    }

    pub fn select_sample_record_list(mut self) -> Self {
        self.query = self.query.relation("sample_record_list");
        self
    }

    pub fn select_sample_record_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("sample_record_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("sample_record_list", selection));
        self
}
    pub fn count_system_statuses(self) -> Self {
        self.count_system_statuses_as("count_system_statuses")
    }

    pub fn count_system_statuses_as(self, alias: impl Into<String>) -> Self {
        self.count_system_statuses_with(alias, crate::Q::system_statuses().unlimited())
    }

    pub fn count_system_statuses_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "system_status_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_system_statuses(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_system_statuses_as("refinements", request)
    }

    pub fn stats_from_system_statuses_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "system_status_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_system_statuses_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_system_statuses(request)
    }




    pub fn count_device_settings(self) -> Self {
        self.count_device_settings_as("count_device_settings")
    }

    pub fn count_device_settings_as(self, alias: impl Into<String>) -> Self {
        self.count_device_settings_with(alias, crate::Q::device_settings().unlimited())
    }

    pub fn count_device_settings_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "device_setting_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_device_settings(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as("refinements", request)
    }

    pub fn stats_from_device_settings_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "device_setting_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_device_settings_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings(request)
    }


    pub fn sum_calibration_point_of_device_settings(self) -> Self {
        self.sum_calibration_point_of_device_settings_as("sum_calibration_point_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn sum_calibration_point_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().sum("calibration_point", "sum_calibration_point"))
    }
    pub fn min_calibration_point_of_device_settings(self) -> Self {
        self.min_calibration_point_of_device_settings_as("min_calibration_point_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn min_calibration_point_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().min("calibration_point", "min_calibration_point"))
    }
    pub fn max_calibration_point_of_device_settings(self) -> Self {
        self.max_calibration_point_of_device_settings_as("max_calibration_point_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn max_calibration_point_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().max("calibration_point", "max_calibration_point"))
    }
    pub fn avg_calibration_point_of_device_settings(self) -> Self {
        self.avg_calibration_point_of_device_settings_as("avg_calibration_point_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn avg_calibration_point_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().avg("calibration_point", "avg_calibration_point"))
    }
    pub fn standard_deviation_calibration_point_of_device_settings(self) -> Self {
        self.standard_deviation_calibration_point_of_device_settings_as("standard_deviation_calibration_point_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn standard_deviation_calibration_point_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().stddev("calibration_point", "stdDev_calibration_point"))
    }
    pub fn square_root_of_population_standard_deviation_calibration_point_of_device_settings(self) -> Self {
        self.square_root_of_population_standard_deviation_calibration_point_of_device_settings_as("square_root_of_population_standard_deviation_calibration_point_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_calibration_point_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().stddev_pop("calibration_point", "stdDevPop_calibration_point"))
    }
    pub fn sample_variance_calibration_point_of_device_settings(self) -> Self {
        self.sample_variance_calibration_point_of_device_settings_as("sample_variance_calibration_point_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn sample_variance_calibration_point_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().var_samp("calibration_point", "varSamp_calibration_point"))
    }
    pub fn sample_population_variance_calibration_point_of_device_settings(self) -> Self {
        self.sample_population_variance_calibration_point_of_device_settings_as("sample_population_variance_calibration_point_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn sample_population_variance_calibration_point_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().var_pop("calibration_point", "varPop_calibration_point"))
    }
    pub fn sum_data_keep_days_of_device_settings(self) -> Self {
        self.sum_data_keep_days_of_device_settings_as("sum_data_keep_days_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn sum_data_keep_days_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().sum("data_keep_days", "sum_data_keep_days"))
    }
    pub fn min_data_keep_days_of_device_settings(self) -> Self {
        self.min_data_keep_days_of_device_settings_as("min_data_keep_days_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn min_data_keep_days_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().min("data_keep_days", "min_data_keep_days"))
    }
    pub fn max_data_keep_days_of_device_settings(self) -> Self {
        self.max_data_keep_days_of_device_settings_as("max_data_keep_days_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn max_data_keep_days_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().max("data_keep_days", "max_data_keep_days"))
    }
    pub fn avg_data_keep_days_of_device_settings(self) -> Self {
        self.avg_data_keep_days_of_device_settings_as("avg_data_keep_days_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn avg_data_keep_days_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().avg("data_keep_days", "avg_data_keep_days"))
    }
    pub fn standard_deviation_data_keep_days_of_device_settings(self) -> Self {
        self.standard_deviation_data_keep_days_of_device_settings_as("standard_deviation_data_keep_days_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn standard_deviation_data_keep_days_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().stddev("data_keep_days", "stdDev_data_keep_days"))
    }
    pub fn square_root_of_population_standard_deviation_data_keep_days_of_device_settings(self) -> Self {
        self.square_root_of_population_standard_deviation_data_keep_days_of_device_settings_as("square_root_of_population_standard_deviation_data_keep_days_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_data_keep_days_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().stddev_pop("data_keep_days", "stdDevPop_data_keep_days"))
    }
    pub fn sample_variance_data_keep_days_of_device_settings(self) -> Self {
        self.sample_variance_data_keep_days_of_device_settings_as("sample_variance_data_keep_days_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn sample_variance_data_keep_days_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().var_samp("data_keep_days", "varSamp_data_keep_days"))
    }
    pub fn sample_population_variance_data_keep_days_of_device_settings(self) -> Self {
        self.sample_population_variance_data_keep_days_of_device_settings_as("sample_population_variance_data_keep_days_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn sample_population_variance_data_keep_days_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().var_pop("data_keep_days", "varPop_data_keep_days"))
    }
    pub fn sum_sampling_frequency_of_device_settings(self) -> Self {
        self.sum_sampling_frequency_of_device_settings_as("sum_sampling_frequency_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn sum_sampling_frequency_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().sum("sampling_frequency", "sum_sampling_frequency"))
    }
    pub fn min_sampling_frequency_of_device_settings(self) -> Self {
        self.min_sampling_frequency_of_device_settings_as("min_sampling_frequency_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn min_sampling_frequency_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().min("sampling_frequency", "min_sampling_frequency"))
    }
    pub fn max_sampling_frequency_of_device_settings(self) -> Self {
        self.max_sampling_frequency_of_device_settings_as("max_sampling_frequency_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn max_sampling_frequency_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().max("sampling_frequency", "max_sampling_frequency"))
    }
    pub fn avg_sampling_frequency_of_device_settings(self) -> Self {
        self.avg_sampling_frequency_of_device_settings_as("avg_sampling_frequency_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn avg_sampling_frequency_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().avg("sampling_frequency", "avg_sampling_frequency"))
    }
    pub fn standard_deviation_sampling_frequency_of_device_settings(self) -> Self {
        self.standard_deviation_sampling_frequency_of_device_settings_as("standard_deviation_sampling_frequency_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn standard_deviation_sampling_frequency_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().stddev("sampling_frequency", "stdDev_sampling_frequency"))
    }
    pub fn square_root_of_population_standard_deviation_sampling_frequency_of_device_settings(self) -> Self {
        self.square_root_of_population_standard_deviation_sampling_frequency_of_device_settings_as("square_root_of_population_standard_deviation_sampling_frequency_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_sampling_frequency_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().stddev_pop("sampling_frequency", "stdDevPop_sampling_frequency"))
    }
    pub fn sample_variance_sampling_frequency_of_device_settings(self) -> Self {
        self.sample_variance_sampling_frequency_of_device_settings_as("sample_variance_sampling_frequency_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn sample_variance_sampling_frequency_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().var_samp("sampling_frequency", "varSamp_sampling_frequency"))
    }
    pub fn sample_population_variance_sampling_frequency_of_device_settings(self) -> Self {
        self.sample_population_variance_sampling_frequency_of_device_settings_as("sample_population_variance_sampling_frequency_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn sample_population_variance_sampling_frequency_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().var_pop("sampling_frequency", "varPop_sampling_frequency"))
    }
    pub fn sum_password_enabled_of_device_settings(self) -> Self {
        self.sum_password_enabled_of_device_settings_as("sum_password_enabled_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn sum_password_enabled_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().sum("password_enabled", "sum_password_enabled"))
    }
    pub fn min_password_enabled_of_device_settings(self) -> Self {
        self.min_password_enabled_of_device_settings_as("min_password_enabled_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn min_password_enabled_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().min("password_enabled", "min_password_enabled"))
    }
    pub fn max_password_enabled_of_device_settings(self) -> Self {
        self.max_password_enabled_of_device_settings_as("max_password_enabled_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn max_password_enabled_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().max("password_enabled", "max_password_enabled"))
    }
    pub fn avg_password_enabled_of_device_settings(self) -> Self {
        self.avg_password_enabled_of_device_settings_as("avg_password_enabled_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn avg_password_enabled_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().avg("password_enabled", "avg_password_enabled"))
    }
    pub fn standard_deviation_password_enabled_of_device_settings(self) -> Self {
        self.standard_deviation_password_enabled_of_device_settings_as("standard_deviation_password_enabled_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn standard_deviation_password_enabled_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().stddev("password_enabled", "stdDev_password_enabled"))
    }
    pub fn square_root_of_population_standard_deviation_password_enabled_of_device_settings(self) -> Self {
        self.square_root_of_population_standard_deviation_password_enabled_of_device_settings_as("square_root_of_population_standard_deviation_password_enabled_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_password_enabled_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().stddev_pop("password_enabled", "stdDevPop_password_enabled"))
    }
    pub fn sample_variance_password_enabled_of_device_settings(self) -> Self {
        self.sample_variance_password_enabled_of_device_settings_as("sample_variance_password_enabled_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn sample_variance_password_enabled_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().var_samp("password_enabled", "varSamp_password_enabled"))
    }
    pub fn sample_population_variance_password_enabled_of_device_settings(self) -> Self {
        self.sample_population_variance_password_enabled_of_device_settings_as("sample_population_variance_password_enabled_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn sample_population_variance_password_enabled_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().var_pop("password_enabled", "varPop_password_enabled"))
    }
    pub fn min_create_time_of_device_settings(self) -> Self {
        self.min_create_time_of_device_settings_as("min_create_time_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn min_create_time_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_device_settings(self) -> Self {
        self.max_create_time_of_device_settings_as("max_create_time_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn max_create_time_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_device_settings(self) -> Self {
        self.min_update_time_of_device_settings_as("min_update_time_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn min_update_time_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_device_settings(self) -> Self {
        self.max_update_time_of_device_settings_as("max_update_time_of_device_settings", crate::Q::device_settings().unlimited())
    }

    pub fn max_update_time_of_device_settings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_device_settings_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_sample_records(self) -> Self {
        self.count_sample_records_as("count_sample_records")
    }

    pub fn count_sample_records_as(self, alias: impl Into<String>) -> Self {
        self.count_sample_records_with(alias, crate::Q::sample_records().unlimited())
    }

    pub fn count_sample_records_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "sample_record_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_sample_records(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as("refinements", request)
    }

    pub fn stats_from_sample_records_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "sample_record_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_sample_records_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records(request)
    }


    pub fn min_sample_time_of_sample_records(self) -> Self {
        self.min_sample_time_of_sample_records_as("min_sample_time_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn min_sample_time_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().min("sample_time", "min_sample_time"))
    }
    pub fn max_sample_time_of_sample_records(self) -> Self {
        self.max_sample_time_of_sample_records_as("max_sample_time_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn max_sample_time_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().max("sample_time", "max_sample_time"))
    }
    pub fn sum_gas_of_sample_records(self) -> Self {
        self.sum_gas_of_sample_records_as("sum_gas_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sum_gas_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().sum("gas", "sum_gas"))
    }
    pub fn min_gas_of_sample_records(self) -> Self {
        self.min_gas_of_sample_records_as("min_gas_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn min_gas_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().min("gas", "min_gas"))
    }
    pub fn max_gas_of_sample_records(self) -> Self {
        self.max_gas_of_sample_records_as("max_gas_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn max_gas_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().max("gas", "max_gas"))
    }
    pub fn avg_gas_of_sample_records(self) -> Self {
        self.avg_gas_of_sample_records_as("avg_gas_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn avg_gas_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().avg("gas", "avg_gas"))
    }
    pub fn standard_deviation_gas_of_sample_records(self) -> Self {
        self.standard_deviation_gas_of_sample_records_as("standard_deviation_gas_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn standard_deviation_gas_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev("gas", "stdDev_gas"))
    }
    pub fn square_root_of_population_standard_deviation_gas_of_sample_records(self) -> Self {
        self.square_root_of_population_standard_deviation_gas_of_sample_records_as("square_root_of_population_standard_deviation_gas_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_gas_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev_pop("gas", "stdDevPop_gas"))
    }
    pub fn sample_variance_gas_of_sample_records(self) -> Self {
        self.sample_variance_gas_of_sample_records_as("sample_variance_gas_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_variance_gas_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_samp("gas", "varSamp_gas"))
    }
    pub fn sample_population_variance_gas_of_sample_records(self) -> Self {
        self.sample_population_variance_gas_of_sample_records_as("sample_population_variance_gas_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_population_variance_gas_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_pop("gas", "varPop_gas"))
    }
    pub fn sum_lref_of_sample_records(self) -> Self {
        self.sum_lref_of_sample_records_as("sum_lref_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sum_lref_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().sum("lref", "sum_lref"))
    }
    pub fn min_lref_of_sample_records(self) -> Self {
        self.min_lref_of_sample_records_as("min_lref_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn min_lref_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().min("lref", "min_lref"))
    }
    pub fn max_lref_of_sample_records(self) -> Self {
        self.max_lref_of_sample_records_as("max_lref_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn max_lref_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().max("lref", "max_lref"))
    }
    pub fn avg_lref_of_sample_records(self) -> Self {
        self.avg_lref_of_sample_records_as("avg_lref_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn avg_lref_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().avg("lref", "avg_lref"))
    }
    pub fn standard_deviation_lref_of_sample_records(self) -> Self {
        self.standard_deviation_lref_of_sample_records_as("standard_deviation_lref_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn standard_deviation_lref_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev("lref", "stdDev_lref"))
    }
    pub fn square_root_of_population_standard_deviation_lref_of_sample_records(self) -> Self {
        self.square_root_of_population_standard_deviation_lref_of_sample_records_as("square_root_of_population_standard_deviation_lref_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_lref_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev_pop("lref", "stdDevPop_lref"))
    }
    pub fn sample_variance_lref_of_sample_records(self) -> Self {
        self.sample_variance_lref_of_sample_records_as("sample_variance_lref_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_variance_lref_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_samp("lref", "varSamp_lref"))
    }
    pub fn sample_population_variance_lref_of_sample_records(self) -> Self {
        self.sample_population_variance_lref_of_sample_records_as("sample_population_variance_lref_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_population_variance_lref_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_pop("lref", "varPop_lref"))
    }
    pub fn sum_impurity1_of_sample_records(self) -> Self {
        self.sum_impurity1_of_sample_records_as("sum_impurity1_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sum_impurity1_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().sum("impurity1", "sum_impurity1"))
    }
    pub fn min_impurity1_of_sample_records(self) -> Self {
        self.min_impurity1_of_sample_records_as("min_impurity1_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn min_impurity1_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().min("impurity1", "min_impurity1"))
    }
    pub fn max_impurity1_of_sample_records(self) -> Self {
        self.max_impurity1_of_sample_records_as("max_impurity1_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn max_impurity1_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().max("impurity1", "max_impurity1"))
    }
    pub fn avg_impurity1_of_sample_records(self) -> Self {
        self.avg_impurity1_of_sample_records_as("avg_impurity1_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn avg_impurity1_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().avg("impurity1", "avg_impurity1"))
    }
    pub fn standard_deviation_impurity1_of_sample_records(self) -> Self {
        self.standard_deviation_impurity1_of_sample_records_as("standard_deviation_impurity1_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn standard_deviation_impurity1_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev("impurity1", "stdDev_impurity1"))
    }
    pub fn square_root_of_population_standard_deviation_impurity1_of_sample_records(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity1_of_sample_records_as("square_root_of_population_standard_deviation_impurity1_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_impurity1_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev_pop("impurity1", "stdDevPop_impurity1"))
    }
    pub fn sample_variance_impurity1_of_sample_records(self) -> Self {
        self.sample_variance_impurity1_of_sample_records_as("sample_variance_impurity1_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_variance_impurity1_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_samp("impurity1", "varSamp_impurity1"))
    }
    pub fn sample_population_variance_impurity1_of_sample_records(self) -> Self {
        self.sample_population_variance_impurity1_of_sample_records_as("sample_population_variance_impurity1_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_population_variance_impurity1_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_pop("impurity1", "varPop_impurity1"))
    }
    pub fn sum_impurity2_of_sample_records(self) -> Self {
        self.sum_impurity2_of_sample_records_as("sum_impurity2_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sum_impurity2_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().sum("impurity2", "sum_impurity2"))
    }
    pub fn min_impurity2_of_sample_records(self) -> Self {
        self.min_impurity2_of_sample_records_as("min_impurity2_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn min_impurity2_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().min("impurity2", "min_impurity2"))
    }
    pub fn max_impurity2_of_sample_records(self) -> Self {
        self.max_impurity2_of_sample_records_as("max_impurity2_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn max_impurity2_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().max("impurity2", "max_impurity2"))
    }
    pub fn avg_impurity2_of_sample_records(self) -> Self {
        self.avg_impurity2_of_sample_records_as("avg_impurity2_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn avg_impurity2_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().avg("impurity2", "avg_impurity2"))
    }
    pub fn standard_deviation_impurity2_of_sample_records(self) -> Self {
        self.standard_deviation_impurity2_of_sample_records_as("standard_deviation_impurity2_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn standard_deviation_impurity2_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev("impurity2", "stdDev_impurity2"))
    }
    pub fn square_root_of_population_standard_deviation_impurity2_of_sample_records(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity2_of_sample_records_as("square_root_of_population_standard_deviation_impurity2_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_impurity2_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev_pop("impurity2", "stdDevPop_impurity2"))
    }
    pub fn sample_variance_impurity2_of_sample_records(self) -> Self {
        self.sample_variance_impurity2_of_sample_records_as("sample_variance_impurity2_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_variance_impurity2_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_samp("impurity2", "varSamp_impurity2"))
    }
    pub fn sample_population_variance_impurity2_of_sample_records(self) -> Self {
        self.sample_population_variance_impurity2_of_sample_records_as("sample_population_variance_impurity2_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_population_variance_impurity2_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_pop("impurity2", "varPop_impurity2"))
    }
    pub fn sum_impurity3_of_sample_records(self) -> Self {
        self.sum_impurity3_of_sample_records_as("sum_impurity3_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sum_impurity3_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().sum("impurity3", "sum_impurity3"))
    }
    pub fn min_impurity3_of_sample_records(self) -> Self {
        self.min_impurity3_of_sample_records_as("min_impurity3_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn min_impurity3_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().min("impurity3", "min_impurity3"))
    }
    pub fn max_impurity3_of_sample_records(self) -> Self {
        self.max_impurity3_of_sample_records_as("max_impurity3_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn max_impurity3_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().max("impurity3", "max_impurity3"))
    }
    pub fn avg_impurity3_of_sample_records(self) -> Self {
        self.avg_impurity3_of_sample_records_as("avg_impurity3_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn avg_impurity3_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().avg("impurity3", "avg_impurity3"))
    }
    pub fn standard_deviation_impurity3_of_sample_records(self) -> Self {
        self.standard_deviation_impurity3_of_sample_records_as("standard_deviation_impurity3_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn standard_deviation_impurity3_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev("impurity3", "stdDev_impurity3"))
    }
    pub fn square_root_of_population_standard_deviation_impurity3_of_sample_records(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity3_of_sample_records_as("square_root_of_population_standard_deviation_impurity3_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_impurity3_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev_pop("impurity3", "stdDevPop_impurity3"))
    }
    pub fn sample_variance_impurity3_of_sample_records(self) -> Self {
        self.sample_variance_impurity3_of_sample_records_as("sample_variance_impurity3_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_variance_impurity3_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_samp("impurity3", "varSamp_impurity3"))
    }
    pub fn sample_population_variance_impurity3_of_sample_records(self) -> Self {
        self.sample_population_variance_impurity3_of_sample_records_as("sample_population_variance_impurity3_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_population_variance_impurity3_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_pop("impurity3", "varPop_impurity3"))
    }
    pub fn sum_impurity4_of_sample_records(self) -> Self {
        self.sum_impurity4_of_sample_records_as("sum_impurity4_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sum_impurity4_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().sum("impurity4", "sum_impurity4"))
    }
    pub fn min_impurity4_of_sample_records(self) -> Self {
        self.min_impurity4_of_sample_records_as("min_impurity4_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn min_impurity4_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().min("impurity4", "min_impurity4"))
    }
    pub fn max_impurity4_of_sample_records(self) -> Self {
        self.max_impurity4_of_sample_records_as("max_impurity4_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn max_impurity4_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().max("impurity4", "max_impurity4"))
    }
    pub fn avg_impurity4_of_sample_records(self) -> Self {
        self.avg_impurity4_of_sample_records_as("avg_impurity4_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn avg_impurity4_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().avg("impurity4", "avg_impurity4"))
    }
    pub fn standard_deviation_impurity4_of_sample_records(self) -> Self {
        self.standard_deviation_impurity4_of_sample_records_as("standard_deviation_impurity4_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn standard_deviation_impurity4_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev("impurity4", "stdDev_impurity4"))
    }
    pub fn square_root_of_population_standard_deviation_impurity4_of_sample_records(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity4_of_sample_records_as("square_root_of_population_standard_deviation_impurity4_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_impurity4_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev_pop("impurity4", "stdDevPop_impurity4"))
    }
    pub fn sample_variance_impurity4_of_sample_records(self) -> Self {
        self.sample_variance_impurity4_of_sample_records_as("sample_variance_impurity4_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_variance_impurity4_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_samp("impurity4", "varSamp_impurity4"))
    }
    pub fn sample_population_variance_impurity4_of_sample_records(self) -> Self {
        self.sample_population_variance_impurity4_of_sample_records_as("sample_population_variance_impurity4_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_population_variance_impurity4_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_pop("impurity4", "varPop_impurity4"))
    }
    pub fn sum_impurity5_of_sample_records(self) -> Self {
        self.sum_impurity5_of_sample_records_as("sum_impurity5_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sum_impurity5_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().sum("impurity5", "sum_impurity5"))
    }
    pub fn min_impurity5_of_sample_records(self) -> Self {
        self.min_impurity5_of_sample_records_as("min_impurity5_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn min_impurity5_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().min("impurity5", "min_impurity5"))
    }
    pub fn max_impurity5_of_sample_records(self) -> Self {
        self.max_impurity5_of_sample_records_as("max_impurity5_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn max_impurity5_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().max("impurity5", "max_impurity5"))
    }
    pub fn avg_impurity5_of_sample_records(self) -> Self {
        self.avg_impurity5_of_sample_records_as("avg_impurity5_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn avg_impurity5_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().avg("impurity5", "avg_impurity5"))
    }
    pub fn standard_deviation_impurity5_of_sample_records(self) -> Self {
        self.standard_deviation_impurity5_of_sample_records_as("standard_deviation_impurity5_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn standard_deviation_impurity5_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev("impurity5", "stdDev_impurity5"))
    }
    pub fn square_root_of_population_standard_deviation_impurity5_of_sample_records(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity5_of_sample_records_as("square_root_of_population_standard_deviation_impurity5_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_impurity5_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev_pop("impurity5", "stdDevPop_impurity5"))
    }
    pub fn sample_variance_impurity5_of_sample_records(self) -> Self {
        self.sample_variance_impurity5_of_sample_records_as("sample_variance_impurity5_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_variance_impurity5_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_samp("impurity5", "varSamp_impurity5"))
    }
    pub fn sample_population_variance_impurity5_of_sample_records(self) -> Self {
        self.sample_population_variance_impurity5_of_sample_records_as("sample_population_variance_impurity5_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_population_variance_impurity5_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_pop("impurity5", "varPop_impurity5"))
    }
    pub fn sum_impurity6_of_sample_records(self) -> Self {
        self.sum_impurity6_of_sample_records_as("sum_impurity6_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sum_impurity6_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().sum("impurity6", "sum_impurity6"))
    }
    pub fn min_impurity6_of_sample_records(self) -> Self {
        self.min_impurity6_of_sample_records_as("min_impurity6_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn min_impurity6_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().min("impurity6", "min_impurity6"))
    }
    pub fn max_impurity6_of_sample_records(self) -> Self {
        self.max_impurity6_of_sample_records_as("max_impurity6_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn max_impurity6_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().max("impurity6", "max_impurity6"))
    }
    pub fn avg_impurity6_of_sample_records(self) -> Self {
        self.avg_impurity6_of_sample_records_as("avg_impurity6_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn avg_impurity6_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().avg("impurity6", "avg_impurity6"))
    }
    pub fn standard_deviation_impurity6_of_sample_records(self) -> Self {
        self.standard_deviation_impurity6_of_sample_records_as("standard_deviation_impurity6_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn standard_deviation_impurity6_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev("impurity6", "stdDev_impurity6"))
    }
    pub fn square_root_of_population_standard_deviation_impurity6_of_sample_records(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity6_of_sample_records_as("square_root_of_population_standard_deviation_impurity6_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_impurity6_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev_pop("impurity6", "stdDevPop_impurity6"))
    }
    pub fn sample_variance_impurity6_of_sample_records(self) -> Self {
        self.sample_variance_impurity6_of_sample_records_as("sample_variance_impurity6_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_variance_impurity6_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_samp("impurity6", "varSamp_impurity6"))
    }
    pub fn sample_population_variance_impurity6_of_sample_records(self) -> Self {
        self.sample_population_variance_impurity6_of_sample_records_as("sample_population_variance_impurity6_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_population_variance_impurity6_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_pop("impurity6", "varPop_impurity6"))
    }
    pub fn sum_impurity7_of_sample_records(self) -> Self {
        self.sum_impurity7_of_sample_records_as("sum_impurity7_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sum_impurity7_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().sum("impurity7", "sum_impurity7"))
    }
    pub fn min_impurity7_of_sample_records(self) -> Self {
        self.min_impurity7_of_sample_records_as("min_impurity7_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn min_impurity7_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().min("impurity7", "min_impurity7"))
    }
    pub fn max_impurity7_of_sample_records(self) -> Self {
        self.max_impurity7_of_sample_records_as("max_impurity7_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn max_impurity7_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().max("impurity7", "max_impurity7"))
    }
    pub fn avg_impurity7_of_sample_records(self) -> Self {
        self.avg_impurity7_of_sample_records_as("avg_impurity7_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn avg_impurity7_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().avg("impurity7", "avg_impurity7"))
    }
    pub fn standard_deviation_impurity7_of_sample_records(self) -> Self {
        self.standard_deviation_impurity7_of_sample_records_as("standard_deviation_impurity7_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn standard_deviation_impurity7_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev("impurity7", "stdDev_impurity7"))
    }
    pub fn square_root_of_population_standard_deviation_impurity7_of_sample_records(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity7_of_sample_records_as("square_root_of_population_standard_deviation_impurity7_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_impurity7_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev_pop("impurity7", "stdDevPop_impurity7"))
    }
    pub fn sample_variance_impurity7_of_sample_records(self) -> Self {
        self.sample_variance_impurity7_of_sample_records_as("sample_variance_impurity7_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_variance_impurity7_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_samp("impurity7", "varSamp_impurity7"))
    }
    pub fn sample_population_variance_impurity7_of_sample_records(self) -> Self {
        self.sample_population_variance_impurity7_of_sample_records_as("sample_population_variance_impurity7_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_population_variance_impurity7_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_pop("impurity7", "varPop_impurity7"))
    }
    pub fn sum_impurity8_of_sample_records(self) -> Self {
        self.sum_impurity8_of_sample_records_as("sum_impurity8_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sum_impurity8_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().sum("impurity8", "sum_impurity8"))
    }
    pub fn min_impurity8_of_sample_records(self) -> Self {
        self.min_impurity8_of_sample_records_as("min_impurity8_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn min_impurity8_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().min("impurity8", "min_impurity8"))
    }
    pub fn max_impurity8_of_sample_records(self) -> Self {
        self.max_impurity8_of_sample_records_as("max_impurity8_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn max_impurity8_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().max("impurity8", "max_impurity8"))
    }
    pub fn avg_impurity8_of_sample_records(self) -> Self {
        self.avg_impurity8_of_sample_records_as("avg_impurity8_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn avg_impurity8_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().avg("impurity8", "avg_impurity8"))
    }
    pub fn standard_deviation_impurity8_of_sample_records(self) -> Self {
        self.standard_deviation_impurity8_of_sample_records_as("standard_deviation_impurity8_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn standard_deviation_impurity8_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev("impurity8", "stdDev_impurity8"))
    }
    pub fn square_root_of_population_standard_deviation_impurity8_of_sample_records(self) -> Self {
        self.square_root_of_population_standard_deviation_impurity8_of_sample_records_as("square_root_of_population_standard_deviation_impurity8_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_impurity8_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().stddev_pop("impurity8", "stdDevPop_impurity8"))
    }
    pub fn sample_variance_impurity8_of_sample_records(self) -> Self {
        self.sample_variance_impurity8_of_sample_records_as("sample_variance_impurity8_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_variance_impurity8_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_samp("impurity8", "varSamp_impurity8"))
    }
    pub fn sample_population_variance_impurity8_of_sample_records(self) -> Self {
        self.sample_population_variance_impurity8_of_sample_records_as("sample_population_variance_impurity8_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn sample_population_variance_impurity8_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().var_pop("impurity8", "varPop_impurity8"))
    }
    pub fn min_create_time_of_sample_records(self) -> Self {
        self.min_create_time_of_sample_records_as("min_create_time_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn min_create_time_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_sample_records(self) -> Self {
        self.max_create_time_of_sample_records_as("max_create_time_of_sample_records", crate::Q::sample_records().unlimited())
    }

    pub fn max_create_time_of_sample_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_sample_records_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
}

impl<R> Default for DeviceSystemRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< DeviceSystemRequest<R> > for SelectQuery {
    fn from(request: DeviceSystemRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< DeviceSystemRequest<R> > for QuerySelection {
    fn from(request: DeviceSystemRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::DeviceSystem> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::DeviceSystemRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move {
            teaql_runtime::save_audited_ledger_entity(self, ctx.user_context())
                .await
                .map_err(DataServiceError::Runtime)
        })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<DeviceSystemRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::DeviceSystem
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::DeviceSystem::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> DeviceSystemRequest<R> {
        self.inner.query.trace_chain.push(teaql_core::TraceNode::new(
            self.inner.query.entity.clone(),
            None,
            self.purpose,
        ));
        self.inner
    }

    pub async fn execute_for_page<'a, C>(
        self,
        ctx: &'a C,
        offset: u64,
        limit: u64,
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::DeviceSystemRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
