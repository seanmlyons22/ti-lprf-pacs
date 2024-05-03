#[doc = "Register `FPMTCTL` reader"]
pub type R = crate::R<FpmtctlSpec>;
#[doc = "Register `FPMTCTL` writer"]
pub type W = crate::W<FpmtctlSpec>;
#[doc = "Field `ADDR_INCR` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type AddrIncrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR_INCR` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type AddrIncrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn addr_incr(&self) -> AddrIncrR {
        AddrIncrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn addr_incr(&mut self) -> AddrIncrW<FpmtctlSpec> {
        AddrIncrW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpmtctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpmtctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpmtctlSpec;
impl crate::RegisterSpec for FpmtctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpmtctl::R`](R) reader structure"]
impl crate::Readable for FpmtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`fpmtctl::W`](W) writer structure"]
impl crate::Writable for FpmtctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPMTCTL to value 0"]
impl crate::Resettable for FpmtctlSpec {
    const RESET_VALUE: u32 = 0;
}
