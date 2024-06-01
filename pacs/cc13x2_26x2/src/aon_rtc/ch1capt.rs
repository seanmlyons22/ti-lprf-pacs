#[doc = "Register `CH1CAPT` reader"]
pub type R = crate::R<Ch1captSpec>;
#[doc = "Register `CH1CAPT` writer"]
pub type W = crate::W<Ch1captSpec>;
#[doc = "Field `SUBSEC` reader - 15:0\\]
Value of SUBSEC.VALUE bits 31:16 at capture time."]
pub type SubsecR = crate::FieldReader<u16>;
#[doc = "Field `SUBSEC` writer - 15:0\\]
Value of SUBSEC.VALUE bits 31:16 at capture time."]
pub type SubsecW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SEC` reader - 31:16\\]
Value of SEC.VALUE bits 15:0 at capture time."]
pub type SecR = crate::FieldReader<u16>;
#[doc = "Field `SEC` writer - 31:16\\]
Value of SEC.VALUE bits 15:0 at capture time."]
pub type SecW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Value of SUBSEC.VALUE bits 31:16 at capture time."]
    #[inline(always)]
    pub fn subsec(&self) -> SubsecR {
        SubsecR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value of SEC.VALUE bits 15:0 at capture time."]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Value of SUBSEC.VALUE bits 31:16 at capture time."]
    #[inline(always)]
    #[must_use]
    pub fn subsec(&mut self) -> SubsecW<Ch1captSpec> {
        SubsecW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Value of SEC.VALUE bits 15:0 at capture time."]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SecW<Ch1captSpec> {
        SecW::new(self, 16)
    }
}
#[doc = "Channel 1 Capture Value If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1capt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1capt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1captSpec;
impl crate::RegisterSpec for Ch1captSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1capt::R`](R) reader structure"]
impl crate::Readable for Ch1captSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1capt::W`](W) writer structure"]
impl crate::Writable for Ch1captSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1CAPT to value 0"]
impl crate::Resettable for Ch1captSpec {
    const RESET_VALUE: u32 = 0;
}
