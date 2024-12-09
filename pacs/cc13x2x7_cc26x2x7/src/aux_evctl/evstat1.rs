#[doc = "Register `EVSTAT1` reader"]
pub type R = crate::R<Evstat1Spec>;
#[doc = "Register `EVSTAT1` writer"]
pub type W = crate::W<Evstat1Spec>;
#[doc = "Field `AUXIO16` reader - 0:0\\]
AUXIO16 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 0."]
pub type Auxio16R = crate::BitReader;
#[doc = "Field `AUXIO17` reader - 1:1\\]
AUXIO17 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 1."]
pub type Auxio17R = crate::BitReader;
#[doc = "Field `AUXIO18` reader - 2:2\\]
AUXIO18 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 2."]
pub type Auxio18R = crate::BitReader;
#[doc = "Field `AUXIO19` reader - 3:3\\]
AUXIO19 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 3."]
pub type Auxio19R = crate::BitReader;
#[doc = "Field `AUXIO20` reader - 4:4\\]
AUXIO20 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 4."]
pub type Auxio20R = crate::BitReader;
#[doc = "Field `AUXIO21` reader - 5:5\\]
AUXIO21 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 5."]
pub type Auxio21R = crate::BitReader;
#[doc = "Field `AUXIO22` reader - 6:6\\]
AUXIO22 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 6."]
pub type Auxio22R = crate::BitReader;
#[doc = "Field `AUXIO23` reader - 7:7\\]
AUXIO23 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 7."]
pub type Auxio23R = crate::BitReader;
#[doc = "Field `AUXIO24` reader - 8:8\\]
AUXIO24 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 0."]
pub type Auxio24R = crate::BitReader;
#[doc = "Field `AUXIO25` reader - 9:9\\]
AUXIO25 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 1."]
pub type Auxio25R = crate::BitReader;
#[doc = "Field `AUXIO26` reader - 10:10\\]
AUXIO26 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 2."]
pub type Auxio26R = crate::BitReader;
#[doc = "Field `AUXIO27` reader - 11:11\\]
AUXIO27 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 3."]
pub type Auxio27R = crate::BitReader;
#[doc = "Field `AUXIO28` reader - 12:12\\]
AUXIO28 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 4."]
pub type Auxio28R = crate::BitReader;
#[doc = "Field `AUXIO29` reader - 13:13\\]
AUXIO29 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 5."]
pub type Auxio29R = crate::BitReader;
#[doc = "Field `AUXIO30` reader - 14:14\\]
AUXIO30 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 6."]
pub type Auxio30R = crate::BitReader;
#[doc = "Field `AUXIO31` reader - 15:15\\]
AUXIO31 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 7."]
pub type Auxio31R = crate::BitReader;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
AUXIO16 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio16(&self) -> Auxio16R {
        Auxio16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO17 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio17(&self) -> Auxio17R {
        Auxio17R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO18 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio18(&self) -> Auxio18R {
        Auxio18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO19 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio19(&self) -> Auxio19R {
        Auxio19R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO20 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio20(&self) -> Auxio20R {
        Auxio20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO21 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio21(&self) -> Auxio21R {
        Auxio21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO22 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio22(&self) -> Auxio22R {
        Auxio22R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO23 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio23(&self) -> Auxio23R {
        Auxio23R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO24 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio24(&self) -> Auxio24R {
        Auxio24R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO25 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio25(&self) -> Auxio25R {
        Auxio25R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO26 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio26(&self) -> Auxio26R {
        Auxio26R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO27 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio27(&self) -> Auxio27R {
        Auxio27R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO28 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio28(&self) -> Auxio28R {
        Auxio28R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
AUXIO29 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio29(&self) -> Auxio29R {
        Auxio29R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
AUXIO30 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio30(&self) -> Auxio30R {
        Auxio30R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
AUXIO31 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio31(&self) -> Auxio31R {
        Auxio31R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Event Status 1 Register holds events 16 thru 31 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Evstat1Spec;
impl crate::RegisterSpec for Evstat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evstat1::R`](R) reader structure"]
impl crate::Readable for Evstat1Spec {}
#[doc = "`write(|w| ..)` method takes [`evstat1::W`](W) writer structure"]
impl crate::Writable for Evstat1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVSTAT1 to value 0"]
impl crate::Resettable for Evstat1Spec {
    const RESET_VALUE: u32 = 0;
}
