#[doc = "Register `DMAPORTERR` reader"]
pub struct R(crate::R<DMAPORTERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAPORTERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAPORTERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAPORTERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAPORTERR` writer"]
pub struct W(crate::W<DMAPORTERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAPORTERR_SPEC>;
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
impl From<crate::W<DMAPORTERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAPORTERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 8:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED0` writer - 8:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMAPORTERR_SPEC, u16, u16, 9, O>;
#[doc = "Field `PORT1_CHANNEL` reader - 9:9\\]
Indicates which channel has serviced last (channel 0 or channel 1) by AHB master port."]
pub type PORT1_CHANNEL_R = crate::BitReader<bool>;
#[doc = "Field `PORT1_CHANNEL` writer - 9:9\\]
Indicates which channel has serviced last (channel 0 or channel 1) by AHB master port."]
pub type PORT1_CHANNEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAPORTERR_SPEC, bool, O>;
#[doc = "Field `RESERVED10` reader - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED10` writer - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAPORTERR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PORT1_AHB_ERROR` reader - 12:12\\]
A value of 1 indicates that the EIP-101 has detected an AHB bus error"]
pub type PORT1_AHB_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `PORT1_AHB_ERROR` writer - 12:12\\]
A value of 1 indicates that the EIP-101 has detected an AHB bus error"]
pub type PORT1_AHB_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAPORTERR_SPEC, bool, O>;
#[doc = "Field `RESERVED13` reader - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED13` writer - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMAPORTERR_SPEC, u32, u32, 19, O>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates which channel has serviced last (channel 0 or channel 1) by AHB master port."]
    #[inline(always)]
    pub fn port1_channel(&self) -> PORT1_CHANNEL_R {
        PORT1_CHANNEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
A value of 1 indicates that the EIP-101 has detected an AHB bus error"]
    #[inline(always)]
    pub fn port1_ahb_error(&self) -> PORT1_AHB_ERROR_R {
        PORT1_AHB_ERROR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Indicates which channel has serviced last (channel 0 or channel 1) by AHB master port."]
    #[inline(always)]
    #[must_use]
    pub fn port1_channel(&mut self) -> PORT1_CHANNEL_W<9> {
        PORT1_CHANNEL_W::new(self)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
A value of 1 indicates that the EIP-101 has detected an AHB bus error"]
    #[inline(always)]
    #[must_use]
    pub fn port1_ahb_error(&mut self) -> PORT1_AHB_ERROR_W<12> {
        PORT1_AHB_ERROR_W::new(self)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> RESERVED13_W<13> {
        RESERVED13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Port Error Raw Status This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMASWRESET register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaporterr](index.html) module"]
pub struct DMAPORTERR_SPEC;
impl crate::RegisterSpec for DMAPORTERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaporterr::R](R) reader structure"]
impl crate::Readable for DMAPORTERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaporterr::W](W) writer structure"]
impl crate::Writable for DMAPORTERR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAPORTERR to value 0"]
impl crate::Resettable for DMAPORTERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
