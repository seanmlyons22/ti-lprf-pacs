#[doc = "Register `ANDCCP` reader"]
pub struct R(crate::R<ANDCCP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANDCCP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANDCCP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANDCCP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANDCCP` writer"]
pub struct W(crate::W<ANDCCP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANDCCP_SPEC>;
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
impl From<crate::W<ANDCCP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANDCCP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCP_AND_EN` reader - 0:0\\]
Enables AND operation of the CCP outputs for timers A and B. 0 : PWM outputs of Timer A and Timer B are the internal generated PWM signals of the respective timers. 1 : PWM output of Timer A is ANDed version of Timer A and Timer B PWM signals and Timer B PWM ouput is Timer B PWM signal only."]
pub type CCP_AND_EN_R = crate::BitReader<bool>;
#[doc = "Field `CCP_AND_EN` writer - 0:0\\]
Enables AND operation of the CCP outputs for timers A and B. 0 : PWM outputs of Timer A and Timer B are the internal generated PWM signals of the respective timers. 1 : PWM output of Timer A is ANDed version of Timer A and Timer B PWM signals and Timer B PWM ouput is Timer B PWM signal only."]
pub type CCP_AND_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANDCCP_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ANDCCP_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables AND operation of the CCP outputs for timers A and B. 0 : PWM outputs of Timer A and Timer B are the internal generated PWM signals of the respective timers. 1 : PWM output of Timer A is ANDed version of Timer A and Timer B PWM signals and Timer B PWM ouput is Timer B PWM signal only."]
    #[inline(always)]
    pub fn ccp_and_en(&self) -> CCP_AND_EN_R {
        CCP_AND_EN_R::new((self.bits & 1) != 0)
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
Enables AND operation of the CCP outputs for timers A and B. 0 : PWM outputs of Timer A and Timer B are the internal generated PWM signals of the respective timers. 1 : PWM output of Timer A is ANDed version of Timer A and Timer B PWM signals and Timer B PWM ouput is Timer B PWM signal only."]
    #[inline(always)]
    #[must_use]
    pub fn ccp_and_en(&mut self) -> CCP_AND_EN_W<0> {
        CCP_AND_EN_W::new(self)
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
#[doc = "Combined CCP Output This register is used to logically AND CCP output pairs for each timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [andccp](index.html) module"]
pub struct ANDCCP_SPEC;
impl crate::RegisterSpec for ANDCCP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [andccp::R](R) reader structure"]
impl crate::Readable for ANDCCP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [andccp::W](W) writer structure"]
impl crate::Writable for ANDCCP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANDCCP to value 0"]
impl crate::Resettable for ANDCCP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
