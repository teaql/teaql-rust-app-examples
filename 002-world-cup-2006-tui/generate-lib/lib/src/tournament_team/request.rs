use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{RepositoryError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::TournamentTeam {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::TournamentTeam {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

#[derive(Debug)]
pub struct TournamentTeamRequest<R = crate::TournamentTeam> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for TournamentTeamRequest<R> {
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

impl<R> TournamentTeamRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("TournamentTeam"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> TournamentTeamRequest<T> {
        TournamentTeamRequest {
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
    ) -> Result<SmartList<R>, TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .tournament_team_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .tournament_team_repository()
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
    ) -> Result<Option<R>, TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
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
    ) -> Result<u64, TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .tournament_team_repository()
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
            .ok_or_else(|| RepositoryError::Runtime(RuntimeError::Graph(format!("count result for TournamentTeam is missing or not numeric"))))
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .tournament_team_repository()
            .map_err(|err| RepositoryError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .tournament_team_repository()
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
    ) -> Result<Option<Record>, TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
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
            "team_name" => Some("team_name"),
            "team_code" => Some("team_code"),
            "emoji_flag" => Some("emoji_flag"),
            "fifa_ranking" => Some("fifa_ranking"),
            "manager_name" => Some("manager_name"),
            "group_letter" => Some("group_letter"),
            "create_time" => Some("create_time"),
            "update_time" => Some("update_time"),
            "version" => Some("version"),
            "confederation" | "confederation_id" => Some("confederation_id"),
            "tournament" | "tournament_id" => Some("tournament_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "confederation" => {
                self.with_confederation_matching(
                    crate::Q::confederations_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "tournament" => {
                self.with_tournament_matching(
                    crate::Q::tournaments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "tournament_match_list_as_home_team" => {
                self.with_tournament_match_list_as_home_team_matching(
                    crate::Q::tournament_matches_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "tournament_match_list_as_away_team" => {
                self.with_tournament_match_list_as_away_team_matching(
                    crate::Q::tournament_matches_minimal()
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
            "group_standing_list" => {
                self.with_group_standing_list_matching(
                    crate::Q::group_standings_minimal()
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
        self.query = self.query.project("team_name");
        self.query = self.query.project("team_code");
        self.query = self.query.project("emoji_flag");
        self.query = self.query.project("fifa_ranking");
        self.query = self.query.project("manager_name");
        self.query = self.query.project("group_letter");
        self.query = self.query.project("create_time");
        self.query = self.query.project("update_time");
        self.query = self.query.project("version");
        self.query = self.query.project("confederation_id");
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
        request = request.select_confederation();
        request = request.select_tournament();
        request
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_tournament_match_list_as_home_team();
        request = request.select_tournament_match_list_as_away_team();
        request = request.select_match_goal_list();
        request = request.select_match_card_list();
        request = request.select_group_standing_list();
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

    pub fn select_team_name(mut self) -> Self {
        self.query = self.query.project("team_name");
        self
    }

    pub fn project_team_name(self) -> Self {
        self.select_team_name()
    }

    pub fn select_team_name_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_team_name_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_team_name_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("team_name", raw_sql_segment));
        self
    }

    pub fn group_by_team_name(self) -> Self {
        self.group_by("team_name")
    }

    pub fn group_by_team_name_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("team_name");
        request.query = request
            .query
            .project_expr(alias, Expr::column("team_name"));
        request
    }

    pub fn group_by_team_name_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("team_name")
            .aggregate_with_function("team_name", alias, function)
    }

    pub fn count_team_name(self) -> Self {
        self.count_team_name_as("team_name_count")
    }

    pub fn count_team_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("team_name", alias)
    }

    pub fn sum_team_name(self) -> Self {
        self.sum_team_name_as("sum_team_name")
    }

    pub fn sum_team_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("team_name", alias)
    }

    pub fn avg_team_name(self) -> Self {
        self.avg_team_name_as("avg_team_name")
    }

    pub fn avg_team_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("team_name", alias)
    }

    pub fn min_team_name(self) -> Self {
        self.min_team_name_as("min_team_name")
    }

    pub fn min_team_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("team_name", alias)
    }

    pub fn max_team_name(self) -> Self {
        self.max_team_name_as("max_team_name")
    }

    pub fn max_team_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("team_name", alias)
    }

    pub fn unselect_team_name(mut self) -> Self {
        self.query.projection.retain(|field| field != "team_name");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "team_name");
        self
    }


    pub fn with_team_name(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "team_name",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_team_name_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "team_name",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_team_name_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("team_name", value));
        self
    }



    pub fn with_team_name_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("team_name", value));
        self
    }

    pub fn with_team_name_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("team_name", value));
        self
    }

    pub fn with_team_name_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("team_name", value));
        self
    }

    pub fn with_team_name_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("team_name", value));
        self
    }

    pub fn with_team_name_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("team_name", value));
        self
    }

    pub fn with_team_name_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("team_name", lower, upper));
        self
    }

    pub fn with_team_name_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "team_name",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_team_name_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "team_name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_team_name_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "team_name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_team_name_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("team_name", value));
        self
    }

    pub fn with_team_name_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("team_name", value));
        self
    }

    pub fn with_team_name_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("team_name", value));
        self
    }

    pub fn with_team_name_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("team_name", value));
        self
    }

    pub fn with_team_name_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("team_name", value));
        self
    }

    pub fn with_team_name_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("team_name", value));
        self
    }

    pub fn with_team_name_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("team_name", value));
        self
    }
    pub fn with_team_name_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("team_name", value));
        self
    }

    pub fn with_team_name_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("team_name", value));
        self
    }

    pub fn with_team_name_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("team_name"));
        self
    }



    pub fn with_team_name_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("team_name"));
        self
    }


    pub fn order_by_team_name_asc(mut self) -> Self {
        self.query = self.query.order_asc("team_name");
        self
    }

    pub fn order_by_team_name_desc(mut self) -> Self {
        self.query = self.query.order_desc("team_name");
        self
    }

    pub fn order_by_team_name_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("team_name");
        self
    }

    pub fn order_by_team_name_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("team_name");
        self
    }

    pub fn select_team_code(mut self) -> Self {
        self.query = self.query.project("team_code");
        self
    }

    pub fn project_team_code(self) -> Self {
        self.select_team_code()
    }

    pub fn select_team_code_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_team_code_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_team_code_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("team_code", raw_sql_segment));
        self
    }

    pub fn group_by_team_code(self) -> Self {
        self.group_by("team_code")
    }

    pub fn group_by_team_code_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("team_code");
        request.query = request
            .query
            .project_expr(alias, Expr::column("team_code"));
        request
    }

    pub fn group_by_team_code_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("team_code")
            .aggregate_with_function("team_code", alias, function)
    }

    pub fn count_team_code(self) -> Self {
        self.count_team_code_as("team_code_count")
    }

    pub fn count_team_code_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("team_code", alias)
    }

    pub fn sum_team_code(self) -> Self {
        self.sum_team_code_as("sum_team_code")
    }

    pub fn sum_team_code_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("team_code", alias)
    }

    pub fn avg_team_code(self) -> Self {
        self.avg_team_code_as("avg_team_code")
    }

    pub fn avg_team_code_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("team_code", alias)
    }

    pub fn min_team_code(self) -> Self {
        self.min_team_code_as("min_team_code")
    }

    pub fn min_team_code_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("team_code", alias)
    }

    pub fn max_team_code(self) -> Self {
        self.max_team_code_as("max_team_code")
    }

    pub fn max_team_code_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("team_code", alias)
    }

    pub fn unselect_team_code(mut self) -> Self {
        self.query.projection.retain(|field| field != "team_code");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "team_code");
        self
    }


    pub fn with_team_code(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "team_code",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_team_code_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "team_code",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_team_code_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("team_code", value));
        self
    }



    pub fn with_team_code_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("team_code", value));
        self
    }

    pub fn with_team_code_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("team_code", value));
        self
    }

    pub fn with_team_code_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("team_code", value));
        self
    }

    pub fn with_team_code_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("team_code", value));
        self
    }

    pub fn with_team_code_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("team_code", value));
        self
    }

    pub fn with_team_code_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("team_code", lower, upper));
        self
    }

    pub fn with_team_code_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "team_code",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_team_code_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "team_code",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_team_code_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "team_code",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_team_code_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("team_code", value));
        self
    }

    pub fn with_team_code_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("team_code", value));
        self
    }

    pub fn with_team_code_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("team_code", value));
        self
    }

    pub fn with_team_code_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("team_code", value));
        self
    }

    pub fn with_team_code_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("team_code", value));
        self
    }

    pub fn with_team_code_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("team_code", value));
        self
    }

    pub fn with_team_code_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("team_code", value));
        self
    }
    pub fn with_team_code_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("team_code", value));
        self
    }

    pub fn with_team_code_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("team_code", value));
        self
    }

    pub fn with_team_code_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("team_code"));
        self
    }



    pub fn with_team_code_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("team_code"));
        self
    }


    pub fn order_by_team_code_asc(mut self) -> Self {
        self.query = self.query.order_asc("team_code");
        self
    }

    pub fn order_by_team_code_desc(mut self) -> Self {
        self.query = self.query.order_desc("team_code");
        self
    }

    pub fn order_by_team_code_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("team_code");
        self
    }

    pub fn order_by_team_code_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("team_code");
        self
    }

    pub fn select_emoji_flag(mut self) -> Self {
        self.query = self.query.project("emoji_flag");
        self
    }

    pub fn project_emoji_flag(self) -> Self {
        self.select_emoji_flag()
    }

    pub fn select_emoji_flag_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_emoji_flag_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_emoji_flag_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("emoji_flag", raw_sql_segment));
        self
    }

    pub fn group_by_emoji_flag(self) -> Self {
        self.group_by("emoji_flag")
    }

    pub fn group_by_emoji_flag_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("emoji_flag");
        request.query = request
            .query
            .project_expr(alias, Expr::column("emoji_flag"));
        request
    }

    pub fn group_by_emoji_flag_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("emoji_flag")
            .aggregate_with_function("emoji_flag", alias, function)
    }

    pub fn count_emoji_flag(self) -> Self {
        self.count_emoji_flag_as("emoji_flag_count")
    }

    pub fn count_emoji_flag_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("emoji_flag", alias)
    }

    pub fn sum_emoji_flag(self) -> Self {
        self.sum_emoji_flag_as("sum_emoji_flag")
    }

    pub fn sum_emoji_flag_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("emoji_flag", alias)
    }

    pub fn avg_emoji_flag(self) -> Self {
        self.avg_emoji_flag_as("avg_emoji_flag")
    }

    pub fn avg_emoji_flag_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("emoji_flag", alias)
    }

    pub fn min_emoji_flag(self) -> Self {
        self.min_emoji_flag_as("min_emoji_flag")
    }

    pub fn min_emoji_flag_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("emoji_flag", alias)
    }

    pub fn max_emoji_flag(self) -> Self {
        self.max_emoji_flag_as("max_emoji_flag")
    }

    pub fn max_emoji_flag_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("emoji_flag", alias)
    }

    pub fn unselect_emoji_flag(mut self) -> Self {
        self.query.projection.retain(|field| field != "emoji_flag");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "emoji_flag");
        self
    }


    pub fn with_emoji_flag(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "emoji_flag",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_emoji_flag_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "emoji_flag",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_emoji_flag_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("emoji_flag", value));
        self
    }



    pub fn with_emoji_flag_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("emoji_flag", value));
        self
    }

    pub fn with_emoji_flag_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("emoji_flag", value));
        self
    }

    pub fn with_emoji_flag_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("emoji_flag", value));
        self
    }

    pub fn with_emoji_flag_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("emoji_flag", value));
        self
    }

    pub fn with_emoji_flag_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("emoji_flag", value));
        self
    }

    pub fn with_emoji_flag_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("emoji_flag", lower, upper));
        self
    }

    pub fn with_emoji_flag_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "emoji_flag",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_emoji_flag_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "emoji_flag",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_emoji_flag_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "emoji_flag",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_emoji_flag_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("emoji_flag", value));
        self
    }

    pub fn with_emoji_flag_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("emoji_flag", value));
        self
    }

    pub fn with_emoji_flag_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("emoji_flag", value));
        self
    }

    pub fn with_emoji_flag_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("emoji_flag", value));
        self
    }

    pub fn with_emoji_flag_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("emoji_flag", value));
        self
    }

    pub fn with_emoji_flag_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("emoji_flag", value));
        self
    }

    pub fn with_emoji_flag_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("emoji_flag", value));
        self
    }
    pub fn with_emoji_flag_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("emoji_flag", value));
        self
    }

    pub fn with_emoji_flag_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("emoji_flag", value));
        self
    }

    pub fn with_emoji_flag_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("emoji_flag"));
        self
    }



    pub fn with_emoji_flag_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("emoji_flag"));
        self
    }


    pub fn order_by_emoji_flag_asc(mut self) -> Self {
        self.query = self.query.order_asc("emoji_flag");
        self
    }

    pub fn order_by_emoji_flag_desc(mut self) -> Self {
        self.query = self.query.order_desc("emoji_flag");
        self
    }

    pub fn order_by_emoji_flag_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("emoji_flag");
        self
    }

    pub fn order_by_emoji_flag_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("emoji_flag");
        self
    }

    pub fn select_fifa_ranking(mut self) -> Self {
        self.query = self.query.project("fifa_ranking");
        self
    }

    pub fn project_fifa_ranking(self) -> Self {
        self.select_fifa_ranking()
    }

    pub fn select_fifa_ranking_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_fifa_ranking_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_fifa_ranking_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("fifa_ranking", raw_sql_segment));
        self
    }

    pub fn select_fifa_ranking_with_function(self, function: AggregateFunction) -> Self {
        self.select_fifa_ranking_as_with_function("fifa_ranking", function)
    }

    pub fn select_fifa_ranking_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("fifa_ranking", alias, function)
    }

    pub fn group_by_fifa_ranking(self) -> Self {
        self.group_by("fifa_ranking")
    }

    pub fn group_by_fifa_ranking_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("fifa_ranking");
        request.query = request
            .query
            .project_expr(alias, Expr::column("fifa_ranking"));
        request
    }

    pub fn group_by_fifa_ranking_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("fifa_ranking")
            .aggregate_with_function("fifa_ranking", alias, function)
    }

    pub fn count_fifa_ranking(self) -> Self {
        self.count_fifa_ranking_as("fifa_ranking_count")
    }

    pub fn count_fifa_ranking_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("fifa_ranking", alias)
    }

    pub fn sum_fifa_ranking(self) -> Self {
        self.sum_fifa_ranking_as("sum_fifa_ranking")
    }

    pub fn sum_fifa_ranking_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("fifa_ranking", alias)
    }

    pub fn avg_fifa_ranking(self) -> Self {
        self.avg_fifa_ranking_as("avg_fifa_ranking")
    }

    pub fn avg_fifa_ranking_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("fifa_ranking", alias)
    }

    pub fn min_fifa_ranking(self) -> Self {
        self.min_fifa_ranking_as("min_fifa_ranking")
    }

    pub fn min_fifa_ranking_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("fifa_ranking", alias)
    }

    pub fn max_fifa_ranking(self) -> Self {
        self.max_fifa_ranking_as("max_fifa_ranking")
    }

    pub fn max_fifa_ranking_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("fifa_ranking", alias)
    }

    pub fn standard_deviation_fifa_ranking(self) -> Self {
        self.standard_deviation_fifa_ranking_as("stdDev_fifa_ranking")
    }

    pub fn standard_deviation_fifa_ranking_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("fifa_ranking", alias)
    }

    pub fn square_root_of_population_standard_deviation_fifa_ranking(self) -> Self {
        self.square_root_of_population_standard_deviation_fifa_ranking_as("stdDevPop_fifa_ranking")
    }

    pub fn square_root_of_population_standard_deviation_fifa_ranking_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("fifa_ranking", alias)
    }

    pub fn sample_variance_fifa_ranking(self) -> Self {
        self.sample_variance_fifa_ranking_as("varSamp_fifa_ranking")
    }

    pub fn sample_variance_fifa_ranking_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("fifa_ranking", alias)
    }

    pub fn sample_population_variance_fifa_ranking(self) -> Self {
        self.sample_population_variance_fifa_ranking_as("varPop_fifa_ranking")
    }

    pub fn sample_population_variance_fifa_ranking_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("fifa_ranking", alias)
    }

    pub fn unselect_fifa_ranking(mut self) -> Self {
        self.query.projection.retain(|field| field != "fifa_ranking");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "fifa_ranking");
        self
    }


    pub fn with_fifa_ranking(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "fifa_ranking",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_fifa_ranking_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "fifa_ranking",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_fifa_ranking_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("fifa_ranking", value));
        self
    }



    pub fn with_fifa_ranking_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("fifa_ranking", value));
        self
    }

    pub fn with_fifa_ranking_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("fifa_ranking", value));
        self
    }

    pub fn with_fifa_ranking_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("fifa_ranking", value));
        self
    }

    pub fn with_fifa_ranking_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("fifa_ranking", value));
        self
    }

    pub fn with_fifa_ranking_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("fifa_ranking", value));
        self
    }

    pub fn with_fifa_ranking_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("fifa_ranking", lower, upper));
        self
    }

    pub fn with_fifa_ranking_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "fifa_ranking",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_fifa_ranking_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "fifa_ranking",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_fifa_ranking_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "fifa_ranking",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_fifa_ranking_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("fifa_ranking", value));
        self
    }

    pub fn with_fifa_ranking_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("fifa_ranking", value));
        self
    }

    pub fn with_fifa_ranking_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("fifa_ranking"));
        self
    }



    pub fn with_fifa_ranking_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("fifa_ranking"));
        self
    }


    pub fn order_by_fifa_ranking_asc(mut self) -> Self {
        self.query = self.query.order_asc("fifa_ranking");
        self
    }

    pub fn order_by_fifa_ranking_desc(mut self) -> Self {
        self.query = self.query.order_desc("fifa_ranking");
        self
    }

    pub fn order_by_fifa_ranking_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("fifa_ranking");
        self
    }

    pub fn order_by_fifa_ranking_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("fifa_ranking");
        self
    }

    pub fn select_manager_name(mut self) -> Self {
        self.query = self.query.project("manager_name");
        self
    }

    pub fn project_manager_name(self) -> Self {
        self.select_manager_name()
    }

    pub fn select_manager_name_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_manager_name_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_manager_name_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("manager_name", raw_sql_segment));
        self
    }

    pub fn group_by_manager_name(self) -> Self {
        self.group_by("manager_name")
    }

    pub fn group_by_manager_name_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("manager_name");
        request.query = request
            .query
            .project_expr(alias, Expr::column("manager_name"));
        request
    }

    pub fn group_by_manager_name_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("manager_name")
            .aggregate_with_function("manager_name", alias, function)
    }

    pub fn count_manager_name(self) -> Self {
        self.count_manager_name_as("manager_name_count")
    }

    pub fn count_manager_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("manager_name", alias)
    }

    pub fn sum_manager_name(self) -> Self {
        self.sum_manager_name_as("sum_manager_name")
    }

    pub fn sum_manager_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("manager_name", alias)
    }

    pub fn avg_manager_name(self) -> Self {
        self.avg_manager_name_as("avg_manager_name")
    }

    pub fn avg_manager_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("manager_name", alias)
    }

    pub fn min_manager_name(self) -> Self {
        self.min_manager_name_as("min_manager_name")
    }

    pub fn min_manager_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("manager_name", alias)
    }

    pub fn max_manager_name(self) -> Self {
        self.max_manager_name_as("max_manager_name")
    }

    pub fn max_manager_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("manager_name", alias)
    }

    pub fn unselect_manager_name(mut self) -> Self {
        self.query.projection.retain(|field| field != "manager_name");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "manager_name");
        self
    }


    pub fn with_manager_name(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "manager_name",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_manager_name_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "manager_name",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_manager_name_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("manager_name", value));
        self
    }



    pub fn with_manager_name_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("manager_name", value));
        self
    }

    pub fn with_manager_name_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("manager_name", value));
        self
    }

    pub fn with_manager_name_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("manager_name", value));
        self
    }

    pub fn with_manager_name_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("manager_name", value));
        self
    }

    pub fn with_manager_name_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("manager_name", value));
        self
    }

    pub fn with_manager_name_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("manager_name", lower, upper));
        self
    }

    pub fn with_manager_name_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "manager_name",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_manager_name_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "manager_name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_manager_name_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "manager_name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_manager_name_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("manager_name", value));
        self
    }

    pub fn with_manager_name_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("manager_name", value));
        self
    }

    pub fn with_manager_name_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("manager_name", value));
        self
    }

    pub fn with_manager_name_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("manager_name", value));
        self
    }

    pub fn with_manager_name_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("manager_name", value));
        self
    }

    pub fn with_manager_name_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("manager_name", value));
        self
    }

    pub fn with_manager_name_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("manager_name", value));
        self
    }
    pub fn with_manager_name_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("manager_name", value));
        self
    }

    pub fn with_manager_name_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("manager_name", value));
        self
    }

    pub fn with_manager_name_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("manager_name"));
        self
    }



    pub fn with_manager_name_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("manager_name"));
        self
    }


    pub fn order_by_manager_name_asc(mut self) -> Self {
        self.query = self.query.order_asc("manager_name");
        self
    }

    pub fn order_by_manager_name_desc(mut self) -> Self {
        self.query = self.query.order_desc("manager_name");
        self
    }

    pub fn order_by_manager_name_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("manager_name");
        self
    }

    pub fn order_by_manager_name_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("manager_name");
        self
    }

    pub fn select_group_letter(mut self) -> Self {
        self.query = self.query.project("group_letter");
        self
    }

    pub fn project_group_letter(self) -> Self {
        self.select_group_letter()
    }

    pub fn select_group_letter_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_group_letter_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_group_letter_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("group_letter", raw_sql_segment));
        self
    }

    pub fn group_by_group_letter(self) -> Self {
        self.group_by("group_letter")
    }

    pub fn group_by_group_letter_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("group_letter");
        request.query = request
            .query
            .project_expr(alias, Expr::column("group_letter"));
        request
    }

    pub fn group_by_group_letter_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("group_letter")
            .aggregate_with_function("group_letter", alias, function)
    }

    pub fn count_group_letter(self) -> Self {
        self.count_group_letter_as("group_letter_count")
    }

    pub fn count_group_letter_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("group_letter", alias)
    }

    pub fn sum_group_letter(self) -> Self {
        self.sum_group_letter_as("sum_group_letter")
    }

    pub fn sum_group_letter_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("group_letter", alias)
    }

    pub fn avg_group_letter(self) -> Self {
        self.avg_group_letter_as("avg_group_letter")
    }

    pub fn avg_group_letter_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("group_letter", alias)
    }

    pub fn min_group_letter(self) -> Self {
        self.min_group_letter_as("min_group_letter")
    }

    pub fn min_group_letter_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("group_letter", alias)
    }

    pub fn max_group_letter(self) -> Self {
        self.max_group_letter_as("max_group_letter")
    }

    pub fn max_group_letter_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("group_letter", alias)
    }

    pub fn unselect_group_letter(mut self) -> Self {
        self.query.projection.retain(|field| field != "group_letter");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "group_letter");
        self
    }


    pub fn with_group_letter(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "group_letter",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_group_letter_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "group_letter",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_group_letter_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("group_letter", value));
        self
    }



    pub fn with_group_letter_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("group_letter", value));
        self
    }

    pub fn with_group_letter_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("group_letter", value));
        self
    }

    pub fn with_group_letter_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("group_letter", value));
        self
    }

    pub fn with_group_letter_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("group_letter", value));
        self
    }

    pub fn with_group_letter_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("group_letter", value));
        self
    }

    pub fn with_group_letter_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("group_letter", lower, upper));
        self
    }

    pub fn with_group_letter_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "group_letter",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_group_letter_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "group_letter",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_group_letter_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "group_letter",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_group_letter_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("group_letter", value));
        self
    }

    pub fn with_group_letter_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("group_letter", value));
        self
    }

    pub fn with_group_letter_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("group_letter", value));
        self
    }

    pub fn with_group_letter_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("group_letter", value));
        self
    }

    pub fn with_group_letter_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("group_letter", value));
        self
    }

    pub fn with_group_letter_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("group_letter", value));
        self
    }

    pub fn with_group_letter_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("group_letter", value));
        self
    }
    pub fn with_group_letter_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("group_letter", value));
        self
    }

    pub fn with_group_letter_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("group_letter", value));
        self
    }

    pub fn with_group_letter_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("group_letter"));
        self
    }



    pub fn with_group_letter_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("group_letter"));
        self
    }


    pub fn order_by_group_letter_asc(mut self) -> Self {
        self.query = self.query.order_asc("group_letter");
        self
    }

    pub fn order_by_group_letter_desc(mut self) -> Self {
        self.query = self.query.order_desc("group_letter");
        self
    }

    pub fn order_by_group_letter_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("group_letter");
        self
    }

    pub fn order_by_group_letter_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("group_letter");
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
    /// Please use `with_confederation_is` instead
    pub(crate) fn filter_by_confederation(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("confederation_id", value.entity_id_value()));
        self
    }
    /// Complex relation filter for `confederation`.
    ///
    /// **Usage Priority:**
    ///
    /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
    ///    - [`Self::with_confederation_is_xxx`]
    ///
    ///    This gives the best code readability.
    ///
    /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
    ///
    /// # Example
    /// ```rust
    /// // Only use when building dynamic queries
    /// let dynamic_query = crate::Q::confederations_minimal().filter(...);
    /// let request = crate::Q::tournament_teams().with_confederation_matching(dynamic_query);
    /// ```
    pub fn with_confederation_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "confederation_id",
            <crate::Confederation as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("confederation", selection));
        self
    }


    /// Complex relation filter for `confederation`.
    ///
    /// **Usage Priority:**
    ///
    /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
    ///    - [`Self::with_confederation_is_not_xxx`]
    ///
    ///    This gives the best code readability.
    ///
    /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
    ///
    /// # Example
    /// ```rust
    /// // Only use when building dynamic queries
    /// let dynamic_query = crate::Q::confederations_minimal().filter(...);
    /// let request = crate::Q::tournament_teams().without_confederation_matching(dynamic_query);
    /// ```
    pub fn without_confederation_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "confederation_id",
            <crate::Confederation as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("confederation", selection));
        self
    }


    pub fn have_confederation(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("confederation_id"));
        self
    }

    pub fn have_no_confederation(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("confederation_id"));
        self
    }


    pub fn group_by_confederation(self) -> Self {
        self.group_by("confederation_id")
    }

    pub fn group_by_confederation_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("confederation_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("confederation_id"));
        request
    }

    pub fn group_by_confederation_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("confederation_id")
            .aggregate_with_function("confederation_id", alias, function)
    }

    pub fn group_by_confederation_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("confederation_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "confederation",
            "confederation_id",
            request,
        ));
        self
    }

    pub fn group_by_confederation_with_details(self) -> Self {
        self.group_by_confederation_with_details_from(crate::Q::confederations().unlimited())
    }

    pub fn group_by_confederation_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_confederation_with(request)
    }


    pub fn roll_up_to_confederation(self) -> Self {
        self.roll_up_to_confederation_with(crate::Q::confederations().unlimited())
    }

    pub fn roll_up_to_confederation_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_confederation_matching(selection.clone())
            .group_by_confederation_with(selection)
    }

    pub fn count_confederation(self) -> Self {
        self.count_confederation_as("confederation_count")
    }

    pub fn count_confederation_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("confederation_id", alias)
    }

    pub fn unselect_confederation(mut self) -> Self {
        self.query.projection.retain(|field| field != "confederation_id");
        self.query.relations.retain(|relation| relation.name != "confederation");
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
    pub fn confederation_is_afc(self) -> Self {
        self.filter_by_confederation(1001_u64)
    }

    pub fn with_confederation_is_afc(self) -> Self {
        self.filter_by_confederation(1001_u64)
    }



    pub fn with_confederation_is_not_afc(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("confederation_id", 1001_u64));
        self
    }


    pub fn confederation_is_caf(self) -> Self {
        self.filter_by_confederation(1002_u64)
    }

    pub fn with_confederation_is_caf(self) -> Self {
        self.filter_by_confederation(1002_u64)
    }



    pub fn with_confederation_is_not_caf(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("confederation_id", 1002_u64));
        self
    }


    pub fn confederation_is_concacaf(self) -> Self {
        self.filter_by_confederation(1003_u64)
    }

    pub fn with_confederation_is_concacaf(self) -> Self {
        self.filter_by_confederation(1003_u64)
    }



    pub fn with_confederation_is_not_concacaf(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("confederation_id", 1003_u64));
        self
    }


    pub fn confederation_is_conmebol(self) -> Self {
        self.filter_by_confederation(1004_u64)
    }

    pub fn with_confederation_is_conmebol(self) -> Self {
        self.filter_by_confederation(1004_u64)
    }



    pub fn with_confederation_is_not_conmebol(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("confederation_id", 1004_u64));
        self
    }


    pub fn confederation_is_ofc(self) -> Self {
        self.filter_by_confederation(1005_u64)
    }

    pub fn with_confederation_is_ofc(self) -> Self {
        self.filter_by_confederation(1005_u64)
    }



    pub fn with_confederation_is_not_ofc(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("confederation_id", 1005_u64));
        self
    }


    pub fn confederation_is_uefa(self) -> Self {
        self.filter_by_confederation(1006_u64)
    }

    pub fn with_confederation_is_uefa(self) -> Self {
        self.filter_by_confederation(1006_u64)
    }



    pub fn with_confederation_is_not_uefa(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("confederation_id", 1006_u64));
        self
    }




    pub fn select_confederation(mut self) -> Self {
        self.query = self.query.relation("confederation");
        self
    }

    pub fn select_confederation_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("confederation", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("confederation", selection));
        self
}

    pub fn facet_by_confederation_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_confederation_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_confederation_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "confederation",
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
    pub fn have_tournament_matches_as_home_team(self) -> Self {
        self.with_tournament_match_list_as_home_team_matching(SelectQuery::new("TournamentMatch"))
    }

    pub fn have_no_tournament_matches_as_home_team(self) -> Self {
        self.without_tournament_match_list_as_home_team_matching(SelectQuery::new("TournamentMatch"))
    }

    pub fn with_tournament_match_list_as_home_team_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::TournamentMatch as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "home_team_id",
        ));
        self.relation_filters.push(RelationFilter::new("tournament_match_list_as_home_team", selection));
        self
    }

    pub fn without_tournament_match_list_as_home_team_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::TournamentMatch as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "home_team_id",
        ));
        self.relation_filters.push(RelationFilter::new("tournament_match_list_as_home_team", selection));
        self
    }

    pub fn select_tournament_match_list_as_home_team(mut self) -> Self {
        self.query = self.query.relation("tournament_match_list_as_home_team");
        self
    }

    pub fn select_tournament_match_list_as_home_team_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("tournament_match_list_as_home_team", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("tournament_match_list_as_home_team", selection));
        self
}

    pub fn have_tournament_matches_as_away_team(self) -> Self {
        self.with_tournament_match_list_as_away_team_matching(SelectQuery::new("TournamentMatch"))
    }

    pub fn have_no_tournament_matches_as_away_team(self) -> Self {
        self.without_tournament_match_list_as_away_team_matching(SelectQuery::new("TournamentMatch"))
    }

    pub fn with_tournament_match_list_as_away_team_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::TournamentMatch as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "away_team_id",
        ));
        self.relation_filters.push(RelationFilter::new("tournament_match_list_as_away_team", selection));
        self
    }

    pub fn without_tournament_match_list_as_away_team_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::TournamentMatch as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "away_team_id",
        ));
        self.relation_filters.push(RelationFilter::new("tournament_match_list_as_away_team", selection));
        self
    }

    pub fn select_tournament_match_list_as_away_team(mut self) -> Self {
        self.query = self.query.relation("tournament_match_list_as_away_team");
        self
    }

    pub fn select_tournament_match_list_as_away_team_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("tournament_match_list_as_away_team", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("tournament_match_list_as_away_team", selection));
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
            "tournament_team_id",
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
            "tournament_team_id",
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
            "tournament_team_id",
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
            "tournament_team_id",
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

    pub fn have_group_standings(self) -> Self {
        self.with_group_standing_list_matching(SelectQuery::new("GroupStanding"))
    }

    pub fn have_no_group_standings(self) -> Self {
        self.without_group_standing_list_matching(SelectQuery::new("GroupStanding"))
    }

    pub fn with_group_standing_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::GroupStanding as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "tournament_team_id",
        ));
        self.relation_filters.push(RelationFilter::new("group_standing_list", selection));
        self
    }

    pub fn without_group_standing_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::GroupStanding as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "tournament_team_id",
        ));
        self.relation_filters.push(RelationFilter::new("group_standing_list", selection));
        self
    }

    pub fn select_group_standing_list(mut self) -> Self {
        self.query = self.query.relation("group_standing_list");
        self
    }

    pub fn select_group_standing_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("group_standing_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("group_standing_list", selection));
        self
}
    pub fn count_tournament_matches_as_home_team(self) -> Self {
        self.count_tournament_matches_as_home_team_as("count_tournament_matches_as_home_team")
    }

    pub fn count_tournament_matches_as_home_team_as(self, alias: impl Into<String>) -> Self {
        self.count_tournament_matches_as_home_team_with(alias, crate::Q::tournament_matches().unlimited())
    }

    pub fn count_tournament_matches_as_home_team_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "tournament_match_list_as_home_team",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_tournament_matches_as_home_team(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as("refinements", request)
    }

    pub fn stats_from_tournament_matches_as_home_team_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "tournament_match_list_as_home_team",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_tournament_matches_as_home_team_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team(request)
    }


    pub fn sum_match_number_of_tournament_matches_as_home_team(self) -> Self {
        self.sum_match_number_of_tournament_matches_as_home_team_as("sum_match_number_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sum_match_number_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().sum("match_number", "sum_match_number"))
    }
    pub fn min_match_number_of_tournament_matches_as_home_team(self) -> Self {
        self.min_match_number_of_tournament_matches_as_home_team_as("min_match_number_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_match_number_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().min("match_number", "min_match_number"))
    }
    pub fn max_match_number_of_tournament_matches_as_home_team(self) -> Self {
        self.max_match_number_of_tournament_matches_as_home_team_as("max_match_number_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_match_number_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().max("match_number", "max_match_number"))
    }
    pub fn avg_match_number_of_tournament_matches_as_home_team(self) -> Self {
        self.avg_match_number_of_tournament_matches_as_home_team_as("avg_match_number_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn avg_match_number_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().avg("match_number", "avg_match_number"))
    }
    pub fn standard_deviation_match_number_of_tournament_matches_as_home_team(self) -> Self {
        self.standard_deviation_match_number_of_tournament_matches_as_home_team_as("standard_deviation_match_number_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn standard_deviation_match_number_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().stddev("match_number", "stdDev_match_number"))
    }
    pub fn square_root_of_population_standard_deviation_match_number_of_tournament_matches_as_home_team(self) -> Self {
        self.square_root_of_population_standard_deviation_match_number_of_tournament_matches_as_home_team_as("square_root_of_population_standard_deviation_match_number_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_match_number_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().stddev_pop("match_number", "stdDevPop_match_number"))
    }
    pub fn sample_variance_match_number_of_tournament_matches_as_home_team(self) -> Self {
        self.sample_variance_match_number_of_tournament_matches_as_home_team_as("sample_variance_match_number_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_variance_match_number_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().var_samp("match_number", "varSamp_match_number"))
    }
    pub fn sample_population_variance_match_number_of_tournament_matches_as_home_team(self) -> Self {
        self.sample_population_variance_match_number_of_tournament_matches_as_home_team_as("sample_population_variance_match_number_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_population_variance_match_number_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().var_pop("match_number", "varPop_match_number"))
    }
    pub fn min_match_date_of_tournament_matches_as_home_team(self) -> Self {
        self.min_match_date_of_tournament_matches_as_home_team_as("min_match_date_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_match_date_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().min("match_date", "min_match_date"))
    }
    pub fn max_match_date_of_tournament_matches_as_home_team(self) -> Self {
        self.max_match_date_of_tournament_matches_as_home_team_as("max_match_date_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_match_date_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().max("match_date", "max_match_date"))
    }
    pub fn sum_home_score_of_tournament_matches_as_home_team(self) -> Self {
        self.sum_home_score_of_tournament_matches_as_home_team_as("sum_home_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sum_home_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().sum("home_score", "sum_home_score"))
    }
    pub fn min_home_score_of_tournament_matches_as_home_team(self) -> Self {
        self.min_home_score_of_tournament_matches_as_home_team_as("min_home_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_home_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().min("home_score", "min_home_score"))
    }
    pub fn max_home_score_of_tournament_matches_as_home_team(self) -> Self {
        self.max_home_score_of_tournament_matches_as_home_team_as("max_home_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_home_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().max("home_score", "max_home_score"))
    }
    pub fn avg_home_score_of_tournament_matches_as_home_team(self) -> Self {
        self.avg_home_score_of_tournament_matches_as_home_team_as("avg_home_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn avg_home_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().avg("home_score", "avg_home_score"))
    }
    pub fn standard_deviation_home_score_of_tournament_matches_as_home_team(self) -> Self {
        self.standard_deviation_home_score_of_tournament_matches_as_home_team_as("standard_deviation_home_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn standard_deviation_home_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().stddev("home_score", "stdDev_home_score"))
    }
    pub fn square_root_of_population_standard_deviation_home_score_of_tournament_matches_as_home_team(self) -> Self {
        self.square_root_of_population_standard_deviation_home_score_of_tournament_matches_as_home_team_as("square_root_of_population_standard_deviation_home_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_home_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().stddev_pop("home_score", "stdDevPop_home_score"))
    }
    pub fn sample_variance_home_score_of_tournament_matches_as_home_team(self) -> Self {
        self.sample_variance_home_score_of_tournament_matches_as_home_team_as("sample_variance_home_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_variance_home_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().var_samp("home_score", "varSamp_home_score"))
    }
    pub fn sample_population_variance_home_score_of_tournament_matches_as_home_team(self) -> Self {
        self.sample_population_variance_home_score_of_tournament_matches_as_home_team_as("sample_population_variance_home_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_population_variance_home_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().var_pop("home_score", "varPop_home_score"))
    }
    pub fn sum_away_score_of_tournament_matches_as_home_team(self) -> Self {
        self.sum_away_score_of_tournament_matches_as_home_team_as("sum_away_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sum_away_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().sum("away_score", "sum_away_score"))
    }
    pub fn min_away_score_of_tournament_matches_as_home_team(self) -> Self {
        self.min_away_score_of_tournament_matches_as_home_team_as("min_away_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_away_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().min("away_score", "min_away_score"))
    }
    pub fn max_away_score_of_tournament_matches_as_home_team(self) -> Self {
        self.max_away_score_of_tournament_matches_as_home_team_as("max_away_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_away_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().max("away_score", "max_away_score"))
    }
    pub fn avg_away_score_of_tournament_matches_as_home_team(self) -> Self {
        self.avg_away_score_of_tournament_matches_as_home_team_as("avg_away_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn avg_away_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().avg("away_score", "avg_away_score"))
    }
    pub fn standard_deviation_away_score_of_tournament_matches_as_home_team(self) -> Self {
        self.standard_deviation_away_score_of_tournament_matches_as_home_team_as("standard_deviation_away_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn standard_deviation_away_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().stddev("away_score", "stdDev_away_score"))
    }
    pub fn square_root_of_population_standard_deviation_away_score_of_tournament_matches_as_home_team(self) -> Self {
        self.square_root_of_population_standard_deviation_away_score_of_tournament_matches_as_home_team_as("square_root_of_population_standard_deviation_away_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_away_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().stddev_pop("away_score", "stdDevPop_away_score"))
    }
    pub fn sample_variance_away_score_of_tournament_matches_as_home_team(self) -> Self {
        self.sample_variance_away_score_of_tournament_matches_as_home_team_as("sample_variance_away_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_variance_away_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().var_samp("away_score", "varSamp_away_score"))
    }
    pub fn sample_population_variance_away_score_of_tournament_matches_as_home_team(self) -> Self {
        self.sample_population_variance_away_score_of_tournament_matches_as_home_team_as("sample_population_variance_away_score_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_population_variance_away_score_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().var_pop("away_score", "varPop_away_score"))
    }
    pub fn sum_extra_time_home_of_tournament_matches_as_home_team(self) -> Self {
        self.sum_extra_time_home_of_tournament_matches_as_home_team_as("sum_extra_time_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sum_extra_time_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().sum("extra_time_home", "sum_extra_time_home"))
    }
    pub fn min_extra_time_home_of_tournament_matches_as_home_team(self) -> Self {
        self.min_extra_time_home_of_tournament_matches_as_home_team_as("min_extra_time_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_extra_time_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().min("extra_time_home", "min_extra_time_home"))
    }
    pub fn max_extra_time_home_of_tournament_matches_as_home_team(self) -> Self {
        self.max_extra_time_home_of_tournament_matches_as_home_team_as("max_extra_time_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_extra_time_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().max("extra_time_home", "max_extra_time_home"))
    }
    pub fn avg_extra_time_home_of_tournament_matches_as_home_team(self) -> Self {
        self.avg_extra_time_home_of_tournament_matches_as_home_team_as("avg_extra_time_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn avg_extra_time_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().avg("extra_time_home", "avg_extra_time_home"))
    }
    pub fn standard_deviation_extra_time_home_of_tournament_matches_as_home_team(self) -> Self {
        self.standard_deviation_extra_time_home_of_tournament_matches_as_home_team_as("standard_deviation_extra_time_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn standard_deviation_extra_time_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().stddev("extra_time_home", "stdDev_extra_time_home"))
    }
    pub fn square_root_of_population_standard_deviation_extra_time_home_of_tournament_matches_as_home_team(self) -> Self {
        self.square_root_of_population_standard_deviation_extra_time_home_of_tournament_matches_as_home_team_as("square_root_of_population_standard_deviation_extra_time_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_extra_time_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().stddev_pop("extra_time_home", "stdDevPop_extra_time_home"))
    }
    pub fn sample_variance_extra_time_home_of_tournament_matches_as_home_team(self) -> Self {
        self.sample_variance_extra_time_home_of_tournament_matches_as_home_team_as("sample_variance_extra_time_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_variance_extra_time_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().var_samp("extra_time_home", "varSamp_extra_time_home"))
    }
    pub fn sample_population_variance_extra_time_home_of_tournament_matches_as_home_team(self) -> Self {
        self.sample_population_variance_extra_time_home_of_tournament_matches_as_home_team_as("sample_population_variance_extra_time_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_population_variance_extra_time_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().var_pop("extra_time_home", "varPop_extra_time_home"))
    }
    pub fn sum_extra_time_away_of_tournament_matches_as_home_team(self) -> Self {
        self.sum_extra_time_away_of_tournament_matches_as_home_team_as("sum_extra_time_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sum_extra_time_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().sum("extra_time_away", "sum_extra_time_away"))
    }
    pub fn min_extra_time_away_of_tournament_matches_as_home_team(self) -> Self {
        self.min_extra_time_away_of_tournament_matches_as_home_team_as("min_extra_time_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_extra_time_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().min("extra_time_away", "min_extra_time_away"))
    }
    pub fn max_extra_time_away_of_tournament_matches_as_home_team(self) -> Self {
        self.max_extra_time_away_of_tournament_matches_as_home_team_as("max_extra_time_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_extra_time_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().max("extra_time_away", "max_extra_time_away"))
    }
    pub fn avg_extra_time_away_of_tournament_matches_as_home_team(self) -> Self {
        self.avg_extra_time_away_of_tournament_matches_as_home_team_as("avg_extra_time_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn avg_extra_time_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().avg("extra_time_away", "avg_extra_time_away"))
    }
    pub fn standard_deviation_extra_time_away_of_tournament_matches_as_home_team(self) -> Self {
        self.standard_deviation_extra_time_away_of_tournament_matches_as_home_team_as("standard_deviation_extra_time_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn standard_deviation_extra_time_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().stddev("extra_time_away", "stdDev_extra_time_away"))
    }
    pub fn square_root_of_population_standard_deviation_extra_time_away_of_tournament_matches_as_home_team(self) -> Self {
        self.square_root_of_population_standard_deviation_extra_time_away_of_tournament_matches_as_home_team_as("square_root_of_population_standard_deviation_extra_time_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_extra_time_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().stddev_pop("extra_time_away", "stdDevPop_extra_time_away"))
    }
    pub fn sample_variance_extra_time_away_of_tournament_matches_as_home_team(self) -> Self {
        self.sample_variance_extra_time_away_of_tournament_matches_as_home_team_as("sample_variance_extra_time_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_variance_extra_time_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().var_samp("extra_time_away", "varSamp_extra_time_away"))
    }
    pub fn sample_population_variance_extra_time_away_of_tournament_matches_as_home_team(self) -> Self {
        self.sample_population_variance_extra_time_away_of_tournament_matches_as_home_team_as("sample_population_variance_extra_time_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_population_variance_extra_time_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().var_pop("extra_time_away", "varPop_extra_time_away"))
    }
    pub fn sum_penalty_home_of_tournament_matches_as_home_team(self) -> Self {
        self.sum_penalty_home_of_tournament_matches_as_home_team_as("sum_penalty_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sum_penalty_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().sum("penalty_home", "sum_penalty_home"))
    }
    pub fn min_penalty_home_of_tournament_matches_as_home_team(self) -> Self {
        self.min_penalty_home_of_tournament_matches_as_home_team_as("min_penalty_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_penalty_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().min("penalty_home", "min_penalty_home"))
    }
    pub fn max_penalty_home_of_tournament_matches_as_home_team(self) -> Self {
        self.max_penalty_home_of_tournament_matches_as_home_team_as("max_penalty_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_penalty_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().max("penalty_home", "max_penalty_home"))
    }
    pub fn avg_penalty_home_of_tournament_matches_as_home_team(self) -> Self {
        self.avg_penalty_home_of_tournament_matches_as_home_team_as("avg_penalty_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn avg_penalty_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().avg("penalty_home", "avg_penalty_home"))
    }
    pub fn standard_deviation_penalty_home_of_tournament_matches_as_home_team(self) -> Self {
        self.standard_deviation_penalty_home_of_tournament_matches_as_home_team_as("standard_deviation_penalty_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn standard_deviation_penalty_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().stddev("penalty_home", "stdDev_penalty_home"))
    }
    pub fn square_root_of_population_standard_deviation_penalty_home_of_tournament_matches_as_home_team(self) -> Self {
        self.square_root_of_population_standard_deviation_penalty_home_of_tournament_matches_as_home_team_as("square_root_of_population_standard_deviation_penalty_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_penalty_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().stddev_pop("penalty_home", "stdDevPop_penalty_home"))
    }
    pub fn sample_variance_penalty_home_of_tournament_matches_as_home_team(self) -> Self {
        self.sample_variance_penalty_home_of_tournament_matches_as_home_team_as("sample_variance_penalty_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_variance_penalty_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().var_samp("penalty_home", "varSamp_penalty_home"))
    }
    pub fn sample_population_variance_penalty_home_of_tournament_matches_as_home_team(self) -> Self {
        self.sample_population_variance_penalty_home_of_tournament_matches_as_home_team_as("sample_population_variance_penalty_home_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_population_variance_penalty_home_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().var_pop("penalty_home", "varPop_penalty_home"))
    }
    pub fn sum_penalty_away_of_tournament_matches_as_home_team(self) -> Self {
        self.sum_penalty_away_of_tournament_matches_as_home_team_as("sum_penalty_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sum_penalty_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().sum("penalty_away", "sum_penalty_away"))
    }
    pub fn min_penalty_away_of_tournament_matches_as_home_team(self) -> Self {
        self.min_penalty_away_of_tournament_matches_as_home_team_as("min_penalty_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_penalty_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().min("penalty_away", "min_penalty_away"))
    }
    pub fn max_penalty_away_of_tournament_matches_as_home_team(self) -> Self {
        self.max_penalty_away_of_tournament_matches_as_home_team_as("max_penalty_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_penalty_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().max("penalty_away", "max_penalty_away"))
    }
    pub fn avg_penalty_away_of_tournament_matches_as_home_team(self) -> Self {
        self.avg_penalty_away_of_tournament_matches_as_home_team_as("avg_penalty_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn avg_penalty_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().avg("penalty_away", "avg_penalty_away"))
    }
    pub fn standard_deviation_penalty_away_of_tournament_matches_as_home_team(self) -> Self {
        self.standard_deviation_penalty_away_of_tournament_matches_as_home_team_as("standard_deviation_penalty_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn standard_deviation_penalty_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().stddev("penalty_away", "stdDev_penalty_away"))
    }
    pub fn square_root_of_population_standard_deviation_penalty_away_of_tournament_matches_as_home_team(self) -> Self {
        self.square_root_of_population_standard_deviation_penalty_away_of_tournament_matches_as_home_team_as("square_root_of_population_standard_deviation_penalty_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_penalty_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().stddev_pop("penalty_away", "stdDevPop_penalty_away"))
    }
    pub fn sample_variance_penalty_away_of_tournament_matches_as_home_team(self) -> Self {
        self.sample_variance_penalty_away_of_tournament_matches_as_home_team_as("sample_variance_penalty_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_variance_penalty_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().var_samp("penalty_away", "varSamp_penalty_away"))
    }
    pub fn sample_population_variance_penalty_away_of_tournament_matches_as_home_team(self) -> Self {
        self.sample_population_variance_penalty_away_of_tournament_matches_as_home_team_as("sample_population_variance_penalty_away_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_population_variance_penalty_away_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().var_pop("penalty_away", "varPop_penalty_away"))
    }
    pub fn min_create_time_of_tournament_matches_as_home_team(self) -> Self {
        self.min_create_time_of_tournament_matches_as_home_team_as("min_create_time_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_create_time_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_tournament_matches_as_home_team(self) -> Self {
        self.max_create_time_of_tournament_matches_as_home_team_as("max_create_time_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_create_time_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_tournament_matches_as_home_team(self) -> Self {
        self.min_update_time_of_tournament_matches_as_home_team_as("min_update_time_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_update_time_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_tournament_matches_as_home_team(self) -> Self {
        self.max_update_time_of_tournament_matches_as_home_team_as("max_update_time_of_tournament_matches_as_home_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_update_time_of_tournament_matches_as_home_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_home_team_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_tournament_matches_as_away_team(self) -> Self {
        self.count_tournament_matches_as_away_team_as("count_tournament_matches_as_away_team")
    }

    pub fn count_tournament_matches_as_away_team_as(self, alias: impl Into<String>) -> Self {
        self.count_tournament_matches_as_away_team_with(alias, crate::Q::tournament_matches().unlimited())
    }

    pub fn count_tournament_matches_as_away_team_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "tournament_match_list_as_away_team",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_tournament_matches_as_away_team(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as("refinements", request)
    }

    pub fn stats_from_tournament_matches_as_away_team_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "tournament_match_list_as_away_team",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_tournament_matches_as_away_team_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team(request)
    }


    pub fn sum_match_number_of_tournament_matches_as_away_team(self) -> Self {
        self.sum_match_number_of_tournament_matches_as_away_team_as("sum_match_number_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sum_match_number_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().sum("match_number", "sum_match_number"))
    }
    pub fn min_match_number_of_tournament_matches_as_away_team(self) -> Self {
        self.min_match_number_of_tournament_matches_as_away_team_as("min_match_number_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_match_number_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().min("match_number", "min_match_number"))
    }
    pub fn max_match_number_of_tournament_matches_as_away_team(self) -> Self {
        self.max_match_number_of_tournament_matches_as_away_team_as("max_match_number_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_match_number_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().max("match_number", "max_match_number"))
    }
    pub fn avg_match_number_of_tournament_matches_as_away_team(self) -> Self {
        self.avg_match_number_of_tournament_matches_as_away_team_as("avg_match_number_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn avg_match_number_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().avg("match_number", "avg_match_number"))
    }
    pub fn standard_deviation_match_number_of_tournament_matches_as_away_team(self) -> Self {
        self.standard_deviation_match_number_of_tournament_matches_as_away_team_as("standard_deviation_match_number_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn standard_deviation_match_number_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().stddev("match_number", "stdDev_match_number"))
    }
    pub fn square_root_of_population_standard_deviation_match_number_of_tournament_matches_as_away_team(self) -> Self {
        self.square_root_of_population_standard_deviation_match_number_of_tournament_matches_as_away_team_as("square_root_of_population_standard_deviation_match_number_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_match_number_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().stddev_pop("match_number", "stdDevPop_match_number"))
    }
    pub fn sample_variance_match_number_of_tournament_matches_as_away_team(self) -> Self {
        self.sample_variance_match_number_of_tournament_matches_as_away_team_as("sample_variance_match_number_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_variance_match_number_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().var_samp("match_number", "varSamp_match_number"))
    }
    pub fn sample_population_variance_match_number_of_tournament_matches_as_away_team(self) -> Self {
        self.sample_population_variance_match_number_of_tournament_matches_as_away_team_as("sample_population_variance_match_number_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_population_variance_match_number_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().var_pop("match_number", "varPop_match_number"))
    }
    pub fn min_match_date_of_tournament_matches_as_away_team(self) -> Self {
        self.min_match_date_of_tournament_matches_as_away_team_as("min_match_date_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_match_date_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().min("match_date", "min_match_date"))
    }
    pub fn max_match_date_of_tournament_matches_as_away_team(self) -> Self {
        self.max_match_date_of_tournament_matches_as_away_team_as("max_match_date_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_match_date_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().max("match_date", "max_match_date"))
    }
    pub fn sum_home_score_of_tournament_matches_as_away_team(self) -> Self {
        self.sum_home_score_of_tournament_matches_as_away_team_as("sum_home_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sum_home_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().sum("home_score", "sum_home_score"))
    }
    pub fn min_home_score_of_tournament_matches_as_away_team(self) -> Self {
        self.min_home_score_of_tournament_matches_as_away_team_as("min_home_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_home_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().min("home_score", "min_home_score"))
    }
    pub fn max_home_score_of_tournament_matches_as_away_team(self) -> Self {
        self.max_home_score_of_tournament_matches_as_away_team_as("max_home_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_home_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().max("home_score", "max_home_score"))
    }
    pub fn avg_home_score_of_tournament_matches_as_away_team(self) -> Self {
        self.avg_home_score_of_tournament_matches_as_away_team_as("avg_home_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn avg_home_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().avg("home_score", "avg_home_score"))
    }
    pub fn standard_deviation_home_score_of_tournament_matches_as_away_team(self) -> Self {
        self.standard_deviation_home_score_of_tournament_matches_as_away_team_as("standard_deviation_home_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn standard_deviation_home_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().stddev("home_score", "stdDev_home_score"))
    }
    pub fn square_root_of_population_standard_deviation_home_score_of_tournament_matches_as_away_team(self) -> Self {
        self.square_root_of_population_standard_deviation_home_score_of_tournament_matches_as_away_team_as("square_root_of_population_standard_deviation_home_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_home_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().stddev_pop("home_score", "stdDevPop_home_score"))
    }
    pub fn sample_variance_home_score_of_tournament_matches_as_away_team(self) -> Self {
        self.sample_variance_home_score_of_tournament_matches_as_away_team_as("sample_variance_home_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_variance_home_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().var_samp("home_score", "varSamp_home_score"))
    }
    pub fn sample_population_variance_home_score_of_tournament_matches_as_away_team(self) -> Self {
        self.sample_population_variance_home_score_of_tournament_matches_as_away_team_as("sample_population_variance_home_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_population_variance_home_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().var_pop("home_score", "varPop_home_score"))
    }
    pub fn sum_away_score_of_tournament_matches_as_away_team(self) -> Self {
        self.sum_away_score_of_tournament_matches_as_away_team_as("sum_away_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sum_away_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().sum("away_score", "sum_away_score"))
    }
    pub fn min_away_score_of_tournament_matches_as_away_team(self) -> Self {
        self.min_away_score_of_tournament_matches_as_away_team_as("min_away_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_away_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().min("away_score", "min_away_score"))
    }
    pub fn max_away_score_of_tournament_matches_as_away_team(self) -> Self {
        self.max_away_score_of_tournament_matches_as_away_team_as("max_away_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_away_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().max("away_score", "max_away_score"))
    }
    pub fn avg_away_score_of_tournament_matches_as_away_team(self) -> Self {
        self.avg_away_score_of_tournament_matches_as_away_team_as("avg_away_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn avg_away_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().avg("away_score", "avg_away_score"))
    }
    pub fn standard_deviation_away_score_of_tournament_matches_as_away_team(self) -> Self {
        self.standard_deviation_away_score_of_tournament_matches_as_away_team_as("standard_deviation_away_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn standard_deviation_away_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().stddev("away_score", "stdDev_away_score"))
    }
    pub fn square_root_of_population_standard_deviation_away_score_of_tournament_matches_as_away_team(self) -> Self {
        self.square_root_of_population_standard_deviation_away_score_of_tournament_matches_as_away_team_as("square_root_of_population_standard_deviation_away_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_away_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().stddev_pop("away_score", "stdDevPop_away_score"))
    }
    pub fn sample_variance_away_score_of_tournament_matches_as_away_team(self) -> Self {
        self.sample_variance_away_score_of_tournament_matches_as_away_team_as("sample_variance_away_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_variance_away_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().var_samp("away_score", "varSamp_away_score"))
    }
    pub fn sample_population_variance_away_score_of_tournament_matches_as_away_team(self) -> Self {
        self.sample_population_variance_away_score_of_tournament_matches_as_away_team_as("sample_population_variance_away_score_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_population_variance_away_score_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().var_pop("away_score", "varPop_away_score"))
    }
    pub fn sum_extra_time_home_of_tournament_matches_as_away_team(self) -> Self {
        self.sum_extra_time_home_of_tournament_matches_as_away_team_as("sum_extra_time_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sum_extra_time_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().sum("extra_time_home", "sum_extra_time_home"))
    }
    pub fn min_extra_time_home_of_tournament_matches_as_away_team(self) -> Self {
        self.min_extra_time_home_of_tournament_matches_as_away_team_as("min_extra_time_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_extra_time_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().min("extra_time_home", "min_extra_time_home"))
    }
    pub fn max_extra_time_home_of_tournament_matches_as_away_team(self) -> Self {
        self.max_extra_time_home_of_tournament_matches_as_away_team_as("max_extra_time_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_extra_time_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().max("extra_time_home", "max_extra_time_home"))
    }
    pub fn avg_extra_time_home_of_tournament_matches_as_away_team(self) -> Self {
        self.avg_extra_time_home_of_tournament_matches_as_away_team_as("avg_extra_time_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn avg_extra_time_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().avg("extra_time_home", "avg_extra_time_home"))
    }
    pub fn standard_deviation_extra_time_home_of_tournament_matches_as_away_team(self) -> Self {
        self.standard_deviation_extra_time_home_of_tournament_matches_as_away_team_as("standard_deviation_extra_time_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn standard_deviation_extra_time_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().stddev("extra_time_home", "stdDev_extra_time_home"))
    }
    pub fn square_root_of_population_standard_deviation_extra_time_home_of_tournament_matches_as_away_team(self) -> Self {
        self.square_root_of_population_standard_deviation_extra_time_home_of_tournament_matches_as_away_team_as("square_root_of_population_standard_deviation_extra_time_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_extra_time_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().stddev_pop("extra_time_home", "stdDevPop_extra_time_home"))
    }
    pub fn sample_variance_extra_time_home_of_tournament_matches_as_away_team(self) -> Self {
        self.sample_variance_extra_time_home_of_tournament_matches_as_away_team_as("sample_variance_extra_time_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_variance_extra_time_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().var_samp("extra_time_home", "varSamp_extra_time_home"))
    }
    pub fn sample_population_variance_extra_time_home_of_tournament_matches_as_away_team(self) -> Self {
        self.sample_population_variance_extra_time_home_of_tournament_matches_as_away_team_as("sample_population_variance_extra_time_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_population_variance_extra_time_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().var_pop("extra_time_home", "varPop_extra_time_home"))
    }
    pub fn sum_extra_time_away_of_tournament_matches_as_away_team(self) -> Self {
        self.sum_extra_time_away_of_tournament_matches_as_away_team_as("sum_extra_time_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sum_extra_time_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().sum("extra_time_away", "sum_extra_time_away"))
    }
    pub fn min_extra_time_away_of_tournament_matches_as_away_team(self) -> Self {
        self.min_extra_time_away_of_tournament_matches_as_away_team_as("min_extra_time_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_extra_time_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().min("extra_time_away", "min_extra_time_away"))
    }
    pub fn max_extra_time_away_of_tournament_matches_as_away_team(self) -> Self {
        self.max_extra_time_away_of_tournament_matches_as_away_team_as("max_extra_time_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_extra_time_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().max("extra_time_away", "max_extra_time_away"))
    }
    pub fn avg_extra_time_away_of_tournament_matches_as_away_team(self) -> Self {
        self.avg_extra_time_away_of_tournament_matches_as_away_team_as("avg_extra_time_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn avg_extra_time_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().avg("extra_time_away", "avg_extra_time_away"))
    }
    pub fn standard_deviation_extra_time_away_of_tournament_matches_as_away_team(self) -> Self {
        self.standard_deviation_extra_time_away_of_tournament_matches_as_away_team_as("standard_deviation_extra_time_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn standard_deviation_extra_time_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().stddev("extra_time_away", "stdDev_extra_time_away"))
    }
    pub fn square_root_of_population_standard_deviation_extra_time_away_of_tournament_matches_as_away_team(self) -> Self {
        self.square_root_of_population_standard_deviation_extra_time_away_of_tournament_matches_as_away_team_as("square_root_of_population_standard_deviation_extra_time_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_extra_time_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().stddev_pop("extra_time_away", "stdDevPop_extra_time_away"))
    }
    pub fn sample_variance_extra_time_away_of_tournament_matches_as_away_team(self) -> Self {
        self.sample_variance_extra_time_away_of_tournament_matches_as_away_team_as("sample_variance_extra_time_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_variance_extra_time_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().var_samp("extra_time_away", "varSamp_extra_time_away"))
    }
    pub fn sample_population_variance_extra_time_away_of_tournament_matches_as_away_team(self) -> Self {
        self.sample_population_variance_extra_time_away_of_tournament_matches_as_away_team_as("sample_population_variance_extra_time_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_population_variance_extra_time_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().var_pop("extra_time_away", "varPop_extra_time_away"))
    }
    pub fn sum_penalty_home_of_tournament_matches_as_away_team(self) -> Self {
        self.sum_penalty_home_of_tournament_matches_as_away_team_as("sum_penalty_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sum_penalty_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().sum("penalty_home", "sum_penalty_home"))
    }
    pub fn min_penalty_home_of_tournament_matches_as_away_team(self) -> Self {
        self.min_penalty_home_of_tournament_matches_as_away_team_as("min_penalty_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_penalty_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().min("penalty_home", "min_penalty_home"))
    }
    pub fn max_penalty_home_of_tournament_matches_as_away_team(self) -> Self {
        self.max_penalty_home_of_tournament_matches_as_away_team_as("max_penalty_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_penalty_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().max("penalty_home", "max_penalty_home"))
    }
    pub fn avg_penalty_home_of_tournament_matches_as_away_team(self) -> Self {
        self.avg_penalty_home_of_tournament_matches_as_away_team_as("avg_penalty_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn avg_penalty_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().avg("penalty_home", "avg_penalty_home"))
    }
    pub fn standard_deviation_penalty_home_of_tournament_matches_as_away_team(self) -> Self {
        self.standard_deviation_penalty_home_of_tournament_matches_as_away_team_as("standard_deviation_penalty_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn standard_deviation_penalty_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().stddev("penalty_home", "stdDev_penalty_home"))
    }
    pub fn square_root_of_population_standard_deviation_penalty_home_of_tournament_matches_as_away_team(self) -> Self {
        self.square_root_of_population_standard_deviation_penalty_home_of_tournament_matches_as_away_team_as("square_root_of_population_standard_deviation_penalty_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_penalty_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().stddev_pop("penalty_home", "stdDevPop_penalty_home"))
    }
    pub fn sample_variance_penalty_home_of_tournament_matches_as_away_team(self) -> Self {
        self.sample_variance_penalty_home_of_tournament_matches_as_away_team_as("sample_variance_penalty_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_variance_penalty_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().var_samp("penalty_home", "varSamp_penalty_home"))
    }
    pub fn sample_population_variance_penalty_home_of_tournament_matches_as_away_team(self) -> Self {
        self.sample_population_variance_penalty_home_of_tournament_matches_as_away_team_as("sample_population_variance_penalty_home_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_population_variance_penalty_home_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().var_pop("penalty_home", "varPop_penalty_home"))
    }
    pub fn sum_penalty_away_of_tournament_matches_as_away_team(self) -> Self {
        self.sum_penalty_away_of_tournament_matches_as_away_team_as("sum_penalty_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sum_penalty_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().sum("penalty_away", "sum_penalty_away"))
    }
    pub fn min_penalty_away_of_tournament_matches_as_away_team(self) -> Self {
        self.min_penalty_away_of_tournament_matches_as_away_team_as("min_penalty_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_penalty_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().min("penalty_away", "min_penalty_away"))
    }
    pub fn max_penalty_away_of_tournament_matches_as_away_team(self) -> Self {
        self.max_penalty_away_of_tournament_matches_as_away_team_as("max_penalty_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_penalty_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().max("penalty_away", "max_penalty_away"))
    }
    pub fn avg_penalty_away_of_tournament_matches_as_away_team(self) -> Self {
        self.avg_penalty_away_of_tournament_matches_as_away_team_as("avg_penalty_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn avg_penalty_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().avg("penalty_away", "avg_penalty_away"))
    }
    pub fn standard_deviation_penalty_away_of_tournament_matches_as_away_team(self) -> Self {
        self.standard_deviation_penalty_away_of_tournament_matches_as_away_team_as("standard_deviation_penalty_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn standard_deviation_penalty_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().stddev("penalty_away", "stdDev_penalty_away"))
    }
    pub fn square_root_of_population_standard_deviation_penalty_away_of_tournament_matches_as_away_team(self) -> Self {
        self.square_root_of_population_standard_deviation_penalty_away_of_tournament_matches_as_away_team_as("square_root_of_population_standard_deviation_penalty_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_penalty_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().stddev_pop("penalty_away", "stdDevPop_penalty_away"))
    }
    pub fn sample_variance_penalty_away_of_tournament_matches_as_away_team(self) -> Self {
        self.sample_variance_penalty_away_of_tournament_matches_as_away_team_as("sample_variance_penalty_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_variance_penalty_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().var_samp("penalty_away", "varSamp_penalty_away"))
    }
    pub fn sample_population_variance_penalty_away_of_tournament_matches_as_away_team(self) -> Self {
        self.sample_population_variance_penalty_away_of_tournament_matches_as_away_team_as("sample_population_variance_penalty_away_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn sample_population_variance_penalty_away_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().var_pop("penalty_away", "varPop_penalty_away"))
    }
    pub fn min_create_time_of_tournament_matches_as_away_team(self) -> Self {
        self.min_create_time_of_tournament_matches_as_away_team_as("min_create_time_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_create_time_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_tournament_matches_as_away_team(self) -> Self {
        self.max_create_time_of_tournament_matches_as_away_team_as("max_create_time_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_create_time_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_tournament_matches_as_away_team(self) -> Self {
        self.min_update_time_of_tournament_matches_as_away_team_as("min_update_time_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn min_update_time_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_tournament_matches_as_away_team(self) -> Self {
        self.max_update_time_of_tournament_matches_as_away_team_as("max_update_time_of_tournament_matches_as_away_team", crate::Q::tournament_matches().unlimited())
    }

    pub fn max_update_time_of_tournament_matches_as_away_team_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tournament_matches_as_away_team_as(alias, request.into().into_query().max("update_time", "max_update_time"))
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

    pub fn count_group_standings(self) -> Self {
        self.count_group_standings_as("count_group_standings")
    }

    pub fn count_group_standings_as(self, alias: impl Into<String>) -> Self {
        self.count_group_standings_with(alias, crate::Q::group_standings().unlimited())
    }

    pub fn count_group_standings_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "group_standing_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_group_standings(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as("refinements", request)
    }

    pub fn stats_from_group_standings_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "group_standing_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_group_standings_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings(request)
    }


    pub fn sum_played_of_group_standings(self) -> Self {
        self.sum_played_of_group_standings_as("sum_played_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sum_played_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().sum("played", "sum_played"))
    }
    pub fn min_played_of_group_standings(self) -> Self {
        self.min_played_of_group_standings_as("min_played_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn min_played_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().min("played", "min_played"))
    }
    pub fn max_played_of_group_standings(self) -> Self {
        self.max_played_of_group_standings_as("max_played_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn max_played_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().max("played", "max_played"))
    }
    pub fn avg_played_of_group_standings(self) -> Self {
        self.avg_played_of_group_standings_as("avg_played_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn avg_played_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().avg("played", "avg_played"))
    }
    pub fn standard_deviation_played_of_group_standings(self) -> Self {
        self.standard_deviation_played_of_group_standings_as("standard_deviation_played_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn standard_deviation_played_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev("played", "stdDev_played"))
    }
    pub fn square_root_of_population_standard_deviation_played_of_group_standings(self) -> Self {
        self.square_root_of_population_standard_deviation_played_of_group_standings_as("square_root_of_population_standard_deviation_played_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_played_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev_pop("played", "stdDevPop_played"))
    }
    pub fn sample_variance_played_of_group_standings(self) -> Self {
        self.sample_variance_played_of_group_standings_as("sample_variance_played_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_variance_played_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_samp("played", "varSamp_played"))
    }
    pub fn sample_population_variance_played_of_group_standings(self) -> Self {
        self.sample_population_variance_played_of_group_standings_as("sample_population_variance_played_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_population_variance_played_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_pop("played", "varPop_played"))
    }
    pub fn sum_won_of_group_standings(self) -> Self {
        self.sum_won_of_group_standings_as("sum_won_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sum_won_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().sum("won", "sum_won"))
    }
    pub fn min_won_of_group_standings(self) -> Self {
        self.min_won_of_group_standings_as("min_won_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn min_won_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().min("won", "min_won"))
    }
    pub fn max_won_of_group_standings(self) -> Self {
        self.max_won_of_group_standings_as("max_won_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn max_won_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().max("won", "max_won"))
    }
    pub fn avg_won_of_group_standings(self) -> Self {
        self.avg_won_of_group_standings_as("avg_won_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn avg_won_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().avg("won", "avg_won"))
    }
    pub fn standard_deviation_won_of_group_standings(self) -> Self {
        self.standard_deviation_won_of_group_standings_as("standard_deviation_won_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn standard_deviation_won_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev("won", "stdDev_won"))
    }
    pub fn square_root_of_population_standard_deviation_won_of_group_standings(self) -> Self {
        self.square_root_of_population_standard_deviation_won_of_group_standings_as("square_root_of_population_standard_deviation_won_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_won_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev_pop("won", "stdDevPop_won"))
    }
    pub fn sample_variance_won_of_group_standings(self) -> Self {
        self.sample_variance_won_of_group_standings_as("sample_variance_won_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_variance_won_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_samp("won", "varSamp_won"))
    }
    pub fn sample_population_variance_won_of_group_standings(self) -> Self {
        self.sample_population_variance_won_of_group_standings_as("sample_population_variance_won_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_population_variance_won_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_pop("won", "varPop_won"))
    }
    pub fn sum_drawn_of_group_standings(self) -> Self {
        self.sum_drawn_of_group_standings_as("sum_drawn_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sum_drawn_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().sum("drawn", "sum_drawn"))
    }
    pub fn min_drawn_of_group_standings(self) -> Self {
        self.min_drawn_of_group_standings_as("min_drawn_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn min_drawn_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().min("drawn", "min_drawn"))
    }
    pub fn max_drawn_of_group_standings(self) -> Self {
        self.max_drawn_of_group_standings_as("max_drawn_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn max_drawn_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().max("drawn", "max_drawn"))
    }
    pub fn avg_drawn_of_group_standings(self) -> Self {
        self.avg_drawn_of_group_standings_as("avg_drawn_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn avg_drawn_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().avg("drawn", "avg_drawn"))
    }
    pub fn standard_deviation_drawn_of_group_standings(self) -> Self {
        self.standard_deviation_drawn_of_group_standings_as("standard_deviation_drawn_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn standard_deviation_drawn_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev("drawn", "stdDev_drawn"))
    }
    pub fn square_root_of_population_standard_deviation_drawn_of_group_standings(self) -> Self {
        self.square_root_of_population_standard_deviation_drawn_of_group_standings_as("square_root_of_population_standard_deviation_drawn_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_drawn_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev_pop("drawn", "stdDevPop_drawn"))
    }
    pub fn sample_variance_drawn_of_group_standings(self) -> Self {
        self.sample_variance_drawn_of_group_standings_as("sample_variance_drawn_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_variance_drawn_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_samp("drawn", "varSamp_drawn"))
    }
    pub fn sample_population_variance_drawn_of_group_standings(self) -> Self {
        self.sample_population_variance_drawn_of_group_standings_as("sample_population_variance_drawn_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_population_variance_drawn_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_pop("drawn", "varPop_drawn"))
    }
    pub fn sum_lost_of_group_standings(self) -> Self {
        self.sum_lost_of_group_standings_as("sum_lost_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sum_lost_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().sum("lost", "sum_lost"))
    }
    pub fn min_lost_of_group_standings(self) -> Self {
        self.min_lost_of_group_standings_as("min_lost_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn min_lost_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().min("lost", "min_lost"))
    }
    pub fn max_lost_of_group_standings(self) -> Self {
        self.max_lost_of_group_standings_as("max_lost_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn max_lost_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().max("lost", "max_lost"))
    }
    pub fn avg_lost_of_group_standings(self) -> Self {
        self.avg_lost_of_group_standings_as("avg_lost_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn avg_lost_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().avg("lost", "avg_lost"))
    }
    pub fn standard_deviation_lost_of_group_standings(self) -> Self {
        self.standard_deviation_lost_of_group_standings_as("standard_deviation_lost_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn standard_deviation_lost_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev("lost", "stdDev_lost"))
    }
    pub fn square_root_of_population_standard_deviation_lost_of_group_standings(self) -> Self {
        self.square_root_of_population_standard_deviation_lost_of_group_standings_as("square_root_of_population_standard_deviation_lost_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_lost_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev_pop("lost", "stdDevPop_lost"))
    }
    pub fn sample_variance_lost_of_group_standings(self) -> Self {
        self.sample_variance_lost_of_group_standings_as("sample_variance_lost_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_variance_lost_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_samp("lost", "varSamp_lost"))
    }
    pub fn sample_population_variance_lost_of_group_standings(self) -> Self {
        self.sample_population_variance_lost_of_group_standings_as("sample_population_variance_lost_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_population_variance_lost_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_pop("lost", "varPop_lost"))
    }
    pub fn sum_goals_for_of_group_standings(self) -> Self {
        self.sum_goals_for_of_group_standings_as("sum_goals_for_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sum_goals_for_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().sum("goals_for", "sum_goals_for"))
    }
    pub fn min_goals_for_of_group_standings(self) -> Self {
        self.min_goals_for_of_group_standings_as("min_goals_for_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn min_goals_for_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().min("goals_for", "min_goals_for"))
    }
    pub fn max_goals_for_of_group_standings(self) -> Self {
        self.max_goals_for_of_group_standings_as("max_goals_for_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn max_goals_for_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().max("goals_for", "max_goals_for"))
    }
    pub fn avg_goals_for_of_group_standings(self) -> Self {
        self.avg_goals_for_of_group_standings_as("avg_goals_for_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn avg_goals_for_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().avg("goals_for", "avg_goals_for"))
    }
    pub fn standard_deviation_goals_for_of_group_standings(self) -> Self {
        self.standard_deviation_goals_for_of_group_standings_as("standard_deviation_goals_for_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn standard_deviation_goals_for_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev("goals_for", "stdDev_goals_for"))
    }
    pub fn square_root_of_population_standard_deviation_goals_for_of_group_standings(self) -> Self {
        self.square_root_of_population_standard_deviation_goals_for_of_group_standings_as("square_root_of_population_standard_deviation_goals_for_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_goals_for_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev_pop("goals_for", "stdDevPop_goals_for"))
    }
    pub fn sample_variance_goals_for_of_group_standings(self) -> Self {
        self.sample_variance_goals_for_of_group_standings_as("sample_variance_goals_for_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_variance_goals_for_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_samp("goals_for", "varSamp_goals_for"))
    }
    pub fn sample_population_variance_goals_for_of_group_standings(self) -> Self {
        self.sample_population_variance_goals_for_of_group_standings_as("sample_population_variance_goals_for_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_population_variance_goals_for_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_pop("goals_for", "varPop_goals_for"))
    }
    pub fn sum_goals_against_of_group_standings(self) -> Self {
        self.sum_goals_against_of_group_standings_as("sum_goals_against_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sum_goals_against_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().sum("goals_against", "sum_goals_against"))
    }
    pub fn min_goals_against_of_group_standings(self) -> Self {
        self.min_goals_against_of_group_standings_as("min_goals_against_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn min_goals_against_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().min("goals_against", "min_goals_against"))
    }
    pub fn max_goals_against_of_group_standings(self) -> Self {
        self.max_goals_against_of_group_standings_as("max_goals_against_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn max_goals_against_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().max("goals_against", "max_goals_against"))
    }
    pub fn avg_goals_against_of_group_standings(self) -> Self {
        self.avg_goals_against_of_group_standings_as("avg_goals_against_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn avg_goals_against_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().avg("goals_against", "avg_goals_against"))
    }
    pub fn standard_deviation_goals_against_of_group_standings(self) -> Self {
        self.standard_deviation_goals_against_of_group_standings_as("standard_deviation_goals_against_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn standard_deviation_goals_against_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev("goals_against", "stdDev_goals_against"))
    }
    pub fn square_root_of_population_standard_deviation_goals_against_of_group_standings(self) -> Self {
        self.square_root_of_population_standard_deviation_goals_against_of_group_standings_as("square_root_of_population_standard_deviation_goals_against_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_goals_against_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev_pop("goals_against", "stdDevPop_goals_against"))
    }
    pub fn sample_variance_goals_against_of_group_standings(self) -> Self {
        self.sample_variance_goals_against_of_group_standings_as("sample_variance_goals_against_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_variance_goals_against_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_samp("goals_against", "varSamp_goals_against"))
    }
    pub fn sample_population_variance_goals_against_of_group_standings(self) -> Self {
        self.sample_population_variance_goals_against_of_group_standings_as("sample_population_variance_goals_against_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_population_variance_goals_against_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_pop("goals_against", "varPop_goals_against"))
    }
    pub fn sum_goal_difference_of_group_standings(self) -> Self {
        self.sum_goal_difference_of_group_standings_as("sum_goal_difference_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sum_goal_difference_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().sum("goal_difference", "sum_goal_difference"))
    }
    pub fn min_goal_difference_of_group_standings(self) -> Self {
        self.min_goal_difference_of_group_standings_as("min_goal_difference_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn min_goal_difference_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().min("goal_difference", "min_goal_difference"))
    }
    pub fn max_goal_difference_of_group_standings(self) -> Self {
        self.max_goal_difference_of_group_standings_as("max_goal_difference_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn max_goal_difference_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().max("goal_difference", "max_goal_difference"))
    }
    pub fn avg_goal_difference_of_group_standings(self) -> Self {
        self.avg_goal_difference_of_group_standings_as("avg_goal_difference_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn avg_goal_difference_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().avg("goal_difference", "avg_goal_difference"))
    }
    pub fn standard_deviation_goal_difference_of_group_standings(self) -> Self {
        self.standard_deviation_goal_difference_of_group_standings_as("standard_deviation_goal_difference_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn standard_deviation_goal_difference_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev("goal_difference", "stdDev_goal_difference"))
    }
    pub fn square_root_of_population_standard_deviation_goal_difference_of_group_standings(self) -> Self {
        self.square_root_of_population_standard_deviation_goal_difference_of_group_standings_as("square_root_of_population_standard_deviation_goal_difference_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_goal_difference_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev_pop("goal_difference", "stdDevPop_goal_difference"))
    }
    pub fn sample_variance_goal_difference_of_group_standings(self) -> Self {
        self.sample_variance_goal_difference_of_group_standings_as("sample_variance_goal_difference_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_variance_goal_difference_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_samp("goal_difference", "varSamp_goal_difference"))
    }
    pub fn sample_population_variance_goal_difference_of_group_standings(self) -> Self {
        self.sample_population_variance_goal_difference_of_group_standings_as("sample_population_variance_goal_difference_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_population_variance_goal_difference_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_pop("goal_difference", "varPop_goal_difference"))
    }
    pub fn sum_points_of_group_standings(self) -> Self {
        self.sum_points_of_group_standings_as("sum_points_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sum_points_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().sum("points", "sum_points"))
    }
    pub fn min_points_of_group_standings(self) -> Self {
        self.min_points_of_group_standings_as("min_points_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn min_points_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().min("points", "min_points"))
    }
    pub fn max_points_of_group_standings(self) -> Self {
        self.max_points_of_group_standings_as("max_points_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn max_points_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().max("points", "max_points"))
    }
    pub fn avg_points_of_group_standings(self) -> Self {
        self.avg_points_of_group_standings_as("avg_points_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn avg_points_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().avg("points", "avg_points"))
    }
    pub fn standard_deviation_points_of_group_standings(self) -> Self {
        self.standard_deviation_points_of_group_standings_as("standard_deviation_points_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn standard_deviation_points_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev("points", "stdDev_points"))
    }
    pub fn square_root_of_population_standard_deviation_points_of_group_standings(self) -> Self {
        self.square_root_of_population_standard_deviation_points_of_group_standings_as("square_root_of_population_standard_deviation_points_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_points_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev_pop("points", "stdDevPop_points"))
    }
    pub fn sample_variance_points_of_group_standings(self) -> Self {
        self.sample_variance_points_of_group_standings_as("sample_variance_points_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_variance_points_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_samp("points", "varSamp_points"))
    }
    pub fn sample_population_variance_points_of_group_standings(self) -> Self {
        self.sample_population_variance_points_of_group_standings_as("sample_population_variance_points_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_population_variance_points_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_pop("points", "varPop_points"))
    }
    pub fn sum_standing_rank_of_group_standings(self) -> Self {
        self.sum_standing_rank_of_group_standings_as("sum_standing_rank_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sum_standing_rank_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().sum("standing_rank", "sum_standing_rank"))
    }
    pub fn min_standing_rank_of_group_standings(self) -> Self {
        self.min_standing_rank_of_group_standings_as("min_standing_rank_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn min_standing_rank_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().min("standing_rank", "min_standing_rank"))
    }
    pub fn max_standing_rank_of_group_standings(self) -> Self {
        self.max_standing_rank_of_group_standings_as("max_standing_rank_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn max_standing_rank_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().max("standing_rank", "max_standing_rank"))
    }
    pub fn avg_standing_rank_of_group_standings(self) -> Self {
        self.avg_standing_rank_of_group_standings_as("avg_standing_rank_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn avg_standing_rank_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().avg("standing_rank", "avg_standing_rank"))
    }
    pub fn standard_deviation_standing_rank_of_group_standings(self) -> Self {
        self.standard_deviation_standing_rank_of_group_standings_as("standard_deviation_standing_rank_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn standard_deviation_standing_rank_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev("standing_rank", "stdDev_standing_rank"))
    }
    pub fn square_root_of_population_standard_deviation_standing_rank_of_group_standings(self) -> Self {
        self.square_root_of_population_standard_deviation_standing_rank_of_group_standings_as("square_root_of_population_standard_deviation_standing_rank_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn square_root_of_population_standard_deviation_standing_rank_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().stddev_pop("standing_rank", "stdDevPop_standing_rank"))
    }
    pub fn sample_variance_standing_rank_of_group_standings(self) -> Self {
        self.sample_variance_standing_rank_of_group_standings_as("sample_variance_standing_rank_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_variance_standing_rank_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_samp("standing_rank", "varSamp_standing_rank"))
    }
    pub fn sample_population_variance_standing_rank_of_group_standings(self) -> Self {
        self.sample_population_variance_standing_rank_of_group_standings_as("sample_population_variance_standing_rank_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn sample_population_variance_standing_rank_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().var_pop("standing_rank", "varPop_standing_rank"))
    }
    pub fn min_create_time_of_group_standings(self) -> Self {
        self.min_create_time_of_group_standings_as("min_create_time_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn min_create_time_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_group_standings(self) -> Self {
        self.max_create_time_of_group_standings_as("max_create_time_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn max_create_time_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_group_standings(self) -> Self {
        self.min_update_time_of_group_standings_as("min_update_time_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn min_update_time_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_group_standings(self) -> Self {
        self.max_update_time_of_group_standings_as("max_update_time_of_group_standings", crate::Q::group_standings().unlimited())
    }

    pub fn max_update_time_of_group_standings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_group_standings_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }
}

impl<R> Default for TournamentTeamRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< TournamentTeamRequest<R> > for SelectQuery {
    fn from(request: TournamentTeamRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< TournamentTeamRequest<R> > for QuerySelection {
    fn from(request: TournamentTeamRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::TournamentTeam> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlRepositoryError<C::TournamentTeamRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<TournamentTeamRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::TournamentTeam
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::TournamentTeam::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> TournamentTeamRequest<R> {
        self.inner.query.trace_chain.push(teaql_core::TraceNode {
            entity_type: self.inner.query.entity.clone(),
            entity_id: None,
            comment: self.purpose,
        });
        self.inner
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlRepositoryError<C::TournamentTeamRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
