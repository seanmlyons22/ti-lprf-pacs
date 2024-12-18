#[doc = "Register `STMPXCNT` reader"]
pub type R = crate::R<StmpxcntSpec>;
#[doc = "Register `STMPXCNT` writer"]
pub type W = crate::W<StmpxcntSpec>;
#[doc = "Field `CURR_VALUE` reader - 15:0\\]
Current value of the XOSC counter, latched when reading STMPWCNT."]
pub type CurrValueR = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Current value of the XOSC counter, latched when reading STMPWCNT."]
    #[inline(always)]
    pub fn curr_value(&self) -> CurrValueR {
        CurrValueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Current Value of XCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpxcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpxcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StmpxcntSpec;
impl crate::RegisterSpec for StmpxcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmpxcnt::R`](R) reader structure"]
impl crate::Readable for StmpxcntSpec {}
#[doc = "`write(|w| ..)` method takes [`stmpxcnt::W`](W) writer structure"]
impl crate::Writable for StmpxcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STMPXCNT to value 0"]
impl crate::Resettable for StmpxcntSpec {
    const RESET_VALUE: u32 = 0;
}
