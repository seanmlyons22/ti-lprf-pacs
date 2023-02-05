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
#[doc = "Field `LAST_CH` reader - 9:9\\]
Indicates which channel was serviced last (channel 0 or channel 1) by the AHB master port."]
pub type LAST_CH_R = crate::BitReader<bool>;
#[doc = "Field `LAST_CH` writer - 9:9\\]
Indicates which channel was serviced last (channel 0 or channel 1) by the AHB master port."]
pub type LAST_CH_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAPORTERR_SPEC, bool, O>;
#[doc = "Field `RESERVED10` reader - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED10` writer - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAPORTERR_SPEC, u8, u8, 2, O>;
#[doc = "Field `AHB_ERR` reader - 12:12\\]
A 1 indicates that the Crypto peripheral has detected an AHB bus error"]
pub type AHB_ERR_R = crate::BitReader<bool>;
#[doc = "Field `AHB_ERR` writer - 12:12\\]
A 1 indicates that the Crypto peripheral has detected an AHB bus error"]
pub type AHB_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAPORTERR_SPEC, bool, O>;
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
Indicates which channel was serviced last (channel 0 or channel 1) by the AHB master port."]
    #[inline(always)]
    pub fn last_ch(&self) -> LAST_CH_R {
        LAST_CH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
A 1 indicates that the Crypto peripheral has detected an AHB bus error"]
    #[inline(always)]
    pub fn ahb_err(&self) -> AHB_ERR_R {
        AHB_ERR_R::new(((self.bits >> 12) & 1) != 0)
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
Indicates which channel was serviced last (channel 0 or channel 1) by the AHB master port."]
    #[inline(always)]
    #[must_use]
    pub fn last_ch(&mut self) -> LAST_CH_W<9> {
        LAST_CH_W::new(self)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
A 1 indicates that the Crypto peripheral has detected an AHB bus error"]
    #[inline(always)]
    #[must_use]
    pub fn ahb_err(&mut self) -> AHB_ERR_W<12> {
        AHB_ERR_W::new(self)
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
#[doc = "DMA Controller Port Error\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaporterr](index.html) module"]
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
