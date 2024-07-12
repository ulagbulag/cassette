use serde::{Deserialize, Serialize, Serializer};

use crate::{cassette::CassetteCrd, component::CassetteComponentCrd};

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Document {
    Cassette(CassetteCrd),
    CassetteComponent(CassetteComponentCrd),
}

impl Serialize for Document {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum DocumentSer<'a> {
            Cassette(&'a CassetteCrd),
            CassetteComponent(&'a CassetteComponentCrd),
        }

        let document = match self {
            Document::Cassette(cr) => DocumentSer::Cassette(cr),
            Document::CassetteComponent(cr) => DocumentSer::CassetteComponent(cr),
        };
        document.serialize(serializer)
    }
}
