#[doc = "Register `RXDATA` reader"]
pub struct R(crate::R<RXDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXDATA` writer"]
pub struct W(crate::W<RXDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDATA_SPEC>;
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
impl From<crate::W<RXDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - 31:0\\]
Received Data"]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA` writer - 31:0\\]
Received Data"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXDATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Received Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Received Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RXDATA Register. Reading this register returns value in the RX FIFO pointed by the current FIFO read pointer. If the RX FIFO is empty, the last read value is returned. Writing has not effect and is ignored.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdata](index.html) module"]
pub struct RXDATA_SPEC;
impl crate::RegisterSpec for RXDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdata::R](R) reader structure"]
impl crate::Readable for RXDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdata::W](W) writer structure"]
impl crate::Writable for RXDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXDATA to value 0"]
impl crate::Resettable for RXDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
