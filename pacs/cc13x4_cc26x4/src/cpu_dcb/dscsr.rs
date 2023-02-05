#[doc = "Register `DSCSR` reader"]
pub struct R(crate::R<DSCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSCSR` writer"]
pub struct W(crate::W<DSCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSCSR_SPEC>;
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
impl From<crate::W<DSCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBRSELEN` reader - 0:0\\]
Controls whether the SBRSEL field or the current Security state of the processor selects which version of the memory-mapped Banked registers are accessed to the debugger"]
pub type SBRSELEN_R = crate::BitReader<bool>;
#[doc = "Field `SBRSELEN` writer - 0:0\\]
Controls whether the SBRSEL field or the current Security state of the processor selects which version of the memory-mapped Banked registers are accessed to the debugger"]
pub type SBRSELEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSCSR_SPEC, bool, O>;
#[doc = "Field `SBRSEL` reader - 1:1\\]
If SBRSELEN is 1 this bit selects whether the Non-secure or the Secure version of the memory-mapped Banked registers are accessible to the debugger"]
pub type SBRSEL_R = crate::BitReader<bool>;
#[doc = "Field `SBRSEL` writer - 1:1\\]
If SBRSELEN is 1 this bit selects whether the Non-secure or the Secure version of the memory-mapped Banked registers are accessible to the debugger"]
pub type SBRSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSCSR_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 15:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED2` writer - 15:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSCSR_SPEC, u16, u16, 14, O>;
#[doc = "Field `CDS` reader - 16:16\\]
This field indicates the current Security state of the processor"]
pub type CDS_R = crate::BitReader<bool>;
#[doc = "Field `CDS` writer - 16:16\\]
This field indicates the current Security state of the processor"]
pub type CDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSCSR_SPEC, bool, O>;
#[doc = "Field `CDSKEY` reader - 17:17\\]
Writes to the CDS bit are ignored unless CDSKEY is concurrently written to zero"]
pub type CDSKEY_R = crate::BitReader<bool>;
#[doc = "Field `CDSKEY` writer - 17:17\\]
Writes to the CDS bit are ignored unless CDSKEY is concurrently written to zero"]
pub type CDSKEY_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSCSR_SPEC, bool, O>;
#[doc = "Field `RESERVED18` reader - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED18_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED18` writer - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSCSR_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Controls whether the SBRSEL field or the current Security state of the processor selects which version of the memory-mapped Banked registers are accessed to the debugger"]
    #[inline(always)]
    pub fn sbrselen(&self) -> SBRSELEN_R {
        SBRSELEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If SBRSELEN is 1 this bit selects whether the Non-secure or the Secure version of the memory-mapped Banked registers are accessible to the debugger"]
    #[inline(always)]
    pub fn sbrsel(&self) -> SBRSEL_R {
        SBRSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
This field indicates the current Security state of the processor"]
    #[inline(always)]
    pub fn cds(&self) -> CDS_R {
        CDS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Writes to the CDS bit are ignored unless CDSKEY is concurrently written to zero"]
    #[inline(always)]
    pub fn cdskey(&self) -> CDSKEY_R {
        CDSKEY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> RESERVED18_R {
        RESERVED18_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Controls whether the SBRSEL field or the current Security state of the processor selects which version of the memory-mapped Banked registers are accessed to the debugger"]
    #[inline(always)]
    #[must_use]
    pub fn sbrselen(&mut self) -> SBRSELEN_W<0> {
        SBRSELEN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
If SBRSELEN is 1 this bit selects whether the Non-secure or the Secure version of the memory-mapped Banked registers are accessible to the debugger"]
    #[inline(always)]
    #[must_use]
    pub fn sbrsel(&mut self) -> SBRSEL_W<1> {
        SBRSEL_W::new(self)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
This field indicates the current Security state of the processor"]
    #[inline(always)]
    #[must_use]
    pub fn cds(&mut self) -> CDS_W<16> {
        CDS_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Writes to the CDS bit are ignored unless CDSKEY is concurrently written to zero"]
    #[inline(always)]
    #[must_use]
    pub fn cdskey(&mut self) -> CDSKEY_W<17> {
        CDSKEY_W::new(self)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> RESERVED18_W<18> {
        RESERVED18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides control and status information for Secure debug\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dscsr](index.html) module"]
pub struct DSCSR_SPEC;
impl crate::RegisterSpec for DSCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dscsr::R](R) reader structure"]
impl crate::Readable for DSCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dscsr::W](W) writer structure"]
impl crate::Writable for DSCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSCSR to value 0"]
impl crate::Resettable for DSCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
