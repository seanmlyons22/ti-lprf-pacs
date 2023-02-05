#[doc = "Register `IEEE_BLE_1` reader"]
pub struct R(crate::R<IEEE_BLE_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEEE_BLE_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEEE_BLE_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEEE_BLE_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEEE_BLE_1` writer"]
pub struct W(crate::W<IEEE_BLE_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEEE_BLE_1_SPEC>;
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
impl From<crate::W<IEEE_BLE_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEEE_BLE_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - 31:0\\]
Bits\\[63:32\\]
of the 64-bits custom IEEE BLE address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - 31:0\\]
Bits\\[63:32\\]
of the 64-bits custom IEEE BLE address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IEEE_BLE_1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Bits\\[63:32\\]
of the 64-bits custom IEEE BLE address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Bits\\[63:32\\]
of the 64-bits custom IEEE BLE address. If different from 0xFFFFFFFF then the value of this field is applied; otherwise use value from FCFG."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IEEE BLE Address 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_ble_1](index.html) module"]
pub struct IEEE_BLE_1_SPEC;
impl crate::RegisterSpec for IEEE_BLE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ieee_ble_1::R](R) reader structure"]
impl crate::Readable for IEEE_BLE_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ieee_ble_1::W](W) writer structure"]
impl crate::Writable for IEEE_BLE_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEEE_BLE_1 to value 0xffff_ffff"]
impl crate::Resettable for IEEE_BLE_1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
