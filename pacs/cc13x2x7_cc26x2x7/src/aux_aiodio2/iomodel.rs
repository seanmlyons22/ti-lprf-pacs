#[doc = "Register `IOMODEL` reader"]
pub type R = crate::R<IomodelSpec>;
#[doc = "Register `IOMODEL` writer"]
pub type W = crate::W<IomodelSpec>;
#[doc = "Field `IO0` reader - 1:0\\]
See IOMODE.IO0."]
pub type Io0R = crate::FieldReader;
#[doc = "Field `IO0` writer - 1:0\\]
See IOMODE.IO0."]
pub type Io0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IO1` reader - 3:2\\]
See IOMODE.IO1."]
pub type Io1R = crate::FieldReader;
#[doc = "Field `IO1` writer - 3:2\\]
See IOMODE.IO1."]
pub type Io1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IO2` reader - 5:4\\]
See IOMODE.IO2."]
pub type Io2R = crate::FieldReader;
#[doc = "Field `IO2` writer - 5:4\\]
See IOMODE.IO2."]
pub type Io2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IO3` reader - 7:6\\]
See IOMODE.IO3."]
pub type Io3R = crate::FieldReader;
#[doc = "Field `IO3` writer - 7:6\\]
See IOMODE.IO3."]
pub type Io3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
See IOMODE.IO0."]
    #[inline(always)]
    pub fn io0(&self) -> Io0R {
        Io0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
See IOMODE.IO1."]
    #[inline(always)]
    pub fn io1(&self) -> Io1R {
        Io1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
See IOMODE.IO2."]
    #[inline(always)]
    pub fn io2(&self) -> Io2R {
        Io2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
See IOMODE.IO3."]
    #[inline(always)]
    pub fn io3(&self) -> Io3R {
        Io3R::new(((self.bits >> 6) & 3) as u8)
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
See IOMODE.IO0."]
    #[inline(always)]
    #[must_use]
    pub fn io0(&mut self) -> Io0W<IomodelSpec> {
        Io0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
See IOMODE.IO1."]
    #[inline(always)]
    #[must_use]
    pub fn io1(&mut self) -> Io1W<IomodelSpec> {
        Io1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
See IOMODE.IO2."]
    #[inline(always)]
    #[must_use]
    pub fn io2(&mut self) -> Io2W<IomodelSpec> {
        Io2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
See IOMODE.IO3."]
    #[inline(always)]
    #[must_use]
    pub fn io3(&mut self) -> Io3W<IomodelSpec> {
        Io3W::new(self, 6)
    }
}
#[doc = "Input Output Mode Low This is an alias register for IOMODE.IO0 thru IOMODE.IO3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iomodel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iomodel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IomodelSpec;
impl crate::RegisterSpec for IomodelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iomodel::R`](R) reader structure"]
impl crate::Readable for IomodelSpec {}
#[doc = "`write(|w| ..)` method takes [`iomodel::W`](W) writer structure"]
impl crate::Writable for IomodelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOMODEL to value 0"]
impl crate::Resettable for IomodelSpec {
    const RESET_VALUE: u32 = 0;
}
