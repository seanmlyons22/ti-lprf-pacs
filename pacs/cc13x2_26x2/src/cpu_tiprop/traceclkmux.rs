#[doc = "Register `TRACECLKMUX` reader"]
pub type R = crate::R<TraceclkmuxSpec>;
#[doc = "Register `TRACECLKMUX` writer"]
pub type W = crate::W<TraceclkmuxSpec>;
#[doc = "0:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceclkNSwv {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Traceclk = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Swv = 0,
}
impl From<TraceclkNSwv> for bool {
    #[inline(always)]
    fn from(variant: TraceclkNSwv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRACECLK_N_SWV` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type TraceclkNSwvR = crate::BitReader<TraceclkNSwv>;
impl TraceclkNSwvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TraceclkNSwv {
        match self.bits {
            true => TraceclkNSwv::Traceclk,
            false => TraceclkNSwv::Swv,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_traceclk(&self) -> bool {
        *self == TraceclkNSwv::Traceclk
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_swv(&self) -> bool {
        *self == TraceclkNSwv::Swv
    }
}
#[doc = "Field `TRACECLK_N_SWV` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type TraceclkNSwvW<'a, REG> = crate::BitWriter<'a, REG, TraceclkNSwv>;
impl<'a, REG> TraceclkNSwvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn traceclk(self) -> &'a mut crate::W<REG> {
        self.variant(TraceclkNSwv::Traceclk)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn swv(self) -> &'a mut crate::W<REG> {
        self.variant(TraceclkNSwv::Swv)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn traceclk_n_swv(&self) -> TraceclkNSwvR {
        TraceclkNSwvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn traceclk_n_swv(&mut self) -> TraceclkNSwvW<TraceclkmuxSpec> {
        TraceclkNSwvW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<TraceclkmuxSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`traceclkmux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`traceclkmux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TraceclkmuxSpec;
impl crate::RegisterSpec for TraceclkmuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`traceclkmux::R`](R) reader structure"]
impl crate::Readable for TraceclkmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`traceclkmux::W`](W) writer structure"]
impl crate::Writable for TraceclkmuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRACECLKMUX to value 0"]
impl crate::Resettable for TraceclkmuxSpec {
    const RESET_VALUE: u32 = 0;
}
