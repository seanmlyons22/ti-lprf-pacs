#[doc = "Register `DMAPROTCTL` reader"]
pub type R = crate::R<DmaprotctlSpec>;
#[doc = "Register `DMAPROTCTL` writer"]
pub type W = crate::W<DmaprotctlSpec>;
#[doc = "Field `PROT_EN` reader - 0:0\\]
Select AHB transfer protection control for DMA transfers using the key store area as destination. 0 : transfers use 'USER' type access. 1 : transfers use 'PRIVILEGED' type access."]
pub type ProtEnR = crate::BitReader;
#[doc = "Field `PROT_EN` writer - 0:0\\]
Select AHB transfer protection control for DMA transfers using the key store area as destination. 0 : transfers use 'USER' type access. 1 : transfers use 'PRIVILEGED' type access."]
pub type ProtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Select AHB transfer protection control for DMA transfers using the key store area as destination. 0 : transfers use 'USER' type access. 1 : transfers use 'PRIVILEGED' type access."]
    #[inline(always)]
    pub fn prot_en(&self) -> ProtEnR {
        ProtEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Select AHB transfer protection control for DMA transfers using the key store area as destination. 0 : transfers use 'USER' type access. 1 : transfers use 'PRIVILEGED' type access."]
    #[inline(always)]
    #[must_use]
    pub fn prot_en(&mut self) -> ProtEnW<DmaprotctlSpec> {
        ProtEnW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<DmaprotctlSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "DMA Protection Control Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaprotctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaprotctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaprotctlSpec;
impl crate::RegisterSpec for DmaprotctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaprotctl::R`](R) reader structure"]
impl crate::Readable for DmaprotctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaprotctl::W`](W) writer structure"]
impl crate::Writable for DmaprotctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAPROTCTL to value 0"]
impl crate::Resettable for DmaprotctlSpec {
    const RESET_VALUE: u32 = 0;
}
