use serde_json::{Map, Value};
use std::fmt::Display;

pub type EventData = Map<String, Value>;

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
    AddSchedule,
    Api,
    BackgroundJob,
    CallDetail,
    CallSecure,
    CallSetupReq,
    CallUpdate,
    Cdr,
    ChannelAnswer,
    ChannelApplication,
    ChannelBridge,
    ChannelCallState,
    ChannelCreate,
    ChannelData,
    ChannelDestroy,
    ChannelExecute,
    ChannelExecuteComplete,
    ChannelGlobal,
    ChannelHangup,
    ChannelHangupComplete,
    ChannelHold,
    ChannelOriginate,
    ChannelOutgoing,
    ChannelPark,
    ChannelProgress,
    ChannelProgressMedia,
    ChannelState,
    ChannelUnbridge,
    ChannelUnhold,
    ChannelUnpark,
    ChannelUuid,
    Clone,
    Codec,
    Command,
    ConferenceData,
    ConferenceDataQuery,
    Custom,
    DelSchedule,
    DetectedSpeech,
    DetectedTone,
    DeviceState,
    Dtmf,
    ExeSchedule,
    Failure,
    General,
    Heartbeat,
    Log,
    MediaBugStart,
    MediaBugStop,
    Message,
    MessageQuery,
    MessageWaiting,
    ModuleLoad,
    ModuleUnload,
    Nat,
    Notalk,
    Notify,
    NotifyIn,
    PhoneFeature,
    PhoneFeatureSubscribe,
    PlaybackStart,
    PlaybackStop,
    PresenceIn,
    PresenceOut,
    PresenceProbe,
    PrivateCommand,
    Publish,
    QueueLen,
    RecordStart,
    RecordStop,
    RecvInfo,
    RecvMessage,
    RecvRtcpMessage,
    Recycle,
    ReloadXml,
    RequestParams,
    ReSchedule,
    Roster,
    SendInfo,
    SendMessage,
    SessionHeartbeat,
    Shutdown,
    Startup,
    SubclassAny,
    Talk,
    Trap,
    Unpublish,
    None,
}

impl From<String> for Event {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ADD_SCHEDULE" => Event::AddSchedule,
            "API" => Event::Api,
            "BACKGROUND_JOB" => Event::BackgroundJob,
            "CALL_DETAIL" => Event::CallDetail,
            "CALL_SECURE" => Event::CallSecure,
            "CALL_SETUP_REQ" => Event::CallSetupReq,
            "CALL_UPDATE" => Event::CallUpdate,
            "CDR" => Event::Cdr,
            "CHANNEL_ANSWER" => Event::ChannelAnswer,
            "CHANNEL_APPLICATION" => Event::ChannelApplication,
            "CHANNEL_BRIDGE" => Event::ChannelBridge,
            "CHANNEL_CALLSTATE" => Event::ChannelCallState,
            "CHANNEL_CREATE" => Event::ChannelCreate,
            "CHANNEL_DATA" => Event::ChannelData,
            "CHANNEL_DESTROY" => Event::ChannelDestroy,
            "CHANNEL_EXECUTE" => Event::ChannelExecute,
            "CHANNEL_EXECUTE_COMPLETE" => Event::ChannelExecuteComplete,
            "CHANNEL_GLOBAL" => Event::ChannelGlobal,
            "CHANNEL_HANGUP" => Event::ChannelHangup,
            "CHANNEL_HANGUP_COMPLETE" => Event::ChannelHangupComplete,
            "CHANNEL_HOLD" => Event::ChannelHold,
            "CHANNEL_ORIGINATE" => Event::ChannelOriginate,
            "CHANNEL_OUTGOING" => Event::ChannelOutgoing,
            "CHANNEL_PARK" => Event::ChannelPark,
            "CHANNEL_PROGRESS" => Event::ChannelProgress,
            "CHANNEL_PROGRESS_MEDIA" => Event::ChannelProgressMedia,
            "CHANNEL_STATE" => Event::ChannelState,
            "CHANNEL_UNBRIDGE" => Event::ChannelUnbridge,
            "CHANNEL_UNHOLD" => Event::ChannelUnhold,
            "CHANNEL_UNPARK" => Event::ChannelUnpark,
            "CHANNEL_UUID" => Event::ChannelUuid,
            "CLONE" => Event::Clone,
            "CODEC" => Event::Codec,
            "COMMAND" => Event::Command,
            "CONFERENCE_DATA" => Event::ConferenceData,
            "CONFERENCE_DATA_QUERY" => Event::ConferenceDataQuery,
            "CUSTOM" => Event::Custom,
            "DEL_SCHEDULE" => Event::DelSchedule,
            "DETECTED_SPEECH" => Event::DetectedSpeech,
            "DETECTED_TONE" => Event::DetectedTone,
            "DEVICE_STATE" => Event::DeviceState,
            "DTMF" => Event::Dtmf,
            "EXE_SCHEDULE" => Event::ExeSchedule,
            "FAILURE" => Event::Failure,
            "GENERAL" => Event::General,
            "HEARTBEAT" => Event::Heartbeat,
            "LOG" => Event::Log,
            "MEDIA_BUG_START" => Event::MediaBugStart,
            "MEDIA_BUG_STOP" => Event::MediaBugStop,
            "MESSAGE" => Event::Message,
            "MESSAGE_QUERY" => Event::MessageQuery,
            "MESSAGE_WAITING" => Event::MessageWaiting,
            "MODULE_LOAD" => Event::ModuleLoad,
            "MODULE_UNLOAD" => Event::ModuleUnload,
            "NAT" => Event::Nat,
            "NOTALK" => Event::Notalk,
            "NOTIFY" => Event::Notify,
            "NOTIFY_IN" => Event::NotifyIn,
            "PHONE_FEATURE" => Event::PhoneFeature,
            "PHONE_FEATURE_SUBSCRIBE" => Event::PhoneFeatureSubscribe,
            "PLAYBACK_START" => Event::PlaybackStart,
            "PLAYBACK_STOP" => Event::PlaybackStop,
            "PRESENCE_IN" => Event::PresenceIn,
            "PRESENCE_OUT" => Event::PresenceOut,
            "PRESENCE_PROBE" => Event::PresenceProbe,
            "PRIVATE_COMMAND" => Event::PrivateCommand,
            "PUBLISH" => Event::Publish,
            "QUEUE_LEN" => Event::QueueLen,
            "RECORD_START" => Event::RecordStart,
            "RECORD_STOP" => Event::RecordStop,
            "RECV_INFO" => Event::RecvInfo,
            "RECV_MESSAGE" => Event::RecvMessage,
            "RECV_RTCP_MESSAGE" => Event::RecvRtcpMessage,
            "RECYCLE" => Event::Recycle,
            "RELOADXML" => Event::ReloadXml,
            "REQUEST_PARAMS" => Event::RequestParams,
            "RE_SCHEDULE" => Event::ReSchedule,
            "ROSTER" => Event::Roster,
            "SEND_INFO" => Event::SendInfo,
            "SEND_MESSAGE" => Event::SendMessage,
            "SESSION_HEARTBEAT" => Event::SessionHeartbeat,
            "SHUTDOWN" => Event::Shutdown,
            "STARTUP" => Event::Startup,
            "SUBCLASS_ANY" => Event::SubclassAny,
            "TALK" => Event::Talk,
            "TRAP" => Event::Trap,
            "UNPUBLISH" => Event::Unpublish,
            _ => Event::None,
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Event::AddSchedule => "ADD_SCHEDULE",
            Event::Api => "API",
            Event::BackgroundJob => "BACKGROUND_JOB",
            Event::CallDetail => "CALL_DETAIL",
            Event::CallSecure => "CALL_SECURE",
            Event::CallSetupReq => "CALL_SETUP_REQ",
            Event::CallUpdate => "CALL_UPDATE",
            Event::Cdr => "CDR",
            Event::ChannelAnswer => "CHANNEL_ANSWER",
            Event::ChannelApplication => "CHANNEL_APPLICATION",
            Event::ChannelBridge => "CHANNEL_BRIDGE",
            Event::ChannelCallState => "CHANNEL_CALLSTATE",
            Event::ChannelCreate => "CHANNEL_CREATE",
            Event::ChannelData => "CHANNEL_DATA",
            Event::ChannelDestroy => "CHANNEL_DESTROY",
            Event::ChannelExecute => "CHANNEL_EXECUTE",
            Event::ChannelExecuteComplete => "CHANNEL_EXECUTE_COMPLETE",
            Event::ChannelGlobal => "CHANNEL_GLOBAL",
            Event::ChannelHangup => "CHANNEL_HANGUP",
            Event::ChannelHangupComplete => "CHANNEL_HANGUP_COMPLETE",
            Event::ChannelHold => "CHANNEL_HOLD",
            Event::ChannelOriginate => "CHANNEL_ORIGINATE",
            Event::ChannelOutgoing => "CHANNEL_OUTGOING",
            Event::ChannelPark => "CHANNEL_PARK",
            Event::ChannelProgress => "CHANNEL_PROGRESS",
            Event::ChannelProgressMedia => "CHANNEL_PROGRESS_MEDIA",
            Event::ChannelState => "CHANNEL_STATE",
            Event::ChannelUnbridge => "CHANNEL_UNBRIDGE",
            Event::ChannelUnhold => "CHANNEL_UNHOLD",
            Event::ChannelUnpark => "CHANNEL_UNPARK",
            Event::ChannelUuid => "CHANNEL_UUID",
            Event::Clone => "CLONE",
            Event::Codec => "CODEC",
            Event::Command => "COMMAND",
            Event::ConferenceData => "CONFERENCE_DATA",
            Event::ConferenceDataQuery => "CONFERENCE_DATA_QUERY",
            Event::Custom => "CUSTOM",
            Event::DelSchedule => "DEL_SCHEDULE",
            Event::DetectedSpeech => "DETECTED_SPEECH",
            Event::DetectedTone => "DETECTED_TONE",
            Event::DeviceState => "DEVICE_STATE",
            Event::Dtmf => "DTMF",
            Event::ExeSchedule => "EXE_SCHEDULE",
            Event::Failure => "FAILURE",
            Event::General => "GENERAL",
            Event::Heartbeat => "HEARTBEAT",
            Event::Log => "LOG",
            Event::MediaBugStart => "MEDIA_BUG_START",
            Event::MediaBugStop => "MEDIA_BUG_STOP",
            Event::Message => "MESSAGE",
            Event::MessageQuery => "MESSAGE_QUERY",
            Event::MessageWaiting => "MESSAGE_WAITING",
            Event::ModuleLoad => "MODULE_LOAD",
            Event::ModuleUnload => "MODULE_UNLOAD",
            Event::Nat => "NAT",
            Event::Notalk => "NOTALK",
            Event::Notify => "NOTIFY",
            Event::NotifyIn => "NOTIFY_IN",
            Event::PhoneFeature => "PHONE_FEATURE",
            Event::PhoneFeatureSubscribe => "PHONE_FEATURE_SUBSCRIBE",
            Event::PlaybackStart => "PLAYBACK_START",
            Event::PlaybackStop => "PLAYBACK_STOP",
            Event::PresenceIn => "PRESENCE_IN",
            Event::PresenceOut => "PRESENCE_OUT",
            Event::PresenceProbe => "PRESENCE_PROBE",
            Event::PrivateCommand => "PRIVATE_COMMAND",
            Event::Publish => "PUBLISH",
            Event::QueueLen => "QUEUE_LEN",
            Event::RecordStart => "RECORD_START",
            Event::RecordStop => "RECORD_STOP",
            Event::RecvInfo => "RECV_INFO",
            Event::RecvMessage => "RECV_MESSAGE",
            Event::RecvRtcpMessage => "RECV_RTCP_MESSAGE",
            Event::Recycle => "RECYCLE",
            Event::ReloadXml => "RELOADXML",
            Event::RequestParams => "REQUEST_PARAMS",
            Event::ReSchedule => "RE_SCHEDULE",
            Event::Roster => "ROSTER",
            Event::SendInfo => "SEND_INFO",
            Event::SendMessage => "SEND_MESSAGE",
            Event::SessionHeartbeat => "SESSION_HEARTBEAT",
            Event::Shutdown => "SHUTDOWN",
            Event::Startup => "STARTUP",
            Event::SubclassAny => "SUBCLASS_ANY",
            Event::Talk => "TALK",
            Event::Trap => "TRAP",
            Event::Unpublish => "UNPUBLISH",
            Event::None => "Invalid Event",
        };
        write!(f, "{}", s)
    }
}
