use crate::{
    error::{WriterError, WriterResult},
    model::{doc::RustDocument, node::RustNode, TryFromNode},
};
use roxmltree::{Document, Node};
use std::{collections::HashMap, fmt::Display, sync::atomic::AtomicBool};

const WELL_KNOWN_NAMESPACES: &[&str] = &[
    "http://www.w3.org/XML/1998/namespace",
    "http://www.w3.org/2001/XMLSchema",
    "http://www.w3.org/2001/XMLSchema-instance",
    "http://www.w3.org/2007/XMLSchema-versioning",
];

pub struct XmlReader;

pub struct File {
    xml: String,
    processed: AtomicBool,
}

impl File {
    pub fn new(xml: String) -> Self {
        File {
            xml,
            processed: AtomicBool::new(false),
        }
    }
}

pub struct Files {
    map: HashMap<String, File>,
}

impl Files {
    pub fn new<F, X>(file_name: F, xml: X) -> Self
    where
        F: Display,
        X: Display,
    {
        Files {
            map: HashMap::from([(file_name.to_string(), File::new(xml.to_string()))]),
        }
    }

    pub fn add<F, X>(&mut self, file_name: F, xml: X)
    where
        F: Display,
        X: Display,
    {
        let Files { map, .. } = self;
        map.insert(file_name.to_string(), File::new(xml.to_string()));
    }
}

impl XmlReader {
    pub fn read_xml(&self, file: &File, file_name: &str, files: &Files) -> WriterResult<Vec<RustNode>> {
        if file.processed.load(std::sync::atomic::Ordering::SeqCst) {
            return Ok(vec![]);
        }

        let xml = &file.xml;
        let doc = roxmltree::Document::parse(xml)
            .map_err(|e| WriterError::new(format!("Unable to parse file {file_name}: {e}")))?;

        let mut rust_doc = RustDocument::new();
        let mut nodes = vec![];
        for child in doc.root().children() {
            let child_nodes = self.read(child, files, &mut rust_doc)?;
            nodes.extend(child_nodes);
        }

        file.processed.store(true, std::sync::atomic::Ordering::SeqCst);

        Ok(nodes)
    }

    fn read<'n>(&self, node: Node<'n, 'n>, files: &Files, doc: &mut RustDocument) -> WriterResult<Vec<RustNode>> {
        if !node.is_element() {
            return Ok(vec![]);
        }

        if let Some(target_namespace) = node.attribute("targetNamespace") {
            doc.switch_to_target_namespace(target_namespace);
        }

        let nodes = match node.tag_name().name() {
            "definitions" => self.read_wsdl(node, files, doc)?,
            "schema" => self.read_xsd(node, files, doc)?,
            _ => return Ok(vec![]),
        };

        Ok(nodes)
    }

    fn read_wsdl<'n>(&self, node: Node<'n, 'n>, files: &Files, doc: &mut RustDocument) -> WriterResult<Vec<RustNode>> {
        let mut nodes = vec![];

        for child in node.children() {
            if let Ok(child_node) = RustNode::try_from_node(child, doc) {
                nodes.push(child_node);
            }
        }

        Ok(nodes)
    }

    fn read_xsd<'n>(&self, node: Node<'n, 'n>, files: &Files, doc: &mut RustDocument) -> WriterResult<Vec<RustNode>> {
        let mut nodes = vec![];

        for child in node.children() {
            if child.tag_name().name() == "import" {
                nodes.extend(self.process_import(child, files)?);
                continue;
            }

            if let Ok(child_node) = RustNode::try_from_node(child, doc) {
                nodes.push(child_node);
            }
        }

        Ok(nodes)
    }

    fn process_import(&self, node: Node, files: &Files) -> WriterResult<Vec<RustNode>> {
        let namespace = node.attribute("namespace").ok_or(WriterError::NamespaceMissing)?;

        if WELL_KNOWN_NAMESPACES.contains(&namespace) {
            return Ok(vec![]);
        }

        let Some(schema_location) = node.attribute("schemaLocation") else {
            return Ok(vec![]);
        };

        let file = files
            .map
            .get(schema_location)
            .ok_or_else(|| WriterError::ImportNotFound(schema_location.to_string()))?;

        if file.processed.load(std::sync::atomic::Ordering::Relaxed) {
            return Ok(vec![]);
        }

        let nodes = self.read_xml(file, schema_location, files)?;
        Ok(nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        field::RustFieldType,
        rust_type::{RustType, StructProps},
    };

    #[test]
    fn can_read_a_simple_xsd() {
        const XSD: &str = include_str!("../test-data/single-complex.xsd");
        let files = Files::new("single-complex.xsd", XSD);
        let (file_name, file) = files.map.get_key_value("single-complex.xsd").unwrap();
        let nodes = XmlReader.read_xml(file, file_name, &files).unwrap();
        assert_eq!(nodes.len(), 1);
        let node = nodes.first().unwrap();
        let RustType::Struct(StructProps {
            fields,
            xml_name,
            target_namespace,
        }) = &node.rust_type
        else {
            panic!()
        };

        assert_eq!(xml_name, "InstalledAppType");
        assert_eq!(fields.len(), 14);
        let id_field = fields.first().unwrap();
        assert_eq!(id_field.xml_name, "Id");
        assert_eq!(id_field.rust_type, RustFieldType::String);
        assert!(id_field.optional);
        assert!(!id_field.vec);

        assert_eq!(
            target_namespace.as_ref().unwrap().namespace,
            "http://schemas.microsoft.com/exchange/services/2006/types"
        );
        assert_eq!(target_namespace.as_ref().unwrap().abbreviation, "typ");
    }
}
