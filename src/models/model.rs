use crate::models::{Field, MultiField, ModelField, DataField, MultiModelField};



#[derive(Debug, PartialEq)]
pub struct Model {
    pub dtype: String,
    pub id: String,
    pub deleted_state: String,
    pub decoding_confidence: String,
    pub is_related: String,
    pub extraction_id: u64,
    pub fields: Vec<Field>,
    pub multi_model_fields: Vec<MultiModelField>,
    pub model_fields: Vec<ModelField>,
    pub data_fields: Vec<DataField>,
    pub multi_fields: Vec<MultiField>,
}


