#[doc = "Register `GRF_GPIO2C_HE` reader"]
pub type R = crate::R<GrfGpio2cHeSpec>;
#[doc = "Register `GRF_GPIO2C_HE` writer"]
pub type W = crate::W<GrfGpio2cHeSpec>;
#[doc = "GPIO2C gpio keep privous state control, every\n\nGPIO bit corresponding to 1bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2cHe {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio2cHe> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2cHe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2cHe {
    type Ux = u8;
}
#[doc = "Field `GPIO2C_HE` reader - GPIO2C gpio keep privous state control, every\n\nGPIO bit corresponding to 1bit"]
pub type Gpio2cHeR = crate::FieldReader<Gpio2cHe>;
impl Gpio2cHeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio2cHe> {
        match self.bits {
            0 => Some(Gpio2cHe::B0),
            1 => Some(Gpio2cHe::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio2cHe::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio2cHe::B1
    }
}
#[doc = "Field `GPIO2C_HE` writer - GPIO2C gpio keep privous state control, every\n\nGPIO bit corresponding to 1bit"]
pub type Gpio2cHeW<'a, REG> = crate::FieldWriter<'a, REG, 8, Gpio2cHe>;
impl<'a, REG> Gpio2cHeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2cHe::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2cHe::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - GPIO2C gpio keep privous state control, every\n\nGPIO bit corresponding to 1bit"]
    #[inline(always)]
    pub fn gpio2c_he(&self) -> Gpio2cHeR {
        Gpio2cHeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO2C gpio keep privous state control, every\n\nGPIO bit corresponding to 1bit"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c_he(&mut self) -> Gpio2cHeW<GrfGpio2cHeSpec> {
        Gpio2cHeW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio2cHeSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2C HE control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2c_he::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2c_he::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio2cHeSpec;
impl crate::RegisterSpec for GrfGpio2cHeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio2c_he::R`](R) reader structure"]
impl crate::Readable for GrfGpio2cHeSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio2c_he::W`](W) writer structure"]
impl crate::Writable for GrfGpio2cHeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO2C_HE to value 0"]
impl crate::Resettable for GrfGpio2cHeSpec {
    const RESET_VALUE: u32 = 0;
}
