#[doc = "Register `RESETSSI` reader"]
pub struct R(crate::R<RESETSSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESETSSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESETSSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESETSSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESETSSI` writer"]
pub struct W(crate::W<RESETSSI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESETSSI_SPEC>;
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
impl From<crate::W<RESETSSI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESETSSI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSI` reader - 1:0\\]
SSI 0: 0: No action 1: Reset SSI. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset. NOTE: MSB is reserverd Writing any other value than the reset value may result in undefined behavior."]
pub type SSI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSI` writer - 1:0\\]
SSI 0: 0: No action 1: Reset SSI. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset. NOTE: MSB is reserverd Writing any other value than the reset value may result in undefined behavior."]
pub type SSI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESETSSI_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESETSSI_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
SSI 0: 0: No action 1: Reset SSI. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset. NOTE: MSB is reserverd Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
SSI 0: 0: No action 1: Reset SSI. HW cleared. Acess will only have effect when SERIAL power domain is on, PDSTAT0.SERIAL_ON = 1 Before writing set FLASH:CFG.DIS_READACCESS = 1 to ensure the reset is not activated while executing from flash. This means one cannot execute from flash when using the SW reset. NOTE: MSB is reserverd Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn ssi(&mut self) -> SSI_W<0> {
        SSI_W::new(self)
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
#[doc = "RESET For SSI IPs\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetssi](index.html) module"]
pub struct RESETSSI_SPEC;
impl crate::RegisterSpec for RESETSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resetssi::R](R) reader structure"]
impl crate::Readable for RESETSSI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resetssi::W](W) writer structure"]
impl crate::Writable for RESETSSI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESETSSI to value 0"]
impl crate::Resettable for RESETSSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
