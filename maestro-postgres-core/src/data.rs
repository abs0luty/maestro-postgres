use sqlx::{sqlite::SqlitePool, query};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum NodeKind {
    Primary,
    Replica,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ReplicationType {
    Synchronous,
    Asynchronous,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeId(i32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClusterId(i32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node<ParentId> {
    pub id: NodeId,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub replication_from: Option<ParentId>,
    pub replication_type: ReplicationType,
}

// Primary nodes have no parents, so ParentId = ()
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrimaryNode(Node<()>);

// Replica nodes have parents, so ParentId = NodeId
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplicaNode(Node<NodeId>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cluster {
    pub id: ClusterId,
    pub name: String,
    pub primary_nodes: Vec<PrimaryNode>,
}

pub async fn initialize_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    query(
       include_str!("../sql/init.sql") 
    )
    .execute(pool)
    .await?;
    Ok(())
}