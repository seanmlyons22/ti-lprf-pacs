#[doc = "Register `FCFG_B1_START` reader"]
pub type R = crate::R<FcfgB1StartSpec>;
#[doc = "Register `FCFG_B1_START` writer"]
pub type W = crate::W<FcfgB1StartSpec>;
#[doc = "Field `B1_START_ADDR` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type B1StartAddrR = crate::FieldReader<u32>;
#[doc = "Field `B1_START_ADDR` writer - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type B1StartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `B1_MUX_FACTOR` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type B1MuxFactorR = crate::FieldReader;
#[doc = "Field `B1_MUX_FACTOR` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type B1MuxFactorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `B1_MAX_SECTOR` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type B1MaxSectorR = crate::FieldReader;
#[doc = "Field `B1_MAX_SECTOR` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type B1MaxSectorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b1_start_addr(&self) -> B1StartAddrR {
        B1StartAddrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b1_mux_factor(&self) -> B1MuxFactorR {
        B1MuxFactorR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b1_max_sector(&self) -> B1MaxSectorR {
        B1MaxSectorR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b1_start_addr(&mut self) -> B1StartAddrW<FcfgB1StartSpec> {
        B1StartAddrW::new(self, 0)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b1_mux_factor(&mut self) -> B1MuxFactorW<FcfgB1StartSpec> {
        B1MuxFactorW::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b1_max_sector(&mut self) -> B1MaxSectorW<FcfgB1StartSpec> {
        B1MaxSectorW::new(self, 28)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b1_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b1_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcfgB1StartSpec;
impl crate::RegisterSpec for FcfgB1StartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg_b1_start::R`](R) reader structure"]
impl crate::Readable for FcfgB1StartSpec {}
#[doc = "`write(|w| ..)` method takes [`fcfg_b1_start::W`](W) writer structure"]
impl crate::Writable for FcfgB1StartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFG_B1_START to value 0x0205_8000"]
impl crate::Resettable for FcfgB1StartSpec {
    const RESET_VALUE: u32 = 0x0205_8000;
}
