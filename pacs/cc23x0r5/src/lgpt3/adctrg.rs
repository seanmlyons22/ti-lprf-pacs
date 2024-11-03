#[doc = "Register `ADCTRG` reader"]
pub type R = crate::R<AdctrgSpec>;
#[doc = "Register `ADCTRG` writer"]
pub type W = crate::W<AdctrgSpec>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "15: Setting of RIS.C11CC generates an ADC trigger."]
    C11cc = 15,
    #[doc = "14: Setting of RIS.C10CC generates an ADC trigger."]
    C10cc = 14,
    #[doc = "13: Setting of RIS.C9CC generates an ADC trigger."]
    C9cc = 13,
    #[doc = "12: Setting of RIS.C8CC generates an ADC trigger."]
    C8cc = 12,
    #[doc = "11: Setting of RIS.C7CC generates an ADC trigger."]
    C7cc = 11,
    #[doc = "10: Setting of RIS.C6CC generates an ADC trigger."]
    C6cc = 10,
    #[doc = "9: Setting of RIS.C5CC generates an ADC trigger."]
    C5cc = 9,
    #[doc = "8: Setting of RIS.C4CC generates an ADC trigger."]
    C4cc = 8,
    #[doc = "7: Setting of RIS.C3CC generates an ADC trigger."]
    C3cc = 7,
    #[doc = "6: Setting of RIS.C2CC generates an ADC trigger."]
    C2cc = 6,
    #[doc = "5: Setting of RIS.C1CC generates an ADC trigger."]
    C1cc = 5,
    #[doc = "4: Setting of RIS.C0CC generates an ADC trigger."]
    C0cc = 4,
    #[doc = "3: Setting of RIS.FAULT generates an ADC trigger."]
    Fault = 3,
    #[doc = "2: Setting of RIS.ZERO generates an ADC trigger."]
    Zero = 2,
    #[doc = "1: Setting of RIS.TGT generates an ADC trigger."]
    Tgt = 1,
    #[doc = "0: Disabled"]
    Dis = 0,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - "]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src {
        match self.bits {
            15 => Src::C11cc,
            14 => Src::C10cc,
            13 => Src::C9cc,
            12 => Src::C8cc,
            11 => Src::C7cc,
            10 => Src::C6cc,
            9 => Src::C5cc,
            8 => Src::C4cc,
            7 => Src::C3cc,
            6 => Src::C2cc,
            5 => Src::C1cc,
            4 => Src::C0cc,
            3 => Src::Fault,
            2 => Src::Zero,
            1 => Src::Tgt,
            0 => Src::Dis,
            _ => unreachable!(),
        }
    }
    #[doc = "Setting of RIS.C11CC generates an ADC trigger."]
    #[inline(always)]
    pub fn is_c11cc(&self) -> bool {
        *self == Src::C11cc
    }
    #[doc = "Setting of RIS.C10CC generates an ADC trigger."]
    #[inline(always)]
    pub fn is_c10cc(&self) -> bool {
        *self == Src::C10cc
    }
    #[doc = "Setting of RIS.C9CC generates an ADC trigger."]
    #[inline(always)]
    pub fn is_c9cc(&self) -> bool {
        *self == Src::C9cc
    }
    #[doc = "Setting of RIS.C8CC generates an ADC trigger."]
    #[inline(always)]
    pub fn is_c8cc(&self) -> bool {
        *self == Src::C8cc
    }
    #[doc = "Setting of RIS.C7CC generates an ADC trigger."]
    #[inline(always)]
    pub fn is_c7cc(&self) -> bool {
        *self == Src::C7cc
    }
    #[doc = "Setting of RIS.C6CC generates an ADC trigger."]
    #[inline(always)]
    pub fn is_c6cc(&self) -> bool {
        *self == Src::C6cc
    }
    #[doc = "Setting of RIS.C5CC generates an ADC trigger."]
    #[inline(always)]
    pub fn is_c5cc(&self) -> bool {
        *self == Src::C5cc
    }
    #[doc = "Setting of RIS.C4CC generates an ADC trigger."]
    #[inline(always)]
    pub fn is_c4cc(&self) -> bool {
        *self == Src::C4cc
    }
    #[doc = "Setting of RIS.C3CC generates an ADC trigger."]
    #[inline(always)]
    pub fn is_c3cc(&self) -> bool {
        *self == Src::C3cc
    }
    #[doc = "Setting of RIS.C2CC generates an ADC trigger."]
    #[inline(always)]
    pub fn is_c2cc(&self) -> bool {
        *self == Src::C2cc
    }
    #[doc = "Setting of RIS.C1CC generates an ADC trigger."]
    #[inline(always)]
    pub fn is_c1cc(&self) -> bool {
        *self == Src::C1cc
    }
    #[doc = "Setting of RIS.C0CC generates an ADC trigger."]
    #[inline(always)]
    pub fn is_c0cc(&self) -> bool {
        *self == Src::C0cc
    }
    #[doc = "Setting of RIS.FAULT generates an ADC trigger."]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == Src::Fault
    }
    #[doc = "Setting of RIS.ZERO generates an ADC trigger."]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Src::Zero
    }
    #[doc = "Setting of RIS.TGT generates an ADC trigger."]
    #[inline(always)]
    pub fn is_tgt(&self) -> bool {
        *self == Src::Tgt
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Src::Dis
    }
}
#[doc = "Field `SRC` writer - "]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Src, crate::Safe>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Setting of RIS.C11CC generates an ADC trigger."]
    #[inline(always)]
    pub fn c11cc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::C11cc)
    }
    #[doc = "Setting of RIS.C10CC generates an ADC trigger."]
    #[inline(always)]
    pub fn c10cc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::C10cc)
    }
    #[doc = "Setting of RIS.C9CC generates an ADC trigger."]
    #[inline(always)]
    pub fn c9cc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::C9cc)
    }
    #[doc = "Setting of RIS.C8CC generates an ADC trigger."]
    #[inline(always)]
    pub fn c8cc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::C8cc)
    }
    #[doc = "Setting of RIS.C7CC generates an ADC trigger."]
    #[inline(always)]
    pub fn c7cc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::C7cc)
    }
    #[doc = "Setting of RIS.C6CC generates an ADC trigger."]
    #[inline(always)]
    pub fn c6cc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::C6cc)
    }
    #[doc = "Setting of RIS.C5CC generates an ADC trigger."]
    #[inline(always)]
    pub fn c5cc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::C5cc)
    }
    #[doc = "Setting of RIS.C4CC generates an ADC trigger."]
    #[inline(always)]
    pub fn c4cc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::C4cc)
    }
    #[doc = "Setting of RIS.C3CC generates an ADC trigger."]
    #[inline(always)]
    pub fn c3cc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::C3cc)
    }
    #[doc = "Setting of RIS.C2CC generates an ADC trigger."]
    #[inline(always)]
    pub fn c2cc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::C2cc)
    }
    #[doc = "Setting of RIS.C1CC generates an ADC trigger."]
    #[inline(always)]
    pub fn c1cc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::C1cc)
    }
    #[doc = "Setting of RIS.C0CC generates an ADC trigger."]
    #[inline(always)]
    pub fn c0cc(self) -> &'a mut crate::W<REG> {
        self.variant(Src::C0cc)
    }
    #[doc = "Setting of RIS.FAULT generates an ADC trigger."]
    #[inline(always)]
    pub fn fault(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Fault)
    }
    #[doc = "Setting of RIS.ZERO generates an ADC trigger."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Zero)
    }
    #[doc = "Setting of RIS.TGT generates an ADC trigger."]
    #[inline(always)]
    pub fn tgt(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Tgt)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Dis)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<AdctrgSpec> {
        SrcW::new(self, 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<AdctrgSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "ADC Trigger This register is used to enable ADC trigger from the timer. Choose ADC trigger source by setting the SRC field. The setting of the corresponding interrupt in the RIS registers also sets the ADC trigger.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctrg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctrg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdctrgSpec;
impl crate::RegisterSpec for AdctrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adctrg::R`](R) reader structure"]
impl crate::Readable for AdctrgSpec {}
#[doc = "`write(|w| ..)` method takes [`adctrg::W`](W) writer structure"]
impl crate::Writable for AdctrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCTRG to value 0"]
impl crate::Resettable for AdctrgSpec {
    const RESET_VALUE: u32 = 0;
}
