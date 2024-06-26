#[doc = "Register `GPIO1D_IOMUX` reader"]
pub type R = crate::R<Gpio1dIomuxSpec>;
#[doc = "Register `GPIO1D_IOMUX` writer"]
pub type W = crate::W<Gpio1dIomuxSpec>;
#[doc = "GPIO1D\\[0\\]
iomux select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1d0Sel {
    #[doc = "0: gpio"]
    B00 = 0,
    #[doc = "1: tcpdusb2_vbussource2"]
    B01 = 1,
}
impl From<Gpio1d0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1d0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1d0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1D0_SEL` reader - GPIO1D\\[0\\]
iomux select"]
pub type Gpio1d0SelR = crate::FieldReader<Gpio1d0Sel>;
impl Gpio1d0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio1d0Sel> {
        match self.bits {
            0 => Some(Gpio1d0Sel::B00),
            1 => Some(Gpio1d0Sel::B01),
            _ => None,
        }
    }
    #[doc = "gpio"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1d0Sel::B00
    }
    #[doc = "tcpdusb2_vbussource2"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1d0Sel::B01
    }
}
#[doc = "Field `GPIO1D0_SEL` writer - GPIO1D\\[0\\]
iomux select"]
pub type Gpio1d0SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio1d0Sel>;
impl<'a, REG> Gpio1d0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "gpio"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1d0Sel::B00)
    }
    #[doc = "tcpdusb2_vbussource2"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1d0Sel::B01)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO1D\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1d0_sel(&self) -> Gpio1d0SelR {
        Gpio1d0SelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO1D\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1d0_sel(&mut self) -> Gpio1d0SelW<Gpio1dIomuxSpec> {
        Gpio1d0SelW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio1dIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO1D iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio1d_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio1d_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio1dIomuxSpec;
impl crate::RegisterSpec for Gpio1dIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio1d_iomux::R`](R) reader structure"]
impl crate::Readable for Gpio1dIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio1d_iomux::W`](W) writer structure"]
impl crate::Writable for Gpio1dIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO1D_IOMUX to value 0"]
impl crate::Resettable for Gpio1dIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
