#![allow(non_camel_case_types)] // I should proably change the python code
#![allow(non_snake_case)]
#![allow(unused)] // for now
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::type_methods::{Hcp, HcpType, Msgtype};

pub mod Types {
    use super::{Deserialize, Error, HcpType, Result, Serialize};
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum tILoopSamplerLoops {
        LOOPSAMPLER_LOOP_A,          //A
        LOOPSAMPLER_LOOP_F,          //F
        LOOPSAMPLER_LOOP_N,          //N
        LOOPSAMPLER_LOOP_G1,         //Guide 1
        LOOPSAMPLER_LOOP_G2,         //Guide 2
        LOOPSAMPLER_LOOP_G3,         //Guide 3
        LOOPSAMPLER_NUMBER_OF_LOOPS, //
    }
    impl HcpType for tILoopSamplerLoops {
        fn u8_to_variant(value: u8) -> Result<impl HcpType> {
            match value {
                0 => Ok(Self::LOOPSAMPLER_LOOP_A),
                1 => Ok(Self::LOOPSAMPLER_LOOP_F),
                2 => Ok(Self::LOOPSAMPLER_LOOP_N),
                3 => Ok(Self::LOOPSAMPLER_LOOP_G1),
                4 => Ok(Self::LOOPSAMPLER_LOOP_G2),
                5 => Ok(Self::LOOPSAMPLER_LOOP_G3),
                6 => Ok(Self::LOOPSAMPLER_NUMBER_OF_LOOPS),
                v => Err(Error::DoesNotCorespondToVariant(format!(
                    "Value {v} does not corespond to a variant in tILoopSamplerLoops"
                ))),
            }
        }
        fn to_u8(value: Self) -> u8 {
            match value {
                Self::LOOPSAMPLER_LOOP_A => 0,
                Self::LOOPSAMPLER_LOOP_F => 1,
                Self::LOOPSAMPLER_LOOP_N => 2,
                Self::LOOPSAMPLER_LOOP_G1 => 3,
                Self::LOOPSAMPLER_LOOP_G2 => 4,
                Self::LOOPSAMPLER_LOOP_G3 => 5,
                Self::LOOPSAMPLER_NUMBER_OF_LOOPS => 6,
            }
        }
    }
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum tIMowerApp_MowerMode {
        IMOWERAPP_MODE_AUTO,   //Auto
        IMOWERAPP_MODE_MANUAL, //Manual
        IMOWERAPP_MODE_HOME,   //Home
        IMOWERAPP_MODE_DEMO,   //Demo
    }
    impl HcpType for tIMowerApp_MowerMode {
        fn u8_to_variant(value: u8) -> Result<impl HcpType> {
            match value {
                0 => Ok(Self::IMOWERAPP_MODE_AUTO),
                1 => Ok(Self::IMOWERAPP_MODE_MANUAL),
                2 => Ok(Self::IMOWERAPP_MODE_HOME),
                3 => Ok(Self::IMOWERAPP_MODE_DEMO),
                v => Err(Error::DoesNotCorespondToVariant(format!(
                    "Value {v} does not corespond to a variant in tIMowerApp_MowerMode"
                ))),
            }
        }
        fn to_u8(value: Self) -> u8 {
            match value {
                Self::IMOWERAPP_MODE_AUTO => 0,
                Self::IMOWERAPP_MODE_MANUAL => 1,
                Self::IMOWERAPP_MODE_HOME => 2,
                Self::IMOWERAPP_MODE_DEMO => 3,
            }
        }
    }
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum tIMowerApp_State {
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
    impl HcpType for tIMowerApp_State {
        fn u8_to_variant(value: u8) -> Result<impl HcpType> {
            match value {
                0 => Ok(Self::IMOWERAPP_STATE_OFF),
                1 => Ok(Self::IMOWERAPP_STATE_WAIT_FOR_SAFETYPIN),
                2 => Ok(Self::IMOWERAPP_STATE_STOPPED),
                3 => Ok(Self::IMOWERAPP_STATE_FATAL_ERROR),
                4 => Ok(Self::IMOWERAPP_STATE_PENDING_START),
                5 => Ok(Self::IMOWERAPP_STATE_PAUSED),
                6 => Ok(Self::IMOWERAPP_STATE_IN_OPERATION),
                7 => Ok(Self::IMOWERAPP_STATE_RESTRICTED),
                8 => Ok(Self::IMOWERAPP_STATE_ERROR),
                v => Err(Error::DoesNotCorespondToVariant(format!(
                    "Value {v} does not corespond to a variant in tIMowerApp_State"
                ))),
            }
        }
        fn to_u8(value: Self) -> u8 {
            match value {
                Self::IMOWERAPP_STATE_OFF => 0,
                Self::IMOWERAPP_STATE_WAIT_FOR_SAFETYPIN => 1,
                Self::IMOWERAPP_STATE_STOPPED => 2,
                Self::IMOWERAPP_STATE_FATAL_ERROR => 3,
                Self::IMOWERAPP_STATE_PENDING_START => 4,
                Self::IMOWERAPP_STATE_PAUSED => 5,
                Self::IMOWERAPP_STATE_IN_OPERATION => 6,
                Self::IMOWERAPP_STATE_RESTRICTED => 7,
                Self::IMOWERAPP_STATE_ERROR => 8,
            }
        }
    }
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum tReturn {
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
    impl HcpType for tReturn {
        fn u8_to_variant(value: u8) -> Result<impl HcpType> {
            match value {
                0 => Ok(Self::OK),
                1 => Ok(Self::E_UNDEFINED),
                2 => Ok(Self::E_INVALID_VALUE),
                3 => Ok(Self::E_TIMEOUT),
                4 => Ok(Self::E_OVERFLOW),
                5 => Ok(Self::E_OUT_OF_MEMORY),
                64 => Ok(Self::W_UNDEFINED),
                65 => Ok(Self::W_BUSY),
                128 => Ok(Self::I_UNDEFINED),
                129 => Ok(Self::I_BUSY),
                130 => Ok(Self::I_QUEUED),
                v => Err(Error::DoesNotCorespondToVariant(format!(
                    "Value {v} does not corespond to a variant in tReturn"
                ))),
            }
        }
        fn to_u8(value: Self) -> u8 {
            match value {
                Self::OK => 0,
                Self::E_UNDEFINED => 1,
                Self::E_INVALID_VALUE => 2,
                Self::E_TIMEOUT => 3,
                Self::E_OVERFLOW => 4,
                Self::E_OUT_OF_MEMORY => 5,
                Self::W_UNDEFINED => 64,
                Self::W_BUSY => 65,
                Self::I_UNDEFINED => 128,
                Self::I_BUSY => 129,
                Self::I_QUEUED => 130,
            }
        }
    }
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum tDeviceTypeGroup {
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
    impl HcpType for tDeviceTypeGroup {
        fn u8_to_variant(value: u8) -> Result<impl HcpType> {
            match value {
                0 => Ok(Self::DEVICE_TYPE_GROUP_UNDEFINED),
                1 => Ok(Self::DEVICE_TYPE_GROUP_GPS_BOARD),
                10 => Ok(Self::DEVICE_TYPE_GROUP_MOWER),
                11 => Ok(Self::DEVICE_TYPE_GROUP_MMI),
                12 => Ok(Self::DEVICE_TYPE_GROUP_CS),
                13 => Ok(Self::DEVICE_TYPE_GROUP_ULTRASONIC),
                14 => Ok(Self::DEVICE_TYPE_GROUP_MOWER_BOOT),
                15 => Ok(Self::DEVICE_TYPE_GROUP_MOWER_LOADER),
                16 => Ok(Self::DEVICE_TYPE_GROUP_COM_UNIT),
                17 => Ok(Self::DEVICE_TYPE_GROUP_COM_UNIT_BOOT),
                20 => Ok(Self::DEVICE_TYPE_GROUP_MAIN_BOARD),
                21 => Ok(Self::DEVICE_TYPE_GROUP_MMI_BOARD),
                22 => Ok(Self::DEVICE_TYPE_GROUP_CS_BOARD),
                23 => Ok(Self::DEVICE_TYPE_GROUP_US_BOARD),
                24 => Ok(Self::DEVICE_TYPE_GROUP_COM_BOARD),
                31 => Ok(Self::DEVICE_TYPE_GROUP_SW_MOWER),
                32 => Ok(Self::DEVICE_TYPE_GROUP_SW_MMI),
                33 => Ok(Self::DEVICE_TYPE_GROUP_SW_CS),
                34 => Ok(Self::DEVICE_TYPE_GROUP_SW_START),
                35 => Ok(Self::DEVICE_TYPE_GROUP_SW_ULTRASONIC),
                36 => Ok(Self::DEVICE_TYPE_GROUP_SW_COM),
                v => Err(Error::DoesNotCorespondToVariant(format!(
                    "Value {v} does not corespond to a variant in tDeviceTypeGroup"
                ))),
            }
        }
        fn to_u8(value: Self) -> u8 {
            match value {
                Self::DEVICE_TYPE_GROUP_UNDEFINED => 0,
                Self::DEVICE_TYPE_GROUP_GPS_BOARD => 1,
                Self::DEVICE_TYPE_GROUP_MOWER => 10,
                Self::DEVICE_TYPE_GROUP_MMI => 11,
                Self::DEVICE_TYPE_GROUP_CS => 12,
                Self::DEVICE_TYPE_GROUP_ULTRASONIC => 13,
                Self::DEVICE_TYPE_GROUP_MOWER_BOOT => 14,
                Self::DEVICE_TYPE_GROUP_MOWER_LOADER => 15,
                Self::DEVICE_TYPE_GROUP_COM_UNIT => 16,
                Self::DEVICE_TYPE_GROUP_COM_UNIT_BOOT => 17,
                Self::DEVICE_TYPE_GROUP_MAIN_BOARD => 20,
                Self::DEVICE_TYPE_GROUP_MMI_BOARD => 21,
                Self::DEVICE_TYPE_GROUP_CS_BOARD => 22,
                Self::DEVICE_TYPE_GROUP_US_BOARD => 23,
                Self::DEVICE_TYPE_GROUP_COM_BOARD => 24,
                Self::DEVICE_TYPE_GROUP_SW_MOWER => 31,
                Self::DEVICE_TYPE_GROUP_SW_MMI => 32,
                Self::DEVICE_TYPE_GROUP_SW_CS => 33,
                Self::DEVICE_TYPE_GROUP_SW_START => 34,
                Self::DEVICE_TYPE_GROUP_SW_ULTRASONIC => 35,
                Self::DEVICE_TYPE_GROUP_SW_COM => 36,
            }
        }
    }
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum tMowerDeviceType {
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
    impl HcpType for tMowerDeviceType {
        fn u8_to_variant(value: u8) -> Result<impl HcpType> {
            match value {
                0 => Ok(Self::MOWER_DEVICE_TYPE_UNDEFINED),
                1 => Ok(Self::MOWER_DEVICE_TYPE_B),
                2 => Ok(Self::MOWER_DEVICE_TYPE_C),
                3 => Ok(Self::MOWER_DEVICE_TYPE_D),
                4 => Ok(Self::MOWER_DEVICE_TYPE_E),
                5 => Ok(Self::MOWER_DEVICE_TYPE_F),
                6 => Ok(Self::MOWER_DEVICE_TYPE_G),
                7 => Ok(Self::MOWER_DEVICE_TYPE_H),
                8 => Ok(Self::MOWER_DEVICE_TYPE_I),
                9 => Ok(Self::MOWER_DEVICE_TYPE_J),
                10 => Ok(Self::MOWER_DEVICE_TYPE_K),
                11 => Ok(Self::MOWER_DEVICE_TYPE_L),
                12 => Ok(Self::MOWER_DEVICE_TYPE_M),
                13 => Ok(Self::MOWER_DEVICE_TYPE_N),
                14 => Ok(Self::MOWER_DEVICE_TYPE_O),
                15 => Ok(Self::MOWER_DEVICE_TYPE_P),
                16 => Ok(Self::MOWER_DEVICE_TYPE_Q),
                17 => Ok(Self::MOWER_DEVICE_TYPE_NO_MORE),
                v => Err(Error::DoesNotCorespondToVariant(format!(
                    "Value {v} does not corespond to a variant in tMowerDeviceType"
                ))),
            }
        }
        fn to_u8(value: Self) -> u8 {
            match value {
                Self::MOWER_DEVICE_TYPE_UNDEFINED => 0,
                Self::MOWER_DEVICE_TYPE_B => 1,
                Self::MOWER_DEVICE_TYPE_C => 2,
                Self::MOWER_DEVICE_TYPE_D => 3,
                Self::MOWER_DEVICE_TYPE_E => 4,
                Self::MOWER_DEVICE_TYPE_F => 5,
                Self::MOWER_DEVICE_TYPE_G => 6,
                Self::MOWER_DEVICE_TYPE_H => 7,
                Self::MOWER_DEVICE_TYPE_I => 8,
                Self::MOWER_DEVICE_TYPE_J => 9,
                Self::MOWER_DEVICE_TYPE_K => 10,
                Self::MOWER_DEVICE_TYPE_L => 11,
                Self::MOWER_DEVICE_TYPE_M => 12,
                Self::MOWER_DEVICE_TYPE_N => 13,
                Self::MOWER_DEVICE_TYPE_O => 14,
                Self::MOWER_DEVICE_TYPE_P => 15,
                Self::MOWER_DEVICE_TYPE_Q => 16,
                Self::MOWER_DEVICE_TYPE_NO_MORE => 17,
            }
        }
    }
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum tMowerVariantType {
        MOWER_VARIANT_TYPE_UNDEFINED, //
        MOWER_VARIANT_TYPE_ORG,       //
        MOWER_VARIANT_TYPE_B,         //
        MOWER_VARIANT_TYPE_C,         //
        MOWER_VARIANT_TYPE_D,         //
        MOWER_VARIANT_TYPE_E,         //
        MOWER_VARIANT_TYPE_F,         //
        MOWER_VARIANT_TYPE_NO_MORE,   //
    }
    impl HcpType for tMowerVariantType {
        fn u8_to_variant(value: u8) -> Result<impl HcpType> {
            match value {
                255 => Ok(Self::MOWER_VARIANT_TYPE_UNDEFINED),
                0 => Ok(Self::MOWER_VARIANT_TYPE_ORG),
                1 => Ok(Self::MOWER_VARIANT_TYPE_B),
                2 => Ok(Self::MOWER_VARIANT_TYPE_C),
                3 => Ok(Self::MOWER_VARIANT_TYPE_D),
                4 => Ok(Self::MOWER_VARIANT_TYPE_E),
                5 => Ok(Self::MOWER_VARIANT_TYPE_F),
                6 => Ok(Self::MOWER_VARIANT_TYPE_NO_MORE),
                v => Err(Error::DoesNotCorespondToVariant(format!(
                    "Value {v} does not corespond to a variant in tMowerVariantType"
                ))),
            }
        }
        fn to_u8(value: Self) -> u8 {
            match value {
                Self::MOWER_VARIANT_TYPE_UNDEFINED => 255,
                Self::MOWER_VARIANT_TYPE_ORG => 0,
                Self::MOWER_VARIANT_TYPE_B => 1,
                Self::MOWER_VARIANT_TYPE_C => 2,
                Self::MOWER_VARIANT_TYPE_D => 3,
                Self::MOWER_VARIANT_TYPE_E => 4,
                Self::MOWER_VARIANT_TYPE_F => 5,
                Self::MOWER_VARIANT_TYPE_NO_MORE => 6,
            }
        }
    }
    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum tSoundType {
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
    impl HcpType for tSoundType {
        fn u8_to_variant(value: u8) -> Result<impl HcpType> {
            match value {
                0 => Ok(Self::SOUND_KEY_CLICK),
                1 => Ok(Self::SOUND_CLICK),
                2 => Ok(Self::SOUND_CHARGING_CONNECTED),
                3 => Ok(Self::SOUND_CHARGING_DISCONNECTED),
                4 => Ok(Self::SOUND_DOUBLE_BEEP),
                5 => Ok(Self::SOUND_LONG_BEEP),
                6 => Ok(Self::SOUND_FAULT),
                7 => Ok(Self::SOUND_START_CUTTING),
                8 => Ok(Self::SOUND_ALARM_WARNING),
                9 => Ok(Self::SOUND_ALARM_1),
                10 => Ok(Self::SOUND_ALARM_2),
                11 => Ok(Self::SOUND_ALARM_5),
                12 => Ok(Self::SOUND_ALARM_10),
                13 => Ok(Self::SOUND_TONE_1),
                14 => Ok(Self::SOUND_OFF),
                v => Err(Error::DoesNotCorespondToVariant(format!(
                    "Value {v} does not corespond to a variant in tSoundType"
                ))),
            }
        }
        fn to_u8(value: Self) -> u8 {
            match value {
                Self::SOUND_KEY_CLICK => 0,
                Self::SOUND_CLICK => 1,
                Self::SOUND_CHARGING_CONNECTED => 2,
                Self::SOUND_CHARGING_DISCONNECTED => 3,
                Self::SOUND_DOUBLE_BEEP => 4,
                Self::SOUND_LONG_BEEP => 5,
                Self::SOUND_FAULT => 6,
                Self::SOUND_START_CUTTING => 7,
                Self::SOUND_ALARM_WARNING => 8,
                Self::SOUND_ALARM_1 => 9,
                Self::SOUND_ALARM_2 => 10,
                Self::SOUND_ALARM_5 => 11,
                Self::SOUND_ALARM_10 => 12,
                Self::SOUND_TONE_1 => 13,
                Self::SOUND_OFF => 14,
            }
        }
    }
}
pub mod Commands {
    pub mod DeviceInformation {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetDeviceIdentification {
            inParams {},
            outParams {
                deviceTypeGroup: Types::tDeviceTypeGroup,
                mowerDeviceType: Types::tMowerDeviceType,
                mowerSerialNo: u32,
                mowerVariantType: Types::tMowerVariantType,
            },
        }
        impl Hcp for GetDeviceIdentification {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(22, 0)
            }
        }
    }
    pub mod RealTimeData {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetWheelMotorData {
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
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(20, 2)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetBatteryData {
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
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(20, 1)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetGPSData {
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
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(20, 7)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetComboardSensorData {
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
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(20, 140)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetSensorData {
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
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(20, 4)
            }
        }
    }
    pub mod SystemSettings {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum SetHeadlightEnabled {
            inParams { headlight: u8 },
            outParams { headlight: u8 },
        }
        impl Hcp for SetHeadlightEnabled {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(2, 0x94)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetLoopDetection {
            inParams {},
            outParams { loopDetection: u8 },
        }
        impl Hcp for GetLoopDetection {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(2, 0x08)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum SetLoopDetection {
            inParams { loopDetection: u8 },
            outParams { loopDetection: u8 },
        }
        impl Hcp for SetLoopDetection {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(2, 0x88)
            }
        }
    }
    pub mod Wheels {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetSpeed {
            inParams { index: u8 },
            outParams { speed: i16 },
        }
        impl Hcp for GetSpeed {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4336, 6)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetRotationCounter {
            inParams { index: u8 },
            outParams { counter: i32 },
        }
        impl Hcp for GetRotationCounter {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4336, 5)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum PowerOff {
            inParams {},
            outParams {},
        }
        impl Hcp for PowerOff {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4336, 8)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum PowerOn {
            inParams {},
            outParams {},
        }
        impl Hcp for PowerOn {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4336, 9)
            }
        }
    }
    pub mod Collision {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetStatus {
            inParams {},
            outParams {
                collisionFrontCenter: bool,
                collisionRearRight: bool,
                collisionRearLeft: bool,
            },
        }
        impl Hcp for GetStatus {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4166, 2)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum SetSimulation {
            inParams { onOff: bool },
            outParams { onOff: bool },
        }
        impl Hcp for SetSimulation {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4166, 5)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetSimulation {
            inParams {},
            outParams { onOff: bool },
        }
        impl Hcp for GetSimulation {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4166, 6)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum SetSimulatedStatus {
            inParams { status: u32 },
            outParams { status: u32 },
        }
        impl Hcp for SetSimulatedStatus {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4166, 7)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetSimulatedStatus {
            inParams {},
            outParams { status: u32 },
        }
        impl Hcp for GetSimulatedStatus {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4166, 8)
            }
        }
    }
    pub mod Charger {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum IsChargingEnabled {
            inParams {},
            outParams { isChargingEnabled: bool },
        }
        impl Hcp for IsChargingEnabled {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4486, 3)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum IsChargingPowerConnected {
            inParams {},
            outParams { isChargingPowerConnected: bool },
        }
        impl Hcp for IsChargingPowerConnected {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4486, 4)
            }
        }
    }
    pub mod LiftSensor {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum IsActivated {
            inParams {},
            outParams { isActivated: bool },
        }
        impl Hcp for IsActivated {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4476, 0)
            }
        }
    }
    pub mod CurrentStatus {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetStatusKeepAlive {
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
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(18, 0x80)
            }
        }
    }
    pub mod LoopSampler {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetLoopSignalMaster {
            inParams {
                selectedloop: Types::tILoopSamplerLoops,
            },
            outParams {
                signalLevel: i16,
            },
        }
        impl Hcp for GetLoopSignalMaster {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4480, 3)
            }
        }
    }
    pub mod StopButton {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum IsActivated {
            inParams {},
            outParams { isActivated: bool },
        }
        impl Hcp for IsActivated {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4464, 4)
            }
        }
    }
    pub mod HardwareControl {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum WheelMotorsPower {
            inParams {
                leftWheelMotorPower: i16,
                rightWheelMotorPower: i16,
            },
            outParams {},
        }
        impl Hcp for WheelMotorsPower {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(16, 2)
            }
        }
    }
    pub mod MowerApp {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum SetMode {
            inParams {
                modeOfOperation: Types::tIMowerApp_MowerMode,
            },
            outParams {},
        }
        impl Hcp for SetMode {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4586, 0)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetMode {
            inParams {},
            outParams {
                modeOfOperation: Types::tIMowerApp_MowerMode,
            },
        }
        impl Hcp for GetMode {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4586, 1)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetState {
            inParams {},
            outParams { mowerState: Types::tIMowerApp_State },
        }
        impl Hcp for GetState {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4586, 2)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum StartTrigger {
            inParams {},
            outParams {},
        }
        impl Hcp for StartTrigger {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4586, 4)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum Pause {
            inParams {},
            outParams {},
        }
        impl Hcp for Pause {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4586, 5)
            }
        }
    }
    pub mod BladeMotor {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum Brake {
            inParams {},
            outParams {},
        }
        impl Hcp for Brake {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4362, 0)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum Run {
            inParams {},
            outParams {},
        }
        impl Hcp for Run {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4362, 1)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum On {
            inParams {},
            outParams {},
        }
        impl Hcp for On {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4362, 9)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum Off {
            inParams {},
            outParams {},
        }
        impl Hcp for Off {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4362, 10)
            }
        }
    }
    pub mod HeightMotor {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum SetHeight {
            inParams { height: u8 },
            outParams { retVal: Types::tReturn },
        }
        impl Hcp for SetHeight {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4488, 8)
            }
        }
    }
    pub mod Sound {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum SetSoundType {
            inParams { soundType: Types::tSoundType },
            outParams { soundType: Types::tSoundType },
        }
        impl Hcp for SetSoundType {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4268, 0)
            }
        }
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetSoundType {
            inParams {},
            outParams { soundType: Types::tSoundType },
        }
        impl Hcp for GetSoundType {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4268, 1)
            }
        }
    }
    pub mod SafetySupervisor {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum GetStatus {
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
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4466, 0)
            }
        }
    }
    pub mod Planner {
        use super::super::{Deserialize, Hcp, Msgtype, Serialize, Types};
        #[derive(Clone, Copy, Serialize, Deserialize)]
        pub enum ClearOverride {
            inParams {},
            outParams {},
        }
        impl Hcp for ClearOverride {
            fn get_msgtype_subcmd() -> Msgtype {
                Msgtype::new(4658, 6)
            }
        }
    }
}
