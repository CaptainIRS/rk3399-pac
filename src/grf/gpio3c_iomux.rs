#[doc = "Register `GPIO3C_IOMUX` reader"]
pub type R = crate::R<Gpio3cIomuxSpec>;
#[doc = "Register `GPIO3C_IOMUX` writer"]
pub type W = crate::W<Gpio3cIomuxSpec>;
#[doc = "GPIO3C\\[0\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3c0Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: mac_col"]
    B01 = 1,
    #[doc = "2: uart3gps_ctsn"]
    B10 = 2,
    #[doc = "3: spdif_txb"]
    B11 = 3,
}
impl From<Gpio3c0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3c0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3c0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3C0_SEL` reader - GPIO3C\\[0\\]
iomux select"]
pub type Gpio3c0SelR = crate::FieldReader<Gpio3c0Sel>;
impl Gpio3c0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3c0Sel {
        match self.bits {
            0 => Gpio3c0Sel::B00,
            1 => Gpio3c0Sel::B01,
            2 => Gpio3c0Sel::B10,
            3 => Gpio3c0Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3c0Sel::B00
    }
    #[doc = "mac_col"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3c0Sel::B01
    }
    #[doc = "uart3gps_ctsn"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3c0Sel::B10
    }
    #[doc = "spdif_txb"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3c0Sel::B11
    }
}
#[doc = "Field `GPIO3C0_SEL` writer - GPIO3C\\[0\\]
iomux select"]
pub type Gpio3c0SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3c0Sel>;
impl<'a, REG> Gpio3c0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3c0Sel::B00)
    }
    #[doc = "mac_col"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3c0Sel::B01)
    }
    #[doc = "uart3gps_ctsn"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3c0Sel::B10)
    }
    #[doc = "spdif_txb"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3c0Sel::B11)
    }
}
#[doc = "GPIO3C\\[1\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3c1Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: mac_txclk"]
    B01 = 1,
    #[doc = "2: uart3gps_rtsn"]
    B10 = 2,
}
impl From<Gpio3c1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3c1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3c1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3C1_SEL` reader - GPIO3C\\[1\\]
iomux select"]
pub type Gpio3c1SelR = crate::FieldReader<Gpio3c1Sel>;
impl Gpio3c1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio3c1Sel> {
        match self.bits {
            0 => Some(Gpio3c1Sel::B00),
            1 => Some(Gpio3c1Sel::B01),
            2 => Some(Gpio3c1Sel::B10),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3c1Sel::B00
    }
    #[doc = "mac_txclk"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3c1Sel::B01
    }
    #[doc = "uart3gps_rtsn"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3c1Sel::B10
    }
}
#[doc = "Field `GPIO3C1_SEL` writer - GPIO3C\\[1\\]
iomux select"]
pub type Gpio3c1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio3c1Sel>;
impl<'a, REG> Gpio3c1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3c1Sel::B00)
    }
    #[doc = "mac_txclk"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3c1Sel::B01)
    }
    #[doc = "uart3gps_rtsn"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3c1Sel::B10)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO3C\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3c0_sel(&self) -> Gpio3c0SelR {
        Gpio3c0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO3C\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3c1_sel(&self) -> Gpio3c1SelR {
        Gpio3c1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO3C\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3c0_sel(&mut self) -> Gpio3c0SelW<Gpio3cIomuxSpec> {
        Gpio3c0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO3C\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3c1_sel(&mut self) -> Gpio3c1SelW<Gpio3cIomuxSpec> {
        Gpio3c1SelW::new(self, 2)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio3cIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO3C iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3c_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3c_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio3cIomuxSpec;
impl crate::RegisterSpec for Gpio3cIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio3c_iomux::R`](R) reader structure"]
impl crate::Readable for Gpio3cIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio3c_iomux::W`](W) writer structure"]
impl crate::Writable for Gpio3cIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO3C_IOMUX to value 0"]
impl crate::Resettable for Gpio3cIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
