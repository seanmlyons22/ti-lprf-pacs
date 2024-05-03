#[doc = "Register `KEYREADAREA` reader"]
pub type R = crate::R<KeyreadareaSpec>;
#[doc = "Register `KEYREADAREA` writer"]
pub type W = crate::W<KeyreadareaSpec>;
#[doc = "3:0\\]
Selects the area of the key store RAM from where the key needs to be read that will be written to the AES engine. Only RAM areas that contain valid written keys can be selected.\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RamArea {
    #[doc = "8: No RAM"]
    NoRam = 8,
    #[doc = "7: RAM Area 7"]
    RamArea7 = 7,
    #[doc = "6: RAM Area 6"]
    RamArea6 = 6,
    #[doc = "5: RAM Area 5"]
    RamArea5 = 5,
    #[doc = "4: RAM Area 4"]
    RamArea4 = 4,
    #[doc = "3: RAM Area 3"]
    RamArea3 = 3,
    #[doc = "2: RAM Area 2"]
    RamArea2 = 2,
    #[doc = "1: RAM Area 1"]
    RamArea1 = 1,
    #[doc = "0: RAM Area 0"]
    RamArea0 = 0,
}
impl From<RamArea> for u8 {
    #[inline(always)]
    fn from(variant: RamArea) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RamArea {
    type Ux = u8;
}
impl crate::IsEnum for RamArea {}
#[doc = "Field `RAM_AREA` reader - 3:0\\]
Selects the area of the key store RAM from where the key needs to be read that will be written to the AES engine. Only RAM areas that contain valid written keys can be selected."]
pub type RamAreaR = crate::FieldReader<RamArea>;
impl RamAreaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RamArea> {
        match self.bits {
            8 => Some(RamArea::NoRam),
            7 => Some(RamArea::RamArea7),
            6 => Some(RamArea::RamArea6),
            5 => Some(RamArea::RamArea5),
            4 => Some(RamArea::RamArea4),
            3 => Some(RamArea::RamArea3),
            2 => Some(RamArea::RamArea2),
            1 => Some(RamArea::RamArea1),
            0 => Some(RamArea::RamArea0),
            _ => None,
        }
    }
    #[doc = "No RAM"]
    #[inline(always)]
    pub fn is_no_ram(&self) -> bool {
        *self == RamArea::NoRam
    }
    #[doc = "RAM Area 7"]
    #[inline(always)]
    pub fn is_ram_area7(&self) -> bool {
        *self == RamArea::RamArea7
    }
    #[doc = "RAM Area 6"]
    #[inline(always)]
    pub fn is_ram_area6(&self) -> bool {
        *self == RamArea::RamArea6
    }
    #[doc = "RAM Area 5"]
    #[inline(always)]
    pub fn is_ram_area5(&self) -> bool {
        *self == RamArea::RamArea5
    }
    #[doc = "RAM Area 4"]
    #[inline(always)]
    pub fn is_ram_area4(&self) -> bool {
        *self == RamArea::RamArea4
    }
    #[doc = "RAM Area 3"]
    #[inline(always)]
    pub fn is_ram_area3(&self) -> bool {
        *self == RamArea::RamArea3
    }
    #[doc = "RAM Area 2"]
    #[inline(always)]
    pub fn is_ram_area2(&self) -> bool {
        *self == RamArea::RamArea2
    }
    #[doc = "RAM Area 1"]
    #[inline(always)]
    pub fn is_ram_area1(&self) -> bool {
        *self == RamArea::RamArea1
    }
    #[doc = "RAM Area 0"]
    #[inline(always)]
    pub fn is_ram_area0(&self) -> bool {
        *self == RamArea::RamArea0
    }
}
#[doc = "Field `RAM_AREA` writer - 3:0\\]
Selects the area of the key store RAM from where the key needs to be read that will be written to the AES engine. Only RAM areas that contain valid written keys can be selected."]
pub type RamAreaW<'a, REG> = crate::FieldWriter<'a, REG, 4, RamArea>;
impl<'a, REG> RamAreaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No RAM"]
    #[inline(always)]
    pub fn no_ram(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea::NoRam)
    }
    #[doc = "RAM Area 7"]
    #[inline(always)]
    pub fn ram_area7(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea::RamArea7)
    }
    #[doc = "RAM Area 6"]
    #[inline(always)]
    pub fn ram_area6(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea::RamArea6)
    }
    #[doc = "RAM Area 5"]
    #[inline(always)]
    pub fn ram_area5(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea::RamArea5)
    }
    #[doc = "RAM Area 4"]
    #[inline(always)]
    pub fn ram_area4(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea::RamArea4)
    }
    #[doc = "RAM Area 3"]
    #[inline(always)]
    pub fn ram_area3(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea::RamArea3)
    }
    #[doc = "RAM Area 2"]
    #[inline(always)]
    pub fn ram_area2(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea::RamArea2)
    }
    #[doc = "RAM Area 1"]
    #[inline(always)]
    pub fn ram_area1(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea::RamArea1)
    }
    #[doc = "RAM Area 0"]
    #[inline(always)]
    pub fn ram_area0(self) -> &'a mut crate::W<REG> {
        self.variant(RamArea::RamArea0)
    }
}
#[doc = "Field `RESERVED4` reader - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
#[doc = "Field `BUSY` reader - 31:31\\]
Key store operation busy status flag (read only) 0: operation is completed. 1: operation is not completed and the key store is busy."]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - 31:31\\]
Key store operation busy status flag (read only) 0: operation is completed. 1: operation is not completed and the key store is busy."]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Selects the area of the key store RAM from where the key needs to be read that will be written to the AES engine. Only RAM areas that contain valid written keys can be selected."]
    #[inline(always)]
    pub fn ram_area(&self) -> RamAreaR {
        RamAreaR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:30 - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x07ff_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
Key store operation busy status flag (read only) 0: operation is completed. 1: operation is not completed and the key store is busy."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Selects the area of the key store RAM from where the key needs to be read that will be written to the AES engine. Only RAM areas that contain valid written keys can be selected."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area(&mut self) -> RamAreaW<KeyreadareaSpec> {
        RamAreaW::new(self, 0)
    }
    #[doc = "Bits 4:30 - 30:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<KeyreadareaSpec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bit 31 - 31:31\\]
Key store operation busy status flag (read only) 0: operation is completed. 1: operation is not completed and the key store is busy."]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<KeyreadareaSpec> {
        BusyW::new(self, 31)
    }
}
#[doc = "Key Read Area\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyreadarea::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyreadarea::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyreadareaSpec;
impl crate::RegisterSpec for KeyreadareaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyreadarea::R`](R) reader structure"]
impl crate::Readable for KeyreadareaSpec {}
#[doc = "`write(|w| ..)` method takes [`keyreadarea::W`](W) writer structure"]
impl crate::Writable for KeyreadareaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEYREADAREA to value 0x08"]
impl crate::Resettable for KeyreadareaSpec {
    const RESET_VALUE: u32 = 0x08;
}
