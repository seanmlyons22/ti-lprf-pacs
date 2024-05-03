#[doc = "Register `MVFR1` reader"]
pub type R = crate::R<Mvfr1Spec>;
#[doc = "Register `MVFR1` writer"]
pub type W = crate::W<Mvfr1Spec>;
#[doc = "Field `FTZ_MODE` reader - 3:0\\]
Indicates whether the FP hardware implementation supports only the Flush-to-Zero mode of operation. The value of this field is: 0b0001 - hardware supports full denormalized number arithmetic."]
pub type FtzModeR = crate::FieldReader;
#[doc = "Field `FTZ_MODE` writer - 3:0\\]
Indicates whether the FP hardware implementation supports only the Flush-to-Zero mode of operation. The value of this field is: 0b0001 - hardware supports full denormalized number arithmetic."]
pub type FtzModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `D_NAN_MODE` reader - 7:4\\]
Indicates whether the FP hardware implementation supports only the Default NaN mode. The value of this field is: 0b0001 - hardware supports propagation of NaN values."]
pub type DNanModeR = crate::FieldReader;
#[doc = "Field `D_NAN_MODE` writer - 7:4\\]
Indicates whether the FP hardware implementation supports only the Default NaN mode. The value of this field is: 0b0001 - hardware supports propagation of NaN values."]
pub type DNanModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED8` reader - 23:8\\]
Software should not rely on the value of a reserved."]
pub type Reserved8R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED8` writer - 23:8\\]
Software should not rely on the value of a reserved."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FP_HPFP` reader - 27:24\\]
Indicates whether the FP supports half-precision floating-point conversion operations. The value of this field is: 0b0001 - supported."]
pub type FpHpfpR = crate::FieldReader;
#[doc = "Field `FP_HPFP` writer - 27:24\\]
Indicates whether the FP supports half-precision floating-point conversion operations. The value of this field is: 0b0001 - supported."]
pub type FpHpfpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FP_FUSED_MAC` reader - 31:28\\]
Indicates whether the FP supports fused multiply accumulate operations. The value of this field is: 0b0001 - supported."]
pub type FpFusedMacR = crate::FieldReader;
#[doc = "Field `FP_FUSED_MAC` writer - 31:28\\]
Indicates whether the FP supports fused multiply accumulate operations. The value of this field is: 0b0001 - supported."]
pub type FpFusedMacW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates whether the FP hardware implementation supports only the Flush-to-Zero mode of operation. The value of this field is: 0b0001 - hardware supports full denormalized number arithmetic."]
    #[inline(always)]
    pub fn ftz_mode(&self) -> FtzModeR {
        FtzModeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates whether the FP hardware implementation supports only the Default NaN mode. The value of this field is: 0b0001 - hardware supports propagation of NaN values."]
    #[inline(always)]
    pub fn d_nan_mode(&self) -> DNanModeR {
        DNanModeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Software should not rely on the value of a reserved."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates whether the FP supports half-precision floating-point conversion operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn fp_hpfp(&self) -> FpHpfpR {
        FpHpfpR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates whether the FP supports fused multiply accumulate operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    pub fn fp_fused_mac(&self) -> FpFusedMacR {
        FpFusedMacR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates whether the FP hardware implementation supports only the Flush-to-Zero mode of operation. The value of this field is: 0b0001 - hardware supports full denormalized number arithmetic."]
    #[inline(always)]
    #[must_use]
    pub fn ftz_mode(&mut self) -> FtzModeW<Mvfr1Spec> {
        FtzModeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates whether the FP hardware implementation supports only the Default NaN mode. The value of this field is: 0b0001 - hardware supports propagation of NaN values."]
    #[inline(always)]
    #[must_use]
    pub fn d_nan_mode(&mut self) -> DNanModeW<Mvfr1Spec> {
        DNanModeW::new(self, 4)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Software should not rely on the value of a reserved."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Mvfr1Spec> {
        Reserved8W::new(self, 8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates whether the FP supports half-precision floating-point conversion operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    #[must_use]
    pub fn fp_hpfp(&mut self) -> FpHpfpW<Mvfr1Spec> {
        FpHpfpW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates whether the FP supports fused multiply accumulate operations. The value of this field is: 0b0001 - supported."]
    #[inline(always)]
    #[must_use]
    pub fn fp_fused_mac(&mut self) -> FpFusedMacW<Mvfr1Spec> {
        FpFusedMacW::new(self, 28)
    }
}
#[doc = "Media and FP Feature 1 Describes the features provided by the Floating-point extension.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mvfr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mvfr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mvfr1Spec;
impl crate::RegisterSpec for Mvfr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mvfr1::R`](R) reader structure"]
impl crate::Readable for Mvfr1Spec {}
#[doc = "`write(|w| ..)` method takes [`mvfr1::W`](W) writer structure"]
impl crate::Writable for Mvfr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MVFR1 to value 0x1100_0011"]
impl crate::Resettable for Mvfr1Spec {
    const RESET_VALUE: u32 = 0x1100_0011;
}
