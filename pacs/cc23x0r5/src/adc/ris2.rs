#[doc = "Register `RIS2` reader"]
pub type R = crate::R<Ris2Spec>;
#[doc = "Register `RIS2` writer"]
pub type W = crate::W<Ris2Spec>;
#[doc = "Field `RESERVED0` reader - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "8:8\\]
Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg0 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg0> for bool {
    #[inline(always)]
    fn from(variant: Memresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG0` reader - 8:8\\]
Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg0R = crate::BitReader<Memresifg0>;
impl Memresifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg0 {
        match self.bits {
            true => Memresifg0::Set,
            false => Memresifg0::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg0::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg0::Clr
    }
}
#[doc = "Field `MEMRESIFG0` writer - 8:8\\]
Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg0W<'a, REG> = crate::BitWriter<'a, REG, Memresifg0>;
impl<'a, REG> Memresifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg0::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg0::Clr)
    }
}
#[doc = "9:9\\]
Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg1 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg1> for bool {
    #[inline(always)]
    fn from(variant: Memresifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG1` reader - 9:9\\]
Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg1R = crate::BitReader<Memresifg1>;
impl Memresifg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg1 {
        match self.bits {
            true => Memresifg1::Set,
            false => Memresifg1::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg1::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg1::Clr
    }
}
#[doc = "Field `MEMRESIFG1` writer - 9:9\\]
Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg1W<'a, REG> = crate::BitWriter<'a, REG, Memresifg1>;
impl<'a, REG> Memresifg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg1::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg1::Clr)
    }
}
#[doc = "10:10\\]
Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg2 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg2> for bool {
    #[inline(always)]
    fn from(variant: Memresifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG2` reader - 10:10\\]
Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg2R = crate::BitReader<Memresifg2>;
impl Memresifg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg2 {
        match self.bits {
            true => Memresifg2::Set,
            false => Memresifg2::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg2::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg2::Clr
    }
}
#[doc = "Field `MEMRESIFG2` writer - 10:10\\]
Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg2W<'a, REG> = crate::BitWriter<'a, REG, Memresifg2>;
impl<'a, REG> Memresifg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg2::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg2::Clr)
    }
}
#[doc = "11:11\\]
Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg3 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg3> for bool {
    #[inline(always)]
    fn from(variant: Memresifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG3` reader - 11:11\\]
Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg3R = crate::BitReader<Memresifg3>;
impl Memresifg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg3 {
        match self.bits {
            true => Memresifg3::Set,
            false => Memresifg3::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg3::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg3::Clr
    }
}
#[doc = "Field `MEMRESIFG3` writer - 11:11\\]
Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg3W<'a, REG> = crate::BitWriter<'a, REG, Memresifg3>;
impl<'a, REG> Memresifg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg3::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg3::Clr)
    }
}
#[doc = "12:12\\]
Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg4 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg4> for bool {
    #[inline(always)]
    fn from(variant: Memresifg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG4` reader - 12:12\\]
Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg4R = crate::BitReader<Memresifg4>;
impl Memresifg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg4 {
        match self.bits {
            true => Memresifg4::Set,
            false => Memresifg4::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg4::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg4::Clr
    }
}
#[doc = "Field `MEMRESIFG4` writer - 12:12\\]
Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg4W<'a, REG> = crate::BitWriter<'a, REG, Memresifg4>;
impl<'a, REG> Memresifg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg4::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg4::Clr)
    }
}
#[doc = "13:13\\]
Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg5 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg5> for bool {
    #[inline(always)]
    fn from(variant: Memresifg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG5` reader - 13:13\\]
Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg5R = crate::BitReader<Memresifg5>;
impl Memresifg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg5 {
        match self.bits {
            true => Memresifg5::Set,
            false => Memresifg5::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg5::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg5::Clr
    }
}
#[doc = "Field `MEMRESIFG5` writer - 13:13\\]
Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg5W<'a, REG> = crate::BitWriter<'a, REG, Memresifg5>;
impl<'a, REG> Memresifg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg5::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg5::Clr)
    }
}
#[doc = "14:14\\]
Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg6 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg6> for bool {
    #[inline(always)]
    fn from(variant: Memresifg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG6` reader - 14:14\\]
Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg6R = crate::BitReader<Memresifg6>;
impl Memresifg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg6 {
        match self.bits {
            true => Memresifg6::Set,
            false => Memresifg6::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg6::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg6::Clr
    }
}
#[doc = "Field `MEMRESIFG6` writer - 14:14\\]
Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg6W<'a, REG> = crate::BitWriter<'a, REG, Memresifg6>;
impl<'a, REG> Memresifg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg6::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg6::Clr)
    }
}
#[doc = "15:15\\]
Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg7 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg7> for bool {
    #[inline(always)]
    fn from(variant: Memresifg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG7` reader - 15:15\\]
Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg7R = crate::BitReader<Memresifg7>;
impl Memresifg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg7 {
        match self.bits {
            true => Memresifg7::Set,
            false => Memresifg7::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg7::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg7::Clr
    }
}
#[doc = "Field `MEMRESIFG7` writer - 15:15\\]
Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg7W<'a, REG> = crate::BitWriter<'a, REG, Memresifg7>;
impl<'a, REG> Memresifg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg7::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg7::Clr)
    }
}
#[doc = "16:16\\]
Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg8 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg8> for bool {
    #[inline(always)]
    fn from(variant: Memresifg8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG8` reader - 16:16\\]
Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg8R = crate::BitReader<Memresifg8>;
impl Memresifg8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg8 {
        match self.bits {
            true => Memresifg8::Set,
            false => Memresifg8::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg8::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg8::Clr
    }
}
#[doc = "Field `MEMRESIFG8` writer - 16:16\\]
Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg8W<'a, REG> = crate::BitWriter<'a, REG, Memresifg8>;
impl<'a, REG> Memresifg8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg8::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg8::Clr)
    }
}
#[doc = "17:17\\]
Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg9 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg9> for bool {
    #[inline(always)]
    fn from(variant: Memresifg9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG9` reader - 17:17\\]
Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg9R = crate::BitReader<Memresifg9>;
impl Memresifg9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg9 {
        match self.bits {
            true => Memresifg9::Set,
            false => Memresifg9::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg9::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg9::Clr
    }
}
#[doc = "Field `MEMRESIFG9` writer - 17:17\\]
Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg9W<'a, REG> = crate::BitWriter<'a, REG, Memresifg9>;
impl<'a, REG> Memresifg9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg9::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg9::Clr)
    }
}
#[doc = "18:18\\]
Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg10 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg10> for bool {
    #[inline(always)]
    fn from(variant: Memresifg10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG10` reader - 18:18\\]
Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg10R = crate::BitReader<Memresifg10>;
impl Memresifg10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg10 {
        match self.bits {
            true => Memresifg10::Set,
            false => Memresifg10::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg10::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg10::Clr
    }
}
#[doc = "Field `MEMRESIFG10` writer - 18:18\\]
Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg10W<'a, REG> = crate::BitWriter<'a, REG, Memresifg10>;
impl<'a, REG> Memresifg10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg10::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg10::Clr)
    }
}
#[doc = "19:19\\]
Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg11 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg11> for bool {
    #[inline(always)]
    fn from(variant: Memresifg11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG11` reader - 19:19\\]
Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg11R = crate::BitReader<Memresifg11>;
impl Memresifg11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg11 {
        match self.bits {
            true => Memresifg11::Set,
            false => Memresifg11::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg11::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg11::Clr
    }
}
#[doc = "Field `MEMRESIFG11` writer - 19:19\\]
Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg11W<'a, REG> = crate::BitWriter<'a, REG, Memresifg11>;
impl<'a, REG> Memresifg11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg11::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg11::Clr)
    }
}
#[doc = "20:20\\]
Raw interrupt status for MEMRES12. This bit is set to 1 when MEMRES12 is loaded with a new conversion result. Reading MEMRES12 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg12 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg12> for bool {
    #[inline(always)]
    fn from(variant: Memresifg12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG12` reader - 20:20\\]
Raw interrupt status for MEMRES12. This bit is set to 1 when MEMRES12 is loaded with a new conversion result. Reading MEMRES12 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg12R = crate::BitReader<Memresifg12>;
impl Memresifg12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg12 {
        match self.bits {
            true => Memresifg12::Set,
            false => Memresifg12::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg12::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg12::Clr
    }
}
#[doc = "Field `MEMRESIFG12` writer - 20:20\\]
Raw interrupt status for MEMRES12. This bit is set to 1 when MEMRES12 is loaded with a new conversion result. Reading MEMRES12 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg12W<'a, REG> = crate::BitWriter<'a, REG, Memresifg12>;
impl<'a, REG> Memresifg12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg12::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg12::Clr)
    }
}
#[doc = "21:21\\]
Raw interrupt status for MEMRES13. This bit is set to 1 when MEMRES13 is loaded with a new conversion result. Reading MEMRES13 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg13 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg13> for bool {
    #[inline(always)]
    fn from(variant: Memresifg13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG13` reader - 21:21\\]
Raw interrupt status for MEMRES13. This bit is set to 1 when MEMRES13 is loaded with a new conversion result. Reading MEMRES13 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg13R = crate::BitReader<Memresifg13>;
impl Memresifg13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg13 {
        match self.bits {
            true => Memresifg13::Set,
            false => Memresifg13::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg13::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg13::Clr
    }
}
#[doc = "Field `MEMRESIFG13` writer - 21:21\\]
Raw interrupt status for MEMRES13. This bit is set to 1 when MEMRES13 is loaded with a new conversion result. Reading MEMRES13 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg13W<'a, REG> = crate::BitWriter<'a, REG, Memresifg13>;
impl<'a, REG> Memresifg13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg13::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg13::Clr)
    }
}
#[doc = "22:22\\]
Raw interrupt status for MEMRES14. This bit is set to 1 when MEMRES14 is loaded with a new conversion result. Reading MEMRES14 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg14 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg14> for bool {
    #[inline(always)]
    fn from(variant: Memresifg14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG14` reader - 22:22\\]
Raw interrupt status for MEMRES14. This bit is set to 1 when MEMRES14 is loaded with a new conversion result. Reading MEMRES14 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg14R = crate::BitReader<Memresifg14>;
impl Memresifg14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg14 {
        match self.bits {
            true => Memresifg14::Set,
            false => Memresifg14::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg14::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg14::Clr
    }
}
#[doc = "Field `MEMRESIFG14` writer - 22:22\\]
Raw interrupt status for MEMRES14. This bit is set to 1 when MEMRES14 is loaded with a new conversion result. Reading MEMRES14 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg14W<'a, REG> = crate::BitWriter<'a, REG, Memresifg14>;
impl<'a, REG> Memresifg14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg14::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg14::Clr)
    }
}
#[doc = "23:23\\]
Raw interrupt status for MEMRES15. This bit is set to 1 when MEMRES15 is loaded with a new conversion result. Reading MEMRES15 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg15 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg15> for bool {
    #[inline(always)]
    fn from(variant: Memresifg15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG15` reader - 23:23\\]
Raw interrupt status for MEMRES15. This bit is set to 1 when MEMRES15 is loaded with a new conversion result. Reading MEMRES15 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg15R = crate::BitReader<Memresifg15>;
impl Memresifg15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg15 {
        match self.bits {
            true => Memresifg15::Set,
            false => Memresifg15::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg15::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg15::Clr
    }
}
#[doc = "Field `MEMRESIFG15` writer - 23:23\\]
Raw interrupt status for MEMRES15. This bit is set to 1 when MEMRES15 is loaded with a new conversion result. Reading MEMRES15 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg15W<'a, REG> = crate::BitWriter<'a, REG, Memresifg15>;
impl<'a, REG> Memresifg15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg15::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg15::Clr)
    }
}
#[doc = "24:24\\]
Raw interrupt status for MEMRES16. This bit is set to 1 when MEMRES16 is loaded with a new conversion result. Reading MEMRES16 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg16 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg16> for bool {
    #[inline(always)]
    fn from(variant: Memresifg16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG16` reader - 24:24\\]
Raw interrupt status for MEMRES16. This bit is set to 1 when MEMRES16 is loaded with a new conversion result. Reading MEMRES16 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg16R = crate::BitReader<Memresifg16>;
impl Memresifg16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg16 {
        match self.bits {
            true => Memresifg16::Set,
            false => Memresifg16::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg16::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg16::Clr
    }
}
#[doc = "Field `MEMRESIFG16` writer - 24:24\\]
Raw interrupt status for MEMRES16. This bit is set to 1 when MEMRES16 is loaded with a new conversion result. Reading MEMRES16 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg16W<'a, REG> = crate::BitWriter<'a, REG, Memresifg16>;
impl<'a, REG> Memresifg16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg16::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg16::Clr)
    }
}
#[doc = "25:25\\]
Raw interrupt status for MEMRES17. This bit is set to 1 when MEMRES17 is loaded with a new conversion result. Reading MEMRES17 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg17 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg17> for bool {
    #[inline(always)]
    fn from(variant: Memresifg17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG17` reader - 25:25\\]
Raw interrupt status for MEMRES17. This bit is set to 1 when MEMRES17 is loaded with a new conversion result. Reading MEMRES17 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg17R = crate::BitReader<Memresifg17>;
impl Memresifg17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg17 {
        match self.bits {
            true => Memresifg17::Set,
            false => Memresifg17::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg17::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg17::Clr
    }
}
#[doc = "Field `MEMRESIFG17` writer - 25:25\\]
Raw interrupt status for MEMRES17. This bit is set to 1 when MEMRES17 is loaded with a new conversion result. Reading MEMRES17 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg17W<'a, REG> = crate::BitWriter<'a, REG, Memresifg17>;
impl<'a, REG> Memresifg17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg17::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg17::Clr)
    }
}
#[doc = "26:26\\]
Raw interrupt status for MEMRES18. This bit is set to 1 when MEMRES18 is loaded with a new conversion result. Reading MEMRES18 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg18 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg18> for bool {
    #[inline(always)]
    fn from(variant: Memresifg18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG18` reader - 26:26\\]
Raw interrupt status for MEMRES18. This bit is set to 1 when MEMRES18 is loaded with a new conversion result. Reading MEMRES18 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg18R = crate::BitReader<Memresifg18>;
impl Memresifg18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg18 {
        match self.bits {
            true => Memresifg18::Set,
            false => Memresifg18::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg18::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg18::Clr
    }
}
#[doc = "Field `MEMRESIFG18` writer - 26:26\\]
Raw interrupt status for MEMRES18. This bit is set to 1 when MEMRES18 is loaded with a new conversion result. Reading MEMRES18 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg18W<'a, REG> = crate::BitWriter<'a, REG, Memresifg18>;
impl<'a, REG> Memresifg18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg18::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg18::Clr)
    }
}
#[doc = "27:27\\]
Raw interrupt status for MEMRES19. This bit is set to 1 when MEMRES19 is loaded with a new conversion result. Reading MEMRES19 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg19 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg19> for bool {
    #[inline(always)]
    fn from(variant: Memresifg19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG19` reader - 27:27\\]
Raw interrupt status for MEMRES19. This bit is set to 1 when MEMRES19 is loaded with a new conversion result. Reading MEMRES19 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg19R = crate::BitReader<Memresifg19>;
impl Memresifg19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg19 {
        match self.bits {
            true => Memresifg19::Set,
            false => Memresifg19::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg19::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg19::Clr
    }
}
#[doc = "Field `MEMRESIFG19` writer - 27:27\\]
Raw interrupt status for MEMRES19. This bit is set to 1 when MEMRES19 is loaded with a new conversion result. Reading MEMRES19 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg19W<'a, REG> = crate::BitWriter<'a, REG, Memresifg19>;
impl<'a, REG> Memresifg19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg19::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg19::Clr)
    }
}
#[doc = "28:28\\]
Raw interrupt status for MEMRES20. This bit is set to 1 when MEMRES20 is loaded with a new conversion result. Reading MEMRES20 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg20 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg20> for bool {
    #[inline(always)]
    fn from(variant: Memresifg20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG20` reader - 28:28\\]
Raw interrupt status for MEMRES20. This bit is set to 1 when MEMRES20 is loaded with a new conversion result. Reading MEMRES20 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg20R = crate::BitReader<Memresifg20>;
impl Memresifg20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg20 {
        match self.bits {
            true => Memresifg20::Set,
            false => Memresifg20::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg20::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg20::Clr
    }
}
#[doc = "Field `MEMRESIFG20` writer - 28:28\\]
Raw interrupt status for MEMRES20. This bit is set to 1 when MEMRES20 is loaded with a new conversion result. Reading MEMRES20 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg20W<'a, REG> = crate::BitWriter<'a, REG, Memresifg20>;
impl<'a, REG> Memresifg20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg20::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg20::Clr)
    }
}
#[doc = "29:29\\]
Raw interrupt status for MEMRES21. This bit is set to 1 when MEMRES21 is loaded with a new conversion result. Reading MEMRES21 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg21 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg21> for bool {
    #[inline(always)]
    fn from(variant: Memresifg21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG21` reader - 29:29\\]
Raw interrupt status for MEMRES21. This bit is set to 1 when MEMRES21 is loaded with a new conversion result. Reading MEMRES21 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg21R = crate::BitReader<Memresifg21>;
impl Memresifg21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg21 {
        match self.bits {
            true => Memresifg21::Set,
            false => Memresifg21::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg21::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg21::Clr
    }
}
#[doc = "Field `MEMRESIFG21` writer - 29:29\\]
Raw interrupt status for MEMRES21. This bit is set to 1 when MEMRES21 is loaded with a new conversion result. Reading MEMRES21 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg21W<'a, REG> = crate::BitWriter<'a, REG, Memresifg21>;
impl<'a, REG> Memresifg21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg21::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg21::Clr)
    }
}
#[doc = "30:30\\]
Raw interrupt status for MEMRES22. This bit is set to 1 when MEMRES22 is loaded with a new conversion result. Reading MEMRES22 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg22 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg22> for bool {
    #[inline(always)]
    fn from(variant: Memresifg22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG22` reader - 30:30\\]
Raw interrupt status for MEMRES22. This bit is set to 1 when MEMRES22 is loaded with a new conversion result. Reading MEMRES22 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg22R = crate::BitReader<Memresifg22>;
impl Memresifg22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg22 {
        match self.bits {
            true => Memresifg22::Set,
            false => Memresifg22::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg22::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg22::Clr
    }
}
#[doc = "Field `MEMRESIFG22` writer - 30:30\\]
Raw interrupt status for MEMRES22. This bit is set to 1 when MEMRES22 is loaded with a new conversion result. Reading MEMRES22 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg22W<'a, REG> = crate::BitWriter<'a, REG, Memresifg22>;
impl<'a, REG> Memresifg22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg22::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg22::Clr)
    }
}
#[doc = "31:31\\]
Raw interrupt status for MEMRES23. This bit is set to 1 when MEMRES23 is loaded with a new conversion result. Reading MEMRES23 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Memresifg23 {
    #[doc = "1: A new data is ready to be read."]
    Set = 1,
    #[doc = "0: No new data ready."]
    Clr = 0,
}
impl From<Memresifg23> for bool {
    #[inline(always)]
    fn from(variant: Memresifg23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMRESIFG23` reader - 31:31\\]
Raw interrupt status for MEMRES23. This bit is set to 1 when MEMRES23 is loaded with a new conversion result. Reading MEMRES23 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg23R = crate::BitReader<Memresifg23>;
impl Memresifg23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Memresifg23 {
        match self.bits {
            true => Memresifg23::Set,
            false => Memresifg23::Clr,
        }
    }
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Memresifg23::Set
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Memresifg23::Clr
    }
}
#[doc = "Field `MEMRESIFG23` writer - 31:31\\]
Raw interrupt status for MEMRES23. This bit is set to 1 when MEMRES23 is loaded with a new conversion result. Reading MEMRES23 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type Memresifg23W<'a, REG> = crate::BitWriter<'a, REG, Memresifg23>;
impl<'a, REG> Memresifg23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A new data is ready to be read."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg23::Set)
    }
    #[doc = "No new data ready."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Memresifg23::Clr)
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg0(&self) -> Memresifg0R {
        Memresifg0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg1(&self) -> Memresifg1R {
        Memresifg1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg2(&self) -> Memresifg2R {
        Memresifg2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg3(&self) -> Memresifg3R {
        Memresifg3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg4(&self) -> Memresifg4R {
        Memresifg4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg5(&self) -> Memresifg5R {
        Memresifg5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg6(&self) -> Memresifg6R {
        Memresifg6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg7(&self) -> Memresifg7R {
        Memresifg7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg8(&self) -> Memresifg8R {
        Memresifg8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg9(&self) -> Memresifg9R {
        Memresifg9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg10(&self) -> Memresifg10R {
        Memresifg10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg11(&self) -> Memresifg11R {
        Memresifg11R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Raw interrupt status for MEMRES12. This bit is set to 1 when MEMRES12 is loaded with a new conversion result. Reading MEMRES12 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg12(&self) -> Memresifg12R {
        Memresifg12R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Raw interrupt status for MEMRES13. This bit is set to 1 when MEMRES13 is loaded with a new conversion result. Reading MEMRES13 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg13(&self) -> Memresifg13R {
        Memresifg13R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Raw interrupt status for MEMRES14. This bit is set to 1 when MEMRES14 is loaded with a new conversion result. Reading MEMRES14 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg14(&self) -> Memresifg14R {
        Memresifg14R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Raw interrupt status for MEMRES15. This bit is set to 1 when MEMRES15 is loaded with a new conversion result. Reading MEMRES15 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg15(&self) -> Memresifg15R {
        Memresifg15R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Raw interrupt status for MEMRES16. This bit is set to 1 when MEMRES16 is loaded with a new conversion result. Reading MEMRES16 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg16(&self) -> Memresifg16R {
        Memresifg16R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Raw interrupt status for MEMRES17. This bit is set to 1 when MEMRES17 is loaded with a new conversion result. Reading MEMRES17 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg17(&self) -> Memresifg17R {
        Memresifg17R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Raw interrupt status for MEMRES18. This bit is set to 1 when MEMRES18 is loaded with a new conversion result. Reading MEMRES18 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg18(&self) -> Memresifg18R {
        Memresifg18R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Raw interrupt status for MEMRES19. This bit is set to 1 when MEMRES19 is loaded with a new conversion result. Reading MEMRES19 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg19(&self) -> Memresifg19R {
        Memresifg19R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Raw interrupt status for MEMRES20. This bit is set to 1 when MEMRES20 is loaded with a new conversion result. Reading MEMRES20 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg20(&self) -> Memresifg20R {
        Memresifg20R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Raw interrupt status for MEMRES21. This bit is set to 1 when MEMRES21 is loaded with a new conversion result. Reading MEMRES21 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg21(&self) -> Memresifg21R {
        Memresifg21R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Raw interrupt status for MEMRES22. This bit is set to 1 when MEMRES22 is loaded with a new conversion result. Reading MEMRES22 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg22(&self) -> Memresifg22R {
        Memresifg22R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Raw interrupt status for MEMRES23. This bit is set to 1 when MEMRES23 is loaded with a new conversion result. Reading MEMRES23 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn memresifg23(&self) -> Memresifg23R {
        Memresifg23R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<Ris2Spec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg0(&mut self) -> Memresifg0W<Ris2Spec> {
        Memresifg0W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg1(&mut self) -> Memresifg1W<Ris2Spec> {
        Memresifg1W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg2(&mut self) -> Memresifg2W<Ris2Spec> {
        Memresifg2W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg3(&mut self) -> Memresifg3W<Ris2Spec> {
        Memresifg3W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg4(&mut self) -> Memresifg4W<Ris2Spec> {
        Memresifg4W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg5(&mut self) -> Memresifg5W<Ris2Spec> {
        Memresifg5W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg6(&mut self) -> Memresifg6W<Ris2Spec> {
        Memresifg6W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg7(&mut self) -> Memresifg7W<Ris2Spec> {
        Memresifg7W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg8(&mut self) -> Memresifg8W<Ris2Spec> {
        Memresifg8W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg9(&mut self) -> Memresifg9W<Ris2Spec> {
        Memresifg9W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg10(&mut self) -> Memresifg10W<Ris2Spec> {
        Memresifg10W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg11(&mut self) -> Memresifg11W<Ris2Spec> {
        Memresifg11W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Raw interrupt status for MEMRES12. This bit is set to 1 when MEMRES12 is loaded with a new conversion result. Reading MEMRES12 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg12(&mut self) -> Memresifg12W<Ris2Spec> {
        Memresifg12W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Raw interrupt status for MEMRES13. This bit is set to 1 when MEMRES13 is loaded with a new conversion result. Reading MEMRES13 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg13(&mut self) -> Memresifg13W<Ris2Spec> {
        Memresifg13W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Raw interrupt status for MEMRES14. This bit is set to 1 when MEMRES14 is loaded with a new conversion result. Reading MEMRES14 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg14(&mut self) -> Memresifg14W<Ris2Spec> {
        Memresifg14W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Raw interrupt status for MEMRES15. This bit is set to 1 when MEMRES15 is loaded with a new conversion result. Reading MEMRES15 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg15(&mut self) -> Memresifg15W<Ris2Spec> {
        Memresifg15W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Raw interrupt status for MEMRES16. This bit is set to 1 when MEMRES16 is loaded with a new conversion result. Reading MEMRES16 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg16(&mut self) -> Memresifg16W<Ris2Spec> {
        Memresifg16W::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Raw interrupt status for MEMRES17. This bit is set to 1 when MEMRES17 is loaded with a new conversion result. Reading MEMRES17 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg17(&mut self) -> Memresifg17W<Ris2Spec> {
        Memresifg17W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Raw interrupt status for MEMRES18. This bit is set to 1 when MEMRES18 is loaded with a new conversion result. Reading MEMRES18 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg18(&mut self) -> Memresifg18W<Ris2Spec> {
        Memresifg18W::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Raw interrupt status for MEMRES19. This bit is set to 1 when MEMRES19 is loaded with a new conversion result. Reading MEMRES19 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg19(&mut self) -> Memresifg19W<Ris2Spec> {
        Memresifg19W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Raw interrupt status for MEMRES20. This bit is set to 1 when MEMRES20 is loaded with a new conversion result. Reading MEMRES20 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg20(&mut self) -> Memresifg20W<Ris2Spec> {
        Memresifg20W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Raw interrupt status for MEMRES21. This bit is set to 1 when MEMRES21 is loaded with a new conversion result. Reading MEMRES21 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg21(&mut self) -> Memresifg21W<Ris2Spec> {
        Memresifg21W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Raw interrupt status for MEMRES22. This bit is set to 1 when MEMRES22 is loaded with a new conversion result. Reading MEMRES22 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg22(&mut self) -> Memresifg22W<Ris2Spec> {
        Memresifg22W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Raw interrupt status for MEMRES23. This bit is set to 1 when MEMRES23 is loaded with a new conversion result. Reading MEMRES23 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn memresifg23(&mut self) -> Memresifg23W<Ris2Spec> {
        Memresifg23W::new(self, 31)
    }
}
#[doc = "Raw interrupt status. Reflects all pending interrupts, regardless of masking. The RIS2 register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ris2Spec;
impl crate::RegisterSpec for Ris2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris2::R`](R) reader structure"]
impl crate::Readable for Ris2Spec {}
#[doc = "`write(|w| ..)` method takes [`ris2::W`](W) writer structure"]
impl crate::Writable for Ris2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RIS2 to value 0"]
impl crate::Resettable for Ris2Spec {
    const RESET_VALUE: u32 = 0;
}
