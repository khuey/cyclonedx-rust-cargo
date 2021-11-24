use std::io;

use serde::Serialize;
use xml_writer::XmlWriter;

use crate::traits::ToXml;

#[derive(Serialize, Clone, Copy)]
pub struct Tool {
    #[serde(skip_serializing_if = "Option::is_none")]
    vendor: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<&'static str>,
}

impl ToXml for Tool {
    fn to_xml<W: io::Write>(&self, xml: &mut XmlWriter<W>) -> io::Result<()> {
        xml.begin_elem("tool")?;
        if let Some(vendor) = self.vendor {
            xml.begin_elem("vendor")?;
            xml.text(vendor)?;
            xml.end_elem()?;
        }
        if let Some(name) = self.name {
            xml.begin_elem("name")?;
            xml.text(name)?;
            xml.end_elem()?;
        }
        if let Some(version) = self.version {
            xml.begin_elem("version")?;
            xml.text(version)?;
            xml.end_elem()?;
        }
        xml.end_elem()
    }
}

impl ToXml for Vec<Tool> {
    fn to_xml<W: io::Write>(&self, xml: &mut XmlWriter<W>) -> io::Result<()> {
        xml.begin_elem("tools")?;

        for tool in self {
            tool.to_xml(xml)?;
        }

        xml.end_elem()
    }
}

pub const CARGO_CYCLONEDX: Tool = Tool {
    vendor: Some("cyclonedx.org"),
    name: Some("cyclonedx-rust-cargo"),
    version: option_env!("CARGO_PKG_VERSION"),
};
