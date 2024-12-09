#[doc = "Register `EVSTAT1H` reader"]
pub type R = crate::R<Evstat1hSpec>;
#[doc = "Register `EVSTAT1H` writer"]
pub type W = crate::W<Evstat1hSpec>;
#[doc = "Field `ALIAS_EV` reader - 7:0\\]
Alias of EVSTAT1 event 15 down to 8."]
pub type AliasEvR = crate::FieldReader;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Alias of EVSTAT1 event 15 down to 8."]
    #[inline(always)]
    pub fn alias_ev(&self) -> AliasEvR {
        AliasEvR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Event Status 1 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat1h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat1h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Evstat1hSpec;
impl crate::RegisterSpec for Evstat1hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evstat1h::R`](R) reader structure"]
impl crate::Readable for Evstat1hSpec {}
#[doc = "`write(|w| ..)` method takes [`evstat1h::W`](W) writer structure"]
impl crate::Writable for Evstat1hSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVSTAT1H to value 0"]
impl crate::Resettable for Evstat1hSpec {
    const RESET_VALUE: u32 = 0;
}
