#[doc = "Register `ACTLR` reader"]
pub struct R(crate::R<ACTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTLR` writer"]
pub struct W(crate::W<ACTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTLR_SPEC>;
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
impl From<crate::W<ACTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISMCYCINT` reader - 0:0\\]
Disables interruption of multi-cycle instructions. This increases the interrupt latency of the processor becuase LDM/STM completes before interrupt stacking occurs."]
pub type DISMCYCINT_R = crate::BitReader<bool>;
#[doc = "Field `DISMCYCINT` writer - 0:0\\]
Disables interruption of multi-cycle instructions. This increases the interrupt latency of the processor becuase LDM/STM completes before interrupt stacking occurs."]
pub type DISMCYCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `DISDEFWBUF` reader - 1:1\\]
Disables write buffer use during default memory map accesses. This causes all bus faults to be precise bus faults but decreases the performance of the processor because the stores to memory have to complete before the next instruction can be executed."]
pub type DISDEFWBUF_R = crate::BitReader<bool>;
#[doc = "Field `DISDEFWBUF` writer - 1:1\\]
Disables write buffer use during default memory map accesses. This causes all bus faults to be precise bus faults but decreases the performance of the processor because the stores to memory have to complete before the next instruction can be executed."]
pub type DISDEFWBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `DISFOLD` reader - 2:2\\]
Disables folding of IT instruction."]
pub type DISFOLD_R = crate::BitReader<bool>;
#[doc = "Field `DISFOLD` writer - 2:2\\]
Disables folding of IT instruction."]
pub type DISFOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED3` writer - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACTLR_SPEC, u8, u8, 5, O>;
#[doc = "Field `DISFPCA` reader - 8:8\\]
Disable automatic update of CONTROL.FPCA"]
pub type DISFPCA_R = crate::BitReader<bool>;
#[doc = "Field `DISFPCA` writer - 8:8\\]
Disable automatic update of CONTROL.FPCA"]
pub type DISFPCA_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED9` writer - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `RESERVED10` reader - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED10` writer - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACTLR_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Disables interruption of multi-cycle instructions. This increases the interrupt latency of the processor becuase LDM/STM completes before interrupt stacking occurs."]
    #[inline(always)]
    pub fn dismcycint(&self) -> DISMCYCINT_R {
        DISMCYCINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Disables write buffer use during default memory map accesses. This causes all bus faults to be precise bus faults but decreases the performance of the processor because the stores to memory have to complete before the next instruction can be executed."]
    #[inline(always)]
    pub fn disdefwbuf(&self) -> DISDEFWBUF_R {
        DISDEFWBUF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Disables folding of IT instruction."]
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Disable automatic update of CONTROL.FPCA"]
    #[inline(always)]
    pub fn disfpca(&self) -> DISFPCA_R {
        DISFPCA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Disables interruption of multi-cycle instructions. This increases the interrupt latency of the processor becuase LDM/STM completes before interrupt stacking occurs."]
    #[inline(always)]
    #[must_use]
    pub fn dismcycint(&mut self) -> DISMCYCINT_W<0> {
        DISMCYCINT_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Disables write buffer use during default memory map accesses. This causes all bus faults to be precise bus faults but decreases the performance of the processor because the stores to memory have to complete before the next instruction can be executed."]
    #[inline(always)]
    #[must_use]
    pub fn disdefwbuf(&mut self) -> DISDEFWBUF_W<1> {
        DISDEFWBUF_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Disables folding of IT instruction."]
    #[inline(always)]
    #[must_use]
    pub fn disfold(&mut self) -> DISFOLD_W<2> {
        DISFOLD_W::new(self)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Disable automatic update of CONTROL.FPCA"]
    #[inline(always)]
    #[must_use]
    pub fn disfpca(&mut self) -> DISFPCA_W<8> {
        DISFPCA_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Control This register is used to disable certain aspects of functionality within the processor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actlr](index.html) module"]
pub struct ACTLR_SPEC;
impl crate::RegisterSpec for ACTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [actlr::R](R) reader structure"]
impl crate::Readable for ACTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [actlr::W](W) writer structure"]
impl crate::Writable for ACTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACTLR to value 0"]
impl crate::Resettable for ACTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
