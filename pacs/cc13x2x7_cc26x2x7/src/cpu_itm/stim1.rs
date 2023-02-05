#[doc = "Register `STIM1` reader"]
pub struct R(crate::R<STIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STIM1` writer"]
pub struct W(crate::W<STIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STIM1_SPEC>;
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
impl From<crate::W<STIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STIM1` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA1 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type STIM1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STIM1` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA1 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type STIM1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STIM1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA1 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim1(&self) -> STIM1_R {
        STIM1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA1 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    #[must_use]
    pub fn stim1(&mut self) -> STIM1_W<0> {
        STIM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stimulus Port 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim1](index.html) module"]
pub struct STIM1_SPEC;
impl crate::RegisterSpec for STIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stim1::R](R) reader structure"]
impl crate::Readable for STIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stim1::W](W) writer structure"]
impl crate::Writable for STIM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STIM1 to value 0"]
impl crate::Resettable for STIM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
