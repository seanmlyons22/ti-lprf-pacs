#[doc = "Register `EVSTAT0` reader"]
pub type R = crate::R<Evstat0Spec>;
#[doc = "Register `EVSTAT0` writer"]
pub type W = crate::W<Evstat0Spec>;
#[doc = "Field `AUXIO0` reader - 0:0\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
pub type Auxio0R = crate::BitReader;
#[doc = "Field `AUXIO1` reader - 1:1\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
pub type Auxio1R = crate::BitReader;
#[doc = "Field `AUXIO2` reader - 2:2\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
pub type Auxio2R = crate::BitReader;
#[doc = "Field `AUXIO3` reader - 3:3\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
pub type Auxio3R = crate::BitReader;
#[doc = "Field `AUXIO4` reader - 4:4\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
pub type Auxio4R = crate::BitReader;
#[doc = "Field `AUXIO5` reader - 5:5\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
pub type Auxio5R = crate::BitReader;
#[doc = "Field `AUXIO6` reader - 6:6\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
pub type Auxio6R = crate::BitReader;
#[doc = "Field `AUXIO7` reader - 7:7\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
pub type Auxio7R = crate::BitReader;
#[doc = "Field `AUXIO8` reader - 8:8\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
pub type Auxio8R = crate::BitReader;
#[doc = "Field `AUXIO9` reader - 9:9\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
pub type Auxio9R = crate::BitReader;
#[doc = "Field `AUXIO10` reader - 10:10\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
pub type Auxio10R = crate::BitReader;
#[doc = "Field `AUXIO11` reader - 11:11\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
pub type Auxio11R = crate::BitReader;
#[doc = "Field `AUXIO12` reader - 12:12\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
pub type Auxio12R = crate::BitReader;
#[doc = "Field `AUXIO13` reader - 13:13\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
pub type Auxio13R = crate::BitReader;
#[doc = "Field `AUXIO14` reader - 14:14\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
pub type Auxio14R = crate::BitReader;
#[doc = "Field `AUXIO15` reader - 15:15\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
pub type Auxio15R = crate::BitReader;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio0(&self) -> Auxio0R {
        Auxio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio1(&self) -> Auxio1R {
        Auxio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio2(&self) -> Auxio2R {
        Auxio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio3(&self) -> Auxio3R {
        Auxio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio4(&self) -> Auxio4R {
        Auxio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio5(&self) -> Auxio5R {
        Auxio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio6(&self) -> Auxio6R {
        Auxio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio7(&self) -> Auxio7R {
        Auxio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio8(&self) -> Auxio8R {
        Auxio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio9(&self) -> Auxio9R {
        Auxio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio10(&self) -> Auxio10R {
        Auxio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio11(&self) -> Auxio11R {
        Auxio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio12(&self) -> Auxio12R {
        Auxio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio13(&self) -> Auxio13R {
        Auxio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio14(&self) -> Auxio14R {
        Auxio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio15(&self) -> Auxio15R {
        Auxio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Event Status 0 Register holds events 0 thru 15 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Evstat0Spec;
impl crate::RegisterSpec for Evstat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evstat0::R`](R) reader structure"]
impl crate::Readable for Evstat0Spec {}
#[doc = "`write(|w| ..)` method takes [`evstat0::W`](W) writer structure"]
impl crate::Writable for Evstat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVSTAT0 to value 0"]
impl crate::Resettable for Evstat0Spec {
    const RESET_VALUE: u32 = 0;
}
