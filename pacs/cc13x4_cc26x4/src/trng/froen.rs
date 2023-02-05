#[doc = "Register `FROEN` reader"]
pub struct R(crate::R<FROEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FROEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FROEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FROEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FROEN` writer"]
pub struct W(crate::W<FROEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FROEN_SPEC>;
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
impl From<crate::W<FROEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FROEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRO_MASK` reader - 23:0\\]
Enable bits for the individual FROs. A '1' in bit \\[n\\]
enables FRO 'n'. Default state is all '1's to enable all FROs after power-up. Note that they are not actually started up before the CTL.TRNG_EN bit is set to '1'. Bits are automatically forced to '0' here (and cannot be written to '1') while the corresponding bit in ALARMSTOP.FRO_FLAGS has value '1'."]
pub type FRO_MASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FRO_MASK` writer - 23:0\\]
Enable bits for the individual FROs. A '1' in bit \\[n\\]
enables FRO 'n'. Default state is all '1's to enable all FROs after power-up. Note that they are not actually started up before the CTL.TRNG_EN bit is set to '1'. Bits are automatically forced to '0' here (and cannot be written to '1') while the corresponding bit in ALARMSTOP.FRO_FLAGS has value '1'."]
pub type FRO_MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FROEN_SPEC, u32, u32, 24, O>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FROEN_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Enable bits for the individual FROs. A '1' in bit \\[n\\]
enables FRO 'n'. Default state is all '1's to enable all FROs after power-up. Note that they are not actually started up before the CTL.TRNG_EN bit is set to '1'. Bits are automatically forced to '0' here (and cannot be written to '1') while the corresponding bit in ALARMSTOP.FRO_FLAGS has value '1'."]
    #[inline(always)]
    pub fn fro_mask(&self) -> FRO_MASK_R {
        FRO_MASK_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Enable bits for the individual FROs. A '1' in bit \\[n\\]
enables FRO 'n'. Default state is all '1's to enable all FROs after power-up. Note that they are not actually started up before the CTL.TRNG_EN bit is set to '1'. Bits are automatically forced to '0' here (and cannot be written to '1') while the corresponding bit in ALARMSTOP.FRO_FLAGS has value '1'."]
    #[inline(always)]
    #[must_use]
    pub fn fro_mask(&mut self) -> FRO_MASK_W<0> {
        FRO_MASK_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FRO Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [froen](index.html) module"]
pub struct FROEN_SPEC;
impl crate::RegisterSpec for FROEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [froen::R](R) reader structure"]
impl crate::Readable for FROEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [froen::W](W) writer structure"]
impl crate::Writable for FROEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FROEN to value 0x00ff_ffff"]
impl crate::Resettable for FROEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_ffff;
}
