#[doc = "Register `DTBBATMON` reader"]
pub type R = crate::R<DtbbatmonSpec>;
#[doc = "Register `DTBBATMON` writer"]
pub type W = crate::W<DtbbatmonSpec>;
#[doc = "4:0\\]
DTB MUX selection signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "15: GRP15 DTB outputs"]
    Grp15 = 15,
    #[doc = "14: GRP14 DTB outputs"]
    Grp14 = 14,
    #[doc = "13: GRP13 DTB outputs"]
    Grp13 = 13,
    #[doc = "12: GRP12 DTB outputs"]
    Grp12 = 12,
    #[doc = "11: GRP11 DTB outputs"]
    Grp11 = 11,
    #[doc = "10: GRP10 DTB outputs"]
    Grp10 = 10,
    #[doc = "9: GRP9 DTB outputs"]
    Grp9 = 9,
    #[doc = "8: GRP8 DTB outputs"]
    Grp8 = 8,
    #[doc = "7: GRP7 DTB outputs"]
    Grp7 = 7,
    #[doc = "6: GRP6 DTB outputs"]
    Grp6 = 6,
    #[doc = "5: GRP5 DTB outputs"]
    Grp5 = 5,
    #[doc = "4: GRP4 DTB outputs"]
    Grp4 = 4,
    #[doc = "3: GRP3 DTB outputs"]
    Grp3 = 3,
    #[doc = "2: GRP2 DTB outputs"]
    Grp2 = 2,
    #[doc = "1: GRP1 DTB outputs"]
    Grp1 = 1,
    #[doc = "0: All DTB outputs driven to zero"]
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
#[doc = "Field `SEL` reader - 4:0\\]
DTB MUX selection signal"]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            15 => Some(Sel::Grp15),
            14 => Some(Sel::Grp14),
            13 => Some(Sel::Grp13),
            12 => Some(Sel::Grp12),
            11 => Some(Sel::Grp11),
            10 => Some(Sel::Grp10),
            9 => Some(Sel::Grp9),
            8 => Some(Sel::Grp8),
            7 => Some(Sel::Grp7),
            6 => Some(Sel::Grp6),
            5 => Some(Sel::Grp5),
            4 => Some(Sel::Grp4),
            3 => Some(Sel::Grp3),
            2 => Some(Sel::Grp2),
            1 => Some(Sel::Grp1),
            0 => Some(Sel::Disable),
            _ => None,
        }
    }
    #[doc = "GRP15 DTB outputs"]
    #[inline(always)]
    pub fn is_grp15(&self) -> bool {
        *self == Sel::Grp15
    }
    #[doc = "GRP14 DTB outputs"]
    #[inline(always)]
    pub fn is_grp14(&self) -> bool {
        *self == Sel::Grp14
    }
    #[doc = "GRP13 DTB outputs"]
    #[inline(always)]
    pub fn is_grp13(&self) -> bool {
        *self == Sel::Grp13
    }
    #[doc = "GRP12 DTB outputs"]
    #[inline(always)]
    pub fn is_grp12(&self) -> bool {
        *self == Sel::Grp12
    }
    #[doc = "GRP11 DTB outputs"]
    #[inline(always)]
    pub fn is_grp11(&self) -> bool {
        *self == Sel::Grp11
    }
    #[doc = "GRP10 DTB outputs"]
    #[inline(always)]
    pub fn is_grp10(&self) -> bool {
        *self == Sel::Grp10
    }
    #[doc = "GRP9 DTB outputs"]
    #[inline(always)]
    pub fn is_grp9(&self) -> bool {
        *self == Sel::Grp9
    }
    #[doc = "GRP8 DTB outputs"]
    #[inline(always)]
    pub fn is_grp8(&self) -> bool {
        *self == Sel::Grp8
    }
    #[doc = "GRP7 DTB outputs"]
    #[inline(always)]
    pub fn is_grp7(&self) -> bool {
        *self == Sel::Grp7
    }
    #[doc = "GRP6 DTB outputs"]
    #[inline(always)]
    pub fn is_grp6(&self) -> bool {
        *self == Sel::Grp6
    }
    #[doc = "GRP5 DTB outputs"]
    #[inline(always)]
    pub fn is_grp5(&self) -> bool {
        *self == Sel::Grp5
    }
    #[doc = "GRP4 DTB outputs"]
    #[inline(always)]
    pub fn is_grp4(&self) -> bool {
        *self == Sel::Grp4
    }
    #[doc = "GRP3 DTB outputs"]
    #[inline(always)]
    pub fn is_grp3(&self) -> bool {
        *self == Sel::Grp3
    }
    #[doc = "GRP2 DTB outputs"]
    #[inline(always)]
    pub fn is_grp2(&self) -> bool {
        *self == Sel::Grp2
    }
    #[doc = "GRP1 DTB outputs"]
    #[inline(always)]
    pub fn is_grp1(&self) -> bool {
        *self == Sel::Grp1
    }
    #[doc = "All DTB outputs driven to zero"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Sel::Disable
    }
}
#[doc = "Field `SEL` writer - 4:0\\]
DTB MUX selection signal"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 5, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GRP15 DTB outputs"]
    #[inline(always)]
    pub fn grp15(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp15)
    }
    #[doc = "GRP14 DTB outputs"]
    #[inline(always)]
    pub fn grp14(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp14)
    }
    #[doc = "GRP13 DTB outputs"]
    #[inline(always)]
    pub fn grp13(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp13)
    }
    #[doc = "GRP12 DTB outputs"]
    #[inline(always)]
    pub fn grp12(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp12)
    }
    #[doc = "GRP11 DTB outputs"]
    #[inline(always)]
    pub fn grp11(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp11)
    }
    #[doc = "GRP10 DTB outputs"]
    #[inline(always)]
    pub fn grp10(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp10)
    }
    #[doc = "GRP9 DTB outputs"]
    #[inline(always)]
    pub fn grp9(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp9)
    }
    #[doc = "GRP8 DTB outputs"]
    #[inline(always)]
    pub fn grp8(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp8)
    }
    #[doc = "GRP7 DTB outputs"]
    #[inline(always)]
    pub fn grp7(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp7)
    }
    #[doc = "GRP6 DTB outputs"]
    #[inline(always)]
    pub fn grp6(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp6)
    }
    #[doc = "GRP5 DTB outputs"]
    #[inline(always)]
    pub fn grp5(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp5)
    }
    #[doc = "GRP4 DTB outputs"]
    #[inline(always)]
    pub fn grp4(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp4)
    }
    #[doc = "GRP3 DTB outputs"]
    #[inline(always)]
    pub fn grp3(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp3)
    }
    #[doc = "GRP2 DTB outputs"]
    #[inline(always)]
    pub fn grp2(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp2)
    }
    #[doc = "GRP1 DTB outputs"]
    #[inline(always)]
    pub fn grp1(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Grp1)
    }
    #[doc = "All DTB outputs driven to zero"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Disable)
    }
}
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
DTB MUX selection signal"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
DTB MUX selection signal"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<DtbbatmonSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "BATMON DTB MUX selection signal\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtbbatmon::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtbbatmon::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtbbatmonSpec;
impl crate::RegisterSpec for DtbbatmonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtbbatmon::R`](R) reader structure"]
impl crate::Readable for DtbbatmonSpec {}
#[doc = "`write(|w| ..)` method takes [`dtbbatmon::W`](W) writer structure"]
impl crate::Writable for DtbbatmonSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTBBATMON to value 0"]
impl crate::Resettable for DtbbatmonSpec {
    const RESET_VALUE: u32 = 0;
}
