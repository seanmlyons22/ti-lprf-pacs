#[doc = "Register `STIM21` reader"]
pub type R = crate::R<Stim21Spec>;
#[doc = "Register `STIM21` writer"]
pub type W = crate::W<Stim21Spec>;
#[doc = "Field `STIM21` reader - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA21 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim21R = crate::FieldReader<u32>;
#[doc = "Field `STIM21` writer - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA21 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
pub type Stim21W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA21 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    pub fn stim21(&self) -> Stim21R {
        Stim21R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write to this location causes data to be written into the FIFO if TER.STIMENA21 is set. Reading from the stimulus port returns the FIFO status in bit \\[0\\]: 0 = full, 1 = not full. The polled FIFO interface does not provide an atomic read-modify-write, so it's users responsibility to ensure exclusive read-modify-write if this ITM port is used concurrently by interrupts or other threads."]
    #[inline(always)]
    #[must_use]
    pub fn stim21(&mut self) -> Stim21W<Stim21Spec> {
        Stim21W::new(self, 0)
    }
}
#[doc = "Stimulus Port 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stim21Spec;
impl crate::RegisterSpec for Stim21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stim21::R`](R) reader structure"]
impl crate::Readable for Stim21Spec {}
#[doc = "`write(|w| ..)` method takes [`stim21::W`](W) writer structure"]
impl crate::Writable for Stim21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STIM21 to value 0"]
impl crate::Resettable for Stim21Spec {
    const RESET_VALUE: u32 = 0;
}
