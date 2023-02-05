#[doc = "Register `CFG0` reader"]
pub struct R(crate::R<CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG0` writer"]
pub struct W(crate::W<CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIN_REFILL_CYCLES` reader - 7:0\\]
This field determines the minimum number of samples (between 2^6 and 2^14) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the value of this field is zero, the number of samples is fixed to the value determined by the MAX_REFILL_CYCLES field, otherwise the minimum number of samples equals the written value times 64 (which can be up to 2^14). To ensure same entropy in all generated random numbers the value 0 should be used. Then MAX_REFILL_CYCLES controls the minimum refill interval. The number of samples defined here cannot be higher than the number defined by the 'max_refill_cycles' field (i.e. that field takes precedence). No random value will be created if min refill > max refill. This field can only be modified while CTL.TRNG_EN = 0. 0x00: Minimum samples = MAX_REFILL_CYCLES (all numbers have same entropy) 0x01: 1*2^6 samples 0x02: 2*2^6 samples ... 0xFF: 255*2^6 samples"]
pub type MIN_REFILL_CYCLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN_REFILL_CYCLES` writer - 7:0\\]
This field determines the minimum number of samples (between 2^6 and 2^14) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the value of this field is zero, the number of samples is fixed to the value determined by the MAX_REFILL_CYCLES field, otherwise the minimum number of samples equals the written value times 64 (which can be up to 2^14). To ensure same entropy in all generated random numbers the value 0 should be used. Then MAX_REFILL_CYCLES controls the minimum refill interval. The number of samples defined here cannot be higher than the number defined by the 'max_refill_cycles' field (i.e. that field takes precedence). No random value will be created if min refill > max refill. This field can only be modified while CTL.TRNG_EN = 0. 0x00: Minimum samples = MAX_REFILL_CYCLES (all numbers have same entropy) 0x01: 1*2^6 samples 0x02: 2*2^6 samples ... 0xFF: 255*2^6 samples"]
pub type MIN_REFILL_CYCLES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 8, O>;
#[doc = "Field `SMPL_DIV` reader - 11:8\\]
This field directly controls the number of clock cycles between samples taken from the FROs. Default value 0 indicates that samples are taken every clock cycle, maximum value 0xF takes one sample every 16 clock cycles. This field must be set to a value such that the slowest FRO (even under worst-case conditions) has a cycle time less than twice the sample period. This field can only be modified while CTL.TRNG_EN is '0'."]
pub type SMPL_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMPL_DIV` writer - 11:8\\]
This field directly controls the number of clock cycles between samples taken from the FROs. Default value 0 indicates that samples are taken every clock cycle, maximum value 0xF takes one sample every 16 clock cycles. This field must be set to a value such that the slowest FRO (even under worst-case conditions) has a cycle time less than twice the sample period. This field can only be modified while CTL.TRNG_EN is '0'."]
pub type SMPL_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED12` reader - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED12` writer - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG0_SPEC, u8, u8, 4, O>;
#[doc = "Field `MAX_REFILL_CYCLES` reader - 31:16\\]
This field determines the maximum number of samples (between 2^8 and 2^24) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while CTL.TRNG_EN is 0."]
pub type MAX_REFILL_CYCLES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MAX_REFILL_CYCLES` writer - 31:16\\]
This field determines the maximum number of samples (between 2^8 and 2^24) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while CTL.TRNG_EN is 0."]
pub type MAX_REFILL_CYCLES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
This field determines the minimum number of samples (between 2^6 and 2^14) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the value of this field is zero, the number of samples is fixed to the value determined by the MAX_REFILL_CYCLES field, otherwise the minimum number of samples equals the written value times 64 (which can be up to 2^14). To ensure same entropy in all generated random numbers the value 0 should be used. Then MAX_REFILL_CYCLES controls the minimum refill interval. The number of samples defined here cannot be higher than the number defined by the 'max_refill_cycles' field (i.e. that field takes precedence). No random value will be created if min refill > max refill. This field can only be modified while CTL.TRNG_EN = 0. 0x00: Minimum samples = MAX_REFILL_CYCLES (all numbers have same entropy) 0x01: 1*2^6 samples 0x02: 2*2^6 samples ... 0xFF: 255*2^6 samples"]
    #[inline(always)]
    pub fn min_refill_cycles(&self) -> MIN_REFILL_CYCLES_R {
        MIN_REFILL_CYCLES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
This field directly controls the number of clock cycles between samples taken from the FROs. Default value 0 indicates that samples are taken every clock cycle, maximum value 0xF takes one sample every 16 clock cycles. This field must be set to a value such that the slowest FRO (even under worst-case conditions) has a cycle time less than twice the sample period. This field can only be modified while CTL.TRNG_EN is '0'."]
    #[inline(always)]
    pub fn smpl_div(&self) -> SMPL_DIV_R {
        SMPL_DIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
This field determines the maximum number of samples (between 2^8 and 2^24) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while CTL.TRNG_EN is 0."]
    #[inline(always)]
    pub fn max_refill_cycles(&self) -> MAX_REFILL_CYCLES_R {
        MAX_REFILL_CYCLES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
This field determines the minimum number of samples (between 2^6 and 2^14) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the value of this field is zero, the number of samples is fixed to the value determined by the MAX_REFILL_CYCLES field, otherwise the minimum number of samples equals the written value times 64 (which can be up to 2^14). To ensure same entropy in all generated random numbers the value 0 should be used. Then MAX_REFILL_CYCLES controls the minimum refill interval. The number of samples defined here cannot be higher than the number defined by the 'max_refill_cycles' field (i.e. that field takes precedence). No random value will be created if min refill > max refill. This field can only be modified while CTL.TRNG_EN = 0. 0x00: Minimum samples = MAX_REFILL_CYCLES (all numbers have same entropy) 0x01: 1*2^6 samples 0x02: 2*2^6 samples ... 0xFF: 255*2^6 samples"]
    #[inline(always)]
    #[must_use]
    pub fn min_refill_cycles(&mut self) -> MIN_REFILL_CYCLES_W<0> {
        MIN_REFILL_CYCLES_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
This field directly controls the number of clock cycles between samples taken from the FROs. Default value 0 indicates that samples are taken every clock cycle, maximum value 0xF takes one sample every 16 clock cycles. This field must be set to a value such that the slowest FRO (even under worst-case conditions) has a cycle time less than twice the sample period. This field can only be modified while CTL.TRNG_EN is '0'."]
    #[inline(always)]
    #[must_use]
    pub fn smpl_div(&mut self) -> SMPL_DIV_W<8> {
        SMPL_DIV_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
This field determines the maximum number of samples (between 2^8 and 2^24) taken to re-generate entropy from the FROs after reading out a 64 bits random number. If the written value of this field is zero, the number of samples is 2^24, otherwise the number of samples equals the written value times 2^8. 0x0000: 2^24 samples 0x0001: 1*2^8 samples 0x0002: 2*2^8 samples 0x0003: 3*2^8 samples ... 0x8000: 32768*2^8 samples 0xC000: 49152*2^8 samples ... 0xFFFF: 65535*2^8 samples This field can only be modified while CTL.TRNG_EN is 0."]
    #[inline(always)]
    #[must_use]
    pub fn max_refill_cycles(&mut self) -> MAX_REFILL_CYCLES_W<16> {
        MAX_REFILL_CYCLES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg0](index.html) module"]
pub struct CFG0_SPEC;
impl crate::RegisterSpec for CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg0::R](R) reader structure"]
impl crate::Readable for CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg0::W](W) writer structure"]
impl crate::Writable for CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG0 to value 0"]
impl crate::Resettable for CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
