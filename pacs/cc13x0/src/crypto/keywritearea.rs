#[doc = "Register `KEYWRITEAREA` reader"]
pub struct R(crate::R<KEYWRITEAREA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYWRITEAREA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYWRITEAREA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYWRITEAREA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYWRITEAREA` writer"]
pub struct W(crate::W<KEYWRITEAREA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYWRITEAREA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<KEYWRITEAREA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYWRITEAREA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM_AREA0` reader - 0:0\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA0_R = crate::BitReader<RAM_AREA0_A>;
#[doc = "0:0\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM_AREA0_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA0_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA0_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM_AREA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA0_A {
        match self.bits {
            true => RAM_AREA0_A::SEL,
            false => RAM_AREA0_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA0_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA0_A::NOT_SEL
    }
}
#[doc = "Field `RAM_AREA0` writer - 0:0\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEYWRITEAREA_SPEC, RAM_AREA0_A, O>;
impl<'a, const O: u8> RAM_AREA0_W<'a, O> {
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA0_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA0_A::NOT_SEL)
    }
}
#[doc = "Field `RAM_AREA1` reader - 1:1\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA1_R = crate::BitReader<RAM_AREA1_A>;
#[doc = "1:1\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM_AREA1_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA1_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA1_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM_AREA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA1_A {
        match self.bits {
            true => RAM_AREA1_A::SEL,
            false => RAM_AREA1_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA1_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA1_A::NOT_SEL
    }
}
#[doc = "Field `RAM_AREA1` writer - 1:1\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEYWRITEAREA_SPEC, RAM_AREA1_A, O>;
impl<'a, const O: u8> RAM_AREA1_W<'a, O> {
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA1_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA1_A::NOT_SEL)
    }
}
#[doc = "Field `RAM_AREA2` reader - 2:2\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA2_R = crate::BitReader<RAM_AREA2_A>;
#[doc = "2:2\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM_AREA2_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA2_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA2_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM_AREA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA2_A {
        match self.bits {
            true => RAM_AREA2_A::SEL,
            false => RAM_AREA2_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA2_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA2_A::NOT_SEL
    }
}
#[doc = "Field `RAM_AREA2` writer - 2:2\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEYWRITEAREA_SPEC, RAM_AREA2_A, O>;
impl<'a, const O: u8> RAM_AREA2_W<'a, O> {
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA2_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA2_A::NOT_SEL)
    }
}
#[doc = "Field `RAM_AREA3` reader - 3:3\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA3_R = crate::BitReader<RAM_AREA3_A>;
#[doc = "3:3\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM_AREA3_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA3_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA3_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM_AREA3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA3_A {
        match self.bits {
            true => RAM_AREA3_A::SEL,
            false => RAM_AREA3_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA3_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA3_A::NOT_SEL
    }
}
#[doc = "Field `RAM_AREA3` writer - 3:3\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEYWRITEAREA_SPEC, RAM_AREA3_A, O>;
impl<'a, const O: u8> RAM_AREA3_W<'a, O> {
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA3_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA3_A::NOT_SEL)
    }
}
#[doc = "Field `RAM_AREA4` reader - 4:4\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA4_R = crate::BitReader<RAM_AREA4_A>;
#[doc = "4:4\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM_AREA4_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA4_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA4_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM_AREA4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA4_A {
        match self.bits {
            true => RAM_AREA4_A::SEL,
            false => RAM_AREA4_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA4_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA4_A::NOT_SEL
    }
}
#[doc = "Field `RAM_AREA4` writer - 4:4\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEYWRITEAREA_SPEC, RAM_AREA4_A, O>;
impl<'a, const O: u8> RAM_AREA4_W<'a, O> {
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA4_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA4_A::NOT_SEL)
    }
}
#[doc = "Field `RAM_AREA5` reader - 5:5\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA5_R = crate::BitReader<RAM_AREA5_A>;
#[doc = "5:5\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM_AREA5_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA5_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA5_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM_AREA5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA5_A {
        match self.bits {
            true => RAM_AREA5_A::SEL,
            false => RAM_AREA5_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA5_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA5_A::NOT_SEL
    }
}
#[doc = "Field `RAM_AREA5` writer - 5:5\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEYWRITEAREA_SPEC, RAM_AREA5_A, O>;
impl<'a, const O: u8> RAM_AREA5_W<'a, O> {
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA5_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA5_A::NOT_SEL)
    }
}
#[doc = "Field `RAM_AREA6` reader - 6:6\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA6_R = crate::BitReader<RAM_AREA6_A>;
#[doc = "6:6\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM_AREA6_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA6_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA6_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM_AREA6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA6_A {
        match self.bits {
            true => RAM_AREA6_A::SEL,
            false => RAM_AREA6_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA6_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA6_A::NOT_SEL
    }
}
#[doc = "Field `RAM_AREA6` writer - 6:6\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEYWRITEAREA_SPEC, RAM_AREA6_A, O>;
impl<'a, const O: u8> RAM_AREA6_W<'a, O> {
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA6_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA6_A::NOT_SEL)
    }
}
#[doc = "Field `RAM_AREA7` reader - 7:7\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA7_R = crate::BitReader<RAM_AREA7_A>;
#[doc = "7:7\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM_AREA7_A {
    #[doc = "1: This RAM area is selected to be written"]
    SEL = 1,
    #[doc = "0: This RAM area is not selected to be written"]
    NOT_SEL = 0,
}
impl From<RAM_AREA7_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_AREA7_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM_AREA7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_AREA7_A {
        match self.bits {
            true => RAM_AREA7_A::SEL,
            false => RAM_AREA7_A::NOT_SEL,
        }
    }
    #[doc = "Checks if the value of the field is `SEL`"]
    #[inline(always)]
    pub fn is_sel(&self) -> bool {
        *self == RAM_AREA7_A::SEL
    }
    #[doc = "Checks if the value of the field is `NOT_SEL`"]
    #[inline(always)]
    pub fn is_not_sel(&self) -> bool {
        *self == RAM_AREA7_A::NOT_SEL
    }
}
#[doc = "Field `RAM_AREA7` writer - 7:7\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
pub type RAM_AREA7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEYWRITEAREA_SPEC, RAM_AREA7_A, O>;
impl<'a, const O: u8> RAM_AREA7_W<'a, O> {
    #[doc = "This RAM area is selected to be written"]
    #[inline(always)]
    pub fn sel(self) -> &'a mut W {
        self.variant(RAM_AREA7_A::SEL)
    }
    #[doc = "This RAM area is not selected to be written"]
    #[inline(always)]
    pub fn not_sel(self) -> &'a mut W {
        self.variant(RAM_AREA7_A::NOT_SEL)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, KEYWRITEAREA_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area0(&self) -> RAM_AREA0_R {
        RAM_AREA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area1(&self) -> RAM_AREA1_R {
        RAM_AREA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area2(&self) -> RAM_AREA2_R {
        RAM_AREA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area3(&self) -> RAM_AREA3_R {
        RAM_AREA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area4(&self) -> RAM_AREA4_R {
        RAM_AREA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area5(&self) -> RAM_AREA5_R {
        RAM_AREA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area6(&self) -> RAM_AREA6_R {
        RAM_AREA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    pub fn ram_area7(&self) -> RAM_AREA7_R {
        RAM_AREA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area0(&mut self) -> RAM_AREA0_W<0> {
        RAM_AREA0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area1(&mut self) -> RAM_AREA1_W<1> {
        RAM_AREA1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area2(&mut self) -> RAM_AREA2_W<2> {
        RAM_AREA2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area3(&mut self) -> RAM_AREA3_W<3> {
        RAM_AREA3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area4(&mut self) -> RAM_AREA4_W<4> {
        RAM_AREA4_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area5(&mut self) -> RAM_AREA5_W<5> {
        RAM_AREA5_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area6(&mut self) -> RAM_AREA6_W<6> {
        RAM_AREA6_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written. Writing to multiple RAM locations is only possible when the selected RAM areas are sequential."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area7(&mut self) -> RAM_AREA7_W<7> {
        RAM_AREA7_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key Write Area\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keywritearea](index.html) module"]
pub struct KEYWRITEAREA_SPEC;
impl crate::RegisterSpec for KEYWRITEAREA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keywritearea::R](R) reader structure"]
impl crate::Readable for KEYWRITEAREA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keywritearea::W](W) writer structure"]
impl crate::Writable for KEYWRITEAREA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYWRITEAREA to value 0"]
impl crate::Resettable for KEYWRITEAREA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
