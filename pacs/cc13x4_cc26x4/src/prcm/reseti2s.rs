#[doc = "Register `RESETI2S` reader"]
pub struct R(crate::R<RESETI2S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESETI2S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESETI2S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESETI2S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESETI2S` writer"]
pub struct W(crate::W<RESETI2S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESETI2S_SPEC>;
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
impl From<crate::W<RESETI2S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESETI2S_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2S` reader - 0:0\\]
0: No action 1: Reset module. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type I2S_R = crate::BitReader<bool>;
#[doc = "Field `I2S` writer - 0:0\\]
0: No action 1: Reset module. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type I2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETI2S_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESETI2S_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Reset module. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new((self.bits & 1) != 0)
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
0: No action 1: Reset module. HW cleared. Access will only have effect when PERIPH power domain is on, PDSTAT0.PERIPH_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn i2s(&mut self) -> I2S_W<0> {
        I2S_W::new(self)
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
#[doc = "RESET For I2S IP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reseti2s](index.html) module"]
pub struct RESETI2S_SPEC;
impl crate::RegisterSpec for RESETI2S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reseti2s::R](R) reader structure"]
impl crate::Readable for RESETI2S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reseti2s::W](W) writer structure"]
impl crate::Writable for RESETI2S_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESETI2S to value 0"]
impl crate::Resettable for RESETI2S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
