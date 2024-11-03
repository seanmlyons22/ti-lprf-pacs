#[doc = "Register `HFXTCTL` reader"]
pub type R = crate::R<HfxtctlSpec>;
#[doc = "Register `HFXTCTL` writer"]
pub type W = crate::W<HfxtctlSpec>;
#[doc = "Field `EN` reader - 0:0\\]
HFXT enable. Setting this bit will enable HFXT. It will automatically be cleared upon STANDBY entry. If AUTOEN is set, this bit will be set automatically on wakeup or before (pre-wake mechanism in PMCTL)."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
HFXT enable. Setting this bit will enable HFXT. It will automatically be cleared upon STANDBY entry. If AUTOEN is set, this bit will be set automatically on wakeup or before (pre-wake mechanism in PMCTL)."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPBUFEN` reader - 1:1\\]
High performance clock buffer enable. This bit controls the clock output for the RF PLL. It is required for radio operation."]
pub type HpbufenR = crate::BitReader;
#[doc = "Field `HPBUFEN` writer - 1:1\\]
High performance clock buffer enable. This bit controls the clock output for the RF PLL. It is required for radio operation."]
pub type HpbufenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOEN` reader - 2:2\\]
Automatic HFXT enable. If this bit is set, EN will automatically be set at wakeup or before (using pre-wake mechanism in PMCTL)."]
pub type AutoenR = crate::BitReader;
#[doc = "Field `AUTOEN` writer - 2:2\\]
Automatic HFXT enable. If this bit is set, EN will automatically be set at wakeup or before (using pre-wake mechanism in PMCTL)."]
pub type AutoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 5:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESERVED3` writer - 5:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "6:6\\]
Type of temperature compensated crystal used. Only has effect if TCXOMODE is set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcxotype {
    #[doc = "1: Use with CMOS TCXO"]
    Cmos = 1,
    #[doc = "0: Use with clipped-sine TCXO"]
    Clippedsine = 0,
}
impl From<Tcxotype> for bool {
    #[inline(always)]
    fn from(variant: Tcxotype) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCXOTYPE` reader - 6:6\\]
Type of temperature compensated crystal used. Only has effect if TCXOMODE is set."]
pub type TcxotypeR = crate::BitReader<Tcxotype>;
impl TcxotypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcxotype {
        match self.bits {
            true => Tcxotype::Cmos,
            false => Tcxotype::Clippedsine,
        }
    }
    #[doc = "Use with CMOS TCXO"]
    #[inline(always)]
    pub fn is_cmos(&self) -> bool {
        *self == Tcxotype::Cmos
    }
    #[doc = "Use with clipped-sine TCXO"]
    #[inline(always)]
    pub fn is_clippedsine(&self) -> bool {
        *self == Tcxotype::Clippedsine
    }
}
#[doc = "Field `TCXOTYPE` writer - 6:6\\]
Type of temperature compensated crystal used. Only has effect if TCXOMODE is set."]
pub type TcxotypeW<'a, REG> = crate::BitWriter<'a, REG, Tcxotype>;
impl<'a, REG> TcxotypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use with CMOS TCXO"]
    #[inline(always)]
    pub fn cmos(self) -> &'a mut crate::W<REG> {
        self.variant(Tcxotype::Cmos)
    }
    #[doc = "Use with clipped-sine TCXO"]
    #[inline(always)]
    pub fn clippedsine(self) -> &'a mut crate::W<REG> {
        self.variant(Tcxotype::Clippedsine)
    }
}
#[doc = "Field `TCXOMODE` reader - 7:7\\]
Temperature compensated crystal oscillator mode. Set this bit if a TXCO is connected."]
pub type TcxomodeR = crate::BitReader;
#[doc = "Field `TCXOMODE` writer - 7:7\\]
Temperature compensated crystal oscillator mode. Set this bit if a TXCO is connected."]
pub type TcxomodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUALDLY` reader - 19:8\\]
Skip potentially unstable clock cycles after enabling HFXT. Number of cycles skipped is 8*QUALDLY."]
pub type QualdlyR = crate::FieldReader<u16>;
#[doc = "Field `QUALDLY` writer - 19:8\\]
Skip potentially unstable clock cycles after enabling HFXT. Number of cycles skipped is 8*QUALDLY."]
pub type QualdlyW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED20` writer - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
HFXT enable. Setting this bit will enable HFXT. It will automatically be cleared upon STANDBY entry. If AUTOEN is set, this bit will be set automatically on wakeup or before (pre-wake mechanism in PMCTL)."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
High performance clock buffer enable. This bit controls the clock output for the RF PLL. It is required for radio operation."]
    #[inline(always)]
    pub fn hpbufen(&self) -> HpbufenR {
        HpbufenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Automatic HFXT enable. If this bit is set, EN will automatically be set at wakeup or before (using pre-wake mechanism in PMCTL)."]
    #[inline(always)]
    pub fn autoen(&self) -> AutoenR {
        AutoenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Type of temperature compensated crystal used. Only has effect if TCXOMODE is set."]
    #[inline(always)]
    pub fn tcxotype(&self) -> TcxotypeR {
        TcxotypeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Temperature compensated crystal oscillator mode. Set this bit if a TXCO is connected."]
    #[inline(always)]
    pub fn tcxomode(&self) -> TcxomodeR {
        TcxomodeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Skip potentially unstable clock cycles after enabling HFXT. Number of cycles skipped is 8*QUALDLY."]
    #[inline(always)]
    pub fn qualdly(&self) -> QualdlyR {
        QualdlyR::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
HFXT enable. Setting this bit will enable HFXT. It will automatically be cleared upon STANDBY entry. If AUTOEN is set, this bit will be set automatically on wakeup or before (pre-wake mechanism in PMCTL)."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<HfxtctlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
High performance clock buffer enable. This bit controls the clock output for the RF PLL. It is required for radio operation."]
    #[inline(always)]
    #[must_use]
    pub fn hpbufen(&mut self) -> HpbufenW<HfxtctlSpec> {
        HpbufenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Automatic HFXT enable. If this bit is set, EN will automatically be set at wakeup or before (using pre-wake mechanism in PMCTL)."]
    #[inline(always)]
    #[must_use]
    pub fn autoen(&mut self) -> AutoenW<HfxtctlSpec> {
        AutoenW::new(self, 2)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<HfxtctlSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 6 - 6:6\\]
Type of temperature compensated crystal used. Only has effect if TCXOMODE is set."]
    #[inline(always)]
    #[must_use]
    pub fn tcxotype(&mut self) -> TcxotypeW<HfxtctlSpec> {
        TcxotypeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Temperature compensated crystal oscillator mode. Set this bit if a TXCO is connected."]
    #[inline(always)]
    #[must_use]
    pub fn tcxomode(&mut self) -> TcxomodeW<HfxtctlSpec> {
        TcxomodeW::new(self, 7)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Skip potentially unstable clock cycles after enabling HFXT. Number of cycles skipped is 8*QUALDLY."]
    #[inline(always)]
    #[must_use]
    pub fn qualdly(&mut self) -> QualdlyW<HfxtctlSpec> {
        QualdlyW::new(self, 8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> Reserved20W<HfxtctlSpec> {
        Reserved20W::new(self, 20)
    }
}
#[doc = "High frequency crystal control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxtctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxtctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfxtctlSpec;
impl crate::RegisterSpec for HfxtctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxtctl::R`](R) reader structure"]
impl crate::Readable for HfxtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`hfxtctl::W`](W) writer structure"]
impl crate::Writable for HfxtctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFXTCTL to value 0"]
impl crate::Resettable for HfxtctlSpec {
    const RESET_VALUE: u32 = 0;
}
