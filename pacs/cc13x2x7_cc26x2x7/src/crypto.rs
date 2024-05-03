#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmach0ctl: Dmach0ctl,
    dmach0extaddr: Dmach0extaddr,
    _reserved2: [u8; 0x04],
    dmach0len: Dmach0len,
    _reserved3: [u8; 0x08],
    dmastat: Dmastat,
    dmaswreset: Dmaswreset,
    dmach1ctl: Dmach1ctl,
    dmach1extaddr: Dmach1extaddr,
    _reserved7: [u8; 0x04],
    dmach1len: Dmach1len,
    _reserved8: [u8; 0x48],
    dmabuscfg: Dmabuscfg,
    dmaporterr: Dmaporterr,
    _reserved10: [u8; 0x7c],
    dmahwver: Dmahwver,
    _reserved11: [u8; 0x0300],
    keywritearea: Keywritearea,
    keywrittenarea: Keywrittenarea,
    keysize: Keysize,
    keyreadarea: Keyreadarea,
    _reserved15: [u8; 0xf0],
    aeskey2: Aeskey2,
    _reserved16: [u8; 0x0c],
    aeskey3: Aeskey3,
    _reserved17: [u8; 0x2c],
    aesiv: Aesiv,
    _reserved18: [u8; 0x0c],
    aesctl: Aesctl,
    aesdatalen0: Aesdatalen0,
    aesdatalen1: Aesdatalen1,
    aesauthlen: Aesauthlen,
    _reserved_22_aesdatain0: [u8; 0x04],
    _reserved_23_aesdatain1: [u8; 0x04],
    _reserved_24_aesdatain2: [u8; 0x04],
    _reserved_25_aesdatain3: [u8; 0x04],
    aestagout: Aestagout,
    _reserved27: [u8; 0x90],
    hashdatain1: Hashdatain1,
    hashdatain2: Hashdatain2,
    hashdatain3: Hashdatain3,
    hashdatain4: Hashdatain4,
    hashdatain5: Hashdatain5,
    hashdatain6: Hashdatain6,
    hashdatain7: Hashdatain7,
    hashdatain8: Hashdatain8,
    hashdatain9: Hashdatain9,
    hashdatain10: Hashdatain10,
    hashdatain11: Hashdatain11,
    hashdatain12: Hashdatain12,
    hashdatain13: Hashdatain13,
    hashdatain14: Hashdatain14,
    hashdatain15: Hashdatain15,
    hashdatain16: Hashdatain16,
    hashdatain17: Hashdatain17,
    hashdatain18: Hashdatain18,
    hashdatain19: Hashdatain19,
    hashdatain20: Hashdatain20,
    hashdatain21: Hashdatain21,
    hashdatain22: Hashdatain22,
    hashdatain23: Hashdatain23,
    hashdatain24: Hashdatain24,
    hashdatain25: Hashdatain25,
    hashdatain26: Hashdatain26,
    hashdatain27: Hashdatain27,
    hashdatain28: Hashdatain28,
    hashdatain29: Hashdatain29,
    hashdatain30: Hashdatain30,
    hashdatain31: Hashdatain31,
    hashiobufctrl: Hashiobufctrl,
    hashmode: Hashmode,
    hashinlenl: Hashinlenl,
    hashinlenh: Hashinlenh,
    _reserved62: [u8; 0x30],
    hashdigesta: Hashdigesta,
    hashdigestb: Hashdigestb,
    hashdigestc: Hashdigestc,
    hashdigestd: Hashdigestd,
    hashdigeste: Hashdigeste,
    hashdigestf: Hashdigestf,
    hashdigestg: Hashdigestg,
    hashdigesth: Hashdigesth,
    hashdigesti: Hashdigesti,
    hashdigestj: Hashdigestj,
    hashdigestk: Hashdigestk,
    hashdigestl: Hashdigestl,
    hashdigestm: Hashdigestm,
    hashdigestn: Hashdigestn,
    hashdigesto: Hashdigesto,
    hashdigestp: Hashdigestp,
    algsel: Algsel,
    dmaprotctl: Dmaprotctl,
    _reserved80: [u8; 0x38],
    swreset: Swreset,
    _reserved81: [u8; 0x3c],
    irqtype: Irqtype,
    irqen: Irqen,
    irqclr: Irqclr,
    irqset: Irqset,
    irqstat: Irqstat,
    _reserved86: [u8; 0x68],
    hwver: Hwver,
}
impl RegisterBlock {
    #[doc = "0x00 - Channel 0 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
    #[inline(always)]
    pub const fn dmach0ctl(&self) -> &Dmach0ctl {
        &self.dmach0ctl
    }
    #[doc = "0x04 - Channel 0 External Address"]
    #[inline(always)]
    pub const fn dmach0extaddr(&self) -> &Dmach0extaddr {
        &self.dmach0extaddr
    }
    #[doc = "0x0c - Channel 0 DMA Length"]
    #[inline(always)]
    pub const fn dmach0len(&self) -> &Dmach0len {
        &self.dmach0len
    }
    #[doc = "0x18 - DMAC Status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer."]
    #[inline(always)]
    pub const fn dmastat(&self) -> &Dmastat {
        &self.dmastat
    }
    #[doc = "0x1c - DMAC Software Reset Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMASTAT."]
    #[inline(always)]
    pub const fn dmaswreset(&self) -> &Dmaswreset {
        &self.dmaswreset
    }
    #[doc = "0x20 - Channel 1 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
    #[inline(always)]
    pub const fn dmach1ctl(&self) -> &Dmach1ctl {
        &self.dmach1ctl
    }
    #[doc = "0x24 - Channel 1 External Address"]
    #[inline(always)]
    pub const fn dmach1extaddr(&self) -> &Dmach1extaddr {
        &self.dmach1extaddr
    }
    #[doc = "0x2c - Channel 1 DMA Length"]
    #[inline(always)]
    pub const fn dmach1len(&self) -> &Dmach1len {
        &self.dmach1len
    }
    #[doc = "0x78 - DMAC Master Run-time Parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter."]
    #[inline(always)]
    pub const fn dmabuscfg(&self) -> &Dmabuscfg {
        &self.dmabuscfg
    }
    #[doc = "0x7c - DMAC Port Error Raw Status This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMASWRESET register."]
    #[inline(always)]
    pub const fn dmaporterr(&self) -> &Dmaporterr {
        &self.dmaporterr
    }
    #[doc = "0xfc - DMAC Version This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers."]
    #[inline(always)]
    pub const fn dmahwver(&self) -> &Dmahwver {
        &self.dmahwver
    }
    #[doc = "0x400 - Key Store Write Area This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine."]
    #[inline(always)]
    pub const fn keywritearea(&self) -> &Keywritearea {
        &self.keywritearea
    }
    #[doc = "0x404 - Key Store Written Area This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error."]
    #[inline(always)]
    pub const fn keywrittenarea(&self) -> &Keywrittenarea {
        &self.keywrittenarea
    }
    #[doc = "0x408 - Key Store Size This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register."]
    #[inline(always)]
    pub const fn keysize(&self) -> &Keysize {
        &self.keysize
    }
    #[doc = "0x40c - Key Store Read Area This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key."]
    #[inline(always)]
    pub const fn keyreadarea(&self) -> &Keyreadarea {
        &self.keyreadarea
    }
    #[doc = "0x500 - AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    #[inline(always)]
    pub const fn aeskey2(&self) -> &Aeskey2 {
        &self.aeskey2
    }
    #[doc = "0x510 - AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    #[inline(always)]
    pub const fn aeskey3(&self) -> &Aeskey3 {
        &self.aeskey3
    }
    #[doc = "0x540 - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    #[inline(always)]
    pub const fn aesiv(&self) -> &Aesiv {
        &self.aesiv
    }
    #[doc = "0x550 - AES Control AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0."]
    #[inline(always)]
    pub const fn aesctl(&self) -> &Aesctl {
        &self.aesctl
    }
    #[doc = "0x554 - AES Crypto Length 0 (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
    #[inline(always)]
    pub const fn aesdatalen0(&self) -> &Aesdatalen0 {
        &self.aesdatalen0
    }
    #[doc = "0x558 - AES Crypto Length 1 (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
    #[inline(always)]
    pub const fn aesdatalen1(&self) -> &Aesdatalen1 {
        &self.aesdatalen1
    }
    #[doc = "0x55c - AES Authentication Length"]
    #[inline(always)]
    pub const fn aesauthlen(&self) -> &Aesauthlen {
        &self.aesauthlen
    }
    #[doc = "0x560 - AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub const fn aesdatain0(&self) -> &Aesdatain0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376).cast() }
    }
    #[doc = "0x560 - Data Input/Output"]
    #[inline(always)]
    pub const fn aesdataout0(&self) -> &Aesdataout0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1376).cast() }
    }
    #[doc = "0x564 - AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub const fn aesdatain1(&self) -> &Aesdatain1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1380).cast() }
    }
    #[doc = "0x564 - Data Input/Output"]
    #[inline(always)]
    pub const fn aesdataout1(&self) -> &Aesdataout1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1380).cast() }
    }
    #[doc = "0x568 - AES Data Input_Output 2 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub const fn aesdatain2(&self) -> &Aesdatain2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1384).cast() }
    }
    #[doc = "0x568 - Data Input/Output"]
    #[inline(always)]
    pub const fn aesdataout2(&self) -> &Aesdataout2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1384).cast() }
    }
    #[doc = "0x56c - AES Data Input_Output 3 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    #[inline(always)]
    pub const fn aesdatain3(&self) -> &Aesdatain3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1388).cast() }
    }
    #[doc = "0x56c - Data Input/Output"]
    #[inline(always)]
    pub const fn aesdataout3(&self) -> &Aesdataout3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1388).cast() }
    }
    #[doc = "0x570 - AES Tag Out 0 The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice."]
    #[inline(always)]
    pub const fn aestagout(&self) -> &Aestagout {
        &self.aestagout
    }
    #[doc = "0x604 - HASH Data Input 1 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain1(&self) -> &Hashdatain1 {
        &self.hashdatain1
    }
    #[doc = "0x608 - HASH Data Input 2 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain2(&self) -> &Hashdatain2 {
        &self.hashdatain2
    }
    #[doc = "0x60c - HASH Data Input 3 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain3(&self) -> &Hashdatain3 {
        &self.hashdatain3
    }
    #[doc = "0x610 - HASH Data Input 4 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain4(&self) -> &Hashdatain4 {
        &self.hashdatain4
    }
    #[doc = "0x614 - HASH Data Input 5 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain5(&self) -> &Hashdatain5 {
        &self.hashdatain5
    }
    #[doc = "0x618 - HASH Data Input 6 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain6(&self) -> &Hashdatain6 {
        &self.hashdatain6
    }
    #[doc = "0x61c - HASH Data Input 7 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain7(&self) -> &Hashdatain7 {
        &self.hashdatain7
    }
    #[doc = "0x620 - HASH Data Input 8 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain8(&self) -> &Hashdatain8 {
        &self.hashdatain8
    }
    #[doc = "0x624 - HASH Data Input 9 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain9(&self) -> &Hashdatain9 {
        &self.hashdatain9
    }
    #[doc = "0x628 - HASH Data Input 10 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain10(&self) -> &Hashdatain10 {
        &self.hashdatain10
    }
    #[doc = "0x62c - HASH Data Input 11 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain11(&self) -> &Hashdatain11 {
        &self.hashdatain11
    }
    #[doc = "0x630 - HASH Data Input 12 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain12(&self) -> &Hashdatain12 {
        &self.hashdatain12
    }
    #[doc = "0x634 - HASH Data Input 13 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain13(&self) -> &Hashdatain13 {
        &self.hashdatain13
    }
    #[doc = "0x638 - HASH Data Input 14 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain14(&self) -> &Hashdatain14 {
        &self.hashdatain14
    }
    #[doc = "0x63c - HASH Data Input 15 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain15(&self) -> &Hashdatain15 {
        &self.hashdatain15
    }
    #[doc = "0x640 - HASH Data Input 16 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain16(&self) -> &Hashdatain16 {
        &self.hashdatain16
    }
    #[doc = "0x644 - HASH Data Input 17 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain17(&self) -> &Hashdatain17 {
        &self.hashdatain17
    }
    #[doc = "0x648 - HASH Data Input 18 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain18(&self) -> &Hashdatain18 {
        &self.hashdatain18
    }
    #[doc = "0x64c - HASH Data Input 19 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain19(&self) -> &Hashdatain19 {
        &self.hashdatain19
    }
    #[doc = "0x650 - HASH Data Input 20 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain20(&self) -> &Hashdatain20 {
        &self.hashdatain20
    }
    #[doc = "0x654 - HASH Data Input 21 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain21(&self) -> &Hashdatain21 {
        &self.hashdatain21
    }
    #[doc = "0x658 - HASH Data Input 22 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain22(&self) -> &Hashdatain22 {
        &self.hashdatain22
    }
    #[doc = "0x65c - HASH Data Input 23 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain23(&self) -> &Hashdatain23 {
        &self.hashdatain23
    }
    #[doc = "0x660 - HASH Data Input 24 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain24(&self) -> &Hashdatain24 {
        &self.hashdatain24
    }
    #[doc = "0x664 - HASH Data Input 25 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain25(&self) -> &Hashdatain25 {
        &self.hashdatain25
    }
    #[doc = "0x668 - HASH Data Input 26 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain26(&self) -> &Hashdatain26 {
        &self.hashdatain26
    }
    #[doc = "0x66c - HASH Data Input 27 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain27(&self) -> &Hashdatain27 {
        &self.hashdatain27
    }
    #[doc = "0x670 - HASH Data Input 28 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain28(&self) -> &Hashdatain28 {
        &self.hashdatain28
    }
    #[doc = "0x674 - HASH Data Input 29 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain29(&self) -> &Hashdatain29 {
        &self.hashdatain29
    }
    #[doc = "0x678 - HASH Data Input 30 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain30(&self) -> &Hashdatain30 {
        &self.hashdatain30
    }
    #[doc = "0x67c - HASH Data Input 31 The data input registers should be used to provide input data to the hash module through the slave interface."]
    #[inline(always)]
    pub const fn hashdatain31(&self) -> &Hashdatain31 {
        &self.hashdatain31
    }
    #[doc = "0x680 - HASH Input_Output Buffer Control This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine."]
    #[inline(always)]
    pub const fn hashiobufctrl(&self) -> &Hashiobufctrl {
        &self.hashiobufctrl
    }
    #[doc = "0x684 - HASH Mode"]
    #[inline(always)]
    pub const fn hashmode(&self) -> &Hashmode {
        &self.hashmode
    }
    #[doc = "0x688 - HASH Input Length LSB"]
    #[inline(always)]
    pub const fn hashinlenl(&self) -> &Hashinlenl {
        &self.hashinlenl
    }
    #[doc = "0x68c - HASH Input Length MSB"]
    #[inline(always)]
    pub const fn hashinlenh(&self) -> &Hashinlenh {
        &self.hashinlenh
    }
    #[doc = "0x6c0 - HASH Digest A The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigesta(&self) -> &Hashdigesta {
        &self.hashdigesta
    }
    #[doc = "0x6c4 - HASH Digest B The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigestb(&self) -> &Hashdigestb {
        &self.hashdigestb
    }
    #[doc = "0x6c8 - HASH Digest C The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigestc(&self) -> &Hashdigestc {
        &self.hashdigestc
    }
    #[doc = "0x6cc - HASH Digest D The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigestd(&self) -> &Hashdigestd {
        &self.hashdigestd
    }
    #[doc = "0x6d0 - HASH Digest E The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigeste(&self) -> &Hashdigeste {
        &self.hashdigeste
    }
    #[doc = "0x6d4 - HASH Digest F The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigestf(&self) -> &Hashdigestf {
        &self.hashdigestf
    }
    #[doc = "0x6d8 - HASH Digest G The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigestg(&self) -> &Hashdigestg {
        &self.hashdigestg
    }
    #[doc = "0x6dc - HASH Digest H The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigesth(&self) -> &Hashdigesth {
        &self.hashdigesth
    }
    #[doc = "0x6e0 - HASH Digest I The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigesti(&self) -> &Hashdigesti {
        &self.hashdigesti
    }
    #[doc = "0x6e4 - HASH Digest J The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigestj(&self) -> &Hashdigestj {
        &self.hashdigestj
    }
    #[doc = "0x6e8 - HASH Digest K The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigestk(&self) -> &Hashdigestk {
        &self.hashdigestk
    }
    #[doc = "0x6ec - HASH Digest L The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigestl(&self) -> &Hashdigestl {
        &self.hashdigestl
    }
    #[doc = "0x6f0 - HASH Digest M The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigestm(&self) -> &Hashdigestm {
        &self.hashdigestm
    }
    #[doc = "0x6f4 - HASH Digest N The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigestn(&self) -> &Hashdigestn {
        &self.hashdigestn
    }
    #[doc = "0x6f8 - HASH Digest 0 The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigesto(&self) -> &Hashdigesto {
        &self.hashdigesto
    }
    #[doc = "0x6fc - HASH Digest P The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    #[inline(always)]
    pub const fn hashdigestp(&self) -> &Hashdigestp {
        &self.hashdigestp
    }
    #[doc = "0x700 - Algorithm Select This algorithm selection register configures the internal destination of the DMA controller."]
    #[inline(always)]
    pub const fn algsel(&self) -> &Algsel {
        &self.algsel
    }
    #[doc = "0x704 - DMA Protection Control Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module."]
    #[inline(always)]
    pub const fn dmaprotctl(&self) -> &Dmaprotctl {
        &self.dmaprotctl
    }
    #[doc = "0x740 - Software Reset"]
    #[inline(always)]
    pub const fn swreset(&self) -> &Swreset {
        &self.swreset
    }
    #[doc = "0x780 - Control Interrupt Configuration"]
    #[inline(always)]
    pub const fn irqtype(&self) -> &Irqtype {
        &self.irqtype
    }
    #[doc = "0x784 - Control Interrupt Enable"]
    #[inline(always)]
    pub const fn irqen(&self) -> &Irqen {
        &self.irqen
    }
    #[doc = "0x788 - Control Interrupt Clear"]
    #[inline(always)]
    pub const fn irqclr(&self) -> &Irqclr {
        &self.irqclr
    }
    #[doc = "0x78c - Control Interrupt Set"]
    #[inline(always)]
    pub const fn irqset(&self) -> &Irqset {
        &self.irqset
    }
    #[doc = "0x790 - Control Interrupt Status"]
    #[inline(always)]
    pub const fn irqstat(&self) -> &Irqstat {
        &self.irqstat
    }
    #[doc = "0x7fc - Hardware Version"]
    #[inline(always)]
    pub const fn hwver(&self) -> &Hwver {
        &self.hwver
    }
}
#[doc = "DMACH0CTL (rw) register accessor: Channel 0 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach0ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach0ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach0ctl`]
module"]
#[doc(alias = "DMACH0CTL")]
pub type Dmach0ctl = crate::Reg<dmach0ctl::Dmach0ctlSpec>;
#[doc = "Channel 0 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub mod dmach0ctl;
#[doc = "DMACH0EXTADDR (rw) register accessor: Channel 0 External Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach0extaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach0extaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach0extaddr`]
module"]
#[doc(alias = "DMACH0EXTADDR")]
pub type Dmach0extaddr = crate::Reg<dmach0extaddr::Dmach0extaddrSpec>;
#[doc = "Channel 0 External Address"]
pub mod dmach0extaddr;
#[doc = "DMACH0LEN (rw) register accessor: Channel 0 DMA Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach0len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach0len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach0len`]
module"]
#[doc(alias = "DMACH0LEN")]
pub type Dmach0len = crate::Reg<dmach0len::Dmach0lenSpec>;
#[doc = "Channel 0 DMA Length"]
pub mod dmach0len;
#[doc = "DMASTAT (rw) register accessor: DMAC Status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmastat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmastat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmastat`]
module"]
#[doc(alias = "DMASTAT")]
pub type Dmastat = crate::Reg<dmastat::DmastatSpec>;
#[doc = "DMAC Status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer."]
pub mod dmastat;
#[doc = "DMASWRESET (rw) register accessor: DMAC Software Reset Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMASTAT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaswreset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaswreset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaswreset`]
module"]
#[doc(alias = "DMASWRESET")]
pub type Dmaswreset = crate::Reg<dmaswreset::DmaswresetSpec>;
#[doc = "DMAC Software Reset Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMASTAT."]
pub mod dmaswreset;
#[doc = "DMACH1CTL (rw) register accessor: Channel 1 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach1ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach1ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach1ctl`]
module"]
#[doc(alias = "DMACH1CTL")]
pub type Dmach1ctl = crate::Reg<dmach1ctl::Dmach1ctlSpec>;
#[doc = "Channel 1 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub mod dmach1ctl;
#[doc = "DMACH1EXTADDR (rw) register accessor: Channel 1 External Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach1extaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach1extaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach1extaddr`]
module"]
#[doc(alias = "DMACH1EXTADDR")]
pub type Dmach1extaddr = crate::Reg<dmach1extaddr::Dmach1extaddrSpec>;
#[doc = "Channel 1 External Address"]
pub mod dmach1extaddr;
#[doc = "DMACH1LEN (rw) register accessor: Channel 1 DMA Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach1len::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach1len::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmach1len`]
module"]
#[doc(alias = "DMACH1LEN")]
pub type Dmach1len = crate::Reg<dmach1len::Dmach1lenSpec>;
#[doc = "Channel 1 DMA Length"]
pub mod dmach1len;
#[doc = "DMABUSCFG (rw) register accessor: DMAC Master Run-time Parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmabuscfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmabuscfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmabuscfg`]
module"]
#[doc(alias = "DMABUSCFG")]
pub type Dmabuscfg = crate::Reg<dmabuscfg::DmabuscfgSpec>;
#[doc = "DMAC Master Run-time Parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter."]
pub mod dmabuscfg;
#[doc = "DMAPORTERR (rw) register accessor: DMAC Port Error Raw Status This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMASWRESET register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaporterr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaporterr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaporterr`]
module"]
#[doc(alias = "DMAPORTERR")]
pub type Dmaporterr = crate::Reg<dmaporterr::DmaporterrSpec>;
#[doc = "DMAC Port Error Raw Status This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMASWRESET register."]
pub mod dmaporterr;
#[doc = "DMAHWVER (rw) register accessor: DMAC Version This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmahwver::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmahwver::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmahwver`]
module"]
#[doc(alias = "DMAHWVER")]
pub type Dmahwver = crate::Reg<dmahwver::DmahwverSpec>;
#[doc = "DMAC Version This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers."]
pub mod dmahwver;
#[doc = "KEYWRITEAREA (rw) register accessor: Key Store Write Area This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keywritearea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keywritearea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keywritearea`]
module"]
#[doc(alias = "KEYWRITEAREA")]
pub type Keywritearea = crate::Reg<keywritearea::KeywriteareaSpec>;
#[doc = "Key Store Write Area This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine."]
pub mod keywritearea;
#[doc = "KEYWRITTENAREA (rw) register accessor: Key Store Written Area This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keywrittenarea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keywrittenarea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keywrittenarea`]
module"]
#[doc(alias = "KEYWRITTENAREA")]
pub type Keywrittenarea = crate::Reg<keywrittenarea::KeywrittenareaSpec>;
#[doc = "Key Store Written Area This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error."]
pub mod keywrittenarea;
#[doc = "KEYSIZE (rw) register accessor: Key Store Size This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keysize::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keysize::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keysize`]
module"]
#[doc(alias = "KEYSIZE")]
pub type Keysize = crate::Reg<keysize::KeysizeSpec>;
#[doc = "Key Store Size This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register."]
pub mod keysize;
#[doc = "KEYREADAREA (rw) register accessor: Key Store Read Area This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyreadarea::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyreadarea::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyreadarea`]
module"]
#[doc(alias = "KEYREADAREA")]
pub type Keyreadarea = crate::Reg<keyreadarea::KeyreadareaSpec>;
#[doc = "Key Store Read Area This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key."]
pub mod keyreadarea;
#[doc = "AESKEY2 (rw) register accessor: AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aeskey2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aeskey2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey2`]
module"]
#[doc(alias = "AESKEY2")]
pub type Aeskey2 = crate::Reg<aeskey2::Aeskey2Spec>;
#[doc = "AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aeskey2;
#[doc = "AESKEY3 (rw) register accessor: AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aeskey3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aeskey3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aeskey3`]
module"]
#[doc(alias = "AESKEY3")]
pub type Aeskey3 = crate::Reg<aeskey3::Aeskey3Spec>;
#[doc = "AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aeskey3;
#[doc = "AESIV (rw) register accessor: AES initialization vector registers These registers are used to provide and read the IV from the AES engine.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesiv`]
module"]
#[doc(alias = "AESIV")]
pub type Aesiv = crate::Reg<aesiv::AesivSpec>;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aesiv;
#[doc = "AESCTL (rw) register accessor: AES Control AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesctl`]
module"]
#[doc(alias = "AESCTL")]
pub type Aesctl = crate::Reg<aesctl::AesctlSpec>;
#[doc = "AES Control AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0."]
pub mod aesctl;
#[doc = "AESDATALEN0 (rw) register accessor: AES Crypto Length 0 (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatalen0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatalen0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdatalen0`]
module"]
#[doc(alias = "AESDATALEN0")]
pub type Aesdatalen0 = crate::Reg<aesdatalen0::Aesdatalen0Spec>;
#[doc = "AES Crypto Length 0 (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub mod aesdatalen0;
#[doc = "AESDATALEN1 (rw) register accessor: AES Crypto Length 1 (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatalen1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatalen1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdatalen1`]
module"]
#[doc(alias = "AESDATALEN1")]
pub type Aesdatalen1 = crate::Reg<aesdatalen1::Aesdatalen1Spec>;
#[doc = "AES Crypto Length 1 (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub mod aesdatalen1;
#[doc = "AESAUTHLEN (rw) register accessor: AES Authentication Length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesauthlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesauthlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesauthlen`]
module"]
#[doc(alias = "AESAUTHLEN")]
pub type Aesauthlen = crate::Reg<aesauthlen::AesauthlenSpec>;
#[doc = "AES Authentication Length"]
pub mod aesauthlen;
#[doc = "AESDATAOUT0 (rw) register accessor: Data Input/Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdataout0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdataout0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdataout0`]
module"]
#[doc(alias = "AESDATAOUT0")]
pub type Aesdataout0 = crate::Reg<aesdataout0::Aesdataout0Spec>;
#[doc = "Data Input/Output"]
pub mod aesdataout0;
#[doc = "AESDATAIN0 (rw) register accessor: AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatain0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatain0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdatain0`]
module"]
#[doc(alias = "AESDATAIN0")]
pub type Aesdatain0 = crate::Reg<aesdatain0::Aesdatain0Spec>;
#[doc = "AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain0;
#[doc = "AESDATAOUT1 (rw) register accessor: Data Input/Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdataout1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdataout1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdataout1`]
module"]
#[doc(alias = "AESDATAOUT1")]
pub type Aesdataout1 = crate::Reg<aesdataout1::Aesdataout1Spec>;
#[doc = "Data Input/Output"]
pub mod aesdataout1;
#[doc = "AESDATAIN1 (rw) register accessor: AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatain1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatain1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdatain1`]
module"]
#[doc(alias = "AESDATAIN1")]
pub type Aesdatain1 = crate::Reg<aesdatain1::Aesdatain1Spec>;
#[doc = "AES Data Input_Output 0 The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain1;
#[doc = "AESDATAOUT2 (rw) register accessor: Data Input/Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdataout2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdataout2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdataout2`]
module"]
#[doc(alias = "AESDATAOUT2")]
pub type Aesdataout2 = crate::Reg<aesdataout2::Aesdataout2Spec>;
#[doc = "Data Input/Output"]
pub mod aesdataout2;
#[doc = "AESDATAIN2 (rw) register accessor: AES Data Input_Output 2 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatain2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatain2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdatain2`]
module"]
#[doc(alias = "AESDATAIN2")]
pub type Aesdatain2 = crate::Reg<aesdatain2::Aesdatain2Spec>;
#[doc = "AES Data Input_Output 2 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain2;
#[doc = "AESDATAOUT3 (rw) register accessor: Data Input/Output\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdataout3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdataout3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdataout3`]
module"]
#[doc(alias = "AESDATAOUT3")]
pub type Aesdataout3 = crate::Reg<aesdataout3::Aesdataout3Spec>;
#[doc = "Data Input/Output"]
pub mod aesdataout3;
#[doc = "AESDATAIN3 (rw) register accessor: AES Data Input_Output 3 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdatain3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdatain3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdatain3`]
module"]
#[doc(alias = "AESDATAIN3")]
pub type Aesdatain3 = crate::Reg<aesdatain3::Aesdatain3Spec>;
#[doc = "AES Data Input_Output 3 The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aesdatain3;
#[doc = "AESTAGOUT (rw) register accessor: AES Tag Out 0 The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aestagout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aestagout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aestagout`]
module"]
#[doc(alias = "AESTAGOUT")]
pub type Aestagout = crate::Reg<aestagout::AestagoutSpec>;
#[doc = "AES Tag Out 0 The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice."]
pub mod aestagout;
#[doc = "HASHDATAIN1 (rw) register accessor: HASH Data Input 1 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain1`]
module"]
#[doc(alias = "HASHDATAIN1")]
pub type Hashdatain1 = crate::Reg<hashdatain1::Hashdatain1Spec>;
#[doc = "HASH Data Input 1 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain1;
#[doc = "HASHDATAIN2 (rw) register accessor: HASH Data Input 2 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain2`]
module"]
#[doc(alias = "HASHDATAIN2")]
pub type Hashdatain2 = crate::Reg<hashdatain2::Hashdatain2Spec>;
#[doc = "HASH Data Input 2 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain2;
#[doc = "HASHDATAIN3 (rw) register accessor: HASH Data Input 3 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain3`]
module"]
#[doc(alias = "HASHDATAIN3")]
pub type Hashdatain3 = crate::Reg<hashdatain3::Hashdatain3Spec>;
#[doc = "HASH Data Input 3 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain3;
#[doc = "HASHDATAIN4 (rw) register accessor: HASH Data Input 4 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain4`]
module"]
#[doc(alias = "HASHDATAIN4")]
pub type Hashdatain4 = crate::Reg<hashdatain4::Hashdatain4Spec>;
#[doc = "HASH Data Input 4 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain4;
#[doc = "HASHDATAIN5 (rw) register accessor: HASH Data Input 5 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain5`]
module"]
#[doc(alias = "HASHDATAIN5")]
pub type Hashdatain5 = crate::Reg<hashdatain5::Hashdatain5Spec>;
#[doc = "HASH Data Input 5 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain5;
#[doc = "HASHDATAIN6 (rw) register accessor: HASH Data Input 6 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain6`]
module"]
#[doc(alias = "HASHDATAIN6")]
pub type Hashdatain6 = crate::Reg<hashdatain6::Hashdatain6Spec>;
#[doc = "HASH Data Input 6 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain6;
#[doc = "HASHDATAIN7 (rw) register accessor: HASH Data Input 7 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain7`]
module"]
#[doc(alias = "HASHDATAIN7")]
pub type Hashdatain7 = crate::Reg<hashdatain7::Hashdatain7Spec>;
#[doc = "HASH Data Input 7 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain7;
#[doc = "HASHDATAIN8 (rw) register accessor: HASH Data Input 8 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain8`]
module"]
#[doc(alias = "HASHDATAIN8")]
pub type Hashdatain8 = crate::Reg<hashdatain8::Hashdatain8Spec>;
#[doc = "HASH Data Input 8 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain8;
#[doc = "HASHDATAIN9 (rw) register accessor: HASH Data Input 9 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain9`]
module"]
#[doc(alias = "HASHDATAIN9")]
pub type Hashdatain9 = crate::Reg<hashdatain9::Hashdatain9Spec>;
#[doc = "HASH Data Input 9 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain9;
#[doc = "HASHDATAIN10 (rw) register accessor: HASH Data Input 10 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain10`]
module"]
#[doc(alias = "HASHDATAIN10")]
pub type Hashdatain10 = crate::Reg<hashdatain10::Hashdatain10Spec>;
#[doc = "HASH Data Input 10 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain10;
#[doc = "HASHDATAIN11 (rw) register accessor: HASH Data Input 11 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain11`]
module"]
#[doc(alias = "HASHDATAIN11")]
pub type Hashdatain11 = crate::Reg<hashdatain11::Hashdatain11Spec>;
#[doc = "HASH Data Input 11 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain11;
#[doc = "HASHDATAIN12 (rw) register accessor: HASH Data Input 12 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain12`]
module"]
#[doc(alias = "HASHDATAIN12")]
pub type Hashdatain12 = crate::Reg<hashdatain12::Hashdatain12Spec>;
#[doc = "HASH Data Input 12 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain12;
#[doc = "HASHDATAIN13 (rw) register accessor: HASH Data Input 13 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain13`]
module"]
#[doc(alias = "HASHDATAIN13")]
pub type Hashdatain13 = crate::Reg<hashdatain13::Hashdatain13Spec>;
#[doc = "HASH Data Input 13 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain13;
#[doc = "HASHDATAIN14 (rw) register accessor: HASH Data Input 14 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain14`]
module"]
#[doc(alias = "HASHDATAIN14")]
pub type Hashdatain14 = crate::Reg<hashdatain14::Hashdatain14Spec>;
#[doc = "HASH Data Input 14 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain14;
#[doc = "HASHDATAIN15 (rw) register accessor: HASH Data Input 15 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain15`]
module"]
#[doc(alias = "HASHDATAIN15")]
pub type Hashdatain15 = crate::Reg<hashdatain15::Hashdatain15Spec>;
#[doc = "HASH Data Input 15 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain15;
#[doc = "HASHDATAIN16 (rw) register accessor: HASH Data Input 16 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain16`]
module"]
#[doc(alias = "HASHDATAIN16")]
pub type Hashdatain16 = crate::Reg<hashdatain16::Hashdatain16Spec>;
#[doc = "HASH Data Input 16 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain16;
#[doc = "HASHDATAIN17 (rw) register accessor: HASH Data Input 17 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain17`]
module"]
#[doc(alias = "HASHDATAIN17")]
pub type Hashdatain17 = crate::Reg<hashdatain17::Hashdatain17Spec>;
#[doc = "HASH Data Input 17 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain17;
#[doc = "HASHDATAIN18 (rw) register accessor: HASH Data Input 18 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain18`]
module"]
#[doc(alias = "HASHDATAIN18")]
pub type Hashdatain18 = crate::Reg<hashdatain18::Hashdatain18Spec>;
#[doc = "HASH Data Input 18 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain18;
#[doc = "HASHDATAIN19 (rw) register accessor: HASH Data Input 19 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain19`]
module"]
#[doc(alias = "HASHDATAIN19")]
pub type Hashdatain19 = crate::Reg<hashdatain19::Hashdatain19Spec>;
#[doc = "HASH Data Input 19 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain19;
#[doc = "HASHDATAIN20 (rw) register accessor: HASH Data Input 20 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain20`]
module"]
#[doc(alias = "HASHDATAIN20")]
pub type Hashdatain20 = crate::Reg<hashdatain20::Hashdatain20Spec>;
#[doc = "HASH Data Input 20 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain20;
#[doc = "HASHDATAIN21 (rw) register accessor: HASH Data Input 21 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain21`]
module"]
#[doc(alias = "HASHDATAIN21")]
pub type Hashdatain21 = crate::Reg<hashdatain21::Hashdatain21Spec>;
#[doc = "HASH Data Input 21 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain21;
#[doc = "HASHDATAIN22 (rw) register accessor: HASH Data Input 22 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain22`]
module"]
#[doc(alias = "HASHDATAIN22")]
pub type Hashdatain22 = crate::Reg<hashdatain22::Hashdatain22Spec>;
#[doc = "HASH Data Input 22 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain22;
#[doc = "HASHDATAIN23 (rw) register accessor: HASH Data Input 23 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain23`]
module"]
#[doc(alias = "HASHDATAIN23")]
pub type Hashdatain23 = crate::Reg<hashdatain23::Hashdatain23Spec>;
#[doc = "HASH Data Input 23 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain23;
#[doc = "HASHDATAIN24 (rw) register accessor: HASH Data Input 24 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain24`]
module"]
#[doc(alias = "HASHDATAIN24")]
pub type Hashdatain24 = crate::Reg<hashdatain24::Hashdatain24Spec>;
#[doc = "HASH Data Input 24 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain24;
#[doc = "HASHDATAIN25 (rw) register accessor: HASH Data Input 25 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain25`]
module"]
#[doc(alias = "HASHDATAIN25")]
pub type Hashdatain25 = crate::Reg<hashdatain25::Hashdatain25Spec>;
#[doc = "HASH Data Input 25 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain25;
#[doc = "HASHDATAIN26 (rw) register accessor: HASH Data Input 26 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain26`]
module"]
#[doc(alias = "HASHDATAIN26")]
pub type Hashdatain26 = crate::Reg<hashdatain26::Hashdatain26Spec>;
#[doc = "HASH Data Input 26 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain26;
#[doc = "HASHDATAIN27 (rw) register accessor: HASH Data Input 27 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain27`]
module"]
#[doc(alias = "HASHDATAIN27")]
pub type Hashdatain27 = crate::Reg<hashdatain27::Hashdatain27Spec>;
#[doc = "HASH Data Input 27 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain27;
#[doc = "HASHDATAIN28 (rw) register accessor: HASH Data Input 28 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain28`]
module"]
#[doc(alias = "HASHDATAIN28")]
pub type Hashdatain28 = crate::Reg<hashdatain28::Hashdatain28Spec>;
#[doc = "HASH Data Input 28 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain28;
#[doc = "HASHDATAIN29 (rw) register accessor: HASH Data Input 29 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain29`]
module"]
#[doc(alias = "HASHDATAIN29")]
pub type Hashdatain29 = crate::Reg<hashdatain29::Hashdatain29Spec>;
#[doc = "HASH Data Input 29 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain29;
#[doc = "HASHDATAIN30 (rw) register accessor: HASH Data Input 30 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain30`]
module"]
#[doc(alias = "HASHDATAIN30")]
pub type Hashdatain30 = crate::Reg<hashdatain30::Hashdatain30Spec>;
#[doc = "HASH Data Input 30 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain30;
#[doc = "HASHDATAIN31 (rw) register accessor: HASH Data Input 31 The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdatain31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdatain31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdatain31`]
module"]
#[doc(alias = "HASHDATAIN31")]
pub type Hashdatain31 = crate::Reg<hashdatain31::Hashdatain31Spec>;
#[doc = "HASH Data Input 31 The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hashdatain31;
#[doc = "HASHIOBUFCTRL (rw) register accessor: HASH Input_Output Buffer Control This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashiobufctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashiobufctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashiobufctrl`]
module"]
#[doc(alias = "HASHIOBUFCTRL")]
pub type Hashiobufctrl = crate::Reg<hashiobufctrl::HashiobufctrlSpec>;
#[doc = "HASH Input_Output Buffer Control This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine."]
pub mod hashiobufctrl;
#[doc = "HASHMODE (rw) register accessor: HASH Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashmode`]
module"]
#[doc(alias = "HASHMODE")]
pub type Hashmode = crate::Reg<hashmode::HashmodeSpec>;
#[doc = "HASH Mode"]
pub mod hashmode;
#[doc = "HASHINLENL (rw) register accessor: HASH Input Length LSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashinlenl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashinlenl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashinlenl`]
module"]
#[doc(alias = "HASHINLENL")]
pub type Hashinlenl = crate::Reg<hashinlenl::HashinlenlSpec>;
#[doc = "HASH Input Length LSB"]
pub mod hashinlenl;
#[doc = "HASHINLENH (rw) register accessor: HASH Input Length MSB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashinlenh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashinlenh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashinlenh`]
module"]
#[doc(alias = "HASHINLENH")]
pub type Hashinlenh = crate::Reg<hashinlenh::HashinlenhSpec>;
#[doc = "HASH Input Length MSB"]
pub mod hashinlenh;
#[doc = "HASHDIGESTA (rw) register accessor: HASH Digest A The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigesta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigesta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigesta`]
module"]
#[doc(alias = "HASHDIGESTA")]
pub type Hashdigesta = crate::Reg<hashdigesta::HashdigestaSpec>;
#[doc = "HASH Digest A The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesta;
#[doc = "HASHDIGESTB (rw) register accessor: HASH Digest B The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigestb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigestb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigestb`]
module"]
#[doc(alias = "HASHDIGESTB")]
pub type Hashdigestb = crate::Reg<hashdigestb::HashdigestbSpec>;
#[doc = "HASH Digest B The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestb;
#[doc = "HASHDIGESTC (rw) register accessor: HASH Digest C The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigestc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigestc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigestc`]
module"]
#[doc(alias = "HASHDIGESTC")]
pub type Hashdigestc = crate::Reg<hashdigestc::HashdigestcSpec>;
#[doc = "HASH Digest C The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestc;
#[doc = "HASHDIGESTD (rw) register accessor: HASH Digest D The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigestd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigestd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigestd`]
module"]
#[doc(alias = "HASHDIGESTD")]
pub type Hashdigestd = crate::Reg<hashdigestd::HashdigestdSpec>;
#[doc = "HASH Digest D The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestd;
#[doc = "HASHDIGESTE (rw) register accessor: HASH Digest E The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigeste::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigeste::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigeste`]
module"]
#[doc(alias = "HASHDIGESTE")]
pub type Hashdigeste = crate::Reg<hashdigeste::HashdigesteSpec>;
#[doc = "HASH Digest E The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigeste;
#[doc = "HASHDIGESTF (rw) register accessor: HASH Digest F The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigestf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigestf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigestf`]
module"]
#[doc(alias = "HASHDIGESTF")]
pub type Hashdigestf = crate::Reg<hashdigestf::HashdigestfSpec>;
#[doc = "HASH Digest F The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestf;
#[doc = "HASHDIGESTG (rw) register accessor: HASH Digest G The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigestg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigestg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigestg`]
module"]
#[doc(alias = "HASHDIGESTG")]
pub type Hashdigestg = crate::Reg<hashdigestg::HashdigestgSpec>;
#[doc = "HASH Digest G The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestg;
#[doc = "HASHDIGESTH (rw) register accessor: HASH Digest H The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigesth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigesth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigesth`]
module"]
#[doc(alias = "HASHDIGESTH")]
pub type Hashdigesth = crate::Reg<hashdigesth::HashdigesthSpec>;
#[doc = "HASH Digest H The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesth;
#[doc = "HASHDIGESTI (rw) register accessor: HASH Digest I The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigesti::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigesti::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigesti`]
module"]
#[doc(alias = "HASHDIGESTI")]
pub type Hashdigesti = crate::Reg<hashdigesti::HashdigestiSpec>;
#[doc = "HASH Digest I The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesti;
#[doc = "HASHDIGESTJ (rw) register accessor: HASH Digest J The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigestj::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigestj::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigestj`]
module"]
#[doc(alias = "HASHDIGESTJ")]
pub type Hashdigestj = crate::Reg<hashdigestj::HashdigestjSpec>;
#[doc = "HASH Digest J The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestj;
#[doc = "HASHDIGESTK (rw) register accessor: HASH Digest K The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigestk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigestk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigestk`]
module"]
#[doc(alias = "HASHDIGESTK")]
pub type Hashdigestk = crate::Reg<hashdigestk::HashdigestkSpec>;
#[doc = "HASH Digest K The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestk;
#[doc = "HASHDIGESTL (rw) register accessor: HASH Digest L The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigestl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigestl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigestl`]
module"]
#[doc(alias = "HASHDIGESTL")]
pub type Hashdigestl = crate::Reg<hashdigestl::HashdigestlSpec>;
#[doc = "HASH Digest L The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestl;
#[doc = "HASHDIGESTM (rw) register accessor: HASH Digest M The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigestm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigestm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigestm`]
module"]
#[doc(alias = "HASHDIGESTM")]
pub type Hashdigestm = crate::Reg<hashdigestm::HashdigestmSpec>;
#[doc = "HASH Digest M The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestm;
#[doc = "HASHDIGESTN (rw) register accessor: HASH Digest N The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigestn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigestn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigestn`]
module"]
#[doc(alias = "HASHDIGESTN")]
pub type Hashdigestn = crate::Reg<hashdigestn::HashdigestnSpec>;
#[doc = "HASH Digest N The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestn;
#[doc = "HASHDIGESTO (rw) register accessor: HASH Digest 0 The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigesto::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigesto::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigesto`]
module"]
#[doc(alias = "HASHDIGESTO")]
pub type Hashdigesto = crate::Reg<hashdigesto::HashdigestoSpec>;
#[doc = "HASH Digest 0 The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigesto;
#[doc = "HASHDIGESTP (rw) register accessor: HASH Digest P The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hashdigestp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hashdigestp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashdigestp`]
module"]
#[doc(alias = "HASHDIGESTP")]
pub type Hashdigestp = crate::Reg<hashdigestp::HashdigestpSpec>;
#[doc = "HASH Digest P The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hashdigestp;
#[doc = "ALGSEL (rw) register accessor: Algorithm Select This algorithm selection register configures the internal destination of the DMA controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`algsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`algsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@algsel`]
module"]
#[doc(alias = "ALGSEL")]
pub type Algsel = crate::Reg<algsel::AlgselSpec>;
#[doc = "Algorithm Select This algorithm selection register configures the internal destination of the DMA controller."]
pub mod algsel;
#[doc = "DMAPROTCTL (rw) register accessor: DMA Protection Control Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaprotctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaprotctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmaprotctl`]
module"]
#[doc(alias = "DMAPROTCTL")]
pub type Dmaprotctl = crate::Reg<dmaprotctl::DmaprotctlSpec>;
#[doc = "DMA Protection Control Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module."]
pub mod dmaprotctl;
#[doc = "SWRESET (rw) register accessor: Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swreset`]
module"]
#[doc(alias = "SWRESET")]
pub type Swreset = crate::Reg<swreset::SwresetSpec>;
#[doc = "Software Reset"]
pub mod swreset;
#[doc = "IRQTYPE (rw) register accessor: Control Interrupt Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqtype`]
module"]
#[doc(alias = "IRQTYPE")]
pub type Irqtype = crate::Reg<irqtype::IrqtypeSpec>;
#[doc = "Control Interrupt Configuration"]
pub mod irqtype;
#[doc = "IRQEN (rw) register accessor: Control Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqen`]
module"]
#[doc(alias = "IRQEN")]
pub type Irqen = crate::Reg<irqen::IrqenSpec>;
#[doc = "Control Interrupt Enable"]
pub mod irqen;
#[doc = "IRQCLR (rw) register accessor: Control Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqclr`]
module"]
#[doc(alias = "IRQCLR")]
pub type Irqclr = crate::Reg<irqclr::IrqclrSpec>;
#[doc = "Control Interrupt Clear"]
pub mod irqclr;
#[doc = "IRQSET (rw) register accessor: Control Interrupt Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqset`]
module"]
#[doc(alias = "IRQSET")]
pub type Irqset = crate::Reg<irqset::IrqsetSpec>;
#[doc = "Control Interrupt Set"]
pub mod irqset;
#[doc = "IRQSTAT (rw) register accessor: Control Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqstat`]
module"]
#[doc(alias = "IRQSTAT")]
pub type Irqstat = crate::Reg<irqstat::IrqstatSpec>;
#[doc = "Control Interrupt Status"]
pub mod irqstat;
#[doc = "HWVER (rw) register accessor: Hardware Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwver::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwver::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwver`]
module"]
#[doc(alias = "HWVER")]
pub type Hwver = crate::Reg<hwver::HwverSpec>;
#[doc = "Hardware Version"]
pub mod hwver;
