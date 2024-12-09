#[doc = "Register `FSEQPMP` reader"]
pub type R = crate::R<FseqpmpSpec>;
#[doc = "Register `FSEQPMP` writer"]
pub type W = crate::W<FseqpmpSpec>;
#[doc = "Field `SEQ_PUMP` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type SeqPumpR = crate::FieldReader;
#[doc = "Field `SEQ_PUMP` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type SeqPumpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VIN_BY_PASS` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type VinByPassR = crate::BitReader;
#[doc = "Field `VIN_BY_PASS` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type VinByPassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 11:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `VIN_AT_X` reader - 14:12\\]
Internal. Only to be used through TI provided API."]
pub type VinAtXR = crate::FieldReader;
#[doc = "Field `VIN_AT_X` writer - 14:12\\]
Internal. Only to be used through TI provided API."]
pub type VinAtXW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED15` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `TRIM_0P8` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type Trim0p8R = crate::FieldReader;
#[doc = "Field `TRIM_0P8` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type Trim0p8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM_1P7` reader - 21:20\\]
Internal. Only to be used through TI provided API."]
pub type Trim1p7R = crate::FieldReader;
#[doc = "Field `TRIM_1P7` writer - 21:20\\]
Internal. Only to be used through TI provided API."]
pub type Trim1p7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED22` reader - 23:22\\]
Internal. Only to be used through TI provided API."]
pub type Reserved22R = crate::FieldReader;
#[doc = "Field `TRIM_3P4` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type Trim3p4R = crate::FieldReader;
#[doc = "Field `TRIM_3P4` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type Trim3p4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved28R = crate::FieldReader;
#[doc = "Field `RESERVED28` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved28W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn seq_pump(&self) -> SeqPumpR {
        SeqPumpR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_by_pass(&self) -> VinByPassR {
        VinByPassR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vin_at_x(&self) -> VinAtXR {
        VinAtXR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_0p8(&self) -> Trim0p8R {
        Trim0p8R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_1p7(&self) -> Trim1p7R {
        Trim1p7R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved22(&self) -> Reserved22R {
        Reserved22R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_3p4(&self) -> Trim3p4R {
        Trim3p4R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn seq_pump(&mut self) -> SeqPumpW<FseqpmpSpec> {
        SeqPumpW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vin_by_pass(&mut self) -> VinByPassW<FseqpmpSpec> {
        VinByPassW::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vin_at_x(&mut self) -> VinAtXW<FseqpmpSpec> {
        VinAtXW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim_0p8(&mut self) -> Trim0p8W<FseqpmpSpec> {
        Trim0p8W::new(self, 16)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim_1p7(&mut self) -> Trim1p7W<FseqpmpSpec> {
        Trim1p7W::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim_3p4(&mut self) -> Trim3p4W<FseqpmpSpec> {
        Trim3p4W::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved28(&mut self) -> Reserved28W<FseqpmpSpec> {
        Reserved28W::new(self, 28)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fseqpmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fseqpmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FseqpmpSpec;
impl crate::RegisterSpec for FseqpmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fseqpmp::R`](R) reader structure"]
impl crate::Readable for FseqpmpSpec {}
#[doc = "`write(|w| ..)` method takes [`fseqpmp::W`](W) writer structure"]
impl crate::Writable for FseqpmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSEQPMP to value 0x8508_0000"]
impl crate::Resettable for FseqpmpSpec {
    const RESET_VALUE: u32 = 0x8508_0000;
}
