#[doc = "Register `PER_CTL` reader"]
pub struct R(crate::R<PER_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PER_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PER_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PER_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PER_CTL` writer"]
pub struct W(crate::W<PER_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PER_CTL_SPEC>;
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
impl From<crate::W<PER_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PER_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PER_DEBUG_ENABLE` reader - 0:0\\]
Parity Error Debug Enable 0: Normal operation 1: An address offset can be written to PER_DBG.PER_DEBUG_ADDR and parity errors will be generated on reads from within this offset"]
pub type PER_DEBUG_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PER_DEBUG_ENABLE` writer - 0:0\\]
Parity Error Debug Enable 0: Normal operation 1: An address offset can be written to PER_DBG.PER_DEBUG_ADDR and parity errors will be generated on reads from within this offset"]
pub type PER_DEBUG_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CTL_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PER_CTL_SPEC, u8, u8, 7, O>;
#[doc = "Field `PER_DISABLE` reader - 8:8\\]
Parity Status Disable 0: A parity error will update PER_CHK.PER_ADDR field 1: Parity error does not update PER_CHK.PER_ADDR field"]
pub type PER_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `PER_DISABLE` writer - 8:8\\]
Parity Status Disable 0: A parity error will update PER_CHK.PER_ADDR field 1: Parity error does not update PER_CHK.PER_ADDR field"]
pub type PER_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_CTL_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PER_CTL_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Parity Error Debug Enable 0: Normal operation 1: An address offset can be written to PER_DBG.PER_DEBUG_ADDR and parity errors will be generated on reads from within this offset"]
    #[inline(always)]
    pub fn per_debug_enable(&self) -> PER_DEBUG_ENABLE_R {
        PER_DEBUG_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity Status Disable 0: A parity error will update PER_CHK.PER_ADDR field 1: Parity error does not update PER_CHK.PER_ADDR field"]
    #[inline(always)]
    pub fn per_disable(&self) -> PER_DISABLE_R {
        PER_DISABLE_R::new(((self.bits >> 8) & 1) != 0)
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
Parity Error Debug Enable 0: Normal operation 1: An address offset can be written to PER_DBG.PER_DEBUG_ADDR and parity errors will be generated on reads from within this offset"]
    #[inline(always)]
    #[must_use]
    pub fn per_debug_enable(&mut self) -> PER_DEBUG_ENABLE_W<0> {
        PER_DEBUG_ENABLE_W::new(self)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity Status Disable 0: A parity error will update PER_CHK.PER_ADDR field 1: Parity error does not update PER_CHK.PER_ADDR field"]
    #[inline(always)]
    #[must_use]
    pub fn per_disable(&mut self) -> PER_DISABLE_W<8> {
        PER_DISABLE_W::new(self)
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
#[doc = "Parity Error Control Parity error check controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_ctl](index.html) module"]
pub struct PER_CTL_SPEC;
impl crate::RegisterSpec for PER_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [per_ctl::R](R) reader structure"]
impl crate::Readable for PER_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [per_ctl::W](W) writer structure"]
impl crate::Writable for PER_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PER_CTL to value 0"]
impl crate::Resettable for PER_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
