#[doc = "Register `PHYSICAL_FUNCTION_BAR_CONFIGURATION_0` reader"]
pub type R = crate::R<PhysicalFunctionBarConfiguration0Spec>;
#[doc = "Register `PHYSICAL_FUNCTION_BAR_CONFIGURATION_0` writer"]
pub type W = crate::W<PhysicalFunctionBarConfiguration0Spec>;
#[doc = "Field `BAR0A` reader - BAR 0 Aperture \\[BAR0A\\]
Specifies the aperture of the 32-bit BAR 0 or 64bit BAR0-1. For 32-bit BAR 0, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512B, 00011 = 1 KB, 00100 = 2 KB, 00101 = 4 KB,00110 = 8 KB, 00111 = 16 KB, 01000 = 32 KB,01001 = 64 KB, 01010 = 128 KB, 01011 = 256KB, 01100 = 512 KB, 01101 = 1 MB, 01110 =2 MB, 01111 = 4 MB, 10000 = 8 MB, 10001 =16 MB, 10010 = 32 MB, 10011 = 64 MB, 10100= 128 MB, 10101 = 256 MB, 10110 = 512 MB,10111 = 1 GB, 11000 = 2 GB For 64-bit BAR0-1, the valid encodings are: 00000 = 128 B, 00001 =256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128 KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110= 512 MB, 10111 = 1 GB, 11000 = 2 GB, 11001= 4 GB, 11010 = 8 GB, 11011 = 16 GB, 11100 =32 GB, 11101 = 64 GB, 11110 = 128 GB, 11111 =256 GB"]
pub type Bar0aR = crate::FieldReader;
#[doc = "Field `BAR0A` writer - BAR 0 Aperture \\[BAR0A\\]
Specifies the aperture of the 32-bit BAR 0 or 64bit BAR0-1. For 32-bit BAR 0, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512B, 00011 = 1 KB, 00100 = 2 KB, 00101 = 4 KB,00110 = 8 KB, 00111 = 16 KB, 01000 = 32 KB,01001 = 64 KB, 01010 = 128 KB, 01011 = 256KB, 01100 = 512 KB, 01101 = 1 MB, 01110 =2 MB, 01111 = 4 MB, 10000 = 8 MB, 10001 =16 MB, 10010 = 32 MB, 10011 = 64 MB, 10100= 128 MB, 10101 = 256 MB, 10110 = 512 MB,10111 = 1 GB, 11000 = 2 GB For 64-bit BAR0-1, the valid encodings are: 00000 = 128 B, 00001 =256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128 KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110= 512 MB, 10111 = 1 GB, 11000 = 2 GB, 11001= 4 GB, 11010 = 8 GB, 11011 = 16 GB, 11100 =32 GB, 11101 = 64 GB, 11110 = 128 GB, 11111 =256 GB"]
pub type Bar0aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "BAR 0 Control \\[BAR0C\\]
Specifies the configuration of BAR0. The various encodings are: 000: Disabled 001: 32bit IO BAR 010-\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bar0c {
    #[doc = "3: 64bit memory BAR, prefetchable"]
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
#[doc = "Field `BAR0C` reader - BAR 0 Control \\[BAR0C\\]
Specifies the configuration of BAR0. The various encodings are: 000: Disabled 001: 32bit IO BAR 010-"]
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
    #[doc = "64bit memory BAR, prefetchable"]
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
#[doc = "Field `BAR0C` writer - BAR 0 Control \\[BAR0C\\]
Specifies the configuration of BAR0. The various encodings are: 000: Disabled 001: 32bit IO BAR 010-"]
pub type Bar0cW<'a, REG> = crate::FieldWriter<'a, REG, 3, Bar0c>;
impl<'a, REG> Bar0cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "64bit memory BAR, prefetchable"]
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
#[doc = "Field `BAR1A` reader - BAR 1 Aperture \\[BAR1A\\]
Specifies the aperture of the BAR 1 when it is configured as a 32-bit BAR. For 32-bit BAR 1, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110 =512 MB, 10111 = 1 GB, 11000 = 2 GB"]
pub type Bar1aR = crate::FieldReader;
#[doc = "Field `BAR1A` writer - BAR 1 Aperture \\[BAR1A\\]
Specifies the aperture of the BAR 1 when it is configured as a 32-bit BAR. For 32-bit BAR 1, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110 =512 MB, 10111 = 1 GB, 11000 = 2 GB"]
pub type Bar1aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BAR1C` reader - BAR 1 Control \\[BAR1C\\]
Specifies the configuration of BAR1. The various encodings are: 000: Disabled 001: 32bit IO BAR 010- 011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
pub type Bar1cR = crate::FieldReader;
#[doc = "Field `BAR1C` writer - BAR 1 Control \\[BAR1C\\]
Specifies the configuration of BAR1. The various encodings are: 000: Disabled 001: 32bit IO BAR 010- 011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
pub type Bar1cW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BAR2A` reader - BAR 2 Aperture \\[BAR2A\\]
Specifies the aperture of the 32-bit BAR 2 or 64bit BAR2-3. For 32-bit BAR 2, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512B, 00011 = 1 KB, 00100 = 2 KB, 00101 = 4 KB,00110 = 8 KB, 00111 = 16 KB, 01000 = 32 KB,01001 = 64 KB, 01010 = 128 KB, 01011 = 256KB, 01100 = 512 KB, 01101 = 1 MB, 01110 =2 MB, 01111 = 4 MB, 10000 = 8 MB, 10001 =16 MB, 10010 = 32 MB, 10011 = 64 MB, 10100= 128 MB, 10101 = 256 MB, 10110 = 512 MB,10111 = 1 GB, 11000 = 2 GB For 64-bit BAR2-3, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128 KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110= 512 MB, 10111 = 1 GB, 11000 = 2 GB, 11001= 4 GB, 11010 = 8 GB, 11011 = 16 GB, 11100 =32 GB, 11101 = 64 GB, 11110 = 128 GB, 11111 =256 GB"]
pub type Bar2aR = crate::FieldReader;
#[doc = "Field `BAR2A` writer - BAR 2 Aperture \\[BAR2A\\]
Specifies the aperture of the 32-bit BAR 2 or 64bit BAR2-3. For 32-bit BAR 2, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512B, 00011 = 1 KB, 00100 = 2 KB, 00101 = 4 KB,00110 = 8 KB, 00111 = 16 KB, 01000 = 32 KB,01001 = 64 KB, 01010 = 128 KB, 01011 = 256KB, 01100 = 512 KB, 01101 = 1 MB, 01110 =2 MB, 01111 = 4 MB, 10000 = 8 MB, 10001 =16 MB, 10010 = 32 MB, 10011 = 64 MB, 10100= 128 MB, 10101 = 256 MB, 10110 = 512 MB,10111 = 1 GB, 11000 = 2 GB For 64-bit BAR2-3, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128 KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110= 512 MB, 10111 = 1 GB, 11000 = 2 GB, 11001= 4 GB, 11010 = 8 GB, 11011 = 16 GB, 11100 =32 GB, 11101 = 64 GB, 11110 = 128 GB, 11111 =256 GB"]
pub type Bar2aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "BAR 2 Control \\[BAR2C\\]
Specifies the configuration of BAR2. The various encodings are: 000: Disabled 001: 32bit IO BAR 010-\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bar2c {
    #[doc = "3: 64bit memory BAR, prefetchable"]
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
#[doc = "Field `BAR2C` reader - BAR 2 Control \\[BAR2C\\]
Specifies the configuration of BAR2. The various encodings are: 000: Disabled 001: 32bit IO BAR 010-"]
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
    #[doc = "64bit memory BAR, prefetchable"]
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
#[doc = "Field `BAR2C` writer - BAR 2 Control \\[BAR2C\\]
Specifies the configuration of BAR2. The various encodings are: 000: Disabled 001: 32bit IO BAR 010-"]
pub type Bar2cW<'a, REG> = crate::FieldWriter<'a, REG, 3, Bar2c>;
impl<'a, REG> Bar2cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "64bit memory BAR, prefetchable"]
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
#[doc = "Field `BAR3A` reader - BAR 3 Aperture \\[BAR3A\\]
Specifies the aperture of the BAR 3 when it is configured as a 32-bit BAR. For 32-bit BAR 3, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110 =512 MB, 10111 = 1 GB, 11000 = 2 GB"]
pub type Bar3aR = crate::FieldReader;
#[doc = "Field `BAR3A` writer - BAR 3 Aperture \\[BAR3A\\]
Specifies the aperture of the BAR 3 when it is configured as a 32-bit BAR. For 32-bit BAR 3, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110 =512 MB, 10111 = 1 GB, 11000 = 2 GB"]
pub type Bar3aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BAR3C` reader - BAR 3 Control \\[BAR3C\\]
Specifies the configuration of BAR3. The various encodings are: 000: Disabled 001: 32bit IO BAR 010- 011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
pub type Bar3cR = crate::FieldReader;
#[doc = "Field `BAR3C` writer - BAR 3 Control \\[BAR3C\\]
Specifies the configuration of BAR3. The various encodings are: 000: Disabled 001: 32bit IO BAR 010- 011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
pub type Bar3cW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - BAR 0 Aperture \\[BAR0A\\]
Specifies the aperture of the 32-bit BAR 0 or 64bit BAR0-1. For 32-bit BAR 0, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512B, 00011 = 1 KB, 00100 = 2 KB, 00101 = 4 KB,00110 = 8 KB, 00111 = 16 KB, 01000 = 32 KB,01001 = 64 KB, 01010 = 128 KB, 01011 = 256KB, 01100 = 512 KB, 01101 = 1 MB, 01110 =2 MB, 01111 = 4 MB, 10000 = 8 MB, 10001 =16 MB, 10010 = 32 MB, 10011 = 64 MB, 10100= 128 MB, 10101 = 256 MB, 10110 = 512 MB,10111 = 1 GB, 11000 = 2 GB For 64-bit BAR0-1, the valid encodings are: 00000 = 128 B, 00001 =256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128 KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110= 512 MB, 10111 = 1 GB, 11000 = 2 GB, 11001= 4 GB, 11010 = 8 GB, 11011 = 16 GB, 11100 =32 GB, 11101 = 64 GB, 11110 = 128 GB, 11111 =256 GB"]
    #[inline(always)]
    pub fn bar0a(&self) -> Bar0aR {
        Bar0aR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - BAR 0 Control \\[BAR0C\\]
Specifies the configuration of BAR0. The various encodings are: 000: Disabled 001: 32bit IO BAR 010-"]
    #[inline(always)]
    pub fn bar0c(&self) -> Bar0cR {
        Bar0cR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - BAR 1 Aperture \\[BAR1A\\]
Specifies the aperture of the BAR 1 when it is configured as a 32-bit BAR. For 32-bit BAR 1, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110 =512 MB, 10111 = 1 GB, 11000 = 2 GB"]
    #[inline(always)]
    pub fn bar1a(&self) -> Bar1aR {
        Bar1aR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - BAR 1 Control \\[BAR1C\\]
Specifies the configuration of BAR1. The various encodings are: 000: Disabled 001: 32bit IO BAR 010- 011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub fn bar1c(&self) -> Bar1cR {
        Bar1cR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - BAR 2 Aperture \\[BAR2A\\]
Specifies the aperture of the 32-bit BAR 2 or 64bit BAR2-3. For 32-bit BAR 2, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512B, 00011 = 1 KB, 00100 = 2 KB, 00101 = 4 KB,00110 = 8 KB, 00111 = 16 KB, 01000 = 32 KB,01001 = 64 KB, 01010 = 128 KB, 01011 = 256KB, 01100 = 512 KB, 01101 = 1 MB, 01110 =2 MB, 01111 = 4 MB, 10000 = 8 MB, 10001 =16 MB, 10010 = 32 MB, 10011 = 64 MB, 10100= 128 MB, 10101 = 256 MB, 10110 = 512 MB,10111 = 1 GB, 11000 = 2 GB For 64-bit BAR2-3, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128 KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110= 512 MB, 10111 = 1 GB, 11000 = 2 GB, 11001= 4 GB, 11010 = 8 GB, 11011 = 16 GB, 11100 =32 GB, 11101 = 64 GB, 11110 = 128 GB, 11111 =256 GB"]
    #[inline(always)]
    pub fn bar2a(&self) -> Bar2aR {
        Bar2aR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - BAR 2 Control \\[BAR2C\\]
Specifies the configuration of BAR2. The various encodings are: 000: Disabled 001: 32bit IO BAR 010-"]
    #[inline(always)]
    pub fn bar2c(&self) -> Bar2cR {
        Bar2cR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - BAR 3 Aperture \\[BAR3A\\]
Specifies the aperture of the BAR 3 when it is configured as a 32-bit BAR. For 32-bit BAR 3, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110 =512 MB, 10111 = 1 GB, 11000 = 2 GB"]
    #[inline(always)]
    pub fn bar3a(&self) -> Bar3aR {
        Bar3aR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - BAR 3 Control \\[BAR3C\\]
Specifies the configuration of BAR3. The various encodings are: 000: Disabled 001: 32bit IO BAR 010- 011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub fn bar3c(&self) -> Bar3cR {
        Bar3cR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - BAR 0 Aperture \\[BAR0A\\]
Specifies the aperture of the 32-bit BAR 0 or 64bit BAR0-1. For 32-bit BAR 0, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512B, 00011 = 1 KB, 00100 = 2 KB, 00101 = 4 KB,00110 = 8 KB, 00111 = 16 KB, 01000 = 32 KB,01001 = 64 KB, 01010 = 128 KB, 01011 = 256KB, 01100 = 512 KB, 01101 = 1 MB, 01110 =2 MB, 01111 = 4 MB, 10000 = 8 MB, 10001 =16 MB, 10010 = 32 MB, 10011 = 64 MB, 10100= 128 MB, 10101 = 256 MB, 10110 = 512 MB,10111 = 1 GB, 11000 = 2 GB For 64-bit BAR0-1, the valid encodings are: 00000 = 128 B, 00001 =256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128 KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110= 512 MB, 10111 = 1 GB, 11000 = 2 GB, 11001= 4 GB, 11010 = 8 GB, 11011 = 16 GB, 11100 =32 GB, 11101 = 64 GB, 11110 = 128 GB, 11111 =256 GB"]
    #[inline(always)]
    #[must_use]
    pub fn bar0a(&mut self) -> Bar0aW<PhysicalFunctionBarConfiguration0Spec> {
        Bar0aW::new(self, 0)
    }
    #[doc = "Bits 5:7 - BAR 0 Control \\[BAR0C\\]
Specifies the configuration of BAR0. The various encodings are: 000: Disabled 001: 32bit IO BAR 010-"]
    #[inline(always)]
    #[must_use]
    pub fn bar0c(&mut self) -> Bar0cW<PhysicalFunctionBarConfiguration0Spec> {
        Bar0cW::new(self, 5)
    }
    #[doc = "Bits 8:12 - BAR 1 Aperture \\[BAR1A\\]
Specifies the aperture of the BAR 1 when it is configured as a 32-bit BAR. For 32-bit BAR 1, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110 =512 MB, 10111 = 1 GB, 11000 = 2 GB"]
    #[inline(always)]
    #[must_use]
    pub fn bar1a(&mut self) -> Bar1aW<PhysicalFunctionBarConfiguration0Spec> {
        Bar1aW::new(self, 8)
    }
    #[doc = "Bits 13:15 - BAR 1 Control \\[BAR1C\\]
Specifies the configuration of BAR1. The various encodings are: 000: Disabled 001: 32bit IO BAR 010- 011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn bar1c(&mut self) -> Bar1cW<PhysicalFunctionBarConfiguration0Spec> {
        Bar1cW::new(self, 13)
    }
    #[doc = "Bits 16:20 - BAR 2 Aperture \\[BAR2A\\]
Specifies the aperture of the 32-bit BAR 2 or 64bit BAR2-3. For 32-bit BAR 2, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512B, 00011 = 1 KB, 00100 = 2 KB, 00101 = 4 KB,00110 = 8 KB, 00111 = 16 KB, 01000 = 32 KB,01001 = 64 KB, 01010 = 128 KB, 01011 = 256KB, 01100 = 512 KB, 01101 = 1 MB, 01110 =2 MB, 01111 = 4 MB, 10000 = 8 MB, 10001 =16 MB, 10010 = 32 MB, 10011 = 64 MB, 10100= 128 MB, 10101 = 256 MB, 10110 = 512 MB,10111 = 1 GB, 11000 = 2 GB For 64-bit BAR2-3, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128 KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110= 512 MB, 10111 = 1 GB, 11000 = 2 GB, 11001= 4 GB, 11010 = 8 GB, 11011 = 16 GB, 11100 =32 GB, 11101 = 64 GB, 11110 = 128 GB, 11111 =256 GB"]
    #[inline(always)]
    #[must_use]
    pub fn bar2a(&mut self) -> Bar2aW<PhysicalFunctionBarConfiguration0Spec> {
        Bar2aW::new(self, 16)
    }
    #[doc = "Bits 21:23 - BAR 2 Control \\[BAR2C\\]
Specifies the configuration of BAR2. The various encodings are: 000: Disabled 001: 32bit IO BAR 010-"]
    #[inline(always)]
    #[must_use]
    pub fn bar2c(&mut self) -> Bar2cW<PhysicalFunctionBarConfiguration0Spec> {
        Bar2cW::new(self, 21)
    }
    #[doc = "Bits 24:28 - BAR 3 Aperture \\[BAR3A\\]
Specifies the aperture of the BAR 3 when it is configured as a 32-bit BAR. For 32-bit BAR 3, the valid encodings are: 00000 = 128 B, 00001 = 256 B, 00010 = 512 B, 00011 = 1 KB, 00100 =2 KB, 00101 = 4 KB, 00110 = 8 KB, 00111 = 16KB, 01000 = 32 KB, 01001 = 64 KB, 01010 = 128KB, 01011 = 256 KB, 01100 = 512 KB, 01101 =1 MB, 01110 = 2 MB, 01111 = 4 MB, 10000 = 8MB, 10001 = 16 MB, 10010 = 32 MB, 10011 = 64MB, 10100 = 128 MB, 10101 = 256 MB, 10110 =512 MB, 10111 = 1 GB, 11000 = 2 GB"]
    #[inline(always)]
    #[must_use]
    pub fn bar3a(&mut self) -> Bar3aW<PhysicalFunctionBarConfiguration0Spec> {
        Bar3aW::new(self, 24)
    }
    #[doc = "Bits 29:31 - BAR 3 Control \\[BAR3C\\]
Specifies the configuration of BAR3. The various encodings are: 000: Disabled 001: 32bit IO BAR 010- 011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn bar3c(&mut self) -> Bar3cW<PhysicalFunctionBarConfiguration0Spec> {
        Bar3cW::new(self, 29)
    }
}
#[doc = "Physical Function BAR Configuration Register 0 Specifies the configuration of BAR3. The various encodings are: 000: Disabled 001: 32bit IO BAR 010- 011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`physical_function_bar_configuration_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`physical_function_bar_configuration_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhysicalFunctionBarConfiguration0Spec;
impl crate::RegisterSpec for PhysicalFunctionBarConfiguration0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`physical_function_bar_configuration_0::R`](R) reader structure"]
impl crate::Readable for PhysicalFunctionBarConfiguration0Spec {}
#[doc = "`write(|w| ..)` method takes [`physical_function_bar_configuration_0::W`](W) writer structure"]
impl crate::Writable for PhysicalFunctionBarConfiguration0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHYSICAL_FUNCTION_BAR_CONFIGURATION_0 to value 0x8fcf_8fcf"]
impl crate::Resettable for PhysicalFunctionBarConfiguration0Spec {
    const RESET_VALUE: u32 = 0x8fcf_8fcf;
}
