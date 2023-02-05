#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel 0 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
    pub dmach0ctl: DMACH0CTL,
    #[doc = "0x04 - Channel 0 External Address"]
    pub dmach0extaddr: DMACH0EXTADDR,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Channel 0 DMA Length"]
    pub dmach0len: DMACH0LEN,
    _reserved3: [u8; 0x08],
    #[doc = "0x18 - DMAC Status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer."]
    pub dmastat: DMASTAT,
    #[doc = "0x1c - DMAC Software Reset Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMASTAT."]
    pub dmaswreset: DMASWRESET,
    #[doc = "0x20 - Channel 1 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
    pub dmach1ctl: DMACH1CTL,
    #[doc = "0x24 - Channel 1 External Address"]
    pub dmach1extaddr: DMACH1EXTADDR,
    _reserved7: [u8; 0x04],
    #[doc = "0x2c - Channel 1 DMA Length"]
    pub dmach1len: DMACH1LEN,
    _reserved8: [u8; 0x48],
    #[doc = "0x78 - DMAC Master Run-time Parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter."]
    pub dmabuscfg: DMABUSCFG,
    #[doc = "0x7c - DMAC Port Error Raw Status This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMASWRESET register."]
    pub dmaporterr: DMAPORTERR,
    _reserved10: [u8; 0x7c],
    #[doc = "0xfc - DMAC Version This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers."]
    pub dmahwver: DMAHWVER,
    _reserved11: [u8; 0x0300],
    #[doc = "0x400 - Key Store Write Area This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine."]
    pub keywritearea: KEYWRITEAREA,
    #[doc = "0x404 - Key Store Written Area This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error."]
    pub keywrittenarea: KEYWRITTENAREA,
    #[doc = "0x408 - Key Store Size This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register."]
    pub keysize: KEYSIZE,
    #[doc = "0x40c - Key Store Read Area This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key."]
    pub keyreadarea: KEYREADAREA,
    _reserved15: [u8; 0xf0],
    #[doc = "0x500 - AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aeskey2: AESKEY2,
    _reserved16: [u8; 0x0c],
    #[doc = "0x510 - AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aeskey3: AESKEY3,
    _reserved17: [u8; 0x2c],
    #[doc = "0x540 - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    pub aesiv: AESIV,
    _reserved18: [u8; 0x0c],
    #[doc = "0x550 - AES Control AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0."]
    pub aesctl: AESCTL,
    #[doc = "0x554 - AES Crypto Length 0 (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
    pub aesdatalen0: AESDATALEN0,
    #[doc = "0x558 - AES Crypto Length 1 (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
    pub aesdatalen1: AESDATALEN1,
    #[doc = "0x55c - AES Authentication Length"]
    pub aesauthlen: AESAUTHLEN,
    _reserved_22_aesdatain0: [u8; 0x04],
    _reserved_23_aesdatain1: [u8; 0x04],
    _reserved_24_aesdatain2: [u8; 0x04],
    _reserved_25_aesdatain3: [u8; 0x04],
    #[doc = "0x570 - AES Tag Out 0 The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice."]
    pub aestagout: AESTAGOUT,
    _reserved27: [u8; 0x90],
    #[doc = "0x604 - HASH Data Input 1 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain1: HASHDATAIN1,
    #[doc = "0x608 - HASH Data Input 2 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain2: HASHDATAIN2,
    #[doc = "0x60c - HASH Data Input 3 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain3: HASHDATAIN3,
    #[doc = "0x610 - HASH Data Input 4 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain4: HASHDATAIN4,
    #[doc = "0x614 - HASH Data Input 5 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain5: HASHDATAIN5,
    #[doc = "0x618 - HASH Data Input 6 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain6: HASHDATAIN6,
    #[doc = "0x61c - HASH Data Input 7 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain7: HASHDATAIN7,
    #[doc = "0x620 - HASH Data Input 8 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain8: HASHDATAIN8,
    #[doc = "0x624 - HASH Data Input 9 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain9: HASHDATAIN9,
    #[doc = "0x628 - HASH Data Input 10 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain10: HASHDATAIN10,
    #[doc = "0x62c - HASH Data Input 11 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain11: HASHDATAIN11,
    #[doc = "0x630 - HASH Data Input 12 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain12: HASHDATAIN12,
    #[doc = "0x634 - HASH Data Input 13 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain13: HASHDATAIN13,
    #[doc = "0x638 - HASH Data Input 14 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain14: HASHDATAIN14,
    #[doc = "0x63c - HASH Data Input 15 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain15: HASHDATAIN15,
    #[doc = "0x640 - HASH Data Input 16 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain16: HASHDATAIN16,
    #[doc = "0x644 - HASH Data Input 17 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain17: HASHDATAIN17,
    #[doc = "0x648 - HASH Data Input 18 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain18: HASHDATAIN18,
    #[doc = "0x64c - HASH Data Input 19 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain19: HASHDATAIN19,
    #[doc = "0x650 - HASH Data Input 20 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain20: HASHDATAIN20,
    #[doc = "0x654 - HASH Data Input 21 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain21: HASHDATAIN21,
    #[doc = "0x658 - HASH Data Input 22 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain22: HASHDATAIN22,
    #[doc = "0x65c - HASH Data Input 23 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain23: HASHDATAIN23,
    #[doc = "0x660 - HASH Data Input 24 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain24: HASHDATAIN24,
    #[doc = "0x664 - HASH Data Input 25 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain25: HASHDATAIN25,
    #[doc = "0x668 - HASH Data Input 26 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain26: HASHDATAIN26,
    #[doc = "0x66c - HASH Data Input 27 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain27: HASHDATAIN27,
    #[doc = "0x670 - HASH Data Input 28 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain28: HASHDATAIN28,
    #[doc = "0x674 - HASH Data Input 29 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain29: HASHDATAIN29,
    #[doc = "0x678 - HASH Data Input 30 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain30: HASHDATAIN30,
    #[doc = "0x67c - HASH Data Input 31 The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hashdatain31: HASHDATAIN31,
    #[doc = "0x680 - HASH Input_Output Buffer Control This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine."]
    pub hashiobufctrl: HASHIOBUFCTRL,
    #[doc = "0x684 - HASH Mode"]
    pub hashmode: HASHMODE,
    #[doc = "0x688 - HASH Input Length LSB"]
    pub hashinlenl: HASHINLENL,
    #[doc = "0x68c - HASH Input Length MSB"]
    pub hashinlenh: HASHINLENH,
    _reserved62: [u8; 0x30],
    #[doc = "0x6c0 - HASH Digest A The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigesta: HASHDIGESTA,
    #[doc = "0x6c4 - HASH Digest B The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestb: HASHDIGESTB,
    #[doc = "0x6c8 - HASH Digest C The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestc: HASHDIGESTC,
    #[doc = "0x6cc - HASH Digest D The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestd: HASHDIGESTD,
    #[doc = "0x6d0 - HASH Digest E The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigeste: HASHDIGESTE,
    #[doc = "0x6d4 - HASH Digest F The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestf: HASHDIGESTF,
    #[doc = "0x6d8 - HASH Digest G The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestg: HASHDIGESTG,
    #[doc = "0x6dc - HASH Digest H The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigesth: HASHDIGESTH,
    #[doc = "0x6e0 - HASH Digest I The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigesti: HASHDIGESTI,
    #[doc = "0x6e4 - HASH Digest J The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestj: HASHDIGESTJ,
    #[doc = "0x6e8 - HASH Digest K The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestk: HASHDIGESTK,
    #[doc = "0x6ec - HASH Digest L The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestl: HASHDIGESTL,
    #[doc = "0x6f0 - HASH Digest M The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestm: HASHDIGESTM,
    #[doc = "0x6f4 - HASH Digest N The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestn: HASHDIGESTN,
    #[doc = "0x6f8 - HASH Digest 0 The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigesto: HASHDIGESTO,
    #[doc = "0x6fc - HASH Digest P The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hashdigestp: HASHDIGESTP,
    #[doc = "0x700 - Algorithm Select This algorithm selection register configures the internal destination of the DMA controller."]
    pub algsel: ALGSEL,
    #[doc = "0x704 - DMA Protection Control Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module."]
    pub dmaprotctl: DMAPROTCTL,
    _reserved80: [u8; 0x38],
    #[doc = "0x740 - Software Reset"]
    pub swreset: SWRESET,
    _reserved81: [u8; 0x3c],
    #[doc = "0x780 - Control Interrupt Configuration"]
    pub irqtype: IRQTYPE,
    #[doc = "0x784 - Control Interrupt Enable"]
    pub irqen: IRQEN,
    #[doc = "0x788 - Control Interrupt Clear"]
    pub irqclr: IRQCLR,
    #[doc = "0x78c - Control Interrupt Set"]
    pub irqset: IRQSET,
    #[doc = "0x790 - Control Interrupt Status"]
    pub irqstat: IRQSTAT,
    _reserved86: [u8; 0x68],
    #[doc = "0x7fc - Hardware Version"]
    pub hwver: HWVER,
}
impl RegisterBlock {
    #[doc = "0x560 - AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub const fn aesdatain0(&self) -> &AESDATAIN0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376usize).cast() }
    }
    #[doc = "0x560 - Data Input/Output"]
    #[inline(always)]
    pub const fn aesdataout0(&self) -> &AESDATAOUT0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376usize).cast() }
    }
    #[doc = "0x564 - AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub const fn aesdatain1(&self) -> &AESDATAIN1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1380usize).cast() }
    }
    #[doc = "0x564 - Data Input/Output"]
    #[inline(always)]
    pub const fn aesdataout1(&self) -> &AESDATAOUT1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1380usize).cast() }
    }
    #[doc = "0x568 - AES Data Input_Output 2 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub const fn aesdatain2(&self) -> &AESDATAIN2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1384usize).cast() }
    }
    #[doc = "0x568 - Data Input/Output"]
    #[inline(always)]
    pub const fn aesdataout2(&self) -> &AESDATAOUT2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1384usize).cast() }
    }
    #[doc = "0x56c - AES Data Input_Output 3 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub const fn aesdatain3(&self) -> &AESDATAIN3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1388usize).cast() }
    }
    #[doc = "0x56c - Data Input/Output"]
    #[inline(always)]
    pub const fn aesdataout3(&self) -> &AESDATAOUT3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1388usize).cast() }
    }
}
#[doc = "DMACH0CTL (rw) register accessor: an alias for `Reg<DMACH0CTL_SPEC>`"]
pub type DMACH0CTL = crate::Reg<dmach0ctl::DMACH0CTL_SPEC>;
#[doc = "Channel 0 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub mod dmach0ctl;
#[doc = "DMACH0EXTADDR (rw) register accessor: an alias for `Reg<DMACH0EXTADDR_SPEC>`"]
pub type DMACH0EXTADDR = crate::Reg<dmach0extaddr::DMACH0EXTADDR_SPEC>;
#[doc = "Channel 0 External Address"]
pub mod dmach0extaddr;
#[doc = "DMACH0LEN (rw) register accessor: an alias for `Reg<DMACH0LEN_SPEC>`"]
pub type DMACH0LEN = crate::Reg<dmach0len::DMACH0LEN_SPEC>;
#[doc = "Channel 0 DMA Length"]
pub mod dmach0len;
#[doc = "DMASTAT (rw) register accessor: an alias for `Reg<DMASTAT_SPEC>`"]
pub type DMASTAT = crate::Reg<dmastat::DMASTAT_SPEC>;
#[doc = "DMAC Status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer."]
pub mod dmastat;
#[doc = "DMASWRESET (rw) register accessor: an alias for `Reg<DMASWRESET_SPEC>`"]
pub type DMASWRESET = crate::Reg<dmaswreset::DMASWRESET_SPEC>;
#[doc = "DMAC Software Reset Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMASTAT."]
pub mod dmaswreset;
#[doc = "DMACH1CTL (rw) register accessor: an alias for `Reg<DMACH1CTL_SPEC>`"]
pub type DMACH1CTL = crate::Reg<dmach1ctl::DMACH1CTL_SPEC>;
#[doc = "Channel 1 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub mod dmach1ctl;
#[doc = "DMACH1EXTADDR (rw) register accessor: an alias for `Reg<DMACH1EXTADDR_SPEC>`"]
pub type DMACH1EXTADDR = crate::Reg<dmach1extaddr::DMACH1EXTADDR_SPEC>;
#[doc = "Channel 1 External Address"]
pub mod dmach1extaddr;
#[doc = "DMACH1LEN (rw) register accessor: an alias for `Reg<DMACH1LEN_SPEC>`"]
pub type DMACH1LEN = crate::Reg<dmach1len::DMACH1LEN_SPEC>;
#[doc = "Channel 1 DMA Length"]
pub mod dmach1len;
#[doc = "DMABUSCFG (rw) register accessor: an alias for `Reg<DMABUSCFG_SPEC>`"]
pub type DMABUSCFG = crate::Reg<dmabuscfg::DMABUSCFG_SPEC>;
#[doc = "DMAC Master Run-time Parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter."]
pub mod dmabuscfg;
#[doc = "DMAPORTERR (rw) register accessor: an alias for `Reg<DMAPORTERR_SPEC>`"]
pub type DMAPORTERR = crate::Reg<dmaporterr::DMAPORTERR_SPEC>;
#[doc = "DMAC Port Error Raw Status This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMASWRESET register."]
pub mod dmaporterr;
#[doc = "DMAHWVER (rw) register accessor: an alias for `Reg<DMAHWVER_SPEC>`"]
pub type DMAHWVER = crate::Reg<dmahwver::DMAHWVER_SPEC>;
#[doc = "DMAC Version This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers."]
pub mod dmahwver;
#[doc = "KEYWRITEAREA (rw) register accessor: an alias for `Reg<KEYWRITEAREA_SPEC>`"]
pub type KEYWRITEAREA = crate::Reg<keywritearea::KEYWRITEAREA_SPEC>;
#[doc = "Key Store Write Area This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine."]
pub mod keywritearea;
#[doc = "KEYWRITTENAREA (rw) register accessor: an alias for `Reg<KEYWRITTENAREA_SPEC>`"]
pub type KEYWRITTENAREA = crate::Reg<keywrittenarea::KEYWRITTENAREA_SPEC>;
#[doc = "Key Store Written Area This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error."]
pub mod keywrittenarea;
#[doc = "KEYSIZE (rw) register accessor: an alias for `Reg<KEYSIZE_SPEC>`"]
pub type KEYSIZE = crate::Reg<keysize::KEYSIZE_SPEC>;
#[doc = "Key Store Size This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register."]
pub mod keysize;
#[doc = "KEYREADAREA (rw) register accessor: an alias for `Reg<KEYREADAREA_SPEC>`"]
pub type KEYREADAREA = crate::Reg<keyreadarea::KEYREADAREA_SPEC>;
#[doc = "Key Store Read Area This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key."]
pub mod keyreadarea;
#[doc = "AESKEY2 (rw) register accessor: an alias for `Reg<AESKEY2_SPEC>`"]
pub type AESKEY2 = crate::Reg<aeskey2::AESKEY2_SPEC>;
#[doc = "AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aeskey2;
#[doc = "AESKEY3 (rw) register accessor: an alias for `Reg<AESKEY3_SPEC>`"]
pub type AESKEY3 = crate::Reg<aeskey3::AESKEY3_SPEC>;
#[doc = "AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aeskey3;
#[doc = "AESIV (rw) register accessor: an alias for `Reg<AESIV_SPEC>`"]
pub type AESIV = crate::Reg<aesiv::AESIV_SPEC>;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aesiv;
#[doc = "AESCTL (rw) register accessor: an alias for `Reg<AESCTL_SPEC>`"]
pub type AESCTL = crate::Reg<aesctl::AESCTL_SPEC>;
#[doc = "AES Control AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0."]
pub mod aesctl;
#[doc = "AESDATALEN0 (rw) register accessor: an alias for `Reg<AESDATALEN0_SPEC>`"]
pub type AESDATALEN0 = crate::Reg<aesdatalen0::AESDATALEN0_SPEC>;
#[doc = "AES Crypto Length 0 (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub mod aesdatalen0;
#[doc = "AESDATALEN1 (rw) register accessor: an alias for `Reg<AESDATALEN1_SPEC>`"]
pub type AESDATALEN1 = crate::Reg<aesdatalen1::AESDATALEN1_SPEC>;
#[doc = "AES Crypto Length 1 (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub mod aesdatalen1;
#[doc = "AESAUTHLEN (rw) register accessor: an alias for `Reg<AESAUTHLEN_SPEC>`"]
pub type AESAUTHLEN = crate::Reg<aesauthlen::AESAUTHLEN_SPEC>;
#[doc = "AES Authentication Length"]
pub mod aesauthlen;
#[doc = "AESDATAOUT0 (rw) register accessor: an alias for `Reg<AESDATAOUT0_SPEC>`"]
pub type AESDATAOUT0 = crate::Reg<aesdataout0::AESDATAOUT0_SPEC>;
#[doc = "Data Input/Output"]
pub mod aesdataout0;
#[doc = "AESDATAIN0 (rw) register accessor: an alias for `Reg<AESDATAIN0_SPEC>`"]
pub type AESDATAIN0 = crate::Reg<aesdatain0::AESDATAIN0_SPEC>;
#[doc = "AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain0;
#[doc = "AESDATAOUT1 (rw) register accessor: an alias for `Reg<AESDATAOUT1_SPEC>`"]
pub type AESDATAOUT1 = crate::Reg<aesdataout1::AESDATAOUT1_SPEC>;
#[doc = "Data Input/Output"]
pub mod aesdataout1;
#[doc = "AESDATAIN1 (rw) register accessor: an alias for `Reg<AESDATAIN1_SPEC>`"]
pub type AESDATAIN1 = crate::Reg<aesdatain1::AESDATAIN1_SPEC>;
#[doc = "AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain1;
#[doc = "AESDATAOUT2 (rw) register accessor: an alias for `Reg<AESDATAOUT2_SPEC>`"]
pub type AESDATAOUT2 = crate::Reg<aesdataout2::AESDATAOUT2_SPEC>;
#[doc = "Data Input/Output"]
pub mod aesdataout2;
#[doc = "AESDATAIN2 (rw) register accessor: an alias for `Reg<AESDATAIN2_SPEC>`"]
pub type AESDATAIN2 = crate::Reg<aesdatain2::AESDATAIN2_SPEC>;
#[doc = "AES Data Input_Output 2 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain2;
#[doc = "AESDATAOUT3 (rw) register accessor: an alias for `Reg<AESDATAOUT3_SPEC>`"]
pub type AESDATAOUT3 = crate::Reg<aesdataout3::AESDATAOUT3_SPEC>;
#[doc = "Data Input/Output"]
pub mod aesdataout3;
#[doc = "AESDATAIN3 (rw) register accessor: an alias for `Reg<AESDATAIN3_SPEC>`"]
pub type AESDATAIN3 = crate::Reg<aesdatain3::AESDATAIN3_SPEC>;
#[doc = "AES Data Input_Output 3 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain3;
#[doc = "AESTAGOUT (rw) register accessor: an alias for `Reg<AESTAGOUT_SPEC>`"]
pub type AESTAGOUT = crate::Reg<aestagout::AESTAGOUT_SPEC>;
#[doc = "AES Tag Out 0 The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice."]
pub mod aestagout;
#[doc = "HASHDATAIN1 (rw) register accessor: an alias for `Reg<HASHDATAIN1_SPEC>`"]
pub type HASHDATAIN1 = crate::Reg<hashdatain1::HASHDATAIN1_SPEC>;
#[doc = "HASH Data Input 1 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain1;
#[doc = "HASHDATAIN2 (rw) register accessor: an alias for `Reg<HASHDATAIN2_SPEC>`"]
pub type HASHDATAIN2 = crate::Reg<hashdatain2::HASHDATAIN2_SPEC>;
#[doc = "HASH Data Input 2 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain2;
#[doc = "HASHDATAIN3 (rw) register accessor: an alias for `Reg<HASHDATAIN3_SPEC>`"]
pub type HASHDATAIN3 = crate::Reg<hashdatain3::HASHDATAIN3_SPEC>;
#[doc = "HASH Data Input 3 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain3;
#[doc = "HASHDATAIN4 (rw) register accessor: an alias for `Reg<HASHDATAIN4_SPEC>`"]
pub type HASHDATAIN4 = crate::Reg<hashdatain4::HASHDATAIN4_SPEC>;
#[doc = "HASH Data Input 4 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain4;
#[doc = "HASHDATAIN5 (rw) register accessor: an alias for `Reg<HASHDATAIN5_SPEC>`"]
pub type HASHDATAIN5 = crate::Reg<hashdatain5::HASHDATAIN5_SPEC>;
#[doc = "HASH Data Input 5 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain5;
#[doc = "HASHDATAIN6 (rw) register accessor: an alias for `Reg<HASHDATAIN6_SPEC>`"]
pub type HASHDATAIN6 = crate::Reg<hashdatain6::HASHDATAIN6_SPEC>;
#[doc = "HASH Data Input 6 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain6;
#[doc = "HASHDATAIN7 (rw) register accessor: an alias for `Reg<HASHDATAIN7_SPEC>`"]
pub type HASHDATAIN7 = crate::Reg<hashdatain7::HASHDATAIN7_SPEC>;
#[doc = "HASH Data Input 7 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain7;
#[doc = "HASHDATAIN8 (rw) register accessor: an alias for `Reg<HASHDATAIN8_SPEC>`"]
pub type HASHDATAIN8 = crate::Reg<hashdatain8::HASHDATAIN8_SPEC>;
#[doc = "HASH Data Input 8 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain8;
#[doc = "HASHDATAIN9 (rw) register accessor: an alias for `Reg<HASHDATAIN9_SPEC>`"]
pub type HASHDATAIN9 = crate::Reg<hashdatain9::HASHDATAIN9_SPEC>;
#[doc = "HASH Data Input 9 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain9;
#[doc = "HASHDATAIN10 (rw) register accessor: an alias for `Reg<HASHDATAIN10_SPEC>`"]
pub type HASHDATAIN10 = crate::Reg<hashdatain10::HASHDATAIN10_SPEC>;
#[doc = "HASH Data Input 10 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain10;
#[doc = "HASHDATAIN11 (rw) register accessor: an alias for `Reg<HASHDATAIN11_SPEC>`"]
pub type HASHDATAIN11 = crate::Reg<hashdatain11::HASHDATAIN11_SPEC>;
#[doc = "HASH Data Input 11 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain11;
#[doc = "HASHDATAIN12 (rw) register accessor: an alias for `Reg<HASHDATAIN12_SPEC>`"]
pub type HASHDATAIN12 = crate::Reg<hashdatain12::HASHDATAIN12_SPEC>;
#[doc = "HASH Data Input 12 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain12;
#[doc = "HASHDATAIN13 (rw) register accessor: an alias for `Reg<HASHDATAIN13_SPEC>`"]
pub type HASHDATAIN13 = crate::Reg<hashdatain13::HASHDATAIN13_SPEC>;
#[doc = "HASH Data Input 13 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain13;
#[doc = "HASHDATAIN14 (rw) register accessor: an alias for `Reg<HASHDATAIN14_SPEC>`"]
pub type HASHDATAIN14 = crate::Reg<hashdatain14::HASHDATAIN14_SPEC>;
#[doc = "HASH Data Input 14 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain14;
#[doc = "HASHDATAIN15 (rw) register accessor: an alias for `Reg<HASHDATAIN15_SPEC>`"]
pub type HASHDATAIN15 = crate::Reg<hashdatain15::HASHDATAIN15_SPEC>;
#[doc = "HASH Data Input 15 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain15;
#[doc = "HASHDATAIN16 (rw) register accessor: an alias for `Reg<HASHDATAIN16_SPEC>`"]
pub type HASHDATAIN16 = crate::Reg<hashdatain16::HASHDATAIN16_SPEC>;
#[doc = "HASH Data Input 16 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain16;
#[doc = "HASHDATAIN17 (rw) register accessor: an alias for `Reg<HASHDATAIN17_SPEC>`"]
pub type HASHDATAIN17 = crate::Reg<hashdatain17::HASHDATAIN17_SPEC>;
#[doc = "HASH Data Input 17 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain17;
#[doc = "HASHDATAIN18 (rw) register accessor: an alias for `Reg<HASHDATAIN18_SPEC>`"]
pub type HASHDATAIN18 = crate::Reg<hashdatain18::HASHDATAIN18_SPEC>;
#[doc = "HASH Data Input 18 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain18;
#[doc = "HASHDATAIN19 (rw) register accessor: an alias for `Reg<HASHDATAIN19_SPEC>`"]
pub type HASHDATAIN19 = crate::Reg<hashdatain19::HASHDATAIN19_SPEC>;
#[doc = "HASH Data Input 19 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain19;
#[doc = "HASHDATAIN20 (rw) register accessor: an alias for `Reg<HASHDATAIN20_SPEC>`"]
pub type HASHDATAIN20 = crate::Reg<hashdatain20::HASHDATAIN20_SPEC>;
#[doc = "HASH Data Input 20 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain20;
#[doc = "HASHDATAIN21 (rw) register accessor: an alias for `Reg<HASHDATAIN21_SPEC>`"]
pub type HASHDATAIN21 = crate::Reg<hashdatain21::HASHDATAIN21_SPEC>;
#[doc = "HASH Data Input 21 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain21;
#[doc = "HASHDATAIN22 (rw) register accessor: an alias for `Reg<HASHDATAIN22_SPEC>`"]
pub type HASHDATAIN22 = crate::Reg<hashdatain22::HASHDATAIN22_SPEC>;
#[doc = "HASH Data Input 22 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain22;
#[doc = "HASHDATAIN23 (rw) register accessor: an alias for `Reg<HASHDATAIN23_SPEC>`"]
pub type HASHDATAIN23 = crate::Reg<hashdatain23::HASHDATAIN23_SPEC>;
#[doc = "HASH Data Input 23 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain23;
#[doc = "HASHDATAIN24 (rw) register accessor: an alias for `Reg<HASHDATAIN24_SPEC>`"]
pub type HASHDATAIN24 = crate::Reg<hashdatain24::HASHDATAIN24_SPEC>;
#[doc = "HASH Data Input 24 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain24;
#[doc = "HASHDATAIN25 (rw) register accessor: an alias for `Reg<HASHDATAIN25_SPEC>`"]
pub type HASHDATAIN25 = crate::Reg<hashdatain25::HASHDATAIN25_SPEC>;
#[doc = "HASH Data Input 25 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain25;
#[doc = "HASHDATAIN26 (rw) register accessor: an alias for `Reg<HASHDATAIN26_SPEC>`"]
pub type HASHDATAIN26 = crate::Reg<hashdatain26::HASHDATAIN26_SPEC>;
#[doc = "HASH Data Input 26 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain26;
#[doc = "HASHDATAIN27 (rw) register accessor: an alias for `Reg<HASHDATAIN27_SPEC>`"]
pub type HASHDATAIN27 = crate::Reg<hashdatain27::HASHDATAIN27_SPEC>;
#[doc = "HASH Data Input 27 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain27;
#[doc = "HASHDATAIN28 (rw) register accessor: an alias for `Reg<HASHDATAIN28_SPEC>`"]
pub type HASHDATAIN28 = crate::Reg<hashdatain28::HASHDATAIN28_SPEC>;
#[doc = "HASH Data Input 28 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain28;
#[doc = "HASHDATAIN29 (rw) register accessor: an alias for `Reg<HASHDATAIN29_SPEC>`"]
pub type HASHDATAIN29 = crate::Reg<hashdatain29::HASHDATAIN29_SPEC>;
#[doc = "HASH Data Input 29 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain29;
#[doc = "HASHDATAIN30 (rw) register accessor: an alias for `Reg<HASHDATAIN30_SPEC>`"]
pub type HASHDATAIN30 = crate::Reg<hashdatain30::HASHDATAIN30_SPEC>;
#[doc = "HASH Data Input 30 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain30;
#[doc = "HASHDATAIN31 (rw) register accessor: an alias for `Reg<HASHDATAIN31_SPEC>`"]
pub type HASHDATAIN31 = crate::Reg<hashdatain31::HASHDATAIN31_SPEC>;
#[doc = "HASH Data Input 31 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain31;
#[doc = "HASHIOBUFCTRL (rw) register accessor: an alias for `Reg<HASHIOBUFCTRL_SPEC>`"]
pub type HASHIOBUFCTRL = crate::Reg<hashiobufctrl::HASHIOBUFCTRL_SPEC>;
#[doc = "HASH Input_Output Buffer Control This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine."]
pub mod hashiobufctrl;
#[doc = "HASHMODE (rw) register accessor: an alias for `Reg<HASHMODE_SPEC>`"]
pub type HASHMODE = crate::Reg<hashmode::HASHMODE_SPEC>;
#[doc = "HASH Mode"]
pub mod hashmode;
#[doc = "HASHINLENL (rw) register accessor: an alias for `Reg<HASHINLENL_SPEC>`"]
pub type HASHINLENL = crate::Reg<hashinlenl::HASHINLENL_SPEC>;
#[doc = "HASH Input Length LSB"]
pub mod hashinlenl;
#[doc = "HASHINLENH (rw) register accessor: an alias for `Reg<HASHINLENH_SPEC>`"]
pub type HASHINLENH = crate::Reg<hashinlenh::HASHINLENH_SPEC>;
#[doc = "HASH Input Length MSB"]
pub mod hashinlenh;
#[doc = "HASHDIGESTA (rw) register accessor: an alias for `Reg<HASHDIGESTA_SPEC>`"]
pub type HASHDIGESTA = crate::Reg<hashdigesta::HASHDIGESTA_SPEC>;
#[doc = "HASH Digest A The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesta;
#[doc = "HASHDIGESTB (rw) register accessor: an alias for `Reg<HASHDIGESTB_SPEC>`"]
pub type HASHDIGESTB = crate::Reg<hashdigestb::HASHDIGESTB_SPEC>;
#[doc = "HASH Digest B The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestb;
#[doc = "HASHDIGESTC (rw) register accessor: an alias for `Reg<HASHDIGESTC_SPEC>`"]
pub type HASHDIGESTC = crate::Reg<hashdigestc::HASHDIGESTC_SPEC>;
#[doc = "HASH Digest C The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestc;
#[doc = "HASHDIGESTD (rw) register accessor: an alias for `Reg<HASHDIGESTD_SPEC>`"]
pub type HASHDIGESTD = crate::Reg<hashdigestd::HASHDIGESTD_SPEC>;
#[doc = "HASH Digest D The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestd;
#[doc = "HASHDIGESTE (rw) register accessor: an alias for `Reg<HASHDIGESTE_SPEC>`"]
pub type HASHDIGESTE = crate::Reg<hashdigeste::HASHDIGESTE_SPEC>;
#[doc = "HASH Digest E The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigeste;
#[doc = "HASHDIGESTF (rw) register accessor: an alias for `Reg<HASHDIGESTF_SPEC>`"]
pub type HASHDIGESTF = crate::Reg<hashdigestf::HASHDIGESTF_SPEC>;
#[doc = "HASH Digest F The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestf;
#[doc = "HASHDIGESTG (rw) register accessor: an alias for `Reg<HASHDIGESTG_SPEC>`"]
pub type HASHDIGESTG = crate::Reg<hashdigestg::HASHDIGESTG_SPEC>;
#[doc = "HASH Digest G The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestg;
#[doc = "HASHDIGESTH (rw) register accessor: an alias for `Reg<HASHDIGESTH_SPEC>`"]
pub type HASHDIGESTH = crate::Reg<hashdigesth::HASHDIGESTH_SPEC>;
#[doc = "HASH Digest H The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesth;
#[doc = "HASHDIGESTI (rw) register accessor: an alias for `Reg<HASHDIGESTI_SPEC>`"]
pub type HASHDIGESTI = crate::Reg<hashdigesti::HASHDIGESTI_SPEC>;
#[doc = "HASH Digest I The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesti;
#[doc = "HASHDIGESTJ (rw) register accessor: an alias for `Reg<HASHDIGESTJ_SPEC>`"]
pub type HASHDIGESTJ = crate::Reg<hashdigestj::HASHDIGESTJ_SPEC>;
#[doc = "HASH Digest J The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestj;
#[doc = "HASHDIGESTK (rw) register accessor: an alias for `Reg<HASHDIGESTK_SPEC>`"]
pub type HASHDIGESTK = crate::Reg<hashdigestk::HASHDIGESTK_SPEC>;
#[doc = "HASH Digest K The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestk;
#[doc = "HASHDIGESTL (rw) register accessor: an alias for `Reg<HASHDIGESTL_SPEC>`"]
pub type HASHDIGESTL = crate::Reg<hashdigestl::HASHDIGESTL_SPEC>;
#[doc = "HASH Digest L The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestl;
#[doc = "HASHDIGESTM (rw) register accessor: an alias for `Reg<HASHDIGESTM_SPEC>`"]
pub type HASHDIGESTM = crate::Reg<hashdigestm::HASHDIGESTM_SPEC>;
#[doc = "HASH Digest M The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestm;
#[doc = "HASHDIGESTN (rw) register accessor: an alias for `Reg<HASHDIGESTN_SPEC>`"]
pub type HASHDIGESTN = crate::Reg<hashdigestn::HASHDIGESTN_SPEC>;
#[doc = "HASH Digest N The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestn;
#[doc = "HASHDIGESTO (rw) register accessor: an alias for `Reg<HASHDIGESTO_SPEC>`"]
pub type HASHDIGESTO = crate::Reg<hashdigesto::HASHDIGESTO_SPEC>;
#[doc = "HASH Digest 0 The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesto;
#[doc = "HASHDIGESTP (rw) register accessor: an alias for `Reg<HASHDIGESTP_SPEC>`"]
pub type HASHDIGESTP = crate::Reg<hashdigestp::HASHDIGESTP_SPEC>;
#[doc = "HASH Digest P The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestp;
#[doc = "ALGSEL (rw) register accessor: an alias for `Reg<ALGSEL_SPEC>`"]
pub type ALGSEL = crate::Reg<algsel::ALGSEL_SPEC>;
#[doc = "Algorithm Select This algorithm selection register configures the internal destination of the DMA controller."]
pub mod algsel;
#[doc = "DMAPROTCTL (rw) register accessor: an alias for `Reg<DMAPROTCTL_SPEC>`"]
pub type DMAPROTCTL = crate::Reg<dmaprotctl::DMAPROTCTL_SPEC>;
#[doc = "DMA Protection Control Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module."]
pub mod dmaprotctl;
#[doc = "SWRESET (rw) register accessor: an alias for `Reg<SWRESET_SPEC>`"]
pub type SWRESET = crate::Reg<swreset::SWRESET_SPEC>;
#[doc = "Software Reset"]
pub mod swreset;
#[doc = "IRQTYPE (rw) register accessor: an alias for `Reg<IRQTYPE_SPEC>`"]
pub type IRQTYPE = crate::Reg<irqtype::IRQTYPE_SPEC>;
#[doc = "Control Interrupt Configuration"]
pub mod irqtype;
#[doc = "IRQEN (rw) register accessor: an alias for `Reg<IRQEN_SPEC>`"]
pub type IRQEN = crate::Reg<irqen::IRQEN_SPEC>;
#[doc = "Control Interrupt Enable"]
pub mod irqen;
#[doc = "IRQCLR (rw) register accessor: an alias for `Reg<IRQCLR_SPEC>`"]
pub type IRQCLR = crate::Reg<irqclr::IRQCLR_SPEC>;
#[doc = "Control Interrupt Clear"]
pub mod irqclr;
#[doc = "IRQSET (rw) register accessor: an alias for `Reg<IRQSET_SPEC>`"]
pub type IRQSET = crate::Reg<irqset::IRQSET_SPEC>;
#[doc = "Control Interrupt Set"]
pub mod irqset;
#[doc = "IRQSTAT (rw) register accessor: an alias for `Reg<IRQSTAT_SPEC>`"]
pub type IRQSTAT = crate::Reg<irqstat::IRQSTAT_SPEC>;
#[doc = "Control Interrupt Status"]
pub mod irqstat;
#[doc = "HWVER (rw) register accessor: an alias for `Reg<HWVER_SPEC>`"]
pub type HWVER = crate::Reg<hwver::HWVER_SPEC>;
#[doc = "Hardware Version"]
pub mod hwver;
