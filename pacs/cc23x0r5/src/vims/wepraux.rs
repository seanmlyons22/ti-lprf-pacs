#[doc = "Register `WEPRAUX` reader"]
pub type R = crate::R<WeprauxSpec>;
#[doc = "Register `WEPRAUX` writer"]
pub type W = crate::W<WeprauxSpec>;
#[doc = "Field `WEPRNMN` reader - 0:0\\]
Flash non main region write/erase protection configuration value."]
pub type WeprnmnR = crate::BitReader;
#[doc = "Field `WEPRNMN` writer - 0:0\\]
Flash non main region write/erase protection configuration value."]
pub type WeprnmnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEPRTRM` reader - 1:1\\]
Flash trim region write/erase protection configuration value."]
pub type WeprtrmR = crate::BitReader;
#[doc = "Field `WEPRTRM` writer - 1:1\\]
Flash trim region write/erase protection configuration value."]
pub type WeprtrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEPREGR` reader - 2:2\\]
Flash engr region write/erase protection configuration value."]
pub type WepregrR = crate::BitReader;
#[doc = "Field `WEPREGR` writer - 2:2\\]
Flash engr region write/erase protection configuration value."]
pub type WepregrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Flash non main region write/erase protection configuration value."]
    #[inline(always)]
    pub fn weprnmn(&self) -> WeprnmnR {
        WeprnmnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Flash trim region write/erase protection configuration value."]
    #[inline(always)]
    pub fn weprtrm(&self) -> WeprtrmR {
        WeprtrmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Flash engr region write/erase protection configuration value."]
    #[inline(always)]
    pub fn wepregr(&self) -> WepregrR {
        WepregrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Flash non main region write/erase protection configuration value."]
    #[inline(always)]
    #[must_use]
    pub fn weprnmn(&mut self) -> WeprnmnW<WeprauxSpec> {
        WeprnmnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Flash trim region write/erase protection configuration value."]
    #[inline(always)]
    #[must_use]
    pub fn weprtrm(&mut self) -> WeprtrmW<WeprauxSpec> {
        WeprtrmW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Flash engr region write/erase protection configuration value."]
    #[inline(always)]
    #[must_use]
    pub fn wepregr(&mut self) -> WepregrW<WeprauxSpec> {
        WepregrW::new(self, 2)
    }
}
#[doc = "Flash Write/Erase protection for Non-Main, TRIM and ENGR Regions. This register is sticky when written with value 0. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wepraux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wepraux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WeprauxSpec;
impl crate::RegisterSpec for WeprauxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wepraux::R`](R) reader structure"]
impl crate::Readable for WeprauxSpec {}
#[doc = "`write(|w| ..)` method takes [`wepraux::W`](W) writer structure"]
impl crate::Writable for WeprauxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WEPRAUX to value 0x07"]
impl crate::Resettable for WeprauxSpec {
    const RESET_VALUE: u32 = 0x07;
}
