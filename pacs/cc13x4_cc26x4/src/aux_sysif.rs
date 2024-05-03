#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    opmodereq: Opmodereq,
    opmodeack: Opmodeack,
    progwu0cfg: Progwu0cfg,
    progwu1cfg: Progwu1cfg,
    progwu2cfg: Progwu2cfg,
    progwu3cfg: Progwu3cfg,
    swwutrig: Swwutrig,
    wuflags: Wuflags,
    wuflagsclr: Wuflagsclr,
    wugate: Wugate,
    veccfg0: Veccfg0,
    veccfg1: Veccfg1,
    veccfg2: Veccfg2,
    veccfg3: Veccfg3,
    veccfg4: Veccfg4,
    veccfg5: Veccfg5,
    veccfg6: Veccfg6,
    veccfg7: Veccfg7,
    evsyncrate: Evsyncrate,
    peroprate: Peroprate,
    adcclkctl: Adcclkctl,
    tdcclkctl: Tdcclkctl,
    tdcrefclkctl: Tdcrefclkctl,
    timer2clkctl: Timer2clkctl,
    timer2clkstat: Timer2clkstat,
    timer2clkswitch: Timer2clkswitch,
    timer2dbgctl: Timer2dbgctl,
    _reserved27: [u8; 0x04],
    clkshiftdet: Clkshiftdet,
    rechargetrig: Rechargetrig,
    rechargedet: Rechargedet,
    rtcsubsecinc0: Rtcsubsecinc0,
    rtcsubsecinc1: Rtcsubsecinc1,
    rtcsubsecincctl: Rtcsubsecincctl,
    rtcsec: Rtcsec,
    rtcsubsec: Rtcsubsec,
    rtcevclr: Rtcevclr,
    batmonbat: Batmonbat,
    _reserved37: [u8; 0x04],
    batmontemp: Batmontemp,
    timerhalt: Timerhalt,
    _reserved39: [u8; 0x0c],
    timer2bridge: Timer2bridge,
    swpwrprof: Swpwrprof,
}
impl RegisterBlock {
    #[doc = "0x00 - Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. - a specific system response to an active AUX wakeup flag. The response is dependent on what operational mode is requested. uLDO power supply state offers limited current supply. AUX_SCE cannot use certain peripherals and functions such as AUX_DDI0_OSC, AUX_TDC and AUX_ANAIF ADC interface in this power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided."]
    #[inline(always)]
    pub const fn opmodereq(&self) -> &Opmodereq {
        &self.opmodereq
    }
    #[doc = "0x04 - Operational Mode Acknowledgement AUX_SCE program must assume that the current operational mode is the one acknowledged."]
    #[inline(always)]
    pub const fn opmodeack(&self) -> &Opmodeack {
        &self.opmodeack
    }
    #[doc = "0x08 - Programmable Wakeup 0 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU0 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    #[inline(always)]
    pub const fn progwu0cfg(&self) -> &Progwu0cfg {
        &self.progwu0cfg
    }
    #[doc = "0x0c - Programmable Wakeup 1 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU1 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    #[inline(always)]
    pub const fn progwu1cfg(&self) -> &Progwu1cfg {
        &self.progwu1cfg
    }
    #[doc = "0x10 - Programmable Wakeup 2 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU2 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    #[inline(always)]
    pub const fn progwu2cfg(&self) -> &Progwu2cfg {
        &self.progwu2cfg
    }
    #[doc = "0x14 - Programmable Wakeup 3 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU3 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
    #[inline(always)]
    pub const fn progwu3cfg(&self) -> &Progwu3cfg {
        &self.progwu3cfg
    }
    #[doc = "0x18 - Software Wakeup Triggers System CPU uses these wakeup flags to perform handshaking with AUX_SCE. The wakeup flags can change the operational mode of AUX and guarantees a non-zero SCE clock rate. AUX_SCE wakeup vectors are configured in VECCFGn."]
    #[inline(always)]
    pub const fn swwutrig(&self) -> &Swwutrig {
        &self.swwutrig
    }
    #[doc = "0x1c - Wakeup Flags This register holds the eight AUX wakeup flags. Each flag can cause AUX operational mode to change as given in OPMODEREQ. To clear flag n you must set bit n in WUFLAGSCLR until flag n is read as 0. You must clear bit n in WUFLAGSCLR before flag n can be set again."]
    #[inline(always)]
    pub const fn wuflags(&self) -> &Wuflags {
        &self.wuflags
    }
    #[doc = "0x20 - Wakeup Flags Clear This register clears AUX wakeup flags WUFLAGS. To clear programmable wakeup flags you must disable the AUX wakeup output first. After the programmable wakeup flags are cleared you must re-enable the AUX wakeup output. Write WUGATE to disable or enable the AUX wakeup output. This procedure is not required when you want to clear a software-triggered wakeup."]
    #[inline(always)]
    pub const fn wuflagsclr(&self) -> &Wuflagsclr {
        &self.wuflagsclr
    }
    #[doc = "0x24 - Wakeup Gate You must disable the AUX wakeup output: - Before you clear a programmable wakeup flag. - Before you change the value of \\[PROGWUnCFG.EN\\]
or \\[PROGWUnCFG.WU_SRC\\]. The AUX wakeup output must be re-enabled after clear operation or programmable wakeup configuration."]
    #[inline(always)]
    pub const fn wugate(&self) -> &Wugate {
        &self.wugate
    }
    #[doc = "0x28 - Vector Configuration 0 AUX_SCE wakeup vector 0 configuration"]
    #[inline(always)]
    pub const fn veccfg0(&self) -> &Veccfg0 {
        &self.veccfg0
    }
    #[doc = "0x2c - Vector Configuration 1 AUX_SCE wakeup vector 1 configuration"]
    #[inline(always)]
    pub const fn veccfg1(&self) -> &Veccfg1 {
        &self.veccfg1
    }
    #[doc = "0x30 - Vector Configuration 2 AUX_SCE wakeup vector 2 configuration"]
    #[inline(always)]
    pub const fn veccfg2(&self) -> &Veccfg2 {
        &self.veccfg2
    }
    #[doc = "0x34 - Vector Configuration 3 AUX_SCE wakeup vector 3 configuration"]
    #[inline(always)]
    pub const fn veccfg3(&self) -> &Veccfg3 {
        &self.veccfg3
    }
    #[doc = "0x38 - Vector Configuration 4 AUX_SCE wakeup vector 4 configuration"]
    #[inline(always)]
    pub const fn veccfg4(&self) -> &Veccfg4 {
        &self.veccfg4
    }
    #[doc = "0x3c - Vector Configuration 5 AUX_SCE wakeup vector 5 configuration"]
    #[inline(always)]
    pub const fn veccfg5(&self) -> &Veccfg5 {
        &self.veccfg5
    }
    #[doc = "0x40 - Vector Configuration 6 AUX_SCE wakeup vector 6 configuration"]
    #[inline(always)]
    pub const fn veccfg6(&self) -> &Veccfg6 {
        &self.veccfg6
    }
    #[doc = "0x44 - Vector Configuration 7 AUX_SCE wakeup vector 7 configuration"]
    #[inline(always)]
    pub const fn veccfg7(&self) -> &Veccfg7 {
        &self.veccfg7
    }
    #[doc = "0x48 - Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. You must select SCE rate when AUX_SCE uses the event. You must select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active."]
    #[inline(always)]
    pub const fn evsyncrate(&self) -> &Evsyncrate {
        &self.evsyncrate
    }
    #[doc = "0x4c - Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. You must select SCE rate when AUX_SCE uses such peripheral or an event produced by it. You must select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active."]
    #[inline(always)]
    pub const fn peroprate(&self) -> &Peroprate {
        &self.peroprate
    }
    #[doc = "0x50 - ADC Clock Control"]
    #[inline(always)]
    pub const fn adcclkctl(&self) -> &Adcclkctl {
        &self.adcclkctl
    }
    #[doc = "0x54 - TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. These are the recommended steps to configure and request the counter clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - If DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL=RCOSC_HF (24 or 48 MHz), wait until ACK=1. - If DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL=XOSC_HF, wait until ACK=1 and DDI_0_OSC:STAT2.XOSC_HF_FREQGOOD=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the counter clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0."]
    #[inline(always)]
    pub const fn tdcclkctl(&self) -> &Tdcclkctl {
        &self.tdcclkctl
    }
    #[doc = "0x58 - TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. These are the recommended steps to configure and request the reference clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - Wait until ACK=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the reference clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0. It is not recommended to enable the TDC reference clock if DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL=RCOSCHFDLF (0x0)."]
    #[inline(always)]
    pub const fn tdcrefclkctl(&self) -> &Tdcrefclkctl {
        &self.tdcrefclkctl
    }
    #[doc = "0x5c - AUX_TIMER2 Clock Control Access to AUX_TIMER2 is only possible when TIMER2CLKSTAT.STAT is different from NONE."]
    #[inline(always)]
    pub const fn timer2clkctl(&self) -> &Timer2clkctl {
        &self.timer2clkctl
    }
    #[doc = "0x60 - AUX_TIMER2 Clock Status"]
    #[inline(always)]
    pub const fn timer2clkstat(&self) -> &Timer2clkstat {
        &self.timer2clkstat
    }
    #[doc = "0x64 - AUX_TIMER2 Clock Switch"]
    #[inline(always)]
    pub const fn timer2clkswitch(&self) -> &Timer2clkswitch {
        &self.timer2clkswitch
    }
    #[doc = "0x68 - AUX_TIMER2 Debug Control"]
    #[inline(always)]
    pub const fn timer2dbgctl(&self) -> &Timer2dbgctl {
        &self.timer2dbgctl
    }
    #[doc = "0x70 - Clock Shift Detection A transition in the MCU domain state causes a non-accumulative change to the SCE clock period when the AUX clock rate is derived from SCLK_MF or SCLK_LF: - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles longer when MCU domain enters active state. - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles shorter when MCU domain exits active state. AUX_SCE detects if such events occurred to the SCE clock during the time period between a clear of STAT and a read of STAT."]
    #[inline(always)]
    pub const fn clkshiftdet(&self) -> &Clkshiftdet {
        &self.clkshiftdet
    }
    #[doc = "0x74 - VDDR Recharge Trigger"]
    #[inline(always)]
    pub const fn rechargetrig(&self) -> &Rechargetrig {
        &self.rechargetrig
    }
    #[doc = "0x78 - VDDR Recharge Detection Some applications can be sensitive to power noise caused by recharge of VDDR. You can detect if VDDR recharge occurs."]
    #[inline(always)]
    pub const fn rechargedet(&self) -> &Rechargedet {
        &self.rechargedet
    }
    #[doc = "0x7c - Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set. AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE."]
    #[inline(always)]
    pub const fn rtcsubsecinc0(&self) -> &Rtcsubsecinc0 {
        &self.rtcsubsecinc0
    }
    #[doc = "0x80 - Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set. AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE."]
    #[inline(always)]
    pub const fn rtcsubsecinc1(&self) -> &Rtcsubsecinc1 {
        &self.rtcsubsecinc1
    }
    #[doc = "0x84 - Real Time Counter Sub Second Increment Control AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE."]
    #[inline(always)]
    pub const fn rtcsubsecincctl(&self) -> &Rtcsubsecincctl {
        &self.rtcsubsecincctl
    }
    #[doc = "0x88 - Real Time Counter Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SEC.VALUE directly."]
    #[inline(always)]
    pub const fn rtcsec(&self) -> &Rtcsec {
        &self.rtcsec
    }
    #[doc = "0x8c - Real Time Counter Sub-Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SUBSEC.VALUE directly."]
    #[inline(always)]
    pub const fn rtcsubsec(&self) -> &Rtcsubsec {
        &self.rtcsubsec
    }
    #[doc = "0x90 - AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY."]
    #[inline(always)]
    pub const fn rtcevclr(&self) -> &Rtcevclr {
        &self.rtcevclr
    }
    #[doc = "0x94 - AON_BATMON Battery Voltage Value Read access to AON_BATMON:BAT. System CPU must not access this register. Instead, system CPU must access AON_BATMON:BAT directly. AON_BATMON:BAT updates during VDDR recharge or active operational mode."]
    #[inline(always)]
    pub const fn batmonbat(&self) -> &Batmonbat {
        &self.batmonbat
    }
    #[doc = "0x9c - AON_BATMON Temperature Value Read access to AON_BATMON:TEMP. System CPU must not access this register. Instead, system CPU must access AON_BATMON:TEMP directly. AON_BATMON:TEMP updates during VDDR recharge or active operational mode."]
    #[inline(always)]
    pub const fn batmontemp(&self) -> &Batmontemp {
        &self.batmontemp
    }
    #[doc = "0xa0 - Timer Halt Debug register"]
    #[inline(always)]
    pub const fn timerhalt(&self) -> &Timerhalt {
        &self.timerhalt
    }
    #[doc = "0xb0 - AUX_TIMER2 Bridge"]
    #[inline(always)]
    pub const fn timer2bridge(&self) -> &Timer2bridge {
        &self.timer2bridge
    }
    #[doc = "0xb4 - Software Power Profiler"]
    #[inline(always)]
    pub const fn swpwrprof(&self) -> &Swpwrprof {
        &self.swpwrprof
    }
}
#[doc = "OPMODEREQ (rw) register accessor: Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. - a specific system response to an active AUX wakeup flag. The response is dependent on what operational mode is requested. uLDO power supply state offers limited current supply. AUX_SCE cannot use certain peripherals and functions such as AUX_DDI0_OSC, AUX_TDC and AUX_ANAIF ADC interface in this power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opmodereq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opmodereq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opmodereq`]
module"]
#[doc(alias = "OPMODEREQ")]
pub type Opmodereq = crate::Reg<opmodereq::OpmodereqSpec>;
#[doc = "Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. - a specific system response to an active AUX wakeup flag. The response is dependent on what operational mode is requested. uLDO power supply state offers limited current supply. AUX_SCE cannot use certain peripherals and functions such as AUX_DDI0_OSC, AUX_TDC and AUX_ANAIF ADC interface in this power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided."]
pub mod opmodereq;
#[doc = "OPMODEACK (rw) register accessor: Operational Mode Acknowledgement AUX_SCE program must assume that the current operational mode is the one acknowledged.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opmodeack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opmodeack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opmodeack`]
module"]
#[doc(alias = "OPMODEACK")]
pub type Opmodeack = crate::Reg<opmodeack::OpmodeackSpec>;
#[doc = "Operational Mode Acknowledgement AUX_SCE program must assume that the current operational mode is the one acknowledged."]
pub mod opmodeack;
#[doc = "PROGWU0CFG (rw) register accessor: Programmable Wakeup 0 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU0 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`progwu0cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`progwu0cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@progwu0cfg`]
module"]
#[doc(alias = "PROGWU0CFG")]
pub type Progwu0cfg = crate::Reg<progwu0cfg::Progwu0cfgSpec>;
#[doc = "Programmable Wakeup 0 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU0 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu0cfg;
#[doc = "PROGWU1CFG (rw) register accessor: Programmable Wakeup 1 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU1 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`progwu1cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`progwu1cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@progwu1cfg`]
module"]
#[doc(alias = "PROGWU1CFG")]
pub type Progwu1cfg = crate::Reg<progwu1cfg::Progwu1cfgSpec>;
#[doc = "Programmable Wakeup 1 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU1 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu1cfg;
#[doc = "PROGWU2CFG (rw) register accessor: Programmable Wakeup 2 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU2 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`progwu2cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`progwu2cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@progwu2cfg`]
module"]
#[doc(alias = "PROGWU2CFG")]
pub type Progwu2cfg = crate::Reg<progwu2cfg::Progwu2cfgSpec>;
#[doc = "Programmable Wakeup 2 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU2 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu2cfg;
#[doc = "PROGWU3CFG (rw) register accessor: Programmable Wakeup 3 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU3 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`progwu3cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`progwu3cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@progwu3cfg`]
module"]
#[doc(alias = "PROGWU3CFG")]
pub type Progwu3cfg = crate::Reg<progwu3cfg::Progwu3cfgSpec>;
#[doc = "Programmable Wakeup 3 Configuration Configure this register to enable a customized AUX wakeup flag. The wakeup flag will be captured by AON_PMCTL which responds according to the current operational mode. You can select WUFLAGS.PROG_WU3 to trigger execution of a programmable AUX_SCE vector by configuration of VECCFGn. You need to follow the procedure described in WUFLAGSCLR to clear this flag. You need to follow the procedure described in WUGATE to configure it."]
pub mod progwu3cfg;
#[doc = "SWWUTRIG (rw) register accessor: Software Wakeup Triggers System CPU uses these wakeup flags to perform handshaking with AUX_SCE. The wakeup flags can change the operational mode of AUX and guarantees a non-zero SCE clock rate. AUX_SCE wakeup vectors are configured in VECCFGn.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swwutrig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swwutrig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swwutrig`]
module"]
#[doc(alias = "SWWUTRIG")]
pub type Swwutrig = crate::Reg<swwutrig::SwwutrigSpec>;
#[doc = "Software Wakeup Triggers System CPU uses these wakeup flags to perform handshaking with AUX_SCE. The wakeup flags can change the operational mode of AUX and guarantees a non-zero SCE clock rate. AUX_SCE wakeup vectors are configured in VECCFGn."]
pub mod swwutrig;
#[doc = "WUFLAGS (rw) register accessor: Wakeup Flags This register holds the eight AUX wakeup flags. Each flag can cause AUX operational mode to change as given in OPMODEREQ. To clear flag n you must set bit n in WUFLAGSCLR until flag n is read as 0. You must clear bit n in WUFLAGSCLR before flag n can be set again.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wuflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wuflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wuflags`]
module"]
#[doc(alias = "WUFLAGS")]
pub type Wuflags = crate::Reg<wuflags::WuflagsSpec>;
#[doc = "Wakeup Flags This register holds the eight AUX wakeup flags. Each flag can cause AUX operational mode to change as given in OPMODEREQ. To clear flag n you must set bit n in WUFLAGSCLR until flag n is read as 0. You must clear bit n in WUFLAGSCLR before flag n can be set again."]
pub mod wuflags;
#[doc = "WUFLAGSCLR (rw) register accessor: Wakeup Flags Clear This register clears AUX wakeup flags WUFLAGS. To clear programmable wakeup flags you must disable the AUX wakeup output first. After the programmable wakeup flags are cleared you must re-enable the AUX wakeup output. Write WUGATE to disable or enable the AUX wakeup output. This procedure is not required when you want to clear a software-triggered wakeup.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wuflagsclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wuflagsclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wuflagsclr`]
module"]
#[doc(alias = "WUFLAGSCLR")]
pub type Wuflagsclr = crate::Reg<wuflagsclr::WuflagsclrSpec>;
#[doc = "Wakeup Flags Clear This register clears AUX wakeup flags WUFLAGS. To clear programmable wakeup flags you must disable the AUX wakeup output first. After the programmable wakeup flags are cleared you must re-enable the AUX wakeup output. Write WUGATE to disable or enable the AUX wakeup output. This procedure is not required when you want to clear a software-triggered wakeup."]
pub mod wuflagsclr;
#[doc = "WUGATE (rw) register accessor: Wakeup Gate You must disable the AUX wakeup output: - Before you clear a programmable wakeup flag. - Before you change the value of \\[PROGWUnCFG.EN\\]
or \\[PROGWUnCFG.WU_SRC\\]. The AUX wakeup output must be re-enabled after clear operation or programmable wakeup configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wugate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wugate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wugate`]
module"]
#[doc(alias = "WUGATE")]
pub type Wugate = crate::Reg<wugate::WugateSpec>;
#[doc = "Wakeup Gate You must disable the AUX wakeup output: - Before you clear a programmable wakeup flag. - Before you change the value of \\[PROGWUnCFG.EN\\]
or \\[PROGWUnCFG.WU_SRC\\]. The AUX wakeup output must be re-enabled after clear operation or programmable wakeup configuration."]
pub mod wugate;
#[doc = "VECCFG0 (rw) register accessor: Vector Configuration 0 AUX_SCE wakeup vector 0 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`veccfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`veccfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@veccfg0`]
module"]
#[doc(alias = "VECCFG0")]
pub type Veccfg0 = crate::Reg<veccfg0::Veccfg0Spec>;
#[doc = "Vector Configuration 0 AUX_SCE wakeup vector 0 configuration"]
pub mod veccfg0;
#[doc = "VECCFG1 (rw) register accessor: Vector Configuration 1 AUX_SCE wakeup vector 1 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`veccfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`veccfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@veccfg1`]
module"]
#[doc(alias = "VECCFG1")]
pub type Veccfg1 = crate::Reg<veccfg1::Veccfg1Spec>;
#[doc = "Vector Configuration 1 AUX_SCE wakeup vector 1 configuration"]
pub mod veccfg1;
#[doc = "VECCFG2 (rw) register accessor: Vector Configuration 2 AUX_SCE wakeup vector 2 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`veccfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`veccfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@veccfg2`]
module"]
#[doc(alias = "VECCFG2")]
pub type Veccfg2 = crate::Reg<veccfg2::Veccfg2Spec>;
#[doc = "Vector Configuration 2 AUX_SCE wakeup vector 2 configuration"]
pub mod veccfg2;
#[doc = "VECCFG3 (rw) register accessor: Vector Configuration 3 AUX_SCE wakeup vector 3 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`veccfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`veccfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@veccfg3`]
module"]
#[doc(alias = "VECCFG3")]
pub type Veccfg3 = crate::Reg<veccfg3::Veccfg3Spec>;
#[doc = "Vector Configuration 3 AUX_SCE wakeup vector 3 configuration"]
pub mod veccfg3;
#[doc = "VECCFG4 (rw) register accessor: Vector Configuration 4 AUX_SCE wakeup vector 4 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`veccfg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`veccfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@veccfg4`]
module"]
#[doc(alias = "VECCFG4")]
pub type Veccfg4 = crate::Reg<veccfg4::Veccfg4Spec>;
#[doc = "Vector Configuration 4 AUX_SCE wakeup vector 4 configuration"]
pub mod veccfg4;
#[doc = "VECCFG5 (rw) register accessor: Vector Configuration 5 AUX_SCE wakeup vector 5 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`veccfg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`veccfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@veccfg5`]
module"]
#[doc(alias = "VECCFG5")]
pub type Veccfg5 = crate::Reg<veccfg5::Veccfg5Spec>;
#[doc = "Vector Configuration 5 AUX_SCE wakeup vector 5 configuration"]
pub mod veccfg5;
#[doc = "VECCFG6 (rw) register accessor: Vector Configuration 6 AUX_SCE wakeup vector 6 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`veccfg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`veccfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@veccfg6`]
module"]
#[doc(alias = "VECCFG6")]
pub type Veccfg6 = crate::Reg<veccfg6::Veccfg6Spec>;
#[doc = "Vector Configuration 6 AUX_SCE wakeup vector 6 configuration"]
pub mod veccfg6;
#[doc = "VECCFG7 (rw) register accessor: Vector Configuration 7 AUX_SCE wakeup vector 7 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`veccfg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`veccfg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@veccfg7`]
module"]
#[doc(alias = "VECCFG7")]
pub type Veccfg7 = crate::Reg<veccfg7::Veccfg7Spec>;
#[doc = "Vector Configuration 7 AUX_SCE wakeup vector 7 configuration"]
pub mod veccfg7;
#[doc = "EVSYNCRATE (rw) register accessor: Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. You must select SCE rate when AUX_SCE uses the event. You must select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evsyncrate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evsyncrate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evsyncrate`]
module"]
#[doc(alias = "EVSYNCRATE")]
pub type Evsyncrate = crate::Reg<evsyncrate::EvsyncrateSpec>;
#[doc = "Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. You must select SCE rate when AUX_SCE uses the event. You must select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active."]
pub mod evsyncrate;
#[doc = "PEROPRATE (rw) register accessor: Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. You must select SCE rate when AUX_SCE uses such peripheral or an event produced by it. You must select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peroprate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peroprate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peroprate`]
module"]
#[doc(alias = "PEROPRATE")]
pub type Peroprate = crate::Reg<peroprate::PeroprateSpec>;
#[doc = "Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. You must select SCE rate when AUX_SCE uses such peripheral or an event produced by it. You must select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active."]
pub mod peroprate;
#[doc = "ADCCLKCTL (rw) register accessor: ADC Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcclkctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcclkctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcclkctl`]
module"]
#[doc(alias = "ADCCLKCTL")]
pub type Adcclkctl = crate::Reg<adcclkctl::AdcclkctlSpec>;
#[doc = "ADC Clock Control"]
pub mod adcclkctl;
#[doc = "TDCCLKCTL (rw) register accessor: TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. These are the recommended steps to configure and request the counter clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - If DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL=RCOSC_HF (24 or 48 MHz), wait until ACK=1. - If DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL=XOSC_HF, wait until ACK=1 and DDI_0_OSC:STAT2.XOSC_HF_FREQGOOD=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the counter clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdcclkctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdcclkctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdcclkctl`]
module"]
#[doc(alias = "TDCCLKCTL")]
pub type Tdcclkctl = crate::Reg<tdcclkctl::TdcclkctlSpec>;
#[doc = "TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. These are the recommended steps to configure and request the counter clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - If DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL=RCOSC_HF (24 or 48 MHz), wait until ACK=1. - If DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL=XOSC_HF, wait until ACK=1 and DDI_0_OSC:STAT2.XOSC_HF_FREQGOOD=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the counter clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0."]
pub mod tdcclkctl;
#[doc = "TDCREFCLKCTL (rw) register accessor: TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. These are the recommended steps to configure and request the reference clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - Wait until ACK=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the reference clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0. It is not recommended to enable the TDC reference clock if DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL=RCOSCHFDLF (0x0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdcrefclkctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdcrefclkctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdcrefclkctl`]
module"]
#[doc(alias = "TDCREFCLKCTL")]
pub type Tdcrefclkctl = crate::Reg<tdcrefclkctl::TdcrefclkctlSpec>;
#[doc = "TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. These are the recommended steps to configure and request the reference clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - Wait until ACK=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the reference clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0. It is not recommended to enable the TDC reference clock if DDI_0_OSC:CTL0.SCLK_LF_SRC_SEL=RCOSCHFDLF (0x0)."]
pub mod tdcrefclkctl;
#[doc = "TIMER2CLKCTL (rw) register accessor: AUX_TIMER2 Clock Control Access to AUX_TIMER2 is only possible when TIMER2CLKSTAT.STAT is different from NONE.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2clkctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2clkctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2clkctl`]
module"]
#[doc(alias = "TIMER2CLKCTL")]
pub type Timer2clkctl = crate::Reg<timer2clkctl::Timer2clkctlSpec>;
#[doc = "AUX_TIMER2 Clock Control Access to AUX_TIMER2 is only possible when TIMER2CLKSTAT.STAT is different from NONE."]
pub mod timer2clkctl;
#[doc = "TIMER2CLKSTAT (rw) register accessor: AUX_TIMER2 Clock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2clkstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2clkstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2clkstat`]
module"]
#[doc(alias = "TIMER2CLKSTAT")]
pub type Timer2clkstat = crate::Reg<timer2clkstat::Timer2clkstatSpec>;
#[doc = "AUX_TIMER2 Clock Status"]
pub mod timer2clkstat;
#[doc = "TIMER2CLKSWITCH (rw) register accessor: AUX_TIMER2 Clock Switch\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2clkswitch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2clkswitch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2clkswitch`]
module"]
#[doc(alias = "TIMER2CLKSWITCH")]
pub type Timer2clkswitch = crate::Reg<timer2clkswitch::Timer2clkswitchSpec>;
#[doc = "AUX_TIMER2 Clock Switch"]
pub mod timer2clkswitch;
#[doc = "TIMER2DBGCTL (rw) register accessor: AUX_TIMER2 Debug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2dbgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2dbgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2dbgctl`]
module"]
#[doc(alias = "TIMER2DBGCTL")]
pub type Timer2dbgctl = crate::Reg<timer2dbgctl::Timer2dbgctlSpec>;
#[doc = "AUX_TIMER2 Debug Control"]
pub mod timer2dbgctl;
#[doc = "CLKSHIFTDET (rw) register accessor: Clock Shift Detection A transition in the MCU domain state causes a non-accumulative change to the SCE clock period when the AUX clock rate is derived from SCLK_MF or SCLK_LF: - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles longer when MCU domain enters active state. - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles shorter when MCU domain exits active state. AUX_SCE detects if such events occurred to the SCE clock during the time period between a clear of STAT and a read of STAT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkshiftdet::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkshiftdet::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkshiftdet`]
module"]
#[doc(alias = "CLKSHIFTDET")]
pub type Clkshiftdet = crate::Reg<clkshiftdet::ClkshiftdetSpec>;
#[doc = "Clock Shift Detection A transition in the MCU domain state causes a non-accumulative change to the SCE clock period when the AUX clock rate is derived from SCLK_MF or SCLK_LF: - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles longer when MCU domain enters active state. - A single SCE clock cycle is 6 thru 8 SCLK_HF cycles shorter when MCU domain exits active state. AUX_SCE detects if such events occurred to the SCE clock during the time period between a clear of STAT and a read of STAT."]
pub mod clkshiftdet;
#[doc = "RECHARGETRIG (rw) register accessor: VDDR Recharge Trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rechargetrig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rechargetrig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rechargetrig`]
module"]
#[doc(alias = "RECHARGETRIG")]
pub type Rechargetrig = crate::Reg<rechargetrig::RechargetrigSpec>;
#[doc = "VDDR Recharge Trigger"]
pub mod rechargetrig;
#[doc = "RECHARGEDET (rw) register accessor: VDDR Recharge Detection Some applications can be sensitive to power noise caused by recharge of VDDR. You can detect if VDDR recharge occurs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rechargedet::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rechargedet::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rechargedet`]
module"]
#[doc(alias = "RECHARGEDET")]
pub type Rechargedet = crate::Reg<rechargedet::RechargedetSpec>;
#[doc = "VDDR Recharge Detection Some applications can be sensitive to power noise caused by recharge of VDDR. You can detect if VDDR recharge occurs."]
pub mod rechargedet;
#[doc = "RTCSUBSECINC0 (rw) register accessor: Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set. AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsecinc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsecinc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcsubsecinc0`]
module"]
#[doc(alias = "RTCSUBSECINC0")]
pub type Rtcsubsecinc0 = crate::Reg<rtcsubsecinc0::Rtcsubsecinc0Spec>;
#[doc = "Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set. AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE."]
pub mod rtcsubsecinc0;
#[doc = "RTCSUBSECINC1 (rw) register accessor: Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set. AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsecinc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsecinc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcsubsecinc1`]
module"]
#[doc(alias = "RTCSUBSECINC1")]
pub type Rtcsubsecinc1 = crate::Reg<rtcsubsecinc1::Rtcsubsecinc1Spec>;
#[doc = "Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set. AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE."]
pub mod rtcsubsecinc1;
#[doc = "RTCSUBSECINCCTL (rw) register accessor: Real Time Counter Sub Second Increment Control AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsecincctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsecincctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcsubsecincctl`]
module"]
#[doc(alias = "RTCSUBSECINCCTL")]
pub type Rtcsubsecincctl = crate::Reg<rtcsubsecincctl::RtcsubsecincctlSpec>;
#[doc = "Real Time Counter Sub Second Increment Control AUX_SCE is not allowed to access this register when system state is secure. Any access will suspend the AUX_SCE."]
pub mod rtcsubsecincctl;
#[doc = "RTCSEC (rw) register accessor: Real Time Counter Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SEC.VALUE directly.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcsec`]
module"]
#[doc(alias = "RTCSEC")]
pub type Rtcsec = crate::Reg<rtcsec::RtcsecSpec>;
#[doc = "Real Time Counter Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SEC.VALUE directly."]
pub mod rtcsec;
#[doc = "RTCSUBSEC (rw) register accessor: Real Time Counter Sub-Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SUBSEC.VALUE directly.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcsubsec`]
module"]
#[doc(alias = "RTCSUBSEC")]
pub type Rtcsubsec = crate::Reg<rtcsubsec::RtcsubsecSpec>;
#[doc = "Real Time Counter Sub-Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SUBSEC.VALUE directly."]
pub mod rtcsubsec;
#[doc = "RTCEVCLR (rw) register accessor: AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcevclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcevclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcevclr`]
module"]
#[doc(alias = "RTCEVCLR")]
pub type Rtcevclr = crate::Reg<rtcevclr::RtcevclrSpec>;
#[doc = "AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY."]
pub mod rtcevclr;
#[doc = "BATMONBAT (rw) register accessor: AON_BATMON Battery Voltage Value Read access to AON_BATMON:BAT. System CPU must not access this register. Instead, system CPU must access AON_BATMON:BAT directly. AON_BATMON:BAT updates during VDDR recharge or active operational mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batmonbat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batmonbat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@batmonbat`]
module"]
#[doc(alias = "BATMONBAT")]
pub type Batmonbat = crate::Reg<batmonbat::BatmonbatSpec>;
#[doc = "AON_BATMON Battery Voltage Value Read access to AON_BATMON:BAT. System CPU must not access this register. Instead, system CPU must access AON_BATMON:BAT directly. AON_BATMON:BAT updates during VDDR recharge or active operational mode."]
pub mod batmonbat;
#[doc = "BATMONTEMP (rw) register accessor: AON_BATMON Temperature Value Read access to AON_BATMON:TEMP. System CPU must not access this register. Instead, system CPU must access AON_BATMON:TEMP directly. AON_BATMON:TEMP updates during VDDR recharge or active operational mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batmontemp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batmontemp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@batmontemp`]
module"]
#[doc(alias = "BATMONTEMP")]
pub type Batmontemp = crate::Reg<batmontemp::BatmontempSpec>;
#[doc = "AON_BATMON Temperature Value Read access to AON_BATMON:TEMP. System CPU must not access this register. Instead, system CPU must access AON_BATMON:TEMP directly. AON_BATMON:TEMP updates during VDDR recharge or active operational mode."]
pub mod batmontemp;
#[doc = "TIMERHALT (rw) register accessor: Timer Halt Debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timerhalt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timerhalt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timerhalt`]
module"]
#[doc(alias = "TIMERHALT")]
pub type Timerhalt = crate::Reg<timerhalt::TimerhaltSpec>;
#[doc = "Timer Halt Debug register"]
pub mod timerhalt;
#[doc = "TIMER2BRIDGE (rw) register accessor: AUX_TIMER2 Bridge\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2bridge::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2bridge::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2bridge`]
module"]
#[doc(alias = "TIMER2BRIDGE")]
pub type Timer2bridge = crate::Reg<timer2bridge::Timer2bridgeSpec>;
#[doc = "AUX_TIMER2 Bridge"]
pub mod timer2bridge;
#[doc = "SWPWRPROF (rw) register accessor: Software Power Profiler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swpwrprof::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swpwrprof::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swpwrprof`]
module"]
#[doc(alias = "SWPWRPROF")]
pub type Swpwrprof = crate::Reg<swpwrprof::SwpwrprofSpec>;
#[doc = "Software Power Profiler"]
pub mod swpwrprof;
