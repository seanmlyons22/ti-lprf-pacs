#[doc = "Register `MAC_15_4_1` reader"]
pub struct R(crate::R<MAC_15_4_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_15_4_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_15_4_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_15_4_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_15_4_1` writer"]
pub struct W(crate::W<MAC_15_4_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_15_4_1_SPEC>;
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
impl From<crate::W<MAC_15_4_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_15_4_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR_32_63` reader - 31:0\\]
The last 32-bits of the 64-bit MAC 15.4 address"]
pub type ADDR_32_63_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR_32_63` writer - 31:0\\]
The last 32-bits of the 64-bit MAC 15.4 address"]
pub type ADDR_32_63_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MAC_15_4_1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
The last 32-bits of the 64-bit MAC 15.4 address"]
    #[inline(always)]
    pub fn addr_32_63(&self) -> ADDR_32_63_R {
        ADDR_32_63_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
The last 32-bits of the 64-bit MAC 15.4 address"]
    #[inline(always)]
    #[must_use]
    pub fn addr_32_63(&mut self) -> ADDR_32_63_W<0> {
        ADDR_32_63_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC IEEE 802.15.4 Address 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_15_4_1](index.html) module"]
pub struct MAC_15_4_1_SPEC;
impl crate::RegisterSpec for MAC_15_4_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_15_4_1::R](R) reader structure"]
impl crate::Readable for MAC_15_4_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_15_4_1::W](W) writer structure"]
impl crate::Writable for MAC_15_4_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_15_4_1 to value 0"]
impl crate::Resettable for MAC_15_4_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
