use crate::{class::Class, node::NodeId};
use serde::{Deserialize, Serialize};

/// ID of a socket, either input or output
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SocketId(pub NodeId, pub usize);

impl From<&SocketId> for u64 {
    fn from(s: &SocketId) -> Self {
        (s.0 as u64) << 32 | s.1 as u64
    }
}

impl From<u64> for SocketId {
    fn from(n: u64) -> Self {
        let (node_id, socket_idx) = ((n >>32) as u32, n as u32);
        Self(node_id, socket_idx as usize)
    }
}

impl Serialize for SocketId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        u64::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SocketId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(SocketId::from(u64::deserialize(deserializer)?))
    }
}

/// ID of an input socket
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct InputSocketId(pub SocketId);

/// ID of an output socket
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct OutputSocketId(pub SocketId);

/// Input of a node.
#[derive(Debug, Clone)]
pub struct InputSocket {
    /// This is merely a type suggestion used to hint what type is expected. Can be used by IDEs to
    /// force only certain type in a connection, requiring to do a proper conversion.
    pub class: Class,
}

impl Serialize for InputSocket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.class.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for InputSocket {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let class = Class::deserialize(deserializer)?;
        Ok(Self { class })
    }
}

/// Output of a node
#[derive(Debug, Clone)]
pub struct OutputSocket {
    pub class: Class,
}

impl Serialize for OutputSocket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.class.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for OutputSocket {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let class = Class::deserialize(deserializer)?;
        Ok(Self { class })
    }
}

/// Connection of a output to an input
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Connection {
    pub output: OutputSocketId,
    pub input: InputSocketId,
}
