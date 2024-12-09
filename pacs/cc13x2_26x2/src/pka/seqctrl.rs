#[doc = "Register `SEQCTRL` reader"]
pub type R = crate::R<SeqctrlSpec>;
#[doc = "Register `SEQCTRL` writer"]
pub type W = crate::W<SeqctrlSpec>;
#[doc = "Field `SW_CONTROL_STAT` reader - 7:0\\]
These bits can be used by software to trigger sequencer operations. External logic can set these bits by writing 1b, cannot reset them by writing 0b. The sequencer can reset these bits by writing 0b, cannot set them by writing 1b. Setting the FUNCTION.RUN bit together with a nonzero sequencer operations field automatically sets bit \\[0\\]
here. This field should always be written with zeroes and ignored when reading this register."]
pub type SwControlStatR = crate::FieldReader;
#[doc = "Field `SW_CONTROL_STAT` writer - 7:0\\]
These bits can be used by software to trigger sequencer operations. External logic can set these bits by writing 1b, cannot reset them by writing 0b. The sequencer can reset these bits by writing 0b, cannot set them by writing 1b. Setting the FUNCTION.RUN bit together with a nonzero sequencer operations field automatically sets bit \\[0\\]
here. This field should always be written with zeroes and ignored when reading this register."]
pub type SwControlStatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEQUENCER_STAT` reader - 15:8\\]
These read-only bits can be used by the sequencer to communicate status to the outside world. Bit \\[8\\]
is also used as sequencer interrupt, with the complement of this bit ORed into the FUNCTION.RUN bit. This field should always be written with zeroes and ignored when reading this register."]
pub type SequencerStatR = crate::FieldReader;
#[doc = "Field `RESERVED16` reader - 30:16\\]
Set to zero on write, ignore on read"]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 30:16\\]
Set to zero on write, ignore on read"]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `RESET` reader - 31:31\\]
Option program ROM: Reset value = 0. Read/Write, reset value 0b (ZERO). Writing 1b resets the sequencer, write to 0b to restart operations again. As the reset value is 0b, the sequencer will automatically start operations executing from program ROM. This bit should always be written with zero and ignored when reading this register. Option Program RAM: Reset value =1. Read/Write, reset value 1b (ONE). When 1b, the sequencer is held in a reset state and the PKA_PROGRAM area is accessible for loading the sequencer program (while the PKA_DATA_RAM is inaccessible), write to 0b to (re)start sequencer operations and disable PKA_PROGRAM area accessibility (also enables the PKA_DATA_RAM accesses). Resetting the sequencer (in order to load other firmware) should only be done when the PKA Engine is not performing any operations (i.e. the FUNCTION.RUN bit should be zero)."]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - 31:31\\]
Option program ROM: Reset value = 0. Read/Write, reset value 0b (ZERO). Writing 1b resets the sequencer, write to 0b to restart operations again. As the reset value is 0b, the sequencer will automatically start operations executing from program ROM. This bit should always be written with zero and ignored when reading this register. Option Program RAM: Reset value =1. Read/Write, reset value 1b (ONE). When 1b, the sequencer is held in a reset state and the PKA_PROGRAM area is accessible for loading the sequencer program (while the PKA_DATA_RAM is inaccessible), write to 0b to (re)start sequencer operations and disable PKA_PROGRAM area accessibility (also enables the PKA_DATA_RAM accesses). Resetting the sequencer (in order to load other firmware) should only be done when the PKA Engine is not performing any operations (i.e. the FUNCTION.RUN bit should be zero)."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
These bits can be used by software to trigger sequencer operations. External logic can set these bits by writing 1b, cannot reset them by writing 0b. The sequencer can reset these bits by writing 0b, cannot set them by writing 1b. Setting the FUNCTION.RUN bit together with a nonzero sequencer operations field automatically sets bit \\[0\\]
here. This field should always be written with zeroes and ignored when reading this register."]
    #[inline(always)]
    pub fn sw_control_stat(&self) -> SwControlStatR {
        SwControlStatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
These read-only bits can be used by the sequencer to communicate status to the outside world. Bit \\[8\\]
is also used as sequencer interrupt, with the complement of this bit ORed into the FUNCTION.RUN bit. This field should always be written with zeroes and ignored when reading this register."]
    #[inline(always)]
    pub fn sequencer_stat(&self) -> SequencerStatR {
        SequencerStatR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Option program ROM: Reset value = 0. Read/Write, reset value 0b (ZERO). Writing 1b resets the sequencer, write to 0b to restart operations again. As the reset value is 0b, the sequencer will automatically start operations executing from program ROM. This bit should always be written with zero and ignored when reading this register. Option Program RAM: Reset value =1. Read/Write, reset value 1b (ONE). When 1b, the sequencer is held in a reset state and the PKA_PROGRAM area is accessible for loading the sequencer program (while the PKA_DATA_RAM is inaccessible), write to 0b to (re)start sequencer operations and disable PKA_PROGRAM area accessibility (also enables the PKA_DATA_RAM accesses). Resetting the sequencer (in order to load other firmware) should only be done when the PKA Engine is not performing any operations (i.e. the FUNCTION.RUN bit should be zero)."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
These bits can be used by software to trigger sequencer operations. External logic can set these bits by writing 1b, cannot reset them by writing 0b. The sequencer can reset these bits by writing 0b, cannot set them by writing 1b. Setting the FUNCTION.RUN bit together with a nonzero sequencer operations field automatically sets bit \\[0\\]
here. This field should always be written with zeroes and ignored when reading this register."]
    #[inline(always)]
    #[must_use]
    pub fn sw_control_stat(&mut self) -> SwControlStatW<SeqctrlSpec> {
        SwControlStatW::new(self, 0)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<SeqctrlSpec> {
        Reserved16W::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
Option program ROM: Reset value = 0. Read/Write, reset value 0b (ZERO). Writing 1b resets the sequencer, write to 0b to restart operations again. As the reset value is 0b, the sequencer will automatically start operations executing from program ROM. This bit should always be written with zero and ignored when reading this register. Option Program RAM: Reset value =1. Read/Write, reset value 1b (ONE). When 1b, the sequencer is held in a reset state and the PKA_PROGRAM area is accessible for loading the sequencer program (while the PKA_DATA_RAM is inaccessible), write to 0b to (re)start sequencer operations and disable PKA_PROGRAM area accessibility (also enables the PKA_DATA_RAM accesses). Resetting the sequencer (in order to load other firmware) should only be done when the PKA Engine is not performing any operations (i.e. the FUNCTION.RUN bit should be zero)."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<SeqctrlSpec> {
        ResetW::new(self, 31)
    }
}
#[doc = "PKA sequencer control and status register The sequencer is interfaced with the outside world through a single control and status register. With the exception of bit \\[31\\], the actual use of bits in the separate sub-fields of this register is determined by the sequencer firmware. This register need only be accessed when the sequencer program is stored in RAM. The reset value of the RESET bit depends upon the option chosen for sequencer program storage. NOTE: For Agama the sequencer firmware is executed from ROM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seqctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seqctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqctrlSpec;
impl crate::RegisterSpec for SeqctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqctrl::R`](R) reader structure"]
impl crate::Readable for SeqctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`seqctrl::W`](W) writer structure"]
impl crate::Writable for SeqctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQCTRL to value 0x0100"]
impl crate::Resettable for SeqctrlSpec {
    const RESET_VALUE: u32 = 0x0100;
}
