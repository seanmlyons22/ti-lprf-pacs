#[doc = "Register `QDECSTAT` reader"]
pub type R = crate::R<QdecstatSpec>;
#[doc = "Register `QDECSTAT` writer"]
pub type W = crate::W<QdecstatSpec>;
#[doc = "0:0\\]
Direction of count during QDEC mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Qdir {
    #[doc = "1: Down (PHB leads PHA)"]
    Down = 1,
    #[doc = "0: Up (PHA leads PHB)"]
    Up = 0,
}
impl From<Qdir> for bool {
    #[inline(always)]
    fn from(variant: Qdir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QDIR` reader - 0:0\\]
Direction of count during QDEC mode."]
pub type QdirR = crate::BitReader<Qdir>;
impl QdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qdir {
        match self.bits {
            true => Qdir::Down,
            false => Qdir::Up,
        }
    }
    #[doc = "Down (PHB leads PHA)"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Qdir::Down
    }
    #[doc = "Up (PHA leads PHB)"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Qdir::Up
    }
}
#[doc = "Field `QDIR` writer - 0:0\\]
Direction of count during QDEC mode."]
pub type QdirW<'a, REG> = crate::BitWriter<'a, REG, Qdir>;
impl<'a, REG> QdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Down (PHB leads PHA)"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Qdir::Down)
    }
    #[doc = "Up (PHA leads PHB)"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Qdir::Up)
    }
}
#[doc = "1:1\\]
Double transition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbltrans {
    #[doc = "1: Double transition on phase inputs."]
    Dbl = 1,
    #[doc = "0: Single or no transition on phase inputs."]
    None = 0,
}
impl From<Dbltrans> for bool {
    #[inline(always)]
    fn from(variant: Dbltrans) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBLTRANS` reader - 1:1\\]
Double transition"]
pub type DbltransR = crate::BitReader<Dbltrans>;
impl DbltransR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbltrans {
        match self.bits {
            true => Dbltrans::Dbl,
            false => Dbltrans::None,
        }
    }
    #[doc = "Double transition on phase inputs."]
    #[inline(always)]
    pub fn is_dbl(&self) -> bool {
        *self == Dbltrans::Dbl
    }
    #[doc = "Single or no transition on phase inputs."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Dbltrans::None
    }
}
#[doc = "Field `DBLTRANS` writer - 1:1\\]
Double transition"]
pub type DbltransW<'a, REG> = crate::BitWriter<'a, REG, Dbltrans>;
impl<'a, REG> DbltransW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Double transition on phase inputs."]
    #[inline(always)]
    pub fn dbl(self) -> &'a mut crate::W<REG> {
        self.variant(Dbltrans::Dbl)
    }
    #[doc = "Single or no transition on phase inputs."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Dbltrans::None)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Direction of count during QDEC mode."]
    #[inline(always)]
    pub fn qdir(&self) -> QdirR {
        QdirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Double transition"]
    #[inline(always)]
    pub fn dbltrans(&self) -> DbltransR {
        DbltransR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Direction of count during QDEC mode."]
    #[inline(always)]
    #[must_use]
    pub fn qdir(&mut self) -> QdirW<QdecstatSpec> {
        QdirW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Double transition"]
    #[inline(always)]
    #[must_use]
    pub fn dbltrans(&mut self) -> DbltransW<QdecstatSpec> {
        DbltransW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<QdecstatSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Quadrature Decoder Status This register can be used during QDEC mode to check the status of the quadrature decoder.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qdecstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qdecstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QdecstatSpec;
impl crate::RegisterSpec for QdecstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdecstat::R`](R) reader structure"]
impl crate::Readable for QdecstatSpec {}
#[doc = "`write(|w| ..)` method takes [`qdecstat::W`](W) writer structure"]
impl crate::Writable for QdecstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QDECSTAT to value 0"]
impl crate::Resettable for QdecstatSpec {
    const RESET_VALUE: u32 = 0;
}
