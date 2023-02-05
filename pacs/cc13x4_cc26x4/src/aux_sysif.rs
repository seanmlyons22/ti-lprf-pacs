#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. - a specific system response to an active AUX wakeup flag. The response is dependent on what operational mode is requested. uLDO power supply state offers limited current supply. AUX_SCE cannot use certain peripherals and functions such as AUX_DDI0_OSC, AUX_TDC and AUX_ANAIF ADC interface in this power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided."]
    pub opmodereq: OPMODEREQ,
    #[doc = "0x04 - Operational Mode Acknowledgement AUX_SCE program must assume that the current operational mode is the one acknowledged."]
    pub opmodeack: OPMODEACK,
    #[doc = "0x08 - Programmable Wakeup 0 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU0 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    pub progwu0cfg: PROGWU0CFG,
    #[doc = "0x0c - Programmable Wakeup 1 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU1 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    pub progwu1cfg: PROGWU1CFG,
    #[doc = "0x10 - Programmable Wakeup 2 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU2 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    pub progwu2cfg: PROGWU2CFG,
    #[doc = "0x14 - Programmable Wakeup 3 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU3 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    pub progwu3cfg: PROGWU3CFG,
    #[doc = "0x18 - Software Wakeup Triggers System CPU uses these wakeup flags to perform handshaking with AUX_SCE. The wakeup flags can change the operational mode of AUX and guarantees a non-zero SCE clock rate. AUX_SCE wakeup vectors are configured in VECCFGn."]
    pub swwutrig: SWWUTRIG,
    #[doc = "0x1c - Wakeup Flags This register holds the eight AUX wakeup flags. Each flag can cause AUX operational mode to change as given in OPMODEREQ. To clear flag n you must set bit n in WUFLAGSCLR until flag n is read as 0. You must clear bit n in WUFLAGSCLR before flag n can be set again."]
    pub wuflags: WUFLAGS,
    #[doc = "0x20 - Wakeup Flags Clear This register clears AUX wakeup flags WUFLAGS. To clear programmable wakeup flags you must disable the AUX wakeup output first. After the programmable wakeup flags are cleared you must re-enable the AUX wakeup output. Write WUGATE to disable or enable the AUX wakeup output. This procedure is not required when you want to clear a software-triggered wakeup."]
    pub wuflagsclr: WUFLAGSCLR,
    #[doc = "0x24 - Wakeup Gate You must disable the AUX wakeup output: - Before you clear a programmable wakeup flag. - Before you change the value of \\[PROGWUnCFG.EN\\]
or \\[PROGWUnCFG.WU_SRC\\]. The AUX wakeup output must be re-enabled after clear operation or programmable wakeup configuration."]
    pub wugate: WUGATE,
    #[doc = "0x28 - Vector Configuration 0 AUX_SCE wakeup vector 0 configuration"]
    pub veccfg0: VECCFG0,
    #[doc = "0x2c - Vector Configuration 1 AUX_SCE wakeup vector 1 configuration"]
    pub veccfg1: VECCFG1,
    #[doc = "0x30 - Vector Configuration 2 AUX_SCE wakeup vector 2 configuration"]
    pub veccfg2: VECCFG2,
    #[doc = "0x34 - Vector Configuration 3 AUX_SCE wakeup vector 3 configuration"]
    pub veccfg3: VECCFG3,
    #[doc = "0x38 - Vector Configuration 4 AUX_SCE wakeup vector 4 configuration"]
    pub veccfg4: VECCFG4,
    #[doc = "0x3c - Vector Configuration 5 AUX_SCE wakeup vector 5 configuration"]
    pub veccfg5: VECCFG5,
    #[doc = "0x40 - Vector Configuration 6 AUX_SCE wakeup vector 6 configuration"]
    pub veccfg6: VECCFG6,
    #[doc = "0x44 - Vector Configuration 7 AUX_SCE wakeup vector 7 configuration"]
    pub veccfg7: VECCFG7,
    #[doc = "0x48 - Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. You must select SCE rate when AUX_SCE uses the event. You must select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active."]
    pub evsyncrate: EVSYNCRATE,
    #[doc = "0x4c - Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. You must select SCE rate when AUX_SCE uses such peripheral or an event produced by it. You must select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active."]
    pub peroprate: PEROPRATE,
    #[doc = "0x50 - ADC Clock Control"]
    pub adcclkctl: ADCCLKCTL,
    #[doc = "0x54 - TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. These are the recommended steps to configure and request the counter clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - If DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL=RCOSC_HF (24 or 48 MHz), wait until ACK=1. - If DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL=XOSC_HF, wait until ACK=1 and DDI_0_OSC:STAT2.XOSC_HF_FREQGOOD=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the counter clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0."]
    pub tdcclkctl: TDCCLKCTL,
    #[doc = "0x58 - TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. These are the recommended steps to configure and request the reference clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - Wait until ACK=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the reference clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0. It is not recommended to enable the TDC reference clock if DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL=RCOSCHFDLF (0x0)."]
    pub tdcrefclkctl: TDCREFCLKCTL,
    #[doc = "0x5c - AUX_TIMER2 Clock Control Access to AUX_TIMER2 is only possible when TIMER2CLKSTAT.STAT is different from NONE."]
    pub timer2clkctl: TIMER2CLKCTL,
    #[doc = "0x60 - AUX_TIMER2 Clock Status"]
    pub timer2clkstat: TIMER2CLKSTAT,
    #[doc = "0x64 - AUX_TIMER2 Clock Switch"]
    pub timer2clkswitch: TIMER2CLKSWITCH,
    #[doc = "0x68 - AUX_TIMER2 Debug Control"]
    pub timer2dbgctl: TIMER2DBGCTL,
    _reserved27: [u8; 0x04],
    #[doc = "0x70 - Clock Shift Detection A transition in the MCU domain state causes a non-accumulative change to the SCE clock period when the AUX clock rate is derived from SCLK_MF or SCLK_LF: - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles longer when MCU domain enters active state. - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles shorter when MCU domain exits active state. AUX_SCE detects if such events occurred to the SCE clock during the time period between a clear of STAT and a read of STAT."]
    pub clkshiftdet: CLKSHIFTDET,
    #[doc = "0x74 - VDDR Recharge Trigger"]
    pub rechargetrig: RECHARGETRIG,
    #[doc = "0x78 - VDDR Recharge Detection Some applications can be sensitive to power noise caused by recharge of VDDR. You can detect if VDDR recharge occurs."]
    pub rechargedet: RECHARGEDET,
    #[doc = "0x7c - Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set. AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE."]
    pub rtcsubsecinc0: RTCSUBSECINC0,
    #[doc = "0x80 - Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set. AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE."]
    pub rtcsubsecinc1: RTCSUBSECINC1,
    #[doc = "0x84 - Real Time Counter Sub Second Increment Control AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE."]
    pub rtcsubsecincctl: RTCSUBSECINCCTL,
    #[doc = "0x88 - Real Time Counter Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SEC.VALUE directly."]
    pub rtcsec: RTCSEC,
    #[doc = "0x8c - Real Time Counter Sub-Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SUBSEC.VALUE directly."]
    pub rtcsubsec: RTCSUBSEC,
    #[doc = "0x90 - AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY."]
    pub rtcevclr: RTCEVCLR,
    #[doc = "0x94 - AON_BATMON Battery Voltage Value Read access to AON_BATMON:BAT. System CPU must not access this register. Instead, system CPU must access AON_BATMON:BAT directly. AON_BATMON:BAT updates during VDDR recharge or active operational mode."]
    pub batmonbat: BATMONBAT,
    _reserved37: [u8; 0x04],
    #[doc = "0x9c - AON_BATMON Temperature Value Read access to AON_BATMON:TEMP. System CPU must not access this register. Instead, system CPU must access AON_BATMON:TEMP directly. AON_BATMON:TEMP updates during VDDR recharge or active operational mode."]
    pub batmontemp: BATMONTEMP,
    #[doc = "0xa0 - Timer Halt Debug register"]
    pub timerhalt: TIMERHALT,
    _reserved39: [u8; 0x0c],
    #[doc = "0xb0 - AUX_TIMER2 Bridge"]
    pub timer2bridge: TIMER2BRIDGE,
    #[doc = "0xb4 - Software Power Profiler"]
    pub swpwrprof: SWPWRPROF,
}
#[doc = "OPMODEREQ (rw) register accessor: an alias for `Reg<OPMODEREQ_SPEC>`"]
pub type OPMODEREQ = crate::Reg<opmodereq::OPMODEREQ_SPEC>;
#[doc = "Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. - a specific system response to an active AUX wakeup flag. The response is dependent on what operational mode is requested. uLDO power supply state offers limited current supply. AUX_SCE cannot use certain peripherals and functions such as AUX_DDI0_OSC, AUX_TDC and AUX_ANAIF ADC interface in this power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided."]
pub mod opmodereq;
#[doc = "OPMODEACK (rw) register accessor: an alias for `Reg<OPMODEACK_SPEC>`"]
pub type OPMODEACK = crate::Reg<opmodeack::OPMODEACK_SPEC>;
#[doc = "Operational Mode Acknowledgement AUX_SCE program must assume that the current operational mode is the one acknowledged."]
pub mod opmodeack;
#[doc = "PROGWU0CFG (rw) register accessor: an alias for `Reg<PROGWU0CFG_SPEC>`"]
pub type PROGWU0CFG = crate::Reg<progwu0cfg::PROGWU0CFG_SPEC>;
#[doc = "Programmable Wakeup 0 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU0 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu0cfg;
#[doc = "PROGWU1CFG (rw) register accessor: an alias for `Reg<PROGWU1CFG_SPEC>`"]
pub type PROGWU1CFG = crate::Reg<progwu1cfg::PROGWU1CFG_SPEC>;
#[doc = "Programmable Wakeup 1 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU1 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu1cfg;
#[doc = "PROGWU2CFG (rw) register accessor: an alias for `Reg<PROGWU2CFG_SPEC>`"]
pub type PROGWU2CFG = crate::Reg<progwu2cfg::PROGWU2CFG_SPEC>;
#[doc = "Programmable Wakeup 2 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU2 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu2cfg;
#[doc = "PROGWU3CFG (rw) register accessor: an alias for `Reg<PROGWU3CFG_SPEC>`"]
pub type PROGWU3CFG = crate::Reg<progwu3cfg::PROGWU3CFG_SPEC>;
#[doc = "Programmable Wakeup 3 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU3 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu3cfg;
#[doc = "SWWUTRIG (rw) register accessor: an alias for `Reg<SWWUTRIG_SPEC>`"]
pub type SWWUTRIG = crate::Reg<swwutrig::SWWUTRIG_SPEC>;
#[doc = "Software Wakeup Triggers System CPU uses these wakeup flags to perform handshaking with AUX_SCE. The wakeup flags can change the operational mode of AUX and guarantees a non-zero SCE clock rate. AUX_SCE wakeup vectors are configured in VECCFGn."]
pub mod swwutrig;
#[doc = "WUFLAGS (rw) register accessor: an alias for `Reg<WUFLAGS_SPEC>`"]
pub type WUFLAGS = crate::Reg<wuflags::WUFLAGS_SPEC>;
#[doc = "Wakeup Flags This register holds the eight AUX wakeup flags. Each flag can cause AUX operational mode to change as given in OPMODEREQ. To clear flag n you must set bit n in WUFLAGSCLR until flag n is read as 0. You must clear bit n in WUFLAGSCLR before flag n can be set again."]
pub mod wuflags;
#[doc = "WUFLAGSCLR (rw) register accessor: an alias for `Reg<WUFLAGSCLR_SPEC>`"]
pub type WUFLAGSCLR = crate::Reg<wuflagsclr::WUFLAGSCLR_SPEC>;
#[doc = "Wakeup Flags Clear This register clears AUX wakeup flags WUFLAGS. To clear programmable wakeup flags you must disable the AUX wakeup output first. After the programmable wakeup flags are cleared you must re-enable the AUX wakeup output. Write WUGATE to disable or enable the AUX wakeup output. This procedure is not required when you want to clear a software-triggered wakeup."]
pub mod wuflagsclr;
#[doc = "WUGATE (rw) register accessor: an alias for `Reg<WUGATE_SPEC>`"]
pub type WUGATE = crate::Reg<wugate::WUGATE_SPEC>;
#[doc = "Wakeup Gate You must disable the AUX wakeup output: - Before you clear a programmable wakeup flag. - Before you change the value of \\[PROGWUnCFG.EN\\]
or \\[PROGWUnCFG.WU_SRC\\]. The AUX wakeup output must be re-enabled after clear operation or programmable wakeup configuration."]
pub mod wugate;
#[doc = "VECCFG0 (rw) register accessor: an alias for `Reg<VECCFG0_SPEC>`"]
pub type VECCFG0 = crate::Reg<veccfg0::VECCFG0_SPEC>;
#[doc = "Vector Configuration 0 AUX_SCE wakeup vector 0 configuration"]
pub mod veccfg0;
#[doc = "VECCFG1 (rw) register accessor: an alias for `Reg<VECCFG1_SPEC>`"]
pub type VECCFG1 = crate::Reg<veccfg1::VECCFG1_SPEC>;
#[doc = "Vector Configuration 1 AUX_SCE wakeup vector 1 configuration"]
pub mod veccfg1;
#[doc = "VECCFG2 (rw) register accessor: an alias for `Reg<VECCFG2_SPEC>`"]
pub type VECCFG2 = crate::Reg<veccfg2::VECCFG2_SPEC>;
#[doc = "Vector Configuration 2 AUX_SCE wakeup vector 2 configuration"]
pub mod veccfg2;
#[doc = "VECCFG3 (rw) register accessor: an alias for `Reg<VECCFG3_SPEC>`"]
pub type VECCFG3 = crate::Reg<veccfg3::VECCFG3_SPEC>;
#[doc = "Vector Configuration 3 AUX_SCE wakeup vector 3 configuration"]
pub mod veccfg3;
#[doc = "VECCFG4 (rw) register accessor: an alias for `Reg<VECCFG4_SPEC>`"]
pub type VECCFG4 = crate::Reg<veccfg4::VECCFG4_SPEC>;
#[doc = "Vector Configuration 4 AUX_SCE wakeup vector 4 configuration"]
pub mod veccfg4;
#[doc = "VECCFG5 (rw) register accessor: an alias for `Reg<VECCFG5_SPEC>`"]
pub type VECCFG5 = crate::Reg<veccfg5::VECCFG5_SPEC>;
#[doc = "Vector Configuration 5 AUX_SCE wakeup vector 5 configuration"]
pub mod veccfg5;
#[doc = "VECCFG6 (rw) register accessor: an alias for `Reg<VECCFG6_SPEC>`"]
pub type VECCFG6 = crate::Reg<veccfg6::VECCFG6_SPEC>;
#[doc = "Vector Configuration 6 AUX_SCE wakeup vector 6 configuration"]
pub mod veccfg6;
#[doc = "VECCFG7 (rw) register accessor: an alias for `Reg<VECCFG7_SPEC>`"]
pub type VECCFG7 = crate::Reg<veccfg7::VECCFG7_SPEC>;
#[doc = "Vector Configuration 7 AUX_SCE wakeup vector 7 configuration"]
pub mod veccfg7;
#[doc = "EVSYNCRATE (rw) register accessor: an alias for `Reg<EVSYNCRATE_SPEC>`"]
pub type EVSYNCRATE = crate::Reg<evsyncrate::EVSYNCRATE_SPEC>;
#[doc = "Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. You must select SCE rate when AUX_SCE uses the event. You must select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active."]
pub mod evsyncrate;
#[doc = "PEROPRATE (rw) register accessor: an alias for `Reg<PEROPRATE_SPEC>`"]
pub type PEROPRATE = crate::Reg<peroprate::PEROPRATE_SPEC>;
#[doc = "Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. You must select SCE rate when AUX_SCE uses such peripheral or an event produced by it. You must select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active."]
pub mod peroprate;
#[doc = "ADCCLKCTL (rw) register accessor: an alias for `Reg<ADCCLKCTL_SPEC>`"]
pub type ADCCLKCTL = crate::Reg<adcclkctl::ADCCLKCTL_SPEC>;
#[doc = "ADC Clock Control"]
pub mod adcclkctl;
#[doc = "TDCCLKCTL (rw) register accessor: an alias for `Reg<TDCCLKCTL_SPEC>`"]
pub type TDCCLKCTL = crate::Reg<tdcclkctl::TDCCLKCTL_SPEC>;
#[doc = "TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. These are the recommended steps to configure and request the counter clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - If DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL=RCOSC_HF (24 or 48 MHz), wait until ACK=1. - If DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL=XOSC_HF, wait until ACK=1 and DDI_0_OSC:STAT2.XOSC_HF_FREQGOOD=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the counter clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0."]
pub mod tdcclkctl;
#[doc = "TDCREFCLKCTL (rw) register accessor: an alias for `Reg<TDCREFCLKCTL_SPEC>`"]
pub type TDCREFCLKCTL = crate::Reg<tdcrefclkctl::TDCREFCLKCTL_SPEC>;
#[doc = "TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. These are the recommended steps to configure and request the reference clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - Wait until ACK=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the reference clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0. It is not recommended to enable the TDC reference clock if DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL=RCOSCHFDLF (0x0)."]
pub mod tdcrefclkctl;
#[doc = "TIMER2CLKCTL (rw) register accessor: an alias for `Reg<TIMER2CLKCTL_SPEC>`"]
pub type TIMER2CLKCTL = crate::Reg<timer2clkctl::TIMER2CLKCTL_SPEC>;
#[doc = "AUX_TIMER2 Clock Control Access to AUX_TIMER2 is only possible when TIMER2CLKSTAT.STAT is different from NONE."]
pub mod timer2clkctl;
#[doc = "TIMER2CLKSTAT (rw) register accessor: an alias for `Reg<TIMER2CLKSTAT_SPEC>`"]
pub type TIMER2CLKSTAT = crate::Reg<timer2clkstat::TIMER2CLKSTAT_SPEC>;
#[doc = "AUX_TIMER2 Clock Status"]
pub mod timer2clkstat;
#[doc = "TIMER2CLKSWITCH (rw) register accessor: an alias for `Reg<TIMER2CLKSWITCH_SPEC>`"]
pub type TIMER2CLKSWITCH = crate::Reg<timer2clkswitch::TIMER2CLKSWITCH_SPEC>;
#[doc = "AUX_TIMER2 Clock Switch"]
pub mod timer2clkswitch;
#[doc = "TIMER2DBGCTL (rw) register accessor: an alias for `Reg<TIMER2DBGCTL_SPEC>`"]
pub type TIMER2DBGCTL = crate::Reg<timer2dbgctl::TIMER2DBGCTL_SPEC>;
#[doc = "AUX_TIMER2 Debug Control"]
pub mod timer2dbgctl;
#[doc = "CLKSHIFTDET (rw) register accessor: an alias for `Reg<CLKSHIFTDET_SPEC>`"]
pub type CLKSHIFTDET = crate::Reg<clkshiftdet::CLKSHIFTDET_SPEC>;
#[doc = "Clock Shift Detection A transition in the MCU domain state causes a non-accumulative change to the SCE clock period when the AUX clock rate is derived from SCLK_MF or SCLK_LF: - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles longer when MCU domain enters active state. - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles shorter when MCU domain exits active state. AUX_SCE detects if such events occurred to the SCE clock during the time period between a clear of STAT and a read of STAT."]
pub mod clkshiftdet;
#[doc = "RECHARGETRIG (rw) register accessor: an alias for `Reg<RECHARGETRIG_SPEC>`"]
pub type RECHARGETRIG = crate::Reg<rechargetrig::RECHARGETRIG_SPEC>;
#[doc = "VDDR Recharge Trigger"]
pub mod rechargetrig;
#[doc = "RECHARGEDET (rw) register accessor: an alias for `Reg<RECHARGEDET_SPEC>`"]
pub type RECHARGEDET = crate::Reg<rechargedet::RECHARGEDET_SPEC>;
#[doc = "VDDR Recharge Detection Some applications can be sensitive to power noise caused by recharge of VDDR. You can detect if VDDR recharge occurs."]
pub mod rechargedet;
#[doc = "RTCSUBSECINC0 (rw) register accessor: an alias for `Reg<RTCSUBSECINC0_SPEC>`"]
pub type RTCSUBSECINC0 = crate::Reg<rtcsubsecinc0::RTCSUBSECINC0_SPEC>;
#[doc = "Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set. AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE."]
pub mod rtcsubsecinc0;
#[doc = "RTCSUBSECINC1 (rw) register accessor: an alias for `Reg<RTCSUBSECINC1_SPEC>`"]
pub type RTCSUBSECINC1 = crate::Reg<rtcsubsecinc1::RTCSUBSECINC1_SPEC>;
#[doc = "Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set. AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE."]
pub mod rtcsubsecinc1;
#[doc = "RTCSUBSECINCCTL (rw) register accessor: an alias for `Reg<RTCSUBSECINCCTL_SPEC>`"]
pub type RTCSUBSECINCCTL = crate::Reg<rtcsubsecincctl::RTCSUBSECINCCTL_SPEC>;
#[doc = "Real Time Counter Sub Second Increment Control AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE."]
pub mod rtcsubsecincctl;
#[doc = "RTCSEC (rw) register accessor: an alias for `Reg<RTCSEC_SPEC>`"]
pub type RTCSEC = crate::Reg<rtcsec::RTCSEC_SPEC>;
#[doc = "Real Time Counter Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SEC.VALUE directly."]
pub mod rtcsec;
#[doc = "RTCSUBSEC (rw) register accessor: an alias for `Reg<RTCSUBSEC_SPEC>`"]
pub type RTCSUBSEC = crate::Reg<rtcsubsec::RTCSUBSEC_SPEC>;
#[doc = "Real Time Counter Sub-Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SUBSEC.VALUE directly."]
pub mod rtcsubsec;
#[doc = "RTCEVCLR (rw) register accessor: an alias for `Reg<RTCEVCLR_SPEC>`"]
pub type RTCEVCLR = crate::Reg<rtcevclr::RTCEVCLR_SPEC>;
#[doc = "AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY."]
pub mod rtcevclr;
#[doc = "BATMONBAT (rw) register accessor: an alias for `Reg<BATMONBAT_SPEC>`"]
pub type BATMONBAT = crate::Reg<batmonbat::BATMONBAT_SPEC>;
#[doc = "AON_BATMON Battery Voltage Value Read access to AON_BATMON:BAT. System CPU must not access this register. Instead, system CPU must access AON_BATMON:BAT directly. AON_BATMON:BAT updates during VDDR recharge or active operational mode."]
pub mod batmonbat;
#[doc = "BATMONTEMP (rw) register accessor: an alias for `Reg<BATMONTEMP_SPEC>`"]
pub type BATMONTEMP = crate::Reg<batmontemp::BATMONTEMP_SPEC>;
#[doc = "AON_BATMON Temperature Value Read access to AON_BATMON:TEMP. System CPU must not access this register. Instead, system CPU must access AON_BATMON:TEMP directly. AON_BATMON:TEMP updates during VDDR recharge or active operational mode."]
pub mod batmontemp;
#[doc = "TIMERHALT (rw) register accessor: an alias for `Reg<TIMERHALT_SPEC>`"]
pub type TIMERHALT = crate::Reg<timerhalt::TIMERHALT_SPEC>;
#[doc = "Timer Halt Debug register"]
pub mod timerhalt;
#[doc = "TIMER2BRIDGE (rw) register accessor: an alias for `Reg<TIMER2BRIDGE_SPEC>`"]
pub type TIMER2BRIDGE = crate::Reg<timer2bridge::TIMER2BRIDGE_SPEC>;
#[doc = "AUX_TIMER2 Bridge"]
pub mod timer2bridge;
#[doc = "SWPWRPROF (rw) register accessor: an alias for `Reg<SWPWRPROF_SPEC>`"]
pub type SWPWRPROF = crate::Reg<swpwrprof::SWPWRPROF_SPEC>;
#[doc = "Software Power Profiler"]
pub mod swpwrprof;
