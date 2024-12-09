#[doc = "Register `SCTL` reader"]
pub type R = crate::R<SctlSpec>;
#[doc = "Register `SCTL` writer"]
pub type W = crate::W<SctlSpec>;
#[doc = "Field `DA` writer - 0:0\\]
Device active 0: Disables the I2C slave operation 1: Enables the I2C slave operation"]
pub type DaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
Device active 0: Disables the I2C slave operation 1: Enables the I2C slave operation"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DaW<SctlSpec> {
        DaW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SctlSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Slave Control Note: This register shares address with SSTAT, meaning that this register functions as a control register when written, and a status register when read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SctlSpec;
impl crate::RegisterSpec for SctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sctl::R`](R) reader structure"]
impl crate::Readable for SctlSpec {}
#[doc = "`write(|w| ..)` method takes [`sctl::W`](W) writer structure"]
impl crate::Writable for SctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCTL to value 0"]
impl crate::Resettable for SctlSpec {
    const RESET_VALUE: u32 = 0;
}
