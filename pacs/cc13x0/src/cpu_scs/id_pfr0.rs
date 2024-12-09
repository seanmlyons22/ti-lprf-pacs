#[doc = "Register `ID_PFR0` reader"]
pub type R = crate::R<IdPfr0Spec>;
#[doc = "Register `ID_PFR0` writer"]
pub type W = crate::W<IdPfr0Spec>;
#[doc = "Field `STATE0` reader - 3:0\\]
State0 (T-bit == 0) 0x0: No ARM encoding 0x1: N/A"]
pub type State0R = crate::FieldReader;
#[doc = "Field `STATE1` reader - 7:4\\]
State1 (T-bit == 1) 0x0: N/A 0x1: N/A 0x2: Thumb-2 encoding with the 16-bit basic instructions plus 32-bit Buncond/BL but no other 32-bit basic instructions (Note non-basic 32-bit instructions can be added using the appropriate instruction attribute, but other 32-bit basic instructions cannot.) 0x3: Thumb-2 encoding with all Thumb-2 basic instructions"]
pub type State1R = crate::FieldReader;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
State0 (T-bit == 0) 0x0: No ARM encoding 0x1: N/A"]
    #[inline(always)]
    pub fn state0(&self) -> State0R {
        State0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
State1 (T-bit == 1) 0x0: N/A 0x1: N/A 0x2: Thumb-2 encoding with the 16-bit basic instructions plus 32-bit Buncond/BL but no other 32-bit basic instructions (Note non-basic 32-bit instructions can be added using the appropriate instruction attribute, but other 32-bit basic instructions cannot.) 0x3: Thumb-2 encoding with all Thumb-2 basic instructions"]
    #[inline(always)]
    pub fn state1(&self) -> State1R {
        State1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Processor Feature 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id_pfr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id_pfr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdPfr0Spec;
impl crate::RegisterSpec for IdPfr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`id_pfr0::R`](R) reader structure"]
impl crate::Readable for IdPfr0Spec {}
#[doc = "`write(|w| ..)` method takes [`id_pfr0::W`](W) writer structure"]
impl crate::Writable for IdPfr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ID_PFR0 to value 0x30"]
impl crate::Resettable for IdPfr0Spec {
    const RESET_VALUE: u32 = 0x30;
}
