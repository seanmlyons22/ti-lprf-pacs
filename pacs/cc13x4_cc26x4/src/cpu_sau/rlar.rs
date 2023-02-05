#[doc = "Register `RLAR` reader"]
pub struct R(crate::R<RLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RLAR` writer"]
pub struct W(crate::W<RLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RLAR_SPEC>;
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
impl From<crate::W<RLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - 0:0\\]
SAU region enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - 0:0\\]
SAU region enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RLAR_SPEC, bool, O>;
#[doc = "Field `NSC` reader - 1:1\\]
Controls whether Non-secure state is permitted to execute an SG instruction from this region"]
pub type NSC_R = crate::BitReader<bool>;
#[doc = "Field `NSC` writer - 1:1\\]
Controls whether Non-secure state is permitted to execute an SG instruction from this region"]
pub type NSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RLAR_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 4:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 4:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLAR_SPEC, u8, u8, 3, O>;
#[doc = "Field `LADDR` reader - 31:5\\]
Holds bits \\[31:5\\]
of the limit address for the selected SAU region"]
pub type LADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LADDR` writer - 31:5\\]
Holds bits \\[31:5\\]
of the limit address for the selected SAU region"]
pub type LADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLAR_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SAU region enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Controls whether Non-secure state is permitted to execute an SG instruction from this region"]
    #[inline(always)]
    pub fn nsc(&self) -> NSC_R {
        NSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Holds bits \\[31:5\\]
of the limit address for the selected SAU region"]
    #[inline(always)]
    pub fn laddr(&self) -> LADDR_R {
        LADDR_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SAU region enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Controls whether Non-secure state is permitted to execute an SG instruction from this region"]
    #[inline(always)]
    #[must_use]
    pub fn nsc(&mut self) -> NSC_W<1> {
        NSC_W::new(self)
    }
    #[doc = "Bits 2:4 - 4:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Holds bits \\[31:5\\]
of the limit address for the selected SAU region"]
    #[inline(always)]
    #[must_use]
    pub fn laddr(&mut self) -> LADDR_W<5> {
        LADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides indirect read and write access to the limit address of the currently selected SAU region\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlar](index.html) module"]
pub struct RLAR_SPEC;
impl crate::RegisterSpec for RLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rlar::R](R) reader structure"]
impl crate::Readable for RLAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rlar::W](W) writer structure"]
impl crate::Writable for RLAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RLAR to value 0"]
impl crate::Resettable for RLAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
