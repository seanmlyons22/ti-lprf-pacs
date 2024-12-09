#[doc = "Register `STMPCTL` reader"]
pub type R = crate::R<StmpctlSpec>;
#[doc = "Register `STMPCTL` writer"]
pub type W = crate::W<StmpctlSpec>;
#[doc = "Field `STMP_EN` reader - 0:0\\]
Enables the samplestamp generator. The samplestamp generator must only be enabled after it has been properly configured. When cleared, all samplestamp generator counters and capture values are cleared."]
pub type StmpEnR = crate::BitReader;
#[doc = "Field `STMP_EN` writer - 0:0\\]
Enables the samplestamp generator. The samplestamp generator must only be enabled after it has been properly configured. When cleared, all samplestamp generator counters and capture values are cleared."]
pub type StmpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_RDY` reader - 1:1\\]
Low until the input pins are ready to be started by the samplestamp generator. When started (that is STMPINTRIG equals the WCLK counter) the bit goes back low."]
pub type InRdyR = crate::BitReader;
#[doc = "Field `OUT_RDY` reader - 2:2\\]
Low until the output pins are ready to be started by the samplestamp generator. When started (that is STMPOUTTRIG equals the WCLK counter) the bit goes back low."]
pub type OutRdyR = crate::BitReader;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the samplestamp generator. The samplestamp generator must only be enabled after it has been properly configured. When cleared, all samplestamp generator counters and capture values are cleared."]
    #[inline(always)]
    pub fn stmp_en(&self) -> StmpEnR {
        StmpEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Low until the input pins are ready to be started by the samplestamp generator. When started (that is STMPINTRIG equals the WCLK counter) the bit goes back low."]
    #[inline(always)]
    pub fn in_rdy(&self) -> InRdyR {
        InRdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Low until the output pins are ready to be started by the samplestamp generator. When started (that is STMPOUTTRIG equals the WCLK counter) the bit goes back low."]
    #[inline(always)]
    pub fn out_rdy(&self) -> OutRdyR {
        OutRdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the samplestamp generator. The samplestamp generator must only be enabled after it has been properly configured. When cleared, all samplestamp generator counters and capture values are cleared."]
    #[inline(always)]
    #[must_use]
    pub fn stmp_en(&mut self) -> StmpEnW<StmpctlSpec> {
        StmpEnW::new(self, 0)
    }
}
#[doc = "Samplestamp Generator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StmpctlSpec;
impl crate::RegisterSpec for StmpctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stmpctl::R`](R) reader structure"]
impl crate::Readable for StmpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`stmpctl::W`](W) writer structure"]
impl crate::Writable for StmpctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STMPCTL to value 0"]
impl crate::Resettable for StmpctlSpec {
    const RESET_VALUE: u32 = 0;
}
