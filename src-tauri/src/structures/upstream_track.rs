use std::str::FromStr;

use serde::Serialize;
use specta::Type;

/// If both are 0, it's in sync. If None, the tracked upstream is missing.
#[derive(Debug, Serialize, Type, Clone)]
pub struct UpstreamTrack(Option<(usize, usize)>);

impl FromStr for UpstreamTrack {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gone" => Ok(Self(None)),
            "" => Ok(Self(Some((0, 0)))),
            v => {
                if let Some((ahead, behind)) = v.split_once(", ") {
                    Ok(Self(Some((
                        ahead
                            .strip_prefix("ahead ")
                            .ok_or("Failed to strip ahead prefix")?
                            .parse()
                            .map_err(|err| format!("Failed to parse ahead: {}", err))?,
                        behind
                            .strip_prefix("behind ")
                            .ok_or("Failed to strip behind prefix")?
                            .parse()
                            .map_err(|err| format!("Failed to parse before: {}", err))?,
                    ))))
                } else if let Some(ahead) = v.strip_prefix("ahead ").and_then(|v| v.parse().ok()) {
                    Ok(Self(Some((ahead, 0))))
                } else {
                    let behind = v
                        .strip_prefix("behind ")
                        .and_then(|v| v.parse().ok())
                        .ok_or(format!("Unknown upstream track format: {}", s))?;
                    Ok(Self(Some((0, behind))))
                }
            }
        }
    }
}
