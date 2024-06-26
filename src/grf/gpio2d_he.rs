#[doc = "Register `GPIO2D_HE` reader"]
pub type R = crate::R<Gpio2dHeSpec>;
#[doc = "Register `GPIO2D_HE` writer"]
pub type W = crate::W<Gpio2dHeSpec>;
#[doc = "GPIO2D gpio keep privous state control, every\n\nGPIO bit corresponding to 1bit\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2dHe {
    #[doc = "0: disable"]
    B0 = 0,
    #[doc = "1: enable"]
    B1 = 1,
}
impl From<Gpio2dHe> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2dHe) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2dHe {
    type Ux = u8;
}
#[doc = "Field `GPIO2D_HE` reader - GPIO2D gpio keep privous state control, every\n\nGPIO bit corresponding to 1bit"]
pub type Gpio2dHeR = crate::FieldReader<Gpio2dHe>;
impl Gpio2dHeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gpio2dHe> {
        match self.bits {
            0 => Some(Gpio2dHe::B0),
            1 => Some(Gpio2dHe::B1),
            _ => None,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gpio2dHe::B0
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gpio2dHe::B1
    }
}
#[doc = "Field `GPIO2D_HE` writer - GPIO2D gpio keep privous state control, every\n\nGPIO bit corresponding to 1bit"]
pub type Gpio2dHeW<'a, REG> = crate::FieldWriter<'a, REG, 5, Gpio2dHe>;
impl<'a, REG> Gpio2dHeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2dHe::B0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2dHe::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - GPIO2D gpio keep privous state control, every\n\nGPIO bit corresponding to 1bit"]
    #[inline(always)]
    pub fn gpio2d_he(&self) -> Gpio2dHeR {
        Gpio2dHeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - GPIO2D gpio keep privous state control, every\n\nGPIO bit corresponding to 1bit"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d_he(&mut self) -> Gpio2dHeW<Gpio2dHeSpec> {
        Gpio2dHeW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio2dHeSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2D HE control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2d_he::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2d_he::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio2dHeSpec;
impl crate::RegisterSpec for Gpio2dHeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio2d_he::R`](R) reader structure"]
impl crate::Readable for Gpio2dHeSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio2d_he::W`](W) writer structure"]
impl crate::Writable for Gpio2dHeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO2D_HE to value 0"]
impl crate::Resettable for Gpio2dHeSpec {
    const RESET_VALUE: u32 = 0;
}
