#[doc = "Register `FBRD` reader"]
pub struct R(crate::R<FBRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBRD` writer"]
pub struct W(crate::W<FBRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBRD_SPEC>;
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
impl From<crate::W<FBRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVFRAC` reader - 5:0\\]
Fractional Baud-Rate Divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, IBRD.DIVINT=0 does not give a valid baud rate. Similarly, if IBRD.DIVINT=0xFFFF, any non-zero values in DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
pub type DIVFRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVFRAC` writer - 5:0\\]
Fractional Baud-Rate Divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, IBRD.DIVINT=0 does not give a valid baud rate. Similarly, if IBRD.DIVINT=0xFFFF, any non-zero values in DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
pub type DIVFRAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBRD_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Fractional Baud-Rate Divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, IBRD.DIVINT=0 does not give a valid baud rate. Similarly, if IBRD.DIVINT=0xFFFF, any non-zero values in DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
    #[inline(always)]
    pub fn divfrac(&self) -> DIVFRAC_R {
        DIVFRAC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Fractional Baud-Rate Divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, IBRD.DIVINT=0 does not give a valid baud rate. Similarly, if IBRD.DIVINT=0xFFFF, any non-zero values in DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
    #[inline(always)]
    #[must_use]
    pub fn divfrac(&mut self) -> DIVFRAC_W<0> {
        DIVFRAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fractional Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbrd](index.html) module"]
pub struct FBRD_SPEC;
impl crate::RegisterSpec for FBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbrd::R](R) reader structure"]
impl crate::Readable for FBRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbrd::W](W) writer structure"]
impl crate::Writable for FBRD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBRD to value 0"]
impl crate::Resettable for FBRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
