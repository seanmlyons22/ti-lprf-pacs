#[doc = "Register `VECCFG4` reader"]
pub type R = crate::R<Veccfg4Spec>;
#[doc = "Register `VECCFG4` writer"]
pub type W = crate::W<Veccfg4Spec>;
#[doc = "3:0\\]
Select trigger event for vector 4. Non-enumerated values are treated as NONE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VecEv {
    #[doc = "9: AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    AonRtcCh2Dly = 9,
    #[doc = "8: WUFLAGS.SW_WU3"]
    SwWu3 = 8,
    #[doc = "7: WUFLAGS.SW_WU2"]
    SwWu2 = 7,
    #[doc = "6: WUFLAGS.SW_WU1"]
    SwWu1 = 6,
    #[doc = "5: WUFLAGS.SW_WU0"]
    SwWu0 = 5,
    #[doc = "4: WUFLAGS.PROG_WU3"]
    ProgWu3 = 4,
    #[doc = "3: WUFLAGS.PROG_WU2"]
    ProgWu2 = 3,
    #[doc = "2: WUFLAGS.PROG_WU1"]
    ProgWu1 = 2,
    #[doc = "1: WUFLAGS.PROG_WU0"]
    ProgWu0 = 1,
    #[doc = "0: Vector is disabled."]
    None = 0,
}
impl From<VecEv> for u8 {
    #[inline(always)]
    fn from(variant: VecEv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VecEv {
    type Ux = u8;
}
impl crate::IsEnum for VecEv {}
#[doc = "Field `VEC_EV` reader - 3:0\\]
Select trigger event for vector 4. Non-enumerated values are treated as NONE."]
pub type VecEvR = crate::FieldReader<VecEv>;
impl VecEvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VecEv> {
        match self.bits {
            9 => Some(VecEv::AonRtcCh2Dly),
            8 => Some(VecEv::SwWu3),
            7 => Some(VecEv::SwWu2),
            6 => Some(VecEv::SwWu1),
            5 => Some(VecEv::SwWu0),
            4 => Some(VecEv::ProgWu3),
            3 => Some(VecEv::ProgWu2),
            2 => Some(VecEv::ProgWu1),
            1 => Some(VecEv::ProgWu0),
            0 => Some(VecEv::None),
            _ => None,
        }
    }
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn is_aon_rtc_ch2_dly(&self) -> bool {
        *self == VecEv::AonRtcCh2Dly
    }
    #[doc = "WUFLAGS.SW_WU3"]
    #[inline(always)]
    pub fn is_sw_wu3(&self) -> bool {
        *self == VecEv::SwWu3
    }
    #[doc = "WUFLAGS.SW_WU2"]
    #[inline(always)]
    pub fn is_sw_wu2(&self) -> bool {
        *self == VecEv::SwWu2
    }
    #[doc = "WUFLAGS.SW_WU1"]
    #[inline(always)]
    pub fn is_sw_wu1(&self) -> bool {
        *self == VecEv::SwWu1
    }
    #[doc = "WUFLAGS.SW_WU0"]
    #[inline(always)]
    pub fn is_sw_wu0(&self) -> bool {
        *self == VecEv::SwWu0
    }
    #[doc = "WUFLAGS.PROG_WU3"]
    #[inline(always)]
    pub fn is_prog_wu3(&self) -> bool {
        *self == VecEv::ProgWu3
    }
    #[doc = "WUFLAGS.PROG_WU2"]
    #[inline(always)]
    pub fn is_prog_wu2(&self) -> bool {
        *self == VecEv::ProgWu2
    }
    #[doc = "WUFLAGS.PROG_WU1"]
    #[inline(always)]
    pub fn is_prog_wu1(&self) -> bool {
        *self == VecEv::ProgWu1
    }
    #[doc = "WUFLAGS.PROG_WU0"]
    #[inline(always)]
    pub fn is_prog_wu0(&self) -> bool {
        *self == VecEv::ProgWu0
    }
    #[doc = "Vector is disabled."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == VecEv::None
    }
}
#[doc = "Field `VEC_EV` writer - 3:0\\]
Select trigger event for vector 4. Non-enumerated values are treated as NONE."]
pub type VecEvW<'a, REG> = crate::FieldWriter<'a, REG, 4, VecEv>;
impl<'a, REG> VecEvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY"]
    #[inline(always)]
    pub fn aon_rtc_ch2_dly(self) -> &'a mut crate::W<REG> {
        self.variant(VecEv::AonRtcCh2Dly)
    }
    #[doc = "WUFLAGS.SW_WU3"]
    #[inline(always)]
    pub fn sw_wu3(self) -> &'a mut crate::W<REG> {
        self.variant(VecEv::SwWu3)
    }
    #[doc = "WUFLAGS.SW_WU2"]
    #[inline(always)]
    pub fn sw_wu2(self) -> &'a mut crate::W<REG> {
        self.variant(VecEv::SwWu2)
    }
    #[doc = "WUFLAGS.SW_WU1"]
    #[inline(always)]
    pub fn sw_wu1(self) -> &'a mut crate::W<REG> {
        self.variant(VecEv::SwWu1)
    }
    #[doc = "WUFLAGS.SW_WU0"]
    #[inline(always)]
    pub fn sw_wu0(self) -> &'a mut crate::W<REG> {
        self.variant(VecEv::SwWu0)
    }
    #[doc = "WUFLAGS.PROG_WU3"]
    #[inline(always)]
    pub fn prog_wu3(self) -> &'a mut crate::W<REG> {
        self.variant(VecEv::ProgWu3)
    }
    #[doc = "WUFLAGS.PROG_WU2"]
    #[inline(always)]
    pub fn prog_wu2(self) -> &'a mut crate::W<REG> {
        self.variant(VecEv::ProgWu2)
    }
    #[doc = "WUFLAGS.PROG_WU1"]
    #[inline(always)]
    pub fn prog_wu1(self) -> &'a mut crate::W<REG> {
        self.variant(VecEv::ProgWu1)
    }
    #[doc = "WUFLAGS.PROG_WU0"]
    #[inline(always)]
    pub fn prog_wu0(self) -> &'a mut crate::W<REG> {
        self.variant(VecEv::ProgWu0)
    }
    #[doc = "Vector is disabled."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(VecEv::None)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Select trigger event for vector 4. Non-enumerated values are treated as NONE."]
    #[inline(always)]
    pub fn vec_ev(&self) -> VecEvR {
        VecEvR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Select trigger event for vector 4. Non-enumerated values are treated as NONE."]
    #[inline(always)]
    #[must_use]
    pub fn vec_ev(&mut self) -> VecEvW<Veccfg4Spec> {
        VecEvW::new(self, 0)
    }
}
#[doc = "Vector Configuration 4 AUX_SCE wakeup vector 4 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`veccfg4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`veccfg4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Veccfg4Spec;
impl crate::RegisterSpec for Veccfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`veccfg4::R`](R) reader structure"]
impl crate::Readable for Veccfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`veccfg4::W`](W) writer structure"]
impl crate::Writable for Veccfg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VECCFG4 to value 0"]
impl crate::Resettable for Veccfg4Spec {
    const RESET_VALUE: u32 = 0;
}
