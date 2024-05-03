#[doc = "Register `EFUSEPROGRAM` reader"]
pub type R = crate::R<EfuseprogramSpec>;
#[doc = "Register `EFUSEPROGRAM` writer"]
pub type W = crate::W<EfuseprogramSpec>;
#[doc = "Field `WRITECLOCK` reader - 8:0\\]
Internal. Only to be used through TI provided API."]
pub type WriteclockR = crate::FieldReader<u16>;
#[doc = "Field `WRITECLOCK` writer - 8:0\\]
Internal. Only to be used through TI provided API."]
pub type WriteclockW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ITERATIONS` reader - 12:9\\]
Internal. Only to be used through TI provided API."]
pub type IterationsR = crate::FieldReader;
#[doc = "Field `ITERATIONS` writer - 12:9\\]
Internal. Only to be used through TI provided API."]
pub type IterationsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VPPTOVDD` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type VpptovddR = crate::BitReader;
#[doc = "Field `VPPTOVDD` writer - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type VpptovddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLOCKSTALL` reader - 29:14\\]
Internal. Only to be used through TI provided API."]
pub type ClockstallR = crate::FieldReader<u16>;
#[doc = "Field `CLOCKSTALL` writer - 29:14\\]
Internal. Only to be used through TI provided API."]
pub type ClockstallW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `COMPAREDISABLE` reader - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type ComparedisableR = crate::BitReader;
#[doc = "Field `COMPAREDISABLE` writer - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type ComparedisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED31` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type Reserved31R = crate::BitReader;
#[doc = "Field `RESERVED31` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type Reserved31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn writeclock(&self) -> WriteclockR {
        WriteclockR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:12 - 12:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iterations(&self) -> IterationsR {
        IterationsR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vpptovdd(&self) -> VpptovddR {
        VpptovddR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:29 - 29:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn clockstall(&self) -> ClockstallR {
        ClockstallR::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn comparedisable(&self) -> ComparedisableR {
        ComparedisableR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn writeclock(&mut self) -> WriteclockW<EfuseprogramSpec> {
        WriteclockW::new(self, 0)
    }
    #[doc = "Bits 9:12 - 12:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn iterations(&mut self) -> IterationsW<EfuseprogramSpec> {
        IterationsW::new(self, 9)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vpptovdd(&mut self) -> VpptovddW<EfuseprogramSpec> {
        VpptovddW::new(self, 13)
    }
    #[doc = "Bits 14:29 - 29:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn clockstall(&mut self) -> ClockstallW<EfuseprogramSpec> {
        ClockstallW::new(self, 14)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn comparedisable(&mut self) -> ComparedisableW<EfuseprogramSpec> {
        ComparedisableW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved31(&mut self) -> Reserved31W<EfuseprogramSpec> {
        Reserved31W::new(self, 31)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuseprogram::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuseprogram::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfuseprogramSpec;
impl crate::RegisterSpec for EfuseprogramSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuseprogram::R`](R) reader structure"]
impl crate::Readable for EfuseprogramSpec {}
#[doc = "`write(|w| ..)` method takes [`efuseprogram::W`](W) writer structure"]
impl crate::Writable for EfuseprogramSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSEPROGRAM to value 0"]
impl crate::Resettable for EfuseprogramSpec {
    const RESET_VALUE: u32 = 0;
}
