#[doc = "Register `FCFG_WRAPPER` reader"]
pub type R = crate::R<FcfgWrapperSpec>;
#[doc = "Register `FCFG_WRAPPER` writer"]
pub type W = crate::W<FcfgWrapperSpec>;
#[doc = "Field `CPU_TYPE1` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type CpuType1R = crate::FieldReader;
#[doc = "Field `CPU_TYPE1` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type CpuType1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UERR` reader - 5:4\\]
Internal. Only to be used through TI provided API."]
pub type UerrR = crate::FieldReader;
#[doc = "Field `UERR` writer - 5:4\\]
Internal. Only to be used through TI provided API."]
pub type UerrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AUTO_SUSP` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type AutoSuspR = crate::FieldReader;
#[doc = "Field `AUTO_SUSP` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type AutoSuspW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECCA` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type EccaR = crate::BitReader;
#[doc = "Field `ECCA` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type EccaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIL3` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type Sil3R = crate::BitReader;
#[doc = "Field `SIL3` writer - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type Sil3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFLUSH` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type IflushR = crate::BitReader;
#[doc = "Field `IFLUSH` writer - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type IflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROM` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type RomR = crate::BitReader;
#[doc = "Field `ROM` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type RomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE_IN_MAIN` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type EeInMainR = crate::FieldReader;
#[doc = "Field `EE_IN_MAIN` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type EeInMainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CPU2` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type Cpu2R = crate::FieldReader;
#[doc = "Field `CPU2` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type Cpu2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MEM_MAP` reader - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type MemMapR = crate::BitReader;
#[doc = "Field `MEM_MAP` writer - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type MemMapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED21` reader - 23:21\\]
Internal. Only to be used through TI provided API."]
pub type Reserved21R = crate::FieldReader;
#[doc = "Field `RESERVED21` writer - 23:21\\]
Internal. Only to be used through TI provided API."]
pub type Reserved21W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FAMILY_TYPE` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type FamilyTypeR = crate::FieldReader;
#[doc = "Field `FAMILY_TYPE` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type FamilyTypeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cpu_type1(&self) -> CpuType1R {
        CpuType1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn uerr(&self) -> UerrR {
        UerrR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn auto_susp(&self) -> AutoSuspR {
        AutoSuspR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ecca(&self) -> EccaR {
        EccaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sil3(&self) -> Sil3R {
        Sil3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn iflush(&self) -> IflushR {
        IflushR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rom(&self) -> RomR {
        RomR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ee_in_main(&self) -> EeInMainR {
        EeInMainR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cpu2(&self) -> Cpu2R {
        Cpu2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mem_map(&self) -> MemMapR {
        MemMapR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn family_type(&self) -> FamilyTypeR {
        FamilyTypeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_type1(&mut self) -> CpuType1W<FcfgWrapperSpec> {
        CpuType1W::new(self, 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn uerr(&mut self) -> UerrW<FcfgWrapperSpec> {
        UerrW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn auto_susp(&mut self) -> AutoSuspW<FcfgWrapperSpec> {
        AutoSuspW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ecca(&mut self) -> EccaW<FcfgWrapperSpec> {
        EccaW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sil3(&mut self) -> Sil3W<FcfgWrapperSpec> {
        Sil3W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn iflush(&mut self) -> IflushW<FcfgWrapperSpec> {
        IflushW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> RomW<FcfgWrapperSpec> {
        RomW::new(self, 11)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ee_in_main(&mut self) -> EeInMainW<FcfgWrapperSpec> {
        EeInMainW::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cpu2(&mut self) -> Cpu2W<FcfgWrapperSpec> {
        Cpu2W::new(self, 16)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn mem_map(&mut self) -> MemMapW<FcfgWrapperSpec> {
        MemMapW::new(self, 20)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> Reserved21W<FcfgWrapperSpec> {
        Reserved21W::new(self, 21)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn family_type(&mut self) -> FamilyTypeW<FcfgWrapperSpec> {
        FamilyTypeW::new(self, 24)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_wrapper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_wrapper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcfgWrapperSpec;
impl crate::RegisterSpec for FcfgWrapperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg_wrapper::R`](R) reader structure"]
impl crate::Readable for FcfgWrapperSpec {}
#[doc = "`write(|w| ..)` method takes [`fcfg_wrapper::W`](W) writer structure"]
impl crate::Writable for FcfgWrapperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFG_WRAPPER to value 0x5000_9007"]
impl crate::Resettable for FcfgWrapperSpec {
    const RESET_VALUE: u32 = 0x5000_9007;
}
