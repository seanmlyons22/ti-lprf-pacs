#[doc = "Register `WKUPMASK` reader"]
pub type R = crate::R<WkupmaskSpec>;
#[doc = "Register `WKUPMASK` writer"]
pub type W = crate::W<WkupmaskSpec>;
#[doc = "Field `RESERVED0` reader - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AON_PMU_COMB` reader - 2:2\\]
Wake-up mask for AON_PMU_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
pub type AonPmuCombR = crate::BitReader;
#[doc = "Field `AON_PMU_COMB` writer - 2:2\\]
Wake-up mask for AON_PMU_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
pub type AonPmuCombW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AON_CKM_COMB` reader - 3:3\\]
Wake-up mask for AON_CKM_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
pub type AonCkmCombR = crate::BitReader;
#[doc = "Field `AON_CKM_COMB` writer - 3:3\\]
Wake-up mask for AON_CKM_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
pub type AonCkmCombW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AON_RTC_COMB` reader - 4:4\\]
Wake-up mask for AON_RTC_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
pub type AonRtcCombR = crate::BitReader;
#[doc = "Field `AON_RTC_COMB` writer - 4:4\\]
Wake-up mask for AON_RTC_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
pub type AonRtcCombW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AON_DBG_COMB` reader - 5:5\\]
Wake-up mask for AON_DBG_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
pub type AonDbgCombR = crate::BitReader;
#[doc = "Field `AON_DBG_COMB` writer - 5:5\\]
Wake-up mask for AON_DBG_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
pub type AonDbgCombW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AON_LPMCMP_IRQ` reader - 6:6\\]
Wake-up mask for AON_LPMCMP_IRQ. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
pub type AonLpmcmpIrqR = crate::BitReader;
#[doc = "Field `AON_LPMCMP_IRQ` writer - 6:6\\]
Wake-up mask for AON_LPMCMP_IRQ. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
pub type AonLpmcmpIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AON_IOC_COMB` reader - 7:7\\]
Wake-up mask for AON_IOC_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
pub type AonIocCombR = crate::BitReader;
#[doc = "Field `AON_IOC_COMB` writer - 7:7\\]
Wake-up mask for AON_IOC_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
pub type AonIocCombW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Wake-up mask for AON_PMU_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
    #[inline(always)]
    pub fn aon_pmu_comb(&self) -> AonPmuCombR {
        AonPmuCombR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Wake-up mask for AON_CKM_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
    #[inline(always)]
    pub fn aon_ckm_comb(&self) -> AonCkmCombR {
        AonCkmCombR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Wake-up mask for AON_RTC_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
    #[inline(always)]
    pub fn aon_rtc_comb(&self) -> AonRtcCombR {
        AonRtcCombR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Wake-up mask for AON_DBG_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
    #[inline(always)]
    pub fn aon_dbg_comb(&self) -> AonDbgCombR {
        AonDbgCombR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Wake-up mask for AON_LPMCMP_IRQ. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
    #[inline(always)]
    pub fn aon_lpmcmp_irq(&self) -> AonLpmcmpIrqR {
        AonLpmcmpIrqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Wake-up mask for AON_IOC_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
    #[inline(always)]
    pub fn aon_ioc_comb(&self) -> AonIocCombR {
        AonIocCombR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<WkupmaskSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Wake-up mask for AON_PMU_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn aon_pmu_comb(&mut self) -> AonPmuCombW<WkupmaskSpec> {
        AonPmuCombW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Wake-up mask for AON_CKM_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn aon_ckm_comb(&mut self) -> AonCkmCombW<WkupmaskSpec> {
        AonCkmCombW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Wake-up mask for AON_RTC_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn aon_rtc_comb(&mut self) -> AonRtcCombW<WkupmaskSpec> {
        AonRtcCombW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Wake-up mask for AON_DBG_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn aon_dbg_comb(&mut self) -> AonDbgCombW<WkupmaskSpec> {
        AonDbgCombW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Wake-up mask for AON_LPMCMP_IRQ. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn aon_lpmcmp_irq(&mut self) -> AonLpmcmpIrqW<WkupmaskSpec> {
        AonLpmcmpIrqW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Wake-up mask for AON_IOC_COMB. 0 - Wakeup Disabled 1 - Wakeup Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn aon_ioc_comb(&mut self) -> AonIocCombW<WkupmaskSpec> {
        AonIocCombW::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<WkupmaskSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "WAKEUP Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkupmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkupmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkupmaskSpec;
impl crate::RegisterSpec for WkupmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkupmask::R`](R) reader structure"]
impl crate::Readable for WkupmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`wkupmask::W`](W) writer structure"]
impl crate::Writable for WkupmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WKUPMASK to value 0"]
impl crate::Resettable for WkupmaskSpec {
    const RESET_VALUE: u32 = 0;
}
