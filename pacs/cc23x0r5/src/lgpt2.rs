#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    descex: Descex,
    startcfg: Startcfg,
    ctl: Ctl,
    outctl: Outctl,
    cntr: Cntr,
    precfg: Precfg,
    preevent: Preevent,
    chfilt: Chfilt,
    _reserved9: [u8; 0x10],
    qdecstat: Qdecstat,
    _reserved10: [u8; 0x04],
    dma: Dma,
    dmarw: Dmarw,
    adctrg: Adctrg,
    ioctl: Ioctl,
    _reserved14: [u8; 0x1c],
    imask: Imask,
    ris: Ris,
    mis: Mis,
    iset: Iset,
    iclr: Iclr,
    imset: Imset,
    imclr: Imclr,
    emu: Emu,
    _reserved22: [u8; 0x38],
    c0cfg: C0cfg,
    c1cfg: C1cfg,
    c2cfg: C2cfg,
    _reserved25: [u8; 0x30],
    ptgt: Ptgt,
    pc0cc: Pc0cc,
    pc1cc: Pc1cc,
    pc2cc: Pc2cc,
    _reserved29: [u8; 0x30],
    tgt: Tgt,
    c0cc: C0cc,
    c1cc: C1cc,
    c2cc: C2cc,
    _reserved33: [u8; 0x30],
    ptgtnc: Ptgtnc,
    pc0ccnc: Pc0ccnc,
    pc1ccnc: Pc1ccnc,
    pc2ccnc: Pc2ccnc,
    _reserved37: [u8; 0x30],
    tgtnc: Tgtnc,
    c0ccnc: C0ccnc,
    c1ccnc: C1ccnc,
    c2ccnc: C2ccnc,
}
impl RegisterBlock {
    #[doc = "0x00 - Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x04 - Description Extended This register describes the parameters of the LGPT."]
    #[inline(always)]
    pub const fn descex(&self) -> &Descex {
        &self.descex
    }
    #[doc = "0x08 - Start Configuration This register is only for when CTL.MODE is configured to one of the SYNC modes. This register defines when this LGPT starts."]
    #[inline(always)]
    pub const fn startcfg(&self) -> &Startcfg {
        &self.startcfg
    }
    #[doc = "0x0c - Timer Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x10 - Output Control Set and clear individual outputs manually. Manual update of an output takes priority over automatic channel updates to the same output. It is not possible to set and clear an output at the same time, such requests will be neglected. An output can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an output at the same time. All outputs are connected to the event fabric and the IO controller. The outputs going to the IO controller have an aditional complementary output, this output is the inverted IO output. Both the IO and the IO complementary outputs are passed through an IO Controller, see IOCTL."]
    #[inline(always)]
    pub const fn outctl(&self) -> &Outctl {
        &self.outctl
    }
    #[doc = "0x14 - Counter The counter of this timer. After CTL.MODE is set the counter updates at the rate specified in PRECFG."]
    #[inline(always)]
    pub const fn cntr(&self) -> &Cntr {
        &self.cntr
    }
    #[doc = "0x18 - Clock Prescaler Configuration This register is used to set the timer clock period. The prescaler is a counter which counts down from the value TICKDIV. When the prescaler counter reaches zero, CNTR is updated. The field TICKDIV effectively divides the prescaler tick source. The timer clock frequency can be calculated as TICKSRC/(TICKDIV+1)."]
    #[inline(always)]
    pub const fn precfg(&self) -> &Precfg {
        &self.precfg
    }
    #[doc = "0x1c - Prescaler Event This register is used to output a logic high signal before the zero crossing of the prescaler counter. The output is routed to the IOC."]
    #[inline(always)]
    pub const fn preevent(&self) -> &Preevent {
        &self.preevent
    }
    #[doc = "0x20 - Channel Input Filter This register is used to configure the filter on the channel inputs. The configuration is for all inputs. The filter is enabled when a channel is in capture mode. The input to the filter is passed to the edge detection logic if LOAD + 1 consecutive input samples are equal. The filter functions as a down counter, counting down every input sample. If two consecutive samples are unequal, the filter counter restarts from LOAD. If the filter counter reaches zero, the input signal is valid and passed to the edge detection logic. The channel filter should only be configured while the CTL.MODE = DIS. Configuring the filter while the timer is running can result in unexpected behavior."]
    #[inline(always)]
    pub const fn chfilt(&self) -> &Chfilt {
        &self.chfilt
    }
    #[doc = "0x34 - Quadrature Decoder Status This register can be used during QDEC mode to check the status of the quadrature decoder."]
    #[inline(always)]
    pub const fn qdecstat(&self) -> &Qdecstat {
        &self.qdecstat
    }
    #[doc = "0x3c - Direct Memory Accsess This register is used to enable DMA requests from the timer and set the register addresses which the DMA will access (read/write). Choose DMA request source by setting the REQ field. The setting of the corresponding interrupt in the RIS registers also sets the DMA request. Upon a DMA request defined by REQ an internal address pointer is set to RWADDR*4. Every access to DMARW will increment the internal pointer by 4 such that the next DMA access will be to the next register. The internal pointer will stop after RWCNTR increments. Further access will be ignored."]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
    #[doc = "0x40 - Direct Memory Access This register is used by the DMA to access (read/write) register inside this LGPT module. Each access to this register will increment the internal DMA address counter. See DMA for description."]
    #[inline(always)]
    pub const fn dmarw(&self) -> &Dmarw {
        &self.dmarw
    }
    #[doc = "0x44 - ADC Trigger This register is used to enable ADC trigger from the timer. Choose ADC trigger source by setting the SRC field. The setting of the corresponding interrupt in the RIS registers also sets the ADC trigger."]
    #[inline(always)]
    pub const fn adctrg(&self) -> &Adctrg {
        &self.adctrg
    }
    #[doc = "0x48 - IO Controller This register controls the IO outputs."]
    #[inline(always)]
    pub const fn ioctl(&self) -> &Ioctl {
        &self.ioctl
    }
    #[doc = "0x68 - Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
    #[inline(always)]
    pub const fn imask(&self) -> &Imask {
        &self.imask
    }
    #[doc = "0x6c - Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x70 - Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x74 - Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
    #[inline(always)]
    pub const fn iset(&self) -> &Iset {
        &self.iset
    }
    #[doc = "0x78 - Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared."]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
    }
    #[doc = "0x7c - Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit."]
    #[inline(always)]
    pub const fn imset(&self) -> &Imset {
        &self.imset
    }
    #[doc = "0x80 - Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit."]
    #[inline(always)]
    pub const fn imclr(&self) -> &Imclr {
        &self.imclr
    }
    #[doc = "0x84 - Debug control This register can be used to freeze the timer when CPU halts when HALT is set to 1. When HALT is set to 0, or when the CPU releases debug halt, the filters and edge detection logic is flushed and the timer starts. For setting a predefined output value during a CPU debug halt, PARK, if the timer has this register, should be configured additionally. If this timer does not have the PARK register a predefined output value during CPU halt is not possible."]
    #[inline(always)]
    pub const fn emu(&self) -> &Emu {
        &self.emu
    }
    #[doc = "0xc0 - Channel 0 Configuration This register configures channel function and enables outputs. Each channel has an edge-detection circuit. The the edge-detection circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and CTL.MODE is changed from DIS to another mode. The flush action uses two system clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit. The channel input signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described above. - the CCACT field is reconfigured while CTL.MODE is different from DIS. Primary use scenario is to select CCACT before starting the timer. Follow these steps to configure CCACT to a capture action while CTL.MODE is different from DIS: - Set EDGE to NONE. - Configure CCACT. - Wait for three system clock periods before setting EDGE different from NONE. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    #[inline(always)]
    pub const fn c0cfg(&self) -> &C0cfg {
        &self.c0cfg
    }
    #[doc = "0xc4 - Channel 1 Configuration This register configures channel function and enables outputs. Each channel has an edge-detection circuit. The the edge-detection circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and CTL.MODE is changed from DIS to another mode. The flush action uses two system clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit. The channel input signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described above. - the CCACT field is reconfigured while CTL.MODE is different from DIS. Primary use scenario is to select CCACT before starting the timer. Follow these steps to configure CCACT to a capture action while CTL.MODE is different from DIS: - Set EDGE to NONE. - Configure CCACT. - Wait for three system clock periods before setting EDGE different from NONE. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    #[inline(always)]
    pub const fn c1cfg(&self) -> &C1cfg {
        &self.c1cfg
    }
    #[doc = "0xc8 - Channel 2 Configuration This register configures channel function and enables outputs. Each channel has an edge-detection circuit. The the edge-detection circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and CTL.MODE is changed from DIS to another mode. The flush action uses two system clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit. The channel input signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described above. - the CCACT field is reconfigured while CTL.MODE is different from DIS. Primary use scenario is to select CCACT before starting the timer. Follow these steps to configure CCACT to a capture action while CTL.MODE is different from DIS: - Set EDGE to NONE. - Configure CCACT. - Wait for three system clock periods before setting EDGE different from NONE. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
    #[inline(always)]
    pub const fn c2cfg(&self) -> &C2cfg {
        &self.c2cfg
    }
    #[doc = "0xfc - Pipeline Target A read or write to this register will clear the RIS.ZERO and RIS.TGT interrupt. If CTL.MODE != QDEC. Target value for next counter period. The timer will copy PTGT.VAL to TGT.VAL on the upcoming CNTR zero crossing only if PTGT.VAL has been written. The copy does not happen when restarting the timer. This is useful to avoid period jitter in PWM applications with time-varying period, sometimes referenced as phase corrected PWM. If CTL.MODE = QDEC The CNTR value is updated with VALUE on IDX if the counter is counting down. If the counter is counting up, CNTR is loaded with zero on IDX. In this mode the VALUE is not loaded into TGT on zero crossing."]
    #[inline(always)]
    pub const fn ptgt(&self) -> &Ptgt {
        &self.ptgt
    }
    #[doc = "0x100 - Pipeline Channel 0 Capture Compare"]
    #[inline(always)]
    pub const fn pc0cc(&self) -> &Pc0cc {
        &self.pc0cc
    }
    #[doc = "0x104 - Pipeline Channel 1 Capture Compare"]
    #[inline(always)]
    pub const fn pc1cc(&self) -> &Pc1cc {
        &self.pc1cc
    }
    #[doc = "0x108 - Pipeline Channel 2 Capture Compare"]
    #[inline(always)]
    pub const fn pc2cc(&self) -> &Pc2cc {
        &self.pc2cc
    }
    #[doc = "0x13c - Target User defined counter target. A read or write to this register will clear the RIS.ZERO and RIS.TGT interrupt."]
    #[inline(always)]
    pub const fn tgt(&self) -> &Tgt {
        &self.tgt
    }
    #[doc = "0x140 - Channel 0 Capture Compare"]
    #[inline(always)]
    pub const fn c0cc(&self) -> &C0cc {
        &self.c0cc
    }
    #[doc = "0x144 - Channel 1 Capture Compare"]
    #[inline(always)]
    pub const fn c1cc(&self) -> &C1cc {
        &self.c1cc
    }
    #[doc = "0x148 - Channel 2 Capture Compare"]
    #[inline(always)]
    pub const fn c2cc(&self) -> &C2cc {
        &self.c2cc
    }
    #[doc = "0x17c - Pipeline Target No Clear Use this register to read or write to PTGT without clearing the RIS.ZERO and RIS.TGT interrupt."]
    #[inline(always)]
    pub const fn ptgtnc(&self) -> &Ptgtnc {
        &self.ptgtnc
    }
    #[doc = "0x180 - Pipeline Channel 0 Capture Compare No Clear"]
    #[inline(always)]
    pub const fn pc0ccnc(&self) -> &Pc0ccnc {
        &self.pc0ccnc
    }
    #[doc = "0x184 - Pipeline Channel 1 Capture Compare No Clear"]
    #[inline(always)]
    pub const fn pc1ccnc(&self) -> &Pc1ccnc {
        &self.pc1ccnc
    }
    #[doc = "0x188 - Pipeline Channel 2 Capture Compare No Clear"]
    #[inline(always)]
    pub const fn pc2ccnc(&self) -> &Pc2ccnc {
        &self.pc2ccnc
    }
    #[doc = "0x1bc - Target No Clear Use this register to read or write to TGT without clearing the RIS.ZERO and RIS.TGT interrupt."]
    #[inline(always)]
    pub const fn tgtnc(&self) -> &Tgtnc {
        &self.tgtnc
    }
    #[doc = "0x1c0 - Channel 0 Capture Compare No Clear"]
    #[inline(always)]
    pub const fn c0ccnc(&self) -> &C0ccnc {
        &self.c0ccnc
    }
    #[doc = "0x1c4 - Channel 1 Capture Compare No Clear"]
    #[inline(always)]
    pub const fn c1ccnc(&self) -> &C1ccnc {
        &self.c1ccnc
    }
    #[doc = "0x1c8 - Channel 2 Capture Compare No Clear"]
    #[inline(always)]
    pub const fn c2ccnc(&self) -> &C2ccnc {
        &self.c2ccnc
    }
}
#[doc = "DESC (rw) register accessor: Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
pub mod desc;
#[doc = "DESCEX (rw) register accessor: Description Extended This register describes the parameters of the LGPT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descex`]
module"]
#[doc(alias = "DESCEX")]
pub type Descex = crate::Reg<descex::DescexSpec>;
#[doc = "Description Extended This register describes the parameters of the LGPT."]
pub mod descex;
#[doc = "STARTCFG (rw) register accessor: Start Configuration This register is only for when CTL.MODE is configured to one of the SYNC modes. This register defines when this LGPT starts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`startcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`startcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@startcfg`]
module"]
#[doc(alias = "STARTCFG")]
pub type Startcfg = crate::Reg<startcfg::StartcfgSpec>;
#[doc = "Start Configuration This register is only for when CTL.MODE is configured to one of the SYNC modes. This register defines when this LGPT starts."]
pub mod startcfg;
#[doc = "CTL (rw) register accessor: Timer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Timer Control"]
pub mod ctl;
#[doc = "OUTCTL (rw) register accessor: Output Control Set and clear individual outputs manually. Manual update of an output takes priority over automatic channel updates to the same output. It is not possible to set and clear an output at the same time, such requests will be neglected. An output can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an output at the same time. All outputs are connected to the event fabric and the IO controller. The outputs going to the IO controller have an aditional complementary output, this output is the inverted IO output. Both the IO and the IO complementary outputs are passed through an IO Controller, see IOCTL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outctl`]
module"]
#[doc(alias = "OUTCTL")]
pub type Outctl = crate::Reg<outctl::OutctlSpec>;
#[doc = "Output Control Set and clear individual outputs manually. Manual update of an output takes priority over automatic channel updates to the same output. It is not possible to set and clear an output at the same time, such requests will be neglected. An output can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an output at the same time. All outputs are connected to the event fabric and the IO controller. The outputs going to the IO controller have an aditional complementary output, this output is the inverted IO output. Both the IO and the IO complementary outputs are passed through an IO Controller, see IOCTL."]
pub mod outctl;
#[doc = "CNTR (rw) register accessor: Counter The counter of this timer. After CTL.MODE is set the counter updates at the rate specified in PRECFG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`]
module"]
#[doc(alias = "CNTR")]
pub type Cntr = crate::Reg<cntr::CntrSpec>;
#[doc = "Counter The counter of this timer. After CTL.MODE is set the counter updates at the rate specified in PRECFG."]
pub mod cntr;
#[doc = "PRECFG (rw) register accessor: Clock Prescaler Configuration This register is used to set the timer clock period. The prescaler is a counter which counts down from the value TICKDIV. When the prescaler counter reaches zero, CNTR is updated. The field TICKDIV effectively divides the prescaler tick source. The timer clock frequency can be calculated as TICKSRC/(TICKDIV+1).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`precfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`precfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@precfg`]
module"]
#[doc(alias = "PRECFG")]
pub type Precfg = crate::Reg<precfg::PrecfgSpec>;
#[doc = "Clock Prescaler Configuration This register is used to set the timer clock period. The prescaler is a counter which counts down from the value TICKDIV. When the prescaler counter reaches zero, CNTR is updated. The field TICKDIV effectively divides the prescaler tick source. The timer clock frequency can be calculated as TICKSRC/(TICKDIV+1)."]
pub mod precfg;
#[doc = "PREEVENT (rw) register accessor: Prescaler Event This register is used to output a logic high signal before the zero crossing of the prescaler counter. The output is routed to the IOC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`preevent::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`preevent::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@preevent`]
module"]
#[doc(alias = "PREEVENT")]
pub type Preevent = crate::Reg<preevent::PreeventSpec>;
#[doc = "Prescaler Event This register is used to output a logic high signal before the zero crossing of the prescaler counter. The output is routed to the IOC."]
pub mod preevent;
#[doc = "CHFILT (rw) register accessor: Channel Input Filter This register is used to configure the filter on the channel inputs. The configuration is for all inputs. The filter is enabled when a channel is in capture mode. The input to the filter is passed to the edge detection logic if LOAD + 1 consecutive input samples are equal. The filter functions as a down counter, counting down every input sample. If two consecutive samples are unequal, the filter counter restarts from LOAD. If the filter counter reaches zero, the input signal is valid and passed to the edge detection logic. The channel filter should only be configured while the CTL.MODE = DIS. Configuring the filter while the timer is running can result in unexpected behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chfilt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chfilt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chfilt`]
module"]
#[doc(alias = "CHFILT")]
pub type Chfilt = crate::Reg<chfilt::ChfiltSpec>;
#[doc = "Channel Input Filter This register is used to configure the filter on the channel inputs. The configuration is for all inputs. The filter is enabled when a channel is in capture mode. The input to the filter is passed to the edge detection logic if LOAD + 1 consecutive input samples are equal. The filter functions as a down counter, counting down every input sample. If two consecutive samples are unequal, the filter counter restarts from LOAD. If the filter counter reaches zero, the input signal is valid and passed to the edge detection logic. The channel filter should only be configured while the CTL.MODE = DIS. Configuring the filter while the timer is running can result in unexpected behavior."]
pub mod chfilt;
#[doc = "QDECSTAT (rw) register accessor: Quadrature Decoder Status This register can be used during QDEC mode to check the status of the quadrature decoder.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qdecstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qdecstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qdecstat`]
module"]
#[doc(alias = "QDECSTAT")]
pub type Qdecstat = crate::Reg<qdecstat::QdecstatSpec>;
#[doc = "Quadrature Decoder Status This register can be used during QDEC mode to check the status of the quadrature decoder."]
pub mod qdecstat;
#[doc = "DMA (rw) register accessor: Direct Memory Accsess This register is used to enable DMA requests from the timer and set the register addresses which the DMA will access (read/write). Choose DMA request source by setting the REQ field. The setting of the corresponding interrupt in the RIS registers also sets the DMA request. Upon a DMA request defined by REQ an internal address pointer is set to RWADDR*4. Every access to DMARW will increment the internal pointer by 4 such that the next DMA access will be to the next register. The internal pointer will stop after RWCNTR increments. Further access will be ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`]
module"]
#[doc(alias = "DMA")]
pub type Dma = crate::Reg<dma::DmaSpec>;
#[doc = "Direct Memory Accsess This register is used to enable DMA requests from the timer and set the register addresses which the DMA will access (read/write). Choose DMA request source by setting the REQ field. The setting of the corresponding interrupt in the RIS registers also sets the DMA request. Upon a DMA request defined by REQ an internal address pointer is set to RWADDR*4. Every access to DMARW will increment the internal pointer by 4 such that the next DMA access will be to the next register. The internal pointer will stop after RWCNTR increments. Further access will be ignored."]
pub mod dma;
#[doc = "DMARW (rw) register accessor: Direct Memory Access This register is used by the DMA to access (read/write) register inside this LGPT module. Each access to this register will increment the internal DMA address counter. See DMA for description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmarw`]
module"]
#[doc(alias = "DMARW")]
pub type Dmarw = crate::Reg<dmarw::DmarwSpec>;
#[doc = "Direct Memory Access This register is used by the DMA to access (read/write) register inside this LGPT module. Each access to this register will increment the internal DMA address counter. See DMA for description."]
pub mod dmarw;
#[doc = "ADCTRG (rw) register accessor: ADC Trigger This register is used to enable ADC trigger from the timer. Choose ADC trigger source by setting the SRC field. The setting of the corresponding interrupt in the RIS registers also sets the ADC trigger.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctrg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctrg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adctrg`]
module"]
#[doc(alias = "ADCTRG")]
pub type Adctrg = crate::Reg<adctrg::AdctrgSpec>;
#[doc = "ADC Trigger This register is used to enable ADC trigger from the timer. Choose ADC trigger source by setting the SRC field. The setting of the corresponding interrupt in the RIS registers also sets the ADC trigger."]
pub mod adctrg;
#[doc = "IOCTL (rw) register accessor: IO Controller This register controls the IO outputs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioctl`]
module"]
#[doc(alias = "IOCTL")]
pub type Ioctl = crate::Reg<ioctl::IoctlSpec>;
#[doc = "IO Controller This register controls the IO outputs."]
pub mod ioctl;
#[doc = "IMASK (rw) register accessor: Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`]
module"]
#[doc(alias = "IMASK")]
pub type Imask = crate::Reg<imask::ImaskSpec>;
#[doc = "Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
pub mod imask;
#[doc = "RIS (rw) register accessor: Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
pub mod ris;
#[doc = "MIS (rw) register accessor: Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
pub mod mis;
#[doc = "ISET (rw) register accessor: Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset`]
module"]
#[doc(alias = "ISET")]
pub type Iset = crate::Reg<iset::IsetSpec>;
#[doc = "Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
pub mod iset;
#[doc = "ICLR (rw) register accessor: Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`]
module"]
#[doc(alias = "ICLR")]
pub type Iclr = crate::Reg<iclr::IclrSpec>;
#[doc = "Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared."]
pub mod iclr;
#[doc = "IMSET (rw) register accessor: Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imset`]
module"]
#[doc(alias = "IMSET")]
pub type Imset = crate::Reg<imset::ImsetSpec>;
#[doc = "Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit."]
pub mod imset;
#[doc = "IMCLR (rw) register accessor: Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imclr`]
module"]
#[doc(alias = "IMCLR")]
pub type Imclr = crate::Reg<imclr::ImclrSpec>;
#[doc = "Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit."]
pub mod imclr;
#[doc = "EMU (rw) register accessor: Debug control This register can be used to freeze the timer when CPU halts when HALT is set to 1. When HALT is set to 0, or when the CPU releases debug halt, the filters and edge detection logic is flushed and the timer starts. For setting a predefined output value during a CPU debug halt, PARK, if the timer has this register, should be configured additionally. If this timer does not have the PARK register a predefined output value during CPU halt is not possible.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emu`]
module"]
#[doc(alias = "EMU")]
pub type Emu = crate::Reg<emu::EmuSpec>;
#[doc = "Debug control This register can be used to freeze the timer when CPU halts when HALT is set to 1. When HALT is set to 0, or when the CPU releases debug halt, the filters and edge detection logic is flushed and the timer starts. For setting a predefined output value during a CPU debug halt, PARK, if the timer has this register, should be configured additionally. If this timer does not have the PARK register a predefined output value during CPU halt is not possible."]
pub mod emu;
#[doc = "C0CFG (rw) register accessor: Channel 0 Configuration This register configures channel function and enables outputs. Each channel has an edge-detection circuit. The the edge-detection circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and CTL.MODE is changed from DIS to another mode. The flush action uses two system clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit. The channel input signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described above. - the CCACT field is reconfigured while CTL.MODE is different from DIS. Primary use scenario is to select CCACT before starting the timer. Follow these steps to configure CCACT to a capture action while CTL.MODE is different from DIS: - Set EDGE to NONE. - Configure CCACT. - Wait for three system clock periods before setting EDGE different from NONE. These steps prevent capture events caused by expired signal values in edge-detection circuit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c0cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c0cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0cfg`]
module"]
#[doc(alias = "C0CFG")]
pub type C0cfg = crate::Reg<c0cfg::C0cfgSpec>;
#[doc = "Channel 0 Configuration This register configures channel function and enables outputs. Each channel has an edge-detection circuit. The the edge-detection circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and CTL.MODE is changed from DIS to another mode. The flush action uses two system clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit. The channel input signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described above. - the CCACT field is reconfigured while CTL.MODE is different from DIS. Primary use scenario is to select CCACT before starting the timer. Follow these steps to configure CCACT to a capture action while CTL.MODE is different from DIS: - Set EDGE to NONE. - Configure CCACT. - Wait for three system clock periods before setting EDGE different from NONE. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
pub mod c0cfg;
#[doc = "C1CFG (rw) register accessor: Channel 1 Configuration This register configures channel function and enables outputs. Each channel has an edge-detection circuit. The the edge-detection circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and CTL.MODE is changed from DIS to another mode. The flush action uses two system clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit. The channel input signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described above. - the CCACT field is reconfigured while CTL.MODE is different from DIS. Primary use scenario is to select CCACT before starting the timer. Follow these steps to configure CCACT to a capture action while CTL.MODE is different from DIS: - Set EDGE to NONE. - Configure CCACT. - Wait for three system clock periods before setting EDGE different from NONE. These steps prevent capture events caused by expired signal values in edge-detection circuit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1cfg`]
module"]
#[doc(alias = "C1CFG")]
pub type C1cfg = crate::Reg<c1cfg::C1cfgSpec>;
#[doc = "Channel 1 Configuration This register configures channel function and enables outputs. Each channel has an edge-detection circuit. The the edge-detection circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and CTL.MODE is changed from DIS to another mode. The flush action uses two system clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit. The channel input signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described above. - the CCACT field is reconfigured while CTL.MODE is different from DIS. Primary use scenario is to select CCACT before starting the timer. Follow these steps to configure CCACT to a capture action while CTL.MODE is different from DIS: - Set EDGE to NONE. - Configure CCACT. - Wait for three system clock periods before setting EDGE different from NONE. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
pub mod c1cfg;
#[doc = "C2CFG (rw) register accessor: Channel 2 Configuration This register configures channel function and enables outputs. Each channel has an edge-detection circuit. The the edge-detection circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and CTL.MODE is changed from DIS to another mode. The flush action uses two system clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit. The channel input signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described above. - the CCACT field is reconfigured while CTL.MODE is different from DIS. Primary use scenario is to select CCACT before starting the timer. Follow these steps to configure CCACT to a capture action while CTL.MODE is different from DIS: - Set EDGE to NONE. - Configure CCACT. - Wait for three system clock periods before setting EDGE different from NONE. These steps prevent capture events caused by expired signal values in edge-detection circuit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2cfg`]
module"]
#[doc(alias = "C2CFG")]
pub type C2cfg = crate::Reg<c2cfg::C2cfgSpec>;
#[doc = "Channel 2 Configuration This register configures channel function and enables outputs. Each channel has an edge-detection circuit. The the edge-detection circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and CTL.MODE is changed from DIS to another mode. The flush action uses two system clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit. The channel input signal enters the edge-detection circuit. False capture events can occur when: - the edge-detection circuit contains expired signal samples and the circuit is enabled without flush as described above. - the CCACT field is reconfigured while CTL.MODE is different from DIS. Primary use scenario is to select CCACT before starting the timer. Follow these steps to configure CCACT to a capture action while CTL.MODE is different from DIS: - Set EDGE to NONE. - Configure CCACT. - Wait for three system clock periods before setting EDGE different from NONE. These steps prevent capture events caused by expired signal values in edge-detection circuit."]
pub mod c2cfg;
#[doc = "PTGT (rw) register accessor: Pipeline Target A read or write to this register will clear the RIS.ZERO and RIS.TGT interrupt. If CTL.MODE != QDEC. Target value for next counter period. The timer will copy PTGT.VAL to TGT.VAL on the upcoming CNTR zero crossing only if PTGT.VAL has been written. The copy does not happen when restarting the timer. This is useful to avoid period jitter in PWM applications with time-varying period, sometimes referenced as phase corrected PWM. If CTL.MODE = QDEC The CNTR value is updated with VALUE on IDX if the counter is counting down. If the counter is counting up, CNTR is loaded with zero on IDX. In this mode the VALUE is not loaded into TGT on zero crossing.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptgt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptgt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptgt`]
module"]
#[doc(alias = "PTGT")]
pub type Ptgt = crate::Reg<ptgt::PtgtSpec>;
#[doc = "Pipeline Target A read or write to this register will clear the RIS.ZERO and RIS.TGT interrupt. If CTL.MODE != QDEC. Target value for next counter period. The timer will copy PTGT.VAL to TGT.VAL on the upcoming CNTR zero crossing only if PTGT.VAL has been written. The copy does not happen when restarting the timer. This is useful to avoid period jitter in PWM applications with time-varying period, sometimes referenced as phase corrected PWM. If CTL.MODE = QDEC The CNTR value is updated with VALUE on IDX if the counter is counting down. If the counter is counting up, CNTR is loaded with zero on IDX. In this mode the VALUE is not loaded into TGT on zero crossing."]
pub mod ptgt;
#[doc = "PC0CC (rw) register accessor: Pipeline Channel 0 Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc0cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc0cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc0cc`]
module"]
#[doc(alias = "PC0CC")]
pub type Pc0cc = crate::Reg<pc0cc::Pc0ccSpec>;
#[doc = "Pipeline Channel 0 Capture Compare"]
pub mod pc0cc;
#[doc = "PC1CC (rw) register accessor: Pipeline Channel 1 Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc1cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc1cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc1cc`]
module"]
#[doc(alias = "PC1CC")]
pub type Pc1cc = crate::Reg<pc1cc::Pc1ccSpec>;
#[doc = "Pipeline Channel 1 Capture Compare"]
pub mod pc1cc;
#[doc = "PC2CC (rw) register accessor: Pipeline Channel 2 Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc2cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc2cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc2cc`]
module"]
#[doc(alias = "PC2CC")]
pub type Pc2cc = crate::Reg<pc2cc::Pc2ccSpec>;
#[doc = "Pipeline Channel 2 Capture Compare"]
pub mod pc2cc;
#[doc = "TGT (rw) register accessor: Target User defined counter target. A read or write to this register will clear the RIS.ZERO and RIS.TGT interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tgt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tgt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tgt`]
module"]
#[doc(alias = "TGT")]
pub type Tgt = crate::Reg<tgt::TgtSpec>;
#[doc = "Target User defined counter target. A read or write to this register will clear the RIS.ZERO and RIS.TGT interrupt."]
pub mod tgt;
#[doc = "C0CC (rw) register accessor: Channel 0 Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c0cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c0cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0cc`]
module"]
#[doc(alias = "C0CC")]
pub type C0cc = crate::Reg<c0cc::C0ccSpec>;
#[doc = "Channel 0 Capture Compare"]
pub mod c0cc;
#[doc = "C1CC (rw) register accessor: Channel 1 Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1cc`]
module"]
#[doc(alias = "C1CC")]
pub type C1cc = crate::Reg<c1cc::C1ccSpec>;
#[doc = "Channel 1 Capture Compare"]
pub mod c1cc;
#[doc = "C2CC (rw) register accessor: Channel 2 Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2cc`]
module"]
#[doc(alias = "C2CC")]
pub type C2cc = crate::Reg<c2cc::C2ccSpec>;
#[doc = "Channel 2 Capture Compare"]
pub mod c2cc;
#[doc = "PTGTNC (rw) register accessor: Pipeline Target No Clear Use this register to read or write to PTGT without clearing the RIS.ZERO and RIS.TGT interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptgtnc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptgtnc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptgtnc`]
module"]
#[doc(alias = "PTGTNC")]
pub type Ptgtnc = crate::Reg<ptgtnc::PtgtncSpec>;
#[doc = "Pipeline Target No Clear Use this register to read or write to PTGT without clearing the RIS.ZERO and RIS.TGT interrupt."]
pub mod ptgtnc;
#[doc = "PC0CCNC (rw) register accessor: Pipeline Channel 0 Capture Compare No Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc0ccnc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc0ccnc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc0ccnc`]
module"]
#[doc(alias = "PC0CCNC")]
pub type Pc0ccnc = crate::Reg<pc0ccnc::Pc0ccncSpec>;
#[doc = "Pipeline Channel 0 Capture Compare No Clear"]
pub mod pc0ccnc;
#[doc = "PC1CCNC (rw) register accessor: Pipeline Channel 1 Capture Compare No Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc1ccnc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc1ccnc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc1ccnc`]
module"]
#[doc(alias = "PC1CCNC")]
pub type Pc1ccnc = crate::Reg<pc1ccnc::Pc1ccncSpec>;
#[doc = "Pipeline Channel 1 Capture Compare No Clear"]
pub mod pc1ccnc;
#[doc = "PC2CCNC (rw) register accessor: Pipeline Channel 2 Capture Compare No Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc2ccnc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc2ccnc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc2ccnc`]
module"]
#[doc(alias = "PC2CCNC")]
pub type Pc2ccnc = crate::Reg<pc2ccnc::Pc2ccncSpec>;
#[doc = "Pipeline Channel 2 Capture Compare No Clear"]
pub mod pc2ccnc;
#[doc = "TGTNC (rw) register accessor: Target No Clear Use this register to read or write to TGT without clearing the RIS.ZERO and RIS.TGT interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tgtnc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tgtnc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tgtnc`]
module"]
#[doc(alias = "TGTNC")]
pub type Tgtnc = crate::Reg<tgtnc::TgtncSpec>;
#[doc = "Target No Clear Use this register to read or write to TGT without clearing the RIS.ZERO and RIS.TGT interrupt."]
pub mod tgtnc;
#[doc = "C0CCNC (rw) register accessor: Channel 0 Capture Compare No Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c0ccnc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c0ccnc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0ccnc`]
module"]
#[doc(alias = "C0CCNC")]
pub type C0ccnc = crate::Reg<c0ccnc::C0ccncSpec>;
#[doc = "Channel 0 Capture Compare No Clear"]
pub mod c0ccnc;
#[doc = "C1CCNC (rw) register accessor: Channel 1 Capture Compare No Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1ccnc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1ccnc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1ccnc`]
module"]
#[doc(alias = "C1CCNC")]
pub type C1ccnc = crate::Reg<c1ccnc::C1ccncSpec>;
#[doc = "Channel 1 Capture Compare No Clear"]
pub mod c1ccnc;
#[doc = "C2CCNC (rw) register accessor: Channel 2 Capture Compare No Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ccnc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ccnc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2ccnc`]
module"]
#[doc(alias = "C2CCNC")]
pub type C2ccnc = crate::Reg<c2ccnc::C2ccncSpec>;
#[doc = "Channel 2 Capture Compare No Clear"]
pub mod c2ccnc;
