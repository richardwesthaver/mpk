//! MPK_DB -- QUERY
use crate::{EdgeKey, Id, Prop, Error};
use std::str::FromStr;

macro_rules! impl_node_query {
  ($i:ident, $v:ident) => {
    impl NodeQueryExt for $i {}

    #[allow(clippy::from_over_into)]
    impl Into<NodeQuery> for $i {
      fn into(self) -> NodeQuery {
        NodeQuery::$v(self)
      }
    }
  };
}

macro_rules! impl_edge_query {
  ($i:ident, $v:ident) => {
    impl EdgeQueryExt for $i {}

    #[allow(clippy::from_over_into)]
    impl Into<EdgeQuery> for $i {
      fn into(self) -> EdgeQuery {
        EdgeQuery::$v(self)
      }
    }
  };
}

#[derive(Eq, PartialEq, Clone, Debug, Hash, Copy)]
pub enum EdgeDirection {
  Outbound,
  Inbound,
}

impl FromStr for EdgeDirection {
  type Err = Error;
  fn from_str(s: &str) -> Result<EdgeDirection, Self::Err> {
    match s {
      "out" => Ok(EdgeDirection::Outbound),
      "in" => Ok(EdgeDirection::Inbound),
      e => Err(Error::BadValue(e.to_string())),
    }
  }
}

impl From<EdgeDirection> for String {
  fn from(d: EdgeDirection) -> Self {
    match d {
      EdgeDirection::Outbound => "out".to_string(),
      EdgeDirection::Inbound => "in".to_string(),
    }
  }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum NodeQuery {
  Range(RangeNodeQuery),
  Specific(SpecificNodeQuery),
  Pipe(PipeNodeQuery),

  PropPresence(PropPresenceNodeQuery),
  PropValue(PropValueNodeQuery),

  PipePropPresence(PipePropPresenceNodeQuery),
  PipePropValue(PipePropValueNodeQuery),
}

pub trait NodeQueryExt: Into<NodeQuery> {
  fn outbound(self) -> PipeEdgeQuery {
    PipeEdgeQuery::new(Box::new(self.into()), EdgeDirection::Outbound)
  }
  fn inbound(self) -> PipeEdgeQuery {
    PipeEdgeQuery::new(Box::new(self.into()), EdgeDirection::Inbound)
  }
  fn property<T: Into<Id>>(self, name: T) -> NodePropQuery {
    NodePropQuery::new(self.into(), name)
  }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PropPresenceNodeQuery {
  pub id: Id,
}

impl_node_query!(PropPresenceNodeQuery, PropPresence);

impl PropPresenceNodeQuery {
  /// Creates a new node query for getting nodes with a property.
  pub fn new<I: Into<Id>>(id: I) -> Self {
    Self { id: id.into() }
  }
}

/// Gets vertices with a property equal to a given value.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PropValueNodeQuery {
  /// The name of the property.
  pub id: Id,
  /// The value of the property.
  pub value: Prop,
}

impl_node_query!(PropValueNodeQuery, PropValue);

impl PropValueNodeQuery {
  /// Creates a new vertex query for getting vertices with a property with a
  /// given value.
  ///
  /// Arguments
  /// * `name`: The name of the property.
  /// * `value`: The value of the property.
  pub fn new<I: Into<Id>>(id: I, value: Prop) -> Self {
    Self {
      id: id.into(),
      value,
    }
  }
}

/// Gets vertices with a property.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipePropPresenceNodeQuery {
  /// The query to filter.
  pub inner: Box<NodeQuery>,
  /// The name of the property.
  pub id: Id,
  /// Whether we should look for property presence or lack thereof.
  pub exists: bool,
}

impl_node_query!(PipePropPresenceNodeQuery, PipePropPresence);

impl PipePropPresenceNodeQuery {
  /// Gets vertices with a property.
  ///
  /// Arguments
  /// * `inner`: The query to filter.
  /// * `name`: The name of the property.
  /// * `exists`: Whether we should look for property presence or lack thereof.
  pub fn new<I: Into<Id>>(inner: Box<NodeQuery>, id: I, exists: bool) -> Self {
    Self {
      inner,
      id: id.into(),
      exists,
    }
  }
}

/// Gets vertices with a property equal to a given value.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipePropValueNodeQuery {
  /// The query to filter.
  pub inner: Box<NodeQuery>,
  /// The name of the property.
  pub id: Id,
  /// The value of the property.
  pub value: Prop,
  /// Whether we should look for property equality or non-equality.
  pub equal: bool,
}

impl_node_query!(PipePropValueNodeQuery, PipePropValue);

impl PipePropValueNodeQuery {
  /// Creates a new vertex query for getting vertices with a property with a
  /// given value.
  ///
  /// Arguments
  /// * `inner`: The query to filter.
  /// * `name`: The name of the property.
  /// * `value`: The value of the property.
  /// * `equal`: Whether we should look for property equality or non-equality.
  pub fn new<I: Into<Id>>(
    inner: Box<NodeQuery>,
    id: I,
    value: Prop,
    equal: bool,
  ) -> Self {
    Self {
      inner,
      id: id.into(),
      value,
      equal,
    }
  }
}

/// Gets a range of vertices.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct RangeNodeQuery {
  /// Limits the number of vertices to get.
  pub limit: u32,

  /// Filters the type of vertices returned.
  pub t: Option<Id>,

  /// Sets the lowest vertex ID to return.
  pub start_id: Option<Id>,
}

impl_node_query!(RangeNodeQuery, Range);

impl Default for RangeNodeQuery {
  fn default() -> Self {
    Self::new()
  }
}

impl RangeNodeQuery {
  /// Creates a new vertex range query.
  pub fn new() -> Self {
    Self {
      limit: u32::max_value(),
      t: None,
      start_id: None,
    }
  }

  /// Sets the limit.
  ///
  /// # Arguments
  /// * `limit`: Limits the number of returned results.
  pub fn limit(self, limit: u32) -> Self {
    Self {
      limit,
      t: self.t,
      start_id: self.start_id,
    }
  }

  /// Filter the type of vertices returned.
  ///
  /// # Arguments
  /// * `t`: Sets the type filter.
  pub fn t(self, t: Id) -> Self {
    Self {
      limit: self.limit,
      t: Some(t),
      start_id: self.start_id,
    }
  }

  /// Sets the lowest vertex ID to return.
  ///
  /// # Arguments
  /// * `start_id`: The lowest vertex ID to return.
  pub fn start_id(self, start_id: Id) -> Self {
    Self {
      limit: self.limit,
      t: self.t,
      start_id: Some(start_id),
    }
  }
}

/// Gets a specific set of vertices.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SpecificNodeQuery {
  /// The IDs of the vertices to get.
  pub ids: Vec<Id>,
}

impl_node_query!(SpecificNodeQuery, Specific);

impl SpecificNodeQuery {
  /// Creates a new vertex query for getting a list of vertices by their
  /// IDs.
  ///
  /// Arguments
  /// * `ids`: The IDs of the vertices to get.
  pub fn new(ids: Vec<Id>) -> Self {
    Self { ids }
  }

  /// Creates a new vertex query for getting a single vertex.
  ///
  /// Arguments
  /// * `id`: The ID of the vertex to get.
  pub fn single(id: Id) -> Self {
    Self { ids: vec![id] }
  }
}

/// Gets the vertices associated with edges.
///
/// Generally, you shouldn't need to construct this directly, but rather call
/// `.outbound()` or `.inbound()` on an edge query.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipeNodeQuery {
  /// The edge query to build off of.
  pub inner: Box<EdgeQuery>,

  /// Whether to get outbound or inbound vertices on the edges.
  pub direction: EdgeDirection,

  /// Limits the number of vertices to get.
  pub limit: u32,

  /// Filters the type of vertices returned.
  pub t: Option<Id>,
}

impl_node_query!(PipeNodeQuery, Pipe);

impl PipeNodeQuery {
  /// Creates a new pipe vertex query.
  ///
  /// Arguments
  /// * `inner`: The edge query to build off of.
  /// * `direction`: Whether to get outbound or inbound vertices on the
  ///   edges.
  pub fn new(inner: Box<EdgeQuery>, direction: EdgeDirection) -> Self {
    Self {
      inner,
      direction,
      limit: u32::max_value(),
      t: None,
    }
  }

  /// Sets the limit.
  ///
  /// # Arguments
  /// * `limit`: Limits the number of returned results.
  pub fn limit(self, limit: u32) -> Self {
    Self {
      inner: self.inner,
      direction: self.direction,
      limit,
      t: self.t,
    }
  }

  /// Filter the type of vertices returned.
  ///
  /// # Arguments
  /// * `t`: Sets the type filter.
  pub fn t(self, t: Id) -> Self {
    Self {
      inner: self.inner,
      direction: self.direction,
      limit: self.limit,
      t: Some(t),
    }
  }
}

/// Gets property values associated with vertices.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct NodePropQuery {
  /// The vertex query to build off of.
  pub inner: NodeQuery,

  /// The name of the property to get.
  pub id: Id,
}

impl NodePropQuery {
  /// Creates a new vertex property query.
  ///
  /// Arguments
  /// * `inner`: The vertex query to build off of.
  /// * `name`: The name of the property to get.
  pub fn new<T: Into<Id>>(inner: NodeQuery, id: T) -> Self {
    Self {
      inner,
      id: id.into(),
    }
  }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum EdgeQuery {
  Specific(SpecificEdgeQuery),
  Pipe(PipeEdgeQuery),
  PropPresence(PropPresenceEdgeQuery),
  PropValue(PropValueEdgeQuery),
  PipePropPresence(PipePropPresenceEdgeQuery),
  PipePropValue(PipePropValueEdgeQuery),
}

pub trait EdgeQueryExt: Into<EdgeQuery> {
  fn outbound(self) -> PipeNodeQuery {
    PipeNodeQuery::new(Box::new(self.into()), EdgeDirection::Outbound)
  }

  fn inbound(self) -> PipeNodeQuery {
    PipeNodeQuery::new(Box::new(self.into()), EdgeDirection::Inbound)
  }

  fn property<T: Into<Id>>(self, name: T) -> EdgePropQuery {
    EdgePropQuery::new(self.into(), name)
  }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PropPresenceEdgeQuery {
  /// The id of the property.
  pub id: Id,
}

impl_edge_query!(PropPresenceEdgeQuery, PropPresence);

impl PropPresenceEdgeQuery {
  /// Creates a new edge query for getting edges with a property.
  ///
  /// Arguments
  /// * `id`: The id of the property.
  pub fn new<T: Into<Id>>(id: T) -> Self {
    Self { id: id.into() }
  }
}

/// Gets edges with a property equal to a given value.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PropValueEdgeQuery {
  /// The id of the property.
  pub id: Id,
  /// The value of the property.
  pub value: Prop,
}

impl_edge_query!(PropValueEdgeQuery, PropValue);

impl PropValueEdgeQuery {
  /// Creates a new edge query for getting edges with a property with a
  /// given value.
  pub fn new<T: Into<Id>>(id: T, value: Prop) -> Self {
    Self {
      id: id.into(),
      value,
    }
  }
}

/// Gets edges with a property.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipePropPresenceEdgeQuery {
  /// The query to filter.
  pub inner: Box<EdgeQuery>,
  /// The id of the property.
  pub id: Id,
  /// Whether we should look for property presence or lack thereof.
  pub exists: bool,
}

impl_edge_query!(PipePropPresenceEdgeQuery, PipePropPresence);

impl PipePropPresenceEdgeQuery {
  /// Gets edges with a property.
  pub fn new<T: Into<Id>>(inner: Box<EdgeQuery>, id: T, exists: bool) -> Self {
    Self {
      inner,
      id: id.into(),
      exists,
    }
  }
}

/// Gets edges with a property equal to a given value.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipePropValueEdgeQuery {
  /// The query to filter.
  pub inner: Box<EdgeQuery>,
  /// The id of the property.
  pub id: Id,
  /// The value of the property.
  pub value: Prop,
  /// Whether we should look for property equality or non-equality.
  pub equal: bool,
}

impl_edge_query!(PipePropValueEdgeQuery, PipePropValue);

impl PipePropValueEdgeQuery {
  /// Creates a new edge query for getting edges with a property with a
  /// given value.
  pub fn new<T: Into<Id>>(
    inner: Box<EdgeQuery>,
    id: T,
    value: Prop,
    equal: bool,
  ) -> Self {
    Self {
      inner,
      id: id.into(),
      value,
      equal,
    }
  }
}

/// Gets a specific set of edges.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct SpecificEdgeQuery {
  /// The keys of the edges to get.
  pub keys: Vec<EdgeKey>,
}

impl_edge_query!(SpecificEdgeQuery, Specific);

impl SpecificEdgeQuery {
  /// Creates a new edge query for getting a list of edges by their
  /// keys.
  ///
  /// Arguments
  /// * `keys`: The keys of the edges to get.
  pub fn new(keys: Vec<EdgeKey>) -> Self {
    Self { keys }
  }

  /// Creates a new edge query for getting a single edge.
  ///
  /// Arguments
  /// * `key`: The key of the edge to get.
  pub fn single(key: EdgeKey) -> Self {
    Self { keys: vec![key] }
  }
}

/// Gets the edges associated with vertices.
///
/// Generally, you shouldn't need to construct this directly, but rather call
/// `.outbound()` or `.inbound()` on a vertex query.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PipeEdgeQuery {
  /// The vertex query to build off of.
  pub inner: Box<NodeQuery>,

  /// Whether to get outbound or inbound edges on the vertices.
  pub direction: EdgeDirection,

  /// Limits the number of edges to get.
  pub limit: u32,

  /// Filters the type of edges returned.
  pub t: Option<Id>,

  /// Specifies the newest update datetime for returned edges.
  pub high: Option<u128>,

  /// Specifies the oldest update datetime for returned edges.
  pub low: Option<u128>,
}

impl_edge_query!(PipeEdgeQuery, Pipe);

impl PipeEdgeQuery {
  /// Creates a new pipe edge query.
  ///
  /// Arguments
  /// * `inner`: The edge query to build off of.
  /// * `direction`: Whether to get outbound or inbound edges on the
  ///   vertices.
  /// * `limit`: Limits the number of edges to get.
  pub fn new(inner: Box<NodeQuery>, direction: EdgeDirection) -> Self {
    Self {
      inner,
      direction,
      limit: u32::max_value(),
      t: None,
      high: None,
      low: None,
    }
  }

  /// Sets the limit.
  ///
  /// # Arguments
  /// * `limit`: Limits the number of returned results.
  pub fn limit(self, limit: u32) -> Self {
    Self {
      inner: self.inner,
      direction: self.direction,
      limit,
      t: self.t,
      high: self.high,
      low: self.low,
    }
  }

  /// Filter the type of edges returned.
  ///
  /// # Arguments
  /// * `t`: Sets the type filter.
  pub fn t(self, t: Id) -> Self {
    Self {
      inner: self.inner,
      direction: self.direction,
      limit: self.limit,
      t: Some(t),
      high: self.high,
      low: self.low,
    }
  }

  /// Filter the update datetime of the edges returned.
  ///
  /// # Arguments
  /// * `high`: The newest update datetime for the edges returned.
  pub fn high(self, high: u128) -> Self {
    Self {
      inner: self.inner,
      direction: self.direction,
      limit: self.limit,
      t: self.t,
      high: Some(high),
      low: self.low,
    }
  }

  /// Filter the update datetime of the edges returned.
  ///
  /// # Arguments
  /// * `low`: The oldest update datetime for the edges returned.
  pub fn low(self, low: u128) -> Self {
    Self {
      inner: self.inner,
      direction: self.direction,
      limit: self.limit,
      t: self.t,
      high: self.high,
      low: Some(low),
    }
  }
}

/// Gets property values associated with edges.
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct EdgePropQuery {
  /// The edge query to build off of.
  pub inner: EdgeQuery,

  /// The id of the property to get.
  pub id: Id,
}

impl EdgePropQuery {
  /// Creates a new edge property query.
  pub fn new<T: Into<Id>>(inner: EdgeQuery, id: T) -> Self {
    Self {
      inner,
      id: id.into(),
    }
  }
}
