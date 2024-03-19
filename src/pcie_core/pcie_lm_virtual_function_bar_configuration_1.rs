#[doc = "Register `PCIE_LM_VIRTUAL_FUNCTION_BAR_CONFIGURATION_1` reader"]
pub type R = crate::R<PcieLmVirtualFunctionBarConfiguration1Spec>;
#[doc = "Register `PCIE_LM_VIRTUAL_FUNCTION_BAR_CONFIGURATION_1` writer"]
pub type W = crate::W<PcieLmVirtualFunctionBarConfiguration1Spec>;
#[doc = "Field `VFBAR4A` reader - VF BAR 4 Aperture \\[VFBAR4A\\]\n\nSpecifies the aperture of the 32-bit\n\nVF BAR 4 or 64bit VF BAR4-5. The\n\nencodings are: 00000 = 128 Bytes,\n\n0001 = 256 Bytes, 0010 = 512\n\nBytes, 0011 = 1 Kbytes, 00100 = 2\n\nKbytes, 00101 = 4Kbytes, 00110 =\n\n8 Kbytes, 00111 = 16 Kbytes,\n\n01000 = 32 Kbytes, 01001 = 64\n\nKbytes, 01010= 128 Kbytes, 01011\n\n= 256 Kbytes, 01100 = 512 Kbytes,\n\n01101 = 1 Mbyte, 01110 = 2\n\nMbytes, 01111 = 4 Mbytes, 10000 =\n\n8 Mbytes, 10001 = 16 Mbytes,\n\n10010 = 32 Mbytes, 10011 = 64\n\nMbytes,10100 = 128 Mbytes, 10101\n\n= 256 Mbytes,10110 = 512 Mbytes,\n\n10111 = 1 Gbyte, 11000 = 2\n\nGbytes, 11001 = 4 Gbytes, 11010 =\n\n8 Gbytes, 11011 = 16 Gbytes,\n\n11100 = 32 Gbytes, 11101 = 64\n\nGbytes, 11110 = 128 Gbytes, 11111\n\n= 256 Gbytes"]
pub type Vfbar4aR = crate::FieldReader;
#[doc = "Field `VFBAR4A` writer - VF BAR 4 Aperture \\[VFBAR4A\\]\n\nSpecifies the aperture of the 32-bit\n\nVF BAR 4 or 64bit VF BAR4-5. The\n\nencodings are: 00000 = 128 Bytes,\n\n0001 = 256 Bytes, 0010 = 512\n\nBytes, 0011 = 1 Kbytes, 00100 = 2\n\nKbytes, 00101 = 4Kbytes, 00110 =\n\n8 Kbytes, 00111 = 16 Kbytes,\n\n01000 = 32 Kbytes, 01001 = 64\n\nKbytes, 01010= 128 Kbytes, 01011\n\n= 256 Kbytes, 01100 = 512 Kbytes,\n\n01101 = 1 Mbyte, 01110 = 2\n\nMbytes, 01111 = 4 Mbytes, 10000 =\n\n8 Mbytes, 10001 = 16 Mbytes,\n\n10010 = 32 Mbytes, 10011 = 64\n\nMbytes,10100 = 128 Mbytes, 10101\n\n= 256 Mbytes,10110 = 512 Mbytes,\n\n10111 = 1 Gbyte, 11000 = 2\n\nGbytes, 11001 = 4 Gbytes, 11010 =\n\n8 Gbytes, 11011 = 16 Gbytes,\n\n11100 = 32 Gbytes, 11101 = 64\n\nGbytes, 11110 = 128 Gbytes, 11111\n\n= 256 Gbytes"]
pub type Vfbar4aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "VF BAR 4 Control \\[VFBAR4C\\]\n\nSpecifies the configuration of VF\n\nBAR4. The various encodings are:\n\nValue on reset: 6"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vfbar4c {
    #[doc = "0: Disabled 001-011: Reserved"]
    B000 = 0,
    #[doc = "4: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    B100 = 4,
}
impl From<Vfbar4c> for u8 {
    #[inline(always)]
    fn from(variant: Vfbar4c) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vfbar4c {
    type Ux = u8;
}
#[doc = "Field `VFBAR4C` reader - VF BAR 4 Control \\[VFBAR4C\\]\n\nSpecifies the configuration of VF\n\nBAR4. The various encodings are:"]
pub type Vfbar4cR = crate::FieldReader<Vfbar4c>;
impl Vfbar4cR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vfbar4c> {
        match self.bits {
            0 => Some(Vfbar4c::B000),
            4 => Some(Vfbar4c::B100),
            _ => None,
        }
    }
    #[doc = "Disabled 001-011: Reserved"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Vfbar4c::B000
    }
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Vfbar4c::B100
    }
}
#[doc = "Field `VFBAR4C` writer - VF BAR 4 Control \\[VFBAR4C\\]\n\nSpecifies the configuration of VF\n\nBAR4. The various encodings are:"]
pub type Vfbar4cW<'a, REG> = crate::FieldWriter<'a, REG, 3, Vfbar4c>;
impl<'a, REG> Vfbar4cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled 001-011: Reserved"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Vfbar4c::B000)
    }
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Vfbar4c::B100)
    }
}
#[doc = "Field `VFBAR5A` reader - VF BAR 5 Aperture \\[VFBAR5A\\]\n\nSpecifies the aperture of the VF BAR\n\n5 when it is configured as a 32-bit\n\nBAR. The encodings are: 00000 =\n\n128 Bytes, 0001 = 256 Bytes, 0010\n\n= 512 Bytes, 0011 = 1 Kbytes,\n\n00100 = 2 Kbytes, 00101 = 4\n\nKbytes, 00110 = 8 Kbytes, 00111 =\n\n16 Kbytes, 01000 = 32 Kbytes,\n\n01001 = 64 Kbytes, 01010 = 128\n\nKbytes, 01011 = 256 Kbytes, 01100\n\n= 512 Kbytes, 01101 = 1 Mbyte,\n\n01110 = 2 Mbytes, 01111 = 4\n\nMbytes, 10000 = 8 Mbytes, 10001 =\n\n16 Mbytes, 10010 = 32 Mbytes,\n\n10011 = 64 Mbytes, 10100 = 128\n\nMbytes, 10101 = 256 Mbytes,\n\n10110 = 512 Mbytes, 10111 = 1\n\nGbyte, 11000 = 2 Gbytes"]
pub type Vfbar5aR = crate::FieldReader;
#[doc = "Field `VFBAR5A` writer - VF BAR 5 Aperture \\[VFBAR5A\\]\n\nSpecifies the aperture of the VF BAR\n\n5 when it is configured as a 32-bit\n\nBAR. The encodings are: 00000 =\n\n128 Bytes, 0001 = 256 Bytes, 0010\n\n= 512 Bytes, 0011 = 1 Kbytes,\n\n00100 = 2 Kbytes, 00101 = 4\n\nKbytes, 00110 = 8 Kbytes, 00111 =\n\n16 Kbytes, 01000 = 32 Kbytes,\n\n01001 = 64 Kbytes, 01010 = 128\n\nKbytes, 01011 = 256 Kbytes, 01100\n\n= 512 Kbytes, 01101 = 1 Mbyte,\n\n01110 = 2 Mbytes, 01111 = 4\n\nMbytes, 10000 = 8 Mbytes, 10001 =\n\n16 Mbytes, 10010 = 32 Mbytes,\n\n10011 = 64 Mbytes, 10100 = 128\n\nMbytes, 10101 = 256 Mbytes,\n\n10110 = 512 Mbytes, 10111 = 1\n\nGbyte, 11000 = 2 Gbytes"]
pub type Vfbar5aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "VF BAR 5 Control \\[VFBAR5C\\]\n\nSpecifies the configuration of VF\n\nBAR5. The various encodings are:\n\nValue on reset: 4"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vfbar5c {
    #[doc = "0: Disabled 001-011: Reserved"]
    B000 = 0,
    #[doc = "4: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    B100 = 4,
}
impl From<Vfbar5c> for u8 {
    #[inline(always)]
    fn from(variant: Vfbar5c) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vfbar5c {
    type Ux = u8;
}
#[doc = "Field `VFBAR5C` reader - VF BAR 5 Control \\[VFBAR5C\\]\n\nSpecifies the configuration of VF\n\nBAR5. The various encodings are:"]
pub type Vfbar5cR = crate::FieldReader<Vfbar5c>;
impl Vfbar5cR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vfbar5c> {
        match self.bits {
            0 => Some(Vfbar5c::B000),
            4 => Some(Vfbar5c::B100),
            _ => None,
        }
    }
    #[doc = "Disabled 001-011: Reserved"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Vfbar5c::B000
    }
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Vfbar5c::B100
    }
}
#[doc = "Field `VFBAR5C` writer - VF BAR 5 Control \\[VFBAR5C\\]\n\nSpecifies the configuration of VF\n\nBAR5. The various encodings are:"]
pub type Vfbar5cW<'a, REG> = crate::FieldWriter<'a, REG, 3, Vfbar5c>;
impl<'a, REG> Vfbar5cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled 001-011: Reserved"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Vfbar5c::B000)
    }
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Vfbar5c::B100)
    }
}
#[doc = "Field `R16` reader - Reserved \\[R16\\]\n\nReserved"]
pub type R16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:4 - VF BAR 4 Aperture \\[VFBAR4A\\]\n\nSpecifies the aperture of the 32-bit\n\nVF BAR 4 or 64bit VF BAR4-5. The\n\nencodings are: 00000 = 128 Bytes,\n\n0001 = 256 Bytes, 0010 = 512\n\nBytes, 0011 = 1 Kbytes, 00100 = 2\n\nKbytes, 00101 = 4Kbytes, 00110 =\n\n8 Kbytes, 00111 = 16 Kbytes,\n\n01000 = 32 Kbytes, 01001 = 64\n\nKbytes, 01010= 128 Kbytes, 01011\n\n= 256 Kbytes, 01100 = 512 Kbytes,\n\n01101 = 1 Mbyte, 01110 = 2\n\nMbytes, 01111 = 4 Mbytes, 10000 =\n\n8 Mbytes, 10001 = 16 Mbytes,\n\n10010 = 32 Mbytes, 10011 = 64\n\nMbytes,10100 = 128 Mbytes, 10101\n\n= 256 Mbytes,10110 = 512 Mbytes,\n\n10111 = 1 Gbyte, 11000 = 2\n\nGbytes, 11001 = 4 Gbytes, 11010 =\n\n8 Gbytes, 11011 = 16 Gbytes,\n\n11100 = 32 Gbytes, 11101 = 64\n\nGbytes, 11110 = 128 Gbytes, 11111\n\n= 256 Gbytes"]
    #[inline(always)]
    pub fn vfbar4a(&self) -> Vfbar4aR {
        Vfbar4aR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - VF BAR 4 Control \\[VFBAR4C\\]\n\nSpecifies the configuration of VF\n\nBAR4. The various encodings are:"]
    #[inline(always)]
    pub fn vfbar4c(&self) -> Vfbar4cR {
        Vfbar4cR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - VF BAR 5 Aperture \\[VFBAR5A\\]\n\nSpecifies the aperture of the VF BAR\n\n5 when it is configured as a 32-bit\n\nBAR. The encodings are: 00000 =\n\n128 Bytes, 0001 = 256 Bytes, 0010\n\n= 512 Bytes, 0011 = 1 Kbytes,\n\n00100 = 2 Kbytes, 00101 = 4\n\nKbytes, 00110 = 8 Kbytes, 00111 =\n\n16 Kbytes, 01000 = 32 Kbytes,\n\n01001 = 64 Kbytes, 01010 = 128\n\nKbytes, 01011 = 256 Kbytes, 01100\n\n= 512 Kbytes, 01101 = 1 Mbyte,\n\n01110 = 2 Mbytes, 01111 = 4\n\nMbytes, 10000 = 8 Mbytes, 10001 =\n\n16 Mbytes, 10010 = 32 Mbytes,\n\n10011 = 64 Mbytes, 10100 = 128\n\nMbytes, 10101 = 256 Mbytes,\n\n10110 = 512 Mbytes, 10111 = 1\n\nGbyte, 11000 = 2 Gbytes"]
    #[inline(always)]
    pub fn vfbar5a(&self) -> Vfbar5aR {
        Vfbar5aR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - VF BAR 5 Control \\[VFBAR5C\\]\n\nSpecifies the configuration of VF\n\nBAR5. The various encodings are:"]
    #[inline(always)]
    pub fn vfbar5c(&self) -> Vfbar5cR {
        Vfbar5cR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:31 - Reserved \\[R16\\]\n\nReserved"]
    #[inline(always)]
    pub fn r16(&self) -> R16R {
        R16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - VF BAR 4 Aperture \\[VFBAR4A\\]\n\nSpecifies the aperture of the 32-bit\n\nVF BAR 4 or 64bit VF BAR4-5. The\n\nencodings are: 00000 = 128 Bytes,\n\n0001 = 256 Bytes, 0010 = 512\n\nBytes, 0011 = 1 Kbytes, 00100 = 2\n\nKbytes, 00101 = 4Kbytes, 00110 =\n\n8 Kbytes, 00111 = 16 Kbytes,\n\n01000 = 32 Kbytes, 01001 = 64\n\nKbytes, 01010= 128 Kbytes, 01011\n\n= 256 Kbytes, 01100 = 512 Kbytes,\n\n01101 = 1 Mbyte, 01110 = 2\n\nMbytes, 01111 = 4 Mbytes, 10000 =\n\n8 Mbytes, 10001 = 16 Mbytes,\n\n10010 = 32 Mbytes, 10011 = 64\n\nMbytes,10100 = 128 Mbytes, 10101\n\n= 256 Mbytes,10110 = 512 Mbytes,\n\n10111 = 1 Gbyte, 11000 = 2\n\nGbytes, 11001 = 4 Gbytes, 11010 =\n\n8 Gbytes, 11011 = 16 Gbytes,\n\n11100 = 32 Gbytes, 11101 = 64\n\nGbytes, 11110 = 128 Gbytes, 11111\n\n= 256 Gbytes"]
    #[inline(always)]
    #[must_use]
    pub fn vfbar4a(&mut self) -> Vfbar4aW<PcieLmVirtualFunctionBarConfiguration1Spec> {
        Vfbar4aW::new(self, 0)
    }
    #[doc = "Bits 5:7 - VF BAR 4 Control \\[VFBAR4C\\]\n\nSpecifies the configuration of VF\n\nBAR4. The various encodings are:"]
    #[inline(always)]
    #[must_use]
    pub fn vfbar4c(&mut self) -> Vfbar4cW<PcieLmVirtualFunctionBarConfiguration1Spec> {
        Vfbar4cW::new(self, 5)
    }
    #[doc = "Bits 8:12 - VF BAR 5 Aperture \\[VFBAR5A\\]\n\nSpecifies the aperture of the VF BAR\n\n5 when it is configured as a 32-bit\n\nBAR. The encodings are: 00000 =\n\n128 Bytes, 0001 = 256 Bytes, 0010\n\n= 512 Bytes, 0011 = 1 Kbytes,\n\n00100 = 2 Kbytes, 00101 = 4\n\nKbytes, 00110 = 8 Kbytes, 00111 =\n\n16 Kbytes, 01000 = 32 Kbytes,\n\n01001 = 64 Kbytes, 01010 = 128\n\nKbytes, 01011 = 256 Kbytes, 01100\n\n= 512 Kbytes, 01101 = 1 Mbyte,\n\n01110 = 2 Mbytes, 01111 = 4\n\nMbytes, 10000 = 8 Mbytes, 10001 =\n\n16 Mbytes, 10010 = 32 Mbytes,\n\n10011 = 64 Mbytes, 10100 = 128\n\nMbytes, 10101 = 256 Mbytes,\n\n10110 = 512 Mbytes, 10111 = 1\n\nGbyte, 11000 = 2 Gbytes"]
    #[inline(always)]
    #[must_use]
    pub fn vfbar5a(&mut self) -> Vfbar5aW<PcieLmVirtualFunctionBarConfiguration1Spec> {
        Vfbar5aW::new(self, 8)
    }
    #[doc = "Bits 13:15 - VF BAR 5 Control \\[VFBAR5C\\]\n\nSpecifies the configuration of VF\n\nBAR5. The various encodings are:"]
    #[inline(always)]
    #[must_use]
    pub fn vfbar5c(&mut self) -> Vfbar5cW<PcieLmVirtualFunctionBarConfiguration1Spec> {
        Vfbar5cW::new(self, 13)
    }
}
#[doc = "Virtual Function BAR Configuration Register 1\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_virtual_function_bar_configuration_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_virtual_function_bar_configuration_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmVirtualFunctionBarConfiguration1Spec;
impl crate::RegisterSpec for PcieLmVirtualFunctionBarConfiguration1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_virtual_function_bar_configuration_1::R`](R) reader structure"]
impl crate::Readable for PcieLmVirtualFunctionBarConfiguration1Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_virtual_function_bar_configuration_1::W`](W) writer structure"]
impl crate::Writable for PcieLmVirtualFunctionBarConfiguration1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_VIRTUAL_FUNCTION_BAR_CONFIGURATION_1 to value 0x8fcf"]
impl crate::Resettable for PcieLmVirtualFunctionBarConfiguration1Spec {
    const RESET_VALUE: u32 = 0x8fcf;
}
