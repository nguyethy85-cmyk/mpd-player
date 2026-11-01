use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use quick_xml::{
    encoding::Decoder,
    events::{BytesStart, Event},
    Reader,
};
use serde::Serialize;
use std::{env, fs};

#[derive(Debug, Serialize)]
struct Protection {
    scheme_id_uri: Option<String>,
    value: Option<String>,
    default_kid: Option<String>,
    pssh_base64: Option<String>,
    pssh_bytes: Option<usize>,
}

impl Protection {
    fn new() -> Self {
        Self {
            scheme_id_uri: None,
            value: None,
            default_kid: None,
            pssh_base64: None,
            pssh_bytes: None,
        }
    }
}

fn main() -> Result<()> {
    let path = env::args()
        .nth(1)
        .context("Usage: mpd-inspect <manifest.mpd>")?;
    let xml = fs::read_to_string(&path)
        .with_context(|| format!("Could not read MPD file: {path}"))?;

    let protections = inspect(&xml)?;
    println!("{}", serde_json::to_string_pretty(&protections)?);
    Ok(())
}

fn parse_protection(event: &BytesStart<'_>, decoder: Decoder) -> Result<Protection> {
    let mut item = Protection::new();

    for attribute in event.attributes().with_checks(false) {
        let attribute = attribute?;
        let name = attribute.key.local_name();
        let value = attribute
            .decode_and_unescape_value(decoder)?
            .into_owned();

        match name.as_ref() {
            b"schemeIdUri" => item.scheme_id_uri = Some(value),
            b"value" => item.value = Some(value),
            b"default_KID" => item.default_kid = Some(value),
            _ => {}
        }
    }

    Ok(item)
}

fn inspect(xml: &str) -> Result<Vec<Protection>> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut results = Vec::new();
    let mut current: Option<Protection> = None;
    let mut in_pssh = false;

    loop {
        match reader.read_event()? {
            Event::Start(event) => match event.local_name().as_ref() {
                b"ContentProtection" => {
                    current = Some(parse_protection(&event, reader.decoder())?);
                }
                b"pssh" => in_pssh = true,
                _ => {}
            },
            Event::Empty(event) => {
                if event.local_name().as_ref() == b"ContentProtection" {
                    results.push(parse_protection(&event, reader.decoder())?);
                }
            }
            Event::Text(text) if in_pssh => {
                if let Some(item) = current.as_mut() {
                    let encoded = text.decode()?.trim().to_owned();
                    if !encoded.is_empty() {
                        item.pssh_bytes = STANDARD
                            .decode(encoded.as_bytes())
                            .ok()
                            .map(|bytes| bytes.len());
                        item.pssh_base64 = Some(encoded);
                    }
                }
            }
            Event::End(event) => match event.local_name().as_ref() {
                b"pssh" => in_pssh = false,
                b"ContentProtection" => {
                    if let Some(item) = current.take() {
                        results.push(item);
                    }
                }
                _ => {}
            },
            Event::Eof => break,
            _ => {}
        }
    }

    Ok(results)
}
