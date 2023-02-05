#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration"]
    pub cfg: CFG,
    #[doc = "0x04 - Timer A Mode"]
    pub tamr: TAMR,
    #[doc = "0x08 - Timer B Mode"]
    pub tbmr: TBMR,
    #[doc = "0x0c - Control"]
    pub ctl: CTL,
    #[doc = "0x10 - Synch Register"]
    pub sync: SYNC,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - Interrupt Mask This register is used to enable the interrupts. Associated registers: RIS, MIS, ICLR"]
    pub imr: IMR,
    #[doc = "0x1c - Raw Interrupt Status Associated registers: IMR, MIS, ICLR"]
    pub ris: RIS,
    #[doc = "0x20 - Masked Interrupt Status Values are result of bitwise AND operation between RIS and IMR Assosciated clear register: ICLR"]
    pub mis: MIS,
    #[doc = "0x24 - Interrupt Clear This register is used to clear status bits in the RIS and MIS registers"]
    pub iclr: ICLR,
    #[doc = "0x28 - Timer A Interval Load Register"]
    pub tailr: TAILR,
    #[doc = "0x2c - Timer B Interval Load Register"]
    pub tbilr: TBILR,
    #[doc = "0x30 - Timer A Match Register Interrupts can be generated when the timer value is equal to the value in this register in one-shot or periodic mode. In Edge-Count mode, this register along with TAILR, determines how many edge events are counted. The total number of edge events counted is equal to the value in TAILR minus this value. Note that in edge-count mode, when executing an up-count, the value of TAPR and TAILR must be greater than the value of TAPMR and this register. In PWM mode, this value along with TAILR, determines the duty cycle of the output PWM signal. When a 16/32-bit GPT is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register. (The upper 16-bits correspond to the contents TBMATCHR). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR. Note : This register is updated internally (takes effect) based on TAMR.TAMRSU"]
    pub tamatchr: TAMATCHR,
    #[doc = "0x34 - Timer B Match Register When a GPT is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of TAMATCHR. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits 15:0 are used for the match value. Bits 31:16 are reserved in both cases. Note : This register is updated internally (takes effect) based on TBMR.TBMRSU"]
    pub tbmatchr: TBMATCHR,
    #[doc = "0x38 - Timer A Pre-scale This register allows software to extend the range of the timers when they are used individually. When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter. When acting as a true prescaler, the prescaler counts down to 0 before the value in TAR and TAV registers are incremented. In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT."]
    pub tapr: TAPR,
    #[doc = "0x3c - Timer B Pre-scale This register allows software to extend the range of the timers when they are used individually. When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter. When acting as a true prescaler, the prescaler counts down to 0 before the value in TBR and TBV registers are incremented. In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT."]
    pub tbpr: TBPR,
    #[doc = "0x40 - Timer A Pre-scale Match This register allows software to extend the range of the TAMATCHR when used individually."]
    pub tapmr: TAPMR,
    #[doc = "0x44 - Timer B Pre-scale Match This register allows software to extend the range of the TBMATCHR when used individually."]
    pub tbpmr: TBPMR,
    #[doc = "0x48 - Timer A Register This register shows the current value of the Timer A counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place. When a GPT is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the Timer B (TBR) register). In the16-bit Input Edge Count, Input Edge Time, and PWM modes, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \\[23:16\\]
in the TAV register. To read the value of the prescalar in periodic snapshot mode, read the Timer A Prescale Snapshot (TAPS) register."]
    pub tar: TAR,
    #[doc = "0x4c - Timer B Register This register shows the current value of the Timer B counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler in Input Edge Count, Input Edge Time, and PWM modes, which is the upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \\[23:16\\]
in the TBV register. To read the value of the prescalar in periodic snapshot mode, read the Timer B Prescale Snapshot (TBPS) register."]
    pub tbr: TBR,
    #[doc = "0x50 - Timer A Value When read, this register shows the current, free-running value of Timer A in all modes. Softwarecan use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a 16/32-bit GPTM is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0."]
    pub tav: TAV,
    #[doc = "0x54 - Timer B Value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a 16/32-bit GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0."]
    pub tbv: TBV,
    _reserved21: [u8; 0x04],
    #[doc = "0x5c - Timer A Pre-scale Snap-shot Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAPR register either on the next cycle or on the next timeout. This register shows the current value of the Timer A pre-scaler in the 16-bit mode."]
    pub taps: TAPS,
    #[doc = "0x60 - Timer B Pre-scale Snap-shot Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBPR register either on the next cycle or on the next timeout. This register shows the current value of the Timer B pre-scaler in the 16-bit mode."]
    pub tbps: TBPS,
    #[doc = "0x64 - Timer A Pre-scale Value This register shows the current value of the Timer A free running pre-scaler in the 16-bit mode."]
    pub tapv: TAPV,
    #[doc = "0x68 - Timer B Pre-scale Value This register shows the current value of the Timer B free running pre-scaler in the 16-bit mode."]
    pub tbpv: TBPV,
    #[doc = "0x6c - DMA Event This register allows software to enable/disable GPT DMA trigger events."]
    pub dmaev: DMAEV,
    _reserved26: [u8; 0x0f40],
    #[doc = "0xfb0 - Peripheral Version This register provides information regarding the GPT version"]
    pub version: VERSION,
    #[doc = "0xfb4 - Combined CCP Output This register is used to logically AND CCP output pairs for each timer"]
    pub andccp: ANDCCP,
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration"]
pub mod cfg;
#[doc = "TAMR (rw) register accessor: an alias for `Reg<TAMR_SPEC>`"]
pub type TAMR = crate::Reg<tamr::TAMR_SPEC>;
#[doc = "Timer A Mode"]
pub mod tamr;
#[doc = "TBMR (rw) register accessor: an alias for `Reg<TBMR_SPEC>`"]
pub type TBMR = crate::Reg<tbmr::TBMR_SPEC>;
#[doc = "Timer B Mode"]
pub mod tbmr;
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "SYNC (rw) register accessor: an alias for `Reg<SYNC_SPEC>`"]
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
#[doc = "Synch Register"]
pub mod sync;
#[doc = "IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask This register is used to enable the interrupts. Associated registers: RIS, MIS, ICLR"]
pub mod imr;
#[doc = "RIS (rw) register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Raw Interrupt Status Associated registers: IMR, MIS, ICLR"]
pub mod ris;
#[doc = "MIS (rw) register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Masked Interrupt Status Values are result of bitwise AND operation between RIS and IMR Assosciated clear register: ICLR"]
pub mod mis;
#[doc = "ICLR (rw) register accessor: an alias for `Reg<ICLR_SPEC>`"]
pub type ICLR = crate::Reg<iclr::ICLR_SPEC>;
#[doc = "Interrupt Clear This register is used to clear status bits in the RIS and MIS registers"]
pub mod iclr;
#[doc = "TAILR (rw) register accessor: an alias for `Reg<TAILR_SPEC>`"]
pub type TAILR = crate::Reg<tailr::TAILR_SPEC>;
#[doc = "Timer A Interval Load Register"]
pub mod tailr;
#[doc = "TBILR (rw) register accessor: an alias for `Reg<TBILR_SPEC>`"]
pub type TBILR = crate::Reg<tbilr::TBILR_SPEC>;
#[doc = "Timer B Interval Load Register"]
pub mod tbilr;
#[doc = "TAMATCHR (rw) register accessor: an alias for `Reg<TAMATCHR_SPEC>`"]
pub type TAMATCHR = crate::Reg<tamatchr::TAMATCHR_SPEC>;
#[doc = "Timer A Match Register Interrupts can be generated when the timer value is equal to the value in this register in one-shot or periodic mode. In Edge-Count mode, this register along with TAILR, determines how many edge events are counted. The total number of edge events counted is equal to the value in TAILR minus this value. Note that in edge-count mode, when executing an up-count, the value of TAPR and TAILR must be greater than the value of TAPMR and this register. In PWM mode, this value along with TAILR, determines the duty cycle of the output PWM signal. When a 16/32-bit GPT is configured to one of the 32-bit modes, TAMATCHR appears as a 32-bit register. (The upper 16-bits correspond to the contents TBMATCHR). In a 16-bit mode, the upper 16 bits of this register read as 0s and have no effect on the state of TBMATCHR. Note : This register is updated internally (takes effect) based on TAMR.TAMRSU"]
pub mod tamatchr;
#[doc = "TBMATCHR (rw) register accessor: an alias for `Reg<TBMATCHR_SPEC>`"]
pub type TBMATCHR = crate::Reg<tbmatchr::TBMATCHR_SPEC>;
#[doc = "Timer B Match Register When a GPT is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of TAMATCHR. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits 15:0 are used for the match value. Bits 31:16 are reserved in both cases. Note : This register is updated internally (takes effect) based on TBMR.TBMRSU"]
pub mod tbmatchr;
#[doc = "TAPR (rw) register accessor: an alias for `Reg<TAPR_SPEC>`"]
pub type TAPR = crate::Reg<tapr::TAPR_SPEC>;
#[doc = "Timer A Pre-scale This register allows software to extend the range of the timers when they are used individually. When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter. When acting as a true prescaler, the prescaler counts down to 0 before the value in TAR and TAV registers are incremented. In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT."]
pub mod tapr;
#[doc = "TBPR (rw) register accessor: an alias for `Reg<TBPR_SPEC>`"]
pub type TBPR = crate::Reg<tbpr::TBPR_SPEC>;
#[doc = "Timer B Pre-scale This register allows software to extend the range of the timers when they are used individually. When in one-shot or periodic down count modes, this register acts as a true prescaler for the timer counter. When acting as a true prescaler, the prescaler counts down to 0 before the value in TBR and TBV registers are incremented. In all other individual/split modes, this register is a linear extension of the upper range of the timer counter, holding bits 23:16 in the 16-bit modes of the 16/32-bit GPT."]
pub mod tbpr;
#[doc = "TAPMR (rw) register accessor: an alias for `Reg<TAPMR_SPEC>`"]
pub type TAPMR = crate::Reg<tapmr::TAPMR_SPEC>;
#[doc = "Timer A Pre-scale Match This register allows software to extend the range of the TAMATCHR when used individually."]
pub mod tapmr;
#[doc = "TBPMR (rw) register accessor: an alias for `Reg<TBPMR_SPEC>`"]
pub type TBPMR = crate::Reg<tbpmr::TBPMR_SPEC>;
#[doc = "Timer B Pre-scale Match This register allows software to extend the range of the TBMATCHR when used individually."]
pub mod tbpmr;
#[doc = "TAR (rw) register accessor: an alias for `Reg<TAR_SPEC>`"]
pub type TAR = crate::Reg<tar::TAR_SPEC>;
#[doc = "Timer A Register This register shows the current value of the Timer A counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place. When a GPT is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the Timer B (TBR) register). In the16-bit Input Edge Count, Input Edge Time, and PWM modes, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler, which is the upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \\[23:16\\]
in the TAV register. To read the value of the prescalar in periodic snapshot mode, read the Timer A Prescale Snapshot (TAPS) register."]
pub mod tar;
#[doc = "TBR (rw) register accessor: an alias for `Reg<TBR_SPEC>`"]
pub type TBR = crate::Reg<tbr::TBR_SPEC>;
#[doc = "Timer B Register This register shows the current value of the Timer B counter in all cases except for Input Edge Count and Time modes. In the Input Edge Count mode, this register contains the number of edges that have occurred. In the Input Edge Time mode, this register contains the time at which the last edge event took place. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the value of the prescaler in Input Edge Count, Input Edge Time, and PWM modes, which is the upper 8 bits of the count. Bits 31:24 always read as 0. To read the value of the prescaler in 16-bit One-Shot and Periodic modes, read bits \\[23:16\\]
in the TBV register. To read the value of the prescalar in periodic snapshot mode, read the Timer B Prescale Snapshot (TBPS) register."]
pub mod tbr;
#[doc = "TAV (rw) register accessor: an alias for `Reg<TAV_SPEC>`"]
pub type TAV = crate::Reg<tav::TAV_SPEC>;
#[doc = "Timer A Value When read, this register shows the current, free-running value of Timer A in all modes. Softwarecan use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a 16/32-bit GPTM is configured to one of the 32-bit modes, this register appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0."]
pub mod tav;
#[doc = "TBV (rw) register accessor: an alias for `Reg<TBV_SPEC>`"]
pub type TBV = crate::Reg<tbv::TBV_SPEC>;
#[doc = "Timer B Value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a 16/32-bit GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits 23:16 contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in Input Edge Count, Input Edge Time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in 23:16 is a true prescaler, meaning bits 23:16 count down before decrementing the value in bits 15:0. The prescaler in bits 31:24 always reads as 0."]
pub mod tbv;
#[doc = "TAPS (rw) register accessor: an alias for `Reg<TAPS_SPEC>`"]
pub type TAPS = crate::Reg<taps::TAPS_SPEC>;
#[doc = "Timer A Pre-scale Snap-shot Based on the value in the register field TAMR.TAILD, this register is updated with the value from TAPR register either on the next cycle or on the next timeout. This register shows the current value of the Timer A pre-scaler in the 16-bit mode."]
pub mod taps;
#[doc = "TBPS (rw) register accessor: an alias for `Reg<TBPS_SPEC>`"]
pub type TBPS = crate::Reg<tbps::TBPS_SPEC>;
#[doc = "Timer B Pre-scale Snap-shot Based on the value in the register field TBMR.TBILD, this register is updated with the value from TBPR register either on the next cycle or on the next timeout. This register shows the current value of the Timer B pre-scaler in the 16-bit mode."]
pub mod tbps;
#[doc = "TAPV (rw) register accessor: an alias for `Reg<TAPV_SPEC>`"]
pub type TAPV = crate::Reg<tapv::TAPV_SPEC>;
#[doc = "Timer A Pre-scale Value This register shows the current value of the Timer A free running pre-scaler in the 16-bit mode."]
pub mod tapv;
#[doc = "TBPV (rw) register accessor: an alias for `Reg<TBPV_SPEC>`"]
pub type TBPV = crate::Reg<tbpv::TBPV_SPEC>;
#[doc = "Timer B Pre-scale Value This register shows the current value of the Timer B free running pre-scaler in the 16-bit mode."]
pub mod tbpv;
#[doc = "DMAEV (rw) register accessor: an alias for `Reg<DMAEV_SPEC>`"]
pub type DMAEV = crate::Reg<dmaev::DMAEV_SPEC>;
#[doc = "DMA Event This register allows software to enable/disable GPT DMA trigger events."]
pub mod dmaev;
#[doc = "VERSION (rw) register accessor: an alias for `Reg<VERSION_SPEC>`"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Peripheral Version This register provides information regarding the GPT version"]
pub mod version;
#[doc = "ANDCCP (rw) register accessor: an alias for `Reg<ANDCCP_SPEC>`"]
pub type ANDCCP = crate::Reg<andccp::ANDCCP_SPEC>;
#[doc = "Combined CCP Output This register is used to logically AND CCP output pairs for each timer"]
pub mod andccp;
