#[doc = "Register `FUNCTION` reader"]
pub type R = crate::R<FunctionSpec>;
#[doc = "Register `FUNCTION` writer"]
pub type W = crate::W<FunctionSpec>;
#[doc = "Field `MULTIPLY` reader - 0:0\\]
Perform multiply operation"]
pub type MultiplyR = crate::BitReader;
#[doc = "Field `MULTIPLY` writer - 0:0\\]
Perform multiply operation"]
pub type MultiplyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDSUB` reader - 1:1\\]
Perform combined add/subtract operation"]
pub type AddsubR = crate::BitReader;
#[doc = "Field `ADDSUB` writer - 1:1\\]
Perform combined add/subtract operation"]
pub type AddsubW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 2:2\\]
Set to zero on write, ignore on read"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `RESERVED2` writer - 2:2\\]
Set to zero on write, ignore on read"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS_ONE` reader - 3:3\\]
Loads the location of the Most Significant one bit within the result word indicated in the MSW register into bits \\[4:0\\]
of the DIVMSW.MSW_ADDRESS register - can only be used with basic PKCP operations, except for Divide, Modulo and Compare."]
pub type MsOneR = crate::BitReader;
#[doc = "Field `MS_ONE` writer - 3:3\\]
Loads the location of the Most Significant one bit within the result word indicated in the MSW register into bits \\[4:0\\]
of the DIVMSW.MSW_ADDRESS register - can only be used with basic PKCP operations, except for Divide, Modulo and Compare."]
pub type MsOneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD` reader - 4:4\\]
Perform add operation"]
pub type AddR = crate::BitReader;
#[doc = "Field `ADD` writer - 4:4\\]
Perform add operation"]
pub type AddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUBTRACT` reader - 5:5\\]
Perform subtract operation"]
pub type SubtractR = crate::BitReader;
#[doc = "Field `SUBTRACT` writer - 5:5\\]
Perform subtract operation"]
pub type SubtractW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSHIFT` reader - 6:6\\]
Perform right shift operation"]
pub type RshiftR = crate::BitReader;
#[doc = "Field `RSHIFT` writer - 6:6\\]
Perform right shift operation"]
pub type RshiftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSHIFT` reader - 7:7\\]
Perform left shift operation"]
pub type LshiftR = crate::BitReader;
#[doc = "Field `LSHIFT` writer - 7:7\\]
Perform left shift operation"]
pub type LshiftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVIDE` reader - 8:8\\]
Perform divide operation"]
pub type DivideR = crate::BitReader;
#[doc = "Field `DIVIDE` writer - 8:8\\]
Perform divide operation"]
pub type DivideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODULO` reader - 9:9\\]
Perform modulo operation"]
pub type ModuloR = crate::BitReader;
#[doc = "Field `MODULO` writer - 9:9\\]
Perform modulo operation"]
pub type ModuloW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPARE` reader - 10:10\\]
Perform compare operation"]
pub type CompareR = crate::BitReader;
#[doc = "Field `COMPARE` writer - 10:10\\]
Perform compare operation"]
pub type CompareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COPY` reader - 11:11\\]
Perform copy operation"]
pub type CopyR = crate::BitReader;
#[doc = "Field `COPY` writer - 11:11\\]
Perform copy operation"]
pub type CopyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQUENCER_OPERATIONS` reader - 14:12\\]
These bits select the complex sequencer operation to perform: 0x0: None 0x1: ExpMod-CRT 0x2: ECmontMUL 0x3: ECC-ADD (if available in firmware, otherwise reserved) 0x4: ExpMod-ACT2 0x5: ECC-MUL (if available in firmware, otherwise reserved) 0x6: ExpMod-variable 0x7: ModInv (if available in firmware, otherwise reserved) The encoding of these operations is determined by sequencer firmware."]
pub type SequencerOperationsR = crate::FieldReader;
#[doc = "Field `SEQUENCER_OPERATIONS` writer - 14:12\\]
These bits select the complex sequencer operation to perform: 0x0: None 0x1: ExpMod-CRT 0x2: ECmontMUL 0x3: ECC-ADD (if available in firmware, otherwise reserved) 0x4: ExpMod-ACT2 0x5: ECC-MUL (if available in firmware, otherwise reserved) 0x6: ExpMod-variable 0x7: ModInv (if available in firmware, otherwise reserved) The encoding of these operations is determined by sequencer firmware."]
pub type SequencerOperationsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RUN` reader - 15:15\\]
The host sets this bit to instruct the PKA module to begin processing the basic PKCP or complex sequencer operation. This bit is reset low automatically when the operation is complete. After a reset, the run bit is always set to 1b. Depending on the option, program ROM or program RAM, the following applies: Program ROM - The first sequencer instruction sets the bit to 0b. This is done immediately after the hardware reset is released. Program RAM - The sequencer must set the bit to 0b. As a valid firmware may not have been loaded, the sequencer is held in software reset after the hardware reset is released (the SEQCTRL.RESET bit is set to 1b). After the FW image is loaded and the Reset bit is cleared, the sequencer starts to execute the FW. The first instruction clears the run bit. In both cases a few clock cycles are needed before the first instruction is executed and the run bit state has been propagated."]
pub type RunR = crate::BitReader;
#[doc = "Field `RUN` writer - 15:15\\]
The host sets this bit to instruct the PKA module to begin processing the basic PKCP or complex sequencer operation. This bit is reset low automatically when the operation is complete. After a reset, the run bit is always set to 1b. Depending on the option, program ROM or program RAM, the following applies: Program ROM - The first sequencer instruction sets the bit to 0b. This is done immediately after the hardware reset is released. Program RAM - The sequencer must set the bit to 0b. As a valid firmware may not have been loaded, the sequencer is held in software reset after the hardware reset is released (the SEQCTRL.RESET bit is set to 1b). After the FW image is loaded and the Reset bit is cleared, the sequencer starts to execute the FW. The first instruction clears the run bit. In both cases a few clock cycles are needed before the first instruction is executed and the run bit state has been propagated."]
pub type RunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED16` reader - 23:16\\]
Set to zero on write, ignore on read"]
pub type Reserved16R = crate::FieldReader;
#[doc = "Field `RESERVED16` writer - 23:16\\]
Set to zero on write, ignore on read"]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STALL_RESULT` reader - 24:24\\]
When written with a 1b, updating of the COMPARE bit, MSW and DIVMSW registers, as well as resetting the run bit is stalled beyond the point that a running operation is actually finished. Use this to allow software enough time to read results from a previous operation when the newly started operation is known to take only a short amount of time. If a result is waiting, the result registers is updated and the run bit is reset in the clock cycle following writing the stall result bit back to 0b. The Stall result function may only be used for basic PKCP operations."]
pub type StallResultR = crate::BitReader;
#[doc = "Field `STALL_RESULT` writer - 24:24\\]
When written with a 1b, updating of the COMPARE bit, MSW and DIVMSW registers, as well as resetting the run bit is stalled beyond the point that a running operation is actually finished. Use this to allow software enough time to read results from a previous operation when the newly started operation is known to take only a short amount of time. If a result is waiting, the result registers is updated and the run bit is reset in the clock cycle following writing the stall result bit back to 0b. The Stall result function may only be used for basic PKCP operations."]
pub type StallResultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Set to zero on write, ignore on read"]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Set to zero on write, ignore on read"]
pub type Reserved25W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Perform multiply operation"]
    #[inline(always)]
    pub fn multiply(&self) -> MultiplyR {
        MultiplyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Perform combined add/subtract operation"]
    #[inline(always)]
    pub fn addsub(&self) -> AddsubR {
        AddsubR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Loads the location of the Most Significant one bit within the result word indicated in the MSW register into bits \\[4:0\\]
of the DIVMSW.MSW_ADDRESS register - can only be used with basic PKCP operations, except for Divide, Modulo and Compare."]
    #[inline(always)]
    pub fn ms_one(&self) -> MsOneR {
        MsOneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Perform add operation"]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Perform subtract operation"]
    #[inline(always)]
    pub fn subtract(&self) -> SubtractR {
        SubtractR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Perform right shift operation"]
    #[inline(always)]
    pub fn rshift(&self) -> RshiftR {
        RshiftR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Perform left shift operation"]
    #[inline(always)]
    pub fn lshift(&self) -> LshiftR {
        LshiftR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Perform divide operation"]
    #[inline(always)]
    pub fn divide(&self) -> DivideR {
        DivideR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Perform modulo operation"]
    #[inline(always)]
    pub fn modulo(&self) -> ModuloR {
        ModuloR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Perform compare operation"]
    #[inline(always)]
    pub fn compare(&self) -> CompareR {
        CompareR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Perform copy operation"]
    #[inline(always)]
    pub fn copy(&self) -> CopyR {
        CopyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - 14:12\\]
These bits select the complex sequencer operation to perform: 0x0: None 0x1: ExpMod-CRT 0x2: ECmontMUL 0x3: ECC-ADD (if available in firmware, otherwise reserved) 0x4: ExpMod-ACT2 0x5: ECC-MUL (if available in firmware, otherwise reserved) 0x6: ExpMod-variable 0x7: ModInv (if available in firmware, otherwise reserved) The encoding of these operations is determined by sequencer firmware."]
    #[inline(always)]
    pub fn sequencer_operations(&self) -> SequencerOperationsR {
        SequencerOperationsR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
The host sets this bit to instruct the PKA module to begin processing the basic PKCP or complex sequencer operation. This bit is reset low automatically when the operation is complete. After a reset, the run bit is always set to 1b. Depending on the option, program ROM or program RAM, the following applies: Program ROM - The first sequencer instruction sets the bit to 0b. This is done immediately after the hardware reset is released. Program RAM - The sequencer must set the bit to 0b. As a valid firmware may not have been loaded, the sequencer is held in software reset after the hardware reset is released (the SEQCTRL.RESET bit is set to 1b). After the FW image is loaded and the Reset bit is cleared, the sequencer starts to execute the FW. The first instruction clears the run bit. In both cases a few clock cycles are needed before the first instruction is executed and the run bit state has been propagated."]
    #[inline(always)]
    pub fn run(&self) -> RunR {
        RunR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
When written with a 1b, updating of the COMPARE bit, MSW and DIVMSW registers, as well as resetting the run bit is stalled beyond the point that a running operation is actually finished. Use this to allow software enough time to read results from a previous operation when the newly started operation is known to take only a short amount of time. If a result is waiting, the result registers is updated and the run bit is reset in the clock cycle following writing the stall result bit back to 0b. The Stall result function may only be used for basic PKCP operations."]
    #[inline(always)]
    pub fn stall_result(&self) -> StallResultR {
        StallResultR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Perform multiply operation"]
    #[inline(always)]
    #[must_use]
    pub fn multiply(&mut self) -> MultiplyW<FunctionSpec> {
        MultiplyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Perform combined add/subtract operation"]
    #[inline(always)]
    #[must_use]
    pub fn addsub(&mut self) -> AddsubW<FunctionSpec> {
        AddsubW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<FunctionSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Loads the location of the Most Significant one bit within the result word indicated in the MSW register into bits \\[4:0\\]
of the DIVMSW.MSW_ADDRESS register - can only be used with basic PKCP operations, except for Divide, Modulo and Compare."]
    #[inline(always)]
    #[must_use]
    pub fn ms_one(&mut self) -> MsOneW<FunctionSpec> {
        MsOneW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Perform add operation"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> AddW<FunctionSpec> {
        AddW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Perform subtract operation"]
    #[inline(always)]
    #[must_use]
    pub fn subtract(&mut self) -> SubtractW<FunctionSpec> {
        SubtractW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Perform right shift operation"]
    #[inline(always)]
    #[must_use]
    pub fn rshift(&mut self) -> RshiftW<FunctionSpec> {
        RshiftW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Perform left shift operation"]
    #[inline(always)]
    #[must_use]
    pub fn lshift(&mut self) -> LshiftW<FunctionSpec> {
        LshiftW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Perform divide operation"]
    #[inline(always)]
    #[must_use]
    pub fn divide(&mut self) -> DivideW<FunctionSpec> {
        DivideW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Perform modulo operation"]
    #[inline(always)]
    #[must_use]
    pub fn modulo(&mut self) -> ModuloW<FunctionSpec> {
        ModuloW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Perform compare operation"]
    #[inline(always)]
    #[must_use]
    pub fn compare(&mut self) -> CompareW<FunctionSpec> {
        CompareW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Perform copy operation"]
    #[inline(always)]
    #[must_use]
    pub fn copy(&mut self) -> CopyW<FunctionSpec> {
        CopyW::new(self, 11)
    }
    #[doc = "Bits 12:14 - 14:12\\]
These bits select the complex sequencer operation to perform: 0x0: None 0x1: ExpMod-CRT 0x2: ECmontMUL 0x3: ECC-ADD (if available in firmware, otherwise reserved) 0x4: ExpMod-ACT2 0x5: ECC-MUL (if available in firmware, otherwise reserved) 0x6: ExpMod-variable 0x7: ModInv (if available in firmware, otherwise reserved) The encoding of these operations is determined by sequencer firmware."]
    #[inline(always)]
    #[must_use]
    pub fn sequencer_operations(&mut self) -> SequencerOperationsW<FunctionSpec> {
        SequencerOperationsW::new(self, 12)
    }
    #[doc = "Bit 15 - 15:15\\]
The host sets this bit to instruct the PKA module to begin processing the basic PKCP or complex sequencer operation. This bit is reset low automatically when the operation is complete. After a reset, the run bit is always set to 1b. Depending on the option, program ROM or program RAM, the following applies: Program ROM - The first sequencer instruction sets the bit to 0b. This is done immediately after the hardware reset is released. Program RAM - The sequencer must set the bit to 0b. As a valid firmware may not have been loaded, the sequencer is held in software reset after the hardware reset is released (the SEQCTRL.RESET bit is set to 1b). After the FW image is loaded and the Reset bit is cleared, the sequencer starts to execute the FW. The first instruction clears the run bit. In both cases a few clock cycles are needed before the first instruction is executed and the run bit state has been propagated."]
    #[inline(always)]
    #[must_use]
    pub fn run(&mut self) -> RunW<FunctionSpec> {
        RunW::new(self, 15)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<FunctionSpec> {
        Reserved16W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
When written with a 1b, updating of the COMPARE bit, MSW and DIVMSW registers, as well as resetting the run bit is stalled beyond the point that a running operation is actually finished. Use this to allow software enough time to read results from a previous operation when the newly started operation is known to take only a short amount of time. If a result is waiting, the result registers is updated and the run bit is reset in the clock cycle following writing the stall result bit back to 0b. The Stall result function may only be used for basic PKCP operations."]
    #[inline(always)]
    #[must_use]
    pub fn stall_result(&mut self) -> StallResultW<FunctionSpec> {
        StallResultW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> Reserved25W<FunctionSpec> {
        Reserved25W::new(self, 25)
    }
}
#[doc = "PKA Function This register contains the control bits to start basic PKCP as well as complex sequencer operations. The run bit can be used to poll for the completion of the operation. Modifying bits \\[11:0\\]
is made impossible during the execution of a basic PKCP operation. During the execution of sequencer-controlled complex operations, this register is modified, the run and stall result bits are set to zero at the conclusion, but other bits are undefined. NOTE: Continuously reading this register to poll the run bit is not allowed when executing complex sequencer operations (the sequencer cannot access the PKCP when this is done). Leave at least one sysclk cycle between poll operations.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`function::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`function::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FunctionSpec;
impl crate::RegisterSpec for FunctionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`function::R`](R) reader structure"]
impl crate::Readable for FunctionSpec {}
#[doc = "`write(|w| ..)` method takes [`function::W`](W) writer structure"]
impl crate::Writable for FunctionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FUNCTION to value 0x8000"]
impl crate::Resettable for FunctionSpec {
    const RESET_VALUE: u32 = 0x8000;
}
