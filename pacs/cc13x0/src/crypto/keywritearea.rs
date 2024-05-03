#[doc = "Register `KEYWRITEAREA` reader"]
pub type R = crate::R<KeywriteareaSpec>;
#[doc = "Register `KEYWRITEAREA` writer"]
pub type W = crate::W<KeywriteareaSpec>;
#[doc = "0:0\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamArea0 {
    #[doc = "1: This RAM area is selected to be written"]
    Sel = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NotSel = 0,
}
impl From<RamArea0> for bool {
    #[inline(always)]
    fn from(variant: RamArea0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA0` reader - 0:0\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea0R = crate::BitReader<RamArea0>;
impl RamArea0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamArea0 {
        match self.bits {
            true => RamArea0::Sel,
            false => RamArea0::NotSel,
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RamArea0::Sel
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RamArea0::NotSel
    }
}
#[doc = "Field `RAM_AREA0` writer - 0:0\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea0W<'a, REG> = crate::BitWriter<'a, REG, RamArea0>;
impl<'a, REG> RamArea0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea0::Sel)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea0::NotSel)
    }
}
#[doc = "1:1\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamArea1 {
    #[doc = "1: This RAM area is selected to be written"]
    Sel = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NotSel = 0,
}
impl From<RamArea1> for bool {
    #[inline(always)]
    fn from(variant: RamArea1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA1` reader - 1:1\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea1R = crate::BitReader<RamArea1>;
impl RamArea1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamArea1 {
        match self.bits {
            true => RamArea1::Sel,
            false => RamArea1::NotSel,
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RamArea1::Sel
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RamArea1::NotSel
    }
}
#[doc = "Field `RAM_AREA1` writer - 1:1\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea1W<'a, REG> = crate::BitWriter<'a, REG, RamArea1>;
impl<'a, REG> RamArea1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea1::Sel)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea1::NotSel)
    }
}
#[doc = "2:2\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamArea2 {
    #[doc = "1: This RAM area is selected to be written"]
    Sel = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NotSel = 0,
}
impl From<RamArea2> for bool {
    #[inline(always)]
    fn from(variant: RamArea2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA2` reader - 2:2\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea2R = crate::BitReader<RamArea2>;
impl RamArea2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamArea2 {
        match self.bits {
            true => RamArea2::Sel,
            false => RamArea2::NotSel,
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RamArea2::Sel
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RamArea2::NotSel
    }
}
#[doc = "Field `RAM_AREA2` writer - 2:2\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea2W<'a, REG> = crate::BitWriter<'a, REG, RamArea2>;
impl<'a, REG> RamArea2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea2::Sel)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea2::NotSel)
    }
}
#[doc = "3:3\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamArea3 {
    #[doc = "1: This RAM area is selected to be written"]
    Sel = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NotSel = 0,
}
impl From<RamArea3> for bool {
    #[inline(always)]
    fn from(variant: RamArea3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA3` reader - 3:3\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea3R = crate::BitReader<RamArea3>;
impl RamArea3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamArea3 {
        match self.bits {
            true => RamArea3::Sel,
            false => RamArea3::NotSel,
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RamArea3::Sel
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RamArea3::NotSel
    }
}
#[doc = "Field `RAM_AREA3` writer - 3:3\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea3W<'a, REG> = crate::BitWriter<'a, REG, RamArea3>;
impl<'a, REG> RamArea3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea3::Sel)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea3::NotSel)
    }
}
#[doc = "4:4\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamArea4 {
    #[doc = "1: This RAM area is selected to be written"]
    Sel = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NotSel = 0,
}
impl From<RamArea4> for bool {
    #[inline(always)]
    fn from(variant: RamArea4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA4` reader - 4:4\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea4R = crate::BitReader<RamArea4>;
impl RamArea4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamArea4 {
        match self.bits {
            true => RamArea4::Sel,
            false => RamArea4::NotSel,
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RamArea4::Sel
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RamArea4::NotSel
    }
}
#[doc = "Field `RAM_AREA4` writer - 4:4\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea4W<'a, REG> = crate::BitWriter<'a, REG, RamArea4>;
impl<'a, REG> RamArea4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea4::Sel)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea4::NotSel)
    }
}
#[doc = "5:5\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamArea5 {
    #[doc = "1: This RAM area is selected to be written"]
    Sel = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NotSel = 0,
}
impl From<RamArea5> for bool {
    #[inline(always)]
    fn from(variant: RamArea5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA5` reader - 5:5\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea5R = crate::BitReader<RamArea5>;
impl RamArea5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamArea5 {
        match self.bits {
            true => RamArea5::Sel,
            false => RamArea5::NotSel,
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RamArea5::Sel
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RamArea5::NotSel
    }
}
#[doc = "Field `RAM_AREA5` writer - 5:5\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea5W<'a, REG> = crate::BitWriter<'a, REG, RamArea5>;
impl<'a, REG> RamArea5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea5::Sel)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea5::NotSel)
    }
}
#[doc = "6:6\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamArea6 {
    #[doc = "1: This RAM area is selected to be written"]
    Sel = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NotSel = 0,
}
impl From<RamArea6> for bool {
    #[inline(always)]
    fn from(variant: RamArea6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA6` reader - 6:6\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea6R = crate::BitReader<RamArea6>;
impl RamArea6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamArea6 {
        match self.bits {
            true => RamArea6::Sel,
            false => RamArea6::NotSel,
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RamArea6::Sel
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RamArea6::NotSel
    }
}
#[doc = "Field `RAM_AREA6` writer - 6:6\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea6W<'a, REG> = crate::BitWriter<'a, REG, RamArea6>;
impl<'a, REG> RamArea6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea6::Sel)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea6::NotSel)
    }
}
#[doc = "7:7\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RamArea7 {
    #[doc = "1: This RAM area is selected to be written"]
    Sel = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NotSel = 0,
}
impl From<RamArea7> for bool {
    #[inline(always)]
    fn from(variant: RamArea7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM_AREA7` reader - 7:7\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea7R = crate::BitReader<RamArea7>;
impl RamArea7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RamArea7 {
        match self.bits {
            true => RamArea7::Sel,
            false => RamArea7::NotSel,
        }
    }
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RamArea7::Sel
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RamArea7::NotSel
    }
}
#[doc = "Field `RAM_AREA7` writer - 7:7\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RamArea7W<'a, REG> = crate::BitWriter<'a, REG, RamArea7>;
impl<'a, REG> RamArea7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea7::Sel)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea7::NotSel)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area0(&self) -> RamArea0R {
        RamArea0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area1(&self) -> RamArea1R {
        RamArea1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area2(&self) -> RamArea2R {
        RamArea2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area3(&self) -> RamArea3R {
        RamArea3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area4(&self) -> RamArea4R {
        RamArea4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area5(&self) -> RamArea5R {
        RamArea5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area6(&self) -> RamArea6R {
        RamArea6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area7(&self) -> RamArea7R {
        RamArea7R::new(((self.bits >> 7) & 1) != 0)
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
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area0(&mut self) -> RamArea0W<KeywriteareaSpec> {
        RamArea0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area1(&mut self) -> RamArea1W<KeywriteareaSpec> {
        RamArea1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area2(&mut self) -> RamArea2W<KeywriteareaSpec> {
        RamArea2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area3(&mut self) -> RamArea3W<KeywriteareaSpec> {
        RamArea3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area4(&mut self) -> RamArea4W<KeywriteareaSpec> {
        RamArea4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area5(&mut self) -> RamArea5W<KeywriteareaSpec> {
        RamArea5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area6(&mut self) -> RamArea6W<KeywriteareaSpec> {
        RamArea6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area7(&mut self) -> RamArea7W<KeywriteareaSpec> {
        RamArea7W::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<KeywriteareaSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Key Write Area\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keywritearea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keywritearea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeywriteareaSpec;
impl crate::RegisterSpec for KeywriteareaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keywritearea::R`](R) reader structure"]
impl crate::Readable for KeywriteareaSpec {}
#[doc = "`write(|w| ..)` method takes [`keywritearea::W`](W) writer structure"]
impl crate::Writable for KeywriteareaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYWRITEAREA to value 0"]
impl crate::Resettable for KeywriteareaSpec {
    const RESET_VALUE: u32 = 0;
}
