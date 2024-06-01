#[doc = "Register `EFUSEREAD` reader"]
pub type R = crate::R<EfusereadSpec>;
#[doc = "Register `EFUSEREAD` writer"]
pub type W = crate::W<EfusereadSpec>;
#[doc = "Field `MARGIN` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type MarginR = crate::FieldReader;
#[doc = "Field `MARGIN` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type MarginW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPARE` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type SpareR = crate::BitReader;
#[doc = "Field `SPARE` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type SpareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBUG` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type DebugR = crate::BitReader;
#[doc = "Field `DEBUG` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type DebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READCLOCK` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type ReadclockR = crate::FieldReader;
#[doc = "Field `READCLOCK` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type ReadclockW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATABIT` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type DatabitR = crate::FieldReader;
#[doc = "Field `DATABIT` writer - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type DatabitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED10` reader - 31:10\\]
Internal. Only to be used through TI provided API."]
pub type Reserved10R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED10` writer - 31:10\\]
Internal. Only to be used through TI provided API."]
pub type Reserved10W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn margin(&self) -> MarginR {
        MarginR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn debug(&self) -> DebugR {
        DebugR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn readclock(&self) -> ReadclockR {
        ReadclockR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn databit(&self) -> DatabitR {
        DatabitR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn margin(&mut self) -> MarginW<EfusereadSpec> {
        MarginW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<EfusereadSpec> {
        SpareW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn debug(&mut self) -> DebugW<EfusereadSpec> {
        DebugW::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn readclock(&mut self) -> ReadclockW<EfusereadSpec> {
        ReadclockW::new(self, 4)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn databit(&mut self) -> DatabitW<EfusereadSpec> {
        DatabitW::new(self, 8)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<EfusereadSpec> {
        Reserved10W::new(self, 10)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuseread::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuseread::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfusereadSpec;
impl crate::RegisterSpec for EfusereadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuseread::R`](R) reader structure"]
impl crate::Readable for EfusereadSpec {}
#[doc = "`write(|w| ..)` method takes [`efuseread::W`](W) writer structure"]
impl crate::Writable for EfusereadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSEREAD to value 0"]
impl crate::Resettable for EfusereadSpec {
    const RESET_VALUE: u32 = 0;
}
