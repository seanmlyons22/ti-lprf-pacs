#[doc = "Register `OPTIONS` reader"]
pub struct R(crate::R<OPTIONS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTIONS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTIONS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTIONS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPTIONS` writer"]
pub struct W(crate::W<OPTIONS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTIONS_SPEC>;
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
impl From<crate::W<OPTIONS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTIONS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLB_INTERFACE` reader - 0:0\\]
When set to '1', indicates that the EIP150 is equipped with a PLB interface"]
pub type PLB_INTERFACE_R = crate::BitReader<bool>;
#[doc = "Field `PLB_INTERFACE` writer - 0:0\\]
When set to '1', indicates that the EIP150 is equipped with a PLB interface"]
pub type PLB_INTERFACE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS_SPEC, bool, O>;
#[doc = "Field `AHB_INTERFACE` reader - 1:1\\]
When set to '1', indicates that the EIP150 is equipped with a AHB interface"]
pub type AHB_INTERFACE_R = crate::BitReader<bool>;
#[doc = "Field `AHB_INTERFACE` writer - 1:1\\]
When set to '1', indicates that the EIP150 is equipped with a AHB interface"]
pub type AHB_INTERFACE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS_SPEC, bool, O>;
#[doc = "Field `AHB_IS_ASYNC` reader - 2:2\\]
When set to '1', indicates that AHB interface is asynchronous Only applicable when AHB_INTERFACE is 1"]
pub type AHB_IS_ASYNC_R = crate::BitReader<bool>;
#[doc = "Field `AHB_IS_ASYNC` writer - 2:2\\]
When set to '1', indicates that AHB interface is asynchronous Only applicable when AHB_INTERFACE is 1"]
pub type AHB_IS_ASYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS_SPEC, bool, O>;
#[doc = "Field `AXI_INTERFACE` reader - 3:3\\]
When set to '1', indicates that the EIP150 is equipped with a AXI interface"]
pub type AXI_INTERFACE_R = crate::BitReader<bool>;
#[doc = "Field `AXI_INTERFACE` writer - 3:3\\]
When set to '1', indicates that the EIP150 is equipped with a AXI interface"]
pub type AXI_INTERFACE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS_SPEC, bool, O>;
#[doc = "Field `RESERVED4` reader - 7:4\\]
Ignore on read"]
pub type RESERVED4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED4` writer - 7:4\\]
Ignore on read"]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTIONS_SPEC, u8, u8, 4, O>;
#[doc = "Field `EIP28_PRESENT` reader - 8:8\\]
When set to '1', indicates that the EIP28 PKA is included in the EIP150"]
pub type EIP28_PRESENT_R = crate::BitReader<bool>;
#[doc = "Field `EIP28_PRESENT` writer - 8:8\\]
When set to '1', indicates that the EIP28 PKA is included in the EIP150"]
pub type EIP28_PRESENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS_SPEC, bool, O>;
#[doc = "Field `EIP76_PRESENT` reader - 9:9\\]
When set to '1', indicates that the EIP76 TRNG is included in the EIP150"]
pub type EIP76_PRESENT_R = crate::BitReader<bool>;
#[doc = "Field `EIP76_PRESENT` writer - 9:9\\]
When set to '1', indicates that the EIP76 TRNG is included in the EIP150"]
pub type EIP76_PRESENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS_SPEC, bool, O>;
#[doc = "Field `AIC_PRESENT` reader - 10:10\\]
When set to '1', indicates that an EIP201 AIC is included in the EIP150"]
pub type AIC_PRESENT_R = crate::BitReader<bool>;
#[doc = "Field `AIC_PRESENT` writer - 10:10\\]
When set to '1', indicates that an EIP201 AIC is included in the EIP150"]
pub type AIC_PRESENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTIONS_SPEC, bool, O>;
#[doc = "Field `RESERVED11` reader - 31:11\\]
Ignore on read"]
pub type RESERVED11_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED11` writer - 31:11\\]
Ignore on read"]
pub type RESERVED11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTIONS_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
When set to '1', indicates that the EIP150 is equipped with a PLB interface"]
    #[inline(always)]
    pub fn plb_interface(&self) -> PLB_INTERFACE_R {
        PLB_INTERFACE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
When set to '1', indicates that the EIP150 is equipped with a AHB interface"]
    #[inline(always)]
    pub fn ahb_interface(&self) -> AHB_INTERFACE_R {
        AHB_INTERFACE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
When set to '1', indicates that AHB interface is asynchronous Only applicable when AHB_INTERFACE is 1"]
    #[inline(always)]
    pub fn ahb_is_async(&self) -> AHB_IS_ASYNC_R {
        AHB_IS_ASYNC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
When set to '1', indicates that the EIP150 is equipped with a AXI interface"]
    #[inline(always)]
    pub fn axi_interface(&self) -> AXI_INTERFACE_R {
        AXI_INTERFACE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
When set to '1', indicates that the EIP28 PKA is included in the EIP150"]
    #[inline(always)]
    pub fn eip28_present(&self) -> EIP28_PRESENT_R {
        EIP28_PRESENT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
When set to '1', indicates that the EIP76 TRNG is included in the EIP150"]
    #[inline(always)]
    pub fn eip76_present(&self) -> EIP76_PRESENT_R {
        EIP76_PRESENT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
When set to '1', indicates that an EIP201 AIC is included in the EIP150"]
    #[inline(always)]
    pub fn aic_present(&self) -> AIC_PRESENT_R {
        AIC_PRESENT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
When set to '1', indicates that the EIP150 is equipped with a PLB interface"]
    #[inline(always)]
    #[must_use]
    pub fn plb_interface(&mut self) -> PLB_INTERFACE_W<0> {
        PLB_INTERFACE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
When set to '1', indicates that the EIP150 is equipped with a AHB interface"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_interface(&mut self) -> AHB_INTERFACE_W<1> {
        AHB_INTERFACE_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
When set to '1', indicates that AHB interface is asynchronous Only applicable when AHB_INTERFACE is 1"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_is_async(&mut self) -> AHB_IS_ASYNC_W<2> {
        AHB_IS_ASYNC_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
When set to '1', indicates that the EIP150 is equipped with a AXI interface"]
    #[inline(always)]
    #[must_use]
    pub fn axi_interface(&mut self) -> AXI_INTERFACE_W<3> {
        AXI_INTERFACE_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
When set to '1', indicates that the EIP28 PKA is included in the EIP150"]
    #[inline(always)]
    #[must_use]
    pub fn eip28_present(&mut self) -> EIP28_PRESENT_W<8> {
        EIP28_PRESENT_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
When set to '1', indicates that the EIP76 TRNG is included in the EIP150"]
    #[inline(always)]
    #[must_use]
    pub fn eip76_present(&mut self) -> EIP76_PRESENT_W<9> {
        EIP76_PRESENT_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
When set to '1', indicates that an EIP201 AIC is included in the EIP150"]
    #[inline(always)]
    #[must_use]
    pub fn aic_present(&mut self) -> AIC_PRESENT_W<10> {
        AIC_PRESENT_W::new(self)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> RESERVED11_W<11> {
        RESERVED11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA Options register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [options](index.html) module"]
pub struct OPTIONS_SPEC;
impl crate::RegisterSpec for OPTIONS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [options::R](R) reader structure"]
impl crate::Readable for OPTIONS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [options::W](W) writer structure"]
impl crate::Writable for OPTIONS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPTIONS to value 0x0102"]
impl crate::Resettable for OPTIONS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0102;
}
