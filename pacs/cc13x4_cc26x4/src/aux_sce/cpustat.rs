#[doc = "Register `CPUSTAT` reader"]
pub type R = crate::R<CpustatSpec>;
#[doc = "Register `CPUSTAT` writer"]
pub type W = crate::W<CpustatSpec>;
#[doc = "Field `Z_FLAG` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type ZFlagR = crate::BitReader;
#[doc = "Field `Z_FLAG` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type ZFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `N_FLAG` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type NFlagR = crate::BitReader;
#[doc = "Field `N_FLAG` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type NFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C_FLAG` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type CFlagR = crate::BitReader;
#[doc = "Field `C_FLAG` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type CFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `V_FLAG` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type VFlagR = crate::BitReader;
#[doc = "Field `V_FLAG` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type VFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `RESERVED4` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HALTED` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type HaltedR = crate::BitReader;
#[doc = "Field `HALTED` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type HaltedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEV` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type WevR = crate::BitReader;
#[doc = "Field `WEV` writer - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type WevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type SleepR = crate::BitReader;
#[doc = "Field `SLEEP` writer - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_ERROR` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type BusErrorR = crate::BitReader;
#[doc = "Field `BUS_ERROR` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type BusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn z_flag(&self) -> ZFlagR {
        ZFlagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn n_flag(&self) -> NFlagR {
        NFlagR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn c_flag(&self) -> CFlagR {
        CFlagR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v_flag(&self) -> VFlagR {
        VFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn halted(&self) -> HaltedR {
        HaltedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wev(&self) -> WevR {
        WevR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bus_error(&self) -> BusErrorR {
        BusErrorR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn z_flag(&mut self) -> ZFlagW<CpustatSpec> {
        ZFlagW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn n_flag(&mut self) -> NFlagW<CpustatSpec> {
        NFlagW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn c_flag(&mut self) -> CFlagW<CpustatSpec> {
        CFlagW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn v_flag(&mut self) -> VFlagW<CpustatSpec> {
        VFlagW::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<CpustatSpec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn halted(&mut self) -> HaltedW<CpustatSpec> {
        HaltedW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wev(&mut self) -> WevW<CpustatSpec> {
        WevW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SleepW<CpustatSpec> {
        SleepW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bus_error(&mut self) -> BusErrorW<CpustatSpec> {
        BusErrorW::new(self, 11)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<CpustatSpec> {
        Reserved12W::new(self, 12)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpustat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpustat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpustatSpec;
impl crate::RegisterSpec for CpustatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpustat::R`](R) reader structure"]
impl crate::Readable for CpustatSpec {}
#[doc = "`write(|w| ..)` method takes [`cpustat::W`](W) writer structure"]
impl crate::Writable for CpustatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUSTAT to value 0"]
impl crate::Resettable for CpustatSpec {
    const RESET_VALUE: u32 = 0;
}
