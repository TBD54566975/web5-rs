use web5::dids::data_model::document::Document as InnerDocument;

pub struct Document(pub InnerDocument);

impl Document {
    pub fn new(document: InnerDocument) -> Self {
        Self(document)
    }

    pub fn get_data(&self) -> InnerDocument {
        self.0.clone()
    }
}
