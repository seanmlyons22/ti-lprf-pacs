#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x28],
    imask: Imask,
    _reserved1: [u8; 0x04],
    ris: Ris,
    _reserved2: [u8; 0x04],
    mis: Mis,
    _reserved3: [u8; 0x04],
    iset: Iset,
    _reserved4: [u8; 0x04],
    iclr: Iclr,
    _reserved5: [u8; 0xb0],
    desc: Desc,
    cmdexec: Cmdexec,
    cmdtype: Cmdtype,
    cmdctl: Cmdctl,
    _reserved9: [u8; 0x14],
    cmdaddr: Cmdaddr,
    cmdbyten: Cmdbyten,
    _reserved11: [u8; 0x08],
    cmddata0: Cmddata0,
    cmddata1: Cmddata1,
    cmddata2: Cmddata2,
    cmddata3: Cmddata3,
    _reserved15: [u8; 0x90],
    cmdweprota: Cmdweprota,
    cmdweprotb: Cmdweprotb,
    _reserved17: [u8; 0x38],
    cmdweprotnm: Cmdweprotnm,
    cmdweprottr: Cmdweprottr,
    cmdweproten: Cmdweproten,
    _reserved20: [u8; 0x0194],
    cfgcmd: Cfgcmd,
    cfgpcnt: Cfgpcnt,
    _reserved22: [u8; 0x18],
    statcmd: Statcmd,
    stataddr: Stataddr,
    statpcnt: Statpcnt,
    statmode: Statmode,
    _reserved26: [u8; 0x10],
    gblinfo0: Gblinfo0,
    gblinfo1: Gblinfo1,
    gblinfo2: Gblinfo2,
    _reserved29: [u8; 0x04],
    bank0info0: Bank0info0,
    bank0info1: Bank0info1,
}
impl RegisterBlock {
    #[doc = "0x28 - Interrupt Mask Register: The IMASK register holds the current interrupt mask settings. Masked interrupts are read in the MIS register. PSD compliant register."]
    #[inline(always)]
    pub const fn imask(&self) -> &Imask {
        &self.imask
    }
    #[doc = "0x30 - Raw Interrupt Status Register: The RIS register reflects all pending interrupts, regardless of masking. The RIS register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing a 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled. A flag can be set by software by writing a 1 to the ISET register. Reading the IIDX register will also clear the corresponding bit in RIS. PSD compliant register."]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x38 - Masked Interrupt Status Register: The MIS register is a bit-wise AND of the contents of the IMASK and RIS registers. This is kept mainly for ARM compatibility, and has limited use since the highest priority interrupt index is returned via the IIDX register. PSD compliant register."]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x40 - Interrupt Set Register: The ISET register allows software to write a 1 to set corresponding interrupt. Safety: This meets a safety requirement to allow software diagnostics to trigger interrupts. PSD compliant register."]
    #[inline(always)]
    pub const fn iset(&self) -> &Iset {
        &self.iset
    }
    #[doc = "0x48 - Interrupt Clear Register. The ICLR register allows allows software to write a 1 to clear corresponding interrupt. PSD compliant register."]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
    }
    #[doc = "0xfc - Hardware Version Description Register: This register identifies the flash wrapper hardware version and feature set used."]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x100 - Command Execute Register: Initiates execution of the command specified in the CMDTYPE register. This register is blocked for writes after being written to 1 and prior to STATCMD.DONE being set by the flash wrapper hardware. flash wrapper hardware clears this register after the processing of the command has completed."]
    #[inline(always)]
    pub const fn cmdexec(&self) -> &Cmdexec {
        &self.cmdexec
    }
    #[doc = "0x104 - Command Type Register This register specifies the type of command to be executed by the flash wrapper hardware. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
    #[inline(always)]
    pub const fn cmdtype(&self) -> &Cmdtype {
        &self.cmdtype
    }
    #[doc = "0x108 - Command Control Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
    #[inline(always)]
    pub const fn cmdctl(&self) -> &Cmdctl {
        &self.cmdctl
    }
    #[doc = "0x120 - Command Address Register: This register forms the target address of a command. The use cases are as follows: 1) For single-word program, this address indicates the flash bank word to be programmed. 2) For multi-word program, this address indicates the first flash bank address for the program. The address will be incremented for further words. 3) For sector erase, this address indicates the sector to be erased. 4) For bank erase, the address indicates the bank to be erased. 5) For read verify, the address indications follow program/erase listed above. Note the address written to this register will be submitted for translation to the flash wrapper address translation interface, and the translated address will be used to access the bank. However, if the CMDCTL.ADDRXLATEOVR bit is set, then the address written to this register will be used directly as the bank address. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware."]
    #[inline(always)]
    pub const fn cmdaddr(&self) -> &Cmdaddr {
        &self.cmdaddr
    }
    #[doc = "0x124 - Command Program Byte Enable Register: This register forms a per-byte enable for programming data. For data bytes to be programmed, a 1 must be written to the corresponding bit in this register. Normally, all bits are written to 1, allowing program of full flash words. However, leaving some bits 0 allows programming of 8-bit, 16-bit, 32-bit or 64-bit portions of a flash word. In addtion, the read verify command will ignore data bytes read from the flash in its comparison if the corresponding CMDBYTEN bit is 0. ECC data bytes are protected by the 1-2 MSB bits in this register, depending on the presence of ECC and the flash word data width. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. This register is written to all 0 after the completion of all flash wrapper commands."]
    #[inline(always)]
    pub const fn cmdbyten(&self) -> &Cmdbyten {
        &self.cmdbyten
    }
    #[doc = "0x130 - Command Data Register 0 This register forms the data for a command. For DATAWIDTH == 128: This register represents bits 31:0 of flash word data register 0. For DATAWIDTH == 64: This register represents bits 31:0 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all flash wrapper commands. Use cases for the CMDDATA* registers are as follows: 1) Program - These registers contain the data to be programmed. 2) Erase - These registers are not used. 3) Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata0(&self) -> &Cmddata0 {
        &self.cmddata0
    }
    #[doc = "0x134 - Command Data Register 1 This register forms the data for a command. For DATAWIDTH == 128: This register represents bits 63:32 of flash word data register 0. For DATAWIDTH == 64: This register represents bits 63:32 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to CMDSTAT.DONE being set by the flash wrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all flash wrapper commands. Use cases for the CMDDATA* registers are as follows: 1) Program - These registers contain the data to be programmed. 2) Erase - These registers are not used. 3) Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata1(&self) -> &Cmddata1 {
        &self.cmddata1
    }
    #[doc = "0x138 - Command Data Register 2 This register forms the data for a command. For DATAWIDTH == 128: This register represents bits 95:64 of flash word data register 0. For DATAWIDTH == 64: This register represents bits 31:0 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all flash wrapper commands. Use cases for the CMDDATA* registers are as follows: 1) Program - These registers contain the data to be programmed. 2) Erase - These registers are not used. 3) Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata2(&self) -> &Cmddata2 {
        &self.cmddata2
    }
    #[doc = "0x13c - Command Data Register 3 This register forms the data for a command. For DATAWIDTH == 128: This register represents bits 127:96 of flash word data register 0. For DATAWIDTH == 64: This register represents bits 63:32 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all flash wrapper commands. Use cases for the CMDDATA* registers are as follows: 1) Program - These registers contain the data to be programmed. 2) Erase - These registers are not used. 3) Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata3(&self) -> &Cmddata3 {
        &self.cmddata3
    }
    #[doc = "0x1d0 - Command WriteErase Protect A Register This register allows the first 32 sectors of the main region to be protected from program or erase, with 1 bit protecting each sector. If the main region size is smaller than 32 sectors, then this register provides protection for the whole region. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands."]
    #[inline(always)]
    pub const fn cmdweprota(&self) -> &Cmdweprota {
        &self.cmdweprota
    }
    #[doc = "0x1d4 - Command WriteErase Protect B Register This register allows main region sectors to be protected from program and erase. Each bit corresponds to a group of 8 sectors. There are 3 cases for how these protect bits are applied: 1. Single-bank system: In the case where only a single flash bank is present, the first 32 sectors are protected via the CMDWEPROTA register. Thus, the protection give by the bits in CMDWEPROTB begin with sector 32. 2. Multi-bank system, Bank 0: When multiple flash banks are present, the first 32 sectors of bank 0 are protected via the CMDWEPROTA register. Thus, only bits 4 and above of CMDWEPROTB would be applicable to bank 0. The protection of bit 4 and above would begin at sector 32. Bits 3:0 of WEPROTB are ignored for bank 0. 3. Multi-bank system, Banks 1-N: For banks other than bank 0 in a multi-bank system, CMDWEPROTA has no effect, so the bits in CMDWEPROTB will protect these banks starting from sector 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands."]
    #[inline(always)]
    pub const fn cmdweprotb(&self) -> &Cmdweprotb {
        &self.cmdweprotb
    }
    #[doc = "0x210 - Command WriteErase Protect Non-Main Register This register allows non-main region region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands."]
    #[inline(always)]
    pub const fn cmdweprotnm(&self) -> &Cmdweprotnm {
        &self.cmdweprotnm
    }
    #[doc = "0x214 - Command WriteErase Protect Trim Register This register allows trim region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands."]
    #[inline(always)]
    pub const fn cmdweprottr(&self) -> &Cmdweprottr {
        &self.cmdweprottr
    }
    #[doc = "0x218 - Command WriteErase Protect Engr Register This register allows engr region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands."]
    #[inline(always)]
    pub const fn cmdweproten(&self) -> &Cmdweproten {
        &self.cmdweproten
    }
    #[doc = "0x3b0 - Command Configuration Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
    #[inline(always)]
    pub const fn cfgcmd(&self) -> &Cfgcmd {
        &self.cfgcmd
    }
    #[doc = "0x3b4 - Pulse Counter Configuration Register This register allows further configuration of maximum pulse counts for program and erase operations. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware."]
    #[inline(always)]
    pub const fn cfgpcnt(&self) -> &Cfgpcnt {
        &self.cfgpcnt
    }
    #[doc = "0x3d0 - Command Status Register This register contains status regarding completion and errors of command execution."]
    #[inline(always)]
    pub const fn statcmd(&self) -> &Statcmd {
        &self.statcmd
    }
    #[doc = "0x3d4 - Current Address Counter Value Read only register giving read access to the state machine current address. A bank id, region id and address are stored in this register and are incremented as necessary during execution of a command."]
    #[inline(always)]
    pub const fn stataddr(&self) -> &Stataddr {
        &self.stataddr
    }
    #[doc = "0x3d8 - Current Pulse Count Register: Read only register giving read access to the state machine current pulse count value for program/erase operations."]
    #[inline(always)]
    pub const fn statpcnt(&self) -> &Statpcnt {
        &self.statpcnt
    }
    #[doc = "0x3dc - Mode Status Register Indicates any banks which not in READ mode, and it indicates the mode which the bank(s) are in."]
    #[inline(always)]
    pub const fn statmode(&self) -> &Statmode {
        &self.statmode
    }
    #[doc = "0x3f0 - Global Info 0 Register Read only register detailing information about sector size and number of banks present."]
    #[inline(always)]
    pub const fn gblinfo0(&self) -> &Gblinfo0 {
        &self.gblinfo0
    }
    #[doc = "0x3f4 - Global Info 1 Register Read only register detailing information about data, ecc and redundant data widths in bits."]
    #[inline(always)]
    pub const fn gblinfo1(&self) -> &Gblinfo1 {
        &self.gblinfo1
    }
    #[doc = "0x3f8 - Global Info 2 Register Read only register detailing information about the number of data registers present."]
    #[inline(always)]
    pub const fn gblinfo2(&self) -> &Gblinfo2 {
        &self.gblinfo2
    }
    #[doc = "0x400 - Bank Info 0 Register for bank 0. Read only register detailing information about Main region size in the bank."]
    #[inline(always)]
    pub const fn bank0info0(&self) -> &Bank0info0 {
        &self.bank0info0
    }
    #[doc = "0x404 - Bank Info1 Register for bank 0. Read only register detailing information about Non-Main, Trim, and Engr region sizes in the bank."]
    #[inline(always)]
    pub const fn bank0info1(&self) -> &Bank0info1 {
        &self.bank0info1
    }
}
#[doc = "IMASK (rw) register accessor: Interrupt Mask Register: The IMASK register holds the current interrupt mask settings. Masked interrupts are read in the MIS register. PSD compliant register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`]
module"]
#[doc(alias = "IMASK")]
pub type Imask = crate::Reg<imask::ImaskSpec>;
#[doc = "Interrupt Mask Register: The IMASK register holds the current interrupt mask settings. Masked interrupts are read in the MIS register. PSD compliant register."]
pub mod imask;
#[doc = "RIS (rw) register accessor: Raw Interrupt Status Register: The RIS register reflects all pending interrupts, regardless of masking. The RIS register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing a 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled. A flag can be set by software by writing a 1 to the ISET register. Reading the IIDX register will also clear the corresponding bit in RIS. PSD compliant register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw Interrupt Status Register: The RIS register reflects all pending interrupts, regardless of masking. The RIS register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing a 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled. A flag can be set by software by writing a 1 to the ISET register. Reading the IIDX register will also clear the corresponding bit in RIS. PSD compliant register."]
pub mod ris;
#[doc = "MIS (rw) register accessor: Masked Interrupt Status Register: The MIS register is a bit-wise AND of the contents of the IMASK and RIS registers. This is kept mainly for ARM compatibility, and has limited use since the highest priority interrupt index is returned via the IIDX register. PSD compliant register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked Interrupt Status Register: The MIS register is a bit-wise AND of the contents of the IMASK and RIS registers. This is kept mainly for ARM compatibility, and has limited use since the highest priority interrupt index is returned via the IIDX register. PSD compliant register."]
pub mod mis;
#[doc = "ISET (rw) register accessor: Interrupt Set Register: The ISET register allows software to write a 1 to set corresponding interrupt. Safety: This meets a safety requirement to allow software diagnostics to trigger interrupts. PSD compliant register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset`]
module"]
#[doc(alias = "ISET")]
pub type Iset = crate::Reg<iset::IsetSpec>;
#[doc = "Interrupt Set Register: The ISET register allows software to write a 1 to set corresponding interrupt. Safety: This meets a safety requirement to allow software diagnostics to trigger interrupts. PSD compliant register."]
pub mod iset;
#[doc = "ICLR (rw) register accessor: Interrupt Clear Register. The ICLR register allows allows software to write a 1 to clear corresponding interrupt. PSD compliant register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`]
module"]
#[doc(alias = "ICLR")]
pub type Iclr = crate::Reg<iclr::IclrSpec>;
#[doc = "Interrupt Clear Register. The ICLR register allows allows software to write a 1 to clear corresponding interrupt. PSD compliant register."]
pub mod iclr;
#[doc = "DESC (rw) register accessor: Hardware Version Description Register: This register identifies the flash wrapper hardware version and feature set used.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Hardware Version Description Register: This register identifies the flash wrapper hardware version and feature set used."]
pub mod desc;
#[doc = "CMDEXEC (rw) register accessor: Command Execute Register: Initiates execution of the command specified in the CMDTYPE register. This register is blocked for writes after being written to 1 and prior to STATCMD.DONE being set by the flash wrapper hardware. flash wrapper hardware clears this register after the processing of the command has completed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdexec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdexec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdexec`]
module"]
#[doc(alias = "CMDEXEC")]
pub type Cmdexec = crate::Reg<cmdexec::CmdexecSpec>;
#[doc = "Command Execute Register: Initiates execution of the command specified in the CMDTYPE register. This register is blocked for writes after being written to 1 and prior to STATCMD.DONE being set by the flash wrapper hardware. flash wrapper hardware clears this register after the processing of the command has completed."]
pub mod cmdexec;
#[doc = "CMDTYPE (rw) register accessor: Command Type Register This register specifies the type of command to be executed by the flash wrapper hardware. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdtype`]
module"]
#[doc(alias = "CMDTYPE")]
pub type Cmdtype = crate::Reg<cmdtype::CmdtypeSpec>;
#[doc = "Command Type Register This register specifies the type of command to be executed by the flash wrapper hardware. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
pub mod cmdtype;
#[doc = "CMDCTL (rw) register accessor: Command Control Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdctl`]
module"]
#[doc(alias = "CMDCTL")]
pub type Cmdctl = crate::Reg<cmdctl::CmdctlSpec>;
#[doc = "Command Control Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
pub mod cmdctl;
#[doc = "CMDADDR (rw) register accessor: Command Address Register: This register forms the target address of a command. The use cases are as follows: 1) For single-word program, this address indicates the flash bank word to be programmed. 2) For multi-word program, this address indicates the first flash bank address for the program. The address will be incremented for further words. 3) For sector erase, this address indicates the sector to be erased. 4) For bank erase, the address indicates the bank to be erased. 5) For read verify, the address indications follow program/erase listed above. Note the address written to this register will be submitted for translation to the flash wrapper address translation interface, and the translated address will be used to access the bank. However, if the CMDCTL.ADDRXLATEOVR bit is set, then the address written to this register will be used directly as the bank address. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdaddr`]
module"]
#[doc(alias = "CMDADDR")]
pub type Cmdaddr = crate::Reg<cmdaddr::CmdaddrSpec>;
#[doc = "Command Address Register: This register forms the target address of a command. The use cases are as follows: 1) For single-word program, this address indicates the flash bank word to be programmed. 2) For multi-word program, this address indicates the first flash bank address for the program. The address will be incremented for further words. 3) For sector erase, this address indicates the sector to be erased. 4) For bank erase, the address indicates the bank to be erased. 5) For read verify, the address indications follow program/erase listed above. Note the address written to this register will be submitted for translation to the flash wrapper address translation interface, and the translated address will be used to access the bank. However, if the CMDCTL.ADDRXLATEOVR bit is set, then the address written to this register will be used directly as the bank address. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware."]
pub mod cmdaddr;
#[doc = "CMDBYTEN (rw) register accessor: Command Program Byte Enable Register: This register forms a per-byte enable for programming data. For data bytes to be programmed, a 1 must be written to the corresponding bit in this register. Normally, all bits are written to 1, allowing program of full flash words. However, leaving some bits 0 allows programming of 8-bit, 16-bit, 32-bit or 64-bit portions of a flash word. In addtion, the read verify command will ignore data bytes read from the flash in its comparison if the corresponding CMDBYTEN bit is 0. ECC data bytes are protected by the 1-2 MSB bits in this register, depending on the presence of ECC and the flash word data width. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. This register is written to all 0 after the completion of all flash wrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdbyten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdbyten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdbyten`]
module"]
#[doc(alias = "CMDBYTEN")]
pub type Cmdbyten = crate::Reg<cmdbyten::CmdbytenSpec>;
#[doc = "Command Program Byte Enable Register: This register forms a per-byte enable for programming data. For data bytes to be programmed, a 1 must be written to the corresponding bit in this register. Normally, all bits are written to 1, allowing program of full flash words. However, leaving some bits 0 allows programming of 8-bit, 16-bit, 32-bit or 64-bit portions of a flash word. In addtion, the read verify command will ignore data bytes read from the flash in its comparison if the corresponding CMDBYTEN bit is 0. ECC data bytes are protected by the 1-2 MSB bits in this register, depending on the presence of ECC and the flash word data width. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. This register is written to all 0 after the completion of all flash wrapper commands."]
pub mod cmdbyten;
#[doc = "CMDDATA0 (rw) register accessor: Command Data Register 0 This register forms the data for a command. For DATAWIDTH == 128: This register represents bits 31:0 of flash word data register 0. For DATAWIDTH == 64: This register represents bits 31:0 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all flash wrapper commands. Use cases for the CMDDATA* registers are as follows: 1) Program - These registers contain the data to be programmed. 2) Erase - These registers are not used. 3) Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata0`]
module"]
#[doc(alias = "CMDDATA0")]
pub type Cmddata0 = crate::Reg<cmddata0::Cmddata0Spec>;
#[doc = "Command Data Register 0 This register forms the data for a command. For DATAWIDTH == 128: This register represents bits 31:0 of flash word data register 0. For DATAWIDTH == 64: This register represents bits 31:0 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all flash wrapper commands. Use cases for the CMDDATA* registers are as follows: 1) Program - These registers contain the data to be programmed. 2) Erase - These registers are not used. 3) Read Verify - These registers contain data to be verified."]
pub mod cmddata0;
#[doc = "CMDDATA1 (rw) register accessor: Command Data Register 1 This register forms the data for a command. For DATAWIDTH == 128: This register represents bits 63:32 of flash word data register 0. For DATAWIDTH == 64: This register represents bits 63:32 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to CMDSTAT.DONE being set by the flash wrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all flash wrapper commands. Use cases for the CMDDATA* registers are as follows: 1) Program - These registers contain the data to be programmed. 2) Erase - These registers are not used. 3) Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata1`]
module"]
#[doc(alias = "CMDDATA1")]
pub type Cmddata1 = crate::Reg<cmddata1::Cmddata1Spec>;
#[doc = "Command Data Register 1 This register forms the data for a command. For DATAWIDTH == 128: This register represents bits 63:32 of flash word data register 0. For DATAWIDTH == 64: This register represents bits 63:32 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to CMDSTAT.DONE being set by the flash wrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all flash wrapper commands. Use cases for the CMDDATA* registers are as follows: 1) Program - These registers contain the data to be programmed. 2) Erase - These registers are not used. 3) Read Verify - These registers contain data to be verified."]
pub mod cmddata1;
#[doc = "CMDDATA2 (rw) register accessor: Command Data Register 2 This register forms the data for a command. For DATAWIDTH == 128: This register represents bits 95:64 of flash word data register 0. For DATAWIDTH == 64: This register represents bits 31:0 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all flash wrapper commands. Use cases for the CMDDATA* registers are as follows: 1) Program - These registers contain the data to be programmed. 2) Erase - These registers are not used. 3) Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata2`]
module"]
#[doc(alias = "CMDDATA2")]
pub type Cmddata2 = crate::Reg<cmddata2::Cmddata2Spec>;
#[doc = "Command Data Register 2 This register forms the data for a command. For DATAWIDTH == 128: This register represents bits 95:64 of flash word data register 0. For DATAWIDTH == 64: This register represents bits 31:0 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all flash wrapper commands. Use cases for the CMDDATA* registers are as follows: 1) Program - These registers contain the data to be programmed. 2) Erase - These registers are not used. 3) Read Verify - These registers contain data to be verified."]
pub mod cmddata2;
#[doc = "CMDDATA3 (rw) register accessor: Command Data Register 3 This register forms the data for a command. For DATAWIDTH == 128: This register represents bits 127:96 of flash word data register 0. For DATAWIDTH == 64: This register represents bits 63:32 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all flash wrapper commands. Use cases for the CMDDATA* registers are as follows: 1) Program - These registers contain the data to be programmed. 2) Erase - These registers are not used. 3) Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata3`]
module"]
#[doc(alias = "CMDDATA3")]
pub type Cmddata3 = crate::Reg<cmddata3::Cmddata3Spec>;
#[doc = "Command Data Register 3 This register forms the data for a command. For DATAWIDTH == 128: This register represents bits 127:96 of flash word data register 0. For DATAWIDTH == 64: This register represents bits 63:32 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all flash wrapper commands. Use cases for the CMDDATA* registers are as follows: 1) Program - These registers contain the data to be programmed. 2) Erase - These registers are not used. 3) Read Verify - These registers contain data to be verified."]
pub mod cmddata3;
#[doc = "CMDWEPROTA (rw) register accessor: Command WriteErase Protect A Register This register allows the first 32 sectors of the main region to be protected from program or erase, with 1 bit protecting each sector. If the main region size is smaller than 32 sectors, then this register provides protection for the whole region. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdweprota::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdweprota::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdweprota`]
module"]
#[doc(alias = "CMDWEPROTA")]
pub type Cmdweprota = crate::Reg<cmdweprota::CmdweprotaSpec>;
#[doc = "Command WriteErase Protect A Register This register allows the first 32 sectors of the main region to be protected from program or erase, with 1 bit protecting each sector. If the main region size is smaller than 32 sectors, then this register provides protection for the whole region. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands."]
pub mod cmdweprota;
#[doc = "CMDWEPROTB (rw) register accessor: Command WriteErase Protect B Register This register allows main region sectors to be protected from program and erase. Each bit corresponds to a group of 8 sectors. There are 3 cases for how these protect bits are applied: 1. Single-bank system: In the case where only a single flash bank is present, the first 32 sectors are protected via the CMDWEPROTA register. Thus, the protection give by the bits in CMDWEPROTB begin with sector 32. 2. Multi-bank system, Bank 0: When multiple flash banks are present, the first 32 sectors of bank 0 are protected via the CMDWEPROTA register. Thus, only bits 4 and above of CMDWEPROTB would be applicable to bank 0. The protection of bit 4 and above would begin at sector 32. Bits 3:0 of WEPROTB are ignored for bank 0. 3. Multi-bank system, Banks 1-N: For banks other than bank 0 in a multi-bank system, CMDWEPROTA has no effect, so the bits in CMDWEPROTB will protect these banks starting from sector 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdweprotb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdweprotb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdweprotb`]
module"]
#[doc(alias = "CMDWEPROTB")]
pub type Cmdweprotb = crate::Reg<cmdweprotb::CmdweprotbSpec>;
#[doc = "Command WriteErase Protect B Register This register allows main region sectors to be protected from program and erase. Each bit corresponds to a group of 8 sectors. There are 3 cases for how these protect bits are applied: 1. Single-bank system: In the case where only a single flash bank is present, the first 32 sectors are protected via the CMDWEPROTA register. Thus, the protection give by the bits in CMDWEPROTB begin with sector 32. 2. Multi-bank system, Bank 0: When multiple flash banks are present, the first 32 sectors of bank 0 are protected via the CMDWEPROTA register. Thus, only bits 4 and above of CMDWEPROTB would be applicable to bank 0. The protection of bit 4 and above would begin at sector 32. Bits 3:0 of WEPROTB are ignored for bank 0. 3. Multi-bank system, Banks 1-N: For banks other than bank 0 in a multi-bank system, CMDWEPROTA has no effect, so the bits in CMDWEPROTB will protect these banks starting from sector 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands."]
pub mod cmdweprotb;
#[doc = "CMDWEPROTNM (rw) register accessor: Command WriteErase Protect Non-Main Register This register allows non-main region region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdweprotnm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdweprotnm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdweprotnm`]
module"]
#[doc(alias = "CMDWEPROTNM")]
pub type Cmdweprotnm = crate::Reg<cmdweprotnm::CmdweprotnmSpec>;
#[doc = "Command WriteErase Protect Non-Main Register This register allows non-main region region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands."]
pub mod cmdweprotnm;
#[doc = "CMDWEPROTTR (rw) register accessor: Command WriteErase Protect Trim Register This register allows trim region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdweprottr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdweprottr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdweprottr`]
module"]
#[doc(alias = "CMDWEPROTTR")]
pub type Cmdweprottr = crate::Reg<cmdweprottr::CmdweprottrSpec>;
#[doc = "Command WriteErase Protect Trim Register This register allows trim region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands."]
pub mod cmdweprottr;
#[doc = "CMDWEPROTEN (rw) register accessor: Command WriteErase Protect Engr Register This register allows engr region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdweproten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdweproten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdweproten`]
module"]
#[doc(alias = "CMDWEPROTEN")]
pub type Cmdweproten = crate::Reg<cmdweproten::CmdweprotenSpec>;
#[doc = "Command WriteErase Protect Engr Register This register allows engr region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all flash wrapper commands."]
pub mod cmdweproten;
#[doc = "CFGCMD (rw) register accessor: Command Configuration Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgcmd`]
module"]
#[doc(alias = "CFGCMD")]
pub type Cfgcmd = crate::Reg<cfgcmd::CfgcmdSpec>;
#[doc = "Command Configuration Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
pub mod cfgcmd;
#[doc = "CFGPCNT (rw) register accessor: Pulse Counter Configuration Register This register allows further configuration of maximum pulse counts for program and erase operations. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgpcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgpcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgpcnt`]
module"]
#[doc(alias = "CFGPCNT")]
pub type Cfgpcnt = crate::Reg<cfgpcnt::CfgpcntSpec>;
#[doc = "Pulse Counter Configuration Register This register allows further configuration of maximum pulse counts for program and erase operations. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the flash wrapper hardware."]
pub mod cfgpcnt;
#[doc = "STATCMD (rw) register accessor: Command Status Register This register contains status regarding completion and errors of command execution.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statcmd`]
module"]
#[doc(alias = "STATCMD")]
pub type Statcmd = crate::Reg<statcmd::StatcmdSpec>;
#[doc = "Command Status Register This register contains status regarding completion and errors of command execution."]
pub mod statcmd;
#[doc = "STATADDR (rw) register accessor: Current Address Counter Value Read only register giving read access to the state machine current address. A bank id, region id and address are stored in this register and are incremented as necessary during execution of a command.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stataddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stataddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stataddr`]
module"]
#[doc(alias = "STATADDR")]
pub type Stataddr = crate::Reg<stataddr::StataddrSpec>;
#[doc = "Current Address Counter Value Read only register giving read access to the state machine current address. A bank id, region id and address are stored in this register and are incremented as necessary during execution of a command."]
pub mod stataddr;
#[doc = "STATPCNT (rw) register accessor: Current Pulse Count Register: Read only register giving read access to the state machine current pulse count value for program/erase operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statpcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statpcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statpcnt`]
module"]
#[doc(alias = "STATPCNT")]
pub type Statpcnt = crate::Reg<statpcnt::StatpcntSpec>;
#[doc = "Current Pulse Count Register: Read only register giving read access to the state machine current pulse count value for program/erase operations."]
pub mod statpcnt;
#[doc = "STATMODE (rw) register accessor: Mode Status Register Indicates any banks which not in READ mode, and it indicates the mode which the bank(s) are in.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`statmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statmode`]
module"]
#[doc(alias = "STATMODE")]
pub type Statmode = crate::Reg<statmode::StatmodeSpec>;
#[doc = "Mode Status Register Indicates any banks which not in READ mode, and it indicates the mode which the bank(s) are in."]
pub mod statmode;
#[doc = "GBLINFO0 (rw) register accessor: Global Info 0 Register Read only register detailing information about sector size and number of banks present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gblinfo0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gblinfo0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gblinfo0`]
module"]
#[doc(alias = "GBLINFO0")]
pub type Gblinfo0 = crate::Reg<gblinfo0::Gblinfo0Spec>;
#[doc = "Global Info 0 Register Read only register detailing information about sector size and number of banks present."]
pub mod gblinfo0;
#[doc = "GBLINFO1 (rw) register accessor: Global Info 1 Register Read only register detailing information about data, ecc and redundant data widths in bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gblinfo1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gblinfo1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gblinfo1`]
module"]
#[doc(alias = "GBLINFO1")]
pub type Gblinfo1 = crate::Reg<gblinfo1::Gblinfo1Spec>;
#[doc = "Global Info 1 Register Read only register detailing information about data, ecc and redundant data widths in bits."]
pub mod gblinfo1;
#[doc = "GBLINFO2 (rw) register accessor: Global Info 2 Register Read only register detailing information about the number of data registers present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gblinfo2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gblinfo2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gblinfo2`]
module"]
#[doc(alias = "GBLINFO2")]
pub type Gblinfo2 = crate::Reg<gblinfo2::Gblinfo2Spec>;
#[doc = "Global Info 2 Register Read only register detailing information about the number of data registers present."]
pub mod gblinfo2;
#[doc = "BANK0INFO0 (rw) register accessor: Bank Info 0 Register for bank 0. Read only register detailing information about Main region size in the bank.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank0info0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank0info0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bank0info0`]
module"]
#[doc(alias = "BANK0INFO0")]
pub type Bank0info0 = crate::Reg<bank0info0::Bank0info0Spec>;
#[doc = "Bank Info 0 Register for bank 0. Read only register detailing information about Main region size in the bank."]
pub mod bank0info0;
#[doc = "BANK0INFO1 (rw) register accessor: Bank Info1 Register for bank 0. Read only register detailing information about Non-Main, Trim, and Engr region sizes in the bank.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank0info1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank0info1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bank0info1`]
module"]
#[doc(alias = "BANK0INFO1")]
pub type Bank0info1 = crate::Reg<bank0info1::Bank0info1Spec>;
#[doc = "Bank Info1 Register for bank 0. Read only register detailing information about Non-Main, Trim, and Engr region sizes in the bank."]
pub mod bank0info1;
