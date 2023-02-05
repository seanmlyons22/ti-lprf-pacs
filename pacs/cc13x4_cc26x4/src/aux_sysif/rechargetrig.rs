#[doc = "Register `RECHARGETRIG` reader"]
pub struct R(crate::R<RECHARGETRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECHARGETRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECHARGETRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECHARGETRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECHARGETRIG` writer"]
pub struct W(crate::W<RECHARGETRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECHARGETRIG_SPEC>;
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
impl From<crate::W<RECHARGETRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECHARGETRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIG` reader - 0:0\\]
Recharge trigger. 0: No effect. 1: Request VDDR recharge. Request VDDR recharge only when AUX_EVCTL:EVSTAT2.PWR_DWN is 1. Follow this sequence when OPMODEREQ.REQ is LP: - Set TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 1. - Clear TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 0. Follow this sequence when OPMODEREQ.REQ is PDA or PDLP: - Set TRIG. - Clear TRIG."]
pub type TRIG_R = crate::BitReader<bool>;
#[doc = "Field `TRIG` writer - 0:0\\]
Recharge trigger. 0: No effect. 1: Request VDDR recharge. Request VDDR recharge only when AUX_EVCTL:EVSTAT2.PWR_DWN is 1. Follow this sequence when OPMODEREQ.REQ is LP: - Set TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 1. - Clear TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 0. Follow this sequence when OPMODEREQ.REQ is PDA or PDLP: - Set TRIG. - Clear TRIG."]
pub type TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, RECHARGETRIG_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RECHARGETRIG_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Recharge trigger. 0: No effect. 1: Request VDDR recharge. Request VDDR recharge only when AUX_EVCTL:EVSTAT2.PWR_DWN is 1. Follow this sequence when OPMODEREQ.REQ is LP: - Set TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 1. - Clear TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 0. Follow this sequence when OPMODEREQ.REQ is PDA or PDLP: - Set TRIG. - Clear TRIG."]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Recharge trigger. 0: No effect. 1: Request VDDR recharge. Request VDDR recharge only when AUX_EVCTL:EVSTAT2.PWR_DWN is 1. Follow this sequence when OPMODEREQ.REQ is LP: - Set TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 1. - Clear TRIG. - Wait until AUX_EVCTL:EVSTAT2.VDDR_RECHARGE is 0. Follow this sequence when OPMODEREQ.REQ is PDA or PDLP: - Set TRIG. - Clear TRIG."]
    #[inline(always)]
    #[must_use]
    pub fn trig(&mut self) -> TRIG_W<0> {
        TRIG_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VDDR Recharge Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rechargetrig](index.html) module"]
pub struct RECHARGETRIG_SPEC;
impl crate::RegisterSpec for RECHARGETRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rechargetrig::R](R) reader structure"]
impl crate::Readable for RECHARGETRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rechargetrig::W](W) writer structure"]
impl crate::Writable for RECHARGETRIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RECHARGETRIG to value 0"]
impl crate::Resettable for RECHARGETRIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
