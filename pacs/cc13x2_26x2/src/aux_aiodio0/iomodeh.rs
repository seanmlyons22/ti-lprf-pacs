#[doc = "Register `IOMODEH` reader"]
pub type R = crate::R<IomodehSpec>;
#[doc = "Register `IOMODEH` writer"]
pub type W = crate::W<IomodehSpec>;
#[doc = "Field `IO4` reader - 1:0\\]
See IOMODE.IO4."]
pub type Io4R = crate::FieldReader;
#[doc = "Field `IO4` writer - 1:0\\]
See IOMODE.IO4."]
pub type Io4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IO5` reader - 3:2\\]
See IOMODE.IO5."]
pub type Io5R = crate::FieldReader;
#[doc = "Field `IO5` writer - 3:2\\]
See IOMODE.IO5."]
pub type Io5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IO6` reader - 5:4\\]
See IOMODE.IO6."]
pub type Io6R = crate::FieldReader;
#[doc = "Field `IO6` writer - 5:4\\]
See IOMODE.IO6."]
pub type Io6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IO7` reader - 7:6\\]
See IOMODE.IO7."]
pub type Io7R = crate::FieldReader;
#[doc = "Field `IO7` writer - 7:6\\]
See IOMODE.IO7."]
pub type Io7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
See IOMODE.IO4."]
    #[inline(always)]
    pub fn io4(&self) -> Io4R {
        Io4R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
See IOMODE.IO5."]
    #[inline(always)]
    pub fn io5(&self) -> Io5R {
        Io5R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
See IOMODE.IO6."]
    #[inline(always)]
    pub fn io6(&self) -> Io6R {
        Io6R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
See IOMODE.IO7."]
    #[inline(always)]
    pub fn io7(&self) -> Io7R {
        Io7R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
See IOMODE.IO4."]
    #[inline(always)]
    #[must_use]
    pub fn io4(&mut self) -> Io4W<IomodehSpec> {
        Io4W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
See IOMODE.IO5."]
    #[inline(always)]
    #[must_use]
    pub fn io5(&mut self) -> Io5W<IomodehSpec> {
        Io5W::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
See IOMODE.IO6."]
    #[inline(always)]
    #[must_use]
    pub fn io6(&mut self) -> Io6W<IomodehSpec> {
        Io6W::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
See IOMODE.IO7."]
    #[inline(always)]
    #[must_use]
    pub fn io7(&mut self) -> Io7W<IomodehSpec> {
        Io7W::new(self, 6)
    }
}
#[doc = "Input Output Mode High This is an alias register for IOMODE.IO4 thru IOMODE.IO7.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iomodeh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iomodeh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IomodehSpec;
impl crate::RegisterSpec for IomodehSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomodeh::R`](R) reader structure"]
impl crate::Readable for IomodehSpec {}
#[doc = "`write(|w| ..)` method takes [`iomodeh::W`](W) writer structure"]
impl crate::Writable for IomodehSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOMODEH to value 0"]
impl crate::Resettable for IomodehSpec {
    const RESET_VALUE: u32 = 0;
}
