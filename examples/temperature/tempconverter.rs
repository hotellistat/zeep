//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.2.0
//!

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_local_definitions)]

use log::{debug, trace, warn};
use std::{
    io::{Read, Write},
    rc::Rc,
};
use yaserde_derive::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
pub mod mod_tem {
    use super::*;
    use restrictions::CheckRestrictions;
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tem", namespaces = {"tem" = "http://learnwebservices.com/services/tempconverter"}, rename = "celsiusToFahrenheitRequest")]
    pub struct CelsiusToFahrenheitRequest {
        #[yaserde(prefix = "tem", rename = "TemperatureInCelsius")]
        pub temperature_in_celsius: f64,
    }
    impl restrictions::CheckRestrictions for CelsiusToFahrenheitRequest {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.temperature_in_celsius.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tem", namespaces = {"tem" = "http://learnwebservices.com/services/tempconverter"}, rename = "celsiusToFahrenheitResponse")]
    pub struct CelsiusToFahrenheitResponse {
        #[yaserde(prefix = "tem", rename = "TemperatureInFahrenheit")]
        pub temperature_in_fahrenheit: f64,
    }
    impl restrictions::CheckRestrictions for CelsiusToFahrenheitResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.temperature_in_fahrenheit
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tem", namespaces = {"tem" = "http://learnwebservices.com/services/tempconverter"}, rename = "fahrenheitToCelsiusRequest")]
    pub struct FahrenheitToCelsiusRequest {
        #[yaserde(prefix = "tem", rename = "TemperatureInFahrenheit")]
        pub temperature_in_fahrenheit: f64,
    }
    impl restrictions::CheckRestrictions for FahrenheitToCelsiusRequest {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.temperature_in_fahrenheit
                .check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tem", namespaces = {"tem" = "http://learnwebservices.com/services/tempconverter"}, rename = "fahrenheitToCelsiusResponse")]
    pub struct FahrenheitToCelsiusResponse {
        #[yaserde(prefix = "tem", rename = "TemperatureInCelsius")]
        pub temperature_in_celsius: f64,
    }
    impl restrictions::CheckRestrictions for FahrenheitToCelsiusResponse {
        fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
            self.temperature_in_celsius.check_restrictions(restrictions.clone())?;
            drop(restrictions);
            Ok(())
        }
    }
}

/* FahrenheitToCelsius */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tem", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct FahrenheitToCelsiusInputEnvelopeBody {
    #[yaserde(prefix = "tem", rename = "FahrenheitToCelsiusRequest")]
    pub fahrenheit_to_celsius_request: mod_tem::FahrenheitToCelsiusRequest,
}
impl restrictions::CheckRestrictions for FahrenheitToCelsiusInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.fahrenheit_to_celsius_request.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct FahrenheitToCelsiusInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: FahrenheitToCelsiusInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for FahrenheitToCelsiusInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tem", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct FahrenheitToCelsiusOutputEnvelopeBody {
    #[yaserde(prefix = "tem", rename = "FahrenheitToCelsiusResponse")]
    pub fahrenheit_to_celsius_response: mod_tem::FahrenheitToCelsiusResponse,
}
impl restrictions::CheckRestrictions for FahrenheitToCelsiusOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.fahrenheit_to_celsius_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct FahrenheitToCelsiusOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: FahrenheitToCelsiusOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for FahrenheitToCelsiusOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}

/* CelsiusToFahrenheit */

#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tem", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct CelsiusToFahrenheitInputEnvelopeBody {
    #[yaserde(prefix = "tem", rename = "CelsiusToFahrenheitRequest")]
    pub celsius_to_fahrenheit_request: mod_tem::CelsiusToFahrenheitRequest,
}
impl restrictions::CheckRestrictions for CelsiusToFahrenheitInputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.celsius_to_fahrenheit_request.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct CelsiusToFahrenheitInputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CelsiusToFahrenheitInputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CelsiusToFahrenheitInputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tem", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct CelsiusToFahrenheitOutputEnvelopeBody {
    #[yaserde(prefix = "tem", rename = "CelsiusToFahrenheitResponse")]
    pub celsius_to_fahrenheit_response: mod_tem::CelsiusToFahrenheitResponse,
}
impl restrictions::CheckRestrictions for CelsiusToFahrenheitOutputEnvelopeBody {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.celsius_to_fahrenheit_response.check_restrictions(restrictions)
    }
}
#[derive(Debug, Default, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "soapenv", rename = "Envelope", namespaces = { "soapenv" = "http://schemas.xmlsoap.org/soap/envelope/", "tem" = "http://learnwebservices.com/services/tempconverter" })]
pub struct CelsiusToFahrenheitOutputEnvelope {
    #[yaserde(prefix = "soapenv", rename = "Body")]
    pub body: CelsiusToFahrenheitOutputEnvelopeBody,
}
impl restrictions::CheckRestrictions for CelsiusToFahrenheitOutputEnvelope {
    fn check_restrictions(&self, restrictions: Option<Rc<restrictions::Restrictions>>) -> error::SoapResult<()> {
        self.body.check_restrictions(restrictions)
    }
}
pub struct TempConverterEndpointService {
    pub client: reqwest::Client,
    pub location: String,
    pub credentials: Option<(String, String)>,
}
impl TempConverterEndpointService {
    pub fn new(credentials: Option<(String, String)>) -> Self {
        Self {
            client: reqwest::Client::new(),
            location: "https://apps.learnwebservices.com/services/tempconverter".to_string(),
            credentials,
        }
    }

    pub async fn fahrenheit_to_celsius(
        &self,
        req: FahrenheitToCelsiusInputEnvelope,
    ) -> error::SoapResult<FahrenheitToCelsiusOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }

    pub async fn celsius_to_fahrenheit(
        &self,
        req: CelsiusToFahrenheitInputEnvelope,
    ) -> error::SoapResult<CelsiusToFahrenheitOutputEnvelope> {
        let credentials = self.credentials.as_ref().map(|(u, p)| (u.as_str(), p.as_str()));
        helpers::send_soap_request_using_client(&self.client, &self.location, credentials, req).await
    }
}
pub mod error {
    #![allow(dead_code)]

    use std::{error::Error, num::ParseIntError};

    #[derive(Debug)]
    pub enum SoapError {
        YaserdeError(String),
        Http(reqwest::Error),
        Restriction(String),
    }

    pub type SoapResult<T> = Result<T, SoapError>;

    impl std::fmt::Display for SoapError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SoapError::YaserdeError(e) => write!(f, "Yaserde error: {e}"),
                SoapError::Http(e) => write!(f, "HTTP error: {e}"),
                SoapError::Restriction(e) => write!(f, "Restriction not met: {e}"),
            }
        }
    }

    impl Error for SoapError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                SoapError::YaserdeError(_) | SoapError::Restriction(_) => None,
                SoapError::Http(e) => Some(e),
            }
        }

        fn cause(&self) -> Option<&dyn Error> {
            self.source()
        }
    }

    impl From<reqwest::Error> for SoapError {
        fn from(e: reqwest::Error) -> Self {
            SoapError::Http(e)
        }
    }

    impl From<ParseIntError> for SoapError {
        fn from(err: ParseIntError) -> Self {
            SoapError::Restriction(format!("invalid restriction in XSD: {err}"))
        }
    }
}

mod helpers {
    #![allow(dead_code)]

    use super::{
        error::{SoapError, SoapResult},
        restrictions::CheckRestrictions,
    };
    use reqwest::Client;
    use std::fmt;
    use yaserde::{YaDeserialize, YaSerialize};

    pub(super) async fn send_soap_request<YI, YO, U, P>(
        url: &str,
        credentials: Option<(U, P)>,
        req: YI,
    ) -> SoapResult<YO>
    where
        YI: YaSerialize + CheckRestrictions,
        YO: YaDeserialize,
        U: fmt::Display,
        P: fmt::Display,
    {
        let client = Client::new();
        send_soap_request_using_client(&client, url, credentials, req).await
    }

    pub(super) async fn send_soap_request_using_client<YI, YO, U, P>(
        client: &Client,
        url: &str,
        credentials: Option<(U, P)>,
        req: YI,
    ) -> SoapResult<YO>
    where
        YI: YaSerialize + CheckRestrictions,
        YO: YaDeserialize,
        U: fmt::Display,
        P: fmt::Display,
    {
        req.check_restrictions(None)?;
        let body = yaserde::ser::to_string(&req).map_err(SoapError::YaserdeError)?;
        let mut req = client.post(url).body(body);
        if let Some((username, password)) = credentials {
            req = req.basic_auth(username, Some(password));
        }
        let response = req.send().await?;
        response.error_for_status_ref()?;
        let response_body = response.text().await?;
        let response = yaserde::de::from_str(&response_body).map_err(SoapError::YaserdeError)?;
        Ok(response)
    }
}

pub mod restrictions {
    use super::error::{SoapError, SoapResult};
    use std::rc::Rc;

    #[derive(Debug, PartialEq, Default)]
    pub struct Restrictions {
        pub min_inclusive: Option<i32>,
        pub max_inclusive: Option<i32>,
        pub min_exclusive: Option<i32>,
        pub max_exclusive: Option<i32>,
        pub length: Option<usize>,
        pub min_length: Option<usize>,
        pub max_length: Option<usize>,
        pub enumeration: Option<Vec<String>>,
    }

    pub trait CheckRestrictions {
        fn check_restrictions(&self, _restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            Ok(())
        }
    }

    impl<C> CheckRestrictions for Vec<C>
    where
        C: CheckRestrictions,
    {
        fn check_restrictions(&self, restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            for c in self {
                c.check_restrictions(restrictions.clone())?;
            }
            Ok(())
        }
    }

    impl<C> CheckRestrictions for Option<C>
    where
        C: CheckRestrictions,
    {
        fn check_restrictions(&self, restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            if let Some(c) = self {
                c.check_restrictions(restrictions)?;
            }
            Ok(())
        }
    }

    impl CheckRestrictions for i32 {
        fn check_restrictions(&self, restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            if let Some(restrictions) = restrictions {
                if let Some(min_inclusive) = restrictions.min_inclusive {
                    if *self <= min_inclusive {
                        return Err(SoapError::Restriction("minInclusive restriction not met".to_string()));
                    }
                }

                if let Some(max_inclusive) = restrictions.max_inclusive {
                    if max_inclusive <= *self {
                        return Err(SoapError::Restriction("maxInclusive restriction not met".to_string()));
                    }
                }

                if let Some(min_exclusive) = restrictions.min_exclusive {
                    if *self < min_exclusive {
                        return Err(SoapError::Restriction("minExclusive restriction not met".to_string()));
                    }
                }

                if let Some(max_exclusive) = restrictions.max_exclusive {
                    if max_exclusive < *self {
                        return Err(SoapError::Restriction("maxExclusive restriction not met".to_string()));
                    }
                }
            }

            Ok(())
        }
    }

    macro_rules! impl_check_restrictions_for_int {
    ($($t:ty),*) => {
        $(
            impl CheckRestrictions for $t {
                fn check_restrictions(&self, restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
                    let value = i32::try_from(*self).map_err(|e| SoapError::Restriction(e.to_string()))?;
                    value.check_restrictions(restrictions)
                }
            }
        )*
    }
}

    impl_check_restrictions_for_int!(i8, u8, i16, u16, u32, i64, u64);

    impl CheckRestrictions for bool {
        fn check_restrictions(&self, _restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            // TODO: check restrictions
            Ok(())
        }
    }

    impl CheckRestrictions for f32 {
        fn check_restrictions(&self, _restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            // TODO: check restrictions
            Ok(())
        }
    }

    impl CheckRestrictions for f64 {
        fn check_restrictions(&self, _restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            // TODO: check restrictions
            Ok(())
        }
    }

    impl CheckRestrictions for String {
        fn check_restrictions(&self, restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            let Some(restrictions) = restrictions else {
                return Ok(());
            };

            let s_len = self.chars().count();

            if let Some(min_length) = restrictions.min_length {
                if s_len < min_length {
                    return Err(SoapError::Restriction("minLength restriction not met".to_string()));
                }
            }

            if let Some(max_length) = restrictions.max_length {
                if max_length < s_len {
                    return Err(SoapError::Restriction("maxLength restriction not met".to_string()));
                }
            }

            if let Some(length) = restrictions.length {
                if length != s_len {
                    return Err(SoapError::Restriction("length restriction not met".to_string()));
                }
            }

            // Enumerations
            if let Some(enumeration) = restrictions.enumeration.as_ref() {
                if !enumeration.contains(self) {
                    return Err(SoapError::Restriction("enumeration restriction not met".to_string()));
                }
            }

            // Number-type checks; see if any of these are set
            if restrictions.min_inclusive.is_none()
                && restrictions.max_inclusive.is_none()
                && restrictions.min_exclusive.is_none()
                && restrictions.max_exclusive.is_none()
            {
                return Ok(());
            }

            let value = self.parse::<i32>()?;

            if let Some(min_inclusive) = restrictions.min_inclusive {
                if value <= min_inclusive {
                    return Err(SoapError::Restriction("minInclusive restriction not met".to_string()));
                }
            }

            if let Some(max_inclusive) = restrictions.max_inclusive {
                if max_inclusive <= value {
                    return Err(SoapError::Restriction("maxInclusive restriction not met".to_string()));
                }
            }

            if let Some(min_exclusive) = restrictions.min_exclusive {
                if value < min_exclusive {
                    return Err(SoapError::Restriction("minExclusive restriction not met".to_string()));
                }
            }

            if let Some(max_exclusive) = restrictions.max_exclusive {
                if max_exclusive < value {
                    return Err(SoapError::Restriction("maxExclusive restriction not met".to_string()));
                }
            }

            Ok(())
        }
    }
}

/// This module contains the `MultiRef` type which is a wrapper around `Arc<RwLock<T>>` that implements `YaDeserialize` and `YaSerialize` for `T` and allows for multiple references to the same object.
/// Inspired by [this](https://github.com/media-io/yaserde/issues/165#issuecomment-1810243674) comment on the yaserde repository.
/// Needs `xml-rs`, `tokio` and `yaserde` as dependencies.
pub mod multi_ref {
    use super::{
        error::SoapResult,
        restrictions::{CheckRestrictions, Restrictions},
    };
    use std::{ops::Deref, rc::Rc, sync::Arc};
    use yaserde::{YaDeserialize, YaSerialize};

    pub struct MultiRef<T> {
        inner: Arc<T>,
    }

    impl<T> MultiRef<T> {
        #[allow(dead_code)]
        pub fn new(inner: T) -> Self {
            Self { inner: Arc::new(inner) }
        }
    }

    impl<C> CheckRestrictions for MultiRef<C>
    where
        C: CheckRestrictions,
    {
        fn check_restrictions(&self, restrictions: Option<Rc<Restrictions>>) -> SoapResult<()> {
            self.inner.check_restrictions(restrictions)
        }
    }

    impl<T: YaDeserialize> YaDeserialize for MultiRef<T> {
        fn deserialize<R: std::io::prelude::Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
            let inner = T::deserialize(reader)?;
            Ok(Self { inner: Arc::new(inner) })
        }
    }

    impl<T: YaSerialize> YaSerialize for MultiRef<T> {
        fn serialize<W: std::io::prelude::Write>(
            &self,
            writer: &mut yaserde::ser::Serializer<W>,
        ) -> Result<(), String> {
            self.inner.serialize(writer)?;
            Ok(())
        }

        fn serialize_attributes(
            &self,
            attributes: Vec<xml::attribute::OwnedAttribute>,
            namespace: xml::namespace::Namespace,
        ) -> Result<(Vec<xml::attribute::OwnedAttribute>, xml::namespace::Namespace), String> {
            self.inner.serialize_attributes(attributes, namespace)
        }
    }

    impl<T: Default> Default for MultiRef<T> {
        fn default() -> Self {
            Self { inner: Arc::default() }
        }
    }

    impl<T: Clone> Clone for MultiRef<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }

    impl<T: std::fmt::Debug> std::fmt::Debug for MultiRef<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.fmt(f)
        }
    }

    impl<T> Deref for MultiRef<T> {
        type Target = Arc<T>;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }
}
