#[doc = "Register `DOE31_0` reader"]
pub type R = crate::R<Doe31_0Spec>;
#[doc = "Register `DOE31_0` writer"]
pub type W = crate::W<Doe31_0Spec>;
#[doc = "0:0\\]
Data output enable for DIO0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio0 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio0> for bool {
    #[inline(always)]
    fn from(variant: Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO0` reader - 0:0\\]
Data output enable for DIO0"]
pub type Dio0R = crate::BitReader<Dio0>;
impl Dio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio0 {
        match self.bits {
            true => Dio0::Enable,
            false => Dio0::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio0::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio0::Disable
    }
}
#[doc = "Field `DIO0` writer - 0:0\\]
Data output enable for DIO0"]
pub type Dio0W<'a, REG> = crate::BitWriter<'a, REG, Dio0>;
impl<'a, REG> Dio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio0::Disable)
    }
}
#[doc = "1:1\\]
Data output enable for DIO1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio1 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio1> for bool {
    #[inline(always)]
    fn from(variant: Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO1` reader - 1:1\\]
Data output enable for DIO1"]
pub type Dio1R = crate::BitReader<Dio1>;
impl Dio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio1 {
        match self.bits {
            true => Dio1::Enable,
            false => Dio1::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio1::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio1::Disable
    }
}
#[doc = "Field `DIO1` writer - 1:1\\]
Data output enable for DIO1"]
pub type Dio1W<'a, REG> = crate::BitWriter<'a, REG, Dio1>;
impl<'a, REG> Dio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio1::Disable)
    }
}
#[doc = "2:2\\]
Data output enable for DIO2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio2 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio2> for bool {
    #[inline(always)]
    fn from(variant: Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO2` reader - 2:2\\]
Data output enable for DIO2"]
pub type Dio2R = crate::BitReader<Dio2>;
impl Dio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio2 {
        match self.bits {
            true => Dio2::Enable,
            false => Dio2::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio2::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio2::Disable
    }
}
#[doc = "Field `DIO2` writer - 2:2\\]
Data output enable for DIO2"]
pub type Dio2W<'a, REG> = crate::BitWriter<'a, REG, Dio2>;
impl<'a, REG> Dio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio2::Disable)
    }
}
#[doc = "3:3\\]
Data output enable for DIO3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio3 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio3> for bool {
    #[inline(always)]
    fn from(variant: Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO3` reader - 3:3\\]
Data output enable for DIO3"]
pub type Dio3R = crate::BitReader<Dio3>;
impl Dio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio3 {
        match self.bits {
            true => Dio3::Enable,
            false => Dio3::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio3::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio3::Disable
    }
}
#[doc = "Field `DIO3` writer - 3:3\\]
Data output enable for DIO3"]
pub type Dio3W<'a, REG> = crate::BitWriter<'a, REG, Dio3>;
impl<'a, REG> Dio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio3::Disable)
    }
}
#[doc = "4:4\\]
Data output enable for DIO4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio4 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio4> for bool {
    #[inline(always)]
    fn from(variant: Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO4` reader - 4:4\\]
Data output enable for DIO4"]
pub type Dio4R = crate::BitReader<Dio4>;
impl Dio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio4 {
        match self.bits {
            true => Dio4::Enable,
            false => Dio4::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio4::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio4::Disable
    }
}
#[doc = "Field `DIO4` writer - 4:4\\]
Data output enable for DIO4"]
pub type Dio4W<'a, REG> = crate::BitWriter<'a, REG, Dio4>;
impl<'a, REG> Dio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio4::Disable)
    }
}
#[doc = "5:5\\]
Data output enable for DIO5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio5 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio5> for bool {
    #[inline(always)]
    fn from(variant: Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO5` reader - 5:5\\]
Data output enable for DIO5"]
pub type Dio5R = crate::BitReader<Dio5>;
impl Dio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio5 {
        match self.bits {
            true => Dio5::Enable,
            false => Dio5::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio5::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio5::Disable
    }
}
#[doc = "Field `DIO5` writer - 5:5\\]
Data output enable for DIO5"]
pub type Dio5W<'a, REG> = crate::BitWriter<'a, REG, Dio5>;
impl<'a, REG> Dio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio5::Disable)
    }
}
#[doc = "6:6\\]
Data output enable for DIO6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio6 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio6> for bool {
    #[inline(always)]
    fn from(variant: Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO6` reader - 6:6\\]
Data output enable for DIO6"]
pub type Dio6R = crate::BitReader<Dio6>;
impl Dio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio6 {
        match self.bits {
            true => Dio6::Enable,
            false => Dio6::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio6::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio6::Disable
    }
}
#[doc = "Field `DIO6` writer - 6:6\\]
Data output enable for DIO6"]
pub type Dio6W<'a, REG> = crate::BitWriter<'a, REG, Dio6>;
impl<'a, REG> Dio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio6::Disable)
    }
}
#[doc = "7:7\\]
Data output enable for DIO7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio7 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio7> for bool {
    #[inline(always)]
    fn from(variant: Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO7` reader - 7:7\\]
Data output enable for DIO7"]
pub type Dio7R = crate::BitReader<Dio7>;
impl Dio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio7 {
        match self.bits {
            true => Dio7::Enable,
            false => Dio7::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio7::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio7::Disable
    }
}
#[doc = "Field `DIO7` writer - 7:7\\]
Data output enable for DIO7"]
pub type Dio7W<'a, REG> = crate::BitWriter<'a, REG, Dio7>;
impl<'a, REG> Dio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio7::Disable)
    }
}
#[doc = "8:8\\]
Data output enable for DIO8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio8 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio8> for bool {
    #[inline(always)]
    fn from(variant: Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO8` reader - 8:8\\]
Data output enable for DIO8"]
pub type Dio8R = crate::BitReader<Dio8>;
impl Dio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio8 {
        match self.bits {
            true => Dio8::Enable,
            false => Dio8::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio8::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio8::Disable
    }
}
#[doc = "Field `DIO8` writer - 8:8\\]
Data output enable for DIO8"]
pub type Dio8W<'a, REG> = crate::BitWriter<'a, REG, Dio8>;
impl<'a, REG> Dio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio8::Disable)
    }
}
#[doc = "9:9\\]
Data output enable for DIO9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio9 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio9> for bool {
    #[inline(always)]
    fn from(variant: Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO9` reader - 9:9\\]
Data output enable for DIO9"]
pub type Dio9R = crate::BitReader<Dio9>;
impl Dio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio9 {
        match self.bits {
            true => Dio9::Enable,
            false => Dio9::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio9::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio9::Disable
    }
}
#[doc = "Field `DIO9` writer - 9:9\\]
Data output enable for DIO9"]
pub type Dio9W<'a, REG> = crate::BitWriter<'a, REG, Dio9>;
impl<'a, REG> Dio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio9::Disable)
    }
}
#[doc = "10:10\\]
Data output enable for DIO10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio10 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio10> for bool {
    #[inline(always)]
    fn from(variant: Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO10` reader - 10:10\\]
Data output enable for DIO10"]
pub type Dio10R = crate::BitReader<Dio10>;
impl Dio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio10 {
        match self.bits {
            true => Dio10::Enable,
            false => Dio10::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio10::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio10::Disable
    }
}
#[doc = "Field `DIO10` writer - 10:10\\]
Data output enable for DIO10"]
pub type Dio10W<'a, REG> = crate::BitWriter<'a, REG, Dio10>;
impl<'a, REG> Dio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio10::Disable)
    }
}
#[doc = "11:11\\]
Data output enable for DIO11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio11 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio11> for bool {
    #[inline(always)]
    fn from(variant: Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO11` reader - 11:11\\]
Data output enable for DIO11"]
pub type Dio11R = crate::BitReader<Dio11>;
impl Dio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio11 {
        match self.bits {
            true => Dio11::Enable,
            false => Dio11::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio11::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio11::Disable
    }
}
#[doc = "Field `DIO11` writer - 11:11\\]
Data output enable for DIO11"]
pub type Dio11W<'a, REG> = crate::BitWriter<'a, REG, Dio11>;
impl<'a, REG> Dio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio11::Disable)
    }
}
#[doc = "12:12\\]
Data output enable for DIO12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio12 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio12> for bool {
    #[inline(always)]
    fn from(variant: Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO12` reader - 12:12\\]
Data output enable for DIO12"]
pub type Dio12R = crate::BitReader<Dio12>;
impl Dio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio12 {
        match self.bits {
            true => Dio12::Enable,
            false => Dio12::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio12::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio12::Disable
    }
}
#[doc = "Field `DIO12` writer - 12:12\\]
Data output enable for DIO12"]
pub type Dio12W<'a, REG> = crate::BitWriter<'a, REG, Dio12>;
impl<'a, REG> Dio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio12::Disable)
    }
}
#[doc = "13:13\\]
Data output enable for DIO13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio13 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio13> for bool {
    #[inline(always)]
    fn from(variant: Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO13` reader - 13:13\\]
Data output enable for DIO13"]
pub type Dio13R = crate::BitReader<Dio13>;
impl Dio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio13 {
        match self.bits {
            true => Dio13::Enable,
            false => Dio13::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio13::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio13::Disable
    }
}
#[doc = "Field `DIO13` writer - 13:13\\]
Data output enable for DIO13"]
pub type Dio13W<'a, REG> = crate::BitWriter<'a, REG, Dio13>;
impl<'a, REG> Dio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio13::Disable)
    }
}
#[doc = "14:14\\]
Data output enable for DIO14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio14 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio14> for bool {
    #[inline(always)]
    fn from(variant: Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO14` reader - 14:14\\]
Data output enable for DIO14"]
pub type Dio14R = crate::BitReader<Dio14>;
impl Dio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio14 {
        match self.bits {
            true => Dio14::Enable,
            false => Dio14::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio14::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio14::Disable
    }
}
#[doc = "Field `DIO14` writer - 14:14\\]
Data output enable for DIO14"]
pub type Dio14W<'a, REG> = crate::BitWriter<'a, REG, Dio14>;
impl<'a, REG> Dio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio14::Disable)
    }
}
#[doc = "15:15\\]
Data output enable for DIO15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio15 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio15> for bool {
    #[inline(always)]
    fn from(variant: Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO15` reader - 15:15\\]
Data output enable for DIO15"]
pub type Dio15R = crate::BitReader<Dio15>;
impl Dio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio15 {
        match self.bits {
            true => Dio15::Enable,
            false => Dio15::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio15::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio15::Disable
    }
}
#[doc = "Field `DIO15` writer - 15:15\\]
Data output enable for DIO15"]
pub type Dio15W<'a, REG> = crate::BitWriter<'a, REG, Dio15>;
impl<'a, REG> Dio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio15::Disable)
    }
}
#[doc = "16:16\\]
Data output enable for DIO16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio16 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio16> for bool {
    #[inline(always)]
    fn from(variant: Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO16` reader - 16:16\\]
Data output enable for DIO16"]
pub type Dio16R = crate::BitReader<Dio16>;
impl Dio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio16 {
        match self.bits {
            true => Dio16::Enable,
            false => Dio16::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio16::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio16::Disable
    }
}
#[doc = "Field `DIO16` writer - 16:16\\]
Data output enable for DIO16"]
pub type Dio16W<'a, REG> = crate::BitWriter<'a, REG, Dio16>;
impl<'a, REG> Dio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio16::Disable)
    }
}
#[doc = "17:17\\]
Data output enable for DIO17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio17 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio17> for bool {
    #[inline(always)]
    fn from(variant: Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO17` reader - 17:17\\]
Data output enable for DIO17"]
pub type Dio17R = crate::BitReader<Dio17>;
impl Dio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio17 {
        match self.bits {
            true => Dio17::Enable,
            false => Dio17::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio17::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio17::Disable
    }
}
#[doc = "Field `DIO17` writer - 17:17\\]
Data output enable for DIO17"]
pub type Dio17W<'a, REG> = crate::BitWriter<'a, REG, Dio17>;
impl<'a, REG> Dio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio17::Disable)
    }
}
#[doc = "18:18\\]
Data output enable for DIO18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio18 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio18> for bool {
    #[inline(always)]
    fn from(variant: Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO18` reader - 18:18\\]
Data output enable for DIO18"]
pub type Dio18R = crate::BitReader<Dio18>;
impl Dio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio18 {
        match self.bits {
            true => Dio18::Enable,
            false => Dio18::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio18::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio18::Disable
    }
}
#[doc = "Field `DIO18` writer - 18:18\\]
Data output enable for DIO18"]
pub type Dio18W<'a, REG> = crate::BitWriter<'a, REG, Dio18>;
impl<'a, REG> Dio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio18::Disable)
    }
}
#[doc = "19:19\\]
Data output enable for DIO19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio19 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio19> for bool {
    #[inline(always)]
    fn from(variant: Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO19` reader - 19:19\\]
Data output enable for DIO19"]
pub type Dio19R = crate::BitReader<Dio19>;
impl Dio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio19 {
        match self.bits {
            true => Dio19::Enable,
            false => Dio19::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio19::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio19::Disable
    }
}
#[doc = "Field `DIO19` writer - 19:19\\]
Data output enable for DIO19"]
pub type Dio19W<'a, REG> = crate::BitWriter<'a, REG, Dio19>;
impl<'a, REG> Dio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio19::Disable)
    }
}
#[doc = "20:20\\]
Data output enable for DIO20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio20 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio20> for bool {
    #[inline(always)]
    fn from(variant: Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO20` reader - 20:20\\]
Data output enable for DIO20"]
pub type Dio20R = crate::BitReader<Dio20>;
impl Dio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio20 {
        match self.bits {
            true => Dio20::Enable,
            false => Dio20::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio20::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio20::Disable
    }
}
#[doc = "Field `DIO20` writer - 20:20\\]
Data output enable for DIO20"]
pub type Dio20W<'a, REG> = crate::BitWriter<'a, REG, Dio20>;
impl<'a, REG> Dio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio20::Disable)
    }
}
#[doc = "21:21\\]
Data output enable for DIO21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio21 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio21> for bool {
    #[inline(always)]
    fn from(variant: Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO21` reader - 21:21\\]
Data output enable for DIO21"]
pub type Dio21R = crate::BitReader<Dio21>;
impl Dio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio21 {
        match self.bits {
            true => Dio21::Enable,
            false => Dio21::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio21::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio21::Disable
    }
}
#[doc = "Field `DIO21` writer - 21:21\\]
Data output enable for DIO21"]
pub type Dio21W<'a, REG> = crate::BitWriter<'a, REG, Dio21>;
impl<'a, REG> Dio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio21::Disable)
    }
}
#[doc = "22:22\\]
Data output enable for DIO22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio22 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio22> for bool {
    #[inline(always)]
    fn from(variant: Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO22` reader - 22:22\\]
Data output enable for DIO22"]
pub type Dio22R = crate::BitReader<Dio22>;
impl Dio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio22 {
        match self.bits {
            true => Dio22::Enable,
            false => Dio22::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio22::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio22::Disable
    }
}
#[doc = "Field `DIO22` writer - 22:22\\]
Data output enable for DIO22"]
pub type Dio22W<'a, REG> = crate::BitWriter<'a, REG, Dio22>;
impl<'a, REG> Dio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio22::Disable)
    }
}
#[doc = "23:23\\]
Data output enable for DIO23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio23 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio23> for bool {
    #[inline(always)]
    fn from(variant: Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO23` reader - 23:23\\]
Data output enable for DIO23"]
pub type Dio23R = crate::BitReader<Dio23>;
impl Dio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio23 {
        match self.bits {
            true => Dio23::Enable,
            false => Dio23::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio23::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio23::Disable
    }
}
#[doc = "Field `DIO23` writer - 23:23\\]
Data output enable for DIO23"]
pub type Dio23W<'a, REG> = crate::BitWriter<'a, REG, Dio23>;
impl<'a, REG> Dio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio23::Disable)
    }
}
#[doc = "24:24\\]
Data output enable for DIO24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio24 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio24> for bool {
    #[inline(always)]
    fn from(variant: Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO24` reader - 24:24\\]
Data output enable for DIO24"]
pub type Dio24R = crate::BitReader<Dio24>;
impl Dio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio24 {
        match self.bits {
            true => Dio24::Enable,
            false => Dio24::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio24::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio24::Disable
    }
}
#[doc = "Field `DIO24` writer - 24:24\\]
Data output enable for DIO24"]
pub type Dio24W<'a, REG> = crate::BitWriter<'a, REG, Dio24>;
impl<'a, REG> Dio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio24::Disable)
    }
}
#[doc = "25:25\\]
Data output enable for DIO25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dio25 {
    #[doc = "1: Output enabled"]
    Enable = 1,
    #[doc = "0: Output disabled"]
    Disable = 0,
}
impl From<Dio25> for bool {
    #[inline(always)]
    fn from(variant: Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIO25` reader - 25:25\\]
Data output enable for DIO25"]
pub type Dio25R = crate::BitReader<Dio25>;
impl Dio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dio25 {
        match self.bits {
            true => Dio25::Enable,
            false => Dio25::Disable,
        }
    }
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dio25::Enable
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dio25::Disable
    }
}
#[doc = "Field `DIO25` writer - 25:25\\]
Data output enable for DIO25"]
pub type Dio25W<'a, REG> = crate::BitWriter<'a, REG, Dio25>;
impl<'a, REG> Dio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::Enable)
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dio25::Disable)
    }
}
#[doc = "Field `RESERVED26` reader - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved26R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data output enable for DIO0"]
    #[inline(always)]
    pub fn dio0(&self) -> Dio0R {
        Dio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Data output enable for DIO1"]
    #[inline(always)]
    pub fn dio1(&self) -> Dio1R {
        Dio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Data output enable for DIO2"]
    #[inline(always)]
    pub fn dio2(&self) -> Dio2R {
        Dio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Data output enable for DIO3"]
    #[inline(always)]
    pub fn dio3(&self) -> Dio3R {
        Dio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Data output enable for DIO4"]
    #[inline(always)]
    pub fn dio4(&self) -> Dio4R {
        Dio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Data output enable for DIO5"]
    #[inline(always)]
    pub fn dio5(&self) -> Dio5R {
        Dio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Data output enable for DIO6"]
    #[inline(always)]
    pub fn dio6(&self) -> Dio6R {
        Dio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Data output enable for DIO7"]
    #[inline(always)]
    pub fn dio7(&self) -> Dio7R {
        Dio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Data output enable for DIO8"]
    #[inline(always)]
    pub fn dio8(&self) -> Dio8R {
        Dio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Data output enable for DIO9"]
    #[inline(always)]
    pub fn dio9(&self) -> Dio9R {
        Dio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Data output enable for DIO10"]
    #[inline(always)]
    pub fn dio10(&self) -> Dio10R {
        Dio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Data output enable for DIO11"]
    #[inline(always)]
    pub fn dio11(&self) -> Dio11R {
        Dio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Data output enable for DIO12"]
    #[inline(always)]
    pub fn dio12(&self) -> Dio12R {
        Dio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Data output enable for DIO13"]
    #[inline(always)]
    pub fn dio13(&self) -> Dio13R {
        Dio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Data output enable for DIO14"]
    #[inline(always)]
    pub fn dio14(&self) -> Dio14R {
        Dio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Data output enable for DIO15"]
    #[inline(always)]
    pub fn dio15(&self) -> Dio15R {
        Dio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Data output enable for DIO16"]
    #[inline(always)]
    pub fn dio16(&self) -> Dio16R {
        Dio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Data output enable for DIO17"]
    #[inline(always)]
    pub fn dio17(&self) -> Dio17R {
        Dio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Data output enable for DIO18"]
    #[inline(always)]
    pub fn dio18(&self) -> Dio18R {
        Dio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Data output enable for DIO19"]
    #[inline(always)]
    pub fn dio19(&self) -> Dio19R {
        Dio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Data output enable for DIO20"]
    #[inline(always)]
    pub fn dio20(&self) -> Dio20R {
        Dio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Data output enable for DIO21"]
    #[inline(always)]
    pub fn dio21(&self) -> Dio21R {
        Dio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Data output enable for DIO22"]
    #[inline(always)]
    pub fn dio22(&self) -> Dio22R {
        Dio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Data output enable for DIO23"]
    #[inline(always)]
    pub fn dio23(&self) -> Dio23R {
        Dio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Data output enable for DIO24"]
    #[inline(always)]
    pub fn dio24(&self) -> Dio24R {
        Dio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Data output enable for DIO25"]
    #[inline(always)]
    pub fn dio25(&self) -> Dio25R {
        Dio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&self) -> Reserved26R {
        Reserved26R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data output enable for DIO0"]
    #[inline(always)]
    #[must_use]
    pub fn dio0(&mut self) -> Dio0W<Doe31_0Spec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Data output enable for DIO1"]
    #[inline(always)]
    #[must_use]
    pub fn dio1(&mut self) -> Dio1W<Doe31_0Spec> {
        Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Data output enable for DIO2"]
    #[inline(always)]
    #[must_use]
    pub fn dio2(&mut self) -> Dio2W<Doe31_0Spec> {
        Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Data output enable for DIO3"]
    #[inline(always)]
    #[must_use]
    pub fn dio3(&mut self) -> Dio3W<Doe31_0Spec> {
        Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Data output enable for DIO4"]
    #[inline(always)]
    #[must_use]
    pub fn dio4(&mut self) -> Dio4W<Doe31_0Spec> {
        Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Data output enable for DIO5"]
    #[inline(always)]
    #[must_use]
    pub fn dio5(&mut self) -> Dio5W<Doe31_0Spec> {
        Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Data output enable for DIO6"]
    #[inline(always)]
    #[must_use]
    pub fn dio6(&mut self) -> Dio6W<Doe31_0Spec> {
        Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Data output enable for DIO7"]
    #[inline(always)]
    #[must_use]
    pub fn dio7(&mut self) -> Dio7W<Doe31_0Spec> {
        Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Data output enable for DIO8"]
    #[inline(always)]
    #[must_use]
    pub fn dio8(&mut self) -> Dio8W<Doe31_0Spec> {
        Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Data output enable for DIO9"]
    #[inline(always)]
    #[must_use]
    pub fn dio9(&mut self) -> Dio9W<Doe31_0Spec> {
        Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Data output enable for DIO10"]
    #[inline(always)]
    #[must_use]
    pub fn dio10(&mut self) -> Dio10W<Doe31_0Spec> {
        Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Data output enable for DIO11"]
    #[inline(always)]
    #[must_use]
    pub fn dio11(&mut self) -> Dio11W<Doe31_0Spec> {
        Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Data output enable for DIO12"]
    #[inline(always)]
    #[must_use]
    pub fn dio12(&mut self) -> Dio12W<Doe31_0Spec> {
        Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Data output enable for DIO13"]
    #[inline(always)]
    #[must_use]
    pub fn dio13(&mut self) -> Dio13W<Doe31_0Spec> {
        Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Data output enable for DIO14"]
    #[inline(always)]
    #[must_use]
    pub fn dio14(&mut self) -> Dio14W<Doe31_0Spec> {
        Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Data output enable for DIO15"]
    #[inline(always)]
    #[must_use]
    pub fn dio15(&mut self) -> Dio15W<Doe31_0Spec> {
        Dio15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Data output enable for DIO16"]
    #[inline(always)]
    #[must_use]
    pub fn dio16(&mut self) -> Dio16W<Doe31_0Spec> {
        Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Data output enable for DIO17"]
    #[inline(always)]
    #[must_use]
    pub fn dio17(&mut self) -> Dio17W<Doe31_0Spec> {
        Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Data output enable for DIO18"]
    #[inline(always)]
    #[must_use]
    pub fn dio18(&mut self) -> Dio18W<Doe31_0Spec> {
        Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Data output enable for DIO19"]
    #[inline(always)]
    #[must_use]
    pub fn dio19(&mut self) -> Dio19W<Doe31_0Spec> {
        Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Data output enable for DIO20"]
    #[inline(always)]
    #[must_use]
    pub fn dio20(&mut self) -> Dio20W<Doe31_0Spec> {
        Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Data output enable for DIO21"]
    #[inline(always)]
    #[must_use]
    pub fn dio21(&mut self) -> Dio21W<Doe31_0Spec> {
        Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Data output enable for DIO22"]
    #[inline(always)]
    #[must_use]
    pub fn dio22(&mut self) -> Dio22W<Doe31_0Spec> {
        Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Data output enable for DIO23"]
    #[inline(always)]
    #[must_use]
    pub fn dio23(&mut self) -> Dio23W<Doe31_0Spec> {
        Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Data output enable for DIO24"]
    #[inline(always)]
    #[must_use]
    pub fn dio24(&mut self) -> Dio24W<Doe31_0Spec> {
        Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Data output enable for DIO25"]
    #[inline(always)]
    #[must_use]
    pub fn dio25(&mut self) -> Dio25W<Doe31_0Spec> {
        Dio25W::new(self, 25)
    }
}
#[doc = "Data output enable for DIO 31 to 0 pins.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doe31_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doe31_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doe31_0Spec;
impl crate::RegisterSpec for Doe31_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doe31_0::R`](R) reader structure"]
impl crate::Readable for Doe31_0Spec {}
#[doc = "`write(|w| ..)` method takes [`doe31_0::W`](W) writer structure"]
impl crate::Writable for Doe31_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOE31_0 to value 0"]
impl crate::Resettable for Doe31_0Spec {
    const RESET_VALUE: u32 = 0;
}
