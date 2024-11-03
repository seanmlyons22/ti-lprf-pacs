#[doc = "Register `DTB` reader"]
pub type R = crate::R<DtbSpec>;
#[doc = "Register `DTB` writer"]
pub type W = crate::W<DtbSpec>;
#[doc = "3:0\\]
DTB MUX select pin value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "15: Selects test group 15"]
    Grp15 = 15,
    #[doc = "14: Selects test group 14"]
    Grp14 = 14,
    #[doc = "13: Selects test group 13"]
    Grp13 = 13,
    #[doc = "12: Selects test group 12"]
    Grp12 = 12,
    #[doc = "11: Selects test group 11"]
    Grp11 = 11,
    #[doc = "10: Selects test group 10"]
    Grp10 = 10,
    #[doc = "9: Selects test group 9"]
    Grp9 = 9,
    #[doc = "8: Selects test group 8"]
    Grp8 = 8,
    #[doc = "7: Selects test group 7"]
    Grp7 = 7,
    #[doc = "6: Selects test group 6"]
    Grp6 = 6,
    #[doc = "5: Selects test group 5"]
    Grp5 = 5,
    #[doc = "4: Selects test group 4"]
    Grp4 = 4,
    #[doc = "3: Selects test group 3"]
    Grp3 = 3,
    #[doc = "2: Selects test group 2"]
    Grp2 = 2,
    #[doc = "1: Selects test group 1"]
    Grp1 = 1,
    #[doc = "0: DTB output from peripheral is 0x0."]
    Disable = 0,
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel {
    type Ux = u8;
}
impl crate::IsEnum for Sel {}
#[doc = "Field `SEL` reader - 3:0\\]
DTB MUX select pin value"]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            15 => Sel::Grp15,
            14 => Sel::Grp14,
            13 => Sel::Grp13,
            12 => Sel::Grp12,
            11 => Sel::Grp11,
            10 => Sel::Grp10,
            9 => Sel::Grp9,
            8 => Sel::Grp8,
            7 => Sel::Grp7,
            6 => Sel::Grp6,
            5 => Sel::Grp5,
            4 => Sel::Grp4,
            3 => Sel::Grp3,
            2 => Sel::Grp2,
            1 => Sel::Grp1,
            0 => Sel::Disable,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects test group 15"]
    #[inline(always)]
    pub fn is_grp15(&self) -> bool {
        *self == Sel::Grp15
    }
    #[doc = "Selects test group 14"]
    #[inline(always)]
    pub fn is_grp14(&self) -> bool {
        *self == Sel::Grp14
    }
    #[doc = "Selects test group 13"]
    #[inline(always)]
    pub fn is_grp13(&self) -> bool {
        *self == Sel::Grp13
    }
    #[doc = "Selects test group 12"]
    #[inline(always)]
    pub fn is_grp12(&self) -> bool {
        *self == Sel::Grp12
    }
    #[doc = "Selects test group 11"]
    #[inline(always)]
    pub fn is_grp11(&self) -> bool {
        *self == Sel::Grp11
    }
    #[doc = "Selects test group 10"]
    #[inline(always)]
    pub fn is_grp10(&self) -> bool {
        *self == Sel::Grp10
    }
    #[doc = "Selects test group 9"]
    #[inline(always)]
    pub fn is_grp9(&self) -> bool {
        *self == Sel::Grp9
    }
    #[doc = "Selects test group 8"]
    #[inline(always)]
    pub fn is_grp8(&self) -> bool {
        *self == Sel::Grp8
    }
    #[doc = "Selects test group 7"]
    #[inline(always)]
    pub fn is_grp7(&self) -> bool {
        *self == Sel::Grp7
    }
    #[doc = "Selects test group 6"]
    #[inline(always)]
    pub fn is_grp6(&self) -> bool {
        *self == Sel::Grp6
    }
    #[doc = "Selects test group 5"]
    #[inline(always)]
    pub fn is_grp5(&self) -> bool {
        *self == Sel::Grp5
    }
    #[doc = "Selects test group 4"]
    #[inline(always)]
    pub fn is_grp4(&self) -> bool {
        *self == Sel::Grp4
    }
    #[doc = "Selects test group 3"]
    #[inline(always)]
    pub fn is_grp3(&self) -> bool {
        *self == Sel::Grp3
    }
    #[doc = "Selects test group 2"]
    #[inline(always)]
    pub fn is_grp2(&self) -> bool {
        *self == Sel::Grp2
    }
    #[doc = "Selects test group 1"]
    #[inline(always)]
    pub fn is_grp1(&self) -> bool {
        *self == Sel::Grp1
    }
    #[doc = "DTB output from peripheral is 0x0."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sel::Disable
    }
}
#[doc = "Field `SEL` writer - 3:0\\]
DTB MUX select pin value"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Sel, crate::Safe>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects test group 15"]
    #[inline(always)]
    pub fn grp15(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp15)
    }
    #[doc = "Selects test group 14"]
    #[inline(always)]
    pub fn grp14(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp14)
    }
    #[doc = "Selects test group 13"]
    #[inline(always)]
    pub fn grp13(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp13)
    }
    #[doc = "Selects test group 12"]
    #[inline(always)]
    pub fn grp12(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp12)
    }
    #[doc = "Selects test group 11"]
    #[inline(always)]
    pub fn grp11(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp11)
    }
    #[doc = "Selects test group 10"]
    #[inline(always)]
    pub fn grp10(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp10)
    }
    #[doc = "Selects test group 9"]
    #[inline(always)]
    pub fn grp9(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp9)
    }
    #[doc = "Selects test group 8"]
    #[inline(always)]
    pub fn grp8(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp8)
    }
    #[doc = "Selects test group 7"]
    #[inline(always)]
    pub fn grp7(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp7)
    }
    #[doc = "Selects test group 6"]
    #[inline(always)]
    pub fn grp6(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp6)
    }
    #[doc = "Selects test group 5"]
    #[inline(always)]
    pub fn grp5(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp5)
    }
    #[doc = "Selects test group 4"]
    #[inline(always)]
    pub fn grp4(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp4)
    }
    #[doc = "Selects test group 3"]
    #[inline(always)]
    pub fn grp3(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp3)
    }
    #[doc = "Selects test group 2"]
    #[inline(always)]
    pub fn grp2(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp2)
    }
    #[doc = "Selects test group 1"]
    #[inline(always)]
    pub fn grp1(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp1)
    }
    #[doc = "DTB output from peripheral is 0x0."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Disable)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
DTB MUX select pin value"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 0x0f) as u8)
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
DTB MUX select pin value"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<DtbSpec> {
        SelW::new(self, 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<DtbSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "Digital test bus mux selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtbSpec;
impl crate::RegisterSpec for DtbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtb::R`](R) reader structure"]
impl crate::Readable for DtbSpec {}
#[doc = "`write(|w| ..)` method takes [`dtb::W`](W) writer structure"]
impl crate::Writable for DtbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTB to value 0"]
impl crate::Resettable for DtbSpec {
    const RESET_VALUE: u32 = 0;
}
