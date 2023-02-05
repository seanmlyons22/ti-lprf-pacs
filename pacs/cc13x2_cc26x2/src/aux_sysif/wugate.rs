#[doc = "Register `WUGATE` reader"]
pub struct R(crate::R<WUGATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUGATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUGATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUGATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUGATE` writer"]
pub struct W(crate::W<WUGATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUGATE_SPEC>;
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
impl From<crate::W<WUGATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUGATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - 0:0\\]
Wakeup output enable. 0: Disable AUX wakeup output. 1: Enable AUX wakeup output."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - 0:0\\]
Wakeup output enable. 0: Disable AUX wakeup output. 1: Enable AUX wakeup output."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUGATE_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUGATE_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Wakeup output enable. 0: Disable AUX wakeup output. 1: Enable AUX wakeup output."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
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
Wakeup output enable. 0: Disable AUX wakeup output. 1: Enable AUX wakeup output."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
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
#[doc = "Wakeup Gate You must disable the AUX wakeup output: - Before you clear a programmable wakeup flag. - Before you change the value of \\[PROGWUnCFG.EN\\]
or \\[PROGWUnCFG.WU_SRC\\]. The AUX wakeup output must be re-enabled after clear operation or programmable wakeup configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wugate](index.html) module"]
pub struct WUGATE_SPEC;
impl crate::RegisterSpec for WUGATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wugate::R](R) reader structure"]
impl crate::Readable for WUGATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wugate::W](W) writer structure"]
impl crate::Writable for WUGATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUGATE to value 0"]
impl crate::Resettable for WUGATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
