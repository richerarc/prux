use futures::Future;
use tokio::prelude::*;
use std::net::{Ipv4Addr, IpAddr};
use crate::IpResolver;
use serde_json::Value;
use hashbrown::HashMap;

const PRUX_ADDR: &str = "Prux-Addr";
const PRUX_CITY: &str = "Prux-City";
const PRUX_COUNTRY: &str = "Prux-Country";
const PRUX_COORD: &str = "Prux-Coord"; // lat / long

pub fn inject_basic_hdr(ip: IpAddr, resolver: IpResolver) -> impl Future<Item=HashMap<String, String>, Error=()> {
    let fut = resolver.lookup(&ip).and_then(move |json: Value| {
        let mut hdr_map = HashMap::new();

        hdr_map.insert(PRUX_ADDR.to_string(), ip.to_string());

        if let Some(city_name_en) = json.get("city").and_then(|val| val.get("names")).and_then(|names| names.get("en").map(|en_name| en_name.as_str())) {
            if let Some(name) = city_name_en {
                hdr_map.insert(PRUX_CITY.to_string(), name.to_string());
            }
        }

        if let Some(country_name_en) = json.get("country").and_then(|val| val.get("names")).and_then(|names| names.get("en").map(|en_name| en_name.as_str())) {
            if let Some(name) = country_name_en {
                hdr_map.insert(PRUX_COUNTRY.to_string(), name.to_string());
            }
        }

        if let Some((Some(lat), Some(long))) = json.get("location").map(|val| {  (val.get("latitude").and_then(|l| l.as_f64().map(|n| n.to_string())), val.get("longitude").and_then(|l| l.as_f64().map(|n| n.to_string()))) }) {
            hdr_map.insert(PRUX_COORD.to_string(), format!("{},{}", lat, long));
        }

        future::finished(hdr_map)
    });

    fut
}

