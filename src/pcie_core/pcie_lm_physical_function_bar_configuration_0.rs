#[doc = "Register `PCIE_LM_PHYSICAL_FUNCTION_BAR_CONFIGURATION_0` reader"]
pub type R = crate::R<PcieLmPhysicalFunctionBarConfiguration0Spec>;
#[doc = "Register `PCIE_LM_PHYSICAL_FUNCTION_BAR_CONFIGURATION_0` writer"]
pub type W = crate::W<PcieLmPhysicalFunctionBarConfiguration0Spec>;
#[doc = "Field `BAR0A` reader - BAR 0 Aperture \\[BAR0A\\]\n\nSpecifies the aperture of the 32-bit\n\nBAR 0 or 64bit BAR0-1. For 32-bit\n\nBAR 0, the valid encodings are:\n\n00000 = 128 B, 00001 = 256 B,\n\n00010 = 512B, 00011 = 1 KB,\n\n00100 = 2 KB, 00101 = 4 KB,00110\n\n= 8 KB, 00111 = 16 KB, 01000 = 32\n\nKB,01001 = 64 KB, 01010 = 128 KB,\n\n01011 = 256KB, 01100 = 512 KB,\n\n01101 = 1 MB, 01110 =2 MB, 01111\n\n= 4 MB, 10000 = 8 MB, 10001 =16\n\nMB, 10010 = 32 MB, 10011 = 64\n\nMB, 10100= 128 MB, 10101 = 256\n\nMB, 10110 = 512 MB,10111 = 1 GB,\n\n11000 = 2 GB\n\nFor 64-bit BAR0-1, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 =256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128\n\nKB, 01011 = 256 KB, 01100 = 512\n\nKB, 01101 =1 MB, 01110 = 2 MB,\n\n01111 = 4 MB, 10000 = 8MB, 10001\n\n= 16 MB, 10010 = 32 MB, 10011 =\n\n64MB, 10100 = 128 MB, 10101 =\n\n256 MB, 10110= 512 MB, 10111 = 1\n\nGB, 11000 = 2 GB, 11001= 4 GB,\n\n11010 = 8 GB, 11011 = 16 GB,\n\n11100 =32 GB, 11101 = 64 GB,\n\n11110 = 128 GB, 11111 =256 GB"]
pub type Bar0aR = crate::FieldReader;
#[doc = "Field `BAR0A` writer - BAR 0 Aperture \\[BAR0A\\]\n\nSpecifies the aperture of the 32-bit\n\nBAR 0 or 64bit BAR0-1. For 32-bit\n\nBAR 0, the valid encodings are:\n\n00000 = 128 B, 00001 = 256 B,\n\n00010 = 512B, 00011 = 1 KB,\n\n00100 = 2 KB, 00101 = 4 KB,00110\n\n= 8 KB, 00111 = 16 KB, 01000 = 32\n\nKB,01001 = 64 KB, 01010 = 128 KB,\n\n01011 = 256KB, 01100 = 512 KB,\n\n01101 = 1 MB, 01110 =2 MB, 01111\n\n= 4 MB, 10000 = 8 MB, 10001 =16\n\nMB, 10010 = 32 MB, 10011 = 64\n\nMB, 10100= 128 MB, 10101 = 256\n\nMB, 10110 = 512 MB,10111 = 1 GB,\n\n11000 = 2 GB\n\nFor 64-bit BAR0-1, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 =256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128\n\nKB, 01011 = 256 KB, 01100 = 512\n\nKB, 01101 =1 MB, 01110 = 2 MB,\n\n01111 = 4 MB, 10000 = 8MB, 10001\n\n= 16 MB, 10010 = 32 MB, 10011 =\n\n64MB, 10100 = 128 MB, 10101 =\n\n256 MB, 10110= 512 MB, 10111 = 1\n\nGB, 11000 = 2 GB, 11001= 4 GB,\n\n11010 = 8 GB, 11011 = 16 GB,\n\n11100 =32 GB, 11101 = 64 GB,\n\n11110 = 128 GB, 11111 =256 GB"]
pub type Bar0aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "BAR 0 Control \\[BAR0C\\]\n\nSpecifies the configuration of BAR0.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\nValue on reset: 6"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bar0c {
    #[doc = "3: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable"]
    B011 = 3,
    #[doc = "7: 64bit memory BAR, prefetchable"]
    B111 = 7,
}
impl From<Bar0c> for u8 {
    #[inline(always)]
    fn from(variant: Bar0c) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bar0c {
    type Ux = u8;
}
#[doc = "Field `BAR0C` reader - BAR 0 Control \\[BAR0C\\]\n\nSpecifies the configuration of BAR0.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-"]
pub type Bar0cR = crate::FieldReader<Bar0c>;
impl Bar0cR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bar0c> {
        match self.bits {
            3 => Some(Bar0c::B011),
            7 => Some(Bar0c::B111),
            _ => None,
        }
    }
    #[doc = "Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Bar0c::B011
    }
    #[doc = "64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Bar0c::B111
    }
}
#[doc = "Field `BAR0C` writer - BAR 0 Control \\[BAR0C\\]\n\nSpecifies the configuration of BAR0.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-"]
pub type Bar0cW<'a, REG> = crate::FieldWriter<'a, REG, 3, Bar0c>;
impl<'a, REG> Bar0cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Bar0c::B011)
    }
    #[doc = "64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Bar0c::B111)
    }
}
#[doc = "Field `BAR1A` reader - BAR 1 Aperture \\[BAR1A\\]\n\nSpecifies the aperture of the BAR 1\n\nwhen it is configured as a 32-bit\n\nBAR. For 32-bit BAR 1, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 = 256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110 =512 MB, 10111 = 1 GB,\n\n11000 = 2 GB"]
pub type Bar1aR = crate::FieldReader;
#[doc = "Field `BAR1A` writer - BAR 1 Aperture \\[BAR1A\\]\n\nSpecifies the aperture of the BAR 1\n\nwhen it is configured as a 32-bit\n\nBAR. For 32-bit BAR 1, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 = 256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110 =512 MB, 10111 = 1 GB,\n\n11000 = 2 GB"]
pub type Bar1aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BAR1C` reader - BAR 1 Control \\[BAR1C\\]\n\nSpecifies the configuration of BAR1.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\n011: Reserved 100: 32bit memory\n\nBAR, non prefetchable 101: 32bit\n\nmemory BAR, prefetchable 110-111:\n\nReserved"]
pub type Bar1cR = crate::FieldReader;
#[doc = "Field `BAR1C` writer - BAR 1 Control \\[BAR1C\\]\n\nSpecifies the configuration of BAR1.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\n011: Reserved 100: 32bit memory\n\nBAR, non prefetchable 101: 32bit\n\nmemory BAR, prefetchable 110-111:\n\nReserved"]
pub type Bar1cW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BAR2A` reader - BAR 2 Aperture \\[BAR2A\\]\n\nSpecifies the aperture of the 32-bit\n\nBAR 2 or 64bit BAR2-3. For 32-bit\n\nBAR 2, the valid encodings are:\n\n00000 = 128 B, 00001 = 256 B,\n\n00010 = 512B, 00011 = 1 KB,\n\n00100 = 2 KB, 00101 = 4 KB,00110\n\n= 8 KB, 00111 = 16 KB, 01000 = 32\n\nKB,01001 = 64 KB, 01010 = 128 KB,\n\n01011 = 256KB, 01100 = 512 KB,\n\n01101 = 1 MB, 01110 =2 MB, 01111\n\n= 4 MB, 10000 = 8 MB, 10001 =16\n\nMB, 10010 = 32 MB, 10011 = 64\n\nMB, 10100= 128 MB, 10101 = 256\n\nMB, 10110 = 512 MB,10111 = 1 GB,\n\n11000 = 2 GB For 64-bit BAR2-3,\n\nthe valid encodings are: 00000 =\n\n128 B, 00001 = 256 B, 00010 = 512\n\nB, 00011 = 1 KB, 00100 =2 KB,\n\n00101 = 4 KB, 00110 = 8 KB, 00111\n\n= 16KB, 01000 = 32 KB, 01001 =\n\n64 KB, 01010 = 128\n\nKB, 01011 = 256 KB, 01100 = 512\n\nKB, 01101 =1 MB, 01110 = 2 MB,\n\n01111 = 4 MB, 10000 = 8MB, 10001\n\n= 16 MB, 10010 = 32 MB, 10011 =\n\n64MB, 10100 = 128 MB, 10101 =\n\n256 MB, 10110= 512 MB, 10111 = 1\n\nGB, 11000 = 2 GB, 11001= 4 GB,\n\n11010 = 8 GB, 11011 = 16 GB,\n\n11100 =32 GB, 11101 = 64 GB,\n\n11110 = 128 GB, 11111 =256 GB"]
pub type Bar2aR = crate::FieldReader;
#[doc = "Field `BAR2A` writer - BAR 2 Aperture \\[BAR2A\\]\n\nSpecifies the aperture of the 32-bit\n\nBAR 2 or 64bit BAR2-3. For 32-bit\n\nBAR 2, the valid encodings are:\n\n00000 = 128 B, 00001 = 256 B,\n\n00010 = 512B, 00011 = 1 KB,\n\n00100 = 2 KB, 00101 = 4 KB,00110\n\n= 8 KB, 00111 = 16 KB, 01000 = 32\n\nKB,01001 = 64 KB, 01010 = 128 KB,\n\n01011 = 256KB, 01100 = 512 KB,\n\n01101 = 1 MB, 01110 =2 MB, 01111\n\n= 4 MB, 10000 = 8 MB, 10001 =16\n\nMB, 10010 = 32 MB, 10011 = 64\n\nMB, 10100= 128 MB, 10101 = 256\n\nMB, 10110 = 512 MB,10111 = 1 GB,\n\n11000 = 2 GB For 64-bit BAR2-3,\n\nthe valid encodings are: 00000 =\n\n128 B, 00001 = 256 B, 00010 = 512\n\nB, 00011 = 1 KB, 00100 =2 KB,\n\n00101 = 4 KB, 00110 = 8 KB, 00111\n\n= 16KB, 01000 = 32 KB, 01001 =\n\n64 KB, 01010 = 128\n\nKB, 01011 = 256 KB, 01100 = 512\n\nKB, 01101 =1 MB, 01110 = 2 MB,\n\n01111 = 4 MB, 10000 = 8MB, 10001\n\n= 16 MB, 10010 = 32 MB, 10011 =\n\n64MB, 10100 = 128 MB, 10101 =\n\n256 MB, 10110= 512 MB, 10111 = 1\n\nGB, 11000 = 2 GB, 11001= 4 GB,\n\n11010 = 8 GB, 11011 = 16 GB,\n\n11100 =32 GB, 11101 = 64 GB,\n\n11110 = 128 GB, 11111 =256 GB"]
pub type Bar2aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "BAR 2 Control \\[BAR2C\\]\n\nSpecifies the configuration of BAR2.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\nValue on reset: 6"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bar2c {
    #[doc = "3: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable"]
    B011 = 3,
    #[doc = "7: 64bit memory BAR, prefetchable"]
    B111 = 7,
}
impl From<Bar2c> for u8 {
    #[inline(always)]
    fn from(variant: Bar2c) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bar2c {
    type Ux = u8;
}
#[doc = "Field `BAR2C` reader - BAR 2 Control \\[BAR2C\\]\n\nSpecifies the configuration of BAR2.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-"]
pub type Bar2cR = crate::FieldReader<Bar2c>;
impl Bar2cR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bar2c> {
        match self.bits {
            3 => Some(Bar2c::B011),
            7 => Some(Bar2c::B111),
            _ => None,
        }
    }
    #[doc = "Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Bar2c::B011
    }
    #[doc = "64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Bar2c::B111
    }
}
#[doc = "Field `BAR2C` writer - BAR 2 Control \\[BAR2C\\]\n\nSpecifies the configuration of BAR2.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-"]
pub type Bar2cW<'a, REG> = crate::FieldWriter<'a, REG, 3, Bar2c>;
impl<'a, REG> Bar2cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Bar2c::B011)
    }
    #[doc = "64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Bar2c::B111)
    }
}
#[doc = "Field `BAR3A` reader - BAR 3 Aperture \\[BAR3A\\]\n\nSpecifies the aperture of the BAR 3\n\nwhen it is configured as a 32-bit\n\nBAR. For 32-bit BAR 3, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 = 256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110 =512 MB, 10111 = 1 GB,\n\n11000 = 2 GB"]
pub type Bar3aR = crate::FieldReader;
#[doc = "Field `BAR3A` writer - BAR 3 Aperture \\[BAR3A\\]\n\nSpecifies the aperture of the BAR 3\n\nwhen it is configured as a 32-bit\n\nBAR. For 32-bit BAR 3, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 = 256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110 =512 MB, 10111 = 1 GB,\n\n11000 = 2 GB"]
pub type Bar3aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BAR3C` reader - BAR 3 Control \\[BAR3C\\]\n\nSpecifies the configuration of BAR3.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\n011: Reserved 100: 32bit memory\n\nBAR, non prefetchable 101: 32bit\n\nmemory BAR, prefetchable 110-111:\n\nReserved"]
pub type Bar3cR = crate::FieldReader;
#[doc = "Field `BAR3C` writer - BAR 3 Control \\[BAR3C\\]\n\nSpecifies the configuration of BAR3.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\n011: Reserved 100: 32bit memory\n\nBAR, non prefetchable 101: 32bit\n\nmemory BAR, prefetchable 110-111:\n\nReserved"]
pub type Bar3cW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - BAR 0 Aperture \\[BAR0A\\]\n\nSpecifies the aperture of the 32-bit\n\nBAR 0 or 64bit BAR0-1. For 32-bit\n\nBAR 0, the valid encodings are:\n\n00000 = 128 B, 00001 = 256 B,\n\n00010 = 512B, 00011 = 1 KB,\n\n00100 = 2 KB, 00101 = 4 KB,00110\n\n= 8 KB, 00111 = 16 KB, 01000 = 32\n\nKB,01001 = 64 KB, 01010 = 128 KB,\n\n01011 = 256KB, 01100 = 512 KB,\n\n01101 = 1 MB, 01110 =2 MB, 01111\n\n= 4 MB, 10000 = 8 MB, 10001 =16\n\nMB, 10010 = 32 MB, 10011 = 64\n\nMB, 10100= 128 MB, 10101 = 256\n\nMB, 10110 = 512 MB,10111 = 1 GB,\n\n11000 = 2 GB\n\nFor 64-bit BAR0-1, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 =256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128\n\nKB, 01011 = 256 KB, 01100 = 512\n\nKB, 01101 =1 MB, 01110 = 2 MB,\n\n01111 = 4 MB, 10000 = 8MB, 10001\n\n= 16 MB, 10010 = 32 MB, 10011 =\n\n64MB, 10100 = 128 MB, 10101 =\n\n256 MB, 10110= 512 MB, 10111 = 1\n\nGB, 11000 = 2 GB, 11001= 4 GB,\n\n11010 = 8 GB, 11011 = 16 GB,\n\n11100 =32 GB, 11101 = 64 GB,\n\n11110 = 128 GB, 11111 =256 GB"]
    #[inline(always)]
    pub fn bar0a(&self) -> Bar0aR {
        Bar0aR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - BAR 0 Control \\[BAR0C\\]\n\nSpecifies the configuration of BAR0.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-"]
    #[inline(always)]
    pub fn bar0c(&self) -> Bar0cR {
        Bar0cR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - BAR 1 Aperture \\[BAR1A\\]\n\nSpecifies the aperture of the BAR 1\n\nwhen it is configured as a 32-bit\n\nBAR. For 32-bit BAR 1, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 = 256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110 =512 MB, 10111 = 1 GB,\n\n11000 = 2 GB"]
    #[inline(always)]
    pub fn bar1a(&self) -> Bar1aR {
        Bar1aR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - BAR 1 Control \\[BAR1C\\]\n\nSpecifies the configuration of BAR1.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\n011: Reserved 100: 32bit memory\n\nBAR, non prefetchable 101: 32bit\n\nmemory BAR, prefetchable 110-111:\n\nReserved"]
    #[inline(always)]
    pub fn bar1c(&self) -> Bar1cR {
        Bar1cR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - BAR 2 Aperture \\[BAR2A\\]\n\nSpecifies the aperture of the 32-bit\n\nBAR 2 or 64bit BAR2-3. For 32-bit\n\nBAR 2, the valid encodings are:\n\n00000 = 128 B, 00001 = 256 B,\n\n00010 = 512B, 00011 = 1 KB,\n\n00100 = 2 KB, 00101 = 4 KB,00110\n\n= 8 KB, 00111 = 16 KB, 01000 = 32\n\nKB,01001 = 64 KB, 01010 = 128 KB,\n\n01011 = 256KB, 01100 = 512 KB,\n\n01101 = 1 MB, 01110 =2 MB, 01111\n\n= 4 MB, 10000 = 8 MB, 10001 =16\n\nMB, 10010 = 32 MB, 10011 = 64\n\nMB, 10100= 128 MB, 10101 = 256\n\nMB, 10110 = 512 MB,10111 = 1 GB,\n\n11000 = 2 GB For 64-bit BAR2-3,\n\nthe valid encodings are: 00000 =\n\n128 B, 00001 = 256 B, 00010 = 512\n\nB, 00011 = 1 KB, 00100 =2 KB,\n\n00101 = 4 KB, 00110 = 8 KB, 00111\n\n= 16KB, 01000 = 32 KB, 01001 =\n\n64 KB, 01010 = 128\n\nKB, 01011 = 256 KB, 01100 = 512\n\nKB, 01101 =1 MB, 01110 = 2 MB,\n\n01111 = 4 MB, 10000 = 8MB, 10001\n\n= 16 MB, 10010 = 32 MB, 10011 =\n\n64MB, 10100 = 128 MB, 10101 =\n\n256 MB, 10110= 512 MB, 10111 = 1\n\nGB, 11000 = 2 GB, 11001= 4 GB,\n\n11010 = 8 GB, 11011 = 16 GB,\n\n11100 =32 GB, 11101 = 64 GB,\n\n11110 = 128 GB, 11111 =256 GB"]
    #[inline(always)]
    pub fn bar2a(&self) -> Bar2aR {
        Bar2aR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - BAR 2 Control \\[BAR2C\\]\n\nSpecifies the configuration of BAR2.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-"]
    #[inline(always)]
    pub fn bar2c(&self) -> Bar2cR {
        Bar2cR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - BAR 3 Aperture \\[BAR3A\\]\n\nSpecifies the aperture of the BAR 3\n\nwhen it is configured as a 32-bit\n\nBAR. For 32-bit BAR 3, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 = 256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110 =512 MB, 10111 = 1 GB,\n\n11000 = 2 GB"]
    #[inline(always)]
    pub fn bar3a(&self) -> Bar3aR {
        Bar3aR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - BAR 3 Control \\[BAR3C\\]\n\nSpecifies the configuration of BAR3.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\n011: Reserved 100: 32bit memory\n\nBAR, non prefetchable 101: 32bit\n\nmemory BAR, prefetchable 110-111:\n\nReserved"]
    #[inline(always)]
    pub fn bar3c(&self) -> Bar3cR {
        Bar3cR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - BAR 0 Aperture \\[BAR0A\\]\n\nSpecifies the aperture of the 32-bit\n\nBAR 0 or 64bit BAR0-1. For 32-bit\n\nBAR 0, the valid encodings are:\n\n00000 = 128 B, 00001 = 256 B,\n\n00010 = 512B, 00011 = 1 KB,\n\n00100 = 2 KB, 00101 = 4 KB,00110\n\n= 8 KB, 00111 = 16 KB, 01000 = 32\n\nKB,01001 = 64 KB, 01010 = 128 KB,\n\n01011 = 256KB, 01100 = 512 KB,\n\n01101 = 1 MB, 01110 =2 MB, 01111\n\n= 4 MB, 10000 = 8 MB, 10001 =16\n\nMB, 10010 = 32 MB, 10011 = 64\n\nMB, 10100= 128 MB, 10101 = 256\n\nMB, 10110 = 512 MB,10111 = 1 GB,\n\n11000 = 2 GB\n\nFor 64-bit BAR0-1, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 =256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128\n\nKB, 01011 = 256 KB, 01100 = 512\n\nKB, 01101 =1 MB, 01110 = 2 MB,\n\n01111 = 4 MB, 10000 = 8MB, 10001\n\n= 16 MB, 10010 = 32 MB, 10011 =\n\n64MB, 10100 = 128 MB, 10101 =\n\n256 MB, 10110= 512 MB, 10111 = 1\n\nGB, 11000 = 2 GB, 11001= 4 GB,\n\n11010 = 8 GB, 11011 = 16 GB,\n\n11100 =32 GB, 11101 = 64 GB,\n\n11110 = 128 GB, 11111 =256 GB"]
    #[inline(always)]
    #[must_use]
    pub fn bar0a(&mut self) -> Bar0aW<PcieLmPhysicalFunctionBarConfiguration0Spec> {
        Bar0aW::new(self, 0)
    }
    #[doc = "Bits 5:7 - BAR 0 Control \\[BAR0C\\]\n\nSpecifies the configuration of BAR0.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-"]
    #[inline(always)]
    #[must_use]
    pub fn bar0c(&mut self) -> Bar0cW<PcieLmPhysicalFunctionBarConfiguration0Spec> {
        Bar0cW::new(self, 5)
    }
    #[doc = "Bits 8:12 - BAR 1 Aperture \\[BAR1A\\]\n\nSpecifies the aperture of the BAR 1\n\nwhen it is configured as a 32-bit\n\nBAR. For 32-bit BAR 1, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 = 256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110 =512 MB, 10111 = 1 GB,\n\n11000 = 2 GB"]
    #[inline(always)]
    #[must_use]
    pub fn bar1a(&mut self) -> Bar1aW<PcieLmPhysicalFunctionBarConfiguration0Spec> {
        Bar1aW::new(self, 8)
    }
    #[doc = "Bits 13:15 - BAR 1 Control \\[BAR1C\\]\n\nSpecifies the configuration of BAR1.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\n011: Reserved 100: 32bit memory\n\nBAR, non prefetchable 101: 32bit\n\nmemory BAR, prefetchable 110-111:\n\nReserved"]
    #[inline(always)]
    #[must_use]
    pub fn bar1c(&mut self) -> Bar1cW<PcieLmPhysicalFunctionBarConfiguration0Spec> {
        Bar1cW::new(self, 13)
    }
    #[doc = "Bits 16:20 - BAR 2 Aperture \\[BAR2A\\]\n\nSpecifies the aperture of the 32-bit\n\nBAR 2 or 64bit BAR2-3. For 32-bit\n\nBAR 2, the valid encodings are:\n\n00000 = 128 B, 00001 = 256 B,\n\n00010 = 512B, 00011 = 1 KB,\n\n00100 = 2 KB, 00101 = 4 KB,00110\n\n= 8 KB, 00111 = 16 KB, 01000 = 32\n\nKB,01001 = 64 KB, 01010 = 128 KB,\n\n01011 = 256KB, 01100 = 512 KB,\n\n01101 = 1 MB, 01110 =2 MB, 01111\n\n= 4 MB, 10000 = 8 MB, 10001 =16\n\nMB, 10010 = 32 MB, 10011 = 64\n\nMB, 10100= 128 MB, 10101 = 256\n\nMB, 10110 = 512 MB,10111 = 1 GB,\n\n11000 = 2 GB For 64-bit BAR2-3,\n\nthe valid encodings are: 00000 =\n\n128 B, 00001 = 256 B, 00010 = 512\n\nB, 00011 = 1 KB, 00100 =2 KB,\n\n00101 = 4 KB, 00110 = 8 KB, 00111\n\n= 16KB, 01000 = 32 KB, 01001 =\n\n64 KB, 01010 = 128\n\nKB, 01011 = 256 KB, 01100 = 512\n\nKB, 01101 =1 MB, 01110 = 2 MB,\n\n01111 = 4 MB, 10000 = 8MB, 10001\n\n= 16 MB, 10010 = 32 MB, 10011 =\n\n64MB, 10100 = 128 MB, 10101 =\n\n256 MB, 10110= 512 MB, 10111 = 1\n\nGB, 11000 = 2 GB, 11001= 4 GB,\n\n11010 = 8 GB, 11011 = 16 GB,\n\n11100 =32 GB, 11101 = 64 GB,\n\n11110 = 128 GB, 11111 =256 GB"]
    #[inline(always)]
    #[must_use]
    pub fn bar2a(&mut self) -> Bar2aW<PcieLmPhysicalFunctionBarConfiguration0Spec> {
        Bar2aW::new(self, 16)
    }
    #[doc = "Bits 21:23 - BAR 2 Control \\[BAR2C\\]\n\nSpecifies the configuration of BAR2.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-"]
    #[inline(always)]
    #[must_use]
    pub fn bar2c(&mut self) -> Bar2cW<PcieLmPhysicalFunctionBarConfiguration0Spec> {
        Bar2cW::new(self, 21)
    }
    #[doc = "Bits 24:28 - BAR 3 Aperture \\[BAR3A\\]\n\nSpecifies the aperture of the BAR 3\n\nwhen it is configured as a 32-bit\n\nBAR. For 32-bit BAR 3, the valid\n\nencodings are: 00000 = 128 B,\n\n00001 = 256 B, 00010 = 512 B,\n\n00011 = 1 KB, 00100 =2 KB, 00101\n\n= 4 KB, 00110 = 8 KB, 00111 =\n\n16KB, 01000 = 32 KB, 01001 = 64\n\nKB, 01010 = 128KB, 01011 = 256\n\nKB, 01100 = 512 KB, 01101 =1 MB,\n\n01110 = 2 MB, 01111 = 4 MB,\n\n10000 = 8MB, 10001 = 16 MB,\n\n10010 = 32 MB, 10011 = 64MB,\n\n10100 = 128 MB, 10101 = 256 MB,\n\n10110 =512 MB, 10111 = 1 GB,\n\n11000 = 2 GB"]
    #[inline(always)]
    #[must_use]
    pub fn bar3a(&mut self) -> Bar3aW<PcieLmPhysicalFunctionBarConfiguration0Spec> {
        Bar3aW::new(self, 24)
    }
    #[doc = "Bits 29:31 - BAR 3 Control \\[BAR3C\\]\n\nSpecifies the configuration of BAR3.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\n011: Reserved 100: 32bit memory\n\nBAR, non prefetchable 101: 32bit\n\nmemory BAR, prefetchable 110-111:\n\nReserved"]
    #[inline(always)]
    #[must_use]
    pub fn bar3c(&mut self) -> Bar3cW<PcieLmPhysicalFunctionBarConfiguration0Spec> {
        Bar3cW::new(self, 29)
    }
}
#[doc = "Physical Function BAR Configuration Register 0\n\nSpecifies the configuration of BAR3.\n\nThe various encodings are: 000:\n\nDisabled 001: 32bit IO BAR 010-\n\n011: Reserved 100: 32bit memory\n\nBAR, non prefetchable 101: 32bit\n\nmemory BAR, prefetchable 110-111:\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_physical_function_bar_configuration_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_physical_function_bar_configuration_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmPhysicalFunctionBarConfiguration0Spec;
impl crate::RegisterSpec for PcieLmPhysicalFunctionBarConfiguration0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_physical_function_bar_configuration_0::R`](R) reader structure"]
impl crate::Readable for PcieLmPhysicalFunctionBarConfiguration0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_physical_function_bar_configuration_0::W`](W) writer structure"]
impl crate::Writable for PcieLmPhysicalFunctionBarConfiguration0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_PHYSICAL_FUNCTION_BAR_CONFIGURATION_0 to value 0x8fcf_8fcf"]
impl crate::Resettable for PcieLmPhysicalFunctionBarConfiguration0Spec {
    const RESET_VALUE: u32 = 0x8fcf_8fcf;
}
