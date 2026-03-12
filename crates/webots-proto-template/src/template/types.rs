use boa_engine::object::builtins::JsArray;
use boa_engine::property::PropertyDescriptor;
use boa_engine::{Context, JsObject, JsString, JsValue};
use derive_new::new;
use derive_setters::Setters;

/// Represents a Webots field type with its value.
#[derive(Debug, Clone)]
pub enum TemplateField {
    SFBool(bool),
    SFInt32(i32),
    SFFloat(f64),
    SFString(String),
    SFVec2f(f64, f64),
    SFVec3f(f64, f64, f64),
    SFRotation(f64, f64, f64, f64),
    SFColor(f64, f64, f64),
    SFNode(String), // For now just string representation or "NULL"
    MFBool(Vec<bool>),
    MFInt32(Vec<i32>),
    MFFloat(Vec<f64>),
    MFString(Vec<String>),
    MFVec2f(Vec<(f64, f64)>),
    MFVec3f(Vec<(f64, f64, f64)>),
    MFRotation(Vec<(f64, f64, f64, f64)>),
    MFColor(Vec<(f64, f64, f64)>),
    MFNode(Vec<String>),
}

impl From<bool> for TemplateField {
    fn from(value: bool) -> Self {
        Self::SFBool(value)
    }
}

impl From<i32> for TemplateField {
    fn from(value: i32) -> Self {
        Self::SFInt32(value)
    }
}

impl From<f64> for TemplateField {
    fn from(value: f64) -> Self {
        Self::SFFloat(value)
    }
}

impl From<String> for TemplateField {
    fn from(value: String) -> Self {
        Self::SFString(value)
    }
}

impl From<&str> for TemplateField {
    fn from(value: &str) -> Self {
        Self::SFString(value.to_string())
    }
}

impl From<(f64, f64)> for TemplateField {
    fn from((x, y): (f64, f64)) -> Self {
        Self::SFVec2f(x, y)
    }
}

impl From<(f64, f64, f64)> for TemplateField {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self::SFVec3f(x, y, z)
    }
}

impl From<(f64, f64, f64, f64)> for TemplateField {
    fn from((x, y, z, angle): (f64, f64, f64, f64)) -> Self {
        Self::SFRotation(x, y, z, angle)
    }
}

/// Represents a single PROTO field exposed to templates.
///
/// Webots exposes both the effective `value` and the declaration-time `defaultValue`
/// for each field key in the `fields` object.
#[derive(Debug, Clone, new)]
pub struct TemplateFieldBinding {
    pub value: TemplateField,
    pub default_value: TemplateField,
}

/// Represents the Webots runtime version exposed via the `context` object.
#[derive(Debug, Clone, Default, new, Setters)]
#[setters(prefix = "with_", strip_option, into)]
pub struct TemplateWebotsVersion {
    pub major: String,
    pub revision: String,
}

/// Fixed metadata keys exposed to templates as `context`.
#[derive(Debug, Clone, Default, Setters)]
#[setters(prefix = "with_", strip_option, into)]
pub struct TemplateContext {
    pub world: Option<String>,
    pub proto: Option<String>,
    pub project_path: Option<String>,
    pub webots_version: Option<TemplateWebotsVersion>,
    pub webots_home: Option<String>,
    pub temporary_files_path: Option<String>,
    pub os: Option<String>,
    pub id: Option<String>,
    pub coordinate_system: Option<String>,
}

impl TemplateField {
    fn vector2_to_js(x: f64, y: f64, context: &mut Context) -> JsValue {
        let arr = JsArray::from_iter([JsValue::new(x), JsValue::new(y)], context);
        let obj: JsObject = arr.into();

        let prop_desc = PropertyDescriptor::builder()
            .writable(false)
            .enumerable(false)
            .configurable(false);

        obj.define_property_or_throw(JsString::from("x"), prop_desc.clone().value(x), context)
            .unwrap();
        obj.define_property_or_throw(JsString::from("y"), prop_desc.value(y), context)
            .unwrap();

        obj.into()
    }

    fn vector3_to_js(x: f64, y: f64, z: f64, context: &mut Context) -> JsValue {
        let arr = JsArray::from_iter([JsValue::new(x), JsValue::new(y), JsValue::new(z)], context);
        let obj: JsObject = arr.into();

        let prop_desc = PropertyDescriptor::builder()
            .writable(false)
            .enumerable(false)
            .configurable(false);

        obj.define_property_or_throw(JsString::from("x"), prop_desc.clone().value(x), context)
            .unwrap();
        obj.define_property_or_throw(JsString::from("y"), prop_desc.clone().value(y), context)
            .unwrap();
        obj.define_property_or_throw(JsString::from("z"), prop_desc.value(z), context)
            .unwrap();

        obj.into()
    }

    fn color_to_js(r: f64, g: f64, b: f64, context: &mut Context) -> JsValue {
        let arr = JsArray::from_iter([JsValue::new(r), JsValue::new(g), JsValue::new(b)], context);
        let obj: JsObject = arr.into();

        let prop_desc = PropertyDescriptor::builder()
            .writable(false)
            .enumerable(false)
            .configurable(false);

        obj.define_property_or_throw(JsString::from("r"), prop_desc.clone().value(r), context)
            .unwrap();
        obj.define_property_or_throw(JsString::from("g"), prop_desc.clone().value(g), context)
            .unwrap();
        obj.define_property_or_throw(JsString::from("b"), prop_desc.value(b), context)
            .unwrap();

        obj.into()
    }

    fn rotation_to_js(x: f64, y: f64, z: f64, a: f64, context: &mut Context) -> JsValue {
        let arr = JsArray::from_iter(
            [
                JsValue::new(x),
                JsValue::new(y),
                JsValue::new(z),
                JsValue::new(a),
            ],
            context,
        );
        let obj: JsObject = arr.into();

        let prop_desc = PropertyDescriptor::builder()
            .writable(false)
            .enumerable(false)
            .configurable(false);

        obj.define_property_or_throw(JsString::from("x"), prop_desc.clone().value(x), context)
            .unwrap();
        obj.define_property_or_throw(JsString::from("y"), prop_desc.clone().value(y), context)
            .unwrap();
        obj.define_property_or_throw(JsString::from("z"), prop_desc.clone().value(z), context)
            .unwrap();
        obj.define_property_or_throw(JsString::from("angle"), prop_desc.value(a), context)
            .unwrap();

        obj.into()
    }

    pub fn to_js_value(&self, context: &mut Context) -> JsValue {
        match self {
            TemplateField::SFBool(v) => JsValue::Boolean(*v),
            TemplateField::SFInt32(v) => JsValue::Integer(*v),
            TemplateField::SFFloat(v) => JsValue::Rational(*v),
            TemplateField::SFString(v) => JsValue::String(JsString::from(v.as_str())),
            TemplateField::SFVec2f(x, y) => Self::vector2_to_js(*x, *y, context),
            TemplateField::SFVec3f(x, y, z) => Self::vector3_to_js(*x, *y, *z, context),
            TemplateField::SFColor(r, g, b) => Self::color_to_js(*r, *g, *b, context),
            TemplateField::SFRotation(x, y, z, a) => Self::rotation_to_js(*x, *y, *z, *a, context),
            TemplateField::SFNode(v) => JsValue::String(JsString::from(v.as_str())),
            TemplateField::MFBool(v) => {
                let arr = v.iter().map(|&b| JsValue::Boolean(b)).collect::<Vec<_>>();
                JsArray::from_iter(arr, context).into()
            }
            TemplateField::MFInt32(v) => {
                let arr = v.iter().map(|&i| JsValue::Integer(i)).collect::<Vec<_>>();
                JsArray::from_iter(arr, context).into()
            }
            TemplateField::MFFloat(v) => {
                let arr = v.iter().map(|&f| JsValue::Rational(f)).collect::<Vec<_>>();
                JsArray::from_iter(arr, context).into()
            }
            TemplateField::MFString(v) => {
                let arr = v
                    .iter()
                    .map(|s| JsValue::String(JsString::from(s.as_str())))
                    .collect::<Vec<_>>();
                JsArray::from_iter(arr, context).into()
            }
            TemplateField::MFVec2f(v) => {
                let arr = v
                    .iter()
                    .map(|(x, y)| Self::vector2_to_js(*x, *y, context))
                    .collect::<Vec<_>>();
                JsArray::from_iter(arr, context).into()
            }
            TemplateField::MFVec3f(v) => {
                let arr = v
                    .iter()
                    .map(|(x, y, z)| Self::vector3_to_js(*x, *y, *z, context))
                    .collect::<Vec<_>>();
                JsArray::from_iter(arr, context).into()
            }
            TemplateField::MFRotation(v) => {
                let arr = v
                    .iter()
                    .map(|(x, y, z, a)| Self::rotation_to_js(*x, *y, *z, *a, context))
                    .collect::<Vec<_>>();
                JsArray::from_iter(arr, context).into()
            }
            TemplateField::MFColor(v) => {
                let arr = v
                    .iter()
                    .map(|(r, g, b)| Self::color_to_js(*r, *g, *b, context))
                    .collect::<Vec<_>>();
                JsArray::from_iter(arr, context).into()
            }
            TemplateField::MFNode(v) => {
                let arr = v
                    .iter()
                    .map(|s| JsValue::String(JsString::from(s.as_str())))
                    .collect::<Vec<_>>();
                JsArray::from_iter(arr, context).into()
            }
        }
    }
}
