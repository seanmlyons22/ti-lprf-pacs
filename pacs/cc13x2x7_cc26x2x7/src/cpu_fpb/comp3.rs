#[doc = "Register `COMP3` reader"]
pub struct R(crate::R<COMP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP3` writer"]
pub struct W(crate::W<COMP3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP3_SPEC>;
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
impl From<crate::W<COMP3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - 0:0\\]
Compare and remap enable comparator 3. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 3 disabled 0x1: Compare and remap for comparator 3 enabled"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - 0:0\\]
Compare and remap enable comparator 3. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 3 disabled 0x1: Compare and remap for comparator 3 enabled"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP3_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED1` writer - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP3_SPEC, bool, O>;
#[doc = "Field `COMP` reader - 28:2\\]
Comparison address."]
pub type COMP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COMP` writer - 28:2\\]
Comparison address."]
pub type COMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP3_SPEC, u32, u32, 27, O>;
#[doc = "Field `RESERVED29` reader - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED29_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED29` writer - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED29_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP3_SPEC, bool, O>;
#[doc = "Field `REPLACE` reader - 31:30\\]
This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
pub type REPLACE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REPLACE` writer - 31:30\\]
This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
pub type REPLACE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Compare and remap enable comparator 3. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 3 disabled 0x1: Compare and remap for comparator 3 enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:28 - 28:2\\]
Comparison address."]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new((self.bits >> 2) & 0x07ff_ffff)
    }
    #[doc = "Bit 29 - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> RESERVED29_R {
        RESERVED29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[inline(always)]
    pub fn replace(&self) -> REPLACE_R {
        REPLACE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Compare and remap enable comparator 3. CTRL.ENABLE must also be set to enable comparisons. 0x0: Compare and remap for comparator 3 disabled 0x1: Compare and remap for comparator 3 enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bits 2:28 - 28:2\\]
Comparison address."]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<2> {
        COMP_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved29(&mut self) -> RESERVED29_W<29> {
        RESERVED29_W::new(self)
    }
    #[doc = "Bits 30:31 - 31:30\\]
This selects what happens when the COMP address is matched. Address remapping only takes place for the 0x0 setting. 0x0: Remap to remap address. See REMAP.REMAP 0x1: Set BKPT on lower halfword, upper is unaffected 0x2: Set BKPT on upper halfword, lower is unaffected 0x3: Set BKPT on both lower and upper halfwords."]
    #[inline(always)]
    #[must_use]
    pub fn replace(&mut self) -> REPLACE_W<30> {
        REPLACE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp3](index.html) module"]
pub struct COMP3_SPEC;
impl crate::RegisterSpec for COMP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp3::R](R) reader structure"]
impl crate::Readable for COMP3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp3::W](W) writer structure"]
impl crate::Writable for COMP3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP3 to value 0"]
impl crate::Resettable for COMP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
