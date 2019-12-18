// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

use dbus as dbus;
use dbus::arg;
use dbus::tree;

pub trait OrgFreedesktopDBusProperties {
    fn get(&self, interface_name: &str, property_name: &str) -> Result<arg::Variant<Box<dyn arg::RefArg + 'static>>, tree::MethodErr>;
    fn get_all(&self, interface_name: &str) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, tree::MethodErr>;
    fn set(&self, interface_name: &str, property_name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> Result<(), tree::MethodErr>;
}

pub fn org_freedesktop_dbus_properties_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Signal: Default,
    T: OrgFreedesktopDBusProperties,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Properties", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = i.read()?;
        let property_name: &str = i.read()?;
        let d = fclone(minfo);
        let value = d.get(interface_name, property_name)?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(value);
        Ok(vec!(rm))
    };
    let m = factory.method("Get", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.in_arg(("property_name", "s"));
    let m = m.out_arg(("value", "v"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = i.read()?;
        let d = fclone(minfo);
        let properties = d.get_all(interface_name)?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(properties);
        Ok(vec!(rm))
    };
    let m = factory.method("GetAll", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.out_arg(("properties", "a{sv}"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface_name: &str = i.read()?;
        let property_name: &str = i.read()?;
        let value: arg::Variant<Box<dyn arg::RefArg>> = i.read()?;
        let d = fclone(minfo);
        d.set(interface_name, property_name, value)?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Set", Default::default(), h);
    let m = m.in_arg(("interface_name", "s"));
    let m = m.in_arg(("property_name", "s"));
    let m = m.in_arg(("value", "v"));
    let i = i.add_m(m);
    let s = factory.signal("PropertiesChanged", Default::default());
    let s = s.arg(("interface_name", "s"));
    let s = s.arg(("changed_properties", "a{sv}"));
    let s = s.arg(("invalidated_properties", "as"));
    let i = i.add_s(s);
    i
}

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

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, tree::MethodErr>;
}

pub fn org_freedesktop_dbus_introspectable_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgFreedesktopDBusIntrospectable,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Introspectable", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let xml_data = d.introspect()?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(xml_data);
        Ok(vec!(rm))
    };
    let m = factory.method("Introspect", Default::default(), h);
    let m = m.out_arg(("xml_data", "s"));
    let i = i.add_m(m);
    i
}

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> Result<(), tree::MethodErr>;
    fn get_machine_id(&self) -> Result<String, tree::MethodErr>;
}

pub fn org_freedesktop_dbus_peer_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgFreedesktopDBusPeer,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Peer", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.ping()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Ping", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let machine_uuid = d.get_machine_id()?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(machine_uuid);
        Ok(vec!(rm))
    };
    let m = factory.method("GetMachineId", Default::default(), h);
    let m = m.out_arg(("machine_uuid", "s"));
    let i = i.add_m(m);
    i
}

pub trait OrgMprisMediaPlayer2 {
    fn raise(&self) -> Result<(), tree::MethodErr>;
    fn quit(&self) -> Result<(), tree::MethodErr>;
    fn get_can_quit(&self) -> Result<bool, tree::MethodErr>;
    fn get_can_raise(&self) -> Result<bool, tree::MethodErr>;
    fn get_has_track_list(&self) -> Result<bool, tree::MethodErr>;
    fn get_identity(&self) -> Result<String, tree::MethodErr>;
    fn get_desktop_entry(&self) -> Result<String, tree::MethodErr>;
    fn get_supported_uri_schemes(&self) -> Result<Vec<String>, tree::MethodErr>;
    fn get_supported_mime_types(&self) -> Result<Vec<String>, tree::MethodErr>;
}

pub fn org_mpris_media_player2_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Property: Default,
    T: OrgMprisMediaPlayer2,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.mpris.MediaPlayer2", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.raise()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Raise", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.quit()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Quit", Default::default(), h);
    let i = i.add_m(m);

    let p = factory.property::<bool, _>("CanQuit", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_can_quit()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("CanRaise", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_can_raise()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("HasTrackList", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_has_track_list()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("Identity", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_identity()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("DesktopEntry", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_desktop_entry()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<Vec<&str>, _>("SupportedUriSchemes", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_supported_uri_schemes()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<Vec<&str>, _>("SupportedMimeTypes", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_supported_mime_types()?);
        Ok(())
    });
    let i = i.add_p(p);
    i
}

pub trait OrgMprisMediaPlayer2Player {
    fn next(&self) -> Result<(), tree::MethodErr>;
    fn previous(&self) -> Result<(), tree::MethodErr>;
    fn pause(&self) -> Result<(), tree::MethodErr>;
    fn play_pause(&self) -> Result<(), tree::MethodErr>;
    fn stop(&self) -> Result<(), tree::MethodErr>;
    fn play(&self) -> Result<(), tree::MethodErr>;
    fn seek(&self, offset: i64) -> Result<(), tree::MethodErr>;
    fn set_position(&self, track_id: dbus::Path, position: i64) -> Result<(), tree::MethodErr>;
    fn open_uri(&self, uri: &str) -> Result<(), tree::MethodErr>;
    fn get_playback_status(&self) -> Result<String, tree::MethodErr>;
    fn get_loop_status(&self) -> Result<String, tree::MethodErr>;
    fn set_loop_status(&self, value: String) -> Result<(), tree::MethodErr>;
    fn get_rate(&self) -> Result<f64, tree::MethodErr>;
    fn set_rate(&self, value: f64) -> Result<(), tree::MethodErr>;
    fn get_shuffle(&self) -> Result<bool, tree::MethodErr>;
    fn set_shuffle(&self, value: bool) -> Result<(), tree::MethodErr>;
    fn get_metadata(&self) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, tree::MethodErr>;
    fn get_volume(&self) -> Result<f64, tree::MethodErr>;
    fn set_volume(&self, value: f64) -> Result<(), tree::MethodErr>;
    fn get_position(&self) -> Result<i64, tree::MethodErr>;
    fn get_minimum_rate(&self) -> Result<f64, tree::MethodErr>;
    fn get_maximum_rate(&self) -> Result<f64, tree::MethodErr>;
    fn get_can_go_next(&self) -> Result<bool, tree::MethodErr>;
    fn get_can_go_previous(&self) -> Result<bool, tree::MethodErr>;
    fn get_can_play(&self) -> Result<bool, tree::MethodErr>;
    fn get_can_pause(&self) -> Result<bool, tree::MethodErr>;
    fn get_can_seek(&self) -> Result<bool, tree::MethodErr>;
    fn get_can_control(&self) -> Result<bool, tree::MethodErr>;
}

pub fn org_mpris_media_player2_player_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Property: Default,
    D::Signal: Default,
    T: OrgMprisMediaPlayer2Player,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.mpris.MediaPlayer2.Player", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.next()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Next", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.previous()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Previous", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.pause()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Pause", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.play_pause()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("PlayPause", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.stop()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Stop", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.play()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Play", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let offset: i64 = i.read()?;
        let d = fclone(minfo);
        d.seek(offset)?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Seek", Default::default(), h);
    let m = m.in_arg(("Offset", "x"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let track_id: dbus::Path = i.read()?;
        let position: i64 = i.read()?;
        let d = fclone(minfo);
        d.set_position(track_id, position)?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("SetPosition", Default::default(), h);
    let m = m.in_arg(("TrackId", "o"));
    let m = m.in_arg(("Position", "x"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let uri: &str = i.read()?;
        let d = fclone(minfo);
        d.open_uri(uri)?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("OpenUri", Default::default(), h);
    let m = m.in_arg(("Uri", "s"));
    let i = i.add_m(m);

    let p = factory.property::<&str, _>("PlaybackStatus", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_playback_status()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("LoopStatus", Default::default());
    let p = p.access(tree::Access::ReadWrite);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_loop_status()?);
        Ok(())
    });
    let fclone = f.clone();
    let p = p.on_set(move |iter, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        d.set_loop_status(iter.read()?)?;
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<f64, _>("Rate", Default::default());
    let p = p.access(tree::Access::ReadWrite);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_rate()?);
        Ok(())
    });
    let fclone = f.clone();
    let p = p.on_set(move |iter, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        d.set_rate(iter.read()?)?;
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("Shuffle", Default::default());
    let p = p.access(tree::Access::ReadWrite);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_shuffle()?);
        Ok(())
    });
    let fclone = f.clone();
    let p = p.on_set(move |iter, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        d.set_shuffle(iter.read()?)?;
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>, _>("Metadata", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_metadata()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<f64, _>("Volume", Default::default());
    let p = p.access(tree::Access::ReadWrite);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_volume()?);
        Ok(())
    });
    let fclone = f.clone();
    let p = p.on_set(move |iter, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        d.set_volume(iter.read()?)?;
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<i64, _>("Position", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_position()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<f64, _>("MinimumRate", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_minimum_rate()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<f64, _>("MaximumRate", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_maximum_rate()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("CanGoNext", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_can_go_next()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("CanGoPrevious", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_can_go_previous()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("CanPlay", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_can_play()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("CanPause", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_can_pause()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("CanSeek", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_can_seek()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("CanControl", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.get_can_control()?);
        Ok(())
    });
    let i = i.add_p(p);
    let s = factory.signal("Seeked", Default::default());
    let s = s.arg(("Position", "x"));
    let i = i.add_s(s);
    i
}

#[derive(Debug)]
pub struct OrgMprisMediaPlayer2PlayerSeeked {
    pub position: i64,
}

impl arg::AppendAll for OrgMprisMediaPlayer2PlayerSeeked {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.position, i);
    }
}

impl arg::ReadAll for OrgMprisMediaPlayer2PlayerSeeked {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgMprisMediaPlayer2PlayerSeeked {
            position: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgMprisMediaPlayer2PlayerSeeked {
    const NAME: &'static str = "Seeked";
    const INTERFACE: &'static str = "org.mpris.MediaPlayer2.Player";
}
