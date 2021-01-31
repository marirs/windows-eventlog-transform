use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use crate::mappings::get_map;

fn get_sa_tasks_mapping(task_id: usize) -> String {
    let tasks_map: HashMap<usize, String> = get_map(SA_TASKS, '\n', ',');

    if let Some(task_name) = tasks_map.get(&task_id) {
        task_name.to_string()
    } else {
        format!("({})", task_id)
    }
}

fn get_setup_tasks_mapping(task_id: usize) -> String {
    let tasks_map: HashMap<usize, String> = get_map(SETUP_TASKS, '\n', ',');

    if let Some(task_name) = tasks_map.get(&task_id) {
        task_name.to_string()
    } else {
        format!("({})", task_id)
    }
}

fn get_event_log_tasks_mapping(task_id: usize) -> String {
    let tasks_map: HashMap<usize, String> = get_map(EVENT_LOG_TASKS, '\n', ',');

    if let Some(task_name) = tasks_map.get(&task_id) {
        task_name.to_string()
    } else {
        format!("({})", task_id)
    }
}

pub(crate) fn tasks_map<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
{
    usize::deserialize(deserializer).map(|x| {
        let mapped = get_sa_tasks_mapping(x);
        match x {
            0 => "None",
            _ => &mapped
        }
            .into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sa_tasks_map() {
        let evt = get_sa_tasks_mapping(12289);
        assert_eq!(evt, "Security System Extension");
        let evt = get_sa_tasks_mapping(1);
        assert_eq!(evt, "(1)")
    }

    #[test]
    fn test_event_log_tasks_map() {
        let evt = get_event_log_tasks_mapping(103);
        assert_eq!(evt, "Service shutdown");
        let evt = get_event_log_tasks_mapping(1);
        assert_eq!(evt, "(1)")
    }

    #[test]
    fn test_setup_tasks_map() {
        let evt = get_setup_tasks_mapping(3000);
        assert_eq!(evt, "Setup information");
        let evt = get_setup_tasks_mapping(1);
        assert_eq!(evt, "(1)")
    }

}

const SA_TASKS: &str = r#"SE_ADT_SYSTEM_SECURITYSTATECHANGE,12288,Security State Change
SE_ADT_SYSTEM_SECURITYSUBSYSTEMEXTENSION,12289,Security System Extension
SE_ADT_SYSTEM_INTEGRITY,12290,System Integrity
SE_ADT_SYSTEM_IPSECDRIVEREVENTS,12291,IPsec Driver
SE_ADT_SYSTEM_OTHERS,12292,Other System Events
SE_ADT_LOGON_LOGON,12544,Logon
SE_ADT_LOGON_LOGOFF,12545,Logoff
SE_ADT_LOGON_ACCOUNTLOCKOUT,12546,Account Lockout
SE_ADT_LOGON_IPSECMAINMODE,12547,IPsec Main Mode
SE_ADT_LOGON_SPECIALLOGON,12548,Special Logon
SE_ADT_LOGON_IPSECQUICKMODE,12549,IPsec Quick Mode
SE_ADT_LOGON_IPSECUSERMODE,12550,IPsec Extended Mode
SE_ADT_LOGON_OTHERS,12551,Other Logon/Logoff Events
SE_ADT_LOGON_NPS,12552,Network Policy Server
SE_ADT_LOGON_CLAIMS,12553,User / Device Claims
SE_ADT_LOGON_GROUPS,12554,Group Membership
SE_ADT_OBJECTACCESS_FILESYSTEM,12800,File System
SE_ADT_OBJECTACCESS_REGISTRY,12801,Registry
SE_ADT_OBJECTACCESS_KERNEL,12802,Kernel Object
SE_ADT_OBJECTACCESS_SAM,12803,SAM
SE_ADT_OBJECTACCESS_OTHER,12804,Other Object Access Events
SE_ADT_OBJECTACCESS_CERTIFICATIONAUTHORITY,12805,Certification Services
SE_ADT_OBJECTACCESS_APPLICATIONGENERATED,12806,Application Generated
SE_ADT_OBJECTACCESS_HANDLE,12807,Handle Manipulation
SE_ADT_OBJECTACCESS_SHARE,12808,File Share
SE_ADT_OBJECTACCESS_FIREWALLPACKETDROPS,12809,Filtering Platform Packet Drop
SE_ADT_OBJECTACCESS_FIREWALLCONNECTION,12810,Filtering Platform Connection
SE_ADT_OBJECTACCESS_DETAILEDFILESHARE,12811,Detailed File Share
SE_ADT_OBJECTACCESS_REMOVABLESTORAGE,12812,Removable Storage
SE_ADT_OBJECTACCESS_CBACSTAGING,12813,Central Access Policy Staging
SE_ADT_PRIVILEGEUSE_SENSITIVE,13056,Sensitive Privilege Use
SE_ADT_PRIVILEGEUSE_NONSENSITIVE,13057,Non Sensitive Privilege Use
SE_ADT_PRIVILEGEUSE_OTHERS,13058,Other Privilege Use Events
SE_ADT_DETAILEDTRACKING_PROCESSCREATION,13312,Process Creation
SE_ADT_DETAILEDTRACKING_PROCESSTERMINATION,13313,Process Termination
SE_ADT_DETAILEDTRACKING_DPAPIACTIVITY,13314,DPAPI Activity
SE_ADT_DETAILEDTRACKING_RPCCALL,13315,RPC Events
SE_ADT_DETAILEDTRACKING_PNPACTIVITY,13316,Plug and Play Events
SE_ADT_DETAILEDTRACKING_TOKENRIGHTADJ,13317,Token Right Adjusted Events
SE_ADT_POLICYCHANGE_AUDITPOLICY,13568,Audit Policy Change
SE_ADT_POLICYCHANGE_AUTHENTICATIONPOLICY,13569,Authentication Policy Change
SE_ADT_POLICYCHANGE_AUTHORIZATIONPOLICY,13570,Authorization Policy Change
SE_ADT_POLICYCHANGE_MPSSCVRULEPOLICY,13571,MPSSVC Rule-Level Policy Change
SE_ADT_POLICYCHANGE_WFPIPSECPOLICY,13572,Filtering Platform Policy Change
SE_ADT_POLICYCHANGE_OTHERS,13573,Other Policy Change Events
SE_ADT_ACCOUNTMANAGEMENT_USERACCOUNT,13824,User Account Management
SE_ADT_ACCOUNTMANAGEMENT_COMPUTERACCOUNT,13825,Computer Account Management
SE_ADT_ACCOUNTMANAGEMENT_SECURITYGROUP,13826,Security Group Management
SE_ADT_ACCOUNTMANAGEMENT_DISTRIBUTIONGROUP,13827,Distribution Group Management
SE_ADT_ACCOUNTMANAGEMENT_APPLICATIONGROUP,13828,Application Group Management
SE_ADT_ACCOUNTMANAGEMENT_OTHERS,13829,Other Account Management Events
SE_ADT_DSACCESS_DSACCESS,14080,Directory Service Access
SE_ADT_DSACCESS_DSCHANGES,14081,Directory Service Changes
SE_ADT_DS_REPLICATION,14082,Directory Service Replication
SE_ADT_DS_DETAILED_REPLICATION,14083,Detailed Directory Service Replication
SE_ADT_ACCOUNTLOGON_CREDENTIALVALIDATION,14336,Credential Validation
SE_ADT_ACCOUNTLOGON_KERBEROS,14337,Kerberos Service Ticket Operations
SE_ADT_ACCOUNTLOGON_OTHERS,14338,Other Account Logon Events
SE_ADT_ACCOUNTLOGON_KERBCREDENTIALVALIDATION,14339,Kerberos Authentication Service
SE_ADT_UNKNOWN_SUBCATEGORY,65280,Subcategory could not be determined"#;

const EVENT_LOG_TASKS: &str = r#"el:Initialization,100,Service startup
el:EventProcessing,101,Event processing
el:Shutdown,103,Service shutdown
el:LogClear,104,Log clear
el:AutoBackup,105,Log automatic backup
el:AbnormalShutdown,108,System Abnormal Shutdown
el:UsageAudit,109,Service Usage Audit"#;

const SETUP_TASKS: &str = r#"tskExecuteSetupPhase,1000,Execute Setup Phase
tskSysprepSpecialize,2000,Sysprep Specialise
tskSetupInformation,3000,Setup information
tskNewOSInformation,4000,OS information"#;