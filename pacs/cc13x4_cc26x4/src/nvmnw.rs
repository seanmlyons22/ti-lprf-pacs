#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    iidx: Iidx,
    _reserved1: [u8; 0x04],
    imask: Imask,
    _reserved2: [u8; 0x04],
    ris: Ris,
    _reserved3: [u8; 0x04],
    mis: Mis,
    _reserved4: [u8; 0x04],
    iset: Iset,
    _reserved5: [u8; 0x04],
    iclr: Iclr,
    _reserved6: [u8; 0x94],
    evt_mode: EvtMode,
    _reserved7: [u8; 0x18],
    desc: Desc,
    cmdexec: Cmdexec,
    cmdtype: Cmdtype,
    cmdctl: Cmdctl,
    _reserved11: [u8; 0x14],
    cmdaddr: Cmdaddr,
    cmdbyten: Cmdbyten,
    _reserved13: [u8; 0x04],
    cmddataindex: Cmddataindex,
    cmddata0: Cmddata0,
    cmddata1: Cmddata1,
    cmddata2: Cmddata2,
    cmddata3: Cmddata3,
    cmddata4: Cmddata4,
    cmddata5: Cmddata5,
    cmddata6: Cmddata6,
    cmddata7: Cmddata7,
    cmddata8: Cmddata8,
    cmddata9: Cmddata9,
    cmddata10: Cmddata10,
    cmddata11: Cmddata11,
    cmddata12: Cmddata12,
    cmddata13: Cmddata13,
    cmddata14: Cmddata14,
    cmddata15: Cmddata15,
    _reserved30: [u8; 0x60],
    cmdweprota: Cmdweprota,
    cmdweprotb: Cmdweprotb,
    _reserved32: [u8; 0x38],
    cmdweprotnm: Cmdweprotnm,
    cmdweprottr: Cmdweprottr,
    cmdweproten: Cmdweproten,
    _reserved35: [u8; 0x0194],
    cfgcmd: Cfgcmd,
    cfgpcnt: Cfgpcnt,
    _reserved37: [u8; 0x18],
    statcmd: Statcmd,
    stataddr: Stataddr,
    statpcnt: Statpcnt,
    statmode: Statmode,
    _reserved41: [u8; 0x10],
    gblinfo0: Gblinfo0,
    gblinfo1: Gblinfo1,
    gblinfo2: Gblinfo2,
    _reserved44: [u8; 0x04],
    bank0info0: Bank0info0,
    bank0info1: Bank0info1,
    _reserved46: [u8; 0x08],
    bank1info0: Bank1info0,
    bank1info1: Bank1info1,
    _reserved48: [u8; 0xe8],
    dften: Dften,
    dftcmdctl: Dftcmdctl,
    dfttimerctl: Dfttimerctl,
    dftexeczctl: Dftexeczctl,
    dftpclktestctl: Dftpclktestctl,
    dftpclkteststat: Dftpclkteststat,
    _reserved54: [u8; 0x28],
    dftdatared0: Dftdatared0,
    dftdatared1: Dftdatared1,
    dftdatared2: Dftdatared2,
    dftdatared3: Dftdatared3,
    _reserved58: [u8; 0x10],
    dftpumpctl: Dftpumpctl,
    dftbankctl: Dftbankctl,
}
impl RegisterBlock {
    #[doc = "0x20 - Interrupt Index Register: The IIDX register provides the highest priority enabled interrupt index. PSD compliant register. Note that it is not recommended to use this register if the system clock is running at a slower clock frequency than the NoWrapper clock. If this is the case, then reading this register may fail to update the RIS register correctly. The MIS register should be read directly, and a write to ICLR should be used to clear interrupts when this clock relationship is present."]
    #[inline(always)]
    pub const fn iidx(&self) -> &Iidx {
        &self.iidx
    }
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
    #[doc = "0xe0 - Event mode register. It is used to select whether each line is disabled, in software mode (software clears the RIS) or in hardware mode (hardware clears the RIS)."]
    #[inline(always)]
    pub const fn evt_mode(&self) -> &EvtMode {
        &self.evt_mode
    }
    #[doc = "0xfc - Hardware Version Description Register: This register identifies the NoWrapper hardware version and feature set used."]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x100 - Command Execute Register: Initiates execution of the command specified in the CMDTYPE register. This register is blocked for writes after being written to 1 and prior to STATCMD.DONE being set by the NoWrapper hardware. NoWrapper hardware clears this register after the processing of the command has completed."]
    #[inline(always)]
    pub const fn cmdexec(&self) -> &Cmdexec {
        &self.cmdexec
    }
    #[doc = "0x104 - Command Type Register This register specifies the type of command to be executed by the NoWrapper hardware. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
    #[inline(always)]
    pub const fn cmdtype(&self) -> &Cmdtype {
        &self.cmdtype
    }
    #[doc = "0x108 - Command Control Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
    #[inline(always)]
    pub const fn cmdctl(&self) -> &Cmdctl {
        &self.cmdctl
    }
    #[doc = "0x120 - Command Address Register: This register forms the target address of a command. The use cases are as follows: 1)For single-word program, this address indicates the flash bank word to be programmed. 2)For multi-word program, this address indicates the first flash bank address for the program. The address will be incremented for further words. 3)For sector erase, this address indicates the sector to be erased. 4)For bank erase, the address indicates the bank to be erased. 5)For read verify, the address indications follow program/erase listed above. Note the address written to this register will be submitted for translation to the NoWrapper address translation interface, and the translated address will be used to access the bank. However, if the CMDCTL.ADDRXLATEOVR bit is set, then the address written to this register will be used directly as the bank address. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    #[inline(always)]
    pub const fn cmdaddr(&self) -> &Cmdaddr {
        &self.cmdaddr
    }
    #[doc = "0x124 - Command Program Byte Enable Register: This register forms a per-byte enable for programming data. For data bytes to be programmed, a 1 must be written to the corresponding bit in this register. Normally, all bits are written to 1, allowing program of full flash words. However, leaving some bits 0 allows programming of 8-bit, 16-bit, 32-bit or 64-bit portions of a flash word. In addtion, the read verify command will ignore data bytes read from the flash in its comparison if the corresponding CMDBYTEN bit is 0. ECC data bytes are protected by the 1-2 MSB bits in this register, depending on the presence of ECC and the flash word data width. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is written to all 0 after the completion of all NoWrapper commands."]
    #[inline(always)]
    pub const fn cmdbyten(&self) -> &Cmdbyten {
        &self.cmdbyten
    }
    #[doc = "0x12c - Command Program Data Index Register: When multiple data registers are available for multi-word program, this register can be written with an index which points to one of the data registers. When a write to CMDDATA* is done, the data will be written to the physical data register indexed by the value in this register. Up to 8 data registers can be present, so this register can be written with 0x0 to 0x7. If less than 8 data registers are present, successive MSB bits of this register are ignored when indexing the CMDDATA* registers. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    #[inline(always)]
    pub const fn cmddataindex(&self) -> &Cmddataindex {
        &self.cmddataindex
    }
    #[doc = "0x130 - Command Data Register 0 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata0(&self) -> &Cmddata0 {
        &self.cmddata0
    }
    #[doc = "0x134 - Command Data Register 1 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to CMDSTAT.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata1(&self) -> &Cmddata1 {
        &self.cmddata1
    }
    #[doc = "0x138 - Command Data Register 2 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata2(&self) -> &Cmddata2 {
        &self.cmddata2
    }
    #[doc = "0x13c - Command Data Register 3 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata3(&self) -> &Cmddata3 {
        &self.cmddata3
    }
    #[doc = "0x140 - Command Data Register 4 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 2. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata4(&self) -> &Cmddata4 {
        &self.cmddata4
    }
    #[doc = "0x144 - Command Data Register 5 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 2. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata5(&self) -> &Cmddata5 {
        &self.cmddata5
    }
    #[doc = "0x148 - Command Data Register 6 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 3. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata6(&self) -> &Cmddata6 {
        &self.cmddata6
    }
    #[doc = "0x14c - Command Data Register 7 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 3. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata7(&self) -> &Cmddata7 {
        &self.cmddata7
    }
    #[doc = "0x150 - Command Data Register 8 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 4. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata8(&self) -> &Cmddata8 {
        &self.cmddata8
    }
    #[doc = "0x154 - Command Data Register 9 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 4. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata9(&self) -> &Cmddata9 {
        &self.cmddata9
    }
    #[doc = "0x158 - Command Data Register 10 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 5. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata10(&self) -> &Cmddata10 {
        &self.cmddata10
    }
    #[doc = "0x15c - Command Data Register 11 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 5. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata11(&self) -> &Cmddata11 {
        &self.cmddata11
    }
    #[doc = "0x160 - Command Data Register 12 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 6. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata12(&self) -> &Cmddata12 {
        &self.cmddata12
    }
    #[doc = "0x164 - Command Data Register 13 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 6. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata13(&self) -> &Cmddata13 {
        &self.cmddata13
    }
    #[doc = "0x168 - Command Data Register 14 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 7. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata14(&self) -> &Cmddata14 {
        &self.cmddata14
    }
    #[doc = "0x16c - Command Data Register 15 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 7. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    #[inline(always)]
    pub const fn cmddata15(&self) -> &Cmddata15 {
        &self.cmddata15
    }
    #[doc = "0x1d0 - Command WriteErase Protect A Register This register allows the first 32 sectors of the main region to be protected from program or erase, with 1 bit protecting each sector. If the main region size is smaller than 32 sectors, then this register provides protection for the whole region. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
    #[inline(always)]
    pub const fn cmdweprota(&self) -> &Cmdweprota {
        &self.cmdweprota
    }
    #[doc = "0x1d4 - Command WriteErase Protect B Register This register allows main region sectors to be protected from program and erase. Each bit corresponds to a group of 8 sectors. There are 3 cases for how these protect bits are applied: 1. Single-bank system: In the case where only a single flash bank is present, the first 32 sectors are protected via the CMDWEPROTA register. Thus, the protection give by the bits in CMDWEPROTB begin with sector 32. 2. Multi-bank system, Bank 0: When multiple flash banks are present, the first 32 sectors of bank 0 are protected via the CMDWEPROTA register. Thus, only bits 4 and above of CMDWEPROTB would be applicable to bank 0. The protection of bit 4 and above would begin at sector 32. Bits 3:0 of WEPROTB are ignored for bank 0. 3. Multi-bank system, Banks 1-N: For banks other than bank 0 in a multi-bank system, CMDWEPROTA has no effect, so the bits in CMDWEPROTB will protect these banks starting from sector 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
    #[inline(always)]
    pub const fn cmdweprotb(&self) -> &Cmdweprotb {
        &self.cmdweprotb
    }
    #[doc = "0x210 - Command WriteErase Protect Non-Main Register This register allows non-main region region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
    #[inline(always)]
    pub const fn cmdweprotnm(&self) -> &Cmdweprotnm {
        &self.cmdweprotnm
    }
    #[doc = "0x214 - Command WriteErase Protect Trim Register This register allows trim region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
    #[inline(always)]
    pub const fn cmdweprottr(&self) -> &Cmdweprottr {
        &self.cmdweprottr
    }
    #[doc = "0x218 - Command WriteErase Protect Engr Register This register allows engr region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
    #[inline(always)]
    pub const fn cmdweproten(&self) -> &Cmdweproten {
        &self.cmdweproten
    }
    #[doc = "0x3b0 - Command Configuration Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
    #[inline(always)]
    pub const fn cfgcmd(&self) -> &Cfgcmd {
        &self.cfgcmd
    }
    #[doc = "0x3b4 - Pulse Counter Configuration Register This register allows further configuration of maximum pulse counts for program and erase operations. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
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
    #[doc = "0x410 - Bank Info 0 Register for bank 1. Read only register detailing information about Main region size in the bank."]
    #[inline(always)]
    pub const fn bank1info0(&self) -> &Bank1info0 {
        &self.bank1info0
    }
    #[doc = "0x414 - Bank Info1 Register for bank 1. Read only register detailing information about Non-Main, Trim, and Engr region sizes in the bank."]
    #[inline(always)]
    pub const fn bank1info1(&self) -> &Bank1info1 {
        &self.bank1info1
    }
    #[doc = "0x500 - DFT Enable Register Allows control of NoWrapper test features. When set, DFT* registers in this aperture open for write access. When cleared, DFT* registers are read-only. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    #[inline(always)]
    pub const fn dften(&self) -> &Dften {
        &self.dften
    }
    #[doc = "0x504 - DFT Command Control Register This register configures specific capabilities for test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    #[inline(always)]
    pub const fn dftcmdctl(&self) -> &Dftcmdctl {
        &self.dftcmdctl
    }
    #[doc = "0x508 - DFT Timer Control Register This allows some configuration of timing values for various phases of flash operations for test. These time values are hard-coded for functional execution. This register is only writable when DFT.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    #[inline(always)]
    pub const fn dfttimerctl(&self) -> &Dfttimerctl {
        &self.dfttimerctl
    }
    #[doc = "0x50c - DFT EXECUTEZ control register. This register allows direct control of the EXECUTEZ to bank and pump for test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    #[inline(always)]
    pub const fn dftexeczctl(&self) -> &Dftexeczctl {
        &self.dftexeczctl
    }
    #[doc = "0x510 - DFT Pump Clock Test Control Register. This register controls hardware features that allow the pump clock to be characterized for trim development. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    #[inline(always)]
    pub const fn dftpclktestctl(&self) -> &Dftpclktestctl {
        &self.dftpclktestctl
    }
    #[doc = "0x514 - DFT Pump Clock Test Status Register. This register shows status reported by the hardware features that allow the pump clock to be characterized for trim development. This register is only writable when DFTEN.ENABLE is set."]
    #[inline(always)]
    pub const fn dftpclkteststat(&self) -> &Dftpclkteststat {
        &self.dftpclkteststat
    }
    #[doc = "0x540 - DFT Redundancy Data Register 0 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 0. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    #[inline(always)]
    pub const fn dftdatared0(&self) -> &Dftdatared0 {
        &self.dftdatared0
    }
    #[doc = "0x544 - DFT Redundancy Data Register 1 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 1. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    #[inline(always)]
    pub const fn dftdatared1(&self) -> &Dftdatared1 {
        &self.dftdatared1
    }
    #[doc = "0x548 - DFT Redundancy Data Register 2 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 2. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    #[inline(always)]
    pub const fn dftdatared2(&self) -> &Dftdatared2 {
        &self.dftdatared2
    }
    #[doc = "0x54c - DFT Redundancy Data Register 3 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 3. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    #[inline(always)]
    pub const fn dftdatared3(&self) -> &Dftdatared3 {
        &self.dftdatared3
    }
    #[doc = "0x560 - DFT Pump Control Register This allows some configuration of pump parameters during test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    #[inline(always)]
    pub const fn dftpumpctl(&self) -> &Dftpumpctl {
        &self.dftpumpctl
    }
    #[doc = "0x564 - DFT Bank Control Register This allows some configuration of bank parameters during test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    #[inline(always)]
    pub const fn dftbankctl(&self) -> &Dftbankctl {
        &self.dftbankctl
    }
}
#[doc = "IIDX (rw) register accessor: Interrupt Index Register: The IIDX register provides the highest priority enabled interrupt index. PSD compliant register. Note that it is not recommended to use this register if the system clock is running at a slower clock frequency than the NoWrapper clock. If this is the case, then reading this register may fail to update the RIS register correctly. The MIS register should be read directly, and a write to ICLR should be used to clear interrupts when this clock relationship is present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iidx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iidx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iidx`]
module"]
#[doc(alias = "IIDX")]
pub type Iidx = crate::Reg<iidx::IidxSpec>;
#[doc = "Interrupt Index Register: The IIDX register provides the highest priority enabled interrupt index. PSD compliant register. Note that it is not recommended to use this register if the system clock is running at a slower clock frequency than the NoWrapper clock. If this is the case, then reading this register may fail to update the RIS register correctly. The MIS register should be read directly, and a write to ICLR should be used to clear interrupts when this clock relationship is present."]
pub mod iidx;
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
#[doc = "EVT_MODE (rw) register accessor: Event mode register. It is used to select whether each line is disabled, in software mode (software clears the RIS) or in hardware mode (hardware clears the RIS).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_mode`]
module"]
#[doc(alias = "EVT_MODE")]
pub type EvtMode = crate::Reg<evt_mode::EvtModeSpec>;
#[doc = "Event mode register. It is used to select whether each line is disabled, in software mode (software clears the RIS) or in hardware mode (hardware clears the RIS)."]
pub mod evt_mode;
#[doc = "DESC (rw) register accessor: Hardware Version Description Register: This register identifies the NoWrapper hardware version and feature set used.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Hardware Version Description Register: This register identifies the NoWrapper hardware version and feature set used."]
pub mod desc;
#[doc = "CMDEXEC (rw) register accessor: Command Execute Register: Initiates execution of the command specified in the CMDTYPE register. This register is blocked for writes after being written to 1 and prior to STATCMD.DONE being set by the NoWrapper hardware. NoWrapper hardware clears this register after the processing of the command has completed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdexec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdexec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdexec`]
module"]
#[doc(alias = "CMDEXEC")]
pub type Cmdexec = crate::Reg<cmdexec::CmdexecSpec>;
#[doc = "Command Execute Register: Initiates execution of the command specified in the CMDTYPE register. This register is blocked for writes after being written to 1 and prior to STATCMD.DONE being set by the NoWrapper hardware. NoWrapper hardware clears this register after the processing of the command has completed."]
pub mod cmdexec;
#[doc = "CMDTYPE (rw) register accessor: Command Type Register This register specifies the type of command to be executed by the NoWrapper hardware. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdtype`]
module"]
#[doc(alias = "CMDTYPE")]
pub type Cmdtype = crate::Reg<cmdtype::CmdtypeSpec>;
#[doc = "Command Type Register This register specifies the type of command to be executed by the NoWrapper hardware. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
pub mod cmdtype;
#[doc = "CMDCTL (rw) register accessor: Command Control Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdctl`]
module"]
#[doc(alias = "CMDCTL")]
pub type Cmdctl = crate::Reg<cmdctl::CmdctlSpec>;
#[doc = "Command Control Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
pub mod cmdctl;
#[doc = "CMDADDR (rw) register accessor: Command Address Register: This register forms the target address of a command. The use cases are as follows: 1)For single-word program, this address indicates the flash bank word to be programmed. 2)For multi-word program, this address indicates the first flash bank address for the program. The address will be incremented for further words. 3)For sector erase, this address indicates the sector to be erased. 4)For bank erase, the address indicates the bank to be erased. 5)For read verify, the address indications follow program/erase listed above. Note the address written to this register will be submitted for translation to the NoWrapper address translation interface, and the translated address will be used to access the bank. However, if the CMDCTL.ADDRXLATEOVR bit is set, then the address written to this register will be used directly as the bank address. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdaddr`]
module"]
#[doc(alias = "CMDADDR")]
pub type Cmdaddr = crate::Reg<cmdaddr::CmdaddrSpec>;
#[doc = "Command Address Register: This register forms the target address of a command. The use cases are as follows: 1)For single-word program, this address indicates the flash bank word to be programmed. 2)For multi-word program, this address indicates the first flash bank address for the program. The address will be incremented for further words. 3)For sector erase, this address indicates the sector to be erased. 4)For bank erase, the address indicates the bank to be erased. 5)For read verify, the address indications follow program/erase listed above. Note the address written to this register will be submitted for translation to the NoWrapper address translation interface, and the translated address will be used to access the bank. However, if the CMDCTL.ADDRXLATEOVR bit is set, then the address written to this register will be used directly as the bank address. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod cmdaddr;
#[doc = "CMDBYTEN (rw) register accessor: Command Program Byte Enable Register: This register forms a per-byte enable for programming data. For data bytes to be programmed, a 1 must be written to the corresponding bit in this register. Normally, all bits are written to 1, allowing program of full flash words. However, leaving some bits 0 allows programming of 8-bit, 16-bit, 32-bit or 64-bit portions of a flash word. In addtion, the read verify command will ignore data bytes read from the flash in its comparison if the corresponding CMDBYTEN bit is 0. ECC data bytes are protected by the 1-2 MSB bits in this register, depending on the presence of ECC and the flash word data width. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is written to all 0 after the completion of all NoWrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdbyten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdbyten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdbyten`]
module"]
#[doc(alias = "CMDBYTEN")]
pub type Cmdbyten = crate::Reg<cmdbyten::CmdbytenSpec>;
#[doc = "Command Program Byte Enable Register: This register forms a per-byte enable for programming data. For data bytes to be programmed, a 1 must be written to the corresponding bit in this register. Normally, all bits are written to 1, allowing program of full flash words. However, leaving some bits 0 allows programming of 8-bit, 16-bit, 32-bit or 64-bit portions of a flash word. In addtion, the read verify command will ignore data bytes read from the flash in its comparison if the corresponding CMDBYTEN bit is 0. ECC data bytes are protected by the 1-2 MSB bits in this register, depending on the presence of ECC and the flash word data width. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is written to all 0 after the completion of all NoWrapper commands."]
pub mod cmdbyten;
#[doc = "CMDDATAINDEX (rw) register accessor: Command Program Data Index Register: When multiple data registers are available for multi-word program, this register can be written with an index which points to one of the data registers. When a write to CMDDATA* is done, the data will be written to the physical data register indexed by the value in this register. Up to 8 data registers can be present, so this register can be written with 0x0 to 0x7. If less than 8 data registers are present, successive MSB bits of this register are ignored when indexing the CMDDATA* registers. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddataindex::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddataindex::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddataindex`]
module"]
#[doc(alias = "CMDDATAINDEX")]
pub type Cmddataindex = crate::Reg<cmddataindex::CmddataindexSpec>;
#[doc = "Command Program Data Index Register: When multiple data registers are available for multi-word program, this register can be written with an index which points to one of the data registers. When a write to CMDDATA* is done, the data will be written to the physical data register indexed by the value in this register. Up to 8 data registers can be present, so this register can be written with 0x0 to 0x7. If less than 8 data registers are present, successive MSB bits of this register are ignored when indexing the CMDDATA* registers. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod cmddataindex;
#[doc = "CMDDATA0 (rw) register accessor: Command Data Register 0 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata0`]
module"]
#[doc(alias = "CMDDATA0")]
pub type Cmddata0 = crate::Reg<cmddata0::Cmddata0Spec>;
#[doc = "Command Data Register 0 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata0;
#[doc = "CMDDATA1 (rw) register accessor: Command Data Register 1 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to CMDSTAT.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata1`]
module"]
#[doc(alias = "CMDDATA1")]
pub type Cmddata1 = crate::Reg<cmddata1::Cmddata1Spec>;
#[doc = "Command Data Register 1 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to CMDSTAT.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata1;
#[doc = "CMDDATA2 (rw) register accessor: Command Data Register 2 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata2`]
module"]
#[doc(alias = "CMDDATA2")]
pub type Cmddata2 = crate::Reg<cmddata2::Cmddata2Spec>;
#[doc = "Command Data Register 2 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata2;
#[doc = "CMDDATA3 (rw) register accessor: Command Data Register 3 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata3`]
module"]
#[doc(alias = "CMDDATA3")]
pub type Cmddata3 = crate::Reg<cmddata3::Cmddata3Spec>;
#[doc = "Command Data Register 3 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata3;
#[doc = "CMDDATA4 (rw) register accessor: Command Data Register 4 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 2. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata4`]
module"]
#[doc(alias = "CMDDATA4")]
pub type Cmddata4 = crate::Reg<cmddata4::Cmddata4Spec>;
#[doc = "Command Data Register 4 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 2. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata4;
#[doc = "CMDDATA5 (rw) register accessor: Command Data Register 5 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 2. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata5`]
module"]
#[doc(alias = "CMDDATA5")]
pub type Cmddata5 = crate::Reg<cmddata5::Cmddata5Spec>;
#[doc = "Command Data Register 5 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 2. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata5;
#[doc = "CMDDATA6 (rw) register accessor: Command Data Register 6 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 3. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata6`]
module"]
#[doc(alias = "CMDDATA6")]
pub type Cmddata6 = crate::Reg<cmddata6::Cmddata6Spec>;
#[doc = "Command Data Register 6 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 3. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata6;
#[doc = "CMDDATA7 (rw) register accessor: Command Data Register 7 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 3. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata7`]
module"]
#[doc(alias = "CMDDATA7")]
pub type Cmddata7 = crate::Reg<cmddata7::Cmddata7Spec>;
#[doc = "Command Data Register 7 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 3. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata7;
#[doc = "CMDDATA8 (rw) register accessor: Command Data Register 8 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 4. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata8`]
module"]
#[doc(alias = "CMDDATA8")]
pub type Cmddata8 = crate::Reg<cmddata8::Cmddata8Spec>;
#[doc = "Command Data Register 8 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 4. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata8;
#[doc = "CMDDATA9 (rw) register accessor: Command Data Register 9 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 4. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata9`]
module"]
#[doc(alias = "CMDDATA9")]
pub type Cmddata9 = crate::Reg<cmddata9::Cmddata9Spec>;
#[doc = "Command Data Register 9 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 4. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata9;
#[doc = "CMDDATA10 (rw) register accessor: Command Data Register 10 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 5. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata10`]
module"]
#[doc(alias = "CMDDATA10")]
pub type Cmddata10 = crate::Reg<cmddata10::Cmddata10Spec>;
#[doc = "Command Data Register 10 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 5. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata10;
#[doc = "CMDDATA11 (rw) register accessor: Command Data Register 11 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 5. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata11`]
module"]
#[doc(alias = "CMDDATA11")]
pub type Cmddata11 = crate::Reg<cmddata11::Cmddata11Spec>;
#[doc = "Command Data Register 11 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 5. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata11;
#[doc = "CMDDATA12 (rw) register accessor: Command Data Register 12 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 6. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata12`]
module"]
#[doc(alias = "CMDDATA12")]
pub type Cmddata12 = crate::Reg<cmddata12::Cmddata12Spec>;
#[doc = "Command Data Register 12 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 6. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata12;
#[doc = "CMDDATA13 (rw) register accessor: Command Data Register 13 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 6. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata13`]
module"]
#[doc(alias = "CMDDATA13")]
pub type Cmddata13 = crate::Reg<cmddata13::Cmddata13Spec>;
#[doc = "Command Data Register 13 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 6. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata13;
#[doc = "CMDDATA14 (rw) register accessor: Command Data Register 14 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 7. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata14`]
module"]
#[doc(alias = "CMDDATA14")]
pub type Cmddata14 = crate::Reg<cmddata14::Cmddata14Spec>;
#[doc = "Command Data Register 14 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 7. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata14;
#[doc = "CMDDATA15 (rw) register accessor: Command Data Register 15 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 7. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmddata15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmddata15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmddata15`]
module"]
#[doc(alias = "CMDDATA15")]
pub type Cmddata15 = crate::Reg<cmddata15::Cmddata15Spec>;
#[doc = "Command Data Register 15 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 7. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata15;
#[doc = "CMDWEPROTA (rw) register accessor: Command WriteErase Protect A Register This register allows the first 32 sectors of the main region to be protected from program or erase, with 1 bit protecting each sector. If the main region size is smaller than 32 sectors, then this register provides protection for the whole region. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdweprota::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdweprota::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdweprota`]
module"]
#[doc(alias = "CMDWEPROTA")]
pub type Cmdweprota = crate::Reg<cmdweprota::CmdweprotaSpec>;
#[doc = "Command WriteErase Protect A Register This register allows the first 32 sectors of the main region to be protected from program or erase, with 1 bit protecting each sector. If the main region size is smaller than 32 sectors, then this register provides protection for the whole region. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
pub mod cmdweprota;
#[doc = "CMDWEPROTB (rw) register accessor: Command WriteErase Protect B Register This register allows main region sectors to be protected from program and erase. Each bit corresponds to a group of 8 sectors. There are 3 cases for how these protect bits are applied: 1. Single-bank system: In the case where only a single flash bank is present, the first 32 sectors are protected via the CMDWEPROTA register. Thus, the protection give by the bits in CMDWEPROTB begin with sector 32. 2. Multi-bank system, Bank 0: When multiple flash banks are present, the first 32 sectors of bank 0 are protected via the CMDWEPROTA register. Thus, only bits 4 and above of CMDWEPROTB would be applicable to bank 0. The protection of bit 4 and above would begin at sector 32. Bits 3:0 of WEPROTB are ignored for bank 0. 3. Multi-bank system, Banks 1-N: For banks other than bank 0 in a multi-bank system, CMDWEPROTA has no effect, so the bits in CMDWEPROTB will protect these banks starting from sector 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdweprotb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdweprotb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdweprotb`]
module"]
#[doc(alias = "CMDWEPROTB")]
pub type Cmdweprotb = crate::Reg<cmdweprotb::CmdweprotbSpec>;
#[doc = "Command WriteErase Protect B Register This register allows main region sectors to be protected from program and erase. Each bit corresponds to a group of 8 sectors. There are 3 cases for how these protect bits are applied: 1. Single-bank system: In the case where only a single flash bank is present, the first 32 sectors are protected via the CMDWEPROTA register. Thus, the protection give by the bits in CMDWEPROTB begin with sector 32. 2. Multi-bank system, Bank 0: When multiple flash banks are present, the first 32 sectors of bank 0 are protected via the CMDWEPROTA register. Thus, only bits 4 and above of CMDWEPROTB would be applicable to bank 0. The protection of bit 4 and above would begin at sector 32. Bits 3:0 of WEPROTB are ignored for bank 0. 3. Multi-bank system, Banks 1-N: For banks other than bank 0 in a multi-bank system, CMDWEPROTA has no effect, so the bits in CMDWEPROTB will protect these banks starting from sector 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
pub mod cmdweprotb;
#[doc = "CMDWEPROTNM (rw) register accessor: Command WriteErase Protect Non-Main Register This register allows non-main region region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdweprotnm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdweprotnm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdweprotnm`]
module"]
#[doc(alias = "CMDWEPROTNM")]
pub type Cmdweprotnm = crate::Reg<cmdweprotnm::CmdweprotnmSpec>;
#[doc = "Command WriteErase Protect Non-Main Register This register allows non-main region region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
pub mod cmdweprotnm;
#[doc = "CMDWEPROTTR (rw) register accessor: Command WriteErase Protect Trim Register This register allows trim region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdweprottr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdweprottr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdweprottr`]
module"]
#[doc(alias = "CMDWEPROTTR")]
pub type Cmdweprottr = crate::Reg<cmdweprottr::CmdweprottrSpec>;
#[doc = "Command WriteErase Protect Trim Register This register allows trim region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
pub mod cmdweprottr;
#[doc = "CMDWEPROTEN (rw) register accessor: Command WriteErase Protect Engr Register This register allows engr region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdweproten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdweproten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdweproten`]
module"]
#[doc(alias = "CMDWEPROTEN")]
pub type Cmdweproten = crate::Reg<cmdweproten::CmdweprotenSpec>;
#[doc = "Command WriteErase Protect Engr Register This register allows engr region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
pub mod cmdweproten;
#[doc = "CFGCMD (rw) register accessor: Command Configuration Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgcmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgcmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgcmd`]
module"]
#[doc(alias = "CFGCMD")]
pub type Cfgcmd = crate::Reg<cfgcmd::CfgcmdSpec>;
#[doc = "Command Configuration Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
pub mod cfgcmd;
#[doc = "CFGPCNT (rw) register accessor: Pulse Counter Configuration Register This register allows further configuration of maximum pulse counts for program and erase operations. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgpcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgpcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgpcnt`]
module"]
#[doc(alias = "CFGPCNT")]
pub type Cfgpcnt = crate::Reg<cfgpcnt::CfgpcntSpec>;
#[doc = "Pulse Counter Configuration Register This register allows further configuration of maximum pulse counts for program and erase operations. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
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
#[doc = "BANK1INFO0 (rw) register accessor: Bank Info 0 Register for bank 1. Read only register detailing information about Main region size in the bank.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank1info0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank1info0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bank1info0`]
module"]
#[doc(alias = "BANK1INFO0")]
pub type Bank1info0 = crate::Reg<bank1info0::Bank1info0Spec>;
#[doc = "Bank Info 0 Register for bank 1. Read only register detailing information about Main region size in the bank."]
pub mod bank1info0;
#[doc = "BANK1INFO1 (rw) register accessor: Bank Info1 Register for bank 1. Read only register detailing information about Non-Main, Trim, and Engr region sizes in the bank.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank1info1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank1info1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bank1info1`]
module"]
#[doc(alias = "BANK1INFO1")]
pub type Bank1info1 = crate::Reg<bank1info1::Bank1info1Spec>;
#[doc = "Bank Info1 Register for bank 1. Read only register detailing information about Non-Main, Trim, and Engr region sizes in the bank."]
pub mod bank1info1;
#[doc = "DFTEN (rw) register accessor: DFT Enable Register Allows control of NoWrapper test features. When set, DFT* registers in this aperture open for write access. When cleared, DFT* registers are read-only. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dften::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dften::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dften`]
module"]
#[doc(alias = "DFTEN")]
pub type Dften = crate::Reg<dften::DftenSpec>;
#[doc = "DFT Enable Register Allows control of NoWrapper test features. When set, DFT* registers in this aperture open for write access. When cleared, DFT* registers are read-only. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dften;
#[doc = "DFTCMDCTL (rw) register accessor: DFT Command Control Register This register configures specific capabilities for test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftcmdctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftcmdctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dftcmdctl`]
module"]
#[doc(alias = "DFTCMDCTL")]
pub type Dftcmdctl = crate::Reg<dftcmdctl::DftcmdctlSpec>;
#[doc = "DFT Command Control Register This register configures specific capabilities for test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftcmdctl;
#[doc = "DFTTIMERCTL (rw) register accessor: DFT Timer Control Register This allows some configuration of timing values for various phases of flash operations for test. These time values are hard-coded for functional execution. This register is only writable when DFT.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfttimerctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfttimerctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfttimerctl`]
module"]
#[doc(alias = "DFTTIMERCTL")]
pub type Dfttimerctl = crate::Reg<dfttimerctl::DfttimerctlSpec>;
#[doc = "DFT Timer Control Register This allows some configuration of timing values for various phases of flash operations for test. These time values are hard-coded for functional execution. This register is only writable when DFT.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dfttimerctl;
#[doc = "DFTEXECZCTL (rw) register accessor: DFT EXECUTEZ control register. This register allows direct control of the EXECUTEZ to bank and pump for test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftexeczctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftexeczctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dftexeczctl`]
module"]
#[doc(alias = "DFTEXECZCTL")]
pub type Dftexeczctl = crate::Reg<dftexeczctl::DftexeczctlSpec>;
#[doc = "DFT EXECUTEZ control register. This register allows direct control of the EXECUTEZ to bank and pump for test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftexeczctl;
#[doc = "DFTPCLKTESTCTL (rw) register accessor: DFT Pump Clock Test Control Register. This register controls hardware features that allow the pump clock to be characterized for trim development. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftpclktestctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftpclktestctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dftpclktestctl`]
module"]
#[doc(alias = "DFTPCLKTESTCTL")]
pub type Dftpclktestctl = crate::Reg<dftpclktestctl::DftpclktestctlSpec>;
#[doc = "DFT Pump Clock Test Control Register. This register controls hardware features that allow the pump clock to be characterized for trim development. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftpclktestctl;
#[doc = "DFTPCLKTESTSTAT (rw) register accessor: DFT Pump Clock Test Status Register. This register shows status reported by the hardware features that allow the pump clock to be characterized for trim development. This register is only writable when DFTEN.ENABLE is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftpclkteststat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftpclkteststat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dftpclkteststat`]
module"]
#[doc(alias = "DFTPCLKTESTSTAT")]
pub type Dftpclkteststat = crate::Reg<dftpclkteststat::DftpclkteststatSpec>;
#[doc = "DFT Pump Clock Test Status Register. This register shows status reported by the hardware features that allow the pump clock to be characterized for trim development. This register is only writable when DFTEN.ENABLE is set."]
pub mod dftpclkteststat;
#[doc = "DFTDATARED0 (rw) register accessor: DFT Redundancy Data Register 0 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 0. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftdatared0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftdatared0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dftdatared0`]
module"]
#[doc(alias = "DFTDATARED0")]
pub type Dftdatared0 = crate::Reg<dftdatared0::Dftdatared0Spec>;
#[doc = "DFT Redundancy Data Register 0 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 0. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftdatared0;
#[doc = "DFTDATARED1 (rw) register accessor: DFT Redundancy Data Register 1 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 1. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftdatared1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftdatared1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dftdatared1`]
module"]
#[doc(alias = "DFTDATARED1")]
pub type Dftdatared1 = crate::Reg<dftdatared1::Dftdatared1Spec>;
#[doc = "DFT Redundancy Data Register 1 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 1. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftdatared1;
#[doc = "DFTDATARED2 (rw) register accessor: DFT Redundancy Data Register 2 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 2. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftdatared2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftdatared2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dftdatared2`]
module"]
#[doc(alias = "DFTDATARED2")]
pub type Dftdatared2 = crate::Reg<dftdatared2::Dftdatared2Spec>;
#[doc = "DFT Redundancy Data Register 2 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 2. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftdatared2;
#[doc = "DFTDATARED3 (rw) register accessor: DFT Redundancy Data Register 3 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 3. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftdatared3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftdatared3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dftdatared3`]
module"]
#[doc(alias = "DFTDATARED3")]
pub type Dftdatared3 = crate::Reg<dftdatared3::Dftdatared3Spec>;
#[doc = "DFT Redundancy Data Register 3 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 3. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftdatared3;
#[doc = "DFTPUMPCTL (rw) register accessor: DFT Pump Control Register This allows some configuration of pump parameters during test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftpumpctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftpumpctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dftpumpctl`]
module"]
#[doc(alias = "DFTPUMPCTL")]
pub type Dftpumpctl = crate::Reg<dftpumpctl::DftpumpctlSpec>;
#[doc = "DFT Pump Control Register This allows some configuration of pump parameters during test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftpumpctl;
#[doc = "DFTBANKCTL (rw) register accessor: DFT Bank Control Register This allows some configuration of bank parameters during test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftbankctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftbankctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dftbankctl`]
module"]
#[doc(alias = "DFTBANKCTL")]
pub type Dftbankctl = crate::Reg<dftbankctl::DftbankctlSpec>;
#[doc = "DFT Bank Control Register This allows some configuration of bank parameters during test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftbankctl;
