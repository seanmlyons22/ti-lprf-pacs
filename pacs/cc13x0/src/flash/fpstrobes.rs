#[doc = "Register `FPSTROBES` reader"]
pub type R = crate::R<FpstrobesSpec>;
#[doc = "Register `FPSTROBES` writer"]
pub type W = crate::W<FpstrobesSpec>;
#[doc = "Field `V5PWRDNZ` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type V5pwrdnzR = crate::BitReader;
#[doc = "Field `V5PWRDNZ` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type V5pwrdnzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `V3PWRDNZ` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type V3pwrdnzR = crate::BitReader;
#[doc = "Field `V3PWRDNZ` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type V3pwrdnzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `EXECUTEZ` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type ExecutezR = crate::BitReader;
#[doc = "Field `EXECUTEZ` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type ExecutezW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v5pwrdnz(&self) -> V5pwrdnzR {
        V5pwrdnzR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v3pwrdnz(&self) -> V3pwrdnzR {
        V3pwrdnzR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn executez(&self) -> ExecutezR {
        ExecutezR::new(((self.bits >> 8) & 1) != 0)
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
    pub fn v5pwrdnz(&mut self) -> V5pwrdnzW<FpstrobesSpec> {
        V5pwrdnzW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn v3pwrdnz(&mut self) -> V3pwrdnzW<FpstrobesSpec> {
        V3pwrdnzW::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn executez(&mut self) -> ExecutezW<FpstrobesSpec> {
        ExecutezW::new(self, 8)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpstrobes::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpstrobes::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpstrobesSpec;
impl crate::RegisterSpec for FpstrobesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpstrobes::R`](R) reader structure"]
impl crate::Readable for FpstrobesSpec {}
#[doc = "`write(|w| ..)` method takes [`fpstrobes::W`](W) writer structure"]
impl crate::Writable for FpstrobesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPSTROBES to value 0x0103"]
impl crate::Resettable for FpstrobesSpec {
    const RESET_VALUE: u32 = 0x0103;
}
