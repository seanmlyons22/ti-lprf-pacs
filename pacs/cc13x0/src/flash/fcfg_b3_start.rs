#[doc = "Register `FCFG_B3_START` reader"]
pub type R = crate::R<FcfgB3StartSpec>;
#[doc = "Register `FCFG_B3_START` writer"]
pub type W = crate::W<FcfgB3StartSpec>;
#[doc = "Field `B3_START_ADDR` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type B3StartAddrR = crate::FieldReader<u32>;
#[doc = "Field `B3_START_ADDR` writer - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type B3StartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `B3_MUX_FACTOR` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type B3MuxFactorR = crate::FieldReader;
#[doc = "Field `B3_MUX_FACTOR` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type B3MuxFactorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `B3_MAX_SECTOR` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type B3MaxSectorR = crate::FieldReader;
#[doc = "Field `B3_MAX_SECTOR` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type B3MaxSectorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b3_start_addr(&self) -> B3StartAddrR {
        B3StartAddrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b3_mux_factor(&self) -> B3MuxFactorR {
        B3MuxFactorR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b3_max_sector(&self) -> B3MaxSectorR {
        B3MaxSectorR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b3_start_addr(&mut self) -> B3StartAddrW<FcfgB3StartSpec> {
        B3StartAddrW::new(self, 0)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b3_mux_factor(&mut self) -> B3MuxFactorW<FcfgB3StartSpec> {
        B3MuxFactorW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b3_max_sector(&mut self) -> B3MaxSectorW<FcfgB3StartSpec> {
        B3MaxSectorW::new(self, 28)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b3_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b3_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcfgB3StartSpec;
impl crate::RegisterSpec for FcfgB3StartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg_b3_start::R`](R) reader structure"]
impl crate::Readable for FcfgB3StartSpec {}
#[doc = "`write(|w| ..)` method takes [`fcfg_b3_start::W`](W) writer structure"]
impl crate::Writable for FcfgB3StartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFG_B3_START to value 0"]
impl crate::Resettable for FcfgB3StartSpec {
    const RESET_VALUE: u32 = 0;
}
