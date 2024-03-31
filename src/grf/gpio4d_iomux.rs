#[doc = "Register `GPIO4D_IOMUX` reader"]
pub type R = crate::R<Gpio4dIomuxSpec>;
#[doc = "Register `GPIO4D_IOMUX` writer"]
pub type W = crate::W<Gpio4dIomuxSpec>;
#[doc = "GPIO4D\\[0\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4d0Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: pcie_clkreqnb"]
    B01 = 1,
}
impl From<Gpio4d0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4d0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4d0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4D0_SEL` reader - GPIO4D\\[0\\]
iomux select"]
pub type Gpio4d0SelR = crate::FieldReader<Gpio4d0Sel>;
impl Gpio4d0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio4d0Sel> {
        match self.bits {
            0 => Some(Gpio4d0Sel::B00),
            1 => Some(Gpio4d0Sel::B01),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4d0Sel::B00
    }
    #[doc = "pcie_clkreqnb"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4d0Sel::B01
    }
}
#[doc = "Field `GPIO4D0_SEL` writer - GPIO4D\\[0\\]
iomux select"]
pub type Gpio4d0SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio4d0Sel>;
impl<'a, REG> Gpio4d0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d0Sel::B00)
    }
    #[doc = "pcie_clkreqnb"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d0Sel::B01)
    }
}
#[doc = "GPIO4D\\[1\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4d1Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: dp_hotplug"]
    B01 = 1,
}
impl From<Gpio4d1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4d1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4d1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4D1_SEL` reader - GPIO4D\\[1\\]
iomux select"]
pub type Gpio4d1SelR = crate::FieldReader<Gpio4d1Sel>;
impl Gpio4d1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio4d1Sel> {
        match self.bits {
            0 => Some(Gpio4d1Sel::B00),
            1 => Some(Gpio4d1Sel::B01),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4d1Sel::B00
    }
    #[doc = "dp_hotplug"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4d1Sel::B01
    }
}
#[doc = "Field `GPIO4D1_SEL` writer - GPIO4D\\[1\\]
iomux select"]
pub type Gpio4d1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio4d1Sel>;
impl<'a, REG> Gpio4d1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d1Sel::B00)
    }
    #[doc = "dp_hotplug"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d1Sel::B01)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO4D\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4d0_sel(&self) -> Gpio4d0SelR {
        Gpio4d0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO4D\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4d1_sel(&self) -> Gpio4d1SelR {
        Gpio4d1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO4D\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4d0_sel(&mut self) -> Gpio4d0SelW<Gpio4dIomuxSpec> {
        Gpio4d0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO4D\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4d1_sel(&mut self) -> Gpio4d1SelW<Gpio4dIomuxSpec> {
        Gpio4d1SelW::new(self, 2)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio4dIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO4D iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4d_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4d_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio4dIomuxSpec;
impl crate::RegisterSpec for Gpio4dIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio4d_iomux::R`](R) reader structure"]
impl crate::Readable for Gpio4dIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio4d_iomux::W`](W) writer structure"]
impl crate::Writable for Gpio4dIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO4D_IOMUX to value 0"]
impl crate::Resettable for Gpio4dIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
