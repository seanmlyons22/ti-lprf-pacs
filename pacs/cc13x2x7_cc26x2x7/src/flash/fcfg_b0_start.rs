#[doc = "Register `FCFG_B0_START` reader"]
pub type R = crate::R<FcfgB0StartSpec>;
#[doc = "Register `FCFG_B0_START` writer"]
pub type W = crate::W<FcfgB0StartSpec>;
#[doc = "Field `B0_START_ADDR` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type B0StartAddrR = crate::FieldReader<u32>;
#[doc = "Field `B0_START_ADDR` writer - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type B0StartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `B0_MUX_FACTOR` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type B0MuxFactorR = crate::FieldReader;
#[doc = "Field `B0_MUX_FACTOR` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type B0MuxFactorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `B0_MAX_SECTOR` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type B0MaxSectorR = crate::FieldReader;
#[doc = "Field `B0_MAX_SECTOR` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type B0MaxSectorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_start_addr(&self) -> B0StartAddrR {
        B0StartAddrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_mux_factor(&self) -> B0MuxFactorR {
        B0MuxFactorR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_max_sector(&self) -> B0MaxSectorR {
        B0MaxSectorR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b0_start_addr(&mut self) -> B0StartAddrW<FcfgB0StartSpec> {
        B0StartAddrW::new(self, 0)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b0_mux_factor(&mut self) -> B0MuxFactorW<FcfgB0StartSpec> {
        B0MuxFactorW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b0_max_sector(&mut self) -> B0MaxSectorW<FcfgB0StartSpec> {
        B0MaxSectorW::new(self, 28)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b0_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b0_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcfgB0StartSpec;
impl crate::RegisterSpec for FcfgB0StartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg_b0_start::R`](R) reader structure"]
impl crate::Readable for FcfgB0StartSpec {}
#[doc = "`write(|w| ..)` method takes [`fcfg_b0_start::W`](W) writer structure"]
impl crate::Writable for FcfgB0StartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFG_B0_START to value 0x0200_0000"]
impl crate::Resettable for FcfgB0StartSpec {
    const RESET_VALUE: u32 = 0x0200_0000;
}
