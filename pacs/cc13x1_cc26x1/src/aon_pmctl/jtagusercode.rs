#[doc = "Register `JTAGUSERCODE` reader"]
pub struct R(crate::R<JTAGUSERCODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JTAGUSERCODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JTAGUSERCODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JTAGUSERCODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JTAGUSERCODE` writer"]
pub struct W(crate::W<JTAGUSERCODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JTAGUSERCODE_SPEC>;
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
impl From<crate::W<JTAGUSERCODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JTAGUSERCODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USER_CODE` reader - 31:0\\]
32-bit JTAG USERCODE register feeding main JTAG TAP Note: This field can be locked by LOCKCFG.LOCK"]
pub type USER_CODE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `USER_CODE` writer - 31:0\\]
32-bit JTAG USERCODE register feeding main JTAG TAP Note: This field can be locked by LOCKCFG.LOCK"]
pub type USER_CODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, JTAGUSERCODE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
32-bit JTAG USERCODE register feeding main JTAG TAP Note: This field can be locked by LOCKCFG.LOCK"]
    #[inline(always)]
    pub fn user_code(&self) -> USER_CODE_R {
        USER_CODE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
32-bit JTAG USERCODE register feeding main JTAG TAP Note: This field can be locked by LOCKCFG.LOCK"]
    #[inline(always)]
    #[must_use]
    pub fn user_code(&mut self) -> USER_CODE_W<0> {
        USER_CODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtagusercode](index.html) module"]
pub struct JTAGUSERCODE_SPEC;
impl crate::RegisterSpec for JTAGUSERCODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jtagusercode::R](R) reader structure"]
impl crate::Readable for JTAGUSERCODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jtagusercode::W](W) writer structure"]
impl crate::Writable for JTAGUSERCODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JTAGUSERCODE to value 0x0b99_a02f"]
impl crate::Resettable for JTAGUSERCODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b99_a02f;
}
