#[doc = "Register `STIM0` reader"]
pub struct R(crate::R<STIM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STIM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STIM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STIM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STIM0` writer"]
pub struct W(crate::W<STIM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STIM0_SPEC>;
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
impl From<crate::W<STIM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STIM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STIM0` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA0 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type STIM0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STIM0` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA0 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type STIM0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STIM0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA0 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim0(&self) -> STIM0_R {
        STIM0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA0 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    #[must_use]
    pub fn stim0(&mut self) -> STIM0_W<0> {
        STIM0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stimulus Port 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stim0](index.html) module"]
pub struct STIM0_SPEC;
impl crate::RegisterSpec for STIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stim0::R](R) reader structure"]
impl crate::Readable for STIM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stim0::W](W) writer structure"]
impl crate::Writable for STIM0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STIM0 to value 0"]
impl crate::Resettable for STIM0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
