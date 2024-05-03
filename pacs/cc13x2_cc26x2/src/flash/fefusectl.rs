#[doc = "Register `FEFUSECTL` reader"]
pub type R = crate::R<FefusectlSpec>;
#[doc = "Register `FEFUSECTL` writer"]
pub type W = crate::W<FefusectlSpec>;
#[doc = "Field `EFUSE_EN` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type EfuseEnR = crate::FieldReader;
#[doc = "Field `EFUSE_EN` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type EfuseEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EF_TEST` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type EfTestR = crate::BitReader;
#[doc = "Field `EF_TEST` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type EfTestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED5` reader - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `RESERVED5` writer - 7:5\\]
Internal. Only to be used through TI provided API."]
pub type Reserved5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EF_CLRZ` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type EfClrzR = crate::BitReader;
#[doc = "Field `EF_CLRZ` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type EfClrzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `RESERVED9` writer - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BP_SEL` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type BpSelR = crate::BitReader;
#[doc = "Field `BP_SEL` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type BpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_EN` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type WriteEnR = crate::BitReader;
#[doc = "Field `WRITE_EN` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type WriteEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED18` reader - 23:18\\]
Internal. Only to be used through TI provided API."]
pub type Reserved18R = crate::FieldReader;
#[doc = "Field `RESERVED18` writer - 23:18\\]
Internal. Only to be used through TI provided API."]
pub type Reserved18W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CHAIN_SEL` reader - 26:24\\]
Internal. Only to be used through TI provided API."]
pub type ChainSelR = crate::FieldReader;
#[doc = "Field `CHAIN_SEL` writer - 26:24\\]
Internal. Only to be used through TI provided API."]
pub type ChainSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED27` reader - 31:27\\]
Internal. Only to be used through TI provided API."]
pub type Reserved27R = crate::FieldReader;
#[doc = "Field `RESERVED27` writer - 31:27\\]
Internal. Only to be used through TI provided API."]
pub type Reserved27W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuse_en(&self) -> EfuseEnR {
        EfuseEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ef_test(&self) -> EfTestR {
        EfTestR::new(((self.bits >> 4) & 1) != 0)
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
    pub fn ef_clrz(&self) -> EfClrzR {
        EfClrzR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bp_sel(&self) -> BpSelR {
        BpSelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn write_en(&self) -> WriteEnR {
        WriteEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn chain_sel(&self) -> ChainSelR {
        ChainSelR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved27(&self) -> Reserved27R {
        Reserved27R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efuse_en(&mut self) -> EfuseEnW<FefusectlSpec> {
        EfuseEnW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ef_test(&mut self) -> EfTestW<FefusectlSpec> {
        EfTestW::new(self, 4)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> Reserved5W<FefusectlSpec> {
        Reserved5W::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ef_clrz(&mut self) -> EfClrzW<FefusectlSpec> {
        EfClrzW::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<FefusectlSpec> {
        Reserved9W::new(self, 9)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bp_sel(&mut self) -> BpSelW<FefusectlSpec> {
        BpSelW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn write_en(&mut self) -> WriteEnW<FefusectlSpec> {
        WriteEnW::new(self, 17)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> Reserved18W<FefusectlSpec> {
        Reserved18W::new(self, 18)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn chain_sel(&mut self) -> ChainSelW<FefusectlSpec> {
        ChainSelW::new(self, 24)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved27(&mut self) -> Reserved27W<FefusectlSpec> {
        Reserved27W::new(self, 27)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fefusectl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fefusectl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FefusectlSpec;
impl crate::RegisterSpec for FefusectlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fefusectl::R`](R) reader structure"]
impl crate::Readable for FefusectlSpec {}
#[doc = "`write(|w| ..)` method takes [`fefusectl::W`](W) writer structure"]
impl crate::Writable for FefusectlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEFUSECTL to value 0x0701_010a"]
impl crate::Resettable for FefusectlSpec {
    const RESET_VALUE: u32 = 0x0701_010a;
}
