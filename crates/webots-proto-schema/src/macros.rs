/// Defines a struct representing a generic Webots node.
#[macro_export]
macro_rules! define_node {
    (
        $(#[$meta:meta])*
        $Name:ident $(($WebotsName:literal))? {
            $(
                $(#[$field_meta:meta])*
                $field:ident : $Type:ty
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, new, Setters)]
        #[serde(rename_all = "camelCase")]
        $(
            #[serde(rename = $WebotsName)]
        )?
        #[setters(prefix = "with_", strip_option, into)]
        pub struct $Name {
            $(
                $(#[$field_meta])*
                #[serde(default, skip_serializing_if = "Option::is_none")]
                #[new(default)]
                pub $field: Option<$crate::types::ProtoField<$Type>>,
            )*
        }

        impl $crate::proto::schema::WebotsNode for $Name {
            fn node_name() -> &'static str {
                #[allow(unused_variables)]
                let name = stringify!($Name);
                $(
                    let name = $WebotsName;
                )?
                name
            }

            fn all_fields() -> &'static [(&'static str, $crate::proto::ast::FieldType)] {
                const FIELDS: &[(&str, $crate::proto::ast::FieldType)] = &[
                    $(
                        (stringify!($field), $crate::map_field_type!($Type)),
                    )*
                ];
                FIELDS
            }
        }
    };
}

/// Maps a Rust type to a Webots `FieldType`.
#[macro_export]
macro_rules! map_field_type {
    ($Type:ty) => {
        <$Type as $crate::proto::schema::WebotsFieldType>::FIELD_TYPE
    };
}

/// Defines a struct representing a Webots `Device` node.
/// Includes standard `Device` fields (`name`, `model`, `description`).
#[macro_export]
macro_rules! define_device {
    (
        $(#[$meta:meta])*
        $Name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident : $Type:ty
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, new, Setters)]
        #[serde(rename_all = "camelCase")]
        #[setters(prefix = "with_", strip_option, into)]
        pub struct $Name {
            // --- Device Fields ---
            #[new(into)]
            #[setters(skip)]
            pub name: $crate::types::ProtoField<$crate::types::SFString>,
            /// Default: ""
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub model: Option<$crate::types::ProtoField<$crate::types::SFString>>,
            /// Default: ""
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub description: Option<$crate::types::ProtoField<$crate::types::SFString>>,

            // --- User Fields ---
            $(
                $(#[$field_meta])*
                #[serde(default, skip_serializing_if = "Option::is_none")]
                #[new(default)]
                pub $field: Option<$crate::types::ProtoField<$Type>>,
            )*
        }

        impl $crate::proto::schema::WebotsNode for $Name {
            fn node_name() -> &'static str {
                stringify!($Name)
            }

            fn all_fields() -> &'static [(&'static str, $crate::proto::ast::FieldType)] {
                const FIELDS: &[(&str, $crate::proto::ast::FieldType)] = &[
                    ("name", $crate::proto::ast::FieldType::SFString),
                    ("model", $crate::proto::ast::FieldType::SFString),
                    ("description", $crate::proto::ast::FieldType::SFString),
                    $(
                        (stringify!($field), $crate::map_field_type!($Type)),
                    )*
                ];
                FIELDS
            }
        }
    };
}

/// Defines a struct representing a Webots `Solid` node.
/// Includes standard `Solid` fields (pose, physics, boundingObject) AND `Device` fields.
#[macro_export]
macro_rules! define_solid {
    (
        $(#[$meta:meta])*
        $Name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident : $Type:ty
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, new, Setters)]
        #[serde(rename_all = "camelCase")]
        #[setters(prefix = "with_", strip_option, into)]
        pub struct $Name {
            // --- Pose Fields ---
            /// Default: 0 0 0
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub translation: Option<$crate::types::ProtoField<$crate::types::SFVec3f>>,
            /// Default: 0 0 1 0
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub rotation: Option<$crate::types::ProtoField<$crate::types::SFRotation>>,
            /// Default: []
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub children: Option<$crate::types::ProtoField<MFNode>>,

            // --- Device Fields ---
            #[new(into)]
            #[setters(skip)]
            pub name: $crate::types::ProtoField<$crate::types::SFString>,
            /// Default: ""
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub model: Option<$crate::types::ProtoField<$crate::types::SFString>>,
            /// Default: ""
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub description: Option<$crate::types::ProtoField<$crate::types::SFString>>,

            // --- Solid Fields ---
            /// Default: "default"
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub contact_material: Option<$crate::types::ProtoField<$crate::types::SFString>>,
            /// Default: []
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub immersion_properties: Option<$crate::types::ProtoField<MFNode>>,
            /// Default: NULL
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub bounding_object: Option<$crate::types::ProtoField<Box<Node>>>,
            /// Default: NULL
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub physics: Option<$crate::types::ProtoField<Box<Node>>>,
            /// Default: FALSE
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub locked: Option<$crate::types::ProtoField<$crate::types::SFBool>>,
            /// Default: 0.0
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub radar_cross_section: Option<$crate::types::ProtoField<$crate::types::SFFloat>>,
            /// Default: []
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub recognition_colors: Option<$crate::types::ProtoField<$crate::types::MFColor>>,

            // --- Hidden Solid fields ---
            /// Default: 0 0 0
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub linear_velocity: Option<$crate::types::ProtoField<$crate::types::SFVec3f>>,
            /// Default: 0 0 0
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub angular_velocity: Option<$crate::types::ProtoField<$crate::types::SFVec3f>>,

            // --- User Fields ---
            $(
                $(#[$field_meta])*
                #[serde(default, skip_serializing_if = "Option::is_none")]
                #[new(default)]
                pub $field: Option<$crate::types::ProtoField<$Type>>,
            )*
        }

        impl $crate::proto::schema::WebotsNode for $Name {
            fn node_name() -> &'static str {
                stringify!($Name)
            }

            fn all_fields() -> &'static [(&'static str, $crate::proto::ast::FieldType)] {
                const FIELDS: &[(&str, $crate::proto::ast::FieldType)] = &[
                    ("translation", $crate::proto::ast::FieldType::SFVec3f),
                    ("rotation", $crate::proto::ast::FieldType::SFRotation),
                    ("children", $crate::proto::ast::FieldType::MFNode),
                    ("name", $crate::proto::ast::FieldType::SFString),
                    ("model", $crate::proto::ast::FieldType::SFString),
                    ("description", $crate::proto::ast::FieldType::SFString),
                    ("contactMaterial", $crate::proto::ast::FieldType::SFString),
                    ("immersionProperties", $crate::proto::ast::FieldType::MFNode),
                    ("boundingObject", $crate::proto::ast::FieldType::SFNode),
                    ("physics", $crate::proto::ast::FieldType::SFNode),
                    ("locked", $crate::proto::ast::FieldType::SFBool),
                    ("radarCrossSection", $crate::proto::ast::FieldType::SFFloat),
                    ("recognitionColors", $crate::proto::ast::FieldType::MFColor),
                    ("linearVelocity", $crate::proto::ast::FieldType::SFVec3f),
                    ("angularVelocity", $crate::proto::ast::FieldType::SFVec3f),
                    $(
                        (stringify!($field), $crate::map_field_type!($Type)),
                    )*
                ];
                FIELDS
            }
        }
    };
}

/// Defines a struct representing a Webots `Motor` node (or descendent).
/// Includes `Device` fields and standard `Motor` fields.
#[macro_export]
macro_rules! define_motor {
    (
        $(#[$meta:meta])*
        $Name:ident {
            $(
                $(#[$field_meta:meta])*
                $field:ident : $Type:ty
            ),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, new, Setters, Default)]
        #[serde(rename_all = "camelCase")]
        #[setters(prefix = "with_", strip_option, into)]
        pub struct $Name {
            // --- Device Fields ---
            #[new(into)]
            #[setters(skip)]
            pub name: $crate::types::ProtoField<$crate::types::SFString>,
            /// Default: ""
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub model: Option<$crate::types::ProtoField<$crate::types::SFString>>,
            /// Default: ""
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub description: Option<$crate::types::ProtoField<$crate::types::SFString>>,

            // --- Motor Fields ---
            /// Default: -1
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub acceleration: Option<$crate::types::ProtoField<$crate::types::SFFloat>>,
            /// Default: 10
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub consumption_factor: Option<$crate::types::ProtoField<$crate::types::SFFloat>>,
            /// Default: 10 0 0
            #[serde(rename = "controlPID", default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub control_pid: Option<$crate::types::ProtoField<$crate::types::SFVec3f>>,
            /// Default: 0
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub min_position: Option<$crate::types::ProtoField<$crate::types::SFFloat>>,
            /// Default: 10
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub max_position: Option<$crate::types::ProtoField<$crate::types::SFFloat>>,
            /// Default: 10
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub max_velocity: Option<$crate::types::ProtoField<$crate::types::SFFloat>>,
            /// Default: 1
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub multiplier: Option<$crate::types::ProtoField<$crate::types::SFFloat>>,
            /// Default: "sound_url"
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub sound: Option<$crate::types::ProtoField<$crate::types::SFString>>,
            /// Default: []
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[new(default)]
            pub muscles: Option<$crate::types::ProtoField<MFNode>>,

            // --- User Fields ---
            $(
                $(#[$field_meta])*
                #[serde(default, skip_serializing_if = "Option::is_none")]
                #[new(default)]
                pub $field: Option<$crate::types::ProtoField<$Type>>,
            )*
        }

        impl $crate::proto::schema::WebotsNode for $Name {
            fn node_name() -> &'static str {
                stringify!($Name)
            }

            fn all_fields() -> &'static [(&'static str, $crate::proto::ast::FieldType)] {
                const FIELDS: &[(&str, $crate::proto::ast::FieldType)] = &[
                    ("name", $crate::proto::ast::FieldType::SFString),
                    ("model", $crate::proto::ast::FieldType::SFString),
                    ("description", $crate::proto::ast::FieldType::SFString),
                    ("acceleration", $crate::proto::ast::FieldType::SFFloat),
                    ("consumptionFactor", $crate::proto::ast::FieldType::SFFloat),
                    ("controlPID", $crate::proto::ast::FieldType::SFVec3f),
                    ("minPosition", $crate::proto::ast::FieldType::SFFloat),
                    ("maxPosition", $crate::proto::ast::FieldType::SFFloat),
                    ("maxVelocity", $crate::proto::ast::FieldType::SFFloat),
                    ("multiplier", $crate::proto::ast::FieldType::SFFloat),
                    ("sound", $crate::proto::ast::FieldType::SFString),
                    ("muscles", $crate::proto::ast::FieldType::MFNode),
                    $(
                        (stringify!($field), $crate::map_field_type!($Type)),
                    )*
                ];
                FIELDS
            }
        }
    };
}
