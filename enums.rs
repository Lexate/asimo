enum tILoopSamplerLoops {
    LoopsamplerLoopA = 0,         /* A */
    LoopsamplerLoopF = 1,         /* F */
    LoopsamplerLoopN = 2,         /* N */
    LoopsamplerLoopG1 = 3,        /* Guide 1 */
    LoopsamplerLoopG2 = 4,        /* Guide 2 */
    LoopsamplerLoopG3 = 5,        /* Guide 3 */
    LoopsamplerNumberOfLoops = 6, /*  */
}
enum tIMowerApp_MowerMode {
    ImowerappModeAuto = 0,   /* Auto */
    ImowerappModeManual = 1, /* Manual */
    ImowerappModeHome = 2,   /* Home */
    ImowerappModeDemo = 3,   /* Demo */
}
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
