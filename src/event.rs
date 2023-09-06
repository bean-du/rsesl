use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::{Error, Result};
use serde_json::Value;

pub type EventData = HashMap<String, Value>;

pub trait EventHandler {
    fn get_header(&self, k: String) -> String;
    fn set_header(&mut self, k: String, v: Value);
}

impl EventHandler for EventData {
    fn get_header(&self, k: String) -> String {
        match self.get(&k) {
            Some(v) => match v {
                Value::String(s) => s.to_string(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Null => String::new(),
                Value::Array(_) => String::new(),
                Value::Object(_) => String::new(),
            },
            None => String::new(),
        }
    }

    fn set_header(&mut self, k: String, v: Value) {
        self.insert(k, v);
    }
}

#[derive(Debug)]
pub enum Event {
    ADD_SCHEDULE,
    API,
    BACKGROUND_JOB,
    CALL_DETAIL,
    CALL_SECURE,
    CALL_SETUP_REQ,
    CALL_UPDATE,
    CDR,
    CHANNEL_ANSWER,
    CHANNEL_APPLICATION,
    CHANNEL_BRIDGE,
    CHANNEL_CALLSTATE,
    CHANNEL_CREATE,
    CHANNEL_DATA,
    CHANNEL_DESTROY,
    CHANNEL_EXECUTE,
    CHANNEL_EXECUTE_COMPLETE,
    CHANNEL_GLOBAL,
    CHANNEL_HANGUP,
    CHANNEL_HANGUP_COMPLETE,
    CHANNEL_HOLD,
    CHANNEL_ORIGINATE,
    CHANNEL_OUTGOING,
    CHANNEL_PARK,
    CHANNEL_PROGRESS,
    CHANNEL_PROGRESS_MEDIA,
    CHANNEL_STATE,
    CHANNEL_UNBRIDGE,
    CHANNEL_UNHOLD,
    CHANNEL_UNPARK,
    CHANNEL_UUID,
    CLONE,
    CODEC,
    COMMAND,
    CONFERENCE_DATA,
    CONFERENCE_DATA_QUERY,
    CUSTOM,
    DEL_SCHEDULE,
    DETECTED_SPEECH,
    DETECTED_TONE,
    DEVICE_STATE,
    DTMF,
    EXE_SCHEDULE,
    FAILURE,
    GENERAL,
    HEARTBEAT,
    LOG,
    MEDIA_BUG_START,
    MEDIA_BUG_STOP,
    MESSAGE,
    MESSAGE_QUERY,
    MESSAGE_WAITING,
    MODULE_LOAD,
    MODULE_UNLOAD,
    NAT,
    NOTALK,
    NOTIFY,
    NOTIFY_IN,
    PHONE_FEATURE,
    PHONE_FEATURE_SUBSCRIBE,
    PLAYBACK_START,
    PLAYBACK_STOP,
    PRESENCE_IN,
    PRESENCE_OUT,
    PRESENCE_PROBE,
    PRIVATE_COMMAND,
    PUBLISH,
    QUEUE_LEN,
    RECORD_START,
    RECORD_STOP,
    RECV_INFO,
    RECV_MESSAGE,
    RECV_RTCP_MESSAGE,
    RECYCLE,
    RELOADXML,
    REQUEST_PARAMS,
    RE_SCHEDULE,
    ROSTER,
    SEND_INFO,
    SEND_MESSAGE,
    SESSION_HEARTBEAT,
    SHUTDOWN,
    STARTUP,
    SUBCLASS_ANY,
    TALK,
    TRAP,
    UNPUBLISH,
    None,
}

impl From<String> for Event {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ADD_SCHEDULE" => Event::ADD_SCHEDULE,
            "API" => Event::API,
            "BACKGROUND_JOB" => Event::BACKGROUND_JOB,
            "CALL_DETAIL" => Event::CALL_DETAIL,
            "CALL_SECURE" => Event::CALL_SECURE,
            "CALL_SETUP_REQ" => Event::CALL_SETUP_REQ,
            "CALL_UPDATE" => Event::CALL_UPDATE,
            "CDR" => Event::CDR,
            "CHANNEL_ANSWER" => Event::CHANNEL_ANSWER,
            "CHANNEL_APPLICATION" => Event::CHANNEL_APPLICATION,
            "CHANNEL_BRIDGE" => Event::CHANNEL_BRIDGE,
            "CHANNEL_CALLSTATE" => Event::CHANNEL_CALLSTATE,
            "CHANNEL_CREATE" => Event::CHANNEL_CREATE,
            "CHANNEL_DATA" => Event::CHANNEL_DATA,
            "CHANNEL_DESTROY" => Event::CHANNEL_DESTROY,
            "CHANNEL_EXECUTE" => Event::CHANNEL_EXECUTE,
            "CHANNEL_EXECUTE_COMPLETE" => Event::CHANNEL_EXECUTE_COMPLETE,
            "CHANNEL_GLOBAL" => Event::CHANNEL_GLOBAL,
            "CHANNEL_HANGUP" => Event::CHANNEL_HANGUP,
            "CHANNEL_HANGUP_COMPLETE" => Event::CHANNEL_HANGUP_COMPLETE,
            "CHANNEL_HOLD" => Event::CHANNEL_HOLD,
            "CHANNEL_ORIGINATE" => Event::CHANNEL_ORIGINATE,
            "CHANNEL_OUTGOING" => Event::CHANNEL_OUTGOING,
            "CHANNEL_PARK" => Event::CHANNEL_PARK,
            "CHANNEL_PROGRESS" => Event::CHANNEL_PROGRESS,
            "CHANNEL_PROGRESS_MEDIA" => Event::CHANNEL_PROGRESS_MEDIA,
            "CHANNEL_STATE" => Event::CHANNEL_STATE,
            "CHANNEL_UNBRIDGE" => Event::CHANNEL_UNBRIDGE,
            "CHANNEL_UNHOLD" => Event::CHANNEL_UNHOLD,
            "CHANNEL_UNPARK" => Event::CHANNEL_UNPARK,
            "CHANNEL_UUID" => Event::CHANNEL_UUID,
            "CLONE" => Event::CLONE,
            "CODEC" => Event::CODEC,
            "COMMAND" => Event::COMMAND,
            "CONFERENCE_DATA" => Event::CONFERENCE_DATA,
            "CONFERENCE_DATA_QUERY" => Event::CONFERENCE_DATA_QUERY,
            "CUSTOM" => Event::CUSTOM,
            "DEL_SCHEDULE" => Event::DEL_SCHEDULE,
            "DETECTED_SPEECH" => Event::DETECTED_SPEECH,
            "DETECTED_TONE" => Event::DETECTED_TONE,
            "DEVICE_STATE" => Event::DEVICE_STATE,
            "DTMF" => Event::DTMF,
            "EXE_SCHEDULE" => Event::EXE_SCHEDULE,
            "FAILURE" => Event::FAILURE,
            "GENERAL" => Event::GENERAL,
            "HEARTBEAT" => Event::HEARTBEAT,
            "LOG" => Event::LOG,
            "MEDIA_BUG_START" => Event::MEDIA_BUG_START,
            "MEDIA_BUG_STOP" => Event::MEDIA_BUG_STOP,
            "MESSAGE" => Event::MESSAGE,
            "MESSAGE_QUERY" => Event::MESSAGE_QUERY,
            "MESSAGE_WAITING" => Event::MESSAGE_WAITING,
            "MODULE_LOAD" => Event::MODULE_LOAD,
            "MODULE_UNLOAD" => Event::MODULE_UNLOAD,
            "NAT" => Event::NAT,
            "NOTALK" => Event::NOTALK,
            "NOTIFY" => Event::NOTIFY,
            "NOTIFY_IN" => Event::NOTIFY_IN,
            "PHONE_FEATURE" => Event::PHONE_FEATURE,
            "PHONE_FEATURE_SUBSCRIBE" => Event::PHONE_FEATURE_SUBSCRIBE,
            "PLAYBACK_START" => Event::PLAYBACK_START,
            "PLAYBACK_STOP" => Event::PLAYBACK_STOP,
            "PRESENCE_IN" => Event::PRESENCE_IN,
            "PRESENCE_OUT" => Event::PRESENCE_OUT,
            "PRESENCE_PROBE" => Event::PRESENCE_PROBE,
            "PRIVATE_COMMAND" => Event::PRIVATE_COMMAND,
            "PUBLISH" => Event::PUBLISH,
            "QUEUE_LEN" => Event::QUEUE_LEN,
            "RECORD_START" => Event::RECORD_START,
            "RECORD_STOP" => Event::RECORD_STOP,
            "RECV_INFO" => Event::RECV_INFO,
            "RECV_MESSAGE" => Event::RECV_MESSAGE,
            "RECV_RTCP_MESSAGE" => Event::RECV_RTCP_MESSAGE,
            "RECYCLE" => Event::RECYCLE,
            "RELOADXML" => Event::RELOADXML,
            "REQUEST_PARAMS" => Event::REQUEST_PARAMS,
            "RE_SCHEDULE" => Event::RE_SCHEDULE,
            "ROSTER" => Event::ROSTER,
            "SEND_INFO" => Event::SEND_INFO,
            "SEND_MESSAGE" => Event::SEND_MESSAGE,
            "SESSION_HEARTBEAT" => Event::SESSION_HEARTBEAT,
            "SHUTDOWN" => Event::SHUTDOWN,
            "STARTUP" => Event::STARTUP,
            "SUBCLASS_ANY" => Event::SUBCLASS_ANY,
            "TALK" => Event::TALK,
            "TRAP" => Event::TRAP,
            "UNPUBLISH" => Event::UNPUBLISH,
            _ => Event::None,
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Event::ADD_SCHEDULE => "ADD_SCHEDULE",
            Event::API => "API",
            Event::BACKGROUND_JOB => "BACKGROUND_JOB",
            Event::CALL_DETAIL => "CALL_DETAIL",
            Event::CALL_SECURE => "CALL_SECURE",
            Event::CALL_SETUP_REQ => "CALL_SETUP_REQ",
            Event::CALL_UPDATE => "CALL_UPDATE",
            Event::CDR => "CDR",
            Event::CHANNEL_ANSWER => "CHANNEL_ANSWER",
            Event::CHANNEL_APPLICATION => "CHANNEL_APPLICATION",
            Event::CHANNEL_BRIDGE => "CHANNEL_BRIDGE",
            Event::CHANNEL_CALLSTATE => "CHANNEL_CALLSTATE",
            Event::CHANNEL_CREATE => "CHANNEL_CREATE",
            Event::CHANNEL_DATA => "CHANNEL_DATA",
            Event::CHANNEL_DESTROY => "CHANNEL_DESTROY",
            Event::CHANNEL_EXECUTE => "CHANNEL_EXECUTE",
            Event::CHANNEL_EXECUTE_COMPLETE => "CHANNEL_EXECUTE_COMPLETE",
            Event::CHANNEL_GLOBAL => "CHANNEL_GLOBAL",
            Event::CHANNEL_HANGUP => "CHANNEL_HANGUP",
            Event::CHANNEL_HANGUP_COMPLETE => "CHANNEL_HANGUP_COMPLETE",
            Event::CHANNEL_HOLD => "CHANNEL_HOLD",
            Event::CHANNEL_ORIGINATE => "CHANNEL_ORIGINATE",
            Event::CHANNEL_OUTGOING => "CHANNEL_OUTGOING",
            Event::CHANNEL_PARK => "CHANNEL_PARK",
            Event::CHANNEL_PROGRESS => "CHANNEL_PROGRESS",
            Event::CHANNEL_PROGRESS_MEDIA => "CHANNEL_PROGRESS_MEDIA",
            Event::CHANNEL_STATE => "CHANNEL_STATE",
            Event::CHANNEL_UNBRIDGE => "CHANNEL_UNBRIDGE",
            Event::CHANNEL_UNHOLD => "CHANNEL_UNHOLD",
            Event::CHANNEL_UNPARK => "CHANNEL_UNPARK",
            Event::CHANNEL_UUID => "CHANNEL_UUID",
            Event::CLONE => "CLONE",
            Event::CODEC => "CODEC",
            Event::COMMAND => "COMMAND",
            Event::CONFERENCE_DATA => "CONFERENCE_DATA",
            Event::CONFERENCE_DATA_QUERY => "CONFERENCE_DATA_QUERY",
            Event::CUSTOM => "CUSTOM",
            Event::DEL_SCHEDULE => "DEL_SCHEDULE",
            Event::DETECTED_SPEECH => "DETECTED_SPEECH",
            Event::DETECTED_TONE => "DETECTED_TONE",
            Event::DEVICE_STATE => "DEVICE_STATE",
            Event::DTMF => "DTMF",
            Event::EXE_SCHEDULE => "EXE_SCHEDULE",
            Event::FAILURE => "FAILURE",
            Event::GENERAL => "GENERAL",
            Event::HEARTBEAT => "HEARTBEAT",
            Event::LOG => "LOG",
            Event::MEDIA_BUG_START => "MEDIA_BUG_START",
            Event::MEDIA_BUG_STOP => "MEDIA_BUG_STOP",
            Event::MESSAGE => "MESSAGE",
            Event::MESSAGE_QUERY => "MESSAGE_QUERY",
            Event::MESSAGE_WAITING => "MESSAGE_WAITING",
            Event::MODULE_LOAD => "MODULE_LOAD",
            Event::MODULE_UNLOAD => "MODULE_UNLOAD",
            Event::NAT => "NAT",
            Event::NOTALK => "NOTALK",
            Event::NOTIFY => "NOTIFY",
            Event::NOTIFY_IN => "NOTIFY_IN",
            Event::PHONE_FEATURE => "PHONE_FEATURE",
            Event::PHONE_FEATURE_SUBSCRIBE => "PHONE_FEATURE_SUBSCRIBE",
            Event::PLAYBACK_START => "PLAYBACK_START",
            Event::PLAYBACK_STOP => "PLAYBACK_STOP",
            Event::PRESENCE_IN => "PRESENCE_IN",
            Event::PRESENCE_OUT => "PRESENCE_OUT",
            Event::PRESENCE_PROBE => "PRESENCE_PROBE",
            Event::PRIVATE_COMMAND => "PRIVATE_COMMAND",
            Event::PUBLISH => "PUBLISH",
            Event::QUEUE_LEN => "QUEUE_LEN",
            Event::RECORD_START => "RECORD_START",
            Event::RECORD_STOP => "RECORD_STOP",
            Event::RECV_INFO => "RECV_INFO",
            Event::RECV_MESSAGE => "RECV_MESSAGE",
            Event::RECV_RTCP_MESSAGE => "RECV_RTCP_MESSAGE",
            Event::RECYCLE => "RECYCLE",
            Event::RELOADXML => "RELOADXML",
            Event::REQUEST_PARAMS => "REQUEST_PARAMS",
            Event::RE_SCHEDULE => "RE_SCHEDULE",
            Event::ROSTER => "ROSTER",
            Event::SEND_INFO => "SEND_INFO",
            Event::SEND_MESSAGE => "SEND_MESSAGE",
            Event::SESSION_HEARTBEAT => "SESSION_HEARTBEAT",
            Event::SHUTDOWN => "SHUTDOWN",
            Event::STARTUP => "STARTUP",
            Event::SUBCLASS_ANY => "SUBCLASS_ANY",
            Event::TALK => "TALK",
            Event::TRAP => "TRAP",
            Event::UNPUBLISH => "UNPUBLISH",
            _ => "Invalid Event",
        };
        write!(f, "{}", s)
    }
}