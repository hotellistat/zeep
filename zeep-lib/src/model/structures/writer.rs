use super::{
    ComplexProps, ElementProps, ElementType, Namespace, Rc, RustFieldType, RustType, SimpleProps, WriteXml,
    WriterError, WriterResult, io, xml_name_to_rust_name,
};
use crate::model::{
    helpers::{write_check_restrictions_footer, write_check_restrictions_header},
    structures::restrictions::Restrictions,
};

impl<W> WriteXml<W> for RustType
where
    W: io::Write,
{
    fn write_xml(&self, writer: &mut W) -> WriterResult<()> {
        match self {
            RustType::Ignore => Ok(()),
            RustType::Complex(props) => write_complex_type(writer, props),
            RustType::Simple(props) => write_simple_type(writer, props),
            RustType::Element(props) => {
                let ElementProps { xml_name, element_type } = &**props;
                let rust_name = xml_name_to_rust_name(xml_name);

                match element_type {
                    ElementType::RustType(rust_type) => {
                        if let Some(segment) = rust_type.to_string().split(':').next_back()
                            && segment == rust_name
                        {
                            // NOOP
                            return Ok(());
                        }

                        writeln!(writer, "pub type {rust_name} = {rust_type};")?;
                    }
                    ElementType::ComplexType(props) => {
                        write_complex_type(writer, props)?;
                    }
                    ElementType::Unsupported => {
                        // NOOP
                    }
                }

                Ok(())
            }
        }
    }
}

fn write_simple_type<W>(writer: &mut W, props: &SimpleProps) -> WriterResult<()>
where
    W: io::Write,
{
    let SimpleProps {
        xml_name,
        rust_type,
        target_namespace,
        restrictions,
        comment,
    } = &props;

    let rust_name = xml_name_to_rust_name(xml_name);
    if let Some(segment) = rust_type.to_string().split(':').next_back()
        && segment == rust_name
    {
        // NOOP
        return Ok(());
    }

    // for now, write this as a type alias; we may want to change this to a newtype
    // in the future
    if let Some(comment) = comment {
        comment.split('\n').for_each(|line| {
            writeln!(writer, "/// {line}").unwrap();
        });
    }

    write_type_alias(
        writer,
        xml_name,
        &rust_name,
        rust_type,
        target_namespace.as_ref(),
        restrictions.as_ref(),
    )?;
    Ok(())
}

fn write_type_alias<W>(
    writer: &mut W,
    xml_name: &str,
    rust_name: &str,
    rust_type: &RustFieldType,
    target_namespace: Option<&Rc<Namespace>>,
    restrictions: Option<&Restrictions>,
) -> Result<(), WriterError>
where
    W: io::Write,
{
    writeln!(writer, "#[derive(Debug, Default, YaSerialize, YaDeserialize)]")?;
    if let Some(tns) = &target_namespace {
        let namespaces = format!("\"{}\" = \"{}\"", tns.abbreviation, tns.namespace);
        writeln!(
            writer,
            "#[yaserde(prefix = \"{}\", namespaces = {{{}}}, rename = \"{}\")]",
            tns.abbreviation, namespaces, xml_name
        )?;
    }
    writeln!(writer, "pub struct {rust_name} {{")?;
    if rust_type.is_string() {
        writeln!(writer, "    #[yaserde(text = true)]")?;
        writeln!(writer, "    pub value: {rust_type}")?;
    } else if rust_type.is_other() {
        writeln!(writer, "    #[yaserde(flatten = true)]")?;
        writeln!(writer, "    pub value: {rust_type}")?;
    } else {
        // note: flatten is not supported for other types
        writeln!(writer, "    #[yaserde(text = true)]")?;
        writeln!(writer, "    pub value: String")?;
    }
    writeln!(writer, "}}")?;

    // Write the restriction check
    write_check_restrictions_header(writer, rust_name, restrictions)?;
    writeln!(writer, "     self.value.check_restrictions(restrictions)")?;
    write_check_restrictions_footer(writer)?;

    Ok(())
}

fn write_complex_type<W>(writer: &mut W, props: &ComplexProps) -> WriterResult<()>
where
    W: io::Write,
{
    let ComplexProps {
        xml_name,
        fields,
        target_namespace,
        comment,
    } = &props;

    let rust_name = xml_name_to_rust_name(xml_name);

    if let Some(comment) = comment {
        comment.split('\n').for_each(|line| {
            writeln!(writer, "/// {line}").unwrap();
        });
    }

    writeln!(writer, "#[derive(Debug, Default, YaSerialize, YaDeserialize)]")?;
    if let Some(tns) = &target_namespace {
        let namespaces = format!("\"{}\" = \"{}\"", tns.abbreviation, tns.namespace);
        writeln!(
            writer,
            "#[yaserde(prefix = \"{}\", namespaces = {{{}}}, rename = \"{}\")]",
            tns.abbreviation, namespaces, xml_name
        )?;
    }
    writeln!(writer, "pub struct {rust_name} {{")?;
    for field in fields {
        field.write_xml(writer)?;
    }
    writeln!(writer, "}}")?;

    // Write the restriction check
    write_check_restrictions_header(writer, &rust_name, None)?;
    for field in fields {
        let field_name = &field.rust_name;
        writeln!(
            writer,
            "     self.{field_name}.check_restrictions(restrictions.clone())?;"
        )?;
    }

    writeln!(writer, "    drop(restrictions);")?;
    writeln!(writer, "    Ok(())")?;
    write_check_restrictions_footer(writer)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::field::{Field, RustFieldType};

    #[test]
    fn can_write_a_struct_type_to_rust() {
        let mut writer = Vec::new();
        let props = prep_struct_props(None);
        let rust_type = RustType::Complex(props.into());
        rust_type.write_xml(&mut writer).unwrap();

        let expected = "/// A person\n#[derive(Debug, Default, YaSerialize, YaDeserialize)]\npub struct Person {\n    #[yaserde(rename = \"name\")]\n    pub name: String,\n    #[yaserde(rename = \"age\")]\n    pub age: i32,\n}\nimpl restrictions::CheckRestrictions for Person {\n  fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()>  {\n     self.name.check_restrictions(restrictions.clone())?;\n     self.age.check_restrictions(restrictions.clone())?;\n    drop(restrictions);\n    Ok(())\n  }\n}\n";
        assert_eq!(String::from_utf8(writer).unwrap(), expected);
    }

    #[test]
    fn can_write_a_struct_type_with_namespace_to_rust() {
        const EXPECTED: &str = "/// A person\n#[derive(Debug, Default, YaSerialize, YaDeserialize)]\n#[yaserde(prefix = \"ex\", namespaces = {\"ex\" = \"http://example.com\"}, rename = \"Person\")]\npub struct Person {\n    #[yaserde(rename = \"name\")]\n    pub name: String,\n    #[yaserde(rename = \"age\")]\n    pub age: i32,\n}\nimpl restrictions::CheckRestrictions for Person {\n  fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()>  {\n     self.name.check_restrictions(restrictions.clone())?;\n     self.age.check_restrictions(restrictions.clone())?;\n    drop(restrictions);\n    Ok(())\n  }\n}\n";

        let mut writer = Vec::new();
        let props = prep_struct_props(Some(Rc::new(Namespace {
            namespace: "http://example.com".to_string(),
            abbreviation: "ex".to_string(),
            rust_mod_name: "mod_ex".to_string(),
        })));
        let rust_type = RustType::Complex(props.into());
        rust_type.write_xml(&mut writer).unwrap();
        assert_eq!(String::from_utf8(writer).unwrap(), EXPECTED);
    }

    fn prep_struct_props(target_namespace: Option<Rc<Namespace>>) -> ComplexProps {
        ComplexProps {
            xml_name: "Person".to_string(),
            fields: vec![
                Field {
                    xml_name: "name".to_string(),
                    rust_name: "name".to_string(),
                    rust_type: RustFieldType::String,
                    ..Default::default()
                },
                Field {
                    xml_name: "age".to_string(),
                    rust_name: "age".to_string(),
                    rust_type: RustFieldType::I32,
                    ..Default::default()
                },
            ],
            target_namespace,
            comment: Some("A person".to_string()),
        }
    }

    #[test]
    fn can_write_a_simple_type_to_rust() {
        const EXPECTED: &str = "/// A person\n#[derive(Debug, Default, YaSerialize, YaDeserialize)]\npub struct Person {\n    #[yaserde(text = true)]\n    pub value: String\n}\nimpl restrictions::CheckRestrictions for Person {\n  fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()>  {\n     self.value.check_restrictions(restrictions)\n  }\n}\n";
        let mut writer = Vec::new();
        let props = prep_simple_props(None);
        let rust_type = RustType::Simple(props);
        rust_type.write_xml(&mut writer).unwrap();
        assert_eq!(String::from_utf8(writer).unwrap(), EXPECTED);
    }

    fn prep_simple_props(target_namespace: Option<Rc<Namespace>>) -> Box<SimpleProps> {
        SimpleProps {
            xml_name: "Person".to_string(),
            rust_type: RustFieldType::String,
            target_namespace,
            restrictions: None,
            comment: Some("A person".to_string()),
        }
        .into()
    }
}
