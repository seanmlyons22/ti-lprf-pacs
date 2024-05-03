#[doc = "Register `EFUSECFG` reader"]
pub type R = crate::R<EfusecfgSpec>;
#[doc = "Register `EFUSECFG` writer"]
pub type W = crate::W<EfusecfgSpec>;
#[doc = "Field `GATING` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type GatingR = crate::BitReader;
#[doc = "Field `GATING` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type GatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 2:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 2:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SLAVEPOWER` reader - 4:3\\]
Internal. Only to be used through TI provided API."]
pub type SlavepowerR = crate::FieldReader;
#[doc = "Field `SLAVEPOWER` writer - 4:3\\]
Internal. Only to be used through TI provided API."]
pub type SlavepowerW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED5` reader - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `RESERVED5` writer - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IDLEGATING` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type IdlegatingR = crate::BitReader;
#[doc = "Field `IDLEGATING` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type IdlegatingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn gating(&self) -> GatingR {
        GatingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn slavepower(&self) -> SlavepowerR {
        SlavepowerR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn idlegating(&self) -> IdlegatingR {
        IdlegatingR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn gating(&mut self) -> GatingW<EfusecfgSpec> {
        GatingW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<EfusecfgSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn slavepower(&mut self) -> SlavepowerW<EfusecfgSpec> {
        SlavepowerW::new(self, 3)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<EfusecfgSpec> {
        Reserved5W::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn idlegating(&mut self) -> IdlegatingW<EfusecfgSpec> {
        IdlegatingW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<EfusecfgSpec> {
        Reserved9W::new(self, 9)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efusecfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efusecfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfusecfgSpec;
impl crate::RegisterSpec for EfusecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efusecfg::R`](R) reader structure"]
impl crate::Readable for EfusecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`efusecfg::W`](W) writer structure"]
impl crate::Writable for EfusecfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSECFG to value 0x01"]
impl crate::Resettable for EfusecfgSpec {
    const RESET_VALUE: u32 = 0x01;
}
