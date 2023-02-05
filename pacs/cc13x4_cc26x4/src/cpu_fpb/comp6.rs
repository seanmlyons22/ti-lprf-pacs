#[doc = "Register `COMP6` reader"]
pub struct R(crate::R<COMP6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP6` writer"]
pub struct W(crate::W<COMP6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP6_SPEC>;
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
impl From<crate::W<COMP6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BE` reader - 0:0\\]
Selects between flashpatch and breakpoint functionality"]
pub type BE_R = crate::BitReader<bool>;
#[doc = "Field `BE` writer - 0:0\\]
Selects between flashpatch and breakpoint functionality"]
pub type BE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP6_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP6_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects between flashpatch and breakpoint functionality"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects between flashpatch and breakpoint functionality"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BE_W<0> {
        BE_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Holds an address for comparison. The effect of the match depends on the configuration of the FPB and whether the comparator is an instruction address comparator or a literal address comparator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp6](index.html) module"]
pub struct COMP6_SPEC;
impl crate::RegisterSpec for COMP6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp6::R](R) reader structure"]
impl crate::Readable for COMP6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp6::W](W) writer structure"]
impl crate::Writable for COMP6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP6 to value 0"]
impl crate::Resettable for COMP6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
