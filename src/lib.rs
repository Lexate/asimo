#![allow(non_camel_case_types)] // I should proably change the python code
#![allow(unused)] // for now

pub mod comms {
    use std::time::Duration;

    use serialport::SerialPort;

    pub struct Serial {
        port: Box<dyn SerialPort>,
    }

    impl Serial {
        pub fn new(
            path: &str,
            baud_rate: u32,
            timeout: Duration,
        ) -> Result<Self, serialport::Error> {
            let port = serialport::new(path, baud_rate).timeout(timeout).open()?;

            Ok(Self { port })
        }

        pub fn write(&mut self, output: &[u8]) -> Result<usize, std::io::Error> {
            self.port.write(&output[..])
        }

        pub fn read(&mut self) -> Result<(usize, Vec<u8>), std::io::Error> {
            let mut serial_buf: Vec<u8> = vec![0; 256];
            let bytes_written = self.port.read(serial_buf.as_mut_slice())?;
            Ok((bytes_written, serial_buf))
        }

        pub fn write_read(&mut self, output: &[u8]) -> Result<Vec<u8>, std::io::Error> {
            self.write(output)?;
            let (_, response) = self.read()?;

            Ok(response)
        }
    }
}

pub mod proto {
    const CRC_TABLE: [u8; 256] = [
        0, 94, 188, 226, 97, 63, 221, 131, 194, 156, 126, 32, 163, 253, 31, 65, 157, 195, 33, 127,
        252, 162, 64, 30, 95, 1, 227, 189, 62, 96, 130, 220, 35, 125, 159, 193, 66, 28, 254, 160,
        225, 191, 93, 3, 128, 222, 60, 98, 190, 224, 2, 92, 223, 129, 99, 61, 124, 34, 192, 158,
        29, 67, 161, 255, 70, 24, 250, 164, 39, 121, 155, 197, 132, 218, 56, 102, 229, 187, 89, 7,
        219, 133, 103, 57, 186, 228, 6, 88, 25, 71, 165, 251, 120, 38, 196, 154, 101, 59, 217, 135,
        4, 90, 184, 230, 167, 249, 27, 69, 198, 152, 122, 36, 248, 166, 68, 26, 153, 199, 37, 123,
        58, 100, 134, 216, 91, 5, 231, 185, 140, 210, 48, 110, 237, 179, 81, 15, 78, 16, 242, 172,
        47, 113, 147, 205, 17, 79, 173, 243, 112, 46, 204, 146, 211, 141, 111, 49, 178, 236, 14,
        80, 175, 241, 19, 77, 206, 144, 114, 44, 109, 51, 209, 143, 12, 82, 176, 238, 50, 108, 142,
        208, 83, 13, 239, 177, 240, 174, 76, 18, 145, 207, 45, 115, 202, 148, 118, 40, 171, 245,
        23, 73, 8, 86, 180, 234, 105, 55, 213, 139, 87, 9, 235, 181, 54, 104, 138, 212, 149, 203,
        41, 119, 244, 170, 72, 22, 233, 183, 85, 11, 136, 214, 52, 106, 43, 117, 151, 201, 74, 20,
        246, 168, 116, 42, 200, 150, 21, 75, 169, 247, 182, 232, 10, 84, 215, 137, 107, 53,
    ];

    pub fn calc_crc(array: &Vec<u8>, start: usize) -> u8 {
        let mut crc = 0;
        for i in &array[start..] {
            crc = CRC_TABLE[(crc ^ i) as usize]
        }
        crc
    }

    pub struct AmProto {
        msgtype: u8,
        subcmd: u8,
        payload: inParams,
    }

    impl AmProto {
        pub fn to_byte_array(&self) -> Vec<u8> {
            let mut array = Vec::new();

            array.push(0x02); // STX
            array.push(self.msgtype); // msgtype
                                      // length, (added later)
            array.push(self.subcmd); // subcmd
            array.append(self.payload.to_byte_array().as_mut());
            array.insert(2, (array.len() - 2) as u8); //length, -2 to skip STX and msgtype
            array.push(calc_crc(&array, 1)); //crc, 1 to skip STX
            array.push(0x03); // ETX

            array
        }
        pub fn new(command: inParams) -> Self {
            let (msgtype, subcmd) = get_msgtype(command);
            Self {
                msgtype: msgtype.try_into().expect("amg3 command sent to amproto"),
                subcmd,
                payload: command,
            }
        }
    }

    struct Amg3 {
        id: u8,
        length: u16,
        trans_id: u8,
        message_type: u16, // Can be u8 if the first byte is larger than 0x7F
        payload_length: u8,
        payload: inParams,
        crc: u8,
    }
    #[derive(Clone, Copy)]
    enum tILoopSamplerLoops {
        LoopsamplerLoopA = 0,         /* A */
        LoopsamplerLoopF = 1,         /* F */
        LoopsamplerLoopN = 2,         /* N */
        LoopsamplerLoopG1 = 3,        /* Guide 1 */
        LoopsamplerLoopG2 = 4,        /* Guide 2 */
        LoopsamplerLoopG3 = 5,        /* Guide 3 */
        LoopsamplerNumberOfLoops = 6, /*  */
    }
    #[derive(Clone, Copy)]
    enum tIMowerApp_MowerMode {
        ImowerappModeAuto = 0,   /* Auto */
        ImowerappModeManual = 1, /* Manual */
        ImowerappModeHome = 2,   /* Home */
        ImowerappModeDemo = 3,   /* Demo */
    }
    #[derive(Clone, Copy)]
    enum tIMowerApp_State {
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
    enum tReturn {
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
    enum tDeviceTypeGroup {
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
    enum tMowerDeviceType {
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
    enum tMowerVariantType {
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
    enum tSoundType {
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
    impl inParams {
        fn to_byte_array(&self) -> Vec<u8> {
            match *self {
                inParams::SystemSettingsSetHeadlightEnabled(p) => vec![p],
                inParams::WheelsGetSpeed(p) => vec![p],
                inParams::WheelsGetRotationCounter(p) => vec![p],
                inParams::LoopSamplerGetLoopSignalMaster(p) => vec![p as u8],
                inParams::HardwareControlWheelMotorsPower(p, q) => {
                    [p.to_le_bytes(), q.to_le_bytes()].concat()
                }
                inParams::MowerAppSetMode(p) => vec![p as u8],
                inParams::SystemSettingsSetLoopDetection(p) => vec![p],
                inParams::HeightMotorSetHeight(p) => vec![p],
                inParams::CollisionSetSimulation(p) => vec![p as u8],
                inParams::CollisionSetSimulatedStatus(p) => Vec::from(p.to_le_bytes()),
                inParams::SoundSetSoundType(p) => vec![p as u8],
                _ => Vec::new(),
            }
        }
    }
    #[derive(Clone, Copy)]
    enum outParams {
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

    fn get_msgtype(param: inParams) -> (u16, u8) {
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
