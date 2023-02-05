#[doc = "Register `DMACFG` reader"]
pub type R = crate::R<DmacfgSpec>;
#[doc = "Register `DMACFG` writer"]
pub type W = crate::W<DmacfgSpec>;
#[doc = "0:0\\]
Enables the DMA interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "1: Enable DMA interface. The triggers are able to give activity on the interface"]
    On = 1,
    #[doc = "0: Disable DMA interface, no activity on interface"]
    Off = 0,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - 0:0\\]
Enables the DMA interface"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            true => En::On,
            false => En::Off,
        }
    }
    #[doc = "Enable DMA interface. The triggers are able to give activity on the interface"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == En::On
    }
    #[doc = "Disable DMA interface, no activity on interface"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == En::Off
    }
}
#[doc = "Field `EN` writer - 0:0\\]
Enables the DMA interface"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable DMA interface. The triggers are able to give activity on the interface"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(En::On)
    }
    #[doc = "Disable DMA interface, no activity on interface"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(En::Off)
    }
}
#[doc = "2:1\\]
Select DMA trigger source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigsrc {
    #[doc = "3: The DMA is triggered from the FIFO. See the FIFO configration register for what FIFO event will generate the trigger"]
    Fifo = 3,
    #[doc = "2: The DMA is triggered by the MCE FW trigger"]
    Rfefw = 2,
    #[doc = "1: The DMA is triggered by the MCE FW trigger"]
    Mcefw = 1,
    #[doc = "0: The DMA is triggered by the PBE FW trigger"]
    Pbefw = 0,
}
impl From<Trigsrc> for u8 {
    #[inline(always)]
    fn from(variant: Trigsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigsrc {
    type Ux = u8;
}
impl crate::IsEnum for Trigsrc {}
#[doc = "Field `TRIGSRC` reader - 2:1\\]
Select DMA trigger source"]
pub type TrigsrcR = crate::FieldReader<Trigsrc>;
impl TrigsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigsrc {
        match self.bits {
            3 => Trigsrc::Fifo,
            2 => Trigsrc::Rfefw,
            1 => Trigsrc::Mcefw,
            0 => Trigsrc::Pbefw,
            _ => unreachable!(),
        }
    }
    #[doc = "The DMA is triggered from the FIFO. See the FIFO configration register for what FIFO event will generate the trigger"]
    #[inline(always)]
    pub fn is_fifo(&self) -> bool {
        *self == Trigsrc::Fifo
    }
    #[doc = "The DMA is triggered by the MCE FW trigger"]
    #[inline(always)]
    pub fn is_rfefw(&self) -> bool {
        *self == Trigsrc::Rfefw
    }
    #[doc = "The DMA is triggered by the MCE FW trigger"]
    #[inline(always)]
    pub fn is_mcefw(&self) -> bool {
        *self == Trigsrc::Mcefw
    }
    #[doc = "The DMA is triggered by the PBE FW trigger"]
    #[inline(always)]
    pub fn is_pbefw(&self) -> bool {
        *self == Trigsrc::Pbefw
    }
}
#[doc = "Field `TRIGSRC` writer - 2:1\\]
Select DMA trigger source"]
pub type TrigsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Trigsrc, crate::Safe>;
impl<'a, REG> TrigsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The DMA is triggered from the FIFO. See the FIFO configration register for what FIFO event will generate the trigger"]
    #[inline(always)]
    pub fn fifo(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrc::Fifo)
    }
    #[doc = "The DMA is triggered by the MCE FW trigger"]
    #[inline(always)]
    pub fn rfefw(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrc::Rfefw)
    }
    #[doc = "The DMA is triggered by the MCE FW trigger"]
    #[inline(always)]
    pub fn mcefw(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrc::Mcefw)
    }
    #[doc = "The DMA is triggered by the PBE FW trigger"]
    #[inline(always)]
    pub fn pbefw(self) -> &'a mut crate::W<REG> {
        self.variant(Trigsrc::Pbefw)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables the DMA interface"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Select DMA trigger source"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TrigsrcR {
        TrigsrcR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables the DMA interface"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<DmacfgSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Select DMA trigger source"]
    #[inline(always)]
    #[must_use]
    pub fn trigsrc(&mut self) -> TrigsrcW<DmacfgSpec> {
        TrigsrcW::new(self, 1)
    }
}
#[doc = "DMA Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacfgSpec;
impl crate::RegisterSpec for DmacfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacfg::R`](R) reader structure"]
impl crate::Readable for DmacfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacfg::W`](W) writer structure"]
impl crate::Writable for DmacfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACFG to value 0"]
impl crate::Resettable for DmacfgSpec {
    const RESET_VALUE: u32 = 0;
}
