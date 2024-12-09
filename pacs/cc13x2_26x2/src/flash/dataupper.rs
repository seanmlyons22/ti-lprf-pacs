#[doc = "Register `DATAUPPER` reader"]
pub type R = crate::R<DataupperSpec>;
#[doc = "Register `DATAUPPER` writer"]
pub type W = crate::W<DataupperSpec>;
#[doc = "Field `EEN` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type EenR = crate::BitReader;
#[doc = "Field `EEN` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type EenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type RR = crate::BitReader;
#[doc = "Field `R` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type RW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type PR = crate::BitReader;
#[doc = "Field `P` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type PW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE` reader - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type SpareR = crate::FieldReader;
#[doc = "Field `SPARE` writer - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type SpareW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Internal. Only to be used through TI provided API."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn een(&self) -> EenR {
        EenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn r(&self) -> RR {
        RR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn een(&mut self) -> EenW<DataupperSpec> {
        EenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn r(&mut self) -> RW<DataupperSpec> {
        RW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> PW<DataupperSpec> {
        PW::new(self, 2)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<DataupperSpec> {
        SpareW::new(self, 3)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dataupper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dataupper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataupperSpec;
impl crate::RegisterSpec for DataupperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dataupper::R`](R) reader structure"]
impl crate::Readable for DataupperSpec {}
#[doc = "`write(|w| ..)` method takes [`dataupper::W`](W) writer structure"]
impl crate::Writable for DataupperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATAUPPER to value 0"]
impl crate::Resettable for DataupperSpec {
    const RESET_VALUE: u32 = 0;
}
