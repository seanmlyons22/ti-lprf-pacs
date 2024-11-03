#[doc = "Register `TIMMUTE1` reader"]
pub type R = crate::R<Timmute1Spec>;
#[doc = "Register `TIMMUTE1` writer"]
pub type W = crate::W<Timmute1Spec>;
#[doc = "Field `VDDR` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type VddrR = crate::FieldReader;
#[doc = "Field `VDDR` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type VddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VDDRSL` reader - 9:5\\]
Internal. Only to be used through TI provided API."]
pub type VddrslR = crate::FieldReader;
#[doc = "Field `VDDRSL` writer - 9:5\\]
Internal. Only to be used through TI provided API."]
pub type VddrslW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IPTAT` reader - 11:10\\]
Internal. Only to be used through TI provided API."]
pub type IptatR = crate::FieldReader;
#[doc = "Field `IPTAT` writer - 11:10\\]
Internal. Only to be used through TI provided API."]
pub type IptatW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BOD` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type BodR = crate::FieldReader;
#[doc = "Field `BOD` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type BodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UDIG` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type UdigR = crate::FieldReader;
#[doc = "Field `UDIG` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type UdigW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIG` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type DigR = crate::FieldReader;
#[doc = "Field `DIG` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type DigW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COARSE` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type CoarseR = crate::FieldReader;
#[doc = "Field `COARSE` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type CoarseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DELTA` reader - 30:28\\]
Internal. Only to be used through TI provided API."]
pub type DeltaR = crate::FieldReader;
#[doc = "Field `DELTA` writer - 30:28\\]
Internal. Only to be used through TI provided API."]
pub type DeltaW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED31` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type Reserved31R = crate::BitReader;
#[doc = "Field `RESERVED31` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type Reserved31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddr(&self) -> VddrR {
        VddrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vddrsl(&self) -> VddrslR {
        VddrslR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iptat(&self) -> IptatR {
        IptatR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bod(&self) -> BodR {
        BodR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn udig(&self) -> UdigR {
        UdigR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dig(&self) -> DigR {
        DigR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn coarse(&self) -> CoarseR {
        CoarseR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn delta(&self) -> DeltaR {
        DeltaR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddr(&mut self) -> VddrW<Timmute1Spec> {
        VddrW::new(self, 0)
    }
    #[doc = "Bits 5:9 - 9:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vddrsl(&mut self) -> VddrslW<Timmute1Spec> {
        VddrslW::new(self, 5)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn iptat(&mut self) -> IptatW<Timmute1Spec> {
        IptatW::new(self, 10)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bod(&mut self) -> BodW<Timmute1Spec> {
        BodW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn udig(&mut self) -> UdigW<Timmute1Spec> {
        UdigW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dig(&mut self) -> DigW<Timmute1Spec> {
        DigW::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn coarse(&mut self) -> CoarseW<Timmute1Spec> {
        CoarseW::new(self, 24)
    }
    #[doc = "Bits 28:30 - 30:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn delta(&mut self) -> DeltaW<Timmute1Spec> {
        DeltaW::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved31(&mut self) -> Reserved31W<Timmute1Spec> {
        Reserved31W::new(self, 31)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timmute1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timmute1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timmute1Spec;
impl crate::RegisterSpec for Timmute1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timmute1::R`](R) reader structure"]
impl crate::Readable for Timmute1Spec {}
#[doc = "`write(|w| ..)` method takes [`timmute1::W`](W) writer structure"]
impl crate::Writable for Timmute1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMMUTE1 to value 0"]
impl crate::Resettable for Timmute1Spec {
    const RESET_VALUE: u32 = 0;
}
