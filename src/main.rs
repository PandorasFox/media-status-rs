extern crate dbus;
use dbus::arg;

/* begin dbus-codegen generated code */
#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}
/* end generated code */

#[inline]
fn output(playback_status: &String, title: String, artist: String, album: String) {
    // see https://github.com/rust-lang/rust/issues/46016 for why there's a single panic in main
    // when running in waybar (you can see if you launch waybar in your term)
    // outputting for waybar. sorry folks.
    // if spotify supported the Position prop, i'd also export a %. alas,
    // tho it would probably cause a /lot/ of propertiesChanged pings
    // or would be hard to handle well (sigh)
    if playback_status != "Playing" {
        println!("{{\"text\": \"{} - {}\", \"tooltip\": \"{}\", \"class\": \"pause\"}}", artist, title, album);
    } else {
        println!("{{\"text\": \"{} - {}\", \"tooltip\": \"{}\", \"class\": \"play\"}}", artist, title, album);
    }
}

use dbus::arg::{Variant, RefArg};
use std::boxed::Box;
use std::option::Option;

#[inline]
fn uber_unwrap(maybe_value: Option<&Variant<Box<dyn RefArg>>>) -> Option<String> {
    // pull out of the map optional
    if let Some(value) = maybe_value {
        if let Some(s) = value.0.as_str() {
            return Some(s.to_string());
        } else if let Some(mut iter) = value.0.as_iter() {
            if let Some(ref arr_innerstr_refarg) = iter.next() {
                if let Some(arr_innerstr) = arr_innerstr_refarg.as_str() {
                    return Some(arr_innerstr.to_string());
                }
            }
        }
    }
    return None;
}

use dbus::blocking::Connection;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;

fn main() ->  Result<(), Box<dyn Error>> {
    #[cfg(feature = "timing")]
    let init = std::time::SystemTime::now();
    
    let mut c = Connection::new_session()?;
    // could replace 'spotify' with any mediaplayer name passed via cli, probably
    let proxy = c.with_proxy("org.mpris.MediaPlayer2.spotify", "/org/mpris/MediaPlayer2", Duration::from_millis(5000));
    let playback_status: String = proxy.get("org.mpris.MediaPlayer2.Player", "PlaybackStatus")?;
    // xesam:album and xesam:title are always at least "" with spotify
    // xesam:artist is always at least [""] with spotify
    let metadata: HashMap<String, Variant<Box<dyn RefArg>>> = proxy.get("org.mpris.MediaPlayer2.Player", "Metadata")?;

    #[cfg(feature = "timing")]
    let prop_get_time = init.elapsed().unwrap();

    // .get is Option<Variant<Box<dyn RefArg>>>
    // refarg has to be .as_str'd for the title (and then unwrapped)
    let title: Option<String> = uber_unwrap(metadata.get("xesam:title"));
    // xesam:artist is always a list of artists. we only care about the first one.
    let artist: Option<String> = uber_unwrap(metadata.get("xesam:artist"));
    // maybe feature-enable album stuff?
    let album: Option<String> = uber_unwrap(metadata.get("xesam:album"));
    
    output(&playback_status,
           title.unwrap(),
           artist.unwrap(),
           album.unwrap()
    );
    
    #[cfg(feature = "timing")] {
        let print_time = init.elapsed().unwrap();
        println!("micros to fetch props: {}\nmicros to format and print: {}\ntotal elapsed time: {}",
                 prop_get_time.as_micros(),
                 print_time.as_micros() - prop_get_time.as_micros(),
                 print_time.as_micros()
        );
    }
    
    {
        let _id = proxy.match_signal(|h: OrgFreedesktopDBusPropertiesPropertiesChanged, _: &Connection| {
            #[cfg(feature = "timing")]
            let wake_time = std::time::SystemTime::now();
            let maybe_playback_status = h.changed_properties.get("PlaybackStatus");
            let maybe_metadata = h.changed_properties.get("Metadata");
            // fortunately, metadata dict always has all values in it
            let mut new_playback_status: String = "".to_string();
            let mut new_title: String = "".to_string();
            let mut new_artist: String = "".to_string();
            let mut new_album: String = "".to_string();
            if maybe_playback_status.is_some() {
                // yolo unwrap, wouldn't be in the dict otherwise
                new_playback_status = uber_unwrap(maybe_playback_status).unwrap();
            }
            if maybe_metadata.is_some() {
                let mut metadata_iter = maybe_metadata.unwrap().0.as_iter().unwrap();
                while let Some(key) = metadata_iter.next() {
                    if key.as_str().unwrap_or("") == "xesam:artist" {
                        /* This block is kinda weird.
                         * Instead of the val being a refarg containing an iter (array), it's
                         * actually a refarg containing a variant containing an iter (array).
                         * unfortunately no way to pull out the variant from the refarg without
                         * casting from any, so I have to iter through twice
                         */
                        let artist_value = metadata_iter.next().unwrap();
                        let mut artist_value_variant_iter = artist_value.as_iter().unwrap();
                        let mut artist_value_arr_iter = artist_value_variant_iter.next().unwrap().as_iter().unwrap();
                        new_artist = artist_value_arr_iter.next().unwrap().as_str().unwrap_or("").to_string();
                    } else if key.as_str().unwrap_or("") == "xesam:title" {
                        new_title = metadata_iter.next().unwrap().as_str().unwrap_or("").to_string();
                    } else if key.as_str().unwrap_or("") == "xesam:album" {
                        new_album = metadata_iter.next().unwrap().as_str().unwrap_or("").to_string();
                    }
                }
            }
            output(&new_playback_status, new_title, new_artist,new_album);
            #[cfg(feature = "timing")] {
                let long_time_no_see = wake_time.elapsed().unwrap();
                println!("time from wake to print: {}", long_time_no_see.as_micros());
            }
            return true;
        });
    }

    // Listen to incoming signals forever.
    loop { c.process(Duration::from_millis(1000))?; }
}
