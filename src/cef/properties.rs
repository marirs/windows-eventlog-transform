use crate::cef::{
    cef_map::CefMap,
};
use std::collections::HashMap;

use convert_case::{Case, Casing};

type EventData = HashMap<String, String>;
const EMPTY_STRING: String = String::new();

struct KeyVal {
    key: String,
    val: String,
}

fn get_cef_key_for(english_field: &str) -> String {
    //! Gets the CEF Key if available or camelCase of the given word(s) as a key
    let cef_mapping = CefMap::from_mapping(include_str!("../../assets/cef_mapping.csv"));
    cef_mapping
        .get_cef_field_for(english_field)
        .unwrap_or(english_field.to_case(Case::Camel))
}

fn get_key_val_for(english_field: &str, xml_field: &str, evt: EventData) -> KeyVal {
    //! Gets a Key & Value mapping
    //!
    //! ## Returns
    //! KeyVal object
    // get the cef field name
    let key = get_cef_key_for(english_field);

    // get the field's value
    let val = evt
        .get(xml_field).unwrap_or(&EMPTY_STRING)
        .to_string();

    KeyVal {
        key,
        val
    }
}

#[allow(non_snake_case)]
fn get_dest_nt_domain_from_SubjectDomainName(evt: EventData) -> KeyVal {
    //! Gets the Destination NT Domain from
    //! Destination NT Domain & SubjectDomainName
    get_key_val_for("Destination NT Domain", "SubjectDomainName", evt)
}

#[allow(non_snake_case)]
fn get_dest_nt_domain_from_TargetUserDomain(evt: EventData) -> KeyVal {
    get_key_val_for("Destination NT Domain", "TargetUserDomain", evt)
}

#[allow(non_snake_case)]
fn get_device_nt_domain_from_SubjectDomainName(evt: EventData) -> KeyVal {
    //! Gets the Device NT Domain from
    //! Device NT Domain & SubjectDomainName
    get_key_val_for("Device NT Domain", "SubjectDomainName", evt)
}

#[allow(non_snake_case)]
fn get_device_nt_domain_from_TargetUserDomain(evt: EventData) -> KeyVal {
    get_key_val_for("Device NT Domain", "TargetUserDomain", evt)
}

#[allow(non_snake_case)]
fn get_dest_user_id_from_SubjectLogonId(evt: EventData) -> KeyVal {
    //! Get `destination user id` from `SubjectLogonId`
    get_key_val_for("Destination User ID", "SubjectLogonId", evt)
}

#[allow(non_snake_case)]
fn get_dest_user_id_from_SubjectUserName_or_SubjectUserSid(evt: EventData) -> KeyVal {
    //! Get `destination user id` from `one_of(SubjectUserName, SubjectUserSid)`
    let dest_user_name_key = get_cef_key_for("Destination User ID");
    let dest_user_name_val = one_of!(
        evt.get("SubjectUserName").unwrap_or(&EMPTY_STRING).to_string(),
        evt.get("SubjectUserSid").unwrap_or(&EMPTY_STRING).to_string()
    );

    KeyVal {
        key: dest_user_name_key.to_string(),
        val: dest_user_name_val.to_string()
    }
}

#[allow(non_snake_case)]
fn get_dest_username_from_SubjectLogonId(evt: EventData) -> KeyVal {
    //! Get `destination user name` from `SubjectLogonId`
    get_key_val_for("Destination User Name", "SubjectLogonId", evt)
}

#[allow(non_snake_case)]
fn get_dest_username_from_SubjectUserName_or_SubjectUserSid(evt: EventData) -> KeyVal {
    //! Get `destination user name` from `one_of(SubjectUserName, SubjectUserSid)`
    let dest_user_name_key = get_cef_key_for("Destination User Name");
    let dest_user_name_val = one_of!(
        evt.get("SubjectUserName").unwrap_or(&EMPTY_STRING).to_string(),
        evt.get("SubjectUserSid").unwrap_or(&EMPTY_STRING).to_string()
    );

    KeyVal {
        key: dest_user_name_key.to_string(),
        val: dest_user_name_val.to_string()
    }
}

#[allow(non_snake_case)]
fn get_dest_user_id_from_TargetLogonId(evt: EventData) -> KeyVal {
    get_key_val_for(
        "Destination User ID", "TargetLogonId", evt
    )
}

#[allow(non_snake_case)]
fn get_dest_username_from_TargetUserName_or_TargetUserSid(evt: EventData) -> KeyVal {
    //! Get `destination user name` from `one_of(TargetUserName, TargetUserSid)`
    let dest_user_name_key = get_cef_key_for("Destination User Name");
    let dest_user_name_val = one_of!(
        evt.get("TargetUserName").unwrap_or(&EMPTY_STRING).to_string(),
        evt.get("TargetUserSid").unwrap_or(&EMPTY_STRING).to_string()
    );

    KeyVal {
        key: dest_user_name_key.to_string(),
        val: dest_user_name_val.to_string()
    }
}

pub(crate) fn event_mappings(event_id: usize, event_data: EventData) -> String {
    //! Map all the events with their corresponding CEF fields
    //! ## Parameters
    //! `event_id` as usize, `EventData` as HashMap
    //! ## Returns
    //! CEF String
    match event_id {
        104 => {
            let channel = get_key_val_for(
                "File Type", "Channel", event_data.clone()
            );
            let src_user_name = get_key_val_for(
                "Source User Name", "SubjectUserName", event_data.clone()
            );
            let dest_domain_name = get_key_val_for(
                "Destination NT Domain", "SubjectDomainName", event_data.clone()
            );

            format!(
                "{}={} {}={} {}={}",
                channel.key, channel.val,
                src_user_name.key, channel.val,
                dest_domain_name.key, dest_domain_name.val
            )
        },
        1102 => {
            let dest_nt_domain = get_dest_nt_domain_from_SubjectDomainName(event_data.clone());
            let dest_user_id = get_dest_user_id_from_SubjectUserName_or_SubjectUserSid(event_data.clone());
            let dest_user_name = get_dest_username_from_SubjectLogonId(event_data.clone());

            format!(
                "{}={} {}={} {}={}",
                dest_nt_domain.key, dest_nt_domain.val,
                dest_user_id.key, dest_user_id.val,
                dest_user_name.key, dest_user_name.val
            )
        },
        4610 => {
            format!("cs5Label=AuthenticationPackageName cs5={}",
                event_data.get("AuthenticationPackageName").unwrap_or(&EMPTY_STRING)
            )
        },
        4611 => {
            let src_process_name = get_key_val_for(
                "Source Process Name", "LogonProcessName", event_data.clone()
            );
            let dest_process_name = get_key_val_for(
                "Destination Process Name", "LogonProcessName", event_data.clone()
            );
            let dest_user_id = get_dest_user_id_from_SubjectLogonId(event_data.clone());
            let dest_user_name = get_dest_username_from_SubjectUserName_or_SubjectUserSid(event_data.clone());
            let dest_nt_domain = get_dest_nt_domain_from_SubjectDomainName(event_data.clone());
            let dvc_nt_domain = get_device_nt_domain_from_SubjectDomainName(event_data.clone());

            format!(
                "{}={} {}={} {}={} {}={} {}={} {}={}",
                src_process_name.key, src_process_name.val,
                dest_process_name.key, dest_process_name.val,
                dest_user_id.key, dest_user_id.val,
                dest_user_name.key, dest_user_name.val,
                dest_nt_domain.key, dest_nt_domain.val,
                dvc_nt_domain.key, dvc_nt_domain.val
            )
        },
        4612 => {
            format!("cn3Label=AuditsDiscarded cn3={} msg={}",
                event_data.get("AuditsDiscarded").unwrap_or(&EMPTY_STRING),
                "This event is generated when audit queues are filled and events must be discarded. This most commonly occurs when security events are being generated faster than they are being written to disk, or when the auditing system loses connectivity to the event log, such as when the event log service is stopped."
            )
        },
        4614 => {
            format!("cs5Label=NotificationPackageName cs5={}",
                event_data.get("NotificationPackageName").unwrap_or(&EMPTY_STRING)
            )
        },
        4615 => {
            // cef key
            let dest_user_id = get_dest_user_id_from_SubjectLogonId(event_data.clone());
            let dest_user_name = get_dest_username_from_SubjectUserName_or_SubjectUserSid(event_data.clone());
            let dest_nt_domain = get_dest_nt_domain_from_SubjectDomainName(event_data.clone());
            let dvc_nt_domain = get_device_nt_domain_from_SubjectDomainName(event_data.clone());

            format!(
                "msg={} {}={} {}={} {}={} {}={}",
                "Windows Local Security Authority (LSA) communicates with the Windows kernel using Local Procedure Call (LPC) ports. If you see this event, an application has inadvertently or intentionally accessed this port which is reserved exclusively for LSA's use. The application (process) should be investigated to ensure that it is not attempting to tamper with this communications channel.",
                dest_user_id.key, dest_user_id.val,
                dest_user_name.key, dest_user_name.val,
                dest_nt_domain.key, dest_nt_domain.val,
                dvc_nt_domain.key, dvc_nt_domain.val
            )
        },
        4616 => {
            let dest_user_id = get_dest_user_id_from_SubjectLogonId(event_data.clone());
            let dest_user_name = get_dest_username_from_SubjectUserName_or_SubjectUserSid(event_data.clone());
            let dest_nt_domain = get_dest_nt_domain_from_SubjectDomainName(event_data.clone());
            let dvc_nt_domain = get_device_nt_domain_from_SubjectDomainName(event_data.clone());
            let dest_proc_name = get_key_val_for(
                "Destination process Name", "ProcessName", event_data.clone()
            );
            let cs3 = get_key_val_for(
                "Device Custom String 3", "ProcessId", event_data.clone()
            );

            format!(
                "msg={} {}={} {}={} {}={} {}={} {}={} cs3Label=ProcessId {}={}",
                "This event is generated when the system time is changed. It is normal for the Windows Time Service, which runs with System privilege, to change the system time on a regular basis. Other system time changes may be indicative of attempts to tamper with the computer.",
                dest_user_id.key, dest_user_id.val,
                dest_user_name.key, dest_user_name.val,
                dest_nt_domain.key, dest_nt_domain.val,
                dvc_nt_domain.key, dvc_nt_domain.val,
                dest_proc_name.key, dest_proc_name.val,
                cs3.key, cs3.val
            )
        },
        4618 => {
            let dest_user_id = get_dest_user_id_from_TargetLogonId(event_data.clone());
            let dest_username = get_dest_username_from_TargetUserName_or_TargetUserSid(event_data.clone());
            let dest_nt_domain = get_dest_nt_domain_from_TargetUserDomain(event_data.clone());
            let dvc_nt_domain = get_device_nt_domain_from_TargetUserDomain(event_data.clone());

            format!(
                "msg={} {}={} {}={} {}={} {}={}",
                "This event is generated when Windows is configured to generate alerts in accordance with the Common Criteria Security Audit Analysis requirements (FAU_SAA) and an auditable event pattern occurs.",
                dest_user_id.key, dest_user_id.val,
                dest_username.key, dest_username.val,
                dest_nt_domain.key, dest_nt_domain.val,
                dvc_nt_domain.key, dvc_nt_domain.val
            )
        },
        4621 => {
            format!("cn2Label=CrashOnAuditFail cn2={} msg={}",
                event_data.get("CrashOnAuditFail").unwrap_or(&EMPTY_STRING),
                "This event is logged after a system reboots following CarshOnAuditFail."
            )
        },
        4622 => {
            format!("{}={}",
                get_cef_key_for("File Path"),
                event_data.get("SecurityPackageName").unwrap_or(&EMPTY_STRING)
            )
        },
        _ => "".to_string()
    }
}
