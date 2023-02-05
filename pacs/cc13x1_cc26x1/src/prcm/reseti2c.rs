#[doc = "Register `RESETI2C` reader"]
pub struct R(crate::R<RESETI2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESETI2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESETI2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESETI2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESETI2C` writer"]
pub struct W(crate::W<RESETI2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESETI2C_SPEC>;
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
impl From<crate::W<RESETI2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESETI2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C` reader - 0:0\\]
0: No action 1: Reset I2C. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type I2C_R = crate::BitReader<bool>;
#[doc = "Field `I2C` writer - 0:0\\]
0: No action 1: Reset I2C. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
pub type I2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETI2C_SPEC, bool, O>;
#[doc = "Field `SPARE1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPARE1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESETI2C_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Reset I2C. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare1(&self) -> SPARE1_R {
        SPARE1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: No action 1: Reset I2C. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset."]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2C_W<0> {
        I2C_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare1(&mut self) -> SPARE1_W<1> {
        SPARE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RESET For I2C IPs\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reseti2c](index.html) module"]
pub struct RESETI2C_SPEC;
impl crate::RegisterSpec for RESETI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reseti2c::R](R) reader structure"]
impl crate::Readable for RESETI2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reseti2c::W](W) writer structure"]
impl crate::Writable for RESETI2C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESETI2C to value 0"]
impl crate::Resettable for RESETI2C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
