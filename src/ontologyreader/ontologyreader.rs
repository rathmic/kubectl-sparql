pub mod ontologyreader{
    use oxigraph::model::*;
    use oxigraph::MemoryStore;
    use oxigraph::sparql::QueryResults;
    use oxiri::{Iri, IriParseError};

    pub struct MemoryOxigraph{
        pub store: MemoryStore
    }

    impl MemoryOxigraph{
        pub fn new() -> MemoryOxigraph{
             MemoryOxigraph{ store: oxigraph::MemoryStore::new()}
        }
    }

    pub trait Ontologyreader {
        fn add_node(&self,subject: String,predicate: String, object: String) -> Result<(), IriParseError>;
    }

    impl Ontologyreader for MemoryOxigraph {
        fn add_node(&self,subject: String,predicate: String, object: String) -> Result<(), IriParseError>{
            let sub = NamedNode::new(subject)?;
            let pred = NamedNode::new(predicate)?;
            let obj = NamedNode::new(object)?;
            let quad = Quad::new(sub.clone(), pred.clone(), obj.clone(), None);
            self.store.insert(quad.clone());
            Ok(())
        }
    }

    #[test]
    fn test_oxigraph_ontologyreader()
    {
        let reader: MemoryOxigraph = MemoryOxigraph::new();
        let lala: String = "http://lala".to_string();
        reader.add_node(lala.clone(),lala.clone(),lala.clone());
        let sub = NamedNode::new(lala.clone()).unwrap();
        let quad = Quad::new(sub.clone(), sub.clone(), sub.clone(), None);
        let results: Vec<Quad> = reader.store.quads_for_pattern(Some(sub.as_ref().into()), None, None, None).collect();
        assert_eq!(vec![quad], results);
    }

    #[test]
    fn test_oxigraph()
    {
        let reader = MemoryOxigraph::new();
    }
}

