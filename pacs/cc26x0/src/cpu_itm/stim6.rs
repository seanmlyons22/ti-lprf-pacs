#[doc = "Register `STIM6` reader"]
pub type R = crate::R<Stim6Spec>;
#[doc = "Register `STIM6` writer"]
pub type W = crate::W<Stim6Spec>;
#[doc = "Field `STIM6` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA6 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim6R = crate::FieldReader<u32>;
#[doc = "Field `STIM6` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA6 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA6 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim6(&self) -> Stim6R {
        Stim6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA6 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    #[must_use]
    pub fn stim6(&mut self) -> Stim6W<Stim6Spec> {
        Stim6W::new(self, 0)
    }
}
#[doc = "Stimulus Port 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stim6Spec;
impl crate::RegisterSpec for Stim6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stim6::R`](R) reader structure"]
impl crate::Readable for Stim6Spec {}
#[doc = "`write(|w| ..)` method takes [`stim6::W`](W) writer structure"]
impl crate::Writable for Stim6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STIM6 to value 0"]
impl crate::Resettable for Stim6Spec {
    const RESET_VALUE: u32 = 0;
}
