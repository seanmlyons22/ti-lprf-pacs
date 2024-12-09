#[doc = "Register `RTCSEC` reader"]
pub type R = crate::R<RtcsecSpec>;
#[doc = "Register `RTCSEC` writer"]
pub type W = crate::W<RtcsecSpec>;
#[doc = "Field `SEC` reader - 15:0\\]
Bits 15:0 in AON_RTC:SEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads of SEC. - Then read SEC until two consecutive reads are equal."]
pub type SecR = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Bits 15:0 in AON_RTC:SEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads of SEC. - Then read SEC until two consecutive reads are equal."]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Real Time Counter Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SEC.VALUE directly.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcsecSpec;
impl crate::RegisterSpec for RtcsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcsec::R`](R) reader structure"]
impl crate::Readable for RtcsecSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcsec::W`](W) writer structure"]
impl crate::Writable for RtcsecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTCSEC to value 0"]
impl crate::Resettable for RtcsecSpec {
    const RESET_VALUE: u32 = 0;
}
