#[doc = "Register `STIM8` reader"]
pub type R = crate::R<Stim8Spec>;
#[doc = "Register `STIM8` writer"]
pub type W = crate::W<Stim8Spec>;
#[doc = "Field `STIM8` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA8 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim8R = crate::FieldReader<u32>;
#[doc = "Field `STIM8` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA8 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim8W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA8 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim8(&self) -> Stim8R {
        Stim8R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA8 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    #[must_use]
    pub fn stim8(&mut self) -> Stim8W<Stim8Spec> {
        Stim8W::new(self, 0)
    }
}
#[doc = "Stimulus Port 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stim8Spec;
impl crate::RegisterSpec for Stim8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stim8::R`](R) reader structure"]
impl crate::Readable for Stim8Spec {}
#[doc = "`write(|w| ..)` method takes [`stim8::W`](W) writer structure"]
impl crate::Writable for Stim8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STIM8 to value 0"]
impl crate::Resettable for Stim8Spec {
    const RESET_VALUE: u32 = 0;
}
