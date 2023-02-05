#[doc = "Register `RECHARGEDET` reader"]
pub struct R(crate::R<RECHARGEDET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECHARGEDET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECHARGEDET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECHARGEDET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECHARGEDET` writer"]
pub struct W(crate::W<RECHARGEDET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECHARGEDET_SPEC>;
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
impl From<crate::W<RECHARGEDET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECHARGEDET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - 0:0\\]
VDDR recharge detector enable. 0: Disable recharge detection. STAT becomes zero. 1: Enable recharge detection."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - 0:0\\]
VDDR recharge detector enable. 0: Disable recharge detection. STAT becomes zero. 1: Enable recharge detection."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RECHARGEDET_SPEC, bool, O>;
#[doc = "Field `STAT` reader - 1:1\\]
VDDR recharge detector status. 0: No recharge of VDDR has occurred since EN was set. 1: Recharge of VDDR has occurred since EN was set."]
pub type STAT_R = crate::BitReader<bool>;
#[doc = "Field `STAT` writer - 1:1\\]
VDDR recharge detector status. 0: No recharge of VDDR has occurred since EN was set. 1: Recharge of VDDR has occurred since EN was set."]
pub type STAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RECHARGEDET_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RECHARGEDET_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
VDDR recharge detector enable. 0: Disable recharge detection. STAT becomes zero. 1: Enable recharge detection."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
VDDR recharge detector status. 0: No recharge of VDDR has occurred since EN was set. 1: Recharge of VDDR has occurred since EN was set."]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
VDDR recharge detector enable. 0: Disable recharge detection. STAT becomes zero. 1: Enable recharge detection."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
VDDR recharge detector status. 0: No recharge of VDDR has occurred since EN was set. 1: Recharge of VDDR has occurred since EN was set."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> STAT_W<1> {
        STAT_W::new(self)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VDDR Recharge Detection Some applications can be sensitive to power noise caused by recharge of VDDR. You can detect if VDDR recharge occurs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rechargedet](index.html) module"]
pub struct RECHARGEDET_SPEC;
impl crate::RegisterSpec for RECHARGEDET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rechargedet::R](R) reader structure"]
impl crate::Readable for RECHARGEDET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rechargedet::W](W) writer structure"]
impl crate::Writable for RECHARGEDET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RECHARGEDET to value 0"]
impl crate::Resettable for RECHARGEDET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
