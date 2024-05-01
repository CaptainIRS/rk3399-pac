#[doc = "Register `BCSH_BCS` reader"]
pub type R = crate::R<BcshBcsSpec>;
#[doc = "Register `BCSH_BCS` writer"]
pub type W = crate::W<BcshBcsSpec>;
#[doc = "Field `BRIGHTNESS` reader - Brightness : -32,31"]
pub type BrightnessR = crate::FieldReader;
#[doc = "Field `BRIGHTNESS` writer - Brightness : -32,31"]
pub type BrightnessW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CONTRAST` reader - Contrast*256 : 0,1.992"]
pub type ContrastR = crate::FieldReader<u16>;
#[doc = "Field `CONTRAST` writer - Contrast*256 : 0,1.992"]
pub type ContrastW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SAT_CON` reader - Saturation*Contrast*256 : 0,1.992*1.992"]
pub type SatConR = crate::FieldReader<u16>;
#[doc = "Field `SAT_CON` writer - Saturation*Contrast*256 : 0,1.992*1.992"]
pub type SatConW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "video out mode config register\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OutMode {
    #[doc = "0: black"]
    B00 = 0,
    #[doc = "1: blue"]
    B01 = 1,
    #[doc = "2: color bar"]
    B10 = 2,
    #[doc = "3: normal video"]
    B11 = 3,
}
impl From<OutMode> for u8 {
    #[inline(always)]
    fn from(variant: OutMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OutMode {
    type Ux = u8;
}
#[doc = "Field `OUT_MODE` reader - video out mode config register"]
pub type OutModeR = crate::FieldReader<OutMode>;
impl OutModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OutMode {
        match self.bits {
            0 => OutMode::B00,
            1 => OutMode::B01,
            2 => OutMode::B10,
            3 => OutMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "black"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == OutMode::B00
    }
    #[doc = "blue"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == OutMode::B01
    }
    #[doc = "color bar"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == OutMode::B10
    }
    #[doc = "normal video"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == OutMode::B11
    }
}
#[doc = "Field `OUT_MODE` writer - video out mode config register"]
pub type OutModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, OutMode>;
impl<'a, REG> OutModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "black"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(OutMode::B00)
    }
    #[doc = "blue"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(OutMode::B01)
    }
    #[doc = "color bar"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(OutMode::B10)
    }
    #[doc = "normal video"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(OutMode::B11)
    }
}
impl R {
    #[doc = "Bits 0:7 - Brightness : -32,31"]
    #[inline(always)]
    pub fn brightness(&self) -> BrightnessR {
        BrightnessR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - Contrast*256 : 0,1.992"]
    #[inline(always)]
    pub fn contrast(&self) -> ContrastR {
        ContrastR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:29 - Saturation*Contrast*256 : 0,1.992*1.992"]
    #[inline(always)]
    pub fn sat_con(&self) -> SatConR {
        SatConR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 30:31 - video out mode config register"]
    #[inline(always)]
    pub fn out_mode(&self) -> OutModeR {
        OutModeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Brightness : -32,31"]
    #[inline(always)]
    #[must_use]
    pub fn brightness(&mut self) -> BrightnessW<BcshBcsSpec> {
        BrightnessW::new(self, 0)
    }
    #[doc = "Bits 8:16 - Contrast*256 : 0,1.992"]
    #[inline(always)]
    #[must_use]
    pub fn contrast(&mut self) -> ContrastW<BcshBcsSpec> {
        ContrastW::new(self, 8)
    }
    #[doc = "Bits 20:29 - Saturation*Contrast*256 : 0,1.992*1.992"]
    #[inline(always)]
    #[must_use]
    pub fn sat_con(&mut self) -> SatConW<BcshBcsSpec> {
        SatConW::new(self, 20)
    }
    #[doc = "Bits 30:31 - video out mode config register"]
    #[inline(always)]
    #[must_use]
    pub fn out_mode(&mut self) -> OutModeW<BcshBcsSpec> {
        OutModeW::new(self, 30)
    }
}
#[doc = "Brightness contrast saturation*contrast config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcsh_bcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcsh_bcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcshBcsSpec;
impl crate::RegisterSpec for BcshBcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcsh_bcs::R`](R) reader structure"]
impl crate::Readable for BcshBcsSpec {}
#[doc = "`write(|w| ..)` method takes [`bcsh_bcs::W`](W) writer structure"]
impl crate::Writable for BcshBcsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCSH_BCS to value 0xd001_0000"]
impl crate::Resettable for BcshBcsSpec {
    const RESET_VALUE: u32 = 0xd001_0000;
}
