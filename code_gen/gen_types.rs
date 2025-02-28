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
    enum tIMowerApp_MowerMode {
        IMOWERAPP_MODE_AUTO,   //Auto
        IMOWERAPP_MODE_MANUAL, //Manual
        IMOWERAPP_MODE_HOME,   //Home
        IMOWERAPP_MODE_DEMO,   //Demo
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
    }
    mod SystemSettings {
        enum SetHeadlightEnabled {
            inParams { headlight: u8 },
            outParams { headlight: u8 },
        }
        enum GetLoopDetection {
            inParams {},
            outParams { loopDetection: u8 },
        }
        enum SetLoopDetection {
            inParams { loopDetection: u8 },
            outParams { loopDetection: u8 },
        }
    }
    mod Wheels {
        enum GetSpeed {
            inParams { index: u8 },
            outParams { speed: i16 },
        }
        enum GetRotationCounter {
            inParams { index: u8 },
            outParams { counter: i32 },
        }
        enum PowerOff {
            inParams {},
            outParams {},
        }
        enum PowerOn {
            inParams {},
            outParams {},
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
        enum SetSimulation {
            inParams { onOff: bool },
            outParams { onOff: bool },
        }
        enum GetSimulation {
            inParams {},
            outParams { onOff: bool },
        }
        enum SetSimulatedStatus {
            inParams { status: u32 },
            outParams { status: u32 },
        }
        enum GetSimulatedStatus {
            inParams {},
            outParams { status: u32 },
        }
    }
    mod Charger {
        enum IsChargingEnabled {
            inParams {},
            outParams { isChargingEnabled: bool },
        }
        enum IsChargingPowerConnected {
            inParams {},
            outParams { isChargingPowerConnected: bool },
        }
    }
    mod LiftSensor {
        enum IsActivated {
            inParams {},
            outParams { isActivated: bool },
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
    }
    mod LoopSampler {
        enum GetLoopSignalMaster {
            inParams { selectedloop: tILoopSamplerLoops },
            outParams { signalLevel: i16 },
        }
    }
    mod StopButton {
        enum IsActivated {
            inParams {},
            outParams { isActivated: bool },
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
    }
    mod MowerApp {
        enum SetMode {
            inParams {
                modeOfOperation: tIMowerApp_MowerMode,
            },
            outParams {},
        }
        enum GetMode {
            inParams {},
            outParams {
                modeOfOperation: tIMowerApp_MowerMode,
            },
        }
        enum GetState {
            inParams {},
            outParams { mowerState: tIMowerApp_State },
        }
        enum StartTrigger {
            inParams {},
            outParams {},
        }
        enum Pause {
            inParams {},
            outParams {},
        }
    }
    mod BladeMotor {
        enum Brake {
            inParams {},
            outParams {},
        }
        enum Run {
            inParams {},
            outParams {},
        }
        enum On {
            inParams {},
            outParams {},
        }
        enum Off {
            inParams {},
            outParams {},
        }
    }
    mod HeightMotor {
        enum SetHeight {
            inParams { height: u8 },
            outParams { retVal: tReturn },
        }
    }
    mod Sound {
        enum SetSoundType {
            inParams { soundType: tSoundType },
            outParams { soundType: tSoundType },
        }
        enum GetSoundType {
            inParams {},
            outParams { soundType: tSoundType },
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
    }
    mod Planner {
        enum ClearOverride {
            inParams {},
            outParams {},
        }
    }
}
