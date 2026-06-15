use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{RepositoryError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::TournamentMatch {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::TournamentMatch {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

#[derive(Debug)]
pub struct TournamentMatchRequest<R = crate::TournamentMatch> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for TournamentMatchRequest<R> {
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

impl<R> TournamentMatchRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("TournamentMatch"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> TournamentMatchRequest<T> {
        TournamentMatchRequest {
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
    ) -> Result<SmartList<R>, TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .tournament_match_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .tournament_match_repository()
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
    ) -> Result<Option<R>, TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
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
    ) -> Result<u64, TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .tournament_match_repository()
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
            .ok_or_else(|| RepositoryError::Runtime(RuntimeError::Graph(format!("count result for TournamentMatch is missing or not numeric"))))
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .tournament_match_repository()
            .map_err(|err| RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .tournament_match_repository()
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
    ) -> Result<Option<Record>, TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
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
            "match_number" => Some("match_number"),
            "match_date" => Some("match_date"),
            "venue_name" => Some("venue_name"),
            "venue_city" => Some("venue_city"),
            "venue_country" => Some("venue_country"),
            "home_score" => Some("home_score"),
            "away_score" => Some("away_score"),
            "extra_time_home" => Some("extra_time_home"),
            "extra_time_away" => Some("extra_time_away"),
            "penalty_home" => Some("penalty_home"),
            "penalty_away" => Some("penalty_away"),
            "create_time" => Some("create_time"),
            "update_time" => Some("update_time"),
            "version" => Some("version"),
            "home_team" | "home_team_id" => Some("home_team_id"),
            "away_team" | "away_team_id" => Some("away_team_id"),
            "match_stage" | "match_stage_id" => Some("match_stage_id"),
            "match_group" | "match_group_id" => Some("match_group_id"),
            "match_status" | "match_status_id" => Some("match_status_id"),
            "tournament" | "tournament_id" => Some("tournament_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "home_team" => {
                self.with_home_team_matching(
                    crate::Q::tournament_teams_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "away_team" => {
                self.with_away_team_matching(
                    crate::Q::tournament_teams_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "match_stage" => {
                self.with_match_stage_matching(
                    crate::Q::match_stages_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "match_group" => {
                self.with_match_group_matching(
                    crate::Q::match_groups_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "match_status" => {
                self.with_match_status_matching(
                    crate::Q::match_statuses_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "tournament" => {
                self.with_tournament_matching(
                    crate::Q::tournaments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "match_goal_list" => {
                self.with_match_goal_list_matching(
                    crate::Q::match_goals_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "match_card_list" => {
                self.with_match_card_list_matching(
                    crate::Q::match_cards_minimal()
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
        self.query = self.query.project("match_number");
        self.query = self.query.project("match_date");
        self.query = self.query.project("venue_name");
        self.query = self.query.project("venue_city");
        self.query = self.query.project("venue_country");
        self.query = self.query.project("home_score");
        self.query = self.query.project("away_score");
        self.query = self.query.project("extra_time_home");
        self.query = self.query.project("extra_time_away");
        self.query = self.query.project("penalty_home");
        self.query = self.query.project("penalty_away");
        self.query = self.query.project("create_time");
        self.query = self.query.project("update_time");
        self.query = self.query.project("version");
        self.query = self.query.project("home_team_id");
        self.query = self.query.project("away_team_id");
        self.query = self.query.project("match_stage_id");
        self.query = self.query.project("match_group_id");
        self.query = self.query.project("match_status_id");
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
        request = request.select_home_team();
        request = request.select_away_team();
        request = request.select_match_stage();
        request = request.select_match_group();
        request = request.select_match_status();
        request = request.select_tournament();
        request
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_match_goal_list();
        request = request.select_match_card_list();
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

    pub fn select_match_number(mut self) -> Self {
        self.query = self.query.project("match_number");
        self
    }

    pub fn project_match_number(self) -> Self {
        self.select_match_number()
    }

    pub fn select_match_number_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_match_number_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_match_number_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("match_number", raw_sql_segment));
        self
    }

    pub fn select_match_number_with_function(self, function: AggregateFunction) -> Self {
        self.select_match_number_as_with_function("match_number", function)
    }

    pub fn select_match_number_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("match_number", alias, function)
    }

    pub fn group_by_match_number(self) -> Self {
        self.group_by("match_number")
    }

    pub fn group_by_match_number_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("match_number");
        request.query = request
            .query
            .project_expr(alias, Expr::column("match_number"));
        request
    }

    pub fn group_by_match_number_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("match_number")
            .aggregate_with_function("match_number", alias, function)
    }

    pub fn count_match_number(self) -> Self {
        self.count_match_number_as("match_number_count")
    }

    pub fn count_match_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("match_number", alias)
    }

    pub fn sum_match_number(self) -> Self {
        self.sum_match_number_as("sum_match_number")
    }

    pub fn sum_match_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("match_number", alias)
    }

    pub fn avg_match_number(self) -> Self {
        self.avg_match_number_as("avg_match_number")
    }

    pub fn avg_match_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("match_number", alias)
    }

    pub fn min_match_number(self) -> Self {
        self.min_match_number_as("min_match_number")
    }

    pub fn min_match_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("match_number", alias)
    }

    pub fn max_match_number(self) -> Self {
        self.max_match_number_as("max_match_number")
    }

    pub fn max_match_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("match_number", alias)
    }

    pub fn standard_deviation_match_number(self) -> Self {
        self.standard_deviation_match_number_as("stdDev_match_number")
    }

    pub fn standard_deviation_match_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("match_number", alias)
    }

    pub fn square_root_of_population_standard_deviation_match_number(self) -> Self {
        self.square_root_of_population_standard_deviation_match_number_as("stdDevPop_match_number")
    }

    pub fn square_root_of_population_standard_deviation_match_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("match_number", alias)
    }

    pub fn sample_variance_match_number(self) -> Self {
        self.sample_variance_match_number_as("varSamp_match_number")
    }

    pub fn sample_variance_match_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("match_number", alias)
    }

    pub fn sample_population_variance_match_number(self) -> Self {
        self.sample_population_variance_match_number_as("varPop_match_number")
    }

    pub fn sample_population_variance_match_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("match_number", alias)
    }

    pub fn unselect_match_number(mut self) -> Self {
        self.query.projection.retain(|field| field != "match_number");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "match_number");
        self
    }


    pub fn with_match_number(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "match_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_match_number_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "match_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_match_number_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("match_number", value));
        self
    }



    pub fn with_match_number_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("match_number", value));
        self
    }

    pub fn with_match_number_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("match_number", value));
        self
    }

    pub fn with_match_number_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("match_number", value));
        self
    }

    pub fn with_match_number_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("match_number", value));
        self
    }

    pub fn with_match_number_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("match_number", value));
        self
    }

    pub fn with_match_number_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("match_number", lower, upper));
        self
    }

    pub fn with_match_number_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "match_number",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_match_number_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "match_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_match_number_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "match_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_match_number_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("match_number", value));
        self
    }

    pub fn with_match_number_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("match_number", value));
        self
    }

    pub fn with_match_number_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("match_number"));
        self
    }



    pub fn with_match_number_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("match_number"));
        self
    }


    pub fn order_by_match_number_asc(mut self) -> Self {
        self.query = self.query.order_asc("match_number");
        self
    }

    pub fn order_by_match_number_desc(mut self) -> Self {
        self.query = self.query.order_desc("match_number");
        self
    }

    pub fn order_by_match_number_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("match_number");
        self
    }

    pub fn order_by_match_number_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("match_number");
        self
    }

    pub fn select_match_date(mut self) -> Self {
        self.query = self.query.project("match_date");
        self
    }

    pub fn project_match_date(self) -> Self {
        self.select_match_date()
    }

    pub fn select_match_date_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_match_date_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_match_date_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("match_date", raw_sql_segment));
        self
    }

    pub fn group_by_match_date(self) -> Self {
        self.group_by("match_date")
    }

    pub fn group_by_match_date_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("match_date");
        request.query = request
            .query
            .project_expr(alias, Expr::column("match_date"));
        request
    }

    pub fn group_by_match_date_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("match_date")
            .aggregate_with_function("match_date", alias, function)
    }

    pub fn count_match_date(self) -> Self {
        self.count_match_date_as("match_date_count")
    }

    pub fn count_match_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("match_date", alias)
    }

    pub fn sum_match_date(self) -> Self {
        self.sum_match_date_as("sum_match_date")
    }

    pub fn sum_match_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("match_date", alias)
    }

    pub fn avg_match_date(self) -> Self {
        self.avg_match_date_as("avg_match_date")
    }

    pub fn avg_match_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("match_date", alias)
    }

    pub fn min_match_date(self) -> Self {
        self.min_match_date_as("min_match_date")
    }

    pub fn min_match_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("match_date", alias)
    }

    pub fn max_match_date(self) -> Self {
        self.max_match_date_as("max_match_date")
    }

    pub fn max_match_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("match_date", alias)
    }

    pub fn unselect_match_date(mut self) -> Self {
        self.query.projection.retain(|field| field != "match_date");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "match_date");
        self
    }


    pub fn with_match_date(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "match_date",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_match_date_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "match_date",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_match_date_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("match_date", value));
        self
    }



    pub fn with_match_date_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("match_date", value));
        self
    }

    pub fn with_match_date_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("match_date", value));
        self
    }

    pub fn with_match_date_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("match_date", value));
        self
    }

    pub fn with_match_date_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("match_date", value));
        self
    }

    pub fn with_match_date_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("match_date", value));
        self
    }

    pub fn with_match_date_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("match_date", lower, upper));
        self
    }

    pub fn with_match_date_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "match_date",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_match_date_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "match_date",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_match_date_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "match_date",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_match_date_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("match_date", value));
        self
    }

    pub fn with_match_date_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("match_date", value));
        self
    }

    pub fn with_match_date_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("match_date"));
        self
    }



    pub fn with_match_date_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("match_date"));
        self
    }


    pub fn order_by_match_date_asc(mut self) -> Self {
        self.query = self.query.order_asc("match_date");
        self
    }

    pub fn order_by_match_date_desc(mut self) -> Self {
        self.query = self.query.order_desc("match_date");
        self
    }

    pub fn order_by_match_date_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("match_date");
        self
    }

    pub fn order_by_match_date_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("match_date");
        self
    }

    pub fn select_venue_name(mut self) -> Self {
        self.query = self.query.project("venue_name");
        self
    }

    pub fn project_venue_name(self) -> Self {
        self.select_venue_name()
    }

    pub fn select_venue_name_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_venue_name_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_venue_name_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("venue_name", raw_sql_segment));
        self
    }

    pub fn group_by_venue_name(self) -> Self {
        self.group_by("venue_name")
    }

    pub fn group_by_venue_name_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("venue_name");
        request.query = request
            .query
            .project_expr(alias, Expr::column("venue_name"));
        request
    }

    pub fn group_by_venue_name_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("venue_name")
            .aggregate_with_function("venue_name", alias, function)
    }

    pub fn count_venue_name(self) -> Self {
        self.count_venue_name_as("venue_name_count")
    }

    pub fn count_venue_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("venue_name", alias)
    }

    pub fn sum_venue_name(self) -> Self {
        self.sum_venue_name_as("sum_venue_name")
    }

    pub fn sum_venue_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("venue_name", alias)
    }

    pub fn avg_venue_name(self) -> Self {
        self.avg_venue_name_as("avg_venue_name")
    }

    pub fn avg_venue_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("venue_name", alias)
    }

    pub fn min_venue_name(self) -> Self {
        self.min_venue_name_as("min_venue_name")
    }

    pub fn min_venue_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("venue_name", alias)
    }

    pub fn max_venue_name(self) -> Self {
        self.max_venue_name_as("max_venue_name")
    }

    pub fn max_venue_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("venue_name", alias)
    }

    pub fn unselect_venue_name(mut self) -> Self {
        self.query.projection.retain(|field| field != "venue_name");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "venue_name");
        self
    }


    pub fn with_venue_name(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "venue_name",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_venue_name_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "venue_name",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_venue_name_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("venue_name", value));
        self
    }



    pub fn with_venue_name_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("venue_name", value));
        self
    }

    pub fn with_venue_name_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("venue_name", value));
        self
    }

    pub fn with_venue_name_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("venue_name", value));
        self
    }

    pub fn with_venue_name_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("venue_name", value));
        self
    }

    pub fn with_venue_name_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("venue_name", value));
        self
    }

    pub fn with_venue_name_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("venue_name", lower, upper));
        self
    }

    pub fn with_venue_name_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "venue_name",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_venue_name_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "venue_name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_venue_name_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "venue_name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_venue_name_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("venue_name", value));
        self
    }

    pub fn with_venue_name_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("venue_name", value));
        self
    }

    pub fn with_venue_name_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("venue_name", value));
        self
    }

    pub fn with_venue_name_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("venue_name", value));
        self
    }

    pub fn with_venue_name_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("venue_name", value));
        self
    }

    pub fn with_venue_name_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("venue_name", value));
        self
    }

    pub fn with_venue_name_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("venue_name", value));
        self
    }
    pub fn with_venue_name_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("venue_name", value));
        self
    }

    pub fn with_venue_name_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("venue_name", value));
        self
    }

    pub fn with_venue_name_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("venue_name"));
        self
    }



    pub fn with_venue_name_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("venue_name"));
        self
    }


    pub fn order_by_venue_name_asc(mut self) -> Self {
        self.query = self.query.order_asc("venue_name");
        self
    }

    pub fn order_by_venue_name_desc(mut self) -> Self {
        self.query = self.query.order_desc("venue_name");
        self
    }

    pub fn order_by_venue_name_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("venue_name");
        self
    }

    pub fn order_by_venue_name_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("venue_name");
        self
    }

    pub fn select_venue_city(mut self) -> Self {
        self.query = self.query.project("venue_city");
        self
    }

    pub fn project_venue_city(self) -> Self {
        self.select_venue_city()
    }

    pub fn select_venue_city_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_venue_city_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_venue_city_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("venue_city", raw_sql_segment));
        self
    }

    pub fn group_by_venue_city(self) -> Self {
        self.group_by("venue_city")
    }

    pub fn group_by_venue_city_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("venue_city");
        request.query = request
            .query
            .project_expr(alias, Expr::column("venue_city"));
        request
    }

    pub fn group_by_venue_city_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("venue_city")
            .aggregate_with_function("venue_city", alias, function)
    }

    pub fn count_venue_city(self) -> Self {
        self.count_venue_city_as("venue_city_count")
    }

    pub fn count_venue_city_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("venue_city", alias)
    }

    pub fn sum_venue_city(self) -> Self {
        self.sum_venue_city_as("sum_venue_city")
    }

    pub fn sum_venue_city_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("venue_city", alias)
    }

    pub fn avg_venue_city(self) -> Self {
        self.avg_venue_city_as("avg_venue_city")
    }

    pub fn avg_venue_city_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("venue_city", alias)
    }

    pub fn min_venue_city(self) -> Self {
        self.min_venue_city_as("min_venue_city")
    }

    pub fn min_venue_city_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("venue_city", alias)
    }

    pub fn max_venue_city(self) -> Self {
        self.max_venue_city_as("max_venue_city")
    }

    pub fn max_venue_city_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("venue_city", alias)
    }

    pub fn unselect_venue_city(mut self) -> Self {
        self.query.projection.retain(|field| field != "venue_city");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "venue_city");
        self
    }


    pub fn with_venue_city(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "venue_city",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_venue_city_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "venue_city",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_venue_city_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("venue_city", value));
        self
    }



    pub fn with_venue_city_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("venue_city", value));
        self
    }

    pub fn with_venue_city_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("venue_city", value));
        self
    }

    pub fn with_venue_city_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("venue_city", value));
        self
    }

    pub fn with_venue_city_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("venue_city", value));
        self
    }

    pub fn with_venue_city_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("venue_city", value));
        self
    }

    pub fn with_venue_city_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("venue_city", lower, upper));
        self
    }

    pub fn with_venue_city_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "venue_city",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_venue_city_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "venue_city",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_venue_city_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "venue_city",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_venue_city_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("venue_city", value));
        self
    }

    pub fn with_venue_city_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("venue_city", value));
        self
    }

    pub fn with_venue_city_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("venue_city", value));
        self
    }

    pub fn with_venue_city_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("venue_city", value));
        self
    }

    pub fn with_venue_city_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("venue_city", value));
        self
    }

    pub fn with_venue_city_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("venue_city", value));
        self
    }

    pub fn with_venue_city_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("venue_city", value));
        self
    }
    pub fn with_venue_city_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("venue_city", value));
        self
    }

    pub fn with_venue_city_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("venue_city", value));
        self
    }

    pub fn with_venue_city_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("venue_city"));
        self
    }



    pub fn with_venue_city_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("venue_city"));
        self
    }


    pub fn order_by_venue_city_asc(mut self) -> Self {
        self.query = self.query.order_asc("venue_city");
        self
    }

    pub fn order_by_venue_city_desc(mut self) -> Self {
        self.query = self.query.order_desc("venue_city");
        self
    }

    pub fn order_by_venue_city_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("venue_city");
        self
    }

    pub fn order_by_venue_city_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("venue_city");
        self
    }

    pub fn select_venue_country(mut self) -> Self {
        self.query = self.query.project("venue_country");
        self
    }

    pub fn project_venue_country(self) -> Self {
        self.select_venue_country()
    }

    pub fn select_venue_country_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_venue_country_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_venue_country_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("venue_country", raw_sql_segment));
        self
    }

    pub fn group_by_venue_country(self) -> Self {
        self.group_by("venue_country")
    }

    pub fn group_by_venue_country_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("venue_country");
        request.query = request
            .query
            .project_expr(alias, Expr::column("venue_country"));
        request
    }

    pub fn group_by_venue_country_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("venue_country")
            .aggregate_with_function("venue_country", alias, function)
    }

    pub fn count_venue_country(self) -> Self {
        self.count_venue_country_as("venue_country_count")
    }

    pub fn count_venue_country_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("venue_country", alias)
    }

    pub fn sum_venue_country(self) -> Self {
        self.sum_venue_country_as("sum_venue_country")
    }

    pub fn sum_venue_country_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("venue_country", alias)
    }

    pub fn avg_venue_country(self) -> Self {
        self.avg_venue_country_as("avg_venue_country")
    }

    pub fn avg_venue_country_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("venue_country", alias)
    }

    pub fn min_venue_country(self) -> Self {
        self.min_venue_country_as("min_venue_country")
    }

    pub fn min_venue_country_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("venue_country", alias)
    }

    pub fn max_venue_country(self) -> Self {
        self.max_venue_country_as("max_venue_country")
    }

    pub fn max_venue_country_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("venue_country", alias)
    }

    pub fn unselect_venue_country(mut self) -> Self {
        self.query.projection.retain(|field| field != "venue_country");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "venue_country");
        self
    }


    pub fn with_venue_country(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "venue_country",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_venue_country_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "venue_country",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_venue_country_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("venue_country", value));
        self
    }



    pub fn with_venue_country_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("venue_country", value));
        self
    }

    pub fn with_venue_country_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("venue_country", value));
        self
    }

    pub fn with_venue_country_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("venue_country", value));
        self
    }

    pub fn with_venue_country_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("venue_country", value));
        self
    }

    pub fn with_venue_country_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("venue_country", value));
        self
    }

    pub fn with_venue_country_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("venue_country", lower, upper));
        self
    }

    pub fn with_venue_country_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "venue_country",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_venue_country_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "venue_country",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_venue_country_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "venue_country",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_venue_country_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("venue_country", value));
        self
    }

    pub fn with_venue_country_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("venue_country", value));
        self
    }

    pub fn with_venue_country_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("venue_country", value));
        self
    }

    pub fn with_venue_country_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("venue_country", value));
        self
    }

    pub fn with_venue_country_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("venue_country", value));
        self
    }

    pub fn with_venue_country_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("venue_country", value));
        self
    }

    pub fn with_venue_country_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("venue_country", value));
        self
    }
    pub fn with_venue_country_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("venue_country", value));
        self
    }

    pub fn with_venue_country_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("venue_country", value));
        self
    }

    pub fn with_venue_country_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("venue_country"));
        self
    }



    pub fn with_venue_country_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("venue_country"));
        self
    }


    pub fn order_by_venue_country_asc(mut self) -> Self {
        self.query = self.query.order_asc("venue_country");
        self
    }

    pub fn order_by_venue_country_desc(mut self) -> Self {
        self.query = self.query.order_desc("venue_country");
        self
    }

    pub fn order_by_venue_country_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("venue_country");
        self
    }

    pub fn order_by_venue_country_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("venue_country");
        self
    }

    pub fn select_home_score(mut self) -> Self {
        self.query = self.query.project("home_score");
        self
    }

    pub fn project_home_score(self) -> Self {
        self.select_home_score()
    }

    pub fn select_home_score_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_home_score_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_home_score_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("home_score", raw_sql_segment));
        self
    }

    pub fn select_home_score_with_function(self, function: AggregateFunction) -> Self {
        self.select_home_score_as_with_function("home_score", function)
    }

    pub fn select_home_score_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("home_score", alias, function)
    }

    pub fn group_by_home_score(self) -> Self {
        self.group_by("home_score")
    }

    pub fn group_by_home_score_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("home_score");
        request.query = request
            .query
            .project_expr(alias, Expr::column("home_score"));
        request
    }

    pub fn group_by_home_score_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("home_score")
            .aggregate_with_function("home_score", alias, function)
    }

    pub fn count_home_score(self) -> Self {
        self.count_home_score_as("home_score_count")
    }

    pub fn count_home_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("home_score", alias)
    }

    pub fn sum_home_score(self) -> Self {
        self.sum_home_score_as("sum_home_score")
    }

    pub fn sum_home_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("home_score", alias)
    }

    pub fn avg_home_score(self) -> Self {
        self.avg_home_score_as("avg_home_score")
    }

    pub fn avg_home_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("home_score", alias)
    }

    pub fn min_home_score(self) -> Self {
        self.min_home_score_as("min_home_score")
    }

    pub fn min_home_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("home_score", alias)
    }

    pub fn max_home_score(self) -> Self {
        self.max_home_score_as("max_home_score")
    }

    pub fn max_home_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("home_score", alias)
    }

    pub fn standard_deviation_home_score(self) -> Self {
        self.standard_deviation_home_score_as("stdDev_home_score")
    }

    pub fn standard_deviation_home_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("home_score", alias)
    }

    pub fn square_root_of_population_standard_deviation_home_score(self) -> Self {
        self.square_root_of_population_standard_deviation_home_score_as("stdDevPop_home_score")
    }

    pub fn square_root_of_population_standard_deviation_home_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("home_score", alias)
    }

    pub fn sample_variance_home_score(self) -> Self {
        self.sample_variance_home_score_as("varSamp_home_score")
    }

    pub fn sample_variance_home_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("home_score", alias)
    }

    pub fn sample_population_variance_home_score(self) -> Self {
        self.sample_population_variance_home_score_as("varPop_home_score")
    }

    pub fn sample_population_variance_home_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("home_score", alias)
    }

    pub fn unselect_home_score(mut self) -> Self {
        self.query.projection.retain(|field| field != "home_score");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "home_score");
        self
    }


    pub fn with_home_score(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "home_score",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_home_score_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "home_score",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_home_score_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("home_score", value));
        self
    }



    pub fn with_home_score_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("home_score", value));
        self
    }

    pub fn with_home_score_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("home_score", value));
        self
    }

    pub fn with_home_score_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("home_score", value));
        self
    }

    pub fn with_home_score_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("home_score", value));
        self
    }

    pub fn with_home_score_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("home_score", value));
        self
    }

    pub fn with_home_score_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("home_score", lower, upper));
        self
    }

    pub fn with_home_score_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "home_score",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_home_score_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "home_score",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_home_score_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "home_score",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_home_score_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("home_score", value));
        self
    }

    pub fn with_home_score_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("home_score", value));
        self
    }

    pub fn with_home_score_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("home_score"));
        self
    }



    pub fn with_home_score_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("home_score"));
        self
    }


    pub fn order_by_home_score_asc(mut self) -> Self {
        self.query = self.query.order_asc("home_score");
        self
    }

    pub fn order_by_home_score_desc(mut self) -> Self {
        self.query = self.query.order_desc("home_score");
        self
    }

    pub fn order_by_home_score_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("home_score");
        self
    }

    pub fn order_by_home_score_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("home_score");
        self
    }

    pub fn select_away_score(mut self) -> Self {
        self.query = self.query.project("away_score");
        self
    }

    pub fn project_away_score(self) -> Self {
        self.select_away_score()
    }

    pub fn select_away_score_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_away_score_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_away_score_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("away_score", raw_sql_segment));
        self
    }

    pub fn select_away_score_with_function(self, function: AggregateFunction) -> Self {
        self.select_away_score_as_with_function("away_score", function)
    }

    pub fn select_away_score_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("away_score", alias, function)
    }

    pub fn group_by_away_score(self) -> Self {
        self.group_by("away_score")
    }

    pub fn group_by_away_score_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("away_score");
        request.query = request
            .query
            .project_expr(alias, Expr::column("away_score"));
        request
    }

    pub fn group_by_away_score_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("away_score")
            .aggregate_with_function("away_score", alias, function)
    }

    pub fn count_away_score(self) -> Self {
        self.count_away_score_as("away_score_count")
    }

    pub fn count_away_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("away_score", alias)
    }

    pub fn sum_away_score(self) -> Self {
        self.sum_away_score_as("sum_away_score")
    }

    pub fn sum_away_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("away_score", alias)
    }

    pub fn avg_away_score(self) -> Self {
        self.avg_away_score_as("avg_away_score")
    }

    pub fn avg_away_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("away_score", alias)
    }

    pub fn min_away_score(self) -> Self {
        self.min_away_score_as("min_away_score")
    }

    pub fn min_away_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("away_score", alias)
    }

    pub fn max_away_score(self) -> Self {
        self.max_away_score_as("max_away_score")
    }

    pub fn max_away_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("away_score", alias)
    }

    pub fn standard_deviation_away_score(self) -> Self {
        self.standard_deviation_away_score_as("stdDev_away_score")
    }

    pub fn standard_deviation_away_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("away_score", alias)
    }

    pub fn square_root_of_population_standard_deviation_away_score(self) -> Self {
        self.square_root_of_population_standard_deviation_away_score_as("stdDevPop_away_score")
    }

    pub fn square_root_of_population_standard_deviation_away_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("away_score", alias)
    }

    pub fn sample_variance_away_score(self) -> Self {
        self.sample_variance_away_score_as("varSamp_away_score")
    }

    pub fn sample_variance_away_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("away_score", alias)
    }

    pub fn sample_population_variance_away_score(self) -> Self {
        self.sample_population_variance_away_score_as("varPop_away_score")
    }

    pub fn sample_population_variance_away_score_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("away_score", alias)
    }

    pub fn unselect_away_score(mut self) -> Self {
        self.query.projection.retain(|field| field != "away_score");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "away_score");
        self
    }


    pub fn with_away_score(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "away_score",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_away_score_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "away_score",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_away_score_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("away_score", value));
        self
    }



    pub fn with_away_score_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("away_score", value));
        self
    }

    pub fn with_away_score_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("away_score", value));
        self
    }

    pub fn with_away_score_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("away_score", value));
        self
    }

    pub fn with_away_score_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("away_score", value));
        self
    }

    pub fn with_away_score_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("away_score", value));
        self
    }

    pub fn with_away_score_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("away_score", lower, upper));
        self
    }

    pub fn with_away_score_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "away_score",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_away_score_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "away_score",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_away_score_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "away_score",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_away_score_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("away_score", value));
        self
    }

    pub fn with_away_score_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("away_score", value));
        self
    }

    pub fn with_away_score_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("away_score"));
        self
    }



    pub fn with_away_score_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("away_score"));
        self
    }


    pub fn order_by_away_score_asc(mut self) -> Self {
        self.query = self.query.order_asc("away_score");
        self
    }

    pub fn order_by_away_score_desc(mut self) -> Self {
        self.query = self.query.order_desc("away_score");
        self
    }

    pub fn order_by_away_score_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("away_score");
        self
    }

    pub fn order_by_away_score_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("away_score");
        self
    }

    pub fn select_extra_time_home(mut self) -> Self {
        self.query = self.query.project("extra_time_home");
        self
    }

    pub fn project_extra_time_home(self) -> Self {
        self.select_extra_time_home()
    }

    pub fn select_extra_time_home_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_extra_time_home_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_extra_time_home_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("extra_time_home", raw_sql_segment));
        self
    }

    pub fn select_extra_time_home_with_function(self, function: AggregateFunction) -> Self {
        self.select_extra_time_home_as_with_function("extra_time_home", function)
    }

    pub fn select_extra_time_home_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("extra_time_home", alias, function)
    }

    pub fn group_by_extra_time_home(self) -> Self {
        self.group_by("extra_time_home")
    }

    pub fn group_by_extra_time_home_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("extra_time_home");
        request.query = request
            .query
            .project_expr(alias, Expr::column("extra_time_home"));
        request
    }

    pub fn group_by_extra_time_home_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("extra_time_home")
            .aggregate_with_function("extra_time_home", alias, function)
    }

    pub fn count_extra_time_home(self) -> Self {
        self.count_extra_time_home_as("extra_time_home_count")
    }

    pub fn count_extra_time_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("extra_time_home", alias)
    }

    pub fn sum_extra_time_home(self) -> Self {
        self.sum_extra_time_home_as("sum_extra_time_home")
    }

    pub fn sum_extra_time_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("extra_time_home", alias)
    }

    pub fn avg_extra_time_home(self) -> Self {
        self.avg_extra_time_home_as("avg_extra_time_home")
    }

    pub fn avg_extra_time_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("extra_time_home", alias)
    }

    pub fn min_extra_time_home(self) -> Self {
        self.min_extra_time_home_as("min_extra_time_home")
    }

    pub fn min_extra_time_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("extra_time_home", alias)
    }

    pub fn max_extra_time_home(self) -> Self {
        self.max_extra_time_home_as("max_extra_time_home")
    }

    pub fn max_extra_time_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("extra_time_home", alias)
    }

    pub fn standard_deviation_extra_time_home(self) -> Self {
        self.standard_deviation_extra_time_home_as("stdDev_extra_time_home")
    }

    pub fn standard_deviation_extra_time_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("extra_time_home", alias)
    }

    pub fn square_root_of_population_standard_deviation_extra_time_home(self) -> Self {
        self.square_root_of_population_standard_deviation_extra_time_home_as("stdDevPop_extra_time_home")
    }

    pub fn square_root_of_population_standard_deviation_extra_time_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("extra_time_home", alias)
    }

    pub fn sample_variance_extra_time_home(self) -> Self {
        self.sample_variance_extra_time_home_as("varSamp_extra_time_home")
    }

    pub fn sample_variance_extra_time_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("extra_time_home", alias)
    }

    pub fn sample_population_variance_extra_time_home(self) -> Self {
        self.sample_population_variance_extra_time_home_as("varPop_extra_time_home")
    }

    pub fn sample_population_variance_extra_time_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("extra_time_home", alias)
    }

    pub fn unselect_extra_time_home(mut self) -> Self {
        self.query.projection.retain(|field| field != "extra_time_home");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "extra_time_home");
        self
    }


    pub fn with_extra_time_home(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "extra_time_home",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_extra_time_home_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "extra_time_home",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_extra_time_home_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("extra_time_home", value));
        self
    }



    pub fn with_extra_time_home_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("extra_time_home", value));
        self
    }

    pub fn with_extra_time_home_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("extra_time_home", value));
        self
    }

    pub fn with_extra_time_home_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("extra_time_home", value));
        self
    }

    pub fn with_extra_time_home_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("extra_time_home", value));
        self
    }

    pub fn with_extra_time_home_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("extra_time_home", value));
        self
    }

    pub fn with_extra_time_home_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("extra_time_home", lower, upper));
        self
    }

    pub fn with_extra_time_home_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "extra_time_home",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_extra_time_home_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "extra_time_home",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_extra_time_home_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "extra_time_home",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_extra_time_home_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("extra_time_home", value));
        self
    }

    pub fn with_extra_time_home_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("extra_time_home", value));
        self
    }

    pub fn with_extra_time_home_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("extra_time_home"));
        self
    }



    pub fn with_extra_time_home_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("extra_time_home"));
        self
    }


    pub fn order_by_extra_time_home_asc(mut self) -> Self {
        self.query = self.query.order_asc("extra_time_home");
        self
    }

    pub fn order_by_extra_time_home_desc(mut self) -> Self {
        self.query = self.query.order_desc("extra_time_home");
        self
    }

    pub fn order_by_extra_time_home_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("extra_time_home");
        self
    }

    pub fn order_by_extra_time_home_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("extra_time_home");
        self
    }

    pub fn select_extra_time_away(mut self) -> Self {
        self.query = self.query.project("extra_time_away");
        self
    }

    pub fn project_extra_time_away(self) -> Self {
        self.select_extra_time_away()
    }

    pub fn select_extra_time_away_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_extra_time_away_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_extra_time_away_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("extra_time_away", raw_sql_segment));
        self
    }

    pub fn select_extra_time_away_with_function(self, function: AggregateFunction) -> Self {
        self.select_extra_time_away_as_with_function("extra_time_away", function)
    }

    pub fn select_extra_time_away_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("extra_time_away", alias, function)
    }

    pub fn group_by_extra_time_away(self) -> Self {
        self.group_by("extra_time_away")
    }

    pub fn group_by_extra_time_away_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("extra_time_away");
        request.query = request
            .query
            .project_expr(alias, Expr::column("extra_time_away"));
        request
    }

    pub fn group_by_extra_time_away_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("extra_time_away")
            .aggregate_with_function("extra_time_away", alias, function)
    }

    pub fn count_extra_time_away(self) -> Self {
        self.count_extra_time_away_as("extra_time_away_count")
    }

    pub fn count_extra_time_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("extra_time_away", alias)
    }

    pub fn sum_extra_time_away(self) -> Self {
        self.sum_extra_time_away_as("sum_extra_time_away")
    }

    pub fn sum_extra_time_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("extra_time_away", alias)
    }

    pub fn avg_extra_time_away(self) -> Self {
        self.avg_extra_time_away_as("avg_extra_time_away")
    }

    pub fn avg_extra_time_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("extra_time_away", alias)
    }

    pub fn min_extra_time_away(self) -> Self {
        self.min_extra_time_away_as("min_extra_time_away")
    }

    pub fn min_extra_time_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("extra_time_away", alias)
    }

    pub fn max_extra_time_away(self) -> Self {
        self.max_extra_time_away_as("max_extra_time_away")
    }

    pub fn max_extra_time_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("extra_time_away", alias)
    }

    pub fn standard_deviation_extra_time_away(self) -> Self {
        self.standard_deviation_extra_time_away_as("stdDev_extra_time_away")
    }

    pub fn standard_deviation_extra_time_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("extra_time_away", alias)
    }

    pub fn square_root_of_population_standard_deviation_extra_time_away(self) -> Self {
        self.square_root_of_population_standard_deviation_extra_time_away_as("stdDevPop_extra_time_away")
    }

    pub fn square_root_of_population_standard_deviation_extra_time_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("extra_time_away", alias)
    }

    pub fn sample_variance_extra_time_away(self) -> Self {
        self.sample_variance_extra_time_away_as("varSamp_extra_time_away")
    }

    pub fn sample_variance_extra_time_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("extra_time_away", alias)
    }

    pub fn sample_population_variance_extra_time_away(self) -> Self {
        self.sample_population_variance_extra_time_away_as("varPop_extra_time_away")
    }

    pub fn sample_population_variance_extra_time_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("extra_time_away", alias)
    }

    pub fn unselect_extra_time_away(mut self) -> Self {
        self.query.projection.retain(|field| field != "extra_time_away");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "extra_time_away");
        self
    }


    pub fn with_extra_time_away(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "extra_time_away",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_extra_time_away_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "extra_time_away",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_extra_time_away_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("extra_time_away", value));
        self
    }



    pub fn with_extra_time_away_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("extra_time_away", value));
        self
    }

    pub fn with_extra_time_away_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("extra_time_away", value));
        self
    }

    pub fn with_extra_time_away_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("extra_time_away", value));
        self
    }

    pub fn with_extra_time_away_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("extra_time_away", value));
        self
    }

    pub fn with_extra_time_away_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("extra_time_away", value));
        self
    }

    pub fn with_extra_time_away_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("extra_time_away", lower, upper));
        self
    }

    pub fn with_extra_time_away_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "extra_time_away",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_extra_time_away_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "extra_time_away",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_extra_time_away_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "extra_time_away",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_extra_time_away_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("extra_time_away", value));
        self
    }

    pub fn with_extra_time_away_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("extra_time_away", value));
        self
    }

    pub fn with_extra_time_away_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("extra_time_away"));
        self
    }



    pub fn with_extra_time_away_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("extra_time_away"));
        self
    }


    pub fn order_by_extra_time_away_asc(mut self) -> Self {
        self.query = self.query.order_asc("extra_time_away");
        self
    }

    pub fn order_by_extra_time_away_desc(mut self) -> Self {
        self.query = self.query.order_desc("extra_time_away");
        self
    }

    pub fn order_by_extra_time_away_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("extra_time_away");
        self
    }

    pub fn order_by_extra_time_away_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("extra_time_away");
        self
    }

    pub fn select_penalty_home(mut self) -> Self {
        self.query = self.query.project("penalty_home");
        self
    }

    pub fn project_penalty_home(self) -> Self {
        self.select_penalty_home()
    }

    pub fn select_penalty_home_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_penalty_home_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_penalty_home_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("penalty_home", raw_sql_segment));
        self
    }

    pub fn select_penalty_home_with_function(self, function: AggregateFunction) -> Self {
        self.select_penalty_home_as_with_function("penalty_home", function)
    }

    pub fn select_penalty_home_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("penalty_home", alias, function)
    }

    pub fn group_by_penalty_home(self) -> Self {
        self.group_by("penalty_home")
    }

    pub fn group_by_penalty_home_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("penalty_home");
        request.query = request
            .query
            .project_expr(alias, Expr::column("penalty_home"));
        request
    }

    pub fn group_by_penalty_home_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("penalty_home")
            .aggregate_with_function("penalty_home", alias, function)
    }

    pub fn count_penalty_home(self) -> Self {
        self.count_penalty_home_as("penalty_home_count")
    }

    pub fn count_penalty_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("penalty_home", alias)
    }

    pub fn sum_penalty_home(self) -> Self {
        self.sum_penalty_home_as("sum_penalty_home")
    }

    pub fn sum_penalty_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("penalty_home", alias)
    }

    pub fn avg_penalty_home(self) -> Self {
        self.avg_penalty_home_as("avg_penalty_home")
    }

    pub fn avg_penalty_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("penalty_home", alias)
    }

    pub fn min_penalty_home(self) -> Self {
        self.min_penalty_home_as("min_penalty_home")
    }

    pub fn min_penalty_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("penalty_home", alias)
    }

    pub fn max_penalty_home(self) -> Self {
        self.max_penalty_home_as("max_penalty_home")
    }

    pub fn max_penalty_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("penalty_home", alias)
    }

    pub fn standard_deviation_penalty_home(self) -> Self {
        self.standard_deviation_penalty_home_as("stdDev_penalty_home")
    }

    pub fn standard_deviation_penalty_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("penalty_home", alias)
    }

    pub fn square_root_of_population_standard_deviation_penalty_home(self) -> Self {
        self.square_root_of_population_standard_deviation_penalty_home_as("stdDevPop_penalty_home")
    }

    pub fn square_root_of_population_standard_deviation_penalty_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("penalty_home", alias)
    }

    pub fn sample_variance_penalty_home(self) -> Self {
        self.sample_variance_penalty_home_as("varSamp_penalty_home")
    }

    pub fn sample_variance_penalty_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("penalty_home", alias)
    }

    pub fn sample_population_variance_penalty_home(self) -> Self {
        self.sample_population_variance_penalty_home_as("varPop_penalty_home")
    }

    pub fn sample_population_variance_penalty_home_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("penalty_home", alias)
    }

    pub fn unselect_penalty_home(mut self) -> Self {
        self.query.projection.retain(|field| field != "penalty_home");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "penalty_home");
        self
    }


    pub fn with_penalty_home(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "penalty_home",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_penalty_home_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "penalty_home",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_penalty_home_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("penalty_home", value));
        self
    }



    pub fn with_penalty_home_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("penalty_home", value));
        self
    }

    pub fn with_penalty_home_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("penalty_home", value));
        self
    }

    pub fn with_penalty_home_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("penalty_home", value));
        self
    }

    pub fn with_penalty_home_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("penalty_home", value));
        self
    }

    pub fn with_penalty_home_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("penalty_home", value));
        self
    }

    pub fn with_penalty_home_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("penalty_home", lower, upper));
        self
    }

    pub fn with_penalty_home_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "penalty_home",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_penalty_home_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "penalty_home",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_penalty_home_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "penalty_home",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_penalty_home_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("penalty_home", value));
        self
    }

    pub fn with_penalty_home_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("penalty_home", value));
        self
    }

    pub fn with_penalty_home_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("penalty_home"));
        self
    }



    pub fn with_penalty_home_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("penalty_home"));
        self
    }


    pub fn order_by_penalty_home_asc(mut self) -> Self {
        self.query = self.query.order_asc("penalty_home");
        self
    }

    pub fn order_by_penalty_home_desc(mut self) -> Self {
        self.query = self.query.order_desc("penalty_home");
        self
    }

    pub fn order_by_penalty_home_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("penalty_home");
        self
    }

    pub fn order_by_penalty_home_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("penalty_home");
        self
    }

    pub fn select_penalty_away(mut self) -> Self {
        self.query = self.query.project("penalty_away");
        self
    }

    pub fn project_penalty_away(self) -> Self {
        self.select_penalty_away()
    }

    pub fn select_penalty_away_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_penalty_away_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_penalty_away_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("penalty_away", raw_sql_segment));
        self
    }

    pub fn select_penalty_away_with_function(self, function: AggregateFunction) -> Self {
        self.select_penalty_away_as_with_function("penalty_away", function)
    }

    pub fn select_penalty_away_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("penalty_away", alias, function)
    }

    pub fn group_by_penalty_away(self) -> Self {
        self.group_by("penalty_away")
    }

    pub fn group_by_penalty_away_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("penalty_away");
        request.query = request
            .query
            .project_expr(alias, Expr::column("penalty_away"));
        request
    }

    pub fn group_by_penalty_away_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("penalty_away")
            .aggregate_with_function("penalty_away", alias, function)
    }

    pub fn count_penalty_away(self) -> Self {
        self.count_penalty_away_as("penalty_away_count")
    }

    pub fn count_penalty_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("penalty_away", alias)
    }

    pub fn sum_penalty_away(self) -> Self {
        self.sum_penalty_away_as("sum_penalty_away")
    }

    pub fn sum_penalty_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("penalty_away", alias)
    }

    pub fn avg_penalty_away(self) -> Self {
        self.avg_penalty_away_as("avg_penalty_away")
    }

    pub fn avg_penalty_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("penalty_away", alias)
    }

    pub fn min_penalty_away(self) -> Self {
        self.min_penalty_away_as("min_penalty_away")
    }

    pub fn min_penalty_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("penalty_away", alias)
    }

    pub fn max_penalty_away(self) -> Self {
        self.max_penalty_away_as("max_penalty_away")
    }

    pub fn max_penalty_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("penalty_away", alias)
    }

    pub fn standard_deviation_penalty_away(self) -> Self {
        self.standard_deviation_penalty_away_as("stdDev_penalty_away")
    }

    pub fn standard_deviation_penalty_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("penalty_away", alias)
    }

    pub fn square_root_of_population_standard_deviation_penalty_away(self) -> Self {
        self.square_root_of_population_standard_deviation_penalty_away_as("stdDevPop_penalty_away")
    }

    pub fn square_root_of_population_standard_deviation_penalty_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("penalty_away", alias)
    }

    pub fn sample_variance_penalty_away(self) -> Self {
        self.sample_variance_penalty_away_as("varSamp_penalty_away")
    }

    pub fn sample_variance_penalty_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("penalty_away", alias)
    }

    pub fn sample_population_variance_penalty_away(self) -> Self {
        self.sample_population_variance_penalty_away_as("varPop_penalty_away")
    }

    pub fn sample_population_variance_penalty_away_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("penalty_away", alias)
    }

    pub fn unselect_penalty_away(mut self) -> Self {
        self.query.projection.retain(|field| field != "penalty_away");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "penalty_away");
        self
    }


    pub fn with_penalty_away(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "penalty_away",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_penalty_away_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "penalty_away",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_penalty_away_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("penalty_away", value));
        self
    }



    pub fn with_penalty_away_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("penalty_away", value));
        self
    }

    pub fn with_penalty_away_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("penalty_away", value));
        self
    }

    pub fn with_penalty_away_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("penalty_away", value));
        self
    }

    pub fn with_penalty_away_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("penalty_away", value));
        self
    }

    pub fn with_penalty_away_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("penalty_away", value));
        self
    }

    pub fn with_penalty_away_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("penalty_away", lower, upper));
        self
    }

    pub fn with_penalty_away_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "penalty_away",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_penalty_away_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "penalty_away",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_penalty_away_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "penalty_away",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_penalty_away_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("penalty_away", value));
        self
    }

    pub fn with_penalty_away_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("penalty_away", value));
        self
    }

    pub fn with_penalty_away_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("penalty_away"));
        self
    }



    pub fn with_penalty_away_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("penalty_away"));
        self
    }


    pub fn order_by_penalty_away_asc(mut self) -> Self {
        self.query = self.query.order_asc("penalty_away");
        self
    }

    pub fn order_by_penalty_away_desc(mut self) -> Self {
        self.query = self.query.order_desc("penalty_away");
        self
    }

    pub fn order_by_penalty_away_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("penalty_away");
        self
    }

    pub fn order_by_penalty_away_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("penalty_away");
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
    pub fn filter_by_home_team(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("home_team_id", value.entity_id_value()));
        self
    }

    pub fn with_home_team_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "home_team_id",
            <crate::TournamentTeam as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("home_team", selection));
        self
    }


    pub fn without_home_team_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "home_team_id",
            <crate::TournamentTeam as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("home_team", selection));
        self
    }


    pub fn have_home_team(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("home_team_id"));
        self
    }

    pub fn have_no_home_team(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("home_team_id"));
        self
    }


    pub fn group_by_home_team(self) -> Self {
        self.group_by("home_team_id")
    }

    pub fn group_by_home_team_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("home_team_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("home_team_id"));
        request
    }

    pub fn group_by_home_team_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("home_team_id")
            .aggregate_with_function("home_team_id", alias, function)
    }

    pub fn group_by_home_team_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("home_team_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "home_team",
            "home_team_id",
            request,
        ));
        self
    }

    pub fn group_by_home_team_with_details(self) -> Self {
        self.group_by_home_team_with_details_from(crate::Q::tournament_teams().unlimited())
    }

    pub fn group_by_home_team_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_home_team_with(request)
    }


    pub fn roll_up_to_home_team(self) -> Self {
        self.roll_up_to_home_team_with(crate::Q::tournament_teams().unlimited())
    }

    pub fn roll_up_to_home_team_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_home_team_matching(selection.clone())
            .group_by_home_team_with(selection)
    }

    pub fn count_home_team(self) -> Self {
        self.count_home_team_as("home_team_count")
    }

    pub fn count_home_team_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("home_team_id", alias)
    }

    pub fn unselect_home_team(mut self) -> Self {
        self.query.projection.retain(|field| field != "home_team_id");
        self.query.relations.retain(|relation| relation.name != "home_team");
        self
    }


    pub fn filter_by_away_team(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("away_team_id", value.entity_id_value()));
        self
    }

    pub fn with_away_team_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "away_team_id",
            <crate::TournamentTeam as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("away_team", selection));
        self
    }


    pub fn without_away_team_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "away_team_id",
            <crate::TournamentTeam as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("away_team", selection));
        self
    }


    pub fn have_away_team(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("away_team_id"));
        self
    }

    pub fn have_no_away_team(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("away_team_id"));
        self
    }


    pub fn group_by_away_team(self) -> Self {
        self.group_by("away_team_id")
    }

    pub fn group_by_away_team_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("away_team_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("away_team_id"));
        request
    }

    pub fn group_by_away_team_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("away_team_id")
            .aggregate_with_function("away_team_id", alias, function)
    }

    pub fn group_by_away_team_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("away_team_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "away_team",
            "away_team_id",
            request,
        ));
        self
    }

    pub fn group_by_away_team_with_details(self) -> Self {
        self.group_by_away_team_with_details_from(crate::Q::tournament_teams().unlimited())
    }

    pub fn group_by_away_team_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_away_team_with(request)
    }


    pub fn roll_up_to_away_team(self) -> Self {
        self.roll_up_to_away_team_with(crate::Q::tournament_teams().unlimited())
    }

    pub fn roll_up_to_away_team_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_away_team_matching(selection.clone())
            .group_by_away_team_with(selection)
    }

    pub fn count_away_team(self) -> Self {
        self.count_away_team_as("away_team_count")
    }

    pub fn count_away_team_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("away_team_id", alias)
    }

    pub fn unselect_away_team(mut self) -> Self {
        self.query.projection.retain(|field| field != "away_team_id");
        self.query.relations.retain(|relation| relation.name != "away_team");
        self
    }


    /// Please use `with_match_stage_is` instead
    pub(crate) fn filter_by_match_stage(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("match_stage_id", value.entity_id_value()));
        self
    }
    /// Complex relation filter for `match_stage`.
    ///
    /// **Usage Priority:**
    ///
    /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
    ///    - [`Self::with_match_stage_is_xxx`]
    ///
    ///    This gives the best code readability.
    ///
    /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
    ///
    /// # Example
    /// ```rust
    /// // Only use when building dynamic queries
    /// let dynamic_query = crate::Q::match_stages_minimal().filter(...);
    /// let request = crate::Q::tournament_matches().with_match_stage_matching(dynamic_query);
    /// ```
    pub fn with_match_stage_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "match_stage_id",
            <crate::MatchStage as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("match_stage", selection));
        self
    }


    /// Complex relation filter for `match_stage`.
    ///
    /// **Usage Priority:**
    ///
    /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
    ///    - [`Self::with_match_stage_is_not_xxx`]
    ///
    ///    This gives the best code readability.
    ///
    /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
    ///
    /// # Example
    /// ```rust
    /// // Only use when building dynamic queries
    /// let dynamic_query = crate::Q::match_stages_minimal().filter(...);
    /// let request = crate::Q::tournament_matches().without_match_stage_matching(dynamic_query);
    /// ```
    pub fn without_match_stage_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "match_stage_id",
            <crate::MatchStage as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("match_stage", selection));
        self
    }


    pub fn have_match_stage(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("match_stage_id"));
        self
    }

    pub fn have_no_match_stage(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("match_stage_id"));
        self
    }


    pub fn group_by_match_stage(self) -> Self {
        self.group_by("match_stage_id")
    }

    pub fn group_by_match_stage_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("match_stage_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("match_stage_id"));
        request
    }

    pub fn group_by_match_stage_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("match_stage_id")
            .aggregate_with_function("match_stage_id", alias, function)
    }

    pub fn group_by_match_stage_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("match_stage_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "match_stage",
            "match_stage_id",
            request,
        ));
        self
    }

    pub fn group_by_match_stage_with_details(self) -> Self {
        self.group_by_match_stage_with_details_from(crate::Q::match_stages().unlimited())
    }

    pub fn group_by_match_stage_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_match_stage_with(request)
    }


    pub fn roll_up_to_match_stage(self) -> Self {
        self.roll_up_to_match_stage_with(crate::Q::match_stages().unlimited())
    }

    pub fn roll_up_to_match_stage_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_match_stage_matching(selection.clone())
            .group_by_match_stage_with(selection)
    }

    pub fn count_match_stage(self) -> Self {
        self.count_match_stage_as("match_stage_count")
    }

    pub fn count_match_stage_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("match_stage_id", alias)
    }

    pub fn unselect_match_stage(mut self) -> Self {
        self.query.projection.retain(|field| field != "match_stage_id");
        self.query.relations.retain(|relation| relation.name != "match_stage");
        self
    }


    pub fn filter_by_match_group(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("match_group_id", value.entity_id_value()));
        self
    }

    pub fn with_match_group_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "match_group_id",
            <crate::MatchGroup as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("match_group", selection));
        self
    }


    pub fn without_match_group_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "match_group_id",
            <crate::MatchGroup as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("match_group", selection));
        self
    }


    pub fn have_match_group(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("match_group_id"));
        self
    }

    pub fn have_no_match_group(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("match_group_id"));
        self
    }


    pub fn group_by_match_group(self) -> Self {
        self.group_by("match_group_id")
    }

    pub fn group_by_match_group_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("match_group_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("match_group_id"));
        request
    }

    pub fn group_by_match_group_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("match_group_id")
            .aggregate_with_function("match_group_id", alias, function)
    }

    pub fn group_by_match_group_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("match_group_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "match_group",
            "match_group_id",
            request,
        ));
        self
    }

    pub fn group_by_match_group_with_details(self) -> Self {
        self.group_by_match_group_with_details_from(crate::Q::match_groups().unlimited())
    }

    pub fn group_by_match_group_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_match_group_with(request)
    }


    pub fn roll_up_to_match_group(self) -> Self {
        self.roll_up_to_match_group_with(crate::Q::match_groups().unlimited())
    }

    pub fn roll_up_to_match_group_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_match_group_matching(selection.clone())
            .group_by_match_group_with(selection)
    }

    pub fn count_match_group(self) -> Self {
        self.count_match_group_as("match_group_count")
    }

    pub fn count_match_group_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("match_group_id", alias)
    }

    pub fn unselect_match_group(mut self) -> Self {
        self.query.projection.retain(|field| field != "match_group_id");
        self.query.relations.retain(|relation| relation.name != "match_group");
        self
    }


    /// Please use `with_match_status_is` instead
    pub(crate) fn filter_by_match_status(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("match_status_id", value.entity_id_value()));
        self
    }
    /// Complex relation filter for `match_status`.
    ///
    /// **Usage Priority:**
    ///
    /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
    ///    - [`Self::with_match_status_is_xxx`]
    ///
    ///    This gives the best code readability.
    ///
    /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
    ///
    /// # Example
    /// ```rust
    /// // Only use when building dynamic queries
    /// let dynamic_query = crate::Q::match_statuses_minimal().filter(...);
    /// let request = crate::Q::tournament_matches().with_match_status_matching(dynamic_query);
    /// ```
    pub fn with_match_status_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "match_status_id",
            <crate::MatchStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("match_status", selection));
        self
    }


    /// Complex relation filter for `match_status`.
    ///
    /// **Usage Priority:**
    ///
    /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
    ///    - [`Self::with_match_status_is_not_xxx`]
    ///
    ///    This gives the best code readability.
    ///
    /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
    ///
    /// # Example
    /// ```rust
    /// // Only use when building dynamic queries
    /// let dynamic_query = crate::Q::match_statuses_minimal().filter(...);
    /// let request = crate::Q::tournament_matches().without_match_status_matching(dynamic_query);
    /// ```
    pub fn without_match_status_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "match_status_id",
            <crate::MatchStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("match_status", selection));
        self
    }


    pub fn have_match_status(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("match_status_id"));
        self
    }

    pub fn have_no_match_status(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("match_status_id"));
        self
    }


    pub fn group_by_match_status(self) -> Self {
        self.group_by("match_status_id")
    }

    pub fn group_by_match_status_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("match_status_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("match_status_id"));
        request
    }

    pub fn group_by_match_status_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("match_status_id")
            .aggregate_with_function("match_status_id", alias, function)
    }

    pub fn group_by_match_status_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("match_status_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "match_status",
            "match_status_id",
            request,
        ));
        self
    }

    pub fn group_by_match_status_with_details(self) -> Self {
        self.group_by_match_status_with_details_from(crate::Q::match_statuses().unlimited())
    }

    pub fn group_by_match_status_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_match_status_with(request)
    }


    pub fn roll_up_to_match_status(self) -> Self {
        self.roll_up_to_match_status_with(crate::Q::match_statuses().unlimited())
    }

    pub fn roll_up_to_match_status_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_match_status_matching(selection.clone())
            .group_by_match_status_with(selection)
    }

    pub fn count_match_status(self) -> Self {
        self.count_match_status_as("match_status_count")
    }

    pub fn count_match_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("match_status_id", alias)
    }

    pub fn unselect_match_status(mut self) -> Self {
        self.query.projection.retain(|field| field != "match_status_id");
        self.query.relations.retain(|relation| relation.name != "match_status");
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
    pub fn match_stage_is_group(self) -> Self {
        self.filter_by_match_stage(1001_u64)
    }

    pub fn with_match_stage_is_group(self) -> Self {
        self.filter_by_match_stage(1001_u64)
    }



    pub fn with_match_stage_is_not_group(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("match_stage_id", 1001_u64));
        self
    }


    pub fn match_stage_is_round_of32(self) -> Self {
        self.filter_by_match_stage(1002_u64)
    }

    pub fn with_match_stage_is_round_of32(self) -> Self {
        self.filter_by_match_stage(1002_u64)
    }



    pub fn with_match_stage_is_not_round_of32(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("match_stage_id", 1002_u64));
        self
    }


    pub fn match_stage_is_round_of16(self) -> Self {
        self.filter_by_match_stage(1003_u64)
    }

    pub fn with_match_stage_is_round_of16(self) -> Self {
        self.filter_by_match_stage(1003_u64)
    }



    pub fn with_match_stage_is_not_round_of16(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("match_stage_id", 1003_u64));
        self
    }


    pub fn match_stage_is_quarter_final(self) -> Self {
        self.filter_by_match_stage(1004_u64)
    }

    pub fn with_match_stage_is_quarter_final(self) -> Self {
        self.filter_by_match_stage(1004_u64)
    }



    pub fn with_match_stage_is_not_quarter_final(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("match_stage_id", 1004_u64));
        self
    }


    pub fn match_stage_is_semi_final(self) -> Self {
        self.filter_by_match_stage(1005_u64)
    }

    pub fn with_match_stage_is_semi_final(self) -> Self {
        self.filter_by_match_stage(1005_u64)
    }



    pub fn with_match_stage_is_not_semi_final(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("match_stage_id", 1005_u64));
        self
    }


    pub fn match_stage_is_third_place(self) -> Self {
        self.filter_by_match_stage(1006_u64)
    }

    pub fn with_match_stage_is_third_place(self) -> Self {
        self.filter_by_match_stage(1006_u64)
    }



    pub fn with_match_stage_is_not_third_place(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("match_stage_id", 1006_u64));
        self
    }


    pub fn match_stage_is_final(self) -> Self {
        self.filter_by_match_stage(1007_u64)
    }

    pub fn with_match_stage_is_final(self) -> Self {
        self.filter_by_match_stage(1007_u64)
    }



    pub fn with_match_stage_is_not_final(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("match_stage_id", 1007_u64));
        self
    }





    pub fn match_status_is_scheduled(self) -> Self {
        self.filter_by_match_status(1001_u64)
    }

    pub fn with_match_status_is_scheduled(self) -> Self {
        self.filter_by_match_status(1001_u64)
    }



    pub fn with_match_status_is_not_scheduled(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("match_status_id", 1001_u64));
        self
    }


    pub fn match_status_is_live(self) -> Self {
        self.filter_by_match_status(1002_u64)
    }

    pub fn with_match_status_is_live(self) -> Self {
        self.filter_by_match_status(1002_u64)
    }



    pub fn with_match_status_is_not_live(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("match_status_id", 1002_u64));
        self
    }


    pub fn match_status_is_finished(self) -> Self {
        self.filter_by_match_status(1003_u64)
    }

    pub fn with_match_status_is_finished(self) -> Self {
        self.filter_by_match_status(1003_u64)
    }



    pub fn with_match_status_is_not_finished(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("match_status_id", 1003_u64));
        self
    }


    pub fn match_status_is_postponed(self) -> Self {
        self.filter_by_match_status(1004_u64)
    }

    pub fn with_match_status_is_postponed(self) -> Self {
        self.filter_by_match_status(1004_u64)
    }



    pub fn with_match_status_is_not_postponed(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("match_status_id", 1004_u64));
        self
    }




    pub fn select_home_team(mut self) -> Self {
        self.query = self.query.relation("home_team");
        self
    }

    pub fn select_home_team_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("home_team", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("home_team", selection));
        self
}

    pub fn facet_by_home_team_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_home_team_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_home_team_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "home_team",
            request,
            include_all_facets,
        ));
        self
    }

    pub fn select_away_team(mut self) -> Self {
        self.query = self.query.relation("away_team");
        self
    }

    pub fn select_away_team_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("away_team", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("away_team", selection));
        self
}

    pub fn facet_by_away_team_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_away_team_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_away_team_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "away_team",
            request,
            include_all_facets,
        ));
        self
    }

    pub fn select_match_stage(mut self) -> Self {
        self.query = self.query.relation("match_stage");
        self
    }

    pub fn select_match_stage_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("match_stage", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("match_stage", selection));
        self
}

    pub fn facet_by_match_stage_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_match_stage_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_match_stage_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "match_stage",
            request,
            include_all_facets,
        ));
        self
    }

    pub fn select_match_group(mut self) -> Self {
        self.query = self.query.relation("match_group");
        self
    }

    pub fn select_match_group_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("match_group", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("match_group", selection));
        self
}

    pub fn facet_by_match_group_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_match_group_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_match_group_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "match_group",
            request,
            include_all_facets,
        ));
        self
    }

    pub fn select_match_status(mut self) -> Self {
        self.query = self.query.relation("match_status");
        self
    }

    pub fn select_match_status_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("match_status", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("match_status", selection));
        self
}

    pub fn facet_by_match_status_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_match_status_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_match_status_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "match_status",
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
    pub fn have_match_goals(self) -> Self {
        self.with_match_goal_list_matching(SelectQuery::new("MatchGoal"))
    }

    pub fn have_no_match_goals(self) -> Self {
        self.without_match_goal_list_matching(SelectQuery::new("MatchGoal"))
    }

    pub fn with_match_goal_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::MatchGoal as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "tournament_match_id",
        ));
        self.relation_filters.push(RelationFilter::new("match_goal_list", selection));
        self
    }

    pub fn without_match_goal_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::MatchGoal as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "tournament_match_id",
        ));
        self.relation_filters.push(RelationFilter::new("match_goal_list", selection));
        self
    }

    pub fn select_match_goal_list(mut self) -> Self {
        self.query = self.query.relation("match_goal_list");
        self
    }

    pub fn select_match_goal_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("match_goal_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("match_goal_list", selection));
        self
}

    pub fn have_match_cards(self) -> Self {
        self.with_match_card_list_matching(SelectQuery::new("MatchCard"))
    }

    pub fn have_no_match_cards(self) -> Self {
        self.without_match_card_list_matching(SelectQuery::new("MatchCard"))
    }

    pub fn with_match_card_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::MatchCard as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "tournament_match_id",
        ));
        self.relation_filters.push(RelationFilter::new("match_card_list", selection));
        self
    }

    pub fn without_match_card_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::MatchCard as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "tournament_match_id",
        ));
        self.relation_filters.push(RelationFilter::new("match_card_list", selection));
        self
    }

    pub fn select_match_card_list(mut self) -> Self {
        self.query = self.query.relation("match_card_list");
        self
    }

    pub fn select_match_card_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("match_card_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("match_card_list", selection));
        self
}
    pub fn count_match_goals(self) -> Self {
        self.count_match_goals_as("count_match_goals")
    }

    pub fn count_match_goals_as(self, alias: impl Into<String>) -> Self {
        self.count_match_goals_with(alias, crate::Q::match_goals().unlimited())
    }

    pub fn count_match_goals_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "match_goal_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_match_goals(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_goals_as("refinements", request)
    }

    pub fn stats_from_match_goals_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "match_goal_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_match_goals_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_goals(request)
    }


    pub fn sum_minute_scored_of_match_goals(self) -> Self {
        self.sum_minute_scored_of_match_goals_as("sum_minute_scored_of_match_goals", crate::Q::match_goals().unlimited())
    }

    pub fn sum_minute_scored_of_match_goals_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_goals_as(alias, request.into().into_query().sum("minute_scored", "sum_minute_scored"))
    }
    pub fn min_minute_scored_of_match_goals(self) -> Self {
        self.min_minute_scored_of_match_goals_as("min_minute_scored_of_match_goals", crate::Q::match_goals().unlimited())
    }

    pub fn min_minute_scored_of_match_goals_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_goals_as(alias, request.into().into_query().min("minute_scored", "min_minute_scored"))
    }
    pub fn max_minute_scored_of_match_goals(self) -> Self {
        self.max_minute_scored_of_match_goals_as("max_minute_scored_of_match_goals", crate::Q::match_goals().unlimited())
    }

    pub fn max_minute_scored_of_match_goals_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_goals_as(alias, request.into().into_query().max("minute_scored", "max_minute_scored"))
    }
    pub fn avg_minute_scored_of_match_goals(self) -> Self {
        self.avg_minute_scored_of_match_goals_as("avg_minute_scored_of_match_goals", crate::Q::match_goals().unlimited())
    }

    pub fn avg_minute_scored_of_match_goals_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_goals_as(alias, request.into().into_query().avg("minute_scored", "avg_minute_scored"))
    }
    pub fn standard_deviation_minute_scored_of_match_goals(self) -> Self {
        self.standard_deviation_minute_scored_of_match_goals_as("standard_deviation_minute_scored_of_match_goals", crate::Q::match_goals().unlimited())
    }

    pub fn standard_deviation_minute_scored_of_match_goals_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_goals_as(alias, request.into().into_query().stddev("minute_scored", "stdDev_minute_scored"))
    }
    pub fn square_root_of_population_standard_deviation_minute_scored_of_match_goals(self) -> Self {
        self.square_root_of_population_standard_deviation_minute_scored_of_match_goals_as("square_root_of_population_standard_deviation_minute_scored_of_match_goals", crate::Q::match_goals().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_minute_scored_of_match_goals_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_goals_as(alias, request.into().into_query().stddev_pop("minute_scored", "stdDevPop_minute_scored"))
    }
    pub fn sample_variance_minute_scored_of_match_goals(self) -> Self {
        self.sample_variance_minute_scored_of_match_goals_as("sample_variance_minute_scored_of_match_goals", crate::Q::match_goals().unlimited())
    }

    pub fn sample_variance_minute_scored_of_match_goals_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_goals_as(alias, request.into().into_query().var_samp("minute_scored", "varSamp_minute_scored"))
    }
    pub fn sample_population_variance_minute_scored_of_match_goals(self) -> Self {
        self.sample_population_variance_minute_scored_of_match_goals_as("sample_population_variance_minute_scored_of_match_goals", crate::Q::match_goals().unlimited())
    }

    pub fn sample_population_variance_minute_scored_of_match_goals_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_goals_as(alias, request.into().into_query().var_pop("minute_scored", "varPop_minute_scored"))
    }
    pub fn min_create_time_of_match_goals(self) -> Self {
        self.min_create_time_of_match_goals_as("min_create_time_of_match_goals", crate::Q::match_goals().unlimited())
    }

    pub fn min_create_time_of_match_goals_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_goals_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_match_goals(self) -> Self {
        self.max_create_time_of_match_goals_as("max_create_time_of_match_goals", crate::Q::match_goals().unlimited())
    }

    pub fn max_create_time_of_match_goals_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_goals_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_match_goals(self) -> Self {
        self.min_update_time_of_match_goals_as("min_update_time_of_match_goals", crate::Q::match_goals().unlimited())
    }

    pub fn min_update_time_of_match_goals_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_goals_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_match_goals(self) -> Self {
        self.max_update_time_of_match_goals_as("max_update_time_of_match_goals", crate::Q::match_goals().unlimited())
    }

    pub fn max_update_time_of_match_goals_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_goals_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_match_cards(self) -> Self {
        self.count_match_cards_as("count_match_cards")
    }

    pub fn count_match_cards_as(self, alias: impl Into<String>) -> Self {
        self.count_match_cards_with(alias, crate::Q::match_cards().unlimited())
    }

    pub fn count_match_cards_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "match_card_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_match_cards(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_cards_as("refinements", request)
    }

    pub fn stats_from_match_cards_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "match_card_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_match_cards_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_cards(request)
    }


    pub fn sum_minute_issued_of_match_cards(self) -> Self {
        self.sum_minute_issued_of_match_cards_as("sum_minute_issued_of_match_cards", crate::Q::match_cards().unlimited())
    }

    pub fn sum_minute_issued_of_match_cards_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_cards_as(alias, request.into().into_query().sum("minute_issued", "sum_minute_issued"))
    }
    pub fn min_minute_issued_of_match_cards(self) -> Self {
        self.min_minute_issued_of_match_cards_as("min_minute_issued_of_match_cards", crate::Q::match_cards().unlimited())
    }

    pub fn min_minute_issued_of_match_cards_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_cards_as(alias, request.into().into_query().min("minute_issued", "min_minute_issued"))
    }
    pub fn max_minute_issued_of_match_cards(self) -> Self {
        self.max_minute_issued_of_match_cards_as("max_minute_issued_of_match_cards", crate::Q::match_cards().unlimited())
    }

    pub fn max_minute_issued_of_match_cards_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_cards_as(alias, request.into().into_query().max("minute_issued", "max_minute_issued"))
    }
    pub fn avg_minute_issued_of_match_cards(self) -> Self {
        self.avg_minute_issued_of_match_cards_as("avg_minute_issued_of_match_cards", crate::Q::match_cards().unlimited())
    }

    pub fn avg_minute_issued_of_match_cards_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_cards_as(alias, request.into().into_query().avg("minute_issued", "avg_minute_issued"))
    }
    pub fn standard_deviation_minute_issued_of_match_cards(self) -> Self {
        self.standard_deviation_minute_issued_of_match_cards_as("standard_deviation_minute_issued_of_match_cards", crate::Q::match_cards().unlimited())
    }

    pub fn standard_deviation_minute_issued_of_match_cards_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_cards_as(alias, request.into().into_query().stddev("minute_issued", "stdDev_minute_issued"))
    }
    pub fn square_root_of_population_standard_deviation_minute_issued_of_match_cards(self) -> Self {
        self.square_root_of_population_standard_deviation_minute_issued_of_match_cards_as("square_root_of_population_standard_deviation_minute_issued_of_match_cards", crate::Q::match_cards().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_minute_issued_of_match_cards_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_cards_as(alias, request.into().into_query().stddev_pop("minute_issued", "stdDevPop_minute_issued"))
    }
    pub fn sample_variance_minute_issued_of_match_cards(self) -> Self {
        self.sample_variance_minute_issued_of_match_cards_as("sample_variance_minute_issued_of_match_cards", crate::Q::match_cards().unlimited())
    }

    pub fn sample_variance_minute_issued_of_match_cards_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_cards_as(alias, request.into().into_query().var_samp("minute_issued", "varSamp_minute_issued"))
    }
    pub fn sample_population_variance_minute_issued_of_match_cards(self) -> Self {
        self.sample_population_variance_minute_issued_of_match_cards_as("sample_population_variance_minute_issued_of_match_cards", crate::Q::match_cards().unlimited())
    }

    pub fn sample_population_variance_minute_issued_of_match_cards_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_cards_as(alias, request.into().into_query().var_pop("minute_issued", "varPop_minute_issued"))
    }
    pub fn min_create_time_of_match_cards(self) -> Self {
        self.min_create_time_of_match_cards_as("min_create_time_of_match_cards", crate::Q::match_cards().unlimited())
    }

    pub fn min_create_time_of_match_cards_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_cards_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_match_cards(self) -> Self {
        self.max_create_time_of_match_cards_as("max_create_time_of_match_cards", crate::Q::match_cards().unlimited())
    }

    pub fn max_create_time_of_match_cards_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_cards_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_match_cards(self) -> Self {
        self.min_update_time_of_match_cards_as("min_update_time_of_match_cards", crate::Q::match_cards().unlimited())
    }

    pub fn min_update_time_of_match_cards_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_cards_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_match_cards(self) -> Self {
        self.max_update_time_of_match_cards_as("max_update_time_of_match_cards", crate::Q::match_cards().unlimited())
    }

    pub fn max_update_time_of_match_cards_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_match_cards_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }
}

impl<R> Default for TournamentMatchRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< TournamentMatchRequest<R> > for SelectQuery {
    fn from(request: TournamentMatchRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< TournamentMatchRequest<R> > for QuerySelection {
    fn from(request: TournamentMatchRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::TournamentMatch> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlRepositoryError<C::TournamentMatchRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<TournamentMatchRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::TournamentMatch
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::TournamentMatch::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> TournamentMatchRequest<R> {
        self.inner.query.trace_chain.push(teaql_core::TraceNode {
            entity_type: self.inner.query.entity.clone(),
            entity_id: None,
            comment: self.purpose,
        });
        self.inner
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlRepositoryError<C::TournamentMatchRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
