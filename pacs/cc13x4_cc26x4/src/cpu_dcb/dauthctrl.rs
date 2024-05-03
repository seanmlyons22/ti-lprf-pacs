#[doc = "Register `DAUTHCTRL` reader"]
pub type R = crate::R<DauthctrlSpec>;
#[doc = "Register `DAUTHCTRL` writer"]
pub type W = crate::W<DauthctrlSpec>;
#[doc = "Field `SPIDENSEL` reader - 0:0\\]
Secure invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure invasive debug."]
pub type SpidenselR = crate::BitReader;
#[doc = "Field `SPIDENSEL` writer - 0:0\\]
Secure invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure invasive debug."]
pub type SpidenselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTSPIDEN` reader - 1:1\\]
Internal Secure invasive debug enable. Overrides the external Secure invasive debug authentication Interfaces."]
pub type IntspidenR = crate::BitReader;
#[doc = "Field `INTSPIDEN` writer - 1:1\\]
Internal Secure invasive debug enable. Overrides the external Secure invasive debug authentication Interfaces."]
pub type IntspidenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPNIDENSEL` reader - 2:2\\]
Secure non-invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure non-invasive debug"]
pub type SpnidenselR = crate::BitReader;
#[doc = "Field `SPNIDENSEL` writer - 2:2\\]
Secure non-invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure non-invasive debug"]
pub type SpnidenselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTSPNIDEN` reader - 3:3\\]
Internal Secure non-invasive debug enable. Overrides the external Secure non-invasive debug authentication interface"]
pub type IntspnidenR = crate::BitReader;
#[doc = "Field `INTSPNIDEN` writer - 3:3\\]
Internal Secure non-invasive debug enable. Overrides the external Secure non-invasive debug authentication interface"]
pub type IntspnidenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Secure invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure invasive debug."]
    #[inline(always)]
    pub fn spidensel(&self) -> SpidenselR {
        SpidenselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal Secure invasive debug enable. Overrides the external Secure invasive debug authentication Interfaces."]
    #[inline(always)]
    pub fn intspiden(&self) -> IntspidenR {
        IntspidenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Secure non-invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure non-invasive debug"]
    #[inline(always)]
    pub fn spnidensel(&self) -> SpnidenselR {
        SpnidenselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal Secure non-invasive debug enable. Overrides the external Secure non-invasive debug authentication interface"]
    #[inline(always)]
    pub fn intspniden(&self) -> IntspnidenR {
        IntspnidenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Secure invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure invasive debug."]
    #[inline(always)]
    #[must_use]
    pub fn spidensel(&mut self) -> SpidenselW<DauthctrlSpec> {
        SpidenselW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal Secure invasive debug enable. Overrides the external Secure invasive debug authentication Interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn intspiden(&mut self) -> IntspidenW<DauthctrlSpec> {
        IntspidenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Secure non-invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure non-invasive debug"]
    #[inline(always)]
    #[must_use]
    pub fn spnidensel(&mut self) -> SpnidenselW<DauthctrlSpec> {
        SpnidenselW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal Secure non-invasive debug enable. Overrides the external Secure non-invasive debug authentication interface"]
    #[inline(always)]
    #[must_use]
    pub fn intspniden(&mut self) -> IntspnidenW<DauthctrlSpec> {
        IntspnidenW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<DauthctrlSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "This register allows the external authentication interface to be overridden from software.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dauthctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dauthctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DauthctrlSpec;
impl crate::RegisterSpec for DauthctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dauthctrl::R`](R) reader structure"]
impl crate::Readable for DauthctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dauthctrl::W`](W) writer structure"]
impl crate::Writable for DauthctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAUTHCTRL to value 0"]
impl crate::Resettable for DauthctrlSpec {
    const RESET_VALUE: u32 = 0;
}
