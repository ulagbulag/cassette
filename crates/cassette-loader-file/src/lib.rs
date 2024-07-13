use cassette_core::document::Document;
use cassette_loader_core::CassetteDB;
use kube::ResourceExt;
use once_cell::sync::OnceCell;
use sha2::{Digest, Sha256};
use tracing::error;
use uuid::Uuid;

pub fn db() -> &'static CassetteDB {
    static DB: OnceCell<CassetteDB> = OnceCell::new();

    DB.get_or_init(|| {
        // create a memory db
        let mut db = CassetteDB::default();

        // parse the documents
        let documents: Vec<Document> = match ::serde_json::from_str(DOCUMENTS) {
            Ok(documents) => documents,
            Err(error) => {
                error!("Failed to parse embedded example documents: {error}");
                return db;
            }
        };

        // insert all documents
        documents.into_iter().for_each(|document| match document {
            Document::Cassette(cr) => db.insert(cr.generate_uid()),
            Document::CassetteComponent(cr) => db.insert_component(cr.generate_uid()),
        });
        db
    });

    DB.get().unwrap()
}

trait GenerateUid {
    fn generate_uid(self) -> Self;
}

impl<T> GenerateUid for T
where
    Self: ResourceExt,
{
    fn generate_uid(mut self) -> Self {
        let name = self.name_any();

        let uid = &mut self.meta_mut().uid;
        if uid.is_none() {
            // create a Sha256 object
            let mut hasher = Sha256::new();

            // write input message
            hasher.update(name);

            // read hash digest and consume hasher
            let hash = hasher.finalize();

            // convert the hash digest prefix into UUID
            *uid = Uuid::from_slice_le(&hash[..16])
                .ok()
                .map(|id| id.to_string());
        }
        self
    }
}

const DOCUMENTS: &str = include_str!(concat!(env!("OUT_DIR"), "/examples.yaml"));
