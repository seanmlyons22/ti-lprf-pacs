#[doc = "Register `TEST5` reader"]
pub type R = crate::R<Test5Spec>;
#[doc = "Register `TEST5` writer"]
pub type W = crate::W<Test5Spec>;
#[doc = "Field `CAL_CAP_CTL` reader - 9:0\\]
This regsiter updated ull_usc_ulpadchp_dft_i\\[26:0\\]
value if Test 5: HW_STEP_SEL_DIS bit enabled ull_usc_ulpadchp_dft_i\\[26:0\\]"]
pub type CalCapCtlR = crate::FieldReader<u16>;
#[doc = "Field `CAL_CAP_CTL` writer - 9:0\\]
This regsiter updated ull_usc_ulpadchp_dft_i\\[26:0\\]
value if Test 5: HW_STEP_SEL_DIS bit enabled ull_usc_ulpadchp_dft_i\\[26:0\\]"]
pub type CalCapCtlW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RESERVED10` reader - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
This regsiter updated ull_usc_ulpadchp_dft_i\\[26:0\\]
value if Test 5: HW_STEP_SEL_DIS bit enabled ull_usc_ulpadchp_dft_i\\[26:0\\]"]
    #[inline(always)]
    pub fn cal_cap_ctl(&self) -> CalCapCtlR {
        CalCapCtlR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
This regsiter updated ull_usc_ulpadchp_dft_i\\[26:0\\]
value if Test 5: HW_STEP_SEL_DIS bit enabled ull_usc_ulpadchp_dft_i\\[26:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn cal_cap_ctl(&mut self) -> CalCapCtlW<Test5Spec> {
        CalCapCtlW::new(self, 0)
    }
}
#[doc = "This regsiter updated ull_usc_ulpadchp_dft_i\\[26:0\\]
value if Test 5: HW_STEP_SEL_DIS bit enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Test5Spec;
impl crate::RegisterSpec for Test5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test5::R`](R) reader structure"]
impl crate::Readable for Test5Spec {}
#[doc = "`write(|w| ..)` method takes [`test5::W`](W) writer structure"]
impl crate::Writable for Test5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST5 to value 0"]
impl crate::Resettable for Test5Spec {
    const RESET_VALUE: u32 = 0;
}
