#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    #[doc = "0x20 - Interrupt Index Register: The IIDX register provides the highest priority enabled interrupt index. PSD compliant register. Note that it is not recommended to use this register if the system clock is running at a slower clock frequency than the NoWrapper clock. If this is the case, then reading this register may fail to update the RIS register correctly. The MIS register should be read directly, and a write to ICLR should be used to clear interrupts when this clock relationship is present."]
    pub iidx: IIDX,
    _reserved1: [u8; 0x04],
    #[doc = "0x28 - Interrupt Mask Register: The IMASK register holds the current interrupt mask settings. Masked interrupts are read in the MIS register. PSD compliant register."]
    pub imask: IMASK,
    _reserved2: [u8; 0x04],
    #[doc = "0x30 - Raw Interrupt Status Register: The RIS register reflects all pending interrupts, regardless of masking. The RIS register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing a 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled. A flag can be set by software by writing a 1 to the ISET register. Reading the IIDX register will also clear the corresponding bit in RIS. PSD compliant register."]
    pub ris: RIS,
    _reserved3: [u8; 0x04],
    #[doc = "0x38 - Masked Interrupt Status Register: The MIS register is a bit-wise AND of the contents of the IMASK and RIS registers. This is kept mainly for ARM compatibility, and has limited use since the highest priority interrupt index is returned via the IIDX register. PSD compliant register."]
    pub mis: MIS,
    _reserved4: [u8; 0x04],
    #[doc = "0x40 - Interrupt Set Register: The ISET register allows software to write a 1 to set corresponding interrupt. Safety: This meets a safety requirement to allow software diagnostics to trigger interrupts. PSD compliant register."]
    pub iset: ISET,
    _reserved5: [u8; 0x04],
    #[doc = "0x48 - Interrupt Clear Register. The ICLR register allows allows software to write a 1 to clear corresponding interrupt. PSD compliant register."]
    pub iclr: ICLR,
    _reserved6: [u8; 0x94],
    #[doc = "0xe0 - Event mode register. It is used to select whether each line is disabled, in software mode (software clears the RIS) or in hardware mode (hardware clears the RIS)."]
    pub evt_mode: EVT_MODE,
    _reserved7: [u8; 0x18],
    #[doc = "0xfc - Hardware Version Description Register: This register identifies the NoWrapper hardware version and feature set used."]
    pub desc: DESC,
    #[doc = "0x100 - Command Execute Register: Initiates execution of the command specified in the CMDTYPE register. This register is blocked for writes after being written to 1 and prior to STATCMD.DONE being set by the NoWrapper hardware. NoWrapper hardware clears this register after the processing of the command has completed."]
    pub cmdexec: CMDEXEC,
    #[doc = "0x104 - Command Type Register This register specifies the type of command to be executed by the NoWrapper hardware. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
    pub cmdtype: CMDTYPE,
    #[doc = "0x108 - Command Control Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
    pub cmdctl: CMDCTL,
    _reserved11: [u8; 0x14],
    #[doc = "0x120 - Command Address Register: This register forms the target address of a command. The use cases are as follows: 1)For single-word program, this address indicates the flash bank word to be programmed. 2)For multi-word program, this address indicates the first flash bank address for the program. The address will be incremented for further words. 3)For sector erase, this address indicates the sector to be erased. 4)For bank erase, the address indicates the bank to be erased. 5)For read verify, the address indications follow program/erase listed above. Note the address written to this register will be submitted for translation to the NoWrapper address translation interface, and the translated address will be used to access the bank. However, if the CMDCTL.ADDRXLATEOVR bit is set, then the address written to this register will be used directly as the bank address. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    pub cmdaddr: CMDADDR,
    #[doc = "0x124 - Command Program Byte Enable Register: This register forms a per-byte enable for programming data. For data bytes to be programmed, a 1 must be written to the corresponding bit in this register. Normally, all bits are written to 1, allowing program of full flash words. However, leaving some bits 0 allows programming of 8-bit, 16-bit, 32-bit or 64-bit portions of a flash word. In addtion, the read verify command will ignore data bytes read from the flash in its comparison if the corresponding CMDBYTEN bit is 0. ECC data bytes are protected by the 1-2 MSB bits in this register, depending on the presence of ECC and the flash word data width. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is written to all 0 after the completion of all NoWrapper commands."]
    pub cmdbyten: CMDBYTEN,
    _reserved13: [u8; 0x04],
    #[doc = "0x12c - Command Program Data Index Register: When multiple data registers are available for multi-word program, this register can be written with an index which points to one of the data registers. When a write to CMDDATA* is done, the data will be written to the physical data register indexed by the value in this register. Up to 8 data registers can be present, so this register can be written with 0x0 to 0x7. If less than 8 data registers are present, successive MSB bits of this register are ignored when indexing the CMDDATA* registers. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    pub cmddataindex: CMDDATAINDEX,
    #[doc = "0x130 - Command Data Register 0 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata0: CMDDATA0,
    #[doc = "0x134 - Command Data Register 1 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to CMDSTAT.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata1: CMDDATA1,
    #[doc = "0x138 - Command Data Register 2 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata2: CMDDATA2,
    #[doc = "0x13c - Command Data Register 3 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata3: CMDDATA3,
    #[doc = "0x140 - Command Data Register 4 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 2. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata4: CMDDATA4,
    #[doc = "0x144 - Command Data Register 5 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 2. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata5: CMDDATA5,
    #[doc = "0x148 - Command Data Register 6 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 3. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata6: CMDDATA6,
    #[doc = "0x14c - Command Data Register 7 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 3. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata7: CMDDATA7,
    #[doc = "0x150 - Command Data Register 8 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 4. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata8: CMDDATA8,
    #[doc = "0x154 - Command Data Register 9 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 4. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata9: CMDDATA9,
    #[doc = "0x158 - Command Data Register 10 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 5. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata10: CMDDATA10,
    #[doc = "0x15c - Command Data Register 11 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 5. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata11: CMDDATA11,
    #[doc = "0x160 - Command Data Register 12 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 6. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata12: CMDDATA12,
    #[doc = "0x164 - Command Data Register 13 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 6. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata13: CMDDATA13,
    #[doc = "0x168 - Command Data Register 14 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 7. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata14: CMDDATA14,
    #[doc = "0x16c - Command Data Register 15 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 7. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
    pub cmddata15: CMDDATA15,
    _reserved30: [u8; 0x60],
    #[doc = "0x1d0 - Command WriteErase Protect A Register This register allows the first 32 sectors of the main region to be protected from program or erase, with 1 bit protecting each sector. If the main region size is smaller than 32 sectors, then this register provides protection for the whole region. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
    pub cmdweprota: CMDWEPROTA,
    #[doc = "0x1d4 - Command WriteErase Protect B Register This register allows main region sectors to be protected from program and erase. Each bit corresponds to a group of 8 sectors. There are 3 cases for how these protect bits are applied: 1. Single-bank system: In the case where only a single flash bank is present, the first 32 sectors are protected via the CMDWEPROTA register. Thus, the protection give by the bits in CMDWEPROTB begin with sector 32. 2. Multi-bank system, Bank 0: When multiple flash banks are present, the first 32 sectors of bank 0 are protected via the CMDWEPROTA register. Thus, only bits 4 and above of CMDWEPROTB would be applicable to bank 0. The protection of bit 4 and above would begin at sector 32. Bits 3:0 of WEPROTB are ignored for bank 0. 3. Multi-bank system, Banks 1-N: For banks other than bank 0 in a multi-bank system, CMDWEPROTA has no effect, so the bits in CMDWEPROTB will protect these banks starting from sector 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
    pub cmdweprotb: CMDWEPROTB,
    _reserved32: [u8; 0x38],
    #[doc = "0x210 - Command WriteErase Protect Non-Main Register This register allows non-main region region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
    pub cmdweprotnm: CMDWEPROTNM,
    #[doc = "0x214 - Command WriteErase Protect Trim Register This register allows trim region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
    pub cmdweprottr: CMDWEPROTTR,
    #[doc = "0x218 - Command WriteErase Protect Engr Register This register allows engr region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
    pub cmdweproten: CMDWEPROTEN,
    _reserved35: [u8; 0x0194],
    #[doc = "0x3b0 - Command Configuration Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
    pub cfgcmd: CFGCMD,
    #[doc = "0x3b4 - Pulse Counter Configuration Register This register allows further configuration of maximum pulse counts for program and erase operations. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    pub cfgpcnt: CFGPCNT,
    _reserved37: [u8; 0x18],
    #[doc = "0x3d0 - Command Status Register This register contains status regarding completion and errors of command execution."]
    pub statcmd: STATCMD,
    #[doc = "0x3d4 - Current Address Counter Value Read only register giving read access to the state machine current address. A bank id, region id and address are stored in this register and are incremented as necessary during execution of a command."]
    pub stataddr: STATADDR,
    #[doc = "0x3d8 - Current Pulse Count Register: Read only register giving read access to the state machine current pulse count value for program/erase operations."]
    pub statpcnt: STATPCNT,
    #[doc = "0x3dc - Mode Status Register Indicates any banks which not in READ mode, and it indicates the mode which the bank(s) are in."]
    pub statmode: STATMODE,
    _reserved41: [u8; 0x10],
    #[doc = "0x3f0 - Global Info 0 Register Read only register detailing information about sector size and number of banks present."]
    pub gblinfo0: GBLINFO0,
    #[doc = "0x3f4 - Global Info 1 Register Read only register detailing information about data, ecc and redundant data widths in bits."]
    pub gblinfo1: GBLINFO1,
    #[doc = "0x3f8 - Global Info 2 Register Read only register detailing information about the number of data registers present."]
    pub gblinfo2: GBLINFO2,
    _reserved44: [u8; 0x04],
    #[doc = "0x400 - Bank Info 0 Register for bank 0. Read only register detailing information about Main region size in the bank."]
    pub bank0info0: BANK0INFO0,
    #[doc = "0x404 - Bank Info1 Register for bank 0. Read only register detailing information about Non-Main, Trim, and Engr region sizes in the bank."]
    pub bank0info1: BANK0INFO1,
    _reserved46: [u8; 0x08],
    #[doc = "0x410 - Bank Info 0 Register for bank 1. Read only register detailing information about Main region size in the bank."]
    pub bank1info0: BANK1INFO0,
    #[doc = "0x414 - Bank Info1 Register for bank 1. Read only register detailing information about Non-Main, Trim, and Engr region sizes in the bank."]
    pub bank1info1: BANK1INFO1,
    _reserved48: [u8; 0xe8],
    #[doc = "0x500 - DFT Enable Register Allows control of NoWrapper test features. When set, DFT* registers in this aperture open for write access. When cleared, DFT* registers are read-only. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    pub dften: DFTEN,
    #[doc = "0x504 - DFT Command Control Register This register configures specific capabilities for test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    pub dftcmdctl: DFTCMDCTL,
    #[doc = "0x508 - DFT Timer Control Register This allows some configuration of timing values for various phases of flash operations for test. These time values are hard-coded for functional execution. This register is only writable when DFT.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    pub dfttimerctl: DFTTIMERCTL,
    #[doc = "0x50c - DFT EXECUTEZ control register. This register allows direct control of the EXECUTEZ to bank and pump for test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    pub dftexeczctl: DFTEXECZCTL,
    #[doc = "0x510 - DFT Pump Clock Test Control Register. This register controls hardware features that allow the pump clock to be characterized for trim development. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    pub dftpclktestctl: DFTPCLKTESTCTL,
    #[doc = "0x514 - DFT Pump Clock Test Status Register. This register shows status reported by the hardware features that allow the pump clock to be characterized for trim development. This register is only writable when DFTEN.ENABLE is set."]
    pub dftpclkteststat: DFTPCLKTESTSTAT,
    _reserved54: [u8; 0x28],
    #[doc = "0x540 - DFT Redundancy Data Register 0 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 0. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    pub dftdatared0: DFTDATARED0,
    #[doc = "0x544 - DFT Redundancy Data Register 1 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 1. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    pub dftdatared1: DFTDATARED1,
    #[doc = "0x548 - DFT Redundancy Data Register 2 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 2. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    pub dftdatared2: DFTDATARED2,
    #[doc = "0x54c - DFT Redundancy Data Register 3 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 3. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    pub dftdatared3: DFTDATARED3,
    _reserved58: [u8; 0x10],
    #[doc = "0x560 - DFT Pump Control Register This allows some configuration of pump parameters during test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    pub dftpumpctl: DFTPUMPCTL,
    #[doc = "0x564 - DFT Bank Control Register This allows some configuration of bank parameters during test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
    pub dftbankctl: DFTBANKCTL,
}
#[doc = "IIDX (rw) register accessor: an alias for `Reg<IIDX_SPEC>`"]
pub type IIDX = crate::Reg<iidx::IIDX_SPEC>;
#[doc = "Interrupt Index Register: The IIDX register provides the highest priority enabled interrupt index. PSD compliant register. Note that it is not recommended to use this register if the system clock is running at a slower clock frequency than the NoWrapper clock. If this is the case, then reading this register may fail to update the RIS register correctly. The MIS register should be read directly, and a write to ICLR should be used to clear interrupts when this clock relationship is present."]
pub mod iidx;
#[doc = "IMASK (rw) register accessor: an alias for `Reg<IMASK_SPEC>`"]
pub type IMASK = crate::Reg<imask::IMASK_SPEC>;
#[doc = "Interrupt Mask Register: The IMASK register holds the current interrupt mask settings. Masked interrupts are read in the MIS register. PSD compliant register."]
pub mod imask;
#[doc = "RIS (rw) register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Raw Interrupt Status Register: The RIS register reflects all pending interrupts, regardless of masking. The RIS register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing a 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled. A flag can be set by software by writing a 1 to the ISET register. Reading the IIDX register will also clear the corresponding bit in RIS. PSD compliant register."]
pub mod ris;
#[doc = "MIS (rw) register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Masked Interrupt Status Register: The MIS register is a bit-wise AND of the contents of the IMASK and RIS registers. This is kept mainly for ARM compatibility, and has limited use since the highest priority interrupt index is returned via the IIDX register. PSD compliant register."]
pub mod mis;
#[doc = "ISET (rw) register accessor: an alias for `Reg<ISET_SPEC>`"]
pub type ISET = crate::Reg<iset::ISET_SPEC>;
#[doc = "Interrupt Set Register: The ISET register allows software to write a 1 to set corresponding interrupt. Safety: This meets a safety requirement to allow software diagnostics to trigger interrupts. PSD compliant register."]
pub mod iset;
#[doc = "ICLR (rw) register accessor: an alias for `Reg<ICLR_SPEC>`"]
pub type ICLR = crate::Reg<iclr::ICLR_SPEC>;
#[doc = "Interrupt Clear Register. The ICLR register allows allows software to write a 1 to clear corresponding interrupt. PSD compliant register."]
pub mod iclr;
#[doc = "EVT_MODE (rw) register accessor: an alias for `Reg<EVT_MODE_SPEC>`"]
pub type EVT_MODE = crate::Reg<evt_mode::EVT_MODE_SPEC>;
#[doc = "Event mode register. It is used to select whether each line is disabled, in software mode (software clears the RIS) or in hardware mode (hardware clears the RIS)."]
pub mod evt_mode;
#[doc = "DESC (rw) register accessor: an alias for `Reg<DESC_SPEC>`"]
pub type DESC = crate::Reg<desc::DESC_SPEC>;
#[doc = "Hardware Version Description Register: This register identifies the NoWrapper hardware version and feature set used."]
pub mod desc;
#[doc = "CMDEXEC (rw) register accessor: an alias for `Reg<CMDEXEC_SPEC>`"]
pub type CMDEXEC = crate::Reg<cmdexec::CMDEXEC_SPEC>;
#[doc = "Command Execute Register: Initiates execution of the command specified in the CMDTYPE register. This register is blocked for writes after being written to 1 and prior to STATCMD.DONE being set by the NoWrapper hardware. NoWrapper hardware clears this register after the processing of the command has completed."]
pub mod cmdexec;
#[doc = "CMDTYPE (rw) register accessor: an alias for `Reg<CMDTYPE_SPEC>`"]
pub type CMDTYPE = crate::Reg<cmdtype::CMDTYPE_SPEC>;
#[doc = "Command Type Register This register specifies the type of command to be executed by the NoWrapper hardware. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
pub mod cmdtype;
#[doc = "CMDCTL (rw) register accessor: an alias for `Reg<CMDCTL_SPEC>`"]
pub type CMDCTL = crate::Reg<cmdctl::CMDCTL_SPEC>;
#[doc = "Command Control Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
pub mod cmdctl;
#[doc = "CMDADDR (rw) register accessor: an alias for `Reg<CMDADDR_SPEC>`"]
pub type CMDADDR = crate::Reg<cmdaddr::CMDADDR_SPEC>;
#[doc = "Command Address Register: This register forms the target address of a command. The use cases are as follows: 1)For single-word program, this address indicates the flash bank word to be programmed. 2)For multi-word program, this address indicates the first flash bank address for the program. The address will be incremented for further words. 3)For sector erase, this address indicates the sector to be erased. 4)For bank erase, the address indicates the bank to be erased. 5)For read verify, the address indications follow program/erase listed above. Note the address written to this register will be submitted for translation to the NoWrapper address translation interface, and the translated address will be used to access the bank. However, if the CMDCTL.ADDRXLATEOVR bit is set, then the address written to this register will be used directly as the bank address. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod cmdaddr;
#[doc = "CMDBYTEN (rw) register accessor: an alias for `Reg<CMDBYTEN_SPEC>`"]
pub type CMDBYTEN = crate::Reg<cmdbyten::CMDBYTEN_SPEC>;
#[doc = "Command Program Byte Enable Register: This register forms a per-byte enable for programming data. For data bytes to be programmed, a 1 must be written to the corresponding bit in this register. Normally, all bits are written to 1, allowing program of full flash words. However, leaving some bits 0 allows programming of 8-bit, 16-bit, 32-bit or 64-bit portions of a flash word. In addtion, the read verify command will ignore data bytes read from the flash in its comparison if the corresponding CMDBYTEN bit is 0. ECC data bytes are protected by the 1-2 MSB bits in this register, depending on the presence of ECC and the flash word data width. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is written to all 0 after the completion of all NoWrapper commands."]
pub mod cmdbyten;
#[doc = "CMDDATAINDEX (rw) register accessor: an alias for `Reg<CMDDATAINDEX_SPEC>`"]
pub type CMDDATAINDEX = crate::Reg<cmddataindex::CMDDATAINDEX_SPEC>;
#[doc = "Command Program Data Index Register: When multiple data registers are available for multi-word program, this register can be written with an index which points to one of the data registers. When a write to CMDDATA* is done, the data will be written to the physical data register indexed by the value in this register. Up to 8 data registers can be present, so this register can be written with 0x0 to 0x7. If less than 8 data registers are present, successive MSB bits of this register are ignored when indexing the CMDDATA* registers. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod cmddataindex;
#[doc = "CMDDATA0 (rw) register accessor: an alias for `Reg<CMDDATA0_SPEC>`"]
pub type CMDDATA0 = crate::Reg<cmddata0::CMDDATA0_SPEC>;
#[doc = "Command Data Register 0 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata0;
#[doc = "CMDDATA1 (rw) register accessor: an alias for `Reg<CMDDATA1_SPEC>`"]
pub type CMDDATA1 = crate::Reg<cmddata1::CMDDATA1_SPEC>;
#[doc = "Command Data Register 1 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to CMDSTAT.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata1;
#[doc = "CMDDATA2 (rw) register accessor: an alias for `Reg<CMDDATA2_SPEC>`"]
pub type CMDDATA2 = crate::Reg<cmddata2::CMDDATA2_SPEC>;
#[doc = "Command Data Register 2 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata2;
#[doc = "CMDDATA3 (rw) register accessor: an alias for `Reg<CMDDATA3_SPEC>`"]
pub type CMDDATA3 = crate::Reg<cmddata3::CMDDATA3_SPEC>;
#[doc = "Command Data Register 3 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 0. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 1. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata3;
#[doc = "CMDDATA4 (rw) register accessor: an alias for `Reg<CMDDATA4_SPEC>`"]
pub type CMDDATA4 = crate::Reg<cmddata4::CMDDATA4_SPEC>;
#[doc = "Command Data Register 4 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 2. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata4;
#[doc = "CMDDATA5 (rw) register accessor: an alias for `Reg<CMDDATA5_SPEC>`"]
pub type CMDDATA5 = crate::Reg<cmddata5::CMDDATA5_SPEC>;
#[doc = "Command Data Register 5 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 2. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata5;
#[doc = "CMDDATA6 (rw) register accessor: an alias for `Reg<CMDDATA6_SPEC>`"]
pub type CMDDATA6 = crate::Reg<cmddata6::CMDDATA6_SPEC>;
#[doc = "Command Data Register 6 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 3. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata6;
#[doc = "CMDDATA7 (rw) register accessor: an alias for `Reg<CMDDATA7_SPEC>`"]
pub type CMDDATA7 = crate::Reg<cmddata7::CMDDATA7_SPEC>;
#[doc = "Command Data Register 7 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 1. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 3. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata7;
#[doc = "CMDDATA8 (rw) register accessor: an alias for `Reg<CMDDATA8_SPEC>`"]
pub type CMDDATA8 = crate::Reg<cmddata8::CMDDATA8_SPEC>;
#[doc = "Command Data Register 8 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 4. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata8;
#[doc = "CMDDATA9 (rw) register accessor: an alias for `Reg<CMDDATA9_SPEC>`"]
pub type CMDDATA9 = crate::Reg<cmddata9::CMDDATA9_SPEC>;
#[doc = "Command Data Register 9 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 4. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata9;
#[doc = "CMDDATA10 (rw) register accessor: an alias for `Reg<CMDDATA10_SPEC>`"]
pub type CMDDATA10 = crate::Reg<cmddata10::CMDDATA10_SPEC>;
#[doc = "Command Data Register 10 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 5. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata10;
#[doc = "CMDDATA11 (rw) register accessor: an alias for `Reg<CMDDATA11_SPEC>`"]
pub type CMDDATA11 = crate::Reg<cmddata11::CMDDATA11_SPEC>;
#[doc = "Command Data Register 11 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 2. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 5. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata11;
#[doc = "CMDDATA12 (rw) register accessor: an alias for `Reg<CMDDATA12_SPEC>`"]
pub type CMDDATA12 = crate::Reg<cmddata12::CMDDATA12_SPEC>;
#[doc = "Command Data Register 12 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 31:0 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 6. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata12;
#[doc = "CMDDATA13 (rw) register accessor: an alias for `Reg<CMDDATA13_SPEC>`"]
pub type CMDDATA13 = crate::Reg<cmddata13::CMDDATA13_SPEC>;
#[doc = "Command Data Register 13 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 63:32 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 6. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata13;
#[doc = "CMDDATA14 (rw) register accessor: an alias for `Reg<CMDDATA14_SPEC>`"]
pub type CMDDATA14 = crate::Reg<cmddata14::CMDDATA14_SPEC>;
#[doc = "Command Data Register 14 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 95:64 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 31:0 of flash word data register 7. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata14;
#[doc = "CMDDATA15 (rw) register accessor: an alias for `Reg<CMDDATA15_SPEC>`"]
pub type CMDDATA15 = crate::Reg<cmddata15::CMDDATA15_SPEC>;
#[doc = "Command Data Register 15 This register forms the data for a command. For DATAWIDTH == 128:This register represents bits 127:96 of flash word data register 3. For DATAWIDTH == 64:This register represents bits 63:32 of flash word data register 7. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. This register is used to aggregate masking for bits that do not require additional program pulses during program operations, and will be written to all 1 after the completion of all NoWrapper commands. Use cases for the CMDDATA* registers are as follows: 1)Program - These registers contain the data to be programmed. 2)Erase - These registers are not used. 3)Read Verify - These registers contain data to be verified."]
pub mod cmddata15;
#[doc = "CMDWEPROTA (rw) register accessor: an alias for `Reg<CMDWEPROTA_SPEC>`"]
pub type CMDWEPROTA = crate::Reg<cmdweprota::CMDWEPROTA_SPEC>;
#[doc = "Command WriteErase Protect A Register This register allows the first 32 sectors of the main region to be protected from program or erase, with 1 bit protecting each sector. If the main region size is smaller than 32 sectors, then this register provides protection for the whole region. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
pub mod cmdweprota;
#[doc = "CMDWEPROTB (rw) register accessor: an alias for `Reg<CMDWEPROTB_SPEC>`"]
pub type CMDWEPROTB = crate::Reg<cmdweprotb::CMDWEPROTB_SPEC>;
#[doc = "Command WriteErase Protect B Register This register allows main region sectors to be protected from program and erase. Each bit corresponds to a group of 8 sectors. There are 3 cases for how these protect bits are applied: 1. Single-bank system: In the case where only a single flash bank is present, the first 32 sectors are protected via the CMDWEPROTA register. Thus, the protection give by the bits in CMDWEPROTB begin with sector 32. 2. Multi-bank system, Bank 0: When multiple flash banks are present, the first 32 sectors of bank 0 are protected via the CMDWEPROTA register. Thus, only bits 4 and above of CMDWEPROTB would be applicable to bank 0. The protection of bit 4 and above would begin at sector 32. Bits 3:0 of WEPROTB are ignored for bank 0. 3. Multi-bank system, Banks 1-N: For banks other than bank 0 in a multi-bank system, CMDWEPROTA has no effect, so the bits in CMDWEPROTB will protect these banks starting from sector 0. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
pub mod cmdweprotb;
#[doc = "CMDWEPROTNM (rw) register accessor: an alias for `Reg<CMDWEPROTNM_SPEC>`"]
pub type CMDWEPROTNM = crate::Reg<cmdweprotnm::CMDWEPROTNM_SPEC>;
#[doc = "Command WriteErase Protect Non-Main Register This register allows non-main region region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
pub mod cmdweprotnm;
#[doc = "CMDWEPROTTR (rw) register accessor: an alias for `Reg<CMDWEPROTTR_SPEC>`"]
pub type CMDWEPROTTR = crate::Reg<cmdweprottr::CMDWEPROTTR_SPEC>;
#[doc = "Command WriteErase Protect Trim Register This register allows trim region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
pub mod cmdweprottr;
#[doc = "CMDWEPROTEN (rw) register accessor: an alias for `Reg<CMDWEPROTEN_SPEC>`"]
pub type CMDWEPROTEN = crate::Reg<cmdweproten::CMDWEPROTEN_SPEC>;
#[doc = "Command WriteErase Protect Engr Register This register allows engr region sectors to be protected from program and erase. Each bit corresponds to 1 sector. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware. In addition, this register is used to aggregate masking for sectors that do not require additional erase pulses during bank erase operations, and will be written to all 1 after the completion of all NoWrapper commands."]
pub mod cmdweproten;
#[doc = "CFGCMD (rw) register accessor: an alias for `Reg<CFGCMD_SPEC>`"]
pub type CFGCMD = crate::Reg<cfgcmd::CFGCMD_SPEC>;
#[doc = "Command Configuration Register This register configures specific capabilities of the state machine for related to the execution of a command. This register is blocked for writes after CMDEXEC is written to a 1 and prior to STATCMD.DONE being set by the hardware to indicate that command execution has completed."]
pub mod cfgcmd;
#[doc = "CFGPCNT (rw) register accessor: an alias for `Reg<CFGPCNT_SPEC>`"]
pub type CFGPCNT = crate::Reg<cfgpcnt::CFGPCNT_SPEC>;
#[doc = "Pulse Counter Configuration Register This register allows further configuration of maximum pulse counts for program and erase operations. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod cfgpcnt;
#[doc = "STATCMD (rw) register accessor: an alias for `Reg<STATCMD_SPEC>`"]
pub type STATCMD = crate::Reg<statcmd::STATCMD_SPEC>;
#[doc = "Command Status Register This register contains status regarding completion and errors of command execution."]
pub mod statcmd;
#[doc = "STATADDR (rw) register accessor: an alias for `Reg<STATADDR_SPEC>`"]
pub type STATADDR = crate::Reg<stataddr::STATADDR_SPEC>;
#[doc = "Current Address Counter Value Read only register giving read access to the state machine current address. A bank id, region id and address are stored in this register and are incremented as necessary during execution of a command."]
pub mod stataddr;
#[doc = "STATPCNT (rw) register accessor: an alias for `Reg<STATPCNT_SPEC>`"]
pub type STATPCNT = crate::Reg<statpcnt::STATPCNT_SPEC>;
#[doc = "Current Pulse Count Register: Read only register giving read access to the state machine current pulse count value for program/erase operations."]
pub mod statpcnt;
#[doc = "STATMODE (rw) register accessor: an alias for `Reg<STATMODE_SPEC>`"]
pub type STATMODE = crate::Reg<statmode::STATMODE_SPEC>;
#[doc = "Mode Status Register Indicates any banks which not in READ mode, and it indicates the mode which the bank(s) are in."]
pub mod statmode;
#[doc = "GBLINFO0 (rw) register accessor: an alias for `Reg<GBLINFO0_SPEC>`"]
pub type GBLINFO0 = crate::Reg<gblinfo0::GBLINFO0_SPEC>;
#[doc = "Global Info 0 Register Read only register detailing information about sector size and number of banks present."]
pub mod gblinfo0;
#[doc = "GBLINFO1 (rw) register accessor: an alias for `Reg<GBLINFO1_SPEC>`"]
pub type GBLINFO1 = crate::Reg<gblinfo1::GBLINFO1_SPEC>;
#[doc = "Global Info 1 Register Read only register detailing information about data, ecc and redundant data widths in bits."]
pub mod gblinfo1;
#[doc = "GBLINFO2 (rw) register accessor: an alias for `Reg<GBLINFO2_SPEC>`"]
pub type GBLINFO2 = crate::Reg<gblinfo2::GBLINFO2_SPEC>;
#[doc = "Global Info 2 Register Read only register detailing information about the number of data registers present."]
pub mod gblinfo2;
#[doc = "BANK0INFO0 (rw) register accessor: an alias for `Reg<BANK0INFO0_SPEC>`"]
pub type BANK0INFO0 = crate::Reg<bank0info0::BANK0INFO0_SPEC>;
#[doc = "Bank Info 0 Register for bank 0. Read only register detailing information about Main region size in the bank."]
pub mod bank0info0;
#[doc = "BANK0INFO1 (rw) register accessor: an alias for `Reg<BANK0INFO1_SPEC>`"]
pub type BANK0INFO1 = crate::Reg<bank0info1::BANK0INFO1_SPEC>;
#[doc = "Bank Info1 Register for bank 0. Read only register detailing information about Non-Main, Trim, and Engr region sizes in the bank."]
pub mod bank0info1;
#[doc = "BANK1INFO0 (rw) register accessor: an alias for `Reg<BANK1INFO0_SPEC>`"]
pub type BANK1INFO0 = crate::Reg<bank1info0::BANK1INFO0_SPEC>;
#[doc = "Bank Info 0 Register for bank 1. Read only register detailing information about Main region size in the bank."]
pub mod bank1info0;
#[doc = "BANK1INFO1 (rw) register accessor: an alias for `Reg<BANK1INFO1_SPEC>`"]
pub type BANK1INFO1 = crate::Reg<bank1info1::BANK1INFO1_SPEC>;
#[doc = "Bank Info1 Register for bank 1. Read only register detailing information about Non-Main, Trim, and Engr region sizes in the bank."]
pub mod bank1info1;
#[doc = "DFTEN (rw) register accessor: an alias for `Reg<DFTEN_SPEC>`"]
pub type DFTEN = crate::Reg<dften::DFTEN_SPEC>;
#[doc = "DFT Enable Register Allows control of NoWrapper test features. When set, DFT* registers in this aperture open for write access. When cleared, DFT* registers are read-only. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dften;
#[doc = "DFTCMDCTL (rw) register accessor: an alias for `Reg<DFTCMDCTL_SPEC>`"]
pub type DFTCMDCTL = crate::Reg<dftcmdctl::DFTCMDCTL_SPEC>;
#[doc = "DFT Command Control Register This register configures specific capabilities for test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftcmdctl;
#[doc = "DFTTIMERCTL (rw) register accessor: an alias for `Reg<DFTTIMERCTL_SPEC>`"]
pub type DFTTIMERCTL = crate::Reg<dfttimerctl::DFTTIMERCTL_SPEC>;
#[doc = "DFT Timer Control Register This allows some configuration of timing values for various phases of flash operations for test. These time values are hard-coded for functional execution. This register is only writable when DFT.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dfttimerctl;
#[doc = "DFTEXECZCTL (rw) register accessor: an alias for `Reg<DFTEXECZCTL_SPEC>`"]
pub type DFTEXECZCTL = crate::Reg<dftexeczctl::DFTEXECZCTL_SPEC>;
#[doc = "DFT EXECUTEZ control register. This register allows direct control of the EXECUTEZ to bank and pump for test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftexeczctl;
#[doc = "DFTPCLKTESTCTL (rw) register accessor: an alias for `Reg<DFTPCLKTESTCTL_SPEC>`"]
pub type DFTPCLKTESTCTL = crate::Reg<dftpclktestctl::DFTPCLKTESTCTL_SPEC>;
#[doc = "DFT Pump Clock Test Control Register. This register controls hardware features that allow the pump clock to be characterized for trim development. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftpclktestctl;
#[doc = "DFTPCLKTESTSTAT (rw) register accessor: an alias for `Reg<DFTPCLKTESTSTAT_SPEC>`"]
pub type DFTPCLKTESTSTAT = crate::Reg<dftpclkteststat::DFTPCLKTESTSTAT_SPEC>;
#[doc = "DFT Pump Clock Test Status Register. This register shows status reported by the hardware features that allow the pump clock to be characterized for trim development. This register is only writable when DFTEN.ENABLE is set."]
pub mod dftpclkteststat;
#[doc = "DFTDATARED0 (rw) register accessor: an alias for `Reg<DFTDATARED0_SPEC>`"]
pub type DFTDATARED0 = crate::Reg<dftdatared0::DFTDATARED0_SPEC>;
#[doc = "DFT Redundancy Data Register 0 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 0. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftdatared0;
#[doc = "DFTDATARED1 (rw) register accessor: an alias for `Reg<DFTDATARED1_SPEC>`"]
pub type DFTDATARED1 = crate::Reg<dftdatared1::DFTDATARED1_SPEC>;
#[doc = "DFT Redundancy Data Register 1 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 1. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftdatared1;
#[doc = "DFTDATARED2 (rw) register accessor: an alias for `Reg<DFTDATARED2_SPEC>`"]
pub type DFTDATARED2 = crate::Reg<dftdatared2::DFTDATARED2_SPEC>;
#[doc = "DFT Redundancy Data Register 2 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 2. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftdatared2;
#[doc = "DFTDATARED3 (rw) register accessor: an alias for `Reg<DFTDATARED3_SPEC>`"]
pub type DFTDATARED3 = crate::Reg<dftdatared3::DFTDATARED3_SPEC>;
#[doc = "DFT Redundancy Data Register 3 This register is used when testing the redundant columns in the flash. It acts as an extension of the CMDDATA* registers. The bits in this register correspond to flash data word register 3. In addition, this register is used to aggregate masking for bits that do not require additional program pulses during program operations. The original data written to this register will be lost during program command execution. Use cases for this register are as follows: 1)Program - Contains the data to be programmed. 2)Erase - Not used. 3)Read Verify - Contains data to be verified. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftdatared3;
#[doc = "DFTPUMPCTL (rw) register accessor: an alias for `Reg<DFTPUMPCTL_SPEC>`"]
pub type DFTPUMPCTL = crate::Reg<dftpumpctl::DFTPUMPCTL_SPEC>;
#[doc = "DFT Pump Control Register This allows some configuration of pump parameters during test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftpumpctl;
#[doc = "DFTBANKCTL (rw) register accessor: an alias for `Reg<DFTBANKCTL_SPEC>`"]
pub type DFTBANKCTL = crate::Reg<dftbankctl::DFTBANKCTL_SPEC>;
#[doc = "DFT Bank Control Register This allows some configuration of bank parameters during test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware."]
pub mod dftbankctl;
