use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{RepositoryError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::MatchGoal {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::MatchGoal {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

#[derive(Debug)]
pub struct MatchGoalRequest<R = crate::MatchGoal> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for MatchGoalRequest<R> {
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

impl<R> MatchGoalRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("MatchGoal"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> MatchGoalRequest<T> {
        MatchGoalRequest {
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
    ) -> Result<SmartList<R>, TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .match_goal_repository()
            .map_err(|err| RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let query_options = self.query_options.clone();
        let relation_aggregates = runtime_relation_aggregates(&query_options);
        let query = apply_runtime_metadata(self.query, &query_options, &self.child_enhancements);
        let mut rows = repository.fetch_enhanced_entities_with_relation_aggregates::<R>(
            &query,
            &relation_aggregates,
        ).await?;
        let facets = execute_facets(ctx, &query, &query_options)
            .await
            .map_err(RepositoryError::Runtime)?;
        attach_facets(&mut rows, facets);
        Ok(rows)
    }

    pub(crate) async fn _execute_for_stream<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .match_goal_repository()
            .map_err(|err| RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let query_options = self.query_options.clone();
        let query = apply_runtime_metadata(self.query, &query_options, &self.child_enhancements);
        let chunks = repository.fetch_stream(&query)
            .await?;
        Ok(chunks)
    }

    pub(crate) async fn _execute_for_first<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Option<R>, TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        self._execute_for_first(ctx).await
    }


    pub async fn execute_for_page<'a, C>(
        self,
        ctx: &'a C,
        offset: u64,
        limit: u64,
    ) -> Result<SmartList<R>, TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
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
    ) -> Result<u64, TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .match_goal_repository()
            .map_err(|err| RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
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
            .ok_or_else(|| RepositoryError::Runtime(RuntimeError::Graph(format!("count result for MatchGoal is missing or not numeric"))))
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .match_goal_repository()
            .map_err(|err| RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .match_goal_repository()
            .map_err(|err| RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let query_options = self.query_options.clone();
        let outer_query = self.query.clone();
        let relation_aggregates = runtime_relation_aggregates(&query_options);
        let query = apply_runtime_metadata(self.query, &query_options, &self.child_enhancements);
        let mut rows = repository.fetch_smart_list_with_relation_aggregates(&query, &relation_aggregates).await?;
        let facets = execute_facets(ctx, &outer_query, &query_options)
            .await
            .map_err(RepositoryError::Runtime)?;
        attach_facets(&mut rows, facets);
        Ok(rows)
    }

    pub(crate) async fn _execute_for_record<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Option<Record>, TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
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
            "player_name" => Some("player_name"),
            "minute_scored" => Some("minute_scored"),
            "create_time" => Some("create_time"),
            "update_time" => Some("update_time"),
            "version" => Some("version"),
            "tournament_match" | "tournament_match_id" => Some("tournament_match_id"),
            "tournament_team" | "tournament_team_id" => Some("tournament_team_id"),
            "goal_category" | "goal_category_id" => Some("goal_category_id"),
            "tournament" | "tournament_id" => Some("tournament_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "tournament_match" => {
                self.with_tournament_match_matching(
                    crate::Q::tournament_matches_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "tournament_team" => {
                self.with_tournament_team_matching(
                    crate::Q::tournament_teams_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "goal_category" => {
                self.with_goal_category_matching(
                    crate::Q::goal_categories_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "tournament" => {
                self.with_tournament_matching(
                    crate::Q::tournaments_minimal()
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
        self.query = self.query.project("player_name");
        self.query = self.query.project("minute_scored");
        self.query = self.query.project("create_time");
        self.query = self.query.project("update_time");
        self.query = self.query.project("version");
        self.query = self.query.project("tournament_match_id");
        self.query = self.query.project("tournament_team_id");
        self.query = self.query.project("goal_category_id");
        self.query = self.query.project("tournament_id");
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
        request = request.select_tournament_match();
        request = request.select_tournament_team();
        request = request.select_goal_category();
        request = request.select_tournament();
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

    pub fn select_id(mut self) -> Self {
        self.query = self.query.project("id");
        self
    }

    pub fn project_id(self) -> Self {
        self.select_id()
    }

    pub fn select_id_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_id_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_id_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("id", raw_sql_segment));
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

    pub fn unselect_id(mut self) -> Self {
        self.query.projection.retain(|field| field != "id");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "id");
        self
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

    pub fn select_player_name(mut self) -> Self {
        self.query = self.query.project("player_name");
        self
    }

    pub fn project_player_name(self) -> Self {
        self.select_player_name()
    }

    pub fn select_player_name_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_player_name_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_player_name_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("player_name", raw_sql_segment));
        self
    }

    pub fn group_by_player_name(self) -> Self {
        self.group_by("player_name")
    }

    pub fn group_by_player_name_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("player_name");
        request.query = request
            .query
            .project_expr(alias, Expr::column("player_name"));
        request
    }

    pub fn group_by_player_name_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("player_name")
            .aggregate_with_function("player_name", alias, function)
    }

    pub fn count_player_name(self) -> Self {
        self.count_player_name_as("player_name_count")
    }

    pub fn count_player_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("player_name", alias)
    }

    pub fn sum_player_name(self) -> Self {
        self.sum_player_name_as("sum_player_name")
    }

    pub fn sum_player_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("player_name", alias)
    }

    pub fn avg_player_name(self) -> Self {
        self.avg_player_name_as("avg_player_name")
    }

    pub fn avg_player_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("player_name", alias)
    }

    pub fn min_player_name(self) -> Self {
        self.min_player_name_as("min_player_name")
    }

    pub fn min_player_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("player_name", alias)
    }

    pub fn max_player_name(self) -> Self {
        self.max_player_name_as("max_player_name")
    }

    pub fn max_player_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("player_name", alias)
    }

    pub fn unselect_player_name(mut self) -> Self {
        self.query.projection.retain(|field| field != "player_name");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "player_name");
        self
    }


    pub fn with_player_name(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "player_name",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_player_name_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "player_name",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_player_name_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("player_name", value));
        self
    }



    pub fn with_player_name_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("player_name", value));
        self
    }

    pub fn with_player_name_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("player_name", value));
        self
    }

    pub fn with_player_name_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("player_name", value));
        self
    }

    pub fn with_player_name_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("player_name", value));
        self
    }

    pub fn with_player_name_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("player_name", value));
        self
    }

    pub fn with_player_name_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("player_name", lower, upper));
        self
    }

    pub fn with_player_name_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "player_name",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_player_name_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "player_name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_player_name_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "player_name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_player_name_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("player_name", value));
        self
    }

    pub fn with_player_name_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("player_name", value));
        self
    }

    pub fn with_player_name_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("player_name", value));
        self
    }

    pub fn with_player_name_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("player_name", value));
        self
    }

    pub fn with_player_name_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("player_name", value));
        self
    }

    pub fn with_player_name_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("player_name", value));
        self
    }

    pub fn with_player_name_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("player_name", value));
        self
    }
    pub fn with_player_name_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("player_name", value));
        self
    }

    pub fn with_player_name_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("player_name", value));
        self
    }

    pub fn with_player_name_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("player_name"));
        self
    }



    pub fn with_player_name_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("player_name"));
        self
    }


    pub fn order_by_player_name_asc(mut self) -> Self {
        self.query = self.query.order_asc("player_name");
        self
    }

    pub fn order_by_player_name_desc(mut self) -> Self {
        self.query = self.query.order_desc("player_name");
        self
    }

    pub fn order_by_player_name_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("player_name");
        self
    }

    pub fn order_by_player_name_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("player_name");
        self
    }

    pub fn select_minute_scored(mut self) -> Self {
        self.query = self.query.project("minute_scored");
        self
    }

    pub fn project_minute_scored(self) -> Self {
        self.select_minute_scored()
    }

    pub fn select_minute_scored_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_minute_scored_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_minute_scored_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("minute_scored", raw_sql_segment));
        self
    }

    pub fn select_minute_scored_with_function(self, function: AggregateFunction) -> Self {
        self.select_minute_scored_as_with_function("minute_scored", function)
    }

    pub fn select_minute_scored_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("minute_scored", alias, function)
    }

    pub fn group_by_minute_scored(self) -> Self {
        self.group_by("minute_scored")
    }

    pub fn group_by_minute_scored_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("minute_scored");
        request.query = request
            .query
            .project_expr(alias, Expr::column("minute_scored"));
        request
    }

    pub fn group_by_minute_scored_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("minute_scored")
            .aggregate_with_function("minute_scored", alias, function)
    }

    pub fn count_minute_scored(self) -> Self {
        self.count_minute_scored_as("minute_scored_count")
    }

    pub fn count_minute_scored_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("minute_scored", alias)
    }

    pub fn sum_minute_scored(self) -> Self {
        self.sum_minute_scored_as("sum_minute_scored")
    }

    pub fn sum_minute_scored_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("minute_scored", alias)
    }

    pub fn avg_minute_scored(self) -> Self {
        self.avg_minute_scored_as("avg_minute_scored")
    }

    pub fn avg_minute_scored_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("minute_scored", alias)
    }

    pub fn min_minute_scored(self) -> Self {
        self.min_minute_scored_as("min_minute_scored")
    }

    pub fn min_minute_scored_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("minute_scored", alias)
    }

    pub fn max_minute_scored(self) -> Self {
        self.max_minute_scored_as("max_minute_scored")
    }

    pub fn max_minute_scored_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("minute_scored", alias)
    }

    pub fn standard_deviation_minute_scored(self) -> Self {
        self.standard_deviation_minute_scored_as("stdDev_minute_scored")
    }

    pub fn standard_deviation_minute_scored_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("minute_scored", alias)
    }

    pub fn square_root_of_population_standard_deviation_minute_scored(self) -> Self {
        self.square_root_of_population_standard_deviation_minute_scored_as("stdDevPop_minute_scored")
    }

    pub fn square_root_of_population_standard_deviation_minute_scored_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("minute_scored", alias)
    }

    pub fn sample_variance_minute_scored(self) -> Self {
        self.sample_variance_minute_scored_as("varSamp_minute_scored")
    }

    pub fn sample_variance_minute_scored_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("minute_scored", alias)
    }

    pub fn sample_population_variance_minute_scored(self) -> Self {
        self.sample_population_variance_minute_scored_as("varPop_minute_scored")
    }

    pub fn sample_population_variance_minute_scored_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("minute_scored", alias)
    }

    pub fn unselect_minute_scored(mut self) -> Self {
        self.query.projection.retain(|field| field != "minute_scored");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "minute_scored");
        self
    }


    pub fn with_minute_scored(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "minute_scored",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_minute_scored_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "minute_scored",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_minute_scored_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("minute_scored", value));
        self
    }



    pub fn with_minute_scored_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("minute_scored", value));
        self
    }

    pub fn with_minute_scored_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("minute_scored", value));
        self
    }

    pub fn with_minute_scored_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("minute_scored", value));
        self
    }

    pub fn with_minute_scored_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("minute_scored", value));
        self
    }

    pub fn with_minute_scored_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("minute_scored", value));
        self
    }

    pub fn with_minute_scored_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("minute_scored", lower, upper));
        self
    }

    pub fn with_minute_scored_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "minute_scored",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_minute_scored_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "minute_scored",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_minute_scored_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "minute_scored",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_minute_scored_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("minute_scored", value));
        self
    }

    pub fn with_minute_scored_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("minute_scored", value));
        self
    }

    pub fn with_minute_scored_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("minute_scored"));
        self
    }



    pub fn with_minute_scored_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("minute_scored"));
        self
    }


    pub fn order_by_minute_scored_asc(mut self) -> Self {
        self.query = self.query.order_asc("minute_scored");
        self
    }

    pub fn order_by_minute_scored_desc(mut self) -> Self {
        self.query = self.query.order_desc("minute_scored");
        self
    }

    pub fn order_by_minute_scored_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("minute_scored");
        self
    }

    pub fn order_by_minute_scored_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("minute_scored");
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

    pub fn select_version(mut self) -> Self {
        self.query = self.query.project("version");
        self
    }

    pub fn project_version(self) -> Self {
        self.select_version()
    }

    pub fn select_version_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_version_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_version_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("version", raw_sql_segment));
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

    pub fn unselect_version(mut self) -> Self {
        self.query.projection.retain(|field| field != "version");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "version");
        self
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
    pub fn filter_by_tournament_match(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("tournament_match_id", value.entity_id_value()));
        self
    }

    pub fn with_tournament_match_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "tournament_match_id",
            <crate::TournamentMatch as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("tournament_match", selection));
        self
    }


    pub fn without_tournament_match_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "tournament_match_id",
            <crate::TournamentMatch as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("tournament_match", selection));
        self
    }


    pub fn have_tournament_match(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("tournament_match_id"));
        self
    }

    pub fn have_no_tournament_match(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("tournament_match_id"));
        self
    }


    pub fn group_by_tournament_match(self) -> Self {
        self.group_by("tournament_match_id")
    }

    pub fn group_by_tournament_match_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("tournament_match_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("tournament_match_id"));
        request
    }

    pub fn group_by_tournament_match_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("tournament_match_id")
            .aggregate_with_function("tournament_match_id", alias, function)
    }

    pub fn group_by_tournament_match_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("tournament_match_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "tournament_match",
            "tournament_match_id",
            request,
        ));
        self
    }

    pub fn group_by_tournament_match_with_details(self) -> Self {
        self.group_by_tournament_match_with_details_from(crate::Q::tournament_matches().unlimited())
    }

    pub fn group_by_tournament_match_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_tournament_match_with(request)
    }


    pub fn roll_up_to_tournament_match(self) -> Self {
        self.roll_up_to_tournament_match_with(crate::Q::tournament_matches().unlimited())
    }

    pub fn roll_up_to_tournament_match_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_tournament_match_matching(selection.clone())
            .group_by_tournament_match_with(selection)
    }

    pub fn count_tournament_match(self) -> Self {
        self.count_tournament_match_as("tournament_match_count")
    }

    pub fn count_tournament_match_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("tournament_match_id", alias)
    }

    pub fn unselect_tournament_match(mut self) -> Self {
        self.query.projection.retain(|field| field != "tournament_match_id");
        self.query.relations.retain(|relation| relation.name != "tournament_match");
        self
    }


    pub fn filter_by_tournament_team(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("tournament_team_id", value.entity_id_value()));
        self
    }

    pub fn with_tournament_team_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "tournament_team_id",
            <crate::TournamentTeam as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("tournament_team", selection));
        self
    }


    pub fn without_tournament_team_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "tournament_team_id",
            <crate::TournamentTeam as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("tournament_team", selection));
        self
    }


    pub fn have_tournament_team(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("tournament_team_id"));
        self
    }

    pub fn have_no_tournament_team(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("tournament_team_id"));
        self
    }


    pub fn group_by_tournament_team(self) -> Self {
        self.group_by("tournament_team_id")
    }

    pub fn group_by_tournament_team_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("tournament_team_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("tournament_team_id"));
        request
    }

    pub fn group_by_tournament_team_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("tournament_team_id")
            .aggregate_with_function("tournament_team_id", alias, function)
    }

    pub fn group_by_tournament_team_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("tournament_team_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "tournament_team",
            "tournament_team_id",
            request,
        ));
        self
    }

    pub fn group_by_tournament_team_with_details(self) -> Self {
        self.group_by_tournament_team_with_details_from(crate::Q::tournament_teams().unlimited())
    }

    pub fn group_by_tournament_team_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_tournament_team_with(request)
    }


    pub fn roll_up_to_tournament_team(self) -> Self {
        self.roll_up_to_tournament_team_with(crate::Q::tournament_teams().unlimited())
    }

    pub fn roll_up_to_tournament_team_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_tournament_team_matching(selection.clone())
            .group_by_tournament_team_with(selection)
    }

    pub fn count_tournament_team(self) -> Self {
        self.count_tournament_team_as("tournament_team_count")
    }

    pub fn count_tournament_team_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("tournament_team_id", alias)
    }

    pub fn unselect_tournament_team(mut self) -> Self {
        self.query.projection.retain(|field| field != "tournament_team_id");
        self.query.relations.retain(|relation| relation.name != "tournament_team");
        self
    }


    /// Please use `with_goal_category_is` instead
    pub(crate) fn filter_by_goal_category(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("goal_category_id", value.entity_id_value()));
        self
    }
    /// Complex relation filter for `goal_category`.
    ///
    /// **Usage Priority:**
    ///
    /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
    ///    - [`Self::with_goal_category_is_xxx`]
    ///
    ///    This gives the best code readability.
    ///
    /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
    ///
    /// # Example
    /// ```rust
    /// // Only use when building dynamic queries
    /// let dynamic_query = crate::Q::goal_categories_minimal().filter(...);
    /// let request = crate::Q::match_goals().with_goal_category_matching(dynamic_query);
    /// ```
    pub fn with_goal_category_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "goal_category_id",
            <crate::GoalCategory as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("goal_category", selection));
        self
    }


    /// Complex relation filter for `goal_category`.
    ///
    /// **Usage Priority:**
    ///
    /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
    ///    - [`Self::with_goal_category_is_not_xxx`]
    ///
    ///    This gives the best code readability.
    ///
    /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
    ///
    /// # Example
    /// ```rust
    /// // Only use when building dynamic queries
    /// let dynamic_query = crate::Q::goal_categories_minimal().filter(...);
    /// let request = crate::Q::match_goals().without_goal_category_matching(dynamic_query);
    /// ```
    pub fn without_goal_category_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "goal_category_id",
            <crate::GoalCategory as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("goal_category", selection));
        self
    }


    pub fn have_goal_category(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("goal_category_id"));
        self
    }

    pub fn have_no_goal_category(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("goal_category_id"));
        self
    }


    pub fn group_by_goal_category(self) -> Self {
        self.group_by("goal_category_id")
    }

    pub fn group_by_goal_category_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("goal_category_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("goal_category_id"));
        request
    }

    pub fn group_by_goal_category_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("goal_category_id")
            .aggregate_with_function("goal_category_id", alias, function)
    }

    pub fn group_by_goal_category_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("goal_category_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "goal_category",
            "goal_category_id",
            request,
        ));
        self
    }

    pub fn group_by_goal_category_with_details(self) -> Self {
        self.group_by_goal_category_with_details_from(crate::Q::goal_categories().unlimited())
    }

    pub fn group_by_goal_category_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_goal_category_with(request)
    }


    pub fn roll_up_to_goal_category(self) -> Self {
        self.roll_up_to_goal_category_with(crate::Q::goal_categories().unlimited())
    }

    pub fn roll_up_to_goal_category_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_goal_category_matching(selection.clone())
            .group_by_goal_category_with(selection)
    }

    pub fn count_goal_category(self) -> Self {
        self.count_goal_category_as("goal_category_count")
    }

    pub fn count_goal_category_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("goal_category_id", alias)
    }

    pub fn unselect_goal_category(mut self) -> Self {
        self.query.projection.retain(|field| field != "goal_category_id");
        self.query.relations.retain(|relation| relation.name != "goal_category");
        self
    }


    pub fn filter_by_tournament(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("tournament_id", value.entity_id_value()));
        self
    }

    pub fn with_tournament_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "tournament_id",
            <crate::Tournament as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("tournament", selection));
        self
    }


    pub fn without_tournament_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "tournament_id",
            <crate::Tournament as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("tournament", selection));
        self
    }


    pub fn have_tournament(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("tournament_id"));
        self
    }

    pub fn have_no_tournament(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("tournament_id"));
        self
    }


    pub fn group_by_tournament(self) -> Self {
        self.group_by("tournament_id")
    }

    pub fn group_by_tournament_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("tournament_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("tournament_id"));
        request
    }

    pub fn group_by_tournament_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("tournament_id")
            .aggregate_with_function("tournament_id", alias, function)
    }

    pub fn group_by_tournament_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("tournament_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "tournament",
            "tournament_id",
            request,
        ));
        self
    }

    pub fn group_by_tournament_with_details(self) -> Self {
        self.group_by_tournament_with_details_from(crate::Q::tournaments().unlimited())
    }

    pub fn group_by_tournament_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_tournament_with(request)
    }


    pub fn roll_up_to_tournament(self) -> Self {
        self.roll_up_to_tournament_with(crate::Q::tournaments().unlimited())
    }

    pub fn roll_up_to_tournament_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_tournament_matching(selection.clone())
            .group_by_tournament_with(selection)
    }

    pub fn count_tournament(self) -> Self {
        self.count_tournament_as("tournament_count")
    }

    pub fn count_tournament_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("tournament_id", alias)
    }

    pub fn unselect_tournament(mut self) -> Self {
        self.query.projection.retain(|field| field != "tournament_id");
        self.query.relations.retain(|relation| relation.name != "tournament");
        self
    }
    pub fn goal_category_is_normal(self) -> Self {
        self.filter_by_goal_category(1001_u64)
    }

    pub fn with_goal_category_is_normal(self) -> Self {
        self.filter_by_goal_category(1001_u64)
    }



    pub fn with_goal_category_is_not_normal(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("goal_category_id", 1001_u64));
        self
    }


    pub fn goal_category_is_penalty(self) -> Self {
        self.filter_by_goal_category(1002_u64)
    }

    pub fn with_goal_category_is_penalty(self) -> Self {
        self.filter_by_goal_category(1002_u64)
    }



    pub fn with_goal_category_is_not_penalty(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("goal_category_id", 1002_u64));
        self
    }


    pub fn goal_category_is_own_goal(self) -> Self {
        self.filter_by_goal_category(1003_u64)
    }

    pub fn with_goal_category_is_own_goal(self) -> Self {
        self.filter_by_goal_category(1003_u64)
    }



    pub fn with_goal_category_is_not_own_goal(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("goal_category_id", 1003_u64));
        self
    }


    pub fn goal_category_is_free_kick(self) -> Self {
        self.filter_by_goal_category(1004_u64)
    }

    pub fn with_goal_category_is_free_kick(self) -> Self {
        self.filter_by_goal_category(1004_u64)
    }



    pub fn with_goal_category_is_not_free_kick(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("goal_category_id", 1004_u64));
        self
    }




    pub fn select_tournament_match(mut self) -> Self {
        self.query = self.query.relation("tournament_match");
        self
    }

    pub fn select_tournament_match_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("tournament_match", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("tournament_match", selection));
        self
}

    pub fn facet_by_tournament_match_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_tournament_match_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_tournament_match_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "tournament_match",
            request,
            include_all_facets,
        ));
        self
    }

    pub fn select_tournament_team(mut self) -> Self {
        self.query = self.query.relation("tournament_team");
        self
    }

    pub fn select_tournament_team_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("tournament_team", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("tournament_team", selection));
        self
}

    pub fn facet_by_tournament_team_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_tournament_team_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_tournament_team_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "tournament_team",
            request,
            include_all_facets,
        ));
        self
    }

    pub fn select_goal_category(mut self) -> Self {
        self.query = self.query.relation("goal_category");
        self
    }

    pub fn select_goal_category_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("goal_category", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("goal_category", selection));
        self
}

    pub fn facet_by_goal_category_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_goal_category_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_goal_category_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "goal_category",
            request,
            include_all_facets,
        ));
        self
    }

    pub fn select_tournament(mut self) -> Self {
        self.query = self.query.relation("tournament");
        self
    }

    pub fn select_tournament_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("tournament", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("tournament", selection));
        self
}

    pub fn facet_by_tournament_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_tournament_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_tournament_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "tournament",
            request,
            include_all_facets,
        ));
        self
    }
}

impl<R> Default for MatchGoalRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< MatchGoalRequest<R> > for SelectQuery {
    fn from(request: MatchGoalRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< MatchGoalRequest<R> > for QuerySelection {
    fn from(request: MatchGoalRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::MatchGoal> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlRepositoryError<C::MatchGoalRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<MatchGoalRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::MatchGoal
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::MatchGoal::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> MatchGoalRequest<R> {
        self.inner.query.trace_chain.push(teaql_core::TraceNode {
            entity_type: self.inner.query.entity.clone(),
            entity_id: None,
            comment: self.purpose,
        });
        self.inner
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlRepositoryError<C::MatchGoalRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
