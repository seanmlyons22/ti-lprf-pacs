#[doc = "Register `DTBOE` reader"]
pub type R = crate::R<DtboeSpec>;
#[doc = "Register `DTBOE` writer"]
pub type W = crate::W<DtboeSpec>;
#[doc = "0:0\\]
Enables DTB output 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En0 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En0> for bool {
    #[inline(always)]
    fn from(variant: En0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN0` reader - 0:0\\]
Enables DTB output 0"]
pub type En0R = crate::BitReader<En0>;
impl En0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En0 {
        match self.bits {
            true => En0::En,
            false => En0::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En0::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En0::Dis
    }
}
#[doc = "Field `EN0` writer - 0:0\\]
Enables DTB output 0"]
pub type En0W<'a, REG> = crate::BitWriter<'a, REG, En0>;
impl<'a, REG> En0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En0::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En0::Dis)
    }
}
#[doc = "1:1\\]
Enables DTB output 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En1 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En1> for bool {
    #[inline(always)]
    fn from(variant: En1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN1` reader - 1:1\\]
Enables DTB output 1"]
pub type En1R = crate::BitReader<En1>;
impl En1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En1 {
        match self.bits {
            true => En1::En,
            false => En1::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En1::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En1::Dis
    }
}
#[doc = "Field `EN1` writer - 1:1\\]
Enables DTB output 1"]
pub type En1W<'a, REG> = crate::BitWriter<'a, REG, En1>;
impl<'a, REG> En1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En1::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En1::Dis)
    }
}
#[doc = "2:2\\]
Enables DTB output 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En2 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En2> for bool {
    #[inline(always)]
    fn from(variant: En2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN2` reader - 2:2\\]
Enables DTB output 2"]
pub type En2R = crate::BitReader<En2>;
impl En2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En2 {
        match self.bits {
            true => En2::En,
            false => En2::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En2::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En2::Dis
    }
}
#[doc = "Field `EN2` writer - 2:2\\]
Enables DTB output 2"]
pub type En2W<'a, REG> = crate::BitWriter<'a, REG, En2>;
impl<'a, REG> En2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En2::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En2::Dis)
    }
}
#[doc = "3:3\\]
Enables DTB output 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En3 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En3> for bool {
    #[inline(always)]
    fn from(variant: En3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN3` reader - 3:3\\]
Enables DTB output 3"]
pub type En3R = crate::BitReader<En3>;
impl En3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En3 {
        match self.bits {
            true => En3::En,
            false => En3::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En3::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En3::Dis
    }
}
#[doc = "Field `EN3` writer - 3:3\\]
Enables DTB output 3"]
pub type En3W<'a, REG> = crate::BitWriter<'a, REG, En3>;
impl<'a, REG> En3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En3::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En3::Dis)
    }
}
#[doc = "4:4\\]
Enables DTB output 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En4 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En4> for bool {
    #[inline(always)]
    fn from(variant: En4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN4` reader - 4:4\\]
Enables DTB output 4"]
pub type En4R = crate::BitReader<En4>;
impl En4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En4 {
        match self.bits {
            true => En4::En,
            false => En4::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En4::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En4::Dis
    }
}
#[doc = "Field `EN4` writer - 4:4\\]
Enables DTB output 4"]
pub type En4W<'a, REG> = crate::BitWriter<'a, REG, En4>;
impl<'a, REG> En4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En4::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En4::Dis)
    }
}
#[doc = "5:5\\]
Enables DTB output 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En5 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En5> for bool {
    #[inline(always)]
    fn from(variant: En5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN5` reader - 5:5\\]
Enables DTB output 5"]
pub type En5R = crate::BitReader<En5>;
impl En5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En5 {
        match self.bits {
            true => En5::En,
            false => En5::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En5::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En5::Dis
    }
}
#[doc = "Field `EN5` writer - 5:5\\]
Enables DTB output 5"]
pub type En5W<'a, REG> = crate::BitWriter<'a, REG, En5>;
impl<'a, REG> En5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En5::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En5::Dis)
    }
}
#[doc = "6:6\\]
Enables DTB output 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En6 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En6> for bool {
    #[inline(always)]
    fn from(variant: En6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN6` reader - 6:6\\]
Enables DTB output 6"]
pub type En6R = crate::BitReader<En6>;
impl En6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En6 {
        match self.bits {
            true => En6::En,
            false => En6::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En6::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En6::Dis
    }
}
#[doc = "Field `EN6` writer - 6:6\\]
Enables DTB output 6"]
pub type En6W<'a, REG> = crate::BitWriter<'a, REG, En6>;
impl<'a, REG> En6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En6::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En6::Dis)
    }
}
#[doc = "7:7\\]
Enables DTB output 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En7 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En7> for bool {
    #[inline(always)]
    fn from(variant: En7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN7` reader - 7:7\\]
Enables DTB output 7"]
pub type En7R = crate::BitReader<En7>;
impl En7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En7 {
        match self.bits {
            true => En7::En,
            false => En7::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En7::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En7::Dis
    }
}
#[doc = "Field `EN7` writer - 7:7\\]
Enables DTB output 7"]
pub type En7W<'a, REG> = crate::BitWriter<'a, REG, En7>;
impl<'a, REG> En7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En7::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En7::Dis)
    }
}
#[doc = "8:8\\]
Enables DTB output 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En8 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En8> for bool {
    #[inline(always)]
    fn from(variant: En8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN8` reader - 8:8\\]
Enables DTB output 8"]
pub type En8R = crate::BitReader<En8>;
impl En8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En8 {
        match self.bits {
            true => En8::En,
            false => En8::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En8::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En8::Dis
    }
}
#[doc = "Field `EN8` writer - 8:8\\]
Enables DTB output 8"]
pub type En8W<'a, REG> = crate::BitWriter<'a, REG, En8>;
impl<'a, REG> En8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En8::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En8::Dis)
    }
}
#[doc = "9:9\\]
Enables DTB output 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En9 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En9> for bool {
    #[inline(always)]
    fn from(variant: En9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN9` reader - 9:9\\]
Enables DTB output 9"]
pub type En9R = crate::BitReader<En9>;
impl En9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En9 {
        match self.bits {
            true => En9::En,
            false => En9::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En9::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En9::Dis
    }
}
#[doc = "Field `EN9` writer - 9:9\\]
Enables DTB output 9"]
pub type En9W<'a, REG> = crate::BitWriter<'a, REG, En9>;
impl<'a, REG> En9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En9::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En9::Dis)
    }
}
#[doc = "10:10\\]
Enables DTB output 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En10 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En10> for bool {
    #[inline(always)]
    fn from(variant: En10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN10` reader - 10:10\\]
Enables DTB output 10"]
pub type En10R = crate::BitReader<En10>;
impl En10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En10 {
        match self.bits {
            true => En10::En,
            false => En10::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En10::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En10::Dis
    }
}
#[doc = "Field `EN10` writer - 10:10\\]
Enables DTB output 10"]
pub type En10W<'a, REG> = crate::BitWriter<'a, REG, En10>;
impl<'a, REG> En10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En10::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En10::Dis)
    }
}
#[doc = "11:11\\]
Enables DTB output 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En11 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En11> for bool {
    #[inline(always)]
    fn from(variant: En11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN11` reader - 11:11\\]
Enables DTB output 11"]
pub type En11R = crate::BitReader<En11>;
impl En11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En11 {
        match self.bits {
            true => En11::En,
            false => En11::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En11::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En11::Dis
    }
}
#[doc = "Field `EN11` writer - 11:11\\]
Enables DTB output 11"]
pub type En11W<'a, REG> = crate::BitWriter<'a, REG, En11>;
impl<'a, REG> En11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En11::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En11::Dis)
    }
}
#[doc = "12:12\\]
Enables DTB output 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En12 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En12> for bool {
    #[inline(always)]
    fn from(variant: En12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN12` reader - 12:12\\]
Enables DTB output 12"]
pub type En12R = crate::BitReader<En12>;
impl En12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En12 {
        match self.bits {
            true => En12::En,
            false => En12::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En12::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En12::Dis
    }
}
#[doc = "Field `EN12` writer - 12:12\\]
Enables DTB output 12"]
pub type En12W<'a, REG> = crate::BitWriter<'a, REG, En12>;
impl<'a, REG> En12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En12::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En12::Dis)
    }
}
#[doc = "13:13\\]
Enables DTB output 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En13 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En13> for bool {
    #[inline(always)]
    fn from(variant: En13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN13` reader - 13:13\\]
Enables DTB output 13"]
pub type En13R = crate::BitReader<En13>;
impl En13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En13 {
        match self.bits {
            true => En13::En,
            false => En13::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En13::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En13::Dis
    }
}
#[doc = "Field `EN13` writer - 13:13\\]
Enables DTB output 13"]
pub type En13W<'a, REG> = crate::BitWriter<'a, REG, En13>;
impl<'a, REG> En13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En13::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En13::Dis)
    }
}
#[doc = "14:14\\]
Enables DTB output 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En14 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En14> for bool {
    #[inline(always)]
    fn from(variant: En14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN14` reader - 14:14\\]
Enables DTB output 14"]
pub type En14R = crate::BitReader<En14>;
impl En14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En14 {
        match self.bits {
            true => En14::En,
            false => En14::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En14::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En14::Dis
    }
}
#[doc = "Field `EN14` writer - 14:14\\]
Enables DTB output 14"]
pub type En14W<'a, REG> = crate::BitWriter<'a, REG, En14>;
impl<'a, REG> En14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En14::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En14::Dis)
    }
}
#[doc = "15:15\\]
Enables DTB output 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En15 {
    #[doc = "1: DTB output enabled"]
    En = 1,
    #[doc = "0: DTB output disabled"]
    Dis = 0,
}
impl From<En15> for bool {
    #[inline(always)]
    fn from(variant: En15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN15` reader - 15:15\\]
Enables DTB output 15"]
pub type En15R = crate::BitReader<En15>;
impl En15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En15 {
        match self.bits {
            true => En15::En,
            false => En15::Dis,
        }
    }
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En15::En
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En15::Dis
    }
}
#[doc = "Field `EN15` writer - 15:15\\]
Enables DTB output 15"]
pub type En15W<'a, REG> = crate::BitWriter<'a, REG, En15>;
impl<'a, REG> En15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DTB output enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En15::En)
    }
    #[doc = "DTB output disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En15::Dis)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables DTB output 0"]
    #[inline(always)]
    pub fn en0(&self) -> En0R {
        En0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables DTB output 1"]
    #[inline(always)]
    pub fn en1(&self) -> En1R {
        En1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables DTB output 2"]
    #[inline(always)]
    pub fn en2(&self) -> En2R {
        En2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables DTB output 3"]
    #[inline(always)]
    pub fn en3(&self) -> En3R {
        En3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables DTB output 4"]
    #[inline(always)]
    pub fn en4(&self) -> En4R {
        En4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Enables DTB output 5"]
    #[inline(always)]
    pub fn en5(&self) -> En5R {
        En5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Enables DTB output 6"]
    #[inline(always)]
    pub fn en6(&self) -> En6R {
        En6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enables DTB output 7"]
    #[inline(always)]
    pub fn en7(&self) -> En7R {
        En7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables DTB output 8"]
    #[inline(always)]
    pub fn en8(&self) -> En8R {
        En8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Enables DTB output 9"]
    #[inline(always)]
    pub fn en9(&self) -> En9R {
        En9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Enables DTB output 10"]
    #[inline(always)]
    pub fn en10(&self) -> En10R {
        En10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Enables DTB output 11"]
    #[inline(always)]
    pub fn en11(&self) -> En11R {
        En11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Enables DTB output 12"]
    #[inline(always)]
    pub fn en12(&self) -> En12R {
        En12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Enables DTB output 13"]
    #[inline(always)]
    pub fn en13(&self) -> En13R {
        En13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Enables DTB output 14"]
    #[inline(always)]
    pub fn en14(&self) -> En14R {
        En14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Enables DTB output 15"]
    #[inline(always)]
    pub fn en15(&self) -> En15R {
        En15R::new(((self.bits >> 15) & 1) != 0)
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
Enables DTB output 0"]
    #[inline(always)]
    #[must_use]
    pub fn en0(&mut self) -> En0W<DtboeSpec> {
        En0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enables DTB output 1"]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> En1W<DtboeSpec> {
        En1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enables DTB output 2"]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> En2W<DtboeSpec> {
        En2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Enables DTB output 3"]
    #[inline(always)]
    #[must_use]
    pub fn en3(&mut self) -> En3W<DtboeSpec> {
        En3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Enables DTB output 4"]
    #[inline(always)]
    #[must_use]
    pub fn en4(&mut self) -> En4W<DtboeSpec> {
        En4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Enables DTB output 5"]
    #[inline(always)]
    #[must_use]
    pub fn en5(&mut self) -> En5W<DtboeSpec> {
        En5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Enables DTB output 6"]
    #[inline(always)]
    #[must_use]
    pub fn en6(&mut self) -> En6W<DtboeSpec> {
        En6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Enables DTB output 7"]
    #[inline(always)]
    #[must_use]
    pub fn en7(&mut self) -> En7W<DtboeSpec> {
        En7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables DTB output 8"]
    #[inline(always)]
    #[must_use]
    pub fn en8(&mut self) -> En8W<DtboeSpec> {
        En8W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Enables DTB output 9"]
    #[inline(always)]
    #[must_use]
    pub fn en9(&mut self) -> En9W<DtboeSpec> {
        En9W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Enables DTB output 10"]
    #[inline(always)]
    #[must_use]
    pub fn en10(&mut self) -> En10W<DtboeSpec> {
        En10W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Enables DTB output 11"]
    #[inline(always)]
    #[must_use]
    pub fn en11(&mut self) -> En11W<DtboeSpec> {
        En11W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Enables DTB output 12"]
    #[inline(always)]
    #[must_use]
    pub fn en12(&mut self) -> En12W<DtboeSpec> {
        En12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Enables DTB output 13"]
    #[inline(always)]
    #[must_use]
    pub fn en13(&mut self) -> En13W<DtboeSpec> {
        En13W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Enables DTB output 14"]
    #[inline(always)]
    #[must_use]
    pub fn en14(&mut self) -> En14W<DtboeSpec> {
        En14W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Enables DTB output 15"]
    #[inline(always)]
    #[must_use]
    pub fn en15(&mut self) -> En15W<DtboeSpec> {
        En15W::new(self, 15)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<DtboeSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "DTB output enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtboe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtboe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtboeSpec;
impl crate::RegisterSpec for DtboeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtboe::R`](R) reader structure"]
impl crate::Readable for DtboeSpec {}
#[doc = "`write(|w| ..)` method takes [`dtboe::W`](W) writer structure"]
impl crate::Writable for DtboeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTBOE to value 0"]
impl crate::Resettable for DtboeSpec {
    const RESET_VALUE: u32 = 0;
}
