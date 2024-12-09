#[doc = "Register `DOUT7_4` reader"]
pub type R = crate::R<Dout7_4Spec>;
#[doc = "Register `DOUT7_4` writer"]
pub type W = crate::W<Dout7_4Spec>;
#[doc = "Field `DIO4` reader - 0:0\\]
Sets the state of the pin that is configured as DIO#4, if the corresponding DOE47_0 bitfield is set."]
pub type Dio4R = crate::BitReader;
#[doc = "Field `DIO4` writer - 0:0\\]
Sets the state of the pin that is configured as DIO#4, if the corresponding DOE47_0 bitfield is set."]
pub type Dio4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DIO5` reader - 8:8\\]
Sets the state of the pin that is configured as DIO#5, if the corresponding DOE47_0 bitfield is set."]
pub type Dio5R = crate::BitReader;
#[doc = "Field `DIO5` writer - 8:8\\]
Sets the state of the pin that is configured as DIO#5, if the corresponding DOE47_0 bitfield is set."]
pub type Dio5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `DIO6` reader - 16:16\\]
Sets the state of the pin that is configured as DIO#6, if the corresponding DOE47_0 bitfield is set."]
pub type Dio6R = crate::BitReader;
#[doc = "Field `DIO6` writer - 16:16\\]
Sets the state of the pin that is configured as DIO#6, if the corresponding DOE47_0 bitfield is set."]
pub type Dio6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved17R = crate::FieldReader;
#[doc = "Field `DIO7` reader - 24:24\\]
Sets the state of the pin that is configured as DIO#7, if the corresponding DOE47_0 bitfield is set."]
pub type Dio7R = crate::BitReader;
#[doc = "Field `DIO7` writer - 24:24\\]
Sets the state of the pin that is configured as DIO#7, if the corresponding DOE47_0 bitfield is set."]
pub type Dio7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved25R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Sets the state of the pin that is configured as DIO#4, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio4(&self) -> Dio4R {
        Dio4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#5, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio5(&self) -> Dio5R {
        Dio5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#6, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio6(&self) -> Dio6R {
        Dio6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - 23:17\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#7, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    pub fn dio7(&self) -> Dio7R {
        Dio7R::new(((self.bits >> 24) & 1) != 0)
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
Sets the state of the pin that is configured as DIO#4, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio4(&mut self) -> Dio4W<Dout7_4Spec> {
        Dio4W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Sets the state of the pin that is configured as DIO#5, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio5(&mut self) -> Dio5W<Dout7_4Spec> {
        Dio5W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Sets the state of the pin that is configured as DIO#6, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio6(&mut self) -> Dio6W<Dout7_4Spec> {
        Dio6W::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Sets the state of the pin that is configured as DIO#7, if the corresponding DOE47_0 bitfield is set."]
    #[inline(always)]
    #[must_use]
    pub fn dio7(&mut self) -> Dio7W<Dout7_4Spec> {
        Dio7W::new(self, 24)
    }
}
#[doc = "Data Out 4 to 7 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout7_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout7_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout7_4Spec;
impl crate::RegisterSpec for Dout7_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout7_4::R`](R) reader structure"]
impl crate::Readable for Dout7_4Spec {}
#[doc = "`write(|w| ..)` method takes [`dout7_4::W`](W) writer structure"]
impl crate::Writable for Dout7_4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT7_4 to value 0"]
impl crate::Resettable for Dout7_4Spec {
    const RESET_VALUE: u32 = 0;
}
