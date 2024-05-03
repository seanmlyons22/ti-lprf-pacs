#[doc = "Register `MVFR1` reader"]
pub type R = crate::R<Mvfr1Spec>;
#[doc = "Register `MVFR1` writer"]
pub type W = crate::W<Mvfr1Spec>;
#[doc = "Field `FPFtZ` reader - 3:0\\]
Indicates whether subnormals are always flushed-to-zero"]
pub type FpftZR = crate::FieldReader;
#[doc = "Field `FPFtZ` writer - 3:0\\]
Indicates whether subnormals are always flushed-to-zero"]
pub type FpftZW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FPDNaN` reader - 7:4\\]
Indicates whether the FP hardware implementation supports NaN propagation"]
pub type FpdnaNR = crate::FieldReader;
#[doc = "Field `FPDNaN` writer - 7:4\\]
Indicates whether the FP hardware implementation supports NaN propagation"]
pub type FpdnaNW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED8` reader - 23:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED8` writer - 23:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FPHP` reader - 27:24\\]
Indicates whether the FP Extension implements half-precision FP conversion instructions"]
pub type FphpR = crate::FieldReader;
#[doc = "Field `FPHP` writer - 27:24\\]
Indicates whether the FP Extension implements half-precision FP conversion instructions"]
pub type FphpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FMAC` reader - 31:28\\]
Indicates whether the FP Extension implements the fused multiply accumulate instructions"]
pub type FmacR = crate::FieldReader;
#[doc = "Field `FMAC` writer - 31:28\\]
Indicates whether the FP Extension implements the fused multiply accumulate instructions"]
pub type FmacW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates whether subnormals are always flushed-to-zero"]
    #[inline(always)]
    pub fn fpft_z(&self) -> FpftZR {
        FpftZR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates whether the FP hardware implementation supports NaN propagation"]
    #[inline(always)]
    pub fn fpdna_n(&self) -> FpdnaNR {
        FpdnaNR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates whether the FP Extension implements half-precision FP conversion instructions"]
    #[inline(always)]
    pub fn fphp(&self) -> FphpR {
        FphpR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates whether the FP Extension implements the fused multiply accumulate instructions"]
    #[inline(always)]
    pub fn fmac(&self) -> FmacR {
        FmacR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates whether subnormals are always flushed-to-zero"]
    #[inline(always)]
    #[must_use]
    pub fn fpft_z(&mut self) -> FpftZW<Mvfr1Spec> {
        FpftZW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates whether the FP hardware implementation supports NaN propagation"]
    #[inline(always)]
    #[must_use]
    pub fn fpdna_n(&mut self) -> FpdnaNW<Mvfr1Spec> {
        FpdnaNW::new(self, 4)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<Mvfr1Spec> {
        Reserved8W::new(self, 8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Indicates whether the FP Extension implements half-precision FP conversion instructions"]
    #[inline(always)]
    #[must_use]
    pub fn fphp(&mut self) -> FphpW<Mvfr1Spec> {
        FphpW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Indicates whether the FP Extension implements the fused multiply accumulate instructions"]
    #[inline(always)]
    #[must_use]
    pub fn fmac(&mut self) -> FmacW<Mvfr1Spec> {
        FmacW::new(self, 28)
    }
}
#[doc = "Describes the features provided by the Floating-point Extension\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mvfr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mvfr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets MVFR1 to value 0"]
impl crate::Resettable for Mvfr1Spec {
    const RESET_VALUE: u32 = 0;
}
