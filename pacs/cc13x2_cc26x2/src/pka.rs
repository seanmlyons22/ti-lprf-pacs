#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PKA Vector A Address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    pub aptr: APTR,
    #[doc = "0x04 - PKA Vector B Address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    pub bptr: BPTR,
    #[doc = "0x08 - PKA Vector C Address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    pub cptr: CPTR,
    #[doc = "0x0c - PKA Vector D Address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    pub dptr: DPTR,
    #[doc = "0x10 - PKA Vector A Length During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    pub alength: ALENGTH,
    #[doc = "0x14 - PKA Vector B Length During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
    pub blength: BLENGTH,
    #[doc = "0x18 - PKA Bit Shift Value For basic PKCP operations, modifying the contents of this register is made impossible while the operation is being performed. For the ExpMod-variable and ExpMod-CRT operations, this register is used to indicate the number of odd powers to use (directly as a value in the range 1-16). For the ModInv and ECC operations, this register is used to hold a completion code."]
    pub shift: SHIFT,
    #[doc = "0x1c - PKA Function This register contains the control bits to start basic PKCP as well as complex sequencer operations. The run bit can be used to poll for the completion of the operation. Modifying bits \\[11:0\\]
is made impossible during the execution of a basic PKCP operation. During the execution of sequencer-controlled complex operations, this register is modified, the run and stall result bits are set to zero at the conclusion, but other bits are undefined. NOTE: Continuously reading this register to poll the run bit is not allowed when executing complex sequencer operations (the sequencer cannot access the PKCP when this is done). Leave at least one sysclk cycle between poll operations."]
    pub function: FUNCTION,
    #[doc = "0x20 - PKA compare result This register provides the result of a basic PKCP compare operation. It is updated when the FUNCTION.RUN bit is reset at the end of that operation. Status after a complex sequencer operation is unknown"]
    pub compare: COMPARE,
    #[doc = "0x24 - PKA most-significant-word of result vector This register indicates the (word) address in the PKA RAM where the most significant nonzero 32-bit word of the result is stored. Should be ignored for modulo operations. For basic PKCP operations, this register is updated FUNCTION.RUN bit is reset at the end of the operation. For the complex-sequencer controlled operations, updating of the final value matching the actual result is done near the end of the operation; note that the result is only meaningful if no errors were detected and that for ECC operations, this register will provide information for the x-coordinate of the result point only."]
    pub msw: MSW,
    #[doc = "0x28 - PKA most-significant-word of divide remainder This register indicates the (32-bit word) address in the PKA RAM where the most significant nonzero 32-bit word of the remainder result for the basic divide and modulo operations is stored. Bits \\[4:0\\]
are loaded with the bit number of the most-significant nonzero bit in the most-significant nonzero word when MS one control bit is set. For divide, modulo, and MS one reporting, this register is updated when FUNCTION.RUN bit is reset at the end of the operation. For the complex sequencer controlled operations, updating of bits \\[4:0\\]
of this register with the most-significant bit location of the actual result is done near the end of the operation. The result is meaningful only if no errors were detected and that for ECC operations; this register provides information for the x-coordinate of the result point only."]
    pub divmsw: DIVMSW,
    _reserved11: [u8; 0x9c],
    #[doc = "0xc8 - PKA sequencer control and status register The sequencer is interfaced with the outside world through a single control and status register. With the exception of bit \\[31\\], the actual use of bits in the separate sub-fields of this register is determined by the sequencer firmware. This register need only be accessed when the sequencer program is stored in RAM. The reset value of the RESET bit depends upon the option chosen for sequencer program storage. NOTE: For Agama the sequencer firmware is executed from ROM."]
    pub seqctrl: SEQCTRL,
    _reserved12: [u8; 0x28],
    #[doc = "0xf4 - PKA hardware options register This register provides the host with a means to determine the hardware configuration implemented in this PKA engine, focused on options that have an effect on software interacting with the module."]
    pub options: OPTIONS,
    #[doc = "0xf8 - PKA firmware revision and capabilities register This register allows the host access to the internal firmware revision number of the PKA Engine for software driver matching and diagnostic purposes. This register also contains a field that encodes the capabilities of the embedded firmware. This register is written by the firmware within a few clock cycles after starting up that firmware. The hardware reset value is zero, indicating that the information has not been written yet."]
    pub fwrev: FWREV,
    #[doc = "0xfc - PKA hardware revision register This register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module."]
    pub hwrev: HWREV,
}
#[doc = "APTR (rw) register accessor: an alias for `Reg<APTR_SPEC>`"]
pub type APTR = crate::Reg<aptr::APTR_SPEC>;
#[doc = "PKA Vector A Address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod aptr;
#[doc = "BPTR (rw) register accessor: an alias for `Reg<BPTR_SPEC>`"]
pub type BPTR = crate::Reg<bptr::BPTR_SPEC>;
#[doc = "PKA Vector B Address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod bptr;
#[doc = "CPTR (rw) register accessor: an alias for `Reg<CPTR_SPEC>`"]
pub type CPTR = crate::Reg<cptr::CPTR_SPEC>;
#[doc = "PKA Vector C Address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod cptr;
#[doc = "DPTR (rw) register accessor: an alias for `Reg<DPTR_SPEC>`"]
pub type DPTR = crate::Reg<dptr::DPTR_SPEC>;
#[doc = "PKA Vector D Address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod dptr;
#[doc = "ALENGTH (rw) register accessor: an alias for `Reg<ALENGTH_SPEC>`"]
pub type ALENGTH = crate::Reg<alength::ALENGTH_SPEC>;
#[doc = "PKA Vector A Length During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod alength;
#[doc = "BLENGTH (rw) register accessor: an alias for `Reg<BLENGTH_SPEC>`"]
pub type BLENGTH = crate::Reg<blength::BLENGTH_SPEC>;
#[doc = "PKA Vector B Length During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact."]
pub mod blength;
#[doc = "SHIFT (rw) register accessor: an alias for `Reg<SHIFT_SPEC>`"]
pub type SHIFT = crate::Reg<shift::SHIFT_SPEC>;
#[doc = "PKA Bit Shift Value For basic PKCP operations, modifying the contents of this register is made impossible while the operation is being performed. For the ExpMod-variable and ExpMod-CRT operations, this register is used to indicate the number of odd powers to use (directly as a value in the range 1-16). For the ModInv and ECC operations, this register is used to hold a completion code."]
pub mod shift;
#[doc = "FUNCTION (rw) register accessor: an alias for `Reg<FUNCTION_SPEC>`"]
pub type FUNCTION = crate::Reg<function::FUNCTION_SPEC>;
#[doc = "PKA Function This register contains the control bits to start basic PKCP as well as complex sequencer operations. The run bit can be used to poll for the completion of the operation. Modifying bits \\[11:0\\]
is made impossible during the execution of a basic PKCP operation. During the execution of sequencer-controlled complex operations, this register is modified, the run and stall result bits are set to zero at the conclusion, but other bits are undefined. NOTE: Continuously reading this register to poll the run bit is not allowed when executing complex sequencer operations (the sequencer cannot access the PKCP when this is done). Leave at least one sysclk cycle between poll operations."]
pub mod function;
#[doc = "COMPARE (rw) register accessor: an alias for `Reg<COMPARE_SPEC>`"]
pub type COMPARE = crate::Reg<compare::COMPARE_SPEC>;
#[doc = "PKA compare result This register provides the result of a basic PKCP compare operation. It is updated when the FUNCTION.RUN bit is reset at the end of that operation. Status after a complex sequencer operation is unknown"]
pub mod compare;
#[doc = "MSW (rw) register accessor: an alias for `Reg<MSW_SPEC>`"]
pub type MSW = crate::Reg<msw::MSW_SPEC>;
#[doc = "PKA most-significant-word of result vector This register indicates the (word) address in the PKA RAM where the most significant nonzero 32-bit word of the result is stored. Should be ignored for modulo operations. For basic PKCP operations, this register is updated FUNCTION.RUN bit is reset at the end of the operation. For the complex-sequencer controlled operations, updating of the final value matching the actual result is done near the end of the operation; note that the result is only meaningful if no errors were detected and that for ECC operations, this register will provide information for the x-coordinate of the result point only."]
pub mod msw;
#[doc = "DIVMSW (rw) register accessor: an alias for `Reg<DIVMSW_SPEC>`"]
pub type DIVMSW = crate::Reg<divmsw::DIVMSW_SPEC>;
#[doc = "PKA most-significant-word of divide remainder This register indicates the (32-bit word) address in the PKA RAM where the most significant nonzero 32-bit word of the remainder result for the basic divide and modulo operations is stored. Bits \\[4:0\\]
are loaded with the bit number of the most-significant nonzero bit in the most-significant nonzero word when MS one control bit is set. For divide, modulo, and MS one reporting, this register is updated when FUNCTION.RUN bit is reset at the end of the operation. For the complex sequencer controlled operations, updating of bits \\[4:0\\]
of this register with the most-significant bit location of the actual result is done near the end of the operation. The result is meaningful only if no errors were detected and that for ECC operations; this register provides information for the x-coordinate of the result point only."]
pub mod divmsw;
#[doc = "SEQCTRL (rw) register accessor: an alias for `Reg<SEQCTRL_SPEC>`"]
pub type SEQCTRL = crate::Reg<seqctrl::SEQCTRL_SPEC>;
#[doc = "PKA sequencer control and status register The sequencer is interfaced with the outside world through a single control and status register. With the exception of bit \\[31\\], the actual use of bits in the separate sub-fields of this register is determined by the sequencer firmware. This register need only be accessed when the sequencer program is stored in RAM. The reset value of the RESET bit depends upon the option chosen for sequencer program storage. NOTE: For Agama the sequencer firmware is executed from ROM."]
pub mod seqctrl;
#[doc = "OPTIONS (rw) register accessor: an alias for `Reg<OPTIONS_SPEC>`"]
pub type OPTIONS = crate::Reg<options::OPTIONS_SPEC>;
#[doc = "PKA hardware options register This register provides the host with a means to determine the hardware configuration implemented in this PKA engine, focused on options that have an effect on software interacting with the module."]
pub mod options;
#[doc = "FWREV (rw) register accessor: an alias for `Reg<FWREV_SPEC>`"]
pub type FWREV = crate::Reg<fwrev::FWREV_SPEC>;
#[doc = "PKA firmware revision and capabilities register This register allows the host access to the internal firmware revision number of the PKA Engine for software driver matching and diagnostic purposes. This register also contains a field that encodes the capabilities of the embedded firmware. This register is written by the firmware within a few clock cycles after starting up that firmware. The hardware reset value is zero, indicating that the information has not been written yet."]
pub mod fwrev;
#[doc = "HWREV (rw) register accessor: an alias for `Reg<HWREV_SPEC>`"]
pub type HWREV = crate::Reg<hwrev::HWREV_SPEC>;
#[doc = "PKA hardware revision register This register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module."]
pub mod hwrev;
