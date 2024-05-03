#[doc = "Register `FSPRD` reader"]
pub type R = crate::R<FsprdSpec>;
#[doc = "Register `FSPRD` writer"]
pub type W = crate::W<FsprdSpec>;
#[doc = "Field `RM0` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Rm0R = crate::BitReader;
#[doc = "Field `RM0` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Rm0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RM1` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type Rm1R = crate::BitReader;
#[doc = "Field `RM1` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type Rm1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RMBSEM` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type RmbsemR = crate::FieldReader;
#[doc = "Field `RMBSEM` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type RmbsemW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DIS_PREEMPT` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type DisPreemptR = crate::FieldReader<u16>;
#[doc = "Field `DIS_PREEMPT` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type DisPreemptW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm0(&self) -> Rm0R {
        Rm0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm1(&self) -> Rm1R {
        Rm1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rmbsem(&self) -> RmbsemR {
        RmbsemR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dis_preempt(&self) -> DisPreemptR {
        DisPreemptR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rm0(&mut self) -> Rm0W<FsprdSpec> {
        Rm0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rm1(&mut self) -> Rm1W<FsprdSpec> {
        Rm1W::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<FsprdSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rmbsem(&mut self) -> RmbsemW<FsprdSpec> {
        RmbsemW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dis_preempt(&mut self) -> DisPreemptW<FsprdSpec> {
        DisPreemptW::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsprd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsprd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsprdSpec;
impl crate::RegisterSpec for FsprdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsprd::R`](R) reader structure"]
impl crate::Readable for FsprdSpec {}
#[doc = "`write(|w| ..)` method takes [`fsprd::W`](W) writer structure"]
impl crate::Writable for FsprdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSPRD to value 0"]
impl crate::Resettable for FsprdSpec {
    const RESET_VALUE: u32 = 0;
}
