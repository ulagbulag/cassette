use cassette_core::document::Document;
use cassette_loader_core::CassetteDB;
use kube::ResourceExt;
use once_cell::sync::OnceCell;
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
        documents
            .into_iter()
            .enumerate()
            .for_each(|(id, document)| match document {
                Document::Cassette(cr) => db.insert(cr.generate_uid(id)),
                Document::CassetteComponent(cr) => db.insert_component(cr.generate_uid(id)),
            });
        db
    });

    DB.get().unwrap()
}

trait GenerateUid {
    fn generate_uid(self, id: usize) -> Self;
}

impl<T> GenerateUid for T
where
    Self: ResourceExt,
{
    fn generate_uid(mut self, id: usize) -> Self {
        let uid = &mut self.meta_mut().uid;
        if uid.is_none() {
            *uid = id
                .try_into()
                .ok()
                .map(Uuid::from_u128)
                .map(|id| id.to_string());
        }
        self
    }
}

const DOCUMENTS: &str = include_str!(concat!(env!("OUT_DIR"), "/examples.yaml"));
