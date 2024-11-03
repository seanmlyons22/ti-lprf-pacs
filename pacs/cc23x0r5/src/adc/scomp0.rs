#[doc = "Register `SCOMP0` reader"]
pub type R = crate::R<Scomp0Spec>;
#[doc = "Register `SCOMP0` writer"]
pub type W = crate::W<Scomp0Spec>;
#[doc = "Field `VAL` reader - 9:0\\]
Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL $gt; 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - 9:0\\]
Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL $gt; 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RESERVED10` reader - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED10` writer - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL $gt; 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x03ff) as u16)
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
Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL $gt; 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Scomp0Spec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<Scomp0Spec> {
        Reserved10W::new(self, 10)
    }
}
#[doc = "Sample time compare 0 register. Specifies the sample time, in number of ADC sample clock cycles. CTL0.ENC must be set to 0 to write to this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scomp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scomp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scomp0Spec;
impl crate::RegisterSpec for Scomp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scomp0::R`](R) reader structure"]
impl crate::Readable for Scomp0Spec {}
#[doc = "`write(|w| ..)` method takes [`scomp0::W`](W) writer structure"]
impl crate::Writable for Scomp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCOMP0 to value 0"]
impl crate::Resettable for Scomp0Spec {
    const RESET_VALUE: u32 = 0;
}
