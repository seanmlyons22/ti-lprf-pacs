#[doc = "Register `PREG0` reader"]
pub type R = crate::R<Preg0Spec>;
#[doc = "Register `PREG0` writer"]
pub type W = crate::W<Preg0Spec>;
#[doc = "0:0\\]
Enable UDIGLDO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UdigldoEn {
    #[doc = "1: Enable"]
    En = 1,
    #[doc = "0: Disable"]
    Dis = 0,
}
impl From<UdigldoEn> for bool {
    #[inline(always)]
    fn from(variant: UdigldoEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDIGLDO_EN` reader - 0:0\\]
Enable UDIGLDO"]
pub type UdigldoEnR = crate::BitReader<UdigldoEn>;
impl UdigldoEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UdigldoEn {
        match self.bits {
            true => UdigldoEn::En,
            false => UdigldoEn::Dis,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == UdigldoEn::En
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == UdigldoEn::Dis
    }
}
#[doc = "Field `UDIGLDO_EN` writer - 0:0\\]
Enable UDIGLDO"]
pub type UdigldoEnW<'a, REG> = crate::BitWriter<'a, REG, UdigldoEn>;
impl<'a, REG> UdigldoEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(UdigldoEn::En)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(UdigldoEn::Dis)
    }
}
#[doc = "Field `SPARE` reader - 1:1\\]
Spare bit"]
pub type SpareR = crate::BitReader;
#[doc = "Field `SPARE` writer - 1:1\\]
Spare bit"]
pub type SpareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "4:2\\]
DIGLDO ATB selection bits. This is used to enable test currents out from DIGLDO through ATEST.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DigldoAtbsel {
    #[doc = "4: 1/60 of output current"]
    Val4 = 4,
    #[doc = "2: 1/120 of output current"]
    Val2 = 2,
    #[doc = "1: 1/240 of output current"]
    Val1 = 1,
    #[doc = "0: No currents out"]
    Val0 = 0,
}
impl From<DigldoAtbsel> for u8 {
    #[inline(always)]
    fn from(variant: DigldoAtbsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DigldoAtbsel {
    type Ux = u8;
}
impl crate::IsEnum for DigldoAtbsel {}
#[doc = "Field `DIGLDO_ATBSEL` reader - 4:2\\]
DIGLDO ATB selection bits. This is used to enable test currents out from DIGLDO through ATEST."]
pub type DigldoAtbselR = crate::FieldReader<DigldoAtbsel>;
impl DigldoAtbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DigldoAtbsel> {
        match self.bits {
            4 => Some(DigldoAtbsel::Val4),
            2 => Some(DigldoAtbsel::Val2),
            1 => Some(DigldoAtbsel::Val1),
            0 => Some(DigldoAtbsel::Val0),
            _ => None,
        }
    }
    #[doc = "1/60 of output current"]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == DigldoAtbsel::Val4
    }
    #[doc = "1/120 of output current"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == DigldoAtbsel::Val2
    }
    #[doc = "1/240 of output current"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == DigldoAtbsel::Val1
    }
    #[doc = "No currents out"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == DigldoAtbsel::Val0
    }
}
#[doc = "Field `DIGLDO_ATBSEL` writer - 4:2\\]
DIGLDO ATB selection bits. This is used to enable test currents out from DIGLDO through ATEST."]
pub type DigldoAtbselW<'a, REG> = crate::FieldWriter<'a, REG, 3, DigldoAtbsel>;
impl<'a, REG> DigldoAtbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/60 of output current"]
    #[inline(always)]
    pub fn val4(self) -> &'a mut crate::W<REG> {
        self.variant(DigldoAtbsel::Val4)
    }
    #[doc = "1/120 of output current"]
    #[inline(always)]
    pub fn val2(self) -> &'a mut crate::W<REG> {
        self.variant(DigldoAtbsel::Val2)
    }
    #[doc = "1/240 of output current"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut crate::W<REG> {
        self.variant(DigldoAtbsel::Val1)
    }
    #[doc = "No currents out"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut crate::W<REG> {
        self.variant(DigldoAtbsel::Val0)
    }
}
#[doc = "6:5\\]
UDIGLDO ATB selection bits. This is used to enable test currents out from UDIGLDO through ATEST.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UdigldoAtbsel {
    #[doc = "3: 3/10 of output current"]
    Val3 = 3,
    #[doc = "2: 1/5 of output current"]
    Val2 = 2,
    #[doc = "1: 1/10 of output current"]
    Val1 = 1,
    #[doc = "0: No currents out"]
    Val0 = 0,
}
impl From<UdigldoAtbsel> for u8 {
    #[inline(always)]
    fn from(variant: UdigldoAtbsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UdigldoAtbsel {
    type Ux = u8;
}
impl crate::IsEnum for UdigldoAtbsel {}
#[doc = "Field `UDIGLDO_ATBSEL` reader - 6:5\\]
UDIGLDO ATB selection bits. This is used to enable test currents out from UDIGLDO through ATEST."]
pub type UdigldoAtbselR = crate::FieldReader<UdigldoAtbsel>;
impl UdigldoAtbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UdigldoAtbsel {
        match self.bits {
            3 => UdigldoAtbsel::Val3,
            2 => UdigldoAtbsel::Val2,
            1 => UdigldoAtbsel::Val1,
            0 => UdigldoAtbsel::Val0,
            _ => unreachable!(),
        }
    }
    #[doc = "3/10 of output current"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == UdigldoAtbsel::Val3
    }
    #[doc = "1/5 of output current"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == UdigldoAtbsel::Val2
    }
    #[doc = "1/10 of output current"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == UdigldoAtbsel::Val1
    }
    #[doc = "No currents out"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == UdigldoAtbsel::Val0
    }
}
#[doc = "Field `UDIGLDO_ATBSEL` writer - 6:5\\]
UDIGLDO ATB selection bits. This is used to enable test currents out from UDIGLDO through ATEST."]
pub type UdigldoAtbselW<'a, REG> = crate::FieldWriter<'a, REG, 2, UdigldoAtbsel, crate::Safe>;
impl<'a, REG> UdigldoAtbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "3/10 of output current"]
    #[inline(always)]
    pub fn val3(self) -> &'a mut crate::W<REG> {
        self.variant(UdigldoAtbsel::Val3)
    }
    #[doc = "1/5 of output current"]
    #[inline(always)]
    pub fn val2(self) -> &'a mut crate::W<REG> {
        self.variant(UdigldoAtbsel::Val2)
    }
    #[doc = "1/10 of output current"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut crate::W<REG> {
        self.variant(UdigldoAtbsel::Val1)
    }
    #[doc = "No currents out"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut crate::W<REG> {
        self.variant(UdigldoAtbsel::Val0)
    }
}
#[doc = "9:7\\]
SOC LDO ATB selection bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SocldoAtbsel {
    #[doc = "4: Connect VDD_AON"]
    VddAon = 4,
    #[doc = "2: Connect reference amplifier output in SOC LDO"]
    SocldoVrefAmpOut = 2,
    #[doc = "1: Connect selected LDO current output"]
    SocldoItest = 1,
    #[doc = "0: No test muxes connected"]
    Nc = 0,
}
impl From<SocldoAtbsel> for u8 {
    #[inline(always)]
    fn from(variant: SocldoAtbsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SocldoAtbsel {
    type Ux = u8;
}
impl crate::IsEnum for SocldoAtbsel {}
#[doc = "Field `SOCLDO_ATBSEL` reader - 9:7\\]
SOC LDO ATB selection bits."]
pub type SocldoAtbselR = crate::FieldReader<SocldoAtbsel>;
impl SocldoAtbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SocldoAtbsel> {
        match self.bits {
            4 => Some(SocldoAtbsel::VddAon),
            2 => Some(SocldoAtbsel::SocldoVrefAmpOut),
            1 => Some(SocldoAtbsel::SocldoItest),
            0 => Some(SocldoAtbsel::Nc),
            _ => None,
        }
    }
    #[doc = "Connect VDD_AON"]
    #[inline(always)]
    pub fn is_vdd_aon(&self) -> bool {
        *self == SocldoAtbsel::VddAon
    }
    #[doc = "Connect reference amplifier output in SOC LDO"]
    #[inline(always)]
    pub fn is_socldo_vref_amp_out(&self) -> bool {
        *self == SocldoAtbsel::SocldoVrefAmpOut
    }
    #[doc = "Connect selected LDO current output"]
    #[inline(always)]
    pub fn is_socldo_itest(&self) -> bool {
        *self == SocldoAtbsel::SocldoItest
    }
    #[doc = "No test muxes connected"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == SocldoAtbsel::Nc
    }
}
#[doc = "Field `SOCLDO_ATBSEL` writer - 9:7\\]
SOC LDO ATB selection bits."]
pub type SocldoAtbselW<'a, REG> = crate::FieldWriter<'a, REG, 3, SocldoAtbsel>;
impl<'a, REG> SocldoAtbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Connect VDD_AON"]
    #[inline(always)]
    pub fn vdd_aon(self) -> &'a mut crate::W<REG> {
        self.variant(SocldoAtbsel::VddAon)
    }
    #[doc = "Connect reference amplifier output in SOC LDO"]
    #[inline(always)]
    pub fn socldo_vref_amp_out(self) -> &'a mut crate::W<REG> {
        self.variant(SocldoAtbsel::SocldoVrefAmpOut)
    }
    #[doc = "Connect selected LDO current output"]
    #[inline(always)]
    pub fn socldo_itest(self) -> &'a mut crate::W<REG> {
        self.variant(SocldoAtbsel::SocldoItest)
    }
    #[doc = "No test muxes connected"]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(SocldoAtbsel::Nc)
    }
}
#[doc = "10:10\\]
SOC LDO current test mode enable bit. Needs to be enabled for DIGLDO and UDIGLDO output current testing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SocldoItesten {
    #[doc = "1: Enable"]
    En = 1,
    #[doc = "0: Disable"]
    Dis = 0,
}
impl From<SocldoItesten> for bool {
    #[inline(always)]
    fn from(variant: SocldoItesten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOCLDO_ITESTEN` reader - 10:10\\]
SOC LDO current test mode enable bit. Needs to be enabled for DIGLDO and UDIGLDO output current testing."]
pub type SocldoItestenR = crate::BitReader<SocldoItesten>;
impl SocldoItestenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SocldoItesten {
        match self.bits {
            true => SocldoItesten::En,
            false => SocldoItesten::Dis,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == SocldoItesten::En
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SocldoItesten::Dis
    }
}
#[doc = "Field `SOCLDO_ITESTEN` writer - 10:10\\]
SOC LDO current test mode enable bit. Needs to be enabled for DIGLDO and UDIGLDO output current testing."]
pub type SocldoItestenW<'a, REG> = crate::BitWriter<'a, REG, SocldoItesten>;
impl<'a, REG> SocldoItestenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(SocldoItesten::En)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(SocldoItesten::Dis)
    }
}
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED11` writer - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable UDIGLDO"]
    #[inline(always)]
    pub fn udigldo_en(&self) -> UdigldoEnR {
        UdigldoEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Spare bit"]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - 4:2\\]
DIGLDO ATB selection bits. This is used to enable test currents out from DIGLDO through ATEST."]
    #[inline(always)]
    pub fn digldo_atbsel(&self) -> DigldoAtbselR {
        DigldoAtbselR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:6 - 6:5\\]
UDIGLDO ATB selection bits. This is used to enable test currents out from UDIGLDO through ATEST."]
    #[inline(always)]
    pub fn udigldo_atbsel(&self) -> UdigldoAtbselR {
        UdigldoAtbselR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:9 - 9:7\\]
SOC LDO ATB selection bits."]
    #[inline(always)]
    pub fn socldo_atbsel(&self) -> SocldoAtbselR {
        SocldoAtbselR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
SOC LDO current test mode enable bit. Needs to be enabled for DIGLDO and UDIGLDO output current testing."]
    #[inline(always)]
    pub fn socldo_itesten(&self) -> SocldoItestenR {
        SocldoItestenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable UDIGLDO"]
    #[inline(always)]
    #[must_use]
    pub fn udigldo_en(&mut self) -> UdigldoEnW<Preg0Spec> {
        UdigldoEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Spare bit"]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<Preg0Spec> {
        SpareW::new(self, 1)
    }
    #[doc = "Bits 2:4 - 4:2\\]
DIGLDO ATB selection bits. This is used to enable test currents out from DIGLDO through ATEST."]
    #[inline(always)]
    #[must_use]
    pub fn digldo_atbsel(&mut self) -> DigldoAtbselW<Preg0Spec> {
        DigldoAtbselW::new(self, 2)
    }
    #[doc = "Bits 5:6 - 6:5\\]
UDIGLDO ATB selection bits. This is used to enable test currents out from UDIGLDO through ATEST."]
    #[inline(always)]
    #[must_use]
    pub fn udigldo_atbsel(&mut self) -> UdigldoAtbselW<Preg0Spec> {
        UdigldoAtbselW::new(self, 5)
    }
    #[doc = "Bits 7:9 - 9:7\\]
SOC LDO ATB selection bits."]
    #[inline(always)]
    #[must_use]
    pub fn socldo_atbsel(&mut self) -> SocldoAtbselW<Preg0Spec> {
        SocldoAtbselW::new(self, 7)
    }
    #[doc = "Bit 10 - 10:10\\]
SOC LDO current test mode enable bit. Needs to be enabled for DIGLDO and UDIGLDO output current testing."]
    #[inline(always)]
    #[must_use]
    pub fn socldo_itesten(&mut self) -> SocldoItestenW<Preg0Spec> {
        SocldoItestenW::new(self, 10)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> Reserved11W<Preg0Spec> {
        Reserved11W::new(self, 11)
    }
}
#[doc = "PMU REG 0 register. Note: All bits in this register except UDIG_LDO_EN are write-protected based on global lock signal from SYS0 on production devices.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`preg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`preg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Preg0Spec;
impl crate::RegisterSpec for Preg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`preg0::R`](R) reader structure"]
impl crate::Readable for Preg0Spec {}
#[doc = "`write(|w| ..)` method takes [`preg0::W`](W) writer structure"]
impl crate::Writable for Preg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PREG0 to value 0"]
impl crate::Resettable for Preg0Spec {
    const RESET_VALUE: u32 = 0;
}
