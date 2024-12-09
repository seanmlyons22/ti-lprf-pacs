#[doc = "Register `FCFG_B4_START` reader"]
pub type R = crate::R<FcfgB4StartSpec>;
#[doc = "Register `FCFG_B4_START` writer"]
pub type W = crate::W<FcfgB4StartSpec>;
#[doc = "Field `B4_START_ADDR` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type B4StartAddrR = crate::FieldReader<u32>;
#[doc = "Field `B4_MUX_FACTOR` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type B4MuxFactorR = crate::FieldReader;
#[doc = "Field `B4_MAX_SECTOR` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type B4MaxSectorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b4_start_addr(&self) -> B4StartAddrR {
        B4StartAddrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b4_mux_factor(&self) -> B4MuxFactorR {
        B4MuxFactorR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b4_max_sector(&self) -> B4MaxSectorR {
        B4MaxSectorR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b4_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b4_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcfgB4StartSpec;
impl crate::RegisterSpec for FcfgB4StartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg_b4_start::R`](R) reader structure"]
impl crate::Readable for FcfgB4StartSpec {}
#[doc = "`write(|w| ..)` method takes [`fcfg_b4_start::W`](W) writer structure"]
impl crate::Writable for FcfgB4StartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFG_B4_START to value 0"]
impl crate::Resettable for FcfgB4StartSpec {
    const RESET_VALUE: u32 = 0;
}
