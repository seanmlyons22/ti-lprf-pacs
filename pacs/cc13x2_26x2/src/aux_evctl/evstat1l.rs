#[doc = "Register `EVSTAT1L` reader"]
pub type R = crate::R<Evstat1lSpec>;
#[doc = "Register `EVSTAT1L` writer"]
pub type W = crate::W<Evstat1lSpec>;
#[doc = "Field `ALIAS_EV` reader - 7:0\\]
Alias of EVSTAT1 event 7 down to 0."]
pub type AliasEvR = crate::FieldReader;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Alias of EVSTAT1 event 7 down to 0."]
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
#[doc = "Event Status 1 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat1l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat1l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Evstat1lSpec;
impl crate::RegisterSpec for Evstat1lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evstat1l::R`](R) reader structure"]
impl crate::Readable for Evstat1lSpec {}
#[doc = "`write(|w| ..)` method takes [`evstat1l::W`](W) writer structure"]
impl crate::Writable for Evstat1lSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVSTAT1L to value 0"]
impl crate::Resettable for Evstat1lSpec {
    const RESET_VALUE: u32 = 0;
}
