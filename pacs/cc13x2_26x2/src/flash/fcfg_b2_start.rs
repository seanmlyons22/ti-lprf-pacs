#[doc = "Register `FCFG_B2_START` reader"]
pub type R = crate::R<FcfgB2StartSpec>;
#[doc = "Register `FCFG_B2_START` writer"]
pub type W = crate::W<FcfgB2StartSpec>;
#[doc = "Field `B2_START_ADDR` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type B2StartAddrR = crate::FieldReader<u32>;
#[doc = "Field `B2_START_ADDR` writer - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type B2StartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `B2_MUX_FACTOR` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type B2MuxFactorR = crate::FieldReader;
#[doc = "Field `B2_MUX_FACTOR` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type B2MuxFactorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `B2_MAX_SECTOR` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type B2MaxSectorR = crate::FieldReader;
#[doc = "Field `B2_MAX_SECTOR` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type B2MaxSectorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b2_start_addr(&self) -> B2StartAddrR {
        B2StartAddrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b2_mux_factor(&self) -> B2MuxFactorR {
        B2MuxFactorR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b2_max_sector(&self) -> B2MaxSectorR {
        B2MaxSectorR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b2_start_addr(&mut self) -> B2StartAddrW<FcfgB2StartSpec> {
        B2StartAddrW::new(self, 0)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b2_mux_factor(&mut self) -> B2MuxFactorW<FcfgB2StartSpec> {
        B2MuxFactorW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b2_max_sector(&mut self) -> B2MaxSectorW<FcfgB2StartSpec> {
        B2MaxSectorW::new(self, 28)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b2_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b2_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcfgB2StartSpec;
impl crate::RegisterSpec for FcfgB2StartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg_b2_start::R`](R) reader structure"]
impl crate::Readable for FcfgB2StartSpec {}
#[doc = "`write(|w| ..)` method takes [`fcfg_b2_start::W`](W) writer structure"]
impl crate::Writable for FcfgB2StartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFG_B2_START to value 0"]
impl crate::Resettable for FcfgB2StartSpec {
    const RESET_VALUE: u32 = 0;
}
