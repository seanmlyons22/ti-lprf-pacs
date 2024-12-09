#[doc = "Register `ECR` reader"]
pub type R = crate::R<EcrSpec>;
#[doc = "Register `ECR` writer"]
pub type W = crate::W<EcrSpec>;
#[doc = "Field `FE` writer - 0:0\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` writer - 1:1\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` writer - 2:2\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
pub type BeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE` writer - 3:3\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
pub type OeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FeW<EcrSpec> {
        FeW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<EcrSpec> {
        PeW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BeW<EcrSpec> {
        BeW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
The framing (FE), parity (PE), break (BE) and overrun (OE) errors are cleared to 0 by any write to this register."]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OeW<EcrSpec> {
        OeW::new(self, 3)
    }
}
#[doc = "Error Clear This register is mapped to the same address as RSR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcrSpec;
impl crate::RegisterSpec for EcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecr::R`](R) reader structure"]
impl crate::Readable for EcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ecr::W`](W) writer structure"]
impl crate::Writable for EcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for EcrSpec {
    const RESET_VALUE: u32 = 0;
}
