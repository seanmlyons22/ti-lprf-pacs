#[doc = "Register `PRECNTR` reader"]
pub type R = crate::R<PrecntrSpec>;
#[doc = "Register `PRECNTR` writer"]
pub type W = crate::W<PrecntrSpec>;
#[doc = "Field `CNT` reader - 15:0\\]
Prescaler counter value. Write a value to CNT to capture the value of the 16-bit prescaler counter into CNT. Read CNT to get the captured value. The read value gets 1 LSB uncertainty if the event source level rises when you release the reset. The read value gets 1 LSB uncertainty if the event source level rises when you capture the prescaler counter. Please note the following: - The prescaler counter is reset to 2 by PRECTL.RESET_N. - The captured value is 2 when the number of rising edges on prescaler input is less than 3. Otherwise, captured value equals number of event pulses - 1."]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - 15:0\\]
Prescaler counter value. Write a value to CNT to capture the value of the 16-bit prescaler counter into CNT. Read CNT to get the captured value. The read value gets 1 LSB uncertainty if the event source level rises when you release the reset. The read value gets 1 LSB uncertainty if the event source level rises when you capture the prescaler counter. Please note the following: - The prescaler counter is reset to 2 by PRECTL.RESET_N. - The captured value is 2 when the number of rising edges on prescaler input is less than 3. Otherwise, captured value equals number of event pulses - 1."]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Prescaler counter value. Write a value to CNT to capture the value of the 16-bit prescaler counter into CNT. Read CNT to get the captured value. The read value gets 1 LSB uncertainty if the event source level rises when you release the reset. The read value gets 1 LSB uncertainty if the event source level rises when you capture the prescaler counter. Please note the following: - The prescaler counter is reset to 2 by PRECTL.RESET_N. - The captured value is 2 when the number of rising edges on prescaler input is less than 3. Otherwise, captured value equals number of event pulses - 1."]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Prescaler counter value. Write a value to CNT to capture the value of the 16-bit prescaler counter into CNT. Read CNT to get the captured value. The read value gets 1 LSB uncertainty if the event source level rises when you release the reset. The read value gets 1 LSB uncertainty if the event source level rises when you capture the prescaler counter. Please note the following: - The prescaler counter is reset to 2 by PRECTL.RESET_N. - The captured value is 2 when the number of rising edges on prescaler input is less than 3. Otherwise, captured value equals number of event pulses - 1."]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<PrecntrSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "Prescaler Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`precntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`precntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrecntrSpec;
impl crate::RegisterSpec for PrecntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`precntr::R`](R) reader structure"]
impl crate::Readable for PrecntrSpec {}
#[doc = "`write(|w| ..)` method takes [`precntr::W`](W) writer structure"]
impl crate::Writable for PrecntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRECNTR to value 0"]
impl crate::Resettable for PrecntrSpec {
    const RESET_VALUE: u32 = 0;
}
