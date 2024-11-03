#[doc = "Register `TEST3` reader"]
pub type R = crate::R<Test3Spec>;
#[doc = "Register `TEST3` writer"]
pub type W = crate::W<Test3Spec>;
#[doc = "Field `CAL_ACUML` reader - 31:0\\]
Accumulation of # samples during Calibration step"]
pub type CalAcumlR = crate::FieldReader<u32>;
#[doc = "Field `CAL_ACUML` writer - 31:0\\]
Accumulation of # samples during Calibration step"]
pub type CalAcumlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Accumulation of # samples during Calibration step"]
    #[inline(always)]
    pub fn cal_acuml(&self) -> CalAcumlR {
        CalAcumlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Accumulation of # samples during Calibration step"]
    #[inline(always)]
    #[must_use]
    pub fn cal_acuml(&mut self) -> CalAcumlW<Test3Spec> {
        CalAcumlW::new(self, 0)
    }
}
#[doc = "ADC CAL Accumulation Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Test3Spec;
impl crate::RegisterSpec for Test3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test3::R`](R) reader structure"]
impl crate::Readable for Test3Spec {}
#[doc = "`write(|w| ..)` method takes [`test3::W`](W) writer structure"]
impl crate::Writable for Test3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST3 to value 0"]
impl crate::Resettable for Test3Spec {
    const RESET_VALUE: u32 = 0;
}
