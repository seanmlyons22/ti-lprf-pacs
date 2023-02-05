#[doc = "Register `LSUCNT` reader"]
pub struct R(crate::R<LSUCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSUCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSUCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSUCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSUCNT` writer"]
pub struct W(crate::W<LSUCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSUCNT_SPEC>;
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
impl From<crate::W<LSUCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSUCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSUCNT` reader - 7:0\\]
LSU counter. This counts the total number of cycles that the processor is processing an LSU operation. The initial execution cost of the instruction is not counted. For example, an LDR that takes two cycles to complete increments this counter one cycle. Equivalently, an LDR that stalls for two cycles (i.e. takes four cycles to execute), increments this counter three times. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.LSUEVTENA."]
pub type LSUCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSUCNT` writer - 7:0\\]
LSU counter. This counts the total number of cycles that the processor is processing an LSU operation. The initial execution cost of the instruction is not counted. For example, an LDR that takes two cycles to complete increments this counter one cycle. Equivalently, an LDR that stalls for two cycles (i.e. takes four cycles to execute), increments this counter three times. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.LSUEVTENA."]
pub type LSUCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LSUCNT_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LSUCNT_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
LSU counter. This counts the total number of cycles that the processor is processing an LSU operation. The initial execution cost of the instruction is not counted. For example, an LDR that takes two cycles to complete increments this counter one cycle. Equivalently, an LDR that stalls for two cycles (i.e. takes four cycles to execute), increments this counter three times. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.LSUEVTENA."]
    #[inline(always)]
    pub fn lsucnt(&self) -> LSUCNT_R {
        LSUCNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
LSU counter. This counts the total number of cycles that the processor is processing an LSU operation. The initial execution cost of the instruction is not counted. For example, an LDR that takes two cycles to complete increments this counter one cycle. Equivalently, an LDR that stalls for two cycles (i.e. takes four cycles to execute), increments this counter three times. An event is emitted on counter overflow (every 256 cycles). This counter initializes to 0 when it is enabled using CTRL.LSUEVTENA."]
    #[inline(always)]
    #[must_use]
    pub fn lsucnt(&mut self) -> LSUCNT_W<0> {
        LSUCNT_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LSU Count This register is used to count the total number of cycles during which the processor is processing an LSU operation beyond the first cycle.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsucnt](index.html) module"]
pub struct LSUCNT_SPEC;
impl crate::RegisterSpec for LSUCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsucnt::R](R) reader structure"]
impl crate::Readable for LSUCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsucnt::W](W) writer structure"]
impl crate::Writable for LSUCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LSUCNT to value 0"]
impl crate::Resettable for LSUCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
