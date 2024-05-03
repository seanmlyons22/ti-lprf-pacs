#[doc = "Register `DEB_AUTH_CFG` reader"]
pub type R = crate::R<DebAuthCfgSpec>;
#[doc = "Register `DEB_AUTH_CFG` writer"]
pub type W = crate::W<DebAuthCfgSpec>;
#[doc = "Field `SPIDENSEL` reader - 0:0\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPIDENSEL by ROM boot FW"]
pub type SpidenselR = crate::BitReader;
#[doc = "Field `SPIDENSEL` writer - 0:0\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPIDENSEL by ROM boot FW"]
pub type SpidenselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTSPIDEN` reader - 1:1\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPIDEN by ROM boot FW"]
pub type IntspidenR = crate::BitReader;
#[doc = "Field `INTSPIDEN` writer - 1:1\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPIDEN by ROM boot FW"]
pub type IntspidenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPNIDENSEL` reader - 2:2\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPNIDENSEL by ROM boot FW"]
pub type SpnidenselR = crate::BitReader;
#[doc = "Field `SPNIDENSEL` writer - 2:2\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPNIDENSEL by ROM boot FW"]
pub type SpnidenselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTSPNIDEN` reader - 3:3\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPNIDEN by ROM boot FW"]
pub type IntspnidenR = crate::BitReader;
#[doc = "Field `INTSPNIDEN` writer - 3:3\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPNIDEN by ROM boot FW"]
pub type IntspnidenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPIDENSEL by ROM boot FW"]
    #[inline(always)]
    pub fn spidensel(&self) -> SpidenselR {
        SpidenselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPIDEN by ROM boot FW"]
    #[inline(always)]
    pub fn intspiden(&self) -> IntspidenR {
        IntspidenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPNIDENSEL by ROM boot FW"]
    #[inline(always)]
    pub fn spnidensel(&self) -> SpnidenselR {
        SpnidenselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPNIDEN by ROM boot FW"]
    #[inline(always)]
    pub fn intspniden(&self) -> IntspnidenR {
        IntspnidenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPIDENSEL by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn spidensel(&mut self) -> SpidenselW<DebAuthCfgSpec> {
        SpidenselW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPIDEN by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn intspiden(&mut self) -> IntspidenW<DebAuthCfgSpec> {
        IntspidenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Value will be written to CPU_DCB:DAUTHCTRL.SPNIDENSEL by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn spnidensel(&mut self) -> SpnidenselW<DebAuthCfgSpec> {
        SpnidenselW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Value will be written to CPU_DCB:DAUTHCTRL.INTSPNIDEN by ROM boot FW"]
    #[inline(always)]
    #[must_use]
    pub fn intspniden(&mut self) -> IntspnidenW<DebAuthCfgSpec> {
        IntspnidenW::new(self, 3)
    }
}
#[doc = "Configuration register for debug authentication\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deb_auth_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deb_auth_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebAuthCfgSpec;
impl crate::RegisterSpec for DebAuthCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deb_auth_cfg::R`](R) reader structure"]
impl crate::Readable for DebAuthCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`deb_auth_cfg::W`](W) writer structure"]
impl crate::Writable for DebAuthCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEB_AUTH_CFG to value 0xffff_ffff"]
impl crate::Resettable for DebAuthCfgSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
