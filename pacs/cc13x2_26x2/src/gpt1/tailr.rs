#[doc = "Register `TAILR` reader"]
pub type R = crate::R<TailrSpec>;
#[doc = "Register `TAILR` writer"]
pub type W = crate::W<TailrSpec>;
#[doc = "Field `TAILR` reader - 31:0\\]
GPT Timer A Interval Load Register Writing this field loads the counter for Timer A. A read returns the current value of TAILR."]
pub type TailrR = crate::FieldReader<u32>;
#[doc = "Field `TAILR` writer - 31:0\\]
GPT Timer A Interval Load Register Writing this field loads the counter for Timer A. A read returns the current value of TAILR."]
pub type TailrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Interval Load Register Writing this field loads the counter for Timer A. A read returns the current value of TAILR."]
    #[inline(always)]
    pub fn tailr(&self) -> TailrR {
        TailrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer A Interval Load Register Writing this field loads the counter for Timer A. A read returns the current value of TAILR."]
    #[inline(always)]
    #[must_use]
    pub fn tailr(&mut self) -> TailrW<TailrSpec> {
        TailrW::new(self, 0)
    }
}
#[doc = "Timer A Interval Load Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tailr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tailr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TailrSpec;
impl crate::RegisterSpec for TailrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tailr::R`](R) reader structure"]
impl crate::Readable for TailrSpec {}
#[doc = "`write(|w| ..)` method takes [`tailr::W`](W) writer structure"]
impl crate::Writable for TailrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAILR to value 0xffff_ffff"]
impl crate::Resettable for TailrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
