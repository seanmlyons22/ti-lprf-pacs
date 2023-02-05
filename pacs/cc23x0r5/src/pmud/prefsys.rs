#[doc = "Register `PREFSYS` reader"]
pub type R = crate::R<PrefsysSpec>;
#[doc = "Register `PREFSYS` writer"]
pub type W = crate::W<PrefsysSpec>;
#[doc = "0:0\\]
Connects 2uA IPTAT to va_atb_pmurefsys_a\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Test0 {
    #[doc = "1: Connect"]
    Set = 1,
    #[doc = "0: No connect"]
    Clr = 0,
}
impl From<Test0> for bool {
    #[inline(always)]
    fn from(variant: Test0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST0` reader - 0:0\\]
Connects 2uA IPTAT to va_atb_pmurefsys_a"]
pub type Test0R = crate::BitReader<Test0>;
impl Test0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Test0 {
        match self.bits {
            true => Test0::Set,
            false => Test0::Clr,
        }
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Test0::Set
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Test0::Clr
    }
}
#[doc = "Field `TEST0` writer - 0:0\\]
Connects 2uA IPTAT to va_atb_pmurefsys_a"]
pub type Test0W<'a, REG> = crate::BitWriter<'a, REG, Test0>;
impl<'a, REG> Test0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Test0::Set)
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Test0::Clr)
    }
}
#[doc = "1:1\\]
Connects 4uA IREF to va_atb_pmurefsys_a\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Test1 {
    #[doc = "1: Connect"]
    Set = 1,
    #[doc = "0: No connect"]
    Clr = 0,
}
impl From<Test1> for bool {
    #[inline(always)]
    fn from(variant: Test1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST1` reader - 1:1\\]
Connects 4uA IREF to va_atb_pmurefsys_a"]
pub type Test1R = crate::BitReader<Test1>;
impl Test1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Test1 {
        match self.bits {
            true => Test1::Set,
            false => Test1::Clr,
        }
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Test1::Set
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Test1::Clr
    }
}
#[doc = "Field `TEST1` writer - 1:1\\]
Connects 4uA IREF to va_atb_pmurefsys_a"]
pub type Test1W<'a, REG> = crate::BitWriter<'a, REG, Test1>;
impl<'a, REG> Test1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Test1::Set)
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Test1::Clr)
    }
}
#[doc = "2:2\\]
Connects 1uA IREF to va_atb_pmurefsys_a\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Test2 {
    #[doc = "1: Connect"]
    Set = 1,
    #[doc = "0: No connect"]
    Clr = 0,
}
impl From<Test2> for bool {
    #[inline(always)]
    fn from(variant: Test2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST2` reader - 2:2\\]
Connects 1uA IREF to va_atb_pmurefsys_a"]
pub type Test2R = crate::BitReader<Test2>;
impl Test2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Test2 {
        match self.bits {
            true => Test2::Set,
            false => Test2::Clr,
        }
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Test2::Set
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Test2::Clr
    }
}
#[doc = "Field `TEST2` writer - 2:2\\]
Connects 1uA IREF to va_atb_pmurefsys_a"]
pub type Test2W<'a, REG> = crate::BitWriter<'a, REG, Test2>;
impl<'a, REG> Test2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Test2::Set)
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Test2::Clr)
    }
}
#[doc = "3:3\\]
Connects 20uA IREF to va_atb_pmurefsys_a\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Test3 {
    #[doc = "1: Connect"]
    Set = 1,
    #[doc = "0: No connect"]
    Clr = 0,
}
impl From<Test3> for bool {
    #[inline(always)]
    fn from(variant: Test3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST3` reader - 3:3\\]
Connects 20uA IREF to va_atb_pmurefsys_a"]
pub type Test3R = crate::BitReader<Test3>;
impl Test3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Test3 {
        match self.bits {
            true => Test3::Set,
            false => Test3::Clr,
        }
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Test3::Set
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Test3::Clr
    }
}
#[doc = "Field `TEST3` writer - 3:3\\]
Connects 20uA IREF to va_atb_pmurefsys_a"]
pub type Test3W<'a, REG> = crate::BitWriter<'a, REG, Test3>;
impl<'a, REG> Test3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Test3::Set)
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Test3::Clr)
    }
}
#[doc = "4:4\\]
Connects buffered bandgap output to va_atb_pmurefsys_a\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Test4 {
    #[doc = "1: Connect"]
    Set = 1,
    #[doc = "0: No connect"]
    Clr = 0,
}
impl From<Test4> for bool {
    #[inline(always)]
    fn from(variant: Test4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST4` reader - 4:4\\]
Connects buffered bandgap output to va_atb_pmurefsys_a"]
pub type Test4R = crate::BitReader<Test4>;
impl Test4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Test4 {
        match self.bits {
            true => Test4::Set,
            false => Test4::Clr,
        }
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Test4::Set
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Test4::Clr
    }
}
#[doc = "Field `TEST4` writer - 4:4\\]
Connects buffered bandgap output to va_atb_pmurefsys_a"]
pub type Test4W<'a, REG> = crate::BitWriter<'a, REG, Test4>;
impl<'a, REG> Test4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Test4::Set)
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Test4::Clr)
    }
}
#[doc = "5:5\\]
Connects unbuffered bandgap output to va_atb_pmurefsys_a\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Test5 {
    #[doc = "1: Connect"]
    Set = 1,
    #[doc = "0: No connect"]
    Clr = 0,
}
impl From<Test5> for bool {
    #[inline(always)]
    fn from(variant: Test5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST5` reader - 5:5\\]
Connects unbuffered bandgap output to va_atb_pmurefsys_a"]
pub type Test5R = crate::BitReader<Test5>;
impl Test5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Test5 {
        match self.bits {
            true => Test5::Set,
            false => Test5::Clr,
        }
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Test5::Set
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Test5::Clr
    }
}
#[doc = "Field `TEST5` writer - 5:5\\]
Connects unbuffered bandgap output to va_atb_pmurefsys_a"]
pub type Test5W<'a, REG> = crate::BitWriter<'a, REG, Test5>;
impl<'a, REG> Test5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Test5::Set)
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Test5::Clr)
    }
}
#[doc = "6:6\\]
Connects 0.8V VREF for LRF and HFXT to va_atb_pmurefsys_a\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Test6 {
    #[doc = "1: Connect"]
    Set = 1,
    #[doc = "0: No connect"]
    Clr = 0,
}
impl From<Test6> for bool {
    #[inline(always)]
    fn from(variant: Test6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST6` reader - 6:6\\]
Connects 0.8V VREF for LRF and HFXT to va_atb_pmurefsys_a"]
pub type Test6R = crate::BitReader<Test6>;
impl Test6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Test6 {
        match self.bits {
            true => Test6::Set,
            false => Test6::Clr,
        }
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Test6::Set
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Test6::Clr
    }
}
#[doc = "Field `TEST6` writer - 6:6\\]
Connects 0.8V VREF for LRF and HFXT to va_atb_pmurefsys_a"]
pub type Test6W<'a, REG> = crate::BitWriter<'a, REG, Test6>;
impl<'a, REG> Test6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Test6::Set)
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Test6::Clr)
    }
}
#[doc = "7:7\\]
Connects 0.8V VREF for pmui2v circuit (IREF) to va_atb_pmurefsys_a\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Test7 {
    #[doc = "1: Connect"]
    Set = 1,
    #[doc = "0: No connect"]
    Clr = 0,
}
impl From<Test7> for bool {
    #[inline(always)]
    fn from(variant: Test7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST7` reader - 7:7\\]
Connects 0.8V VREF for pmui2v circuit (IREF) to va_atb_pmurefsys_a"]
pub type Test7R = crate::BitReader<Test7>;
impl Test7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Test7 {
        match self.bits {
            true => Test7::Set,
            false => Test7::Clr,
        }
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Test7::Set
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Test7::Clr
    }
}
#[doc = "Field `TEST7` writer - 7:7\\]
Connects 0.8V VREF for pmui2v circuit (IREF) to va_atb_pmurefsys_a"]
pub type Test7W<'a, REG> = crate::BitWriter<'a, REG, Test7>;
impl<'a, REG> Test7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Test7::Set)
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Test7::Clr)
    }
}
#[doc = "8:8\\]
Connects BMON comparator output to va_atb_pmurefsys_a\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Test8 {
    #[doc = "1: Connect"]
    Set = 1,
    #[doc = "0: No connect"]
    Clr = 0,
}
impl From<Test8> for bool {
    #[inline(always)]
    fn from(variant: Test8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST8` reader - 8:8\\]
Connects BMON comparator output to va_atb_pmurefsys_a"]
pub type Test8R = crate::BitReader<Test8>;
impl Test8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Test8 {
        match self.bits {
            true => Test8::Set,
            false => Test8::Clr,
        }
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Test8::Set
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Test8::Clr
    }
}
#[doc = "Field `TEST8` writer - 8:8\\]
Connects BMON comparator output to va_atb_pmurefsys_a"]
pub type Test8W<'a, REG> = crate::BitWriter<'a, REG, Test8>;
impl<'a, REG> Test8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Test8::Set)
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Test8::Clr)
    }
}
#[doc = "9:9\\]
Connects BMON comparator input to va_atb_pmurefsys_a\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Test9 {
    #[doc = "1: Connect"]
    Set = 1,
    #[doc = "0: No connect"]
    Clr = 0,
}
impl From<Test9> for bool {
    #[inline(always)]
    fn from(variant: Test9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST9` reader - 9:9\\]
Connects BMON comparator input to va_atb_pmurefsys_a"]
pub type Test9R = crate::BitReader<Test9>;
impl Test9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Test9 {
        match self.bits {
            true => Test9::Set,
            false => Test9::Clr,
        }
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Test9::Set
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Test9::Clr
    }
}
#[doc = "Field `TEST9` writer - 9:9\\]
Connects BMON comparator input to va_atb_pmurefsys_a"]
pub type Test9W<'a, REG> = crate::BitWriter<'a, REG, Test9>;
impl<'a, REG> Test9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Test9::Set)
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Test9::Clr)
    }
}
#[doc = "10:10\\]
Connects 1uA IPTAT going to BMON to va_atb_pmurefsys_a\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Test10 {
    #[doc = "1: Connect"]
    Set = 1,
    #[doc = "0: No connect"]
    Clr = 0,
}
impl From<Test10> for bool {
    #[inline(always)]
    fn from(variant: Test10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEST10` reader - 10:10\\]
Connects 1uA IPTAT going to BMON to va_atb_pmurefsys_a"]
pub type Test10R = crate::BitReader<Test10>;
impl Test10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Test10 {
        match self.bits {
            true => Test10::Set,
            false => Test10::Clr,
        }
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Test10::Set
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Test10::Clr
    }
}
#[doc = "Field `TEST10` writer - 10:10\\]
Connects 1uA IPTAT going to BMON to va_atb_pmurefsys_a"]
pub type Test10W<'a, REG> = crate::BitWriter<'a, REG, Test10>;
impl<'a, REG> Test10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Test10::Set)
    }
    #[doc = "No connect"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Test10::Clr)
    }
}
#[doc = "Field `SPARE` reader - 15:11\\]
Spare bits. BATMON comparator enable in test mode is based on COMPTEST.EN bit."]
pub type SpareR = crate::FieldReader;
#[doc = "Field `SPARE` writer - 15:11\\]
Spare bits. BATMON comparator enable in test mode is based on COMPTEST.EN bit."]
pub type SpareW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Connects 2uA IPTAT to va_atb_pmurefsys_a"]
    #[inline(always)]
    pub fn test0(&self) -> Test0R {
        Test0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Connects 4uA IREF to va_atb_pmurefsys_a"]
    #[inline(always)]
    pub fn test1(&self) -> Test1R {
        Test1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Connects 1uA IREF to va_atb_pmurefsys_a"]
    #[inline(always)]
    pub fn test2(&self) -> Test2R {
        Test2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Connects 20uA IREF to va_atb_pmurefsys_a"]
    #[inline(always)]
    pub fn test3(&self) -> Test3R {
        Test3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Connects buffered bandgap output to va_atb_pmurefsys_a"]
    #[inline(always)]
    pub fn test4(&self) -> Test4R {
        Test4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Connects unbuffered bandgap output to va_atb_pmurefsys_a"]
    #[inline(always)]
    pub fn test5(&self) -> Test5R {
        Test5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Connects 0.8V VREF for LRF and HFXT to va_atb_pmurefsys_a"]
    #[inline(always)]
    pub fn test6(&self) -> Test6R {
        Test6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Connects 0.8V VREF for pmui2v circuit (IREF) to va_atb_pmurefsys_a"]
    #[inline(always)]
    pub fn test7(&self) -> Test7R {
        Test7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Connects BMON comparator output to va_atb_pmurefsys_a"]
    #[inline(always)]
    pub fn test8(&self) -> Test8R {
        Test8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Connects BMON comparator input to va_atb_pmurefsys_a"]
    #[inline(always)]
    pub fn test9(&self) -> Test9R {
        Test9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Connects 1uA IPTAT going to BMON to va_atb_pmurefsys_a"]
    #[inline(always)]
    pub fn test10(&self) -> Test10R {
        Test10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Spare bits. BATMON comparator enable in test mode is based on COMPTEST.EN bit."]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Connects 2uA IPTAT to va_atb_pmurefsys_a"]
    #[inline(always)]
    #[must_use]
    pub fn test0(&mut self) -> Test0W<PrefsysSpec> {
        Test0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Connects 4uA IREF to va_atb_pmurefsys_a"]
    #[inline(always)]
    #[must_use]
    pub fn test1(&mut self) -> Test1W<PrefsysSpec> {
        Test1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Connects 1uA IREF to va_atb_pmurefsys_a"]
    #[inline(always)]
    #[must_use]
    pub fn test2(&mut self) -> Test2W<PrefsysSpec> {
        Test2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Connects 20uA IREF to va_atb_pmurefsys_a"]
    #[inline(always)]
    #[must_use]
    pub fn test3(&mut self) -> Test3W<PrefsysSpec> {
        Test3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Connects buffered bandgap output to va_atb_pmurefsys_a"]
    #[inline(always)]
    #[must_use]
    pub fn test4(&mut self) -> Test4W<PrefsysSpec> {
        Test4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Connects unbuffered bandgap output to va_atb_pmurefsys_a"]
    #[inline(always)]
    #[must_use]
    pub fn test5(&mut self) -> Test5W<PrefsysSpec> {
        Test5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Connects 0.8V VREF for LRF and HFXT to va_atb_pmurefsys_a"]
    #[inline(always)]
    #[must_use]
    pub fn test6(&mut self) -> Test6W<PrefsysSpec> {
        Test6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Connects 0.8V VREF for pmui2v circuit (IREF) to va_atb_pmurefsys_a"]
    #[inline(always)]
    #[must_use]
    pub fn test7(&mut self) -> Test7W<PrefsysSpec> {
        Test7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Connects BMON comparator output to va_atb_pmurefsys_a"]
    #[inline(always)]
    #[must_use]
    pub fn test8(&mut self) -> Test8W<PrefsysSpec> {
        Test8W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Connects BMON comparator input to va_atb_pmurefsys_a"]
    #[inline(always)]
    #[must_use]
    pub fn test9(&mut self) -> Test9W<PrefsysSpec> {
        Test9W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Connects 1uA IPTAT going to BMON to va_atb_pmurefsys_a"]
    #[inline(always)]
    #[must_use]
    pub fn test10(&mut self) -> Test10W<PrefsysSpec> {
        Test10W::new(self, 10)
    }
    #[doc = "Bits 11:15 - 15:11\\]
Spare bits. BATMON comparator enable in test mode is based on COMPTEST.EN bit."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<PrefsysSpec> {
        SpareW::new(self, 11)
    }
}
#[doc = "PMU REFSYS test register. These test bits connect to PMU REFSYS analog module directly. Note: This register is write-protected except for bits \\[3:1\\]
based on global lock signal from SYS0 on production devices.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prefsys::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prefsys::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrefsysSpec;
impl crate::RegisterSpec for PrefsysSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prefsys::R`](R) reader structure"]
impl crate::Readable for PrefsysSpec {}
#[doc = "`write(|w| ..)` method takes [`prefsys::W`](W) writer structure"]
impl crate::Writable for PrefsysSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PREFSYS to value 0"]
impl crate::Resettable for PrefsysSpec {
    const RESET_VALUE: u32 = 0;
}
