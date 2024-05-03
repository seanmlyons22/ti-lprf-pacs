#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    cyccnt: Cyccnt,
    cpicnt: Cpicnt,
    exccnt: Exccnt,
    sleepcnt: Sleepcnt,
    lsucnt: Lsucnt,
    foldcnt: Foldcnt,
    pcsr: Pcsr,
    comp0: Comp0,
    _reserved9: [u8; 0x04],
    function0: Function0,
    _reserved10: [u8; 0x04],
    comp1: Comp1,
    _reserved11: [u8; 0x04],
    function1: Function1,
    _reserved12: [u8; 0x04],
    comp2: Comp2,
    _reserved13: [u8; 0x04],
    function2: Function2,
    _reserved14: [u8; 0x04],
    comp3: Comp3,
    _reserved15: [u8; 0x04],
    function3: Function3,
    _reserved16: [u8; 0x0f60],
    devarch: Devarch,
    _reserved17: [u8; 0x0c],
    devtype: Devtype,
    pidr4: Pidr4,
    pidr5: Pidr5,
    pidr6: Pidr6,
    pidr7: Pidr7,
    pidr0: Pidr0,
    pidr1: Pidr1,
    pidr2: Pidr2,
    pidr3: Pidr3,
    cidr0: Cidr0,
    cidr1: Cidr1,
    cidr2: Cidr2,
    cidr3: Cidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Provides configuration and status information for the DWT unit, and used to control features of the unit"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Shows or sets the value of the processor cycle counter, CYCCNT"]
    #[inline(always)]
    pub const fn cyccnt(&self) -> &Cyccnt {
        &self.cyccnt
    }
    #[doc = "0x08 - CPI Count Register"]
    #[inline(always)]
    pub const fn cpicnt(&self) -> &Cpicnt {
        &self.cpicnt
    }
    #[doc = "0x0c - Counts the total cycles spent in exception processing"]
    #[inline(always)]
    pub const fn exccnt(&self) -> &Exccnt {
        &self.exccnt
    }
    #[doc = "0x10 - Sleep Count Register"]
    #[inline(always)]
    pub const fn sleepcnt(&self) -> &Sleepcnt {
        &self.sleepcnt
    }
    #[doc = "0x14 - Increments on the additional cycles required to execute all load or store instructions"]
    #[inline(always)]
    pub const fn lsucnt(&self) -> &Lsucnt {
        &self.lsucnt
    }
    #[doc = "0x18 - Increments on the additional cycles required to execute all load or store instructions"]
    #[inline(always)]
    pub const fn foldcnt(&self) -> &Foldcnt {
        &self.foldcnt
    }
    #[doc = "0x1c - Program Counter Sample Register"]
    #[inline(always)]
    pub const fn pcsr(&self) -> &Pcsr {
        &self.pcsr
    }
    #[doc = "0x20 - Provides a reference value for use by watchpoint comparator 0"]
    #[inline(always)]
    pub const fn comp0(&self) -> &Comp0 {
        &self.comp0
    }
    #[doc = "0x28 - Controls the operation of watchpoint comparator 0"]
    #[inline(always)]
    pub const fn function0(&self) -> &Function0 {
        &self.function0
    }
    #[doc = "0x30 - Provides a reference value for use by watchpoint comparator 1"]
    #[inline(always)]
    pub const fn comp1(&self) -> &Comp1 {
        &self.comp1
    }
    #[doc = "0x38 - Controls the operation of watchpoint comparator 1"]
    #[inline(always)]
    pub const fn function1(&self) -> &Function1 {
        &self.function1
    }
    #[doc = "0x40 - Provides a reference value for use by watchpoint comparator 2"]
    #[inline(always)]
    pub const fn comp2(&self) -> &Comp2 {
        &self.comp2
    }
    #[doc = "0x48 - Controls the operation of watchpoint comparator 2"]
    #[inline(always)]
    pub const fn function2(&self) -> &Function2 {
        &self.function2
    }
    #[doc = "0x50 - Provides a reference value for use by watchpoint comparator 3"]
    #[inline(always)]
    pub const fn comp3(&self) -> &Comp3 {
        &self.comp3
    }
    #[doc = "0x58 - Controls the operation of watchpoint comparator 3"]
    #[inline(always)]
    pub const fn function3(&self) -> &Function3 {
        &self.function3
    }
    #[doc = "0xfbc - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn devarch(&self) -> &Devarch {
        &self.devarch
    }
    #[doc = "0xfcc - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn devtype(&self) -> &Devtype {
        &self.devtype
    }
    #[doc = "0xfd0 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn pidr4(&self) -> &Pidr4 {
        &self.pidr4
    }
    #[doc = "0xfd4 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn pidr5(&self) -> &Pidr5 {
        &self.pidr5
    }
    #[doc = "0xfd8 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn pidr6(&self) -> &Pidr6 {
        &self.pidr6
    }
    #[doc = "0xfdc - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn pidr7(&self) -> &Pidr7 {
        &self.pidr7
    }
    #[doc = "0xfe0 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn pidr0(&self) -> &Pidr0 {
        &self.pidr0
    }
    #[doc = "0xfe4 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn pidr1(&self) -> &Pidr1 {
        &self.pidr1
    }
    #[doc = "0xfe8 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn pidr2(&self) -> &Pidr2 {
        &self.pidr2
    }
    #[doc = "0xfec - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn pidr3(&self) -> &Pidr3 {
        &self.pidr3
    }
    #[doc = "0xff0 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn cidr0(&self) -> &Cidr0 {
        &self.cidr0
    }
    #[doc = "0xff4 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn cidr1(&self) -> &Cidr1 {
        &self.cidr1
    }
    #[doc = "0xff8 - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn cidr2(&self) -> &Cidr2 {
        &self.cidr2
    }
    #[doc = "0xffc - Provides CoreSight discovery information for the DWT"]
    #[inline(always)]
    pub const fn cidr3(&self) -> &Cidr3 {
        &self.cidr3
    }
}
#[doc = "CTRL (rw) register accessor: Provides configuration and status information for the DWT unit, and used to control features of the unit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Provides configuration and status information for the DWT unit, and used to control features of the unit"]
pub mod ctrl;
#[doc = "CYCCNT (rw) register accessor: Shows or sets the value of the processor cycle counter, CYCCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cyccnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cyccnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cyccnt`]
module"]
#[doc(alias = "CYCCNT")]
pub type Cyccnt = crate::Reg<cyccnt::CyccntSpec>;
#[doc = "Shows or sets the value of the processor cycle counter, CYCCNT"]
pub mod cyccnt;
#[doc = "CPICNT (rw) register accessor: CPI Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpicnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpicnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpicnt`]
module"]
#[doc(alias = "CPICNT")]
pub type Cpicnt = crate::Reg<cpicnt::CpicntSpec>;
#[doc = "CPI Count Register"]
pub mod cpicnt;
#[doc = "EXCCNT (rw) register accessor: Counts the total cycles spent in exception processing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exccnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exccnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exccnt`]
module"]
#[doc(alias = "EXCCNT")]
pub type Exccnt = crate::Reg<exccnt::ExccntSpec>;
#[doc = "Counts the total cycles spent in exception processing"]
pub mod exccnt;
#[doc = "SLEEPCNT (rw) register accessor: Sleep Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleepcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleepcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleepcnt`]
module"]
#[doc(alias = "SLEEPCNT")]
pub type Sleepcnt = crate::Reg<sleepcnt::SleepcntSpec>;
#[doc = "Sleep Count Register"]
pub mod sleepcnt;
#[doc = "LSUCNT (rw) register accessor: Increments on the additional cycles required to execute all load or store instructions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsucnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsucnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsucnt`]
module"]
#[doc(alias = "LSUCNT")]
pub type Lsucnt = crate::Reg<lsucnt::LsucntSpec>;
#[doc = "Increments on the additional cycles required to execute all load or store instructions"]
pub mod lsucnt;
#[doc = "FOLDCNT (rw) register accessor: Increments on the additional cycles required to execute all load or store instructions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`foldcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`foldcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@foldcnt`]
module"]
#[doc(alias = "FOLDCNT")]
pub type Foldcnt = crate::Reg<foldcnt::FoldcntSpec>;
#[doc = "Increments on the additional cycles required to execute all load or store instructions"]
pub mod foldcnt;
#[doc = "PCSR (rw) register accessor: Program Counter Sample Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcsr`]
module"]
#[doc(alias = "PCSR")]
pub type Pcsr = crate::Reg<pcsr::PcsrSpec>;
#[doc = "Program Counter Sample Register"]
pub mod pcsr;
#[doc = "COMP0 (rw) register accessor: Provides a reference value for use by watchpoint comparator 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0`]
module"]
#[doc(alias = "COMP0")]
pub type Comp0 = crate::Reg<comp0::Comp0Spec>;
#[doc = "Provides a reference value for use by watchpoint comparator 0"]
pub mod comp0;
#[doc = "FUNCTION0 (rw) register accessor: Controls the operation of watchpoint comparator 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`function0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`function0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@function0`]
module"]
#[doc(alias = "FUNCTION0")]
pub type Function0 = crate::Reg<function0::Function0Spec>;
#[doc = "Controls the operation of watchpoint comparator 0"]
pub mod function0;
#[doc = "COMP1 (rw) register accessor: Provides a reference value for use by watchpoint comparator 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1`]
module"]
#[doc(alias = "COMP1")]
pub type Comp1 = crate::Reg<comp1::Comp1Spec>;
#[doc = "Provides a reference value for use by watchpoint comparator 1"]
pub mod comp1;
#[doc = "FUNCTION1 (rw) register accessor: Controls the operation of watchpoint comparator 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`function1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`function1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@function1`]
module"]
#[doc(alias = "FUNCTION1")]
pub type Function1 = crate::Reg<function1::Function1Spec>;
#[doc = "Controls the operation of watchpoint comparator 1"]
pub mod function1;
#[doc = "COMP2 (rw) register accessor: Provides a reference value for use by watchpoint comparator 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2`]
module"]
#[doc(alias = "COMP2")]
pub type Comp2 = crate::Reg<comp2::Comp2Spec>;
#[doc = "Provides a reference value for use by watchpoint comparator 2"]
pub mod comp2;
#[doc = "FUNCTION2 (rw) register accessor: Controls the operation of watchpoint comparator 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`function2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`function2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@function2`]
module"]
#[doc(alias = "FUNCTION2")]
pub type Function2 = crate::Reg<function2::Function2Spec>;
#[doc = "Controls the operation of watchpoint comparator 2"]
pub mod function2;
#[doc = "COMP3 (rw) register accessor: Provides a reference value for use by watchpoint comparator 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp3`]
module"]
#[doc(alias = "COMP3")]
pub type Comp3 = crate::Reg<comp3::Comp3Spec>;
#[doc = "Provides a reference value for use by watchpoint comparator 3"]
pub mod comp3;
#[doc = "FUNCTION3 (rw) register accessor: Controls the operation of watchpoint comparator 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`function3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`function3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@function3`]
module"]
#[doc(alias = "FUNCTION3")]
pub type Function3 = crate::Reg<function3::Function3Spec>;
#[doc = "Controls the operation of watchpoint comparator 3"]
pub mod function3;
#[doc = "DEVARCH (rw) register accessor: Provides CoreSight discovery information for the DWT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devarch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devarch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devarch`]
module"]
#[doc(alias = "DEVARCH")]
pub type Devarch = crate::Reg<devarch::DevarchSpec>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod devarch;
#[doc = "DEVTYPE (rw) register accessor: Provides CoreSight discovery information for the DWT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devtype`]
module"]
#[doc(alias = "DEVTYPE")]
pub type Devtype = crate::Reg<devtype::DevtypeSpec>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod devtype;
#[doc = "PIDR4 (rw) register accessor: Provides CoreSight discovery information for the DWT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr4`]
module"]
#[doc(alias = "PIDR4")]
pub type Pidr4 = crate::Reg<pidr4::Pidr4Spec>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr4;
#[doc = "PIDR5 (rw) register accessor: Provides CoreSight discovery information for the DWT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr5`]
module"]
#[doc(alias = "PIDR5")]
pub type Pidr5 = crate::Reg<pidr5::Pidr5Spec>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr5;
#[doc = "PIDR6 (rw) register accessor: Provides CoreSight discovery information for the DWT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr6`]
module"]
#[doc(alias = "PIDR6")]
pub type Pidr6 = crate::Reg<pidr6::Pidr6Spec>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr6;
#[doc = "PIDR7 (rw) register accessor: Provides CoreSight discovery information for the DWT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr7`]
module"]
#[doc(alias = "PIDR7")]
pub type Pidr7 = crate::Reg<pidr7::Pidr7Spec>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr7;
#[doc = "PIDR0 (rw) register accessor: Provides CoreSight discovery information for the DWT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr0`]
module"]
#[doc(alias = "PIDR0")]
pub type Pidr0 = crate::Reg<pidr0::Pidr0Spec>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr0;
#[doc = "PIDR1 (rw) register accessor: Provides CoreSight discovery information for the DWT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr1`]
module"]
#[doc(alias = "PIDR1")]
pub type Pidr1 = crate::Reg<pidr1::Pidr1Spec>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr1;
#[doc = "PIDR2 (rw) register accessor: Provides CoreSight discovery information for the DWT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr2`]
module"]
#[doc(alias = "PIDR2")]
pub type Pidr2 = crate::Reg<pidr2::Pidr2Spec>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr2;
#[doc = "PIDR3 (rw) register accessor: Provides CoreSight discovery information for the DWT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr3`]
module"]
#[doc(alias = "PIDR3")]
pub type Pidr3 = crate::Reg<pidr3::Pidr3Spec>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod pidr3;
#[doc = "CIDR0 (rw) register accessor: Provides CoreSight discovery information for the DWT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr0`]
module"]
#[doc(alias = "CIDR0")]
pub type Cidr0 = crate::Reg<cidr0::Cidr0Spec>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod cidr0;
#[doc = "CIDR1 (rw) register accessor: Provides CoreSight discovery information for the DWT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr1`]
module"]
#[doc(alias = "CIDR1")]
pub type Cidr1 = crate::Reg<cidr1::Cidr1Spec>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod cidr1;
#[doc = "CIDR2 (rw) register accessor: Provides CoreSight discovery information for the DWT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr2`]
module"]
#[doc(alias = "CIDR2")]
pub type Cidr2 = crate::Reg<cidr2::Cidr2Spec>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod cidr2;
#[doc = "CIDR3 (rw) register accessor: Provides CoreSight discovery information for the DWT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr3`]
module"]
#[doc(alias = "CIDR3")]
pub type Cidr3 = crate::Reg<cidr3::Cidr3Spec>;
#[doc = "Provides CoreSight discovery information for the DWT"]
pub mod cidr3;
