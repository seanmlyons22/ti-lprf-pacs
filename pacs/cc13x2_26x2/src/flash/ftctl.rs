#[doc = "Register `FTCTL` reader"]
pub type R = crate::R<FtctlSpec>;
#[doc = "Register `FTCTL` writer"]
pub type W = crate::W<FtctlSpec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST_EN` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type TestEnR = crate::BitReader;
#[doc = "Field `TEST_EN` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type TestEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 15:2\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED2` writer - 15:2\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `WDATA_BLK_CLR` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type WdataBlkClrR = crate::BitReader;
#[doc = "Field `WDATA_BLK_CLR` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type WdataBlkClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Internal. Only to be used through TI provided API."]
pub type Reserved17R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED17` writer - 31:17\\]
Internal. Only to be used through TI provided API."]
pub type Reserved17W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn test_en(&self) -> TestEnR {
        TestEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wdata_blk_clr(&self) -> WdataBlkClrR {
        WdataBlkClrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<FtctlSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn test_en(&mut self) -> TestEnW<FtctlSpec> {
        TestEnW::new(self, 1)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<FtctlSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wdata_blk_clr(&mut self) -> WdataBlkClrW<FtctlSpec> {
        WdataBlkClrW::new(self, 16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<FtctlSpec> {
        Reserved17W::new(self, 17)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtctlSpec;
impl crate::RegisterSpec for FtctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftctl::R`](R) reader structure"]
impl crate::Readable for FtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ftctl::W`](W) writer structure"]
impl crate::Writable for FtctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FTCTL to value 0"]
impl crate::Resettable for FtctlSpec {
    const RESET_VALUE: u32 = 0;
}
