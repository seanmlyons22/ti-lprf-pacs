#[doc = "Register `RTCSUBSEC` reader"]
pub type R = crate::R<RtcsubsecSpec>;
#[doc = "Register `RTCSUBSEC` writer"]
pub type W = crate::W<RtcsubsecSpec>;
#[doc = "Field `SUBSEC` reader - 15:0\\]
Bits 31:16 in AON_RTC:SUBSEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads SUBSEC. - Then read SUBSEC until two consecutive reads are equal."]
pub type SubsecR = crate::FieldReader<u16>;
#[doc = "Field `SUBSEC` writer - 15:0\\]
Bits 31:16 in AON_RTC:SUBSEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads SUBSEC. - Then read SUBSEC until two consecutive reads are equal."]
pub type SubsecW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Bits 31:16 in AON_RTC:SUBSEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads SUBSEC. - Then read SUBSEC until two consecutive reads are equal."]
    #[inline(always)]
    pub fn subsec(&self) -> SubsecR {
        SubsecR::new((self.bits & 0xffff) as u16)
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
Bits 31:16 in AON_RTC:SUBSEC.VALUE. Follow this procedure to get the correct value: - Do two dummy reads SUBSEC. - Then read SUBSEC until two consecutive reads are equal."]
    #[inline(always)]
    #[must_use]
    pub fn subsec(&mut self) -> SubsecW<RtcsubsecSpec> {
        SubsecW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<RtcsubsecSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Real Time Counter Sub-Second System CPU must not access this register. Instead, system CPU must access AON_RTC:SUBSEC.VALUE directly.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsec::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsec::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcsubsecSpec;
impl crate::RegisterSpec for RtcsubsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcsubsec::R`](R) reader structure"]
impl crate::Readable for RtcsubsecSpec {}
#[doc = "`write(|w| ..)` method takes [`rtcsubsec::W`](W) writer structure"]
impl crate::Writable for RtcsubsecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTCSUBSEC to value 0"]
impl crate::Resettable for RtcsubsecSpec {
    const RESET_VALUE: u32 = 0;
}
