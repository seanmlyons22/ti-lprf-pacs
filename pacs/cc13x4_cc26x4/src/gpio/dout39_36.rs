#[doc = "Register `DOUT39_36` reader"]
pub type R = crate::R<Dout39_36Spec>;
#[doc = "Register `DOUT39_36` writer"]
pub type W = crate::W<Dout39_36Spec>;
#[doc = "Field `DIO36` reader - 0:0\\]
Sets the state of the pin that is configured as DIO#36, if the corresponding DOE47_0 bitfield is set."]
pub type Dio36R = crate::BitReader;
#[doc = "Field `DIO36` writer - 0:0\\]
Sets the state of the pin that is configured as DIO#36, if the corresponding DOE47_0 bitfield is set."]
pub type Dio36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIO37` reader - 8:8\\]
Sets the state of the pin that is configured as DIO#37, if the corresponding DOE47_0 bitfield is set."]
pub type Dio37R = crate::BitReader;
#[doc = "Field `DIO37` writer - 8:8\\]
Sets the state of the pin that is configured as DIO#37, if the corresponding DOE47_0 bitfield is set."]
pub type Dio37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `RESERVED9` writer - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIO38` reader - 16:16\\]
Sets the state of the pin that is configured as DIO#38, if the corresponding DOE47_0 bitfield is set."]
pub type Dio38R = crate::BitReader;
#[doc = "Field `DIO38` writer - 16:16\\]
Sets the state of the pin that is configured as DIO#38, if the corresponding DOE47_0 bitfield is set."]
pub type Dio38W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "Field `RESERVED17` writer - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIO39` reader - 24:24\\]
Sets the state of the pin that is configured as DIO#39, if the corresponding DOE47_0 bitfield is set."]
pub type Dio39R = crate::BitReader;
#[doc = "Field `DIO39` writer - 24:24\\]
Sets the state of the pin that is configured as DIO#39, if the corresponding DOE47_0 bitfield is set."]
pub type Dio39W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#36, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio36(&self) -> Dio36R {
        Dio36R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#37, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio37(&self) -> Dio37R {
        Dio37R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#38, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio38(&self) -> Dio38R {
        Dio38R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#39, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio39(&self) -> Dio39R {
        Dio39R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#36, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio36(&mut self) -> Dio36W<Dout39_36Spec> {
        Dio36W::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Dout39_36Spec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#37, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio37(&mut self) -> Dio37W<Dout39_36Spec> {
        Dio37W::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<Dout39_36Spec> {
        Reserved9W::new(self, 9)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#38, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio38(&mut self) -> Dio38W<Dout39_36Spec> {
        Dio38W::new(self, 16)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<Dout39_36Spec> {
        Reserved17W::new(self, 17)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#39, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio39(&mut self) -> Dio39W<Dout39_36Spec> {
        Dio39W::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> Reserved25W<Dout39_36Spec> {
        Reserved25W::new(self, 25)
    }
}
#[doc = "Data Out 39 to 36 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout39_36::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout39_36::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout39_36Spec;
impl crate::RegisterSpec for Dout39_36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout39_36::R`](R) reader structure"]
impl crate::Readable for Dout39_36Spec {}
#[doc = "`write(|w| ..)` method takes [`dout39_36::W`](W) writer structure"]
impl crate::Writable for Dout39_36Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT39_36 to value 0"]
impl crate::Resettable for Dout39_36Spec {
    const RESET_VALUE: u32 = 0;
}
