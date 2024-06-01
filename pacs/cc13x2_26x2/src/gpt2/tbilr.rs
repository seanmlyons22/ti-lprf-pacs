#[doc = "Register `TBILR` reader"]
pub type R = crate::R<TbilrSpec>;
#[doc = "Register `TBILR` writer"]
pub type W = crate::W<TbilrSpec>;
#[doc = "Field `TBILR` reader - 31:0\\]
GPT Timer B Interval Load Register Writing this field loads the counter for Timer B. A read returns the current value of TBILR."]
pub type TbilrR = crate::FieldReader<u32>;
#[doc = "Field `TBILR` writer - 31:0\\]
GPT Timer B Interval Load Register Writing this field loads the counter for Timer B. A read returns the current value of TBILR."]
pub type TbilrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Interval Load Register Writing this field loads the counter for Timer B. A read returns the current value of TBILR."]
    #[inline(always)]
    pub fn tbilr(&self) -> TbilrR {
        TbilrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
GPT Timer B Interval Load Register Writing this field loads the counter for Timer B. A read returns the current value of TBILR."]
    #[inline(always)]
    #[must_use]
    pub fn tbilr(&mut self) -> TbilrW<TbilrSpec> {
        TbilrW::new(self, 0)
    }
}
#[doc = "Timer B Interval Load Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbilr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbilr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbilrSpec;
impl crate::RegisterSpec for TbilrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbilr::R`](R) reader structure"]
impl crate::Readable for TbilrSpec {}
#[doc = "`write(|w| ..)` method takes [`tbilr::W`](W) writer structure"]
impl crate::Writable for TbilrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBILR to value 0xffff"]
impl crate::Resettable for TbilrSpec {
    const RESET_VALUE: u32 = 0xffff;
}
