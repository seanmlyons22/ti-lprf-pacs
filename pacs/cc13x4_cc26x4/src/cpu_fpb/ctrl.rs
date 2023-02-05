#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - 0:0\\]
Enables the FPB"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - 0:0\\]
Enables the FPB"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `KEY` reader - 1:1\\]
Writes to the FP_CTRL are ignored unless KEY is concurrently written to one"]
pub type KEY_R = crate::BitReader<bool>;
#[doc = "Field `KEY` writer - 1:1\\]
Writes to the FP_CTRL are ignored unless KEY is concurrently written to one"]
pub type KEY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `NUM_CODE_7_4_` reader - 7:4\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
pub type NUM_CODE_7_4__R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_CODE_7_4_` writer - 7:4\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
pub type NUM_CODE_7_4__W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `NUM_LIT` reader - 11:8\\]
Indicates the number of implemented literal address comparators. The Literal Address comparators are numbered from NUM_CODE to NUM_CODE + NUM_LIT - 1"]
pub type NUM_LIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_LIT` writer - 11:8\\]
Indicates the number of implemented literal address comparators. The Literal Address comparators are numbered from NUM_CODE to NUM_CODE + NUM_LIT - 1"]
pub type NUM_LIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `NUM_CODE_14_12_` reader - 14:12\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
pub type NUM_CODE_14_12__R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_CODE_14_12_` writer - 14:12\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
pub type NUM_CODE_14_12__W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED15` reader - 27:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED15` writer - 27:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u16, u16, 13, O>;
#[doc = "Field `REV` reader - 31:28\\]
Flash Patch and Breakpoint Unit architecture revision"]
pub type REV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REV` writer - 31:28\\]
Flash Patch and Breakpoint Unit architecture revision"]
pub type REV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the FPB"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writes to the FP_CTRL are ignored unless KEY is concurrently written to one"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
    #[inline(always)]
    pub fn num_code_7_4_(&self) -> NUM_CODE_7_4__R {
        NUM_CODE_7_4__R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the number of implemented literal address comparators. The Literal Address comparators are numbered from NUM_CODE to NUM_CODE + NUM_LIT - 1"]
    #[inline(always)]
    pub fn num_lit(&self) -> NUM_LIT_R {
        NUM_LIT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
    #[inline(always)]
    pub fn num_code_14_12_(&self) -> NUM_CODE_14_12__R {
        NUM_CODE_14_12__R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:27 - 27:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 15) & 0x1fff) as u16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Flash Patch and Breakpoint Unit architecture revision"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the FPB"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Writes to the FP_CTRL are ignored unless KEY is concurrently written to one"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<1> {
        KEY_W::new(self)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
    #[inline(always)]
    #[must_use]
    pub fn num_code_7_4_(&mut self) -> NUM_CODE_7_4__W<4> {
        NUM_CODE_7_4__W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Indicates the number of implemented literal address comparators. The Literal Address comparators are numbered from NUM_CODE to NUM_CODE + NUM_LIT - 1"]
    #[inline(always)]
    #[must_use]
    pub fn num_lit(&mut self) -> NUM_LIT_W<8> {
        NUM_LIT_W::new(self)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Indicates the number of implemented instruction address comparators. Zero indicates no Instruction Address comparators are implemented. The Instruction Address comparators are numbered from 0 to NUM_CODE - 1"]
    #[inline(always)]
    #[must_use]
    pub fn num_code_14_12_(&mut self) -> NUM_CODE_14_12__W<12> {
        NUM_CODE_14_12__W::new(self)
    }
    #[doc = "Bits 15:27 - 27:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> RESERVED15_W<15> {
        RESERVED15_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Flash Patch and Breakpoint Unit architecture revision"]
    #[inline(always)]
    #[must_use]
    pub fn rev(&mut self) -> REV_W<28> {
        REV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides FPB implementation information, and the global enable for the FPB unit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
