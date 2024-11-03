#[doc = "Register `DESCEX` reader"]
pub type R = crate::R<DescexSpec>;
#[doc = "Register `DESCEX` writer"]
pub type W = crate::W<DescexSpec>;
#[doc = "Field `NPUB` reader - 7:0\\]
Number of Publishers"]
pub type NpubR = crate::FieldReader;
#[doc = "Field `NPUB` writer - 7:0\\]
Number of Publishers"]
pub type NpubW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NSUB` reader - 15:8\\]
Number of Subscribers"]
pub type NsubR = crate::FieldReader;
#[doc = "Field `NSUB` writer - 15:8\\]
Number of Subscribers"]
pub type NsubW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PD` reader - 16:16\\]
Power Domain. 0 : SVT 1 : ULL"]
pub type PdR = crate::BitReader;
#[doc = "Field `PD` writer - 16:16\\]
Power Domain. 0 : SVT 1 : ULL"]
pub type PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDMA` reader - 21:17\\]
Number of DMA output channels"]
pub type NdmaR = crate::FieldReader;
#[doc = "Field `NDMA` writer - 21:17\\]
Number of DMA output channels"]
pub type NdmaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IDMA` reader - 31:22\\]
Nember of DMA input channels"]
pub type IdmaR = crate::FieldReader<u16>;
#[doc = "Field `IDMA` writer - 31:22\\]
Nember of DMA input channels"]
pub type IdmaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Number of Publishers"]
    #[inline(always)]
    pub fn npub(&self) -> NpubR {
        NpubR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of Subscribers"]
    #[inline(always)]
    pub fn nsub(&self) -> NsubR {
        NsubR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Power Domain. 0 : SVT 1 : ULL"]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21 - 21:17\\]
Number of DMA output channels"]
    #[inline(always)]
    pub fn ndma(&self) -> NdmaR {
        NdmaR::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Nember of DMA input channels"]
    #[inline(always)]
    pub fn idma(&self) -> IdmaR {
        IdmaR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Number of Publishers"]
    #[inline(always)]
    #[must_use]
    pub fn npub(&mut self) -> NpubW<DescexSpec> {
        NpubW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of Subscribers"]
    #[inline(always)]
    #[must_use]
    pub fn nsub(&mut self) -> NsubW<DescexSpec> {
        NsubW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Power Domain. 0 : SVT 1 : ULL"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PdW<DescexSpec> {
        PdW::new(self, 16)
    }
    #[doc = "Bits 17:21 - 21:17\\]
Number of DMA output channels"]
    #[inline(always)]
    #[must_use]
    pub fn ndma(&mut self) -> NdmaW<DescexSpec> {
        NdmaW::new(self, 17)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Nember of DMA input channels"]
    #[inline(always)]
    #[must_use]
    pub fn idma(&mut self) -> IdmaW<DescexSpec> {
        IdmaW::new(self, 22)
    }
}
#[doc = "Extended Description Register. This register provides configuration details of the IP to software drivers and end users.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescexSpec;
impl crate::RegisterSpec for DescexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descex::R`](R) reader structure"]
impl crate::Readable for DescexSpec {}
#[doc = "`write(|w| ..)` method takes [`descex::W`](W) writer structure"]
impl crate::Writable for DescexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESCEX to value 0x0218_2d31"]
impl crate::Resettable for DescexSpec {
    const RESET_VALUE: u32 = 0x0218_2d31;
}
