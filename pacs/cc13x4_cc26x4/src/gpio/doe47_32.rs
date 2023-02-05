#[doc = "Register `DOE47_32` reader"]
pub struct R(crate::R<DOE47_32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOE47_32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOE47_32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOE47_32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOE47_32` writer"]
pub struct W(crate::W<DOE47_32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOE47_32_SPEC>;
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
impl From<crate::W<DOE47_32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOE47_32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIO32` reader - 0:0\\]
Data output enable for DIO 32"]
pub type DIO32_R = crate::BitReader<bool>;
#[doc = "Field `DIO32` writer - 0:0\\]
Data output enable for DIO 32"]
pub type DIO32_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `DIO33` reader - 1:1\\]
Data output enable for DIO 33"]
pub type DIO33_R = crate::BitReader<bool>;
#[doc = "Field `DIO33` writer - 1:1\\]
Data output enable for DIO 33"]
pub type DIO33_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `DIO34` reader - 2:2\\]
Data output enable for DIO 34"]
pub type DIO34_R = crate::BitReader<bool>;
#[doc = "Field `DIO34` writer - 2:2\\]
Data output enable for DIO 34"]
pub type DIO34_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `DIO35` reader - 3:3\\]
Data output enable for DIO 35"]
pub type DIO35_R = crate::BitReader<bool>;
#[doc = "Field `DIO35` writer - 3:3\\]
Data output enable for DIO 35"]
pub type DIO35_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `DIO36` reader - 4:4\\]
Data output enable for DIO 36"]
pub type DIO36_R = crate::BitReader<bool>;
#[doc = "Field `DIO36` writer - 4:4\\]
Data output enable for DIO 36"]
pub type DIO36_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `DIO37` reader - 5:5\\]
Data output enable for DIO 37"]
pub type DIO37_R = crate::BitReader<bool>;
#[doc = "Field `DIO37` writer - 5:5\\]
Data output enable for DIO 37"]
pub type DIO37_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `DIO38` reader - 6:6\\]
Data output enable for DIO 38"]
pub type DIO38_R = crate::BitReader<bool>;
#[doc = "Field `DIO38` writer - 6:6\\]
Data output enable for DIO 38"]
pub type DIO38_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `DIO39` reader - 7:7\\]
Data output enable for DIO 39"]
pub type DIO39_R = crate::BitReader<bool>;
#[doc = "Field `DIO39` writer - 7:7\\]
Data output enable for DIO 39"]
pub type DIO39_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `DIO40` reader - 8:8\\]
Data output enable for DIO 40"]
pub type DIO40_R = crate::BitReader<bool>;
#[doc = "Field `DIO40` writer - 8:8\\]
Data output enable for DIO 40"]
pub type DIO40_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `DIO41` reader - 9:9\\]
Data output enable for DIO 41"]
pub type DIO41_R = crate::BitReader<bool>;
#[doc = "Field `DIO41` writer - 9:9\\]
Data output enable for DIO 41"]
pub type DIO41_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `DIO42` reader - 10:10\\]
Data output enable for DIO 42"]
pub type DIO42_R = crate::BitReader<bool>;
#[doc = "Field `DIO42` writer - 10:10\\]
Data output enable for DIO 42"]
pub type DIO42_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `DIO43` reader - 11:11\\]
Data output enable for DIO 43"]
pub type DIO43_R = crate::BitReader<bool>;
#[doc = "Field `DIO43` writer - 11:11\\]
Data output enable for DIO 43"]
pub type DIO43_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `DIO44` reader - 12:12\\]
Data output enable for DIO 44"]
pub type DIO44_R = crate::BitReader<bool>;
#[doc = "Field `DIO44` writer - 12:12\\]
Data output enable for DIO 44"]
pub type DIO44_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `DIO45` reader - 13:13\\]
Data output enable for DIO 45"]
pub type DIO45_R = crate::BitReader<bool>;
#[doc = "Field `DIO45` writer - 13:13\\]
Data output enable for DIO 45"]
pub type DIO45_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `DIO46` reader - 14:14\\]
Data output enable for DIO 46"]
pub type DIO46_R = crate::BitReader<bool>;
#[doc = "Field `DIO46` writer - 14:14\\]
Data output enable for DIO 46"]
pub type DIO46_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `DIO47` reader - 15:15\\]
Data output enable for DIO 47"]
pub type DIO47_R = crate::BitReader<bool>;
#[doc = "Field `DIO47` writer - 15:15\\]
Data output enable for DIO 47"]
pub type DIO47_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOE47_32_SPEC, bool, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DOE47_32_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data output enable for DIO 32"]
    #[inline(always)]
    pub fn dio32(&self) -> DIO32_R {
        DIO32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Data output enable for DIO 33"]
    #[inline(always)]
    pub fn dio33(&self) -> DIO33_R {
        DIO33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Data output enable for DIO 34"]
    #[inline(always)]
    pub fn dio34(&self) -> DIO34_R {
        DIO34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Data output enable for DIO 35"]
    #[inline(always)]
    pub fn dio35(&self) -> DIO35_R {
        DIO35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Data output enable for DIO 36"]
    #[inline(always)]
    pub fn dio36(&self) -> DIO36_R {
        DIO36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Data output enable for DIO 37"]
    #[inline(always)]
    pub fn dio37(&self) -> DIO37_R {
        DIO37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Data output enable for DIO 38"]
    #[inline(always)]
    pub fn dio38(&self) -> DIO38_R {
        DIO38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Data output enable for DIO 39"]
    #[inline(always)]
    pub fn dio39(&self) -> DIO39_R {
        DIO39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Data output enable for DIO 40"]
    #[inline(always)]
    pub fn dio40(&self) -> DIO40_R {
        DIO40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Data output enable for DIO 41"]
    #[inline(always)]
    pub fn dio41(&self) -> DIO41_R {
        DIO41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Data output enable for DIO 42"]
    #[inline(always)]
    pub fn dio42(&self) -> DIO42_R {
        DIO42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Data output enable for DIO 43"]
    #[inline(always)]
    pub fn dio43(&self) -> DIO43_R {
        DIO43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Data output enable for DIO 44"]
    #[inline(always)]
    pub fn dio44(&self) -> DIO44_R {
        DIO44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Data output enable for DIO 45"]
    #[inline(always)]
    pub fn dio45(&self) -> DIO45_R {
        DIO45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Data output enable for DIO 46"]
    #[inline(always)]
    pub fn dio46(&self) -> DIO46_R {
        DIO46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Data output enable for DIO 47"]
    #[inline(always)]
    pub fn dio47(&self) -> DIO47_R {
        DIO47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data output enable for DIO 32"]
    #[inline(always)]
    #[must_use]
    pub fn dio32(&mut self) -> DIO32_W<0> {
        DIO32_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Data output enable for DIO 33"]
    #[inline(always)]
    #[must_use]
    pub fn dio33(&mut self) -> DIO33_W<1> {
        DIO33_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Data output enable for DIO 34"]
    #[inline(always)]
    #[must_use]
    pub fn dio34(&mut self) -> DIO34_W<2> {
        DIO34_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Data output enable for DIO 35"]
    #[inline(always)]
    #[must_use]
    pub fn dio35(&mut self) -> DIO35_W<3> {
        DIO35_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Data output enable for DIO 36"]
    #[inline(always)]
    #[must_use]
    pub fn dio36(&mut self) -> DIO36_W<4> {
        DIO36_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Data output enable for DIO 37"]
    #[inline(always)]
    #[must_use]
    pub fn dio37(&mut self) -> DIO37_W<5> {
        DIO37_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Data output enable for DIO 38"]
    #[inline(always)]
    #[must_use]
    pub fn dio38(&mut self) -> DIO38_W<6> {
        DIO38_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Data output enable for DIO 39"]
    #[inline(always)]
    #[must_use]
    pub fn dio39(&mut self) -> DIO39_W<7> {
        DIO39_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Data output enable for DIO 40"]
    #[inline(always)]
    #[must_use]
    pub fn dio40(&mut self) -> DIO40_W<8> {
        DIO40_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Data output enable for DIO 41"]
    #[inline(always)]
    #[must_use]
    pub fn dio41(&mut self) -> DIO41_W<9> {
        DIO41_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Data output enable for DIO 42"]
    #[inline(always)]
    #[must_use]
    pub fn dio42(&mut self) -> DIO42_W<10> {
        DIO42_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Data output enable for DIO 43"]
    #[inline(always)]
    #[must_use]
    pub fn dio43(&mut self) -> DIO43_W<11> {
        DIO43_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Data output enable for DIO 44"]
    #[inline(always)]
    #[must_use]
    pub fn dio44(&mut self) -> DIO44_W<12> {
        DIO44_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Data output enable for DIO 45"]
    #[inline(always)]
    #[must_use]
    pub fn dio45(&mut self) -> DIO45_W<13> {
        DIO45_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Data output enable for DIO 46"]
    #[inline(always)]
    #[must_use]
    pub fn dio46(&mut self) -> DIO46_W<14> {
        DIO46_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Data output enable for DIO 47"]
    #[inline(always)]
    #[must_use]
    pub fn dio47(&mut self) -> DIO47_W<15> {
        DIO47_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Output Enable for DIO 32 to 47\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doe47_32](index.html) module"]
pub struct DOE47_32_SPEC;
impl crate::RegisterSpec for DOE47_32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doe47_32::R](R) reader structure"]
impl crate::Readable for DOE47_32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doe47_32::W](W) writer structure"]
impl crate::Writable for DOE47_32_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOE47_32 to value 0"]
impl crate::Resettable for DOE47_32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
