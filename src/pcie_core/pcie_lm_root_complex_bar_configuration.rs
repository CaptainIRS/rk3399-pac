#[doc = "Register `PCIE_LM_ROOT_COMPLEX_BAR_CONFIGURATION` reader"]
pub type R = crate::R<PcieLmRootComplexBarConfigurationSpec>;
#[doc = "Register `PCIE_LM_ROOT_COMPLEX_BAR_CONFIGURATION` writer"]
pub type W = crate::W<PcieLmRootComplexBarConfigurationSpec>;
#[doc = "Field `RCBAR0A` reader - RC BAR 0 Aperture \\[RCBAR0A\\]
This field specifies the aperture of the RC BAR 0. The encodings are: 0000 = 4, 00001 =8B, ..... 01_1111 = 8G, ....10_0100 = 256G."]
pub type Rcbar0aR = crate::FieldReader;
#[doc = "Field `RCBAR0A` writer - RC BAR 0 Aperture \\[RCBAR0A\\]
This field specifies the aperture of the RC BAR 0. The encodings are: 0000 = 4, 00001 =8B, ..... 01_1111 = 8G, ....10_0100 = 256G."]
pub type Rcbar0aW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "RC BAR 0 control \\[RCBAR0C\\]
Specifies the configuration of RC BAR0. The various encodings are:\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rcbar0c {
    #[doc = "0: 64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    B000 = 0,
    #[doc = "6: 64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    B110 = 6,
}
impl From<Rcbar0c> for u8 {
    #[inline(always)]
    fn from(variant: Rcbar0c) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rcbar0c {
    type Ux = u8;
}
#[doc = "Field `RCBAR0C` reader - RC BAR 0 control \\[RCBAR0C\\]
Specifies the configuration of RC BAR0. The various encodings are:"]
pub type Rcbar0cR = crate::FieldReader<Rcbar0c>;
impl Rcbar0cR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rcbar0c> {
        match self.bits {
            0 => Some(Rcbar0c::B000),
            6 => Some(Rcbar0c::B110),
            _ => None,
        }
    }
    #[doc = "64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Rcbar0c::B000
    }
    #[doc = "64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Rcbar0c::B110
    }
}
#[doc = "Field `RCBAR0C` writer - RC BAR 0 control \\[RCBAR0C\\]
Specifies the configuration of RC BAR0. The various encodings are:"]
pub type Rcbar0cW<'a, REG> = crate::FieldWriter<'a, REG, 3, Rcbar0c>;
impl<'a, REG> Rcbar0cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Rcbar0c::B000)
    }
    #[doc = "64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Rcbar0c::B110)
    }
}
#[doc = "Field `RCBAR1A` reader - RC BAR 1 Aperture \\[RCBAR1A\\]
This field specifies the aperture of the RC BAR 1. The encodings are: 0000 = 4, 00001 =8B, ..... 1_1101 = 2G"]
pub type Rcbar1aR = crate::FieldReader;
#[doc = "Field `RCBAR1A` writer - RC BAR 1 Aperture \\[RCBAR1A\\]
This field specifies the aperture of the RC BAR 1. The encodings are: 0000 = 4, 00001 =8B, ..... 1_1101 = 2G"]
pub type Rcbar1aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RCBAR1C` reader - RC BAR 1 control \\[RCBAR1C\\]
Specifies the configuration of RC BAR1. The various encodings are: 000: Disabled 001: 32bit IO BAR 010-011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
pub type Rcbar1cR = crate::FieldReader;
#[doc = "Field `RCBAR1C` writer - RC BAR 1 control \\[RCBAR1C\\]
Specifies the configuration of RC BAR1. The various encodings are: 000: Disabled 001: 32bit IO BAR 010-011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
pub type Rcbar1cW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RCBARPME` reader - Type1 cfg prefetchable mem bar enable \\[RCBARPME\\]
Enable for Prefetchable memory base and limit registers in type1 config space"]
pub type RcbarpmeR = crate::BitReader;
#[doc = "Field `RCBARPME` writer - Type1 cfg prefetchable mem bar enable \\[RCBARPME\\]
Enable for Prefetchable memory base and limit registers in type1 config space"]
pub type RcbarpmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCBARPMS` reader - Type1 cfg prefetchable mem bar size \\[RCBARPMS\\]
Width of Prefetchable Memory Base and Limit registers in type1 config space. 0=32 bits, 1=64bits"]
pub type RcbarpmsR = crate::BitReader;
#[doc = "Field `RCBARPMS` writer - Type1 cfg prefetchable mem bar size \\[RCBARPMS\\]
Width of Prefetchable Memory Base and Limit registers in type1 config space. 0=32 bits, 1=64bits"]
pub type RcbarpmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCBARPIE` reader - Type1 cfg IO bar enable \\[RCBARPIE\\]
Enable for IO Base and Limit registers in type1 config space"]
pub type RcbarpieR = crate::BitReader;
#[doc = "Field `RCBARPIE` writer - Type1 cfg IO bar enable \\[RCBARPIE\\]
Enable for IO Base and Limit registers in type1 config space"]
pub type RcbarpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCBARPIS` reader - Type1 cfg IO bar size \\[RCBARPIS\\]
Width of IO Base and Limit registers in type1 config space. 0=32 bits, 1=64bits"]
pub type RcbarpisR = crate::BitReader;
#[doc = "Field `RCBARPIS` writer - Type1 cfg IO bar size \\[RCBARPIS\\]
Width of IO Base and Limit registers in type1 config space. 0=32 bits, 1=64bits"]
pub type RcbarpisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R10` reader - Reserved \\[R10\\]
Reserved"]
pub type R10R = crate::FieldReader<u16>;
#[doc = "Field `RCBCE` reader - RC BAR Check Enable \\[RCBCE\\]
This bit must be set to 1 to enable BAR checking in the RC mode. When this bit is set to 0, the core will forward all incoming memory requests to the client logic without checking their address ranges."]
pub type RcbceR = crate::BitReader;
#[doc = "Field `RCBCE` writer - RC BAR Check Enable \\[RCBCE\\]
This bit must be set to 1 to enable BAR checking in the RC mode. When this bit is set to 0, the core will forward all incoming memory requests to the client logic without checking their address ranges."]
pub type RcbceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - RC BAR 0 Aperture \\[RCBAR0A\\]
This field specifies the aperture of the RC BAR 0. The encodings are: 0000 = 4, 00001 =8B, ..... 01_1111 = 8G, ....10_0100 = 256G."]
    #[inline(always)]
    pub fn rcbar0a(&self) -> Rcbar0aR {
        Rcbar0aR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:8 - RC BAR 0 control \\[RCBAR0C\\]
Specifies the configuration of RC BAR0. The various encodings are:"]
    #[inline(always)]
    pub fn rcbar0c(&self) -> Rcbar0cR {
        Rcbar0cR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:13 - RC BAR 1 Aperture \\[RCBAR1A\\]
This field specifies the aperture of the RC BAR 1. The encodings are: 0000 = 4, 00001 =8B, ..... 1_1101 = 2G"]
    #[inline(always)]
    pub fn rcbar1a(&self) -> Rcbar1aR {
        Rcbar1aR::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:16 - RC BAR 1 control \\[RCBAR1C\\]
Specifies the configuration of RC BAR1. The various encodings are: 000: Disabled 001: 32bit IO BAR 010-011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub fn rcbar1c(&self) -> Rcbar1cR {
        Rcbar1cR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17 - Type1 cfg prefetchable mem bar enable \\[RCBARPME\\]
Enable for Prefetchable memory base and limit registers in type1 config space"]
    #[inline(always)]
    pub fn rcbarpme(&self) -> RcbarpmeR {
        RcbarpmeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Type1 cfg prefetchable mem bar size \\[RCBARPMS\\]
Width of Prefetchable Memory Base and Limit registers in type1 config space. 0=32 bits, 1=64bits"]
    #[inline(always)]
    pub fn rcbarpms(&self) -> RcbarpmsR {
        RcbarpmsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Type1 cfg IO bar enable \\[RCBARPIE\\]
Enable for IO Base and Limit registers in type1 config space"]
    #[inline(always)]
    pub fn rcbarpie(&self) -> RcbarpieR {
        RcbarpieR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Type1 cfg IO bar size \\[RCBARPIS\\]
Width of IO Base and Limit registers in type1 config space. 0=32 bits, 1=64bits"]
    #[inline(always)]
    pub fn rcbarpis(&self) -> RcbarpisR {
        RcbarpisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:30 - Reserved \\[R10\\]
Reserved"]
    #[inline(always)]
    pub fn r10(&self) -> R10R {
        R10R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - RC BAR Check Enable \\[RCBCE\\]
This bit must be set to 1 to enable BAR checking in the RC mode. When this bit is set to 0, the core will forward all incoming memory requests to the client logic without checking their address ranges."]
    #[inline(always)]
    pub fn rcbce(&self) -> RcbceR {
        RcbceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - RC BAR 0 Aperture \\[RCBAR0A\\]
This field specifies the aperture of the RC BAR 0. The encodings are: 0000 = 4, 00001 =8B, ..... 01_1111 = 8G, ....10_0100 = 256G."]
    #[inline(always)]
    #[must_use]
    pub fn rcbar0a(&mut self) -> Rcbar0aW<PcieLmRootComplexBarConfigurationSpec> {
        Rcbar0aW::new(self, 0)
    }
    #[doc = "Bits 6:8 - RC BAR 0 control \\[RCBAR0C\\]
Specifies the configuration of RC BAR0. The various encodings are:"]
    #[inline(always)]
    #[must_use]
    pub fn rcbar0c(&mut self) -> Rcbar0cW<PcieLmRootComplexBarConfigurationSpec> {
        Rcbar0cW::new(self, 6)
    }
    #[doc = "Bits 9:13 - RC BAR 1 Aperture \\[RCBAR1A\\]
This field specifies the aperture of the RC BAR 1. The encodings are: 0000 = 4, 00001 =8B, ..... 1_1101 = 2G"]
    #[inline(always)]
    #[must_use]
    pub fn rcbar1a(&mut self) -> Rcbar1aW<PcieLmRootComplexBarConfigurationSpec> {
        Rcbar1aW::new(self, 9)
    }
    #[doc = "Bits 14:16 - RC BAR 1 control \\[RCBAR1C\\]
Specifies the configuration of RC BAR1. The various encodings are: 000: Disabled 001: 32bit IO BAR 010-011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rcbar1c(&mut self) -> Rcbar1cW<PcieLmRootComplexBarConfigurationSpec> {
        Rcbar1cW::new(self, 14)
    }
    #[doc = "Bit 17 - Type1 cfg prefetchable mem bar enable \\[RCBARPME\\]
Enable for Prefetchable memory base and limit registers in type1 config space"]
    #[inline(always)]
    #[must_use]
    pub fn rcbarpme(&mut self) -> RcbarpmeW<PcieLmRootComplexBarConfigurationSpec> {
        RcbarpmeW::new(self, 17)
    }
    #[doc = "Bit 18 - Type1 cfg prefetchable mem bar size \\[RCBARPMS\\]
Width of Prefetchable Memory Base and Limit registers in type1 config space. 0=32 bits, 1=64bits"]
    #[inline(always)]
    #[must_use]
    pub fn rcbarpms(&mut self) -> RcbarpmsW<PcieLmRootComplexBarConfigurationSpec> {
        RcbarpmsW::new(self, 18)
    }
    #[doc = "Bit 19 - Type1 cfg IO bar enable \\[RCBARPIE\\]
Enable for IO Base and Limit registers in type1 config space"]
    #[inline(always)]
    #[must_use]
    pub fn rcbarpie(&mut self) -> RcbarpieW<PcieLmRootComplexBarConfigurationSpec> {
        RcbarpieW::new(self, 19)
    }
    #[doc = "Bit 20 - Type1 cfg IO bar size \\[RCBARPIS\\]
Width of IO Base and Limit registers in type1 config space. 0=32 bits, 1=64bits"]
    #[inline(always)]
    #[must_use]
    pub fn rcbarpis(&mut self) -> RcbarpisW<PcieLmRootComplexBarConfigurationSpec> {
        RcbarpisW::new(self, 20)
    }
    #[doc = "Bit 31 - RC BAR Check Enable \\[RCBCE\\]
This bit must be set to 1 to enable BAR checking in the RC mode. When this bit is set to 0, the core will forward all incoming memory requests to the client logic without checking their address ranges."]
    #[inline(always)]
    #[must_use]
    pub fn rcbce(&mut self) -> RcbceW<PcieLmRootComplexBarConfigurationSpec> {
        RcbceW::new(self, 31)
    }
}
#[doc = "Root Complex BAR Configuration Register This bit must be set to 1 to enable BAR checking in the RC mode. When this bit is set to 0, the core will forward all incoming memory requests to the client logic without checking their address ranges.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_root_complex_bar_configuration::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_root_complex_bar_configuration::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmRootComplexBarConfigurationSpec;
impl crate::RegisterSpec for PcieLmRootComplexBarConfigurationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_root_complex_bar_configuration::R`](R) reader structure"]
impl crate::Readable for PcieLmRootComplexBarConfigurationSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_root_complex_bar_configuration::W`](W) writer structure"]
impl crate::Writable for PcieLmRootComplexBarConfigurationSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_ROOT_COMPLEX_BAR_CONFIGURATION to value 0x0001_2994"]
impl crate::Resettable for PcieLmRootComplexBarConfigurationSpec {
    const RESET_VALUE: u32 = 0x0001_2994;
}
