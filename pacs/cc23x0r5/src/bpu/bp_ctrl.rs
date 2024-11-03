#[doc = "Register `BP_CTRL` reader"]
pub type R = crate::R<BpCtrlSpec>;
#[doc = "Register `BP_CTRL` writer"]
pub type W = crate::W<BpCtrlSpec>;
#[doc = "0:0\\]
Breakpoint unit enable bit. DBGRESETn clears the ENABLE bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "1: Breakpoint unit enabled"]
    BkptEn = 1,
    #[doc = "0: Breakpoint unit disabled"]
    BkptDis = 0,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - 0:0\\]
Breakpoint unit enable bit. DBGRESETn clears the ENABLE bit."]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            true => Enable::BkptEn,
            false => Enable::BkptDis,
        }
    }
    #[doc = "Breakpoint unit enabled"]
    #[inline(always)]
    pub fn is_bkpt_en(&self) -> bool {
        *self == Enable::BkptEn
    }
    #[doc = "Breakpoint unit disabled"]
    #[inline(always)]
    pub fn is_bkpt_dis(&self) -> bool {
        *self == Enable::BkptDis
    }
}
#[doc = "Field `ENABLE` writer - 0:0\\]
Breakpoint unit enable bit. DBGRESETn clears the ENABLE bit."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Breakpoint unit enabled"]
    #[inline(always)]
    pub fn bkpt_en(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::BkptEn)
    }
    #[doc = "Breakpoint unit disabled"]
    #[inline(always)]
    pub fn bkpt_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::BkptDis)
    }
}
#[doc = "Field `KEY` reader - 1:1\\]
Key field. To write to the Breakpoint Control Register, you must write a 1 to this write-only bit. This bit reads as zero."]
pub type KeyR = crate::BitReader;
#[doc = "Field `KEY` writer - 1:1\\]
Key field. To write to the Breakpoint Control Register, you must write a 1 to this write-only bit. This bit reads as zero."]
pub type KeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RESERVED2` writer - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NUM_CODE` reader - 7:4\\]
Number of comparators."]
pub type NumCodeR = crate::FieldReader;
#[doc = "Field `NUM_CODE` writer - 7:4\\]
Number of comparators."]
pub type NumCodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Breakpoint unit enable bit. DBGRESETn clears the ENABLE bit."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Key field. To write to the Breakpoint Control Register, you must write a 1 to this write-only bit. This bit reads as zero."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of comparators."]
    #[inline(always)]
    pub fn num_code(&self) -> NumCodeR {
        NumCodeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Breakpoint unit enable bit. DBGRESETn clears the ENABLE bit."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<BpCtrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Key field. To write to the Breakpoint Control Register, you must write a 1 to this write-only bit. This bit reads as zero."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<BpCtrlSpec> {
        KeyW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<BpCtrlSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of comparators."]
    #[inline(always)]
    #[must_use]
    pub fn num_code(&mut self) -> NumCodeW<BpCtrlSpec> {
        NumCodeW::new(self, 4)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<BpCtrlSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Use the Breakpoint Control Register to enable the Breakpoint block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bp_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BpCtrlSpec;
impl crate::RegisterSpec for BpCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bp_ctrl::R`](R) reader structure"]
impl crate::Readable for BpCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bp_ctrl::W`](W) writer structure"]
impl crate::Writable for BpCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BP_CTRL to value 0x40"]
impl crate::Resettable for BpCtrlSpec {
    const RESET_VALUE: u32 = 0x40;
}
