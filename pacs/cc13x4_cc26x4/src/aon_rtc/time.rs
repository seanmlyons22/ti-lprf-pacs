#[doc = "Register `TIME` reader"]
pub type R = crate::R<TimeSpec>;
#[doc = "Register `TIME` writer"]
pub type W = crate::W<TimeSpec>;
#[doc = "Field `SUBSEC_H` reader - 15:0\\]
Returns the upper halfword of SUBSEC register."]
pub type SubsecHR = crate::FieldReader<u16>;
#[doc = "Field `SEC_L` reader - 31:16\\]
Returns the lower halfword of SEC register."]
pub type SecLR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Returns the upper halfword of SUBSEC register."]
    #[inline(always)]
    pub fn subsec_h(&self) -> SubsecHR {
        SubsecHR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Returns the lower halfword of SEC register."]
    #[inline(always)]
    pub fn sec_l(&self) -> SecLR {
        SecLR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Current Counter Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeSpec;
impl crate::RegisterSpec for TimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time::R`](R) reader structure"]
impl crate::Readable for TimeSpec {}
#[doc = "`write(|w| ..)` method takes [`time::W`](W) writer structure"]
impl crate::Writable for TimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIME to value 0"]
impl crate::Resettable for TimeSpec {
    const RESET_VALUE: u32 = 0;
}
