#[doc = "Register `DIN47_32` reader"]
pub type R = crate::R<Din47_32Spec>;
#[doc = "Register `DIN47_32` writer"]
pub type W = crate::W<Din47_32Spec>;
#[doc = "Field `DIO32` reader - 0:0\\]
Data input from DIO 32"]
pub type Dio32R = crate::BitReader;
#[doc = "Field `DIO32` writer - 0:0\\]
Data input from DIO 32"]
pub type Dio32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO33` reader - 1:1\\]
Data input from DIO 33"]
pub type Dio33R = crate::BitReader;
#[doc = "Field `DIO33` writer - 1:1\\]
Data input from DIO 33"]
pub type Dio33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO34` reader - 2:2\\]
Data input from DIO 34"]
pub type Dio34R = crate::BitReader;
#[doc = "Field `DIO34` writer - 2:2\\]
Data input from DIO 34"]
pub type Dio34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO35` reader - 3:3\\]
Data input from DIO 35"]
pub type Dio35R = crate::BitReader;
#[doc = "Field `DIO35` writer - 3:3\\]
Data input from DIO 35"]
pub type Dio35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO36` reader - 4:4\\]
Data input from DIO 36"]
pub type Dio36R = crate::BitReader;
#[doc = "Field `DIO36` writer - 4:4\\]
Data input from DIO 36"]
pub type Dio36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO37` reader - 5:5\\]
Data input from DIO 37"]
pub type Dio37R = crate::BitReader;
#[doc = "Field `DIO37` writer - 5:5\\]
Data input from DIO 37"]
pub type Dio37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO38` reader - 6:6\\]
Data input from DIO 38"]
pub type Dio38R = crate::BitReader;
#[doc = "Field `DIO38` writer - 6:6\\]
Data input from DIO 38"]
pub type Dio38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO39` reader - 7:7\\]
Data input from DIO 39"]
pub type Dio39R = crate::BitReader;
#[doc = "Field `DIO39` writer - 7:7\\]
Data input from DIO 39"]
pub type Dio39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO40` reader - 8:8\\]
Data input from DIO 40"]
pub type Dio40R = crate::BitReader;
#[doc = "Field `DIO40` writer - 8:8\\]
Data input from DIO 40"]
pub type Dio40W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO41` reader - 9:9\\]
Data input from DIO 41"]
pub type Dio41R = crate::BitReader;
#[doc = "Field `DIO41` writer - 9:9\\]
Data input from DIO 41"]
pub type Dio41W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO42` reader - 10:10\\]
Data input from DIO 42"]
pub type Dio42R = crate::BitReader;
#[doc = "Field `DIO42` writer - 10:10\\]
Data input from DIO 42"]
pub type Dio42W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO43` reader - 11:11\\]
Data input from DIO 43"]
pub type Dio43R = crate::BitReader;
#[doc = "Field `DIO43` writer - 11:11\\]
Data input from DIO 43"]
pub type Dio43W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO44` reader - 12:12\\]
Data input from DIO 44"]
pub type Dio44R = crate::BitReader;
#[doc = "Field `DIO44` writer - 12:12\\]
Data input from DIO 44"]
pub type Dio44W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO45` reader - 13:13\\]
Data input from DIO 45"]
pub type Dio45R = crate::BitReader;
#[doc = "Field `DIO45` writer - 13:13\\]
Data input from DIO 45"]
pub type Dio45W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO46` reader - 14:14\\]
Data input from DIO 46"]
pub type Dio46R = crate::BitReader;
#[doc = "Field `DIO46` writer - 14:14\\]
Data input from DIO 46"]
pub type Dio46W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO47` reader - 15:15\\]
Data input from DIO 47"]
pub type Dio47R = crate::BitReader;
#[doc = "Field `DIO47` writer - 15:15\\]
Data input from DIO 47"]
pub type Dio47W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data input from DIO 32"]
    #[inline(always)]
    pub fn dio32(&self) -> Dio32R {
        Dio32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Data input from DIO 33"]
    #[inline(always)]
    pub fn dio33(&self) -> Dio33R {
        Dio33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Data input from DIO 34"]
    #[inline(always)]
    pub fn dio34(&self) -> Dio34R {
        Dio34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Data input from DIO 35"]
    #[inline(always)]
    pub fn dio35(&self) -> Dio35R {
        Dio35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Data input from DIO 36"]
    #[inline(always)]
    pub fn dio36(&self) -> Dio36R {
        Dio36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Data input from DIO 37"]
    #[inline(always)]
    pub fn dio37(&self) -> Dio37R {
        Dio37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Data input from DIO 38"]
    #[inline(always)]
    pub fn dio38(&self) -> Dio38R {
        Dio38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Data input from DIO 39"]
    #[inline(always)]
    pub fn dio39(&self) -> Dio39R {
        Dio39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Data input from DIO 40"]
    #[inline(always)]
    pub fn dio40(&self) -> Dio40R {
        Dio40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Data input from DIO 41"]
    #[inline(always)]
    pub fn dio41(&self) -> Dio41R {
        Dio41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Data input from DIO 42"]
    #[inline(always)]
    pub fn dio42(&self) -> Dio42R {
        Dio42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Data input from DIO 43"]
    #[inline(always)]
    pub fn dio43(&self) -> Dio43R {
        Dio43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Data input from DIO 44"]
    #[inline(always)]
    pub fn dio44(&self) -> Dio44R {
        Dio44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Data input from DIO 45"]
    #[inline(always)]
    pub fn dio45(&self) -> Dio45R {
        Dio45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Data input from DIO 46"]
    #[inline(always)]
    pub fn dio46(&self) -> Dio46R {
        Dio46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Data input from DIO 47"]
    #[inline(always)]
    pub fn dio47(&self) -> Dio47R {
        Dio47R::new(((self.bits >> 15) & 1) != 0)
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
Data input from DIO 32"]
    #[inline(always)]
    #[must_use]
    pub fn dio32(&mut self) -> Dio32W<Din47_32Spec> {
        Dio32W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Data input from DIO 33"]
    #[inline(always)]
    #[must_use]
    pub fn dio33(&mut self) -> Dio33W<Din47_32Spec> {
        Dio33W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Data input from DIO 34"]
    #[inline(always)]
    #[must_use]
    pub fn dio34(&mut self) -> Dio34W<Din47_32Spec> {
        Dio34W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Data input from DIO 35"]
    #[inline(always)]
    #[must_use]
    pub fn dio35(&mut self) -> Dio35W<Din47_32Spec> {
        Dio35W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Data input from DIO 36"]
    #[inline(always)]
    #[must_use]
    pub fn dio36(&mut self) -> Dio36W<Din47_32Spec> {
        Dio36W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Data input from DIO 37"]
    #[inline(always)]
    #[must_use]
    pub fn dio37(&mut self) -> Dio37W<Din47_32Spec> {
        Dio37W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Data input from DIO 38"]
    #[inline(always)]
    #[must_use]
    pub fn dio38(&mut self) -> Dio38W<Din47_32Spec> {
        Dio38W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Data input from DIO 39"]
    #[inline(always)]
    #[must_use]
    pub fn dio39(&mut self) -> Dio39W<Din47_32Spec> {
        Dio39W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Data input from DIO 40"]
    #[inline(always)]
    #[must_use]
    pub fn dio40(&mut self) -> Dio40W<Din47_32Spec> {
        Dio40W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Data input from DIO 41"]
    #[inline(always)]
    #[must_use]
    pub fn dio41(&mut self) -> Dio41W<Din47_32Spec> {
        Dio41W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Data input from DIO 42"]
    #[inline(always)]
    #[must_use]
    pub fn dio42(&mut self) -> Dio42W<Din47_32Spec> {
        Dio42W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Data input from DIO 43"]
    #[inline(always)]
    #[must_use]
    pub fn dio43(&mut self) -> Dio43W<Din47_32Spec> {
        Dio43W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Data input from DIO 44"]
    #[inline(always)]
    #[must_use]
    pub fn dio44(&mut self) -> Dio44W<Din47_32Spec> {
        Dio44W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Data input from DIO 45"]
    #[inline(always)]
    #[must_use]
    pub fn dio45(&mut self) -> Dio45W<Din47_32Spec> {
        Dio45W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Data input from DIO 46"]
    #[inline(always)]
    #[must_use]
    pub fn dio46(&mut self) -> Dio46W<Din47_32Spec> {
        Dio46W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Data input from DIO 47"]
    #[inline(always)]
    #[must_use]
    pub fn dio47(&mut self) -> Dio47W<Din47_32Spec> {
        Dio47W::new(self, 15)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Din47_32Spec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Data Input from DIO 32 to 47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din47_32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din47_32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din47_32Spec;
impl crate::RegisterSpec for Din47_32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din47_32::R`](R) reader structure"]
impl crate::Readable for Din47_32Spec {}
#[doc = "`write(|w| ..)` method takes [`din47_32::W`](W) writer structure"]
impl crate::Writable for Din47_32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIN47_32 to value 0"]
impl crate::Resettable for Din47_32Spec {
    const RESET_VALUE: u32 = 0;
}
