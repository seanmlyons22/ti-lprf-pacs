#[doc = "Register `RECHARGESTAT` reader"]
pub struct R(crate::R<RECHARGESTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECHARGESTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECHARGESTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECHARGESTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECHARGESTAT` writer"]
pub struct W(crate::W<RECHARGESTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECHARGESTAT_SPEC>;
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
impl From<crate::W<RECHARGESTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECHARGESTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAX_USED_PER` reader - 15:0\\]
Shows the maximum number of 32kHz periods that have separated two recharge cycles and VDDR still was above VDDR_OK threshold when the latter recharge started. This register can be used as an indication of the leakage current during standby. This bitfield is cleared to 0 when writing this register."]
pub type MAX_USED_PER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MAX_USED_PER` writer - 15:0\\]
Shows the maximum number of 32kHz periods that have separated two recharge cycles and VDDR still was above VDDR_OK threshold when the latter recharge started. This register can be used as an indication of the leakage current during standby. This bitfield is cleared to 0 when writing this register."]
pub type MAX_USED_PER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RECHARGESTAT_SPEC, u16, u16, 16, O>;
#[doc = "Field `VDDR_SMPLS` reader - 19:16\\]
The last 4 VDDR samples. For each bit: 0: VDDR was below VDDR_OK threshold when recharge started 1: VDDR was above VDDR_OK threshold when recharge started The register is updated prior to every recharge period with a shift left, and bit 0 is updated with the last VDDR sample."]
pub type VDDR_SMPLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDDR_SMPLS` writer - 19:16\\]
The last 4 VDDR samples. For each bit: 0: VDDR was below VDDR_OK threshold when recharge started 1: VDDR was above VDDR_OK threshold when recharge started The register is updated prior to every recharge period with a shift left, and bit 0 is updated with the last VDDR sample."]
pub type VDDR_SMPLS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RECHARGESTAT_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED20_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED20` writer - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED20_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RECHARGESTAT_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Shows the maximum number of 32kHz periods that have separated two recharge cycles and VDDR still was above VDDR_OK threshold when the latter recharge started. This register can be used as an indication of the leakage current during standby. This bitfield is cleared to 0 when writing this register."]
    #[inline(always)]
    pub fn max_used_per(&self) -> MAX_USED_PER_R {
        MAX_USED_PER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
The last 4 VDDR samples. For each bit: 0: VDDR was below VDDR_OK threshold when recharge started 1: VDDR was above VDDR_OK threshold when recharge started The register is updated prior to every recharge period with a shift left, and bit 0 is updated with the last VDDR sample."]
    #[inline(always)]
    pub fn vddr_smpls(&self) -> VDDR_SMPLS_R {
        VDDR_SMPLS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Shows the maximum number of 32kHz periods that have separated two recharge cycles and VDDR still was above VDDR_OK threshold when the latter recharge started. This register can be used as an indication of the leakage current during standby. This bitfield is cleared to 0 when writing this register."]
    #[inline(always)]
    #[must_use]
    pub fn max_used_per(&mut self) -> MAX_USED_PER_W<0> {
        MAX_USED_PER_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
The last 4 VDDR samples. For each bit: 0: VDDR was below VDDR_OK threshold when recharge started 1: VDDR was above VDDR_OK threshold when recharge started The register is updated prior to every recharge period with a shift left, and bit 0 is updated with the last VDDR sample."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_smpls(&mut self) -> VDDR_SMPLS_W<16> {
        VDDR_SMPLS_W::new(self)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> RESERVED20_W<20> {
        RESERVED20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rechargestat](index.html) module"]
pub struct RECHARGESTAT_SPEC;
impl crate::RegisterSpec for RECHARGESTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rechargestat::R](R) reader structure"]
impl crate::Readable for RECHARGESTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rechargestat::W](W) writer structure"]
impl crate::Writable for RECHARGESTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RECHARGESTAT to value 0"]
impl crate::Resettable for RECHARGESTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
