#![allow(non_camel_case_types)] // I should proably change the python code
#![allow(unused)] // for now

pub mod types {
    #[derive(Clone, Copy)]
    pub enum tILoopSamplerLoops {
        LoopsamplerLoopA = 0,         /* A */
        LoopsamplerLoopF = 1,         /* F */
        LoopsamplerLoopN = 2,         /* N */
        LoopsamplerLoopG1 = 3,        /* Guide 1 */
        LoopsamplerLoopG2 = 4,        /* Guide 2 */
        LoopsamplerLoopG3 = 5,        /* Guide 3 */
        LoopsamplerNumberOfLoops = 6, /*  */
    }
    #[derive(Clone, Copy)]
    pub enum tIMowerApp_MowerMode {
        ImowerappModeAuto = 0,   /* Auto */
        ImowerappModeManual = 1, /* Manual */
        ImowerappModeHome = 2,   /* Home */
        ImowerappModeDemo = 3,   /* Demo */
    }
    #[derive(Clone, Copy)]
    pub enum tIMowerApp_State {
        ImowerappStateOff = 0,              /* Off */
        ImowerappStateWaitForSafetypin = 1, /* Wait for safety pin */
        ImowerappStateStopped = 2,          /* Stopped */
        ImowerappStateFatalError = 3,       /* Fatal error */
        ImowerappStatePendingStart = 4,     /* Pending start */
        ImowerappStatePaused = 5,           /* Paused */
        ImowerappStateInOperation = 6,      /* In operation */
        ImowerappStateRestricted = 7,       /* Restricted */
        ImowerappStateError = 8,            /* Error */
    }
    #[derive(Clone, Copy)]
    pub enum tReturn {
        Ok = 0,            /* OK */
        EUndefined = 1,    /* Undefined error */
        EInvalidValue = 2, /* Invalid value in argument */
        ETimeout = 3,      /* Unexpected timeout */
        EOverflow = 4,     /* Overflow in data communication */
        EOutOfMemory = 5,  /* No memory buffer available in pool */
        WUndefined = 64,   /* Undefined warning */
        WBusy = 65,        /* Warning: Busy. May need an action */
        IUndefined = 128,  /* Undefined info */
        IBusy = 129,       /* Info: Busy. No action needed */
        IQueued = 130,     /* Info: The call put in queue */
    }
    #[derive(Clone, Copy)]
    pub enum tDeviceTypeGroup {
        DeviceTypeGroupUndefined = 0,     /*  */
        DeviceTypeGroupGpsBoard = 1,      /*  */
        DeviceTypeGroupMower = 10,        /*  */
        DeviceTypeGroupMmi = 11,          /*  */
        DeviceTypeGroupCs = 12,           /*  */
        DeviceTypeGroupUltrasonic = 13,   /*  */
        DeviceTypeGroupMowerBoot = 14,    /*  */
        DeviceTypeGroupMowerLoader = 15,  /*  */
        DeviceTypeGroupComUnit = 16,      /*  */
        DeviceTypeGroupComUnitBoot = 17,  /*  */
        DeviceTypeGroupMainBoard = 20,    /*  */
        DeviceTypeGroupMmiBoard = 21,     /*  */
        DeviceTypeGroupCsBoard = 22,      /*  */
        DeviceTypeGroupUsBoard = 23,      /*  */
        DeviceTypeGroupComBoard = 24,     /*  */
        DeviceTypeGroupSwMower = 31,      /*  */
        DeviceTypeGroupSwMmi = 32,        /*  */
        DeviceTypeGroupSwCs = 33,         /*  */
        DeviceTypeGroupSwStart = 34,      /*  */
        DeviceTypeGroupSwUltrasonic = 35, /*  */
        DeviceTypeGroupSwCom = 36,        /*  */
    }
    #[derive(Clone, Copy)]
    pub enum tMowerDeviceType {
        MowerDeviceTypeUndefined = 0, /*  */
        MowerDeviceTypeB = 1,         /*  */
        MowerDeviceTypeC = 2,         /*  */
        MowerDeviceTypeD = 3,         /*  */
        MowerDeviceTypeE = 4,         /*  */
        MowerDeviceTypeF = 5,         /*  */
        MowerDeviceTypeG = 6,         /*  */
        MowerDeviceTypeH = 7,         /*  */
        MowerDeviceTypeI = 8,         /*  */
        MowerDeviceTypeJ = 9,         /*  */
        MowerDeviceTypeK = 10,        /*  */
        MowerDeviceTypeL = 11,        /*  */
        MowerDeviceTypeM = 12,        /*  */
        MowerDeviceTypeN = 13,        /*  */
        MowerDeviceTypeO = 14,        /*  */
        MowerDeviceTypeP = 15,        /*  */
        MowerDeviceTypeQ = 16,        /*  */
        MowerDeviceTypeNoMore = 17,   /*  */
    }
    #[derive(Clone, Copy)]
    pub enum tMowerVariantType {
        MowerVariantTypeUndefined = 255, /*  */
        MowerVariantTypeOrg = 0,         /*  */
        MowerVariantTypeB = 1,           /*  */
        MowerVariantTypeC = 2,           /*  */
        MowerVariantTypeD = 3,           /*  */
        MowerVariantTypeE = 4,           /*  */
        MowerVariantTypeF = 5,           /*  */
        MowerVariantTypeNoMore = 6,      /*  */
    }
    #[derive(Clone, Copy)]
    pub enum tSoundType {
        SoundKeyClick = 0,             /* Key Click */
        SoundClick = 1,                /* Click Sound */
        SoundChargingConnected = 2,    /* Charging Connected */
        SoundChargingDisconnected = 3, /* Charging Disconnected */
        SoundDoubleBeep = 4,           /* Double Beep */
        SoundLongBeep = 5,             /* Long Beep */
        SoundFault = 6,                /* Fault */
        SoundStartCutting = 7,         /* Start Cutting */
        SoundAlarmWarning = 8,         /* Alarm Warning */
        SoundAlarm1 = 9,               /* Alarm 1 minute */
        SoundAlarm2 = 10,              /* Alarm 2 minutes */
        SoundAlarm5 = 11,              /* Alarm 5 minutes */
        SoundAlarm10 = 12,             /* Alarm 10 minutes */
        SoundTone1 = 13,               /* Tone 1 minute */
        SoundOff = 14,                 /* Sound Off */
    }
    #[derive(Clone, Copy)]
    pub enum inParams {
        DeviceInformationGetDeviceIdentification(),
        RealTimeDataGetWheelMotorData(),
        SystemSettingsSetHeadlightEnabled(u8),
        WheelsGetSpeed(u8),
        WheelsGetRotationCounter(u8),
        CollisionGetStatus(),
        ChargerIsChargingEnabled(),
        LiftSensorIsActivated(),
        RealTimeDataGetBatteryData(),
        CurrentStatusGetStatusKeepAlive(),
        LoopSamplerGetLoopSignalMaster(tILoopSamplerLoops),
        StopButtonIsActivated(),
        WheelsPowerOff(),
        WheelsPowerOn(),
        HardwareControlWheelMotorsPower(i16, i16),
        MowerAppSetMode(tIMowerApp_MowerMode),
        SystemSettingsGetLoopDetection(),
        MowerAppGetMode(),
        MowerAppGetState(),
        BladeMotorBrake(),
        BladeMotorRun(),
        BladeMotorOn(),
        BladeMotorOff(),
        MowerAppStartTrigger(),
        MowerAppPause(),
        SystemSettingsSetLoopDetection(u8),
        HeightMotorSetHeight(u8),
        RealTimeDataGetGPSData(),
        CollisionSetSimulation(bool),
        CollisionGetSimulation(),
        CollisionSetSimulatedStatus(u32),
        CollisionGetSimulatedStatus(),
        SoundSetSoundType(tSoundType),
        SoundGetSoundType(),
        RealTimeDataGetComboardSensorData(),
        RealTimeDataGetSensorData(),
        SafetySupervisorGetStatus(),
        ChargerIsChargingPowerConnected(),
        PlannerClearOverride(),
    }
    #[derive(Clone, Copy)]
    pub enum outParams {
        DeviceInformationGetDeviceIdentification(
            tDeviceTypeGroup,
            tMowerDeviceType,
            u32,
            tMowerVariantType,
        ),
        RealTimeDataGetWheelMotorData(i16, i16, i16, i16, i16, i16, i16),
        SystemSettingsSetHeadlightEnabled(u8),
        WheelsGetSpeed(i16),
        WheelsGetRotationCounter(i32),
        CollisionGetStatus(bool, bool, bool),
        ChargerIsChargingEnabled(bool),
        LiftSensorIsActivated(bool),
        RealTimeDataGetBatteryData(u16, i16, i16, i16, i16, u16, i16, i16, i16, i16),
        CurrentStatusGetStatusKeepAlive(u8, u8, u8, u8, u16),
        LoopSamplerGetLoopSignalMaster(i16),
        StopButtonIsActivated(bool),
        WheelsPowerOff(),
        WheelsPowerOn(),
        HardwareControlWheelMotorsPower(),
        MowerAppSetMode(),
        SystemSettingsGetLoopDetection(u8),
        MowerAppGetMode(tIMowerApp_MowerMode),
        MowerAppGetState(tIMowerApp_State),
        BladeMotorBrake(),
        BladeMotorRun(),
        BladeMotorOn(),
        BladeMotorOff(),
        MowerAppStartTrigger(),
        MowerAppPause(),
        SystemSettingsSetLoopDetection(u8),
        HeightMotorSetHeight(tReturn),
        RealTimeDataGetGPSData(
            u8,
            u8,
            u16,
            u8,
            u8,
            u32,
            u32,
            u32,
            u32,
            u16,
            u16,
            u8,
            u8,
            u8,
            u8,
        ),
        CollisionSetSimulation(bool),
        CollisionGetSimulation(bool),
        CollisionSetSimulatedStatus(u32),
        CollisionGetSimulatedStatus(u32),
        SoundSetSoundType(tSoundType),
        SoundGetSoundType(tSoundType),
        RealTimeDataGetComboardSensorData(i16, i16, i16, u8, i16),
        RealTimeDataGetSensorData(u8, u8, i16, i16, i16, u8, i16),
        SafetySupervisorGetStatus(
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
        ),
        ChargerIsChargingPowerConnected(bool),
        PlannerClearOverride(),
    }

    pub fn get_msgtype(param: inParams) -> (u16, u8) {
        match param {
            inParams::DeviceInformationGetDeviceIdentification() => (22, 0),
            inParams::RealTimeDataGetWheelMotorData() => (20, 2),
            inParams::SystemSettingsSetHeadlightEnabled(..) => (2, 0x94),
            inParams::WheelsGetSpeed(..) => (4336, 6),
            inParams::WheelsGetRotationCounter(..) => (4336, 5),
            inParams::CollisionGetStatus() => (4166, 2),
            inParams::ChargerIsChargingEnabled() => (4486, 3),
            inParams::LiftSensorIsActivated() => (4476, 0),
            inParams::RealTimeDataGetBatteryData() => (20, 1),
            inParams::CurrentStatusGetStatusKeepAlive() => (18, 0x80),
            inParams::LoopSamplerGetLoopSignalMaster(..) => (4480, 3),
            inParams::StopButtonIsActivated() => (4464, 4),
            inParams::WheelsPowerOff() => (4336, 8),
            inParams::WheelsPowerOn() => (4336, 9),
            inParams::HardwareControlWheelMotorsPower(..) => (16, 2),
            inParams::MowerAppSetMode(..) => (4586, 0),
            inParams::SystemSettingsGetLoopDetection() => (2, 0x08),
            inParams::MowerAppGetMode() => (4586, 1),
            inParams::MowerAppGetState() => (4586, 2),
            inParams::BladeMotorBrake() => (4362, 0),
            inParams::BladeMotorRun() => (4362, 1),
            inParams::BladeMotorOn() => (4362, 9),
            inParams::BladeMotorOff() => (4362, 10),
            inParams::MowerAppStartTrigger() => (4586, 4),
            inParams::MowerAppPause() => (4586, 5),
            inParams::SystemSettingsSetLoopDetection(..) => (2, 0x88),
            inParams::HeightMotorSetHeight(..) => (4488, 8),
            inParams::RealTimeDataGetGPSData() => (20, 7),
            inParams::CollisionSetSimulation(..) => (4166, 5),
            inParams::CollisionGetSimulation() => (4166, 6),
            inParams::CollisionSetSimulatedStatus(..) => (4166, 7),
            inParams::CollisionGetSimulatedStatus() => (4166, 8),
            inParams::SoundSetSoundType(..) => (4268, 0),
            inParams::SoundGetSoundType() => (4268, 1),
            inParams::RealTimeDataGetComboardSensorData() => (20, 140),
            inParams::RealTimeDataGetSensorData() => (20, 4),
            inParams::SafetySupervisorGetStatus() => (4466, 0),
            inParams::ChargerIsChargingPowerConnected() => (4486, 4),
            inParams::PlannerClearOverride() => (4658, 6),
        }
    }
}
