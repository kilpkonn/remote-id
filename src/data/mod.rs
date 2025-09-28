pub mod basic_id;
pub mod location;
pub mod operator_id;
pub mod system;
pub mod self_id;

#[derive(Debug, Clone, PartialEq)]
pub enum RemoteIDMessage {
    /// Provides ID for UA, characterizes the type of ID, and identifies the type of UA
    BasicID(basic_id::BasicId),

    /// Provides location, altitude, direction, and speed of UA
    Location(location::Location),

    // /// Provides authentication data for the UA
    // Authentication,

    // /// Message that can be used by Operators to identify themselves and the purpose of an operation
    // SelfId,
    /// Includes Remote Pilot location and multiple aircraft information (group) if applicable, and additional system information
    System(system::System),

    /// Provides Operator ID
    OperatorId(operator_id::OperatorId),

    SelfId(self_id::SelfId)
}
