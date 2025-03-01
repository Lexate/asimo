mod Types {
    enum tILoopSamplerLoops {
        LOOPSAMPLER_LOOP_A,          //A
        LOOPSAMPLER_LOOP_F,          //F
        LOOPSAMPLER_LOOP_N,          //N
        LOOPSAMPLER_LOOP_G1,         //Guide 1
        LOOPSAMPLER_LOOP_G2,         //Guide 2
        LOOPSAMPLER_LOOP_G3,         //Guide 3
        LOOPSAMPLER_NUMBER_OF_LOOPS, //
    }
    impl Hcp_type for tILoopSamplerLoops {
        fn u8_to_variant(value: u8) -> Self {
            match value {
                0 => Self::LOOPSAMPLER_LOOP_A,
                1 => Self::LOOPSAMPLER_LOOP_F,
                2 => Self::LOOPSAMPLER_LOOP_N,
                3 => Self::LOOPSAMPLER_LOOP_G1,
                4 => Self::LOOPSAMPLER_LOOP_G2,
                5 => Self::LOOPSAMPLER_LOOP_G3,
                6 => Self::LOOPSAMPLER_NUMBER_OF_LOOPS,
                _ => Self,
            }
        }
        fn to_u8(value: u8) -> Self {
            match value {
                LOOPSAMPLER_LOOP_A => 0,
                LOOPSAMPLER_LOOP_F => 1,
                LOOPSAMPLER_LOOP_N => 2,
                LOOPSAMPLER_LOOP_G1 => 3,
                LOOPSAMPLER_LOOP_G2 => 4,
                LOOPSAMPLER_LOOP_G3 => 5,
                LOOPSAMPLER_NUMBER_OF_LOOPS => 6,
                _ => 0,
            }
        }
    }
    enum tIMowerApp_MowerMode {
        IMOWERAPP_MODE_AUTO,   //Auto
        IMOWERAPP_MODE_MANUAL, //Manual
        IMOWERAPP_MODE_HOME,   //Home
        IMOWERAPP_MODE_DEMO,   //Demo
    }
    impl Hcp_type for tIMowerApp_MowerMode {
        fn u8_to_variant(value: u8) -> Self {
            match value {
                0 => Self::IMOWERAPP_MODE_AUTO,
                1 => Self::IMOWERAPP_MODE_MANUAL,
                2 => Self::IMOWERAPP_MODE_HOME,
                3 => Self::IMOWERAPP_MODE_DEMO,
                _ => Self,
            }
        }
        fn to_u8(value: u8) -> Self {
            match value {
                IMOWERAPP_MODE_AUTO => 0,
                IMOWERAPP_MODE_MANUAL => 1,
                IMOWERAPP_MODE_HOME => 2,
                IMOWERAPP_MODE_DEMO => 3,
                _ => 0,
            }
        }
    }
    enum tIMowerApp_State {
        IMOWERAPP_STATE_OFF,                //Off
        IMOWERAPP_STATE_WAIT_FOR_SAFETYPIN, //Wait for safety pin
        IMOWERAPP_STATE_STOPPED,            //Stopped
        IMOWERAPP_STATE_FATAL_ERROR,        //Fatal error
        IMOWERAPP_STATE_PENDING_START,      //Pending start
        IMOWERAPP_STATE_PAUSED,             //Paused
        IMOWERAPP_STATE_IN_OPERATION,       //In operation
        IMOWERAPP_STATE_RESTRICTED,         //Restricted
        IMOWERAPP_STATE_ERROR,              //Error
    }
    impl Hcp_type for tIMowerApp_State {
        fn u8_to_variant(value: u8) -> Self {
            match value {
                0 => Self::IMOWERAPP_STATE_OFF,
                1 => Self::IMOWERAPP_STATE_WAIT_FOR_SAFETYPIN,
                2 => Self::IMOWERAPP_STATE_STOPPED,
                3 => Self::IMOWERAPP_STATE_FATAL_ERROR,
                4 => Self::IMOWERAPP_STATE_PENDING_START,
                5 => Self::IMOWERAPP_STATE_PAUSED,
                6 => Self::IMOWERAPP_STATE_IN_OPERATION,
                7 => Self::IMOWERAPP_STATE_RESTRICTED,
                8 => Self::IMOWERAPP_STATE_ERROR,
                _ => Self,
            }
        }
        fn to_u8(value: u8) -> Self {
            match value {
                IMOWERAPP_STATE_OFF => 0,
                IMOWERAPP_STATE_WAIT_FOR_SAFETYPIN => 1,
                IMOWERAPP_STATE_STOPPED => 2,
                IMOWERAPP_STATE_FATAL_ERROR => 3,
                IMOWERAPP_STATE_PENDING_START => 4,
                IMOWERAPP_STATE_PAUSED => 5,
                IMOWERAPP_STATE_IN_OPERATION => 6,
                IMOWERAPP_STATE_RESTRICTED => 7,
                IMOWERAPP_STATE_ERROR => 8,
                _ => 0,
            }
        }
    }
    enum tReturn {
        OK,              //OK
        E_UNDEFINED,     //Undefined error
        E_INVALID_VALUE, //Invalid value in argument
        E_TIMEOUT,       //Unexpected timeout
        E_OVERFLOW,      //Overflow in data communication
        E_OUT_OF_MEMORY, //No memory buffer available in pool
        W_UNDEFINED,     //Undefined warning
        W_BUSY,          //Warning: Busy. May need an action
        I_UNDEFINED,     //Undefined info
        I_BUSY,          //Info: Busy. No action needed
        I_QUEUED,        //Info: The call put in queue
    }
    impl Hcp_type for tReturn {
        fn u8_to_variant(value: u8) -> Self {
            match value {
                0 => Self::OK,
                1 => Self::E_UNDEFINED,
                2 => Self::E_INVALID_VALUE,
                3 => Self::E_TIMEOUT,
                4 => Self::E_OVERFLOW,
                5 => Self::E_OUT_OF_MEMORY,
                64 => Self::W_UNDEFINED,
                65 => Self::W_BUSY,
                128 => Self::I_UNDEFINED,
                129 => Self::I_BUSY,
                130 => Self::I_QUEUED,
                _ => Self,
            }
        }
        fn to_u8(value: u8) -> Self {
            match value {
                OK => 0,
                E_UNDEFINED => 1,
                E_INVALID_VALUE => 2,
                E_TIMEOUT => 3,
                E_OVERFLOW => 4,
                E_OUT_OF_MEMORY => 5,
                W_UNDEFINED => 64,
                W_BUSY => 65,
                I_UNDEFINED => 128,
                I_BUSY => 129,
                I_QUEUED => 130,
                _ => 0,
            }
        }
    }
    enum tDeviceTypeGroup {
        DEVICE_TYPE_GROUP_UNDEFINED,     //
        DEVICE_TYPE_GROUP_GPS_BOARD,     //
        DEVICE_TYPE_GROUP_MOWER,         //
        DEVICE_TYPE_GROUP_MMI,           //
        DEVICE_TYPE_GROUP_CS,            //
        DEVICE_TYPE_GROUP_ULTRASONIC,    //
        DEVICE_TYPE_GROUP_MOWER_BOOT,    //
        DEVICE_TYPE_GROUP_MOWER_LOADER,  //
        DEVICE_TYPE_GROUP_COM_UNIT,      //
        DEVICE_TYPE_GROUP_COM_UNIT_BOOT, //
        DEVICE_TYPE_GROUP_MAIN_BOARD,    //
        DEVICE_TYPE_GROUP_MMI_BOARD,     //
        DEVICE_TYPE_GROUP_CS_BOARD,      //
        DEVICE_TYPE_GROUP_US_BOARD,      //
        DEVICE_TYPE_GROUP_COM_BOARD,     //
        DEVICE_TYPE_GROUP_SW_MOWER,      //
        DEVICE_TYPE_GROUP_SW_MMI,        //
        DEVICE_TYPE_GROUP_SW_CS,         //
        DEVICE_TYPE_GROUP_SW_START,      //
        DEVICE_TYPE_GROUP_SW_ULTRASONIC, //
        DEVICE_TYPE_GROUP_SW_COM,        //
    }
    impl Hcp_type for tDeviceTypeGroup {
        fn u8_to_variant(value: u8) -> Self {
            match value {
                0 => Self::DEVICE_TYPE_GROUP_UNDEFINED,
                1 => Self::DEVICE_TYPE_GROUP_GPS_BOARD,
                10 => Self::DEVICE_TYPE_GROUP_MOWER,
                11 => Self::DEVICE_TYPE_GROUP_MMI,
                12 => Self::DEVICE_TYPE_GROUP_CS,
                13 => Self::DEVICE_TYPE_GROUP_ULTRASONIC,
                14 => Self::DEVICE_TYPE_GROUP_MOWER_BOOT,
                15 => Self::DEVICE_TYPE_GROUP_MOWER_LOADER,
                16 => Self::DEVICE_TYPE_GROUP_COM_UNIT,
                17 => Self::DEVICE_TYPE_GROUP_COM_UNIT_BOOT,
                20 => Self::DEVICE_TYPE_GROUP_MAIN_BOARD,
                21 => Self::DEVICE_TYPE_GROUP_MMI_BOARD,
                22 => Self::DEVICE_TYPE_GROUP_CS_BOARD,
                23 => Self::DEVICE_TYPE_GROUP_US_BOARD,
                24 => Self::DEVICE_TYPE_GROUP_COM_BOARD,
                31 => Self::DEVICE_TYPE_GROUP_SW_MOWER,
                32 => Self::DEVICE_TYPE_GROUP_SW_MMI,
                33 => Self::DEVICE_TYPE_GROUP_SW_CS,
                34 => Self::DEVICE_TYPE_GROUP_SW_START,
                35 => Self::DEVICE_TYPE_GROUP_SW_ULTRASONIC,
                36 => Self::DEVICE_TYPE_GROUP_SW_COM,
                _ => Self,
            }
        }
        fn to_u8(value: u8) -> Self {
            match value {
                DEVICE_TYPE_GROUP_UNDEFINED => 0,
                DEVICE_TYPE_GROUP_GPS_BOARD => 1,
                DEVICE_TYPE_GROUP_MOWER => 10,
                DEVICE_TYPE_GROUP_MMI => 11,
                DEVICE_TYPE_GROUP_CS => 12,
                DEVICE_TYPE_GROUP_ULTRASONIC => 13,
                DEVICE_TYPE_GROUP_MOWER_BOOT => 14,
                DEVICE_TYPE_GROUP_MOWER_LOADER => 15,
                DEVICE_TYPE_GROUP_COM_UNIT => 16,
                DEVICE_TYPE_GROUP_COM_UNIT_BOOT => 17,
                DEVICE_TYPE_GROUP_MAIN_BOARD => 20,
                DEVICE_TYPE_GROUP_MMI_BOARD => 21,
                DEVICE_TYPE_GROUP_CS_BOARD => 22,
                DEVICE_TYPE_GROUP_US_BOARD => 23,
                DEVICE_TYPE_GROUP_COM_BOARD => 24,
                DEVICE_TYPE_GROUP_SW_MOWER => 31,
                DEVICE_TYPE_GROUP_SW_MMI => 32,
                DEVICE_TYPE_GROUP_SW_CS => 33,
                DEVICE_TYPE_GROUP_SW_START => 34,
                DEVICE_TYPE_GROUP_SW_ULTRASONIC => 35,
                DEVICE_TYPE_GROUP_SW_COM => 36,
                _ => 0,
            }
        }
    }
    enum tMowerDeviceType {
        MOWER_DEVICE_TYPE_UNDEFINED, //
        MOWER_DEVICE_TYPE_B,         //
        MOWER_DEVICE_TYPE_C,         //
        MOWER_DEVICE_TYPE_D,         //
        MOWER_DEVICE_TYPE_E,         //
        MOWER_DEVICE_TYPE_F,         //
        MOWER_DEVICE_TYPE_G,         //
        MOWER_DEVICE_TYPE_H,         //
        MOWER_DEVICE_TYPE_I,         //
        MOWER_DEVICE_TYPE_J,         //
        MOWER_DEVICE_TYPE_K,         //
        MOWER_DEVICE_TYPE_L,         //
        MOWER_DEVICE_TYPE_M,         //
        MOWER_DEVICE_TYPE_N,         //
        MOWER_DEVICE_TYPE_O,         //
        MOWER_DEVICE_TYPE_P,         //
        MOWER_DEVICE_TYPE_Q,         //
        MOWER_DEVICE_TYPE_NO_MORE,   //
    }
    impl Hcp_type for tMowerDeviceType {
        fn u8_to_variant(value: u8) -> Self {
            match value {
                0 => Self::MOWER_DEVICE_TYPE_UNDEFINED,
                1 => Self::MOWER_DEVICE_TYPE_B,
                2 => Self::MOWER_DEVICE_TYPE_C,
                3 => Self::MOWER_DEVICE_TYPE_D,
                4 => Self::MOWER_DEVICE_TYPE_E,
                5 => Self::MOWER_DEVICE_TYPE_F,
                6 => Self::MOWER_DEVICE_TYPE_G,
                7 => Self::MOWER_DEVICE_TYPE_H,
                8 => Self::MOWER_DEVICE_TYPE_I,
                9 => Self::MOWER_DEVICE_TYPE_J,
                10 => Self::MOWER_DEVICE_TYPE_K,
                11 => Self::MOWER_DEVICE_TYPE_L,
                12 => Self::MOWER_DEVICE_TYPE_M,
                13 => Self::MOWER_DEVICE_TYPE_N,
                14 => Self::MOWER_DEVICE_TYPE_O,
                15 => Self::MOWER_DEVICE_TYPE_P,
                16 => Self::MOWER_DEVICE_TYPE_Q,
                17 => Self::MOWER_DEVICE_TYPE_NO_MORE,
                _ => Self,
            }
        }
        fn to_u8(value: u8) -> Self {
            match value {
                MOWER_DEVICE_TYPE_UNDEFINED => 0,
                MOWER_DEVICE_TYPE_B => 1,
                MOWER_DEVICE_TYPE_C => 2,
                MOWER_DEVICE_TYPE_D => 3,
                MOWER_DEVICE_TYPE_E => 4,
                MOWER_DEVICE_TYPE_F => 5,
                MOWER_DEVICE_TYPE_G => 6,
                MOWER_DEVICE_TYPE_H => 7,
                MOWER_DEVICE_TYPE_I => 8,
                MOWER_DEVICE_TYPE_J => 9,
                MOWER_DEVICE_TYPE_K => 10,
                MOWER_DEVICE_TYPE_L => 11,
                MOWER_DEVICE_TYPE_M => 12,
                MOWER_DEVICE_TYPE_N => 13,
                MOWER_DEVICE_TYPE_O => 14,
                MOWER_DEVICE_TYPE_P => 15,
                MOWER_DEVICE_TYPE_Q => 16,
                MOWER_DEVICE_TYPE_NO_MORE => 17,
                _ => 0,
            }
        }
    }
    enum tMowerVariantType {
        MOWER_VARIANT_TYPE_UNDEFINED, //
        MOWER_VARIANT_TYPE_ORG,       //
        MOWER_VARIANT_TYPE_B,         //
        MOWER_VARIANT_TYPE_C,         //
        MOWER_VARIANT_TYPE_D,         //
        MOWER_VARIANT_TYPE_E,         //
        MOWER_VARIANT_TYPE_F,         //
        MOWER_VARIANT_TYPE_NO_MORE,   //
    }
    impl Hcp_type for tMowerVariantType {
        fn u8_to_variant(value: u8) -> Self {
            match value {
                255 => Self::MOWER_VARIANT_TYPE_UNDEFINED,
                0 => Self::MOWER_VARIANT_TYPE_ORG,
                1 => Self::MOWER_VARIANT_TYPE_B,
                2 => Self::MOWER_VARIANT_TYPE_C,
                3 => Self::MOWER_VARIANT_TYPE_D,
                4 => Self::MOWER_VARIANT_TYPE_E,
                5 => Self::MOWER_VARIANT_TYPE_F,
                6 => Self::MOWER_VARIANT_TYPE_NO_MORE,
                _ => Self,
            }
        }
        fn to_u8(value: u8) -> Self {
            match value {
                MOWER_VARIANT_TYPE_UNDEFINED => 255,
                MOWER_VARIANT_TYPE_ORG => 0,
                MOWER_VARIANT_TYPE_B => 1,
                MOWER_VARIANT_TYPE_C => 2,
                MOWER_VARIANT_TYPE_D => 3,
                MOWER_VARIANT_TYPE_E => 4,
                MOWER_VARIANT_TYPE_F => 5,
                MOWER_VARIANT_TYPE_NO_MORE => 6,
                _ => 0,
            }
        }
    }
    enum tSoundType {
        SOUND_KEY_CLICK,             //Key Click
        SOUND_CLICK,                 //Click Sound
        SOUND_CHARGING_CONNECTED,    //Charging Connected
        SOUND_CHARGING_DISCONNECTED, //Charging Disconnected
        SOUND_DOUBLE_BEEP,           //Double Beep
        SOUND_LONG_BEEP,             //Long Beep
        SOUND_FAULT,                 //Fault
        SOUND_START_CUTTING,         //Start Cutting
        SOUND_ALARM_WARNING,         //Alarm Warning
        SOUND_ALARM_1,               //Alarm 1 minute
        SOUND_ALARM_2,               //Alarm 2 minutes
        SOUND_ALARM_5,               //Alarm 5 minutes
        SOUND_ALARM_10,              //Alarm 10 minutes
        SOUND_TONE_1,                //Tone 1 minute
        SOUND_OFF,                   //Sound Off
    }
    impl Hcp_type for tSoundType {
        fn u8_to_variant(value: u8) -> Self {
            match value {
                0 => Self::SOUND_KEY_CLICK,
                1 => Self::SOUND_CLICK,
                2 => Self::SOUND_CHARGING_CONNECTED,
                3 => Self::SOUND_CHARGING_DISCONNECTED,
                4 => Self::SOUND_DOUBLE_BEEP,
                5 => Self::SOUND_LONG_BEEP,
                6 => Self::SOUND_FAULT,
                7 => Self::SOUND_START_CUTTING,
                8 => Self::SOUND_ALARM_WARNING,
                9 => Self::SOUND_ALARM_1,
                10 => Self::SOUND_ALARM_2,
                11 => Self::SOUND_ALARM_5,
                12 => Self::SOUND_ALARM_10,
                13 => Self::SOUND_TONE_1,
                14 => Self::SOUND_OFF,
                _ => Self,
            }
        }
        fn to_u8(value: u8) -> Self {
            match value {
                SOUND_KEY_CLICK => 0,
                SOUND_CLICK => 1,
                SOUND_CHARGING_CONNECTED => 2,
                SOUND_CHARGING_DISCONNECTED => 3,
                SOUND_DOUBLE_BEEP => 4,
                SOUND_LONG_BEEP => 5,
                SOUND_FAULT => 6,
                SOUND_START_CUTTING => 7,
                SOUND_ALARM_WARNING => 8,
                SOUND_ALARM_1 => 9,
                SOUND_ALARM_2 => 10,
                SOUND_ALARM_5 => 11,
                SOUND_ALARM_10 => 12,
                SOUND_TONE_1 => 13,
                SOUND_OFF => 14,
                _ => 0,
            }
        }
    }
}
mod Commands {
    mod DeviceInformation {
        enum GetDeviceIdentification {
            inParams {},
            outParams {
                deviceTypeGroup: tDeviceTypeGroup,
                mowerDeviceType: tMowerDeviceType,
                mowerSerialNo: u32,
                mowerVariantType: tMowerVariantType,
            },
        }
        impl Hcp for GetDeviceIdentification {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (22, 0)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod RealTimeData {
        enum GetWheelMotorData {
            inParams {},
            outParams {
                powerleft: i16,
                speedleft: i16,
                currentleft: i16,
                powerright: i16,
                speedright: i16,
                currentright: i16,
                difference: i16,
            },
        }
        impl Hcp for GetWheelMotorData {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (20, 2)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum GetBatteryData {
            inParams {},
            outParams {
                batavoltage: u16,
                bataenergylevel: i16,
                batacurrent: i16,
                batatemp: i16,
                batacapacity: i16,
                batbvoltage: u16,
                batbenergylevel: i16,
                batbcurrent: i16,
                batbtemp: i16,
                batbcapacity: i16,
            },
        }
        impl Hcp for GetBatteryData {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (20, 1)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum GetGPSData {
            inParams {},
            outParams {
                quality: u8,
                noofsatellites: u8,
                hdop: u16,
                northsouth: u8,
                eastwest: u8,
                latitudedegreeminute: u32,
                latitudedecimalminute: u32,
                longitudedegreeminute: u32,
                longitudedecimalminute: u32,
                xpos: u16,
                ypos: u16,
                gpstype: u8,
                gpscoverage: u8,
                gpsnavigationstatus: u8,
                gpsstatus: u8,
            },
        }
        impl Hcp for GetGPSData {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (20, 7)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum GetComboardSensorData {
            inParams {},
            outParams {
                pitch: i16,
                roll: i16,
                zacceleration: i16,
                upsidedown: u8,
                mowertemp: i16,
            },
        }
        impl Hcp for GetComboardSensorData {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (20, 140)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum GetSensorData {
            inParams {},
            outParams {
                collision: u8,
                lift: u8,
                pitch: i16,
                roll: i16,
                zacceleration: i16,
                upsidedown: u8,
                mowertemp: i16,
            },
        }
        impl Hcp for GetSensorData {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (20, 4)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod SystemSettings {
        enum SetHeadlightEnabled {
            inParams { headlight: u8 },
            outParams { headlight: u8 },
        }
        impl Hcp for SetHeadlightEnabled {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (2, 0x94)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum GetLoopDetection {
            inParams {},
            outParams { loopDetection: u8 },
        }
        impl Hcp for GetLoopDetection {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (2, 0x08)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum SetLoopDetection {
            inParams { loopDetection: u8 },
            outParams { loopDetection: u8 },
        }
        impl Hcp for SetLoopDetection {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (2, 0x88)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod Wheels {
        enum GetSpeed {
            inParams { index: u8 },
            outParams { speed: i16 },
        }
        impl Hcp for GetSpeed {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4336, 6)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum GetRotationCounter {
            inParams { index: u8 },
            outParams { counter: i32 },
        }
        impl Hcp for GetRotationCounter {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4336, 5)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum PowerOff {
            inParams {},
            outParams {},
        }
        impl Hcp for PowerOff {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4336, 8)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum PowerOn {
            inParams {},
            outParams {},
        }
        impl Hcp for PowerOn {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4336, 9)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod Collision {
        enum GetStatus {
            inParams {},
            outParams {
                collisionFrontCenter: bool,
                collisionRearRight: bool,
                collisionRearLeft: bool,
            },
        }
        impl Hcp for GetStatus {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4166, 2)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum SetSimulation {
            inParams { onOff: bool },
            outParams { onOff: bool },
        }
        impl Hcp for SetSimulation {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4166, 5)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum GetSimulation {
            inParams {},
            outParams { onOff: bool },
        }
        impl Hcp for GetSimulation {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4166, 6)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum SetSimulatedStatus {
            inParams { status: u32 },
            outParams { status: u32 },
        }
        impl Hcp for SetSimulatedStatus {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4166, 7)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum GetSimulatedStatus {
            inParams {},
            outParams { status: u32 },
        }
        impl Hcp for GetSimulatedStatus {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4166, 8)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod Charger {
        enum IsChargingEnabled {
            inParams {},
            outParams { isChargingEnabled: bool },
        }
        impl Hcp for IsChargingEnabled {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4486, 3)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum IsChargingPowerConnected {
            inParams {},
            outParams { isChargingPowerConnected: bool },
        }
        impl Hcp for IsChargingPowerConnected {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4486, 4)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod LiftSensor {
        enum IsActivated {
            inParams {},
            outParams { isActivated: bool },
        }
        impl Hcp for IsActivated {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4476, 0)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod CurrentStatus {
        enum GetStatusKeepAlive {
            inParams {},
            outParams {
                mainState: u8,
                subState: u8,
                mode: u8,
                timerStatusAndOpMode: u8,
                hostMessage: u16,
            },
        }
        impl Hcp for GetStatusKeepAlive {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (18, 0x80)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod LoopSampler {
        enum GetLoopSignalMaster {
            inParams { selectedloop: tILoopSamplerLoops },
            outParams { signalLevel: i16 },
        }
        impl Hcp for GetLoopSignalMaster {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4480, 3)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod StopButton {
        enum IsActivated {
            inParams {},
            outParams { isActivated: bool },
        }
        impl Hcp for IsActivated {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4464, 4)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod HardwareControl {
        enum WheelMotorsPower {
            inParams {
                leftWheelMotorPower: i16,
                rightWheelMotorPower: i16,
            },
            outParams {},
        }
        impl Hcp for WheelMotorsPower {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (16, 2)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod MowerApp {
        enum SetMode {
            inParams {
                modeOfOperation: tIMowerApp_MowerMode,
            },
            outParams {},
        }
        impl Hcp for SetMode {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4586, 0)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum GetMode {
            inParams {},
            outParams {
                modeOfOperation: tIMowerApp_MowerMode,
            },
        }
        impl Hcp for GetMode {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4586, 1)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum GetState {
            inParams {},
            outParams { mowerState: tIMowerApp_State },
        }
        impl Hcp for GetState {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4586, 2)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum StartTrigger {
            inParams {},
            outParams {},
        }
        impl Hcp for StartTrigger {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4586, 4)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum Pause {
            inParams {},
            outParams {},
        }
        impl Hcp for Pause {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4586, 5)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod BladeMotor {
        enum Brake {
            inParams {},
            outParams {},
        }
        impl Hcp for Brake {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4362, 0)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum Run {
            inParams {},
            outParams {},
        }
        impl Hcp for Run {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4362, 1)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum On {
            inParams {},
            outParams {},
        }
        impl Hcp for On {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4362, 9)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum Off {
            inParams {},
            outParams {},
        }
        impl Hcp for Off {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4362, 10)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod HeightMotor {
        enum SetHeight {
            inParams { height: u8 },
            outParams { retVal: tReturn },
        }
        impl Hcp for SetHeight {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4488, 8)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod Sound {
        enum SetSoundType {
            inParams { soundType: tSoundType },
            outParams { soundType: tSoundType },
        }
        impl Hcp for SetSoundType {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4268, 0)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
        enum GetSoundType {
            inParams {},
            outParams { soundType: tSoundType },
        }
        impl Hcp for GetSoundType {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4268, 1)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod SafetySupervisor {
        enum GetStatus {
            inParams {},
            outParams {
                stopButtonPressed: bool,
                onOffSwitchInactive: bool,
                lifted: bool,
                upsideDown: bool,
                tooMuchTilt: bool,
                collision3s: bool,
                tooFarOutsideBoundary: bool,
                noLoopSignalWheels: bool,
                pinCodeNeeded: bool,
                twoSeperateActionsNeededBlade: bool,
                twoSeperateActionsNeededWheels: bool,
                warningSoundNeeded: bool,
                chargingOngoing: bool,
                noLoopSignalBlade: bool,
                collisionIsActive: bool,
                memNotValidated: bool,
                blade10sLift: bool,
                blade10sTilt: bool,
                blade10sCollision: bool,
                bladeUpSideDown: bool,
                powerModeLedBroken: bool,
            },
        }
        impl Hcp for GetStatus {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4466, 0)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
    mod Planner {
        enum ClearOverride {
            inParams {},
            outParams {},
        }
        impl Hcp for ClearOverride {
            fn get_msgtype_subcmd() -> (u16, u8) {
                (4658, 6)
            }
            fn get_outparams() -> Self {
                Self::outParams {}
            }
        }
    }
}
