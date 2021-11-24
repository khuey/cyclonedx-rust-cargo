/*
 * This file is part of CycloneDX Rust Cargo.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * SPDX-License-Identifier: Apache-2.0
 * Copyright (c) OWASP Foundation. All Rights Reserved.
 */
use std::{convert::TryFrom, io};

use cargo::core::Package;
use serde::{Serialize, Serializer};
use spdx::{Expression, LicenseId, ParseMode};
use thiserror::Error;
use xml_writer::XmlWriter;

use crate::traits::ToXml;

fn serialize_license_id<S: Serializer>(license_id: &LicenseId, s: S) -> Result<S::Ok, S::Error> {
    license_id.full_name.serialize(s)
}

#[derive(Serialize)]
#[serde(untagged, rename_all = "lowercase")]
pub enum License {
    Expression(String),
    #[serde(serialize_with = "serialize_license_id")]
    Id(LicenseId),
}

impl From<LicenseId> for License {
    fn from(license_id: LicenseId) -> Self {
        Self::Id(license_id)
    }
}

pub fn try_parse_licenses(pkg: &Package) -> Result<Vec<License>, LicenseError> {
    let expression = pkg
        .manifest()
        .metadata()
        .license
        .as_ref()
        .ok_or_else(|| LicenseError::NoLicenseProvidedError)?
        .as_str();
    match Expression::parse_mode(expression, ParseMode::Lax) {
        Ok(parsed_licenses) => {
            let mut licenses = Vec::new();
            for expr_req in parsed_licenses.requirements() {
                if expr_req.req.exception.is_some() {
                    log::warn!("Unable to structure '{}' as an SPDX license expression: exceptions are not yet handled",
                               expression);
                    licenses.clear();
                    break;
                }

                if let Some(license_id) = expr_req.req.license.id() {
                    licenses.push(license_id.into());
                } else {
                    log::warn!("Unable to structure '{}' as an SPDX license expression: referencers are not yet handled",
                               expression);
                    licenses.clear();
                    break;
                }
            }

            if !licenses.is_empty() {
                return Ok(licenses);
            }
        }
        Err(parse_error) => {
            log::warn!(
                "Unable to parse '{}' as an SPDX license expression: {}",
                expression,
                parse_error
            );
        }
    }

    Ok(vec![License::Expression(expression.to_string())])
}

#[derive(Debug, Error)]
pub enum LicenseError {
    #[error("No license was found in the package manifest")]
    NoLicenseProvidedError,
}

impl<'a> TryFrom<&'a cargo_metadata::Package> for License {
    type Error = LicenseError;

    fn try_from(pkg: &'a cargo_metadata::Package) -> Result<Self, Self::Error> {
        let expression = pkg
            .license
            .as_ref()
            .ok_or_else(|| LicenseError::NoLicenseProvidedError)?
            .to_string();
        Ok(Self::Expression(expression))
    }
}

impl ToXml for License {
    fn to_xml<W: io::Write>(&self, xml: &mut XmlWriter<W>) -> io::Result<()> {
        match self {
            License::Expression(expr) => {
                xml.begin_elem("expression")?;
                xml.text(expr.trim())?;
                xml.end_elem()
            }
            License::Id(id) => {
                xml.begin_elem("license")?;
                xml.begin_elem("id")?;
                xml.text(id.full_name)?;
                xml.end_elem()?;
                xml.end_elem()
            }
        }
    }
}

impl ToXml for Vec<License> {
    fn to_xml<W: io::Write>(&self, xml: &mut XmlWriter<W>) -> io::Result<()> {
        if self.len() > 0 {
            xml.begin_elem("licenses")?;

            for license in self {
                license.to_xml(xml)?;
            }

            xml.end_elem()?;
        }

        Ok(())
    }
}
