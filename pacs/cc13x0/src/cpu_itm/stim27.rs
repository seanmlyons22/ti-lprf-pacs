#[doc = "Register `STIM27` reader"]
pub type R = crate::R<Stim27Spec>;
#[doc = "Register `STIM27` writer"]
pub type W = crate::W<Stim27Spec>;
#[doc = "Field `STIM27` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA27 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim27R = crate::FieldReader<u32>;
#[doc = "Field `STIM27` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA27 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim27W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA27 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim27(&self) -> Stim27R {
        Stim27R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA27 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    #[must_use]
    pub fn stim27(&mut self) -> Stim27W<Stim27Spec> {
        Stim27W::new(self, 0)
    }
}
#[doc = "Stimulus Port 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stim27Spec;
impl crate::RegisterSpec for Stim27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stim27::R`](R) reader structure"]
impl crate::Readable for Stim27Spec {}
#[doc = "`write(|w| ..)` method takes [`stim27::W`](W) writer structure"]
impl crate::Writable for Stim27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STIM27 to value 0"]
impl crate::Resettable for Stim27Spec {
    const RESET_VALUE: u32 = 0;
}
