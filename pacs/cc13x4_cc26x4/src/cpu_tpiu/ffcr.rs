#[doc = "Register `FFCR` reader"]
pub struct R(crate::R<FFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFCR` writer"]
pub struct W(crate::W<FFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFCR_SPEC>;
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
impl From<crate::W<FFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFCR_SPEC, bool, O>;
#[doc = "Field `ENFCONT` reader - 1:1\\]
Enable continuous formatting: 0: Continuous formatting disabled 1: Continuous formatting enabled"]
pub type ENFCONT_R = crate::BitReader<bool>;
#[doc = "Field `ENFCONT` writer - 1:1\\]
Enable continuous formatting: 0: Continuous formatting disabled 1: Continuous formatting enabled"]
pub type ENFCONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFCR_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 5:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 5:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FFCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `FONMAN` reader - 6:6\\]
Flush on manual. Value can be: 0x0 When the flush completes. Set to 0 on a reset of the TPIU. 0x1 Generates a flush."]
pub type FONMAN_R = crate::BitReader<bool>;
#[doc = "Field `FONMAN` writer - 6:6\\]
Flush on manual. Value can be: 0x0 When the flush completes. Set to 0 on a reset of the TPIU. 0x1 Generates a flush."]
pub type FONMAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFCR_SPEC, bool, O>;
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFCR_SPEC, bool, O>;
#[doc = "Field `TRIGIN` reader - 8:8\\]
Indicates that triggers are inserted when a trigger pin is asserted."]
pub type TRIGIN_R = crate::BitReader<bool>;
#[doc = "Field `TRIGIN` writer - 8:8\\]
Indicates that triggers are inserted when a trigger pin is asserted."]
pub type TRIGIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FFCR_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FFCR_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable continuous formatting: 0: Continuous formatting disabled 1: Continuous formatting enabled"]
    #[inline(always)]
    pub fn enfcont(&self) -> ENFCONT_R {
        ENFCONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - 5:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Flush on manual. Value can be: 0x0 When the flush completes. Set to 0 on a reset of the TPIU. 0x1 Generates a flush."]
    #[inline(always)]
    pub fn fonman(&self) -> FONMAN_R {
        FONMAN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates that triggers are inserted when a trigger pin is asserted."]
    #[inline(always)]
    pub fn trigin(&self) -> TRIGIN_R {
        TRIGIN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable continuous formatting: 0: Continuous formatting disabled 1: Continuous formatting enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enfcont(&mut self) -> ENFCONT_W<1> {
        ENFCONT_W::new(self)
    }
    #[doc = "Bits 2:5 - 5:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Flush on manual. Value can be: 0x0 When the flush completes. Set to 0 on a reset of the TPIU. 0x1 Generates a flush."]
    #[inline(always)]
    #[must_use]
    pub fn fonman(&mut self) -> FONMAN_W<6> {
        FONMAN_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates that triggers are inserted when a trigger pin is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn trigin(&mut self) -> TRIGIN_W<8> {
        TRIGIN_W::new(self)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Formatter and Flush Control When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption. Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffcr](index.html) module"]
pub struct FFCR_SPEC;
impl crate::RegisterSpec for FFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffcr::R](R) reader structure"]
impl crate::Readable for FFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffcr::W](W) writer structure"]
impl crate::Writable for FFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FFCR to value 0x0142"]
impl crate::Resettable for FFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0142;
}
