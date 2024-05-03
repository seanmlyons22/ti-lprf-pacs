#[doc = "Register `FVNVCT` reader"]
pub type R = crate::R<FvnvctSpec>;
#[doc = "Register `FVNVCT` writer"]
pub type W = crate::W<FvnvctSpec>;
#[doc = "Field `VIN_CT` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type VinCtR = crate::FieldReader;
#[doc = "Field `VIN_CT` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type VinCtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED5` reader - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `RESERVED5` writer - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VCG2P5CT` reader - 12:8\\]
Internal. Only to be used through TI provided API."]
pub type Vcg2p5ctR = crate::FieldReader;
#[doc = "Field `VCG2P5CT` writer - 12:8\\]
Internal. Only to be used through TI provided API."]
pub type Vcg2p5ctW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED13` reader - 31:13\\]
Internal. Only to be used through TI provided API."]
pub type Reserved13R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED13` writer - 31:13\\]
Internal. Only to be used through TI provided API."]
pub type Reserved13W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_ct(&self) -> VinCtR {
        VinCtR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vcg2p5ct(&self) -> Vcg2p5ctR {
        Vcg2p5ctR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vin_ct(&mut self) -> VinCtW<FvnvctSpec> {
        VinCtW::new(self, 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<FvnvctSpec> {
        Reserved5W::new(self, 5)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vcg2p5ct(&mut self) -> Vcg2p5ctW<FvnvctSpec> {
        Vcg2p5ctW::new(self, 8)
    }
    #[doc = "Bits 13:31 - 31:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> Reserved13W<FvnvctSpec> {
        Reserved13W::new(self, 13)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fvnvct::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fvnvct::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FvnvctSpec;
impl crate::RegisterSpec for FvnvctSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fvnvct::R`](R) reader structure"]
impl crate::Readable for FvnvctSpec {}
#[doc = "`write(|w| ..)` method takes [`fvnvct::W`](W) writer structure"]
impl crate::Writable for FvnvctSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FVNVCT to value 0x0800"]
impl crate::Resettable for FvnvctSpec {
    const RESET_VALUE: u32 = 0x0800;
}
