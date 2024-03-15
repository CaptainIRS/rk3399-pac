#[doc = "Register `VIRTUAL_FUNCTION_BAR_CONFIGURATION_0` reader"]
pub type R = crate::R<VirtualFunctionBarConfiguration0Spec>;
#[doc = "Register `VIRTUAL_FUNCTION_BAR_CONFIGURATION_0` writer"]
pub type W = crate::W<VirtualFunctionBarConfiguration0Spec>;
#[doc = "Field `VFBAR0A` reader - VF BAR 0 Aperture \\[VFBAR0A\\]
Specifies the aperture of the 32-bit VF BAR 0 or 64bit VF BAR0-1. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 = 512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4Kbytes, 00110 = 8 Kbytes, 00111 = 16 Kbytes, 01000 = 32 Kbytes, 01001 = 64 Kbytes, 01010= 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512 Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16 Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes, 10100 = 128 Mbytes, 10101 = 256 Mbytes,10110 = 512 Mbytes, 10111 = 1 Gbyte, 11000 =2 Gbytes, 11001 = 4 Gbytes, 11010 = 8 Gbytes,11011 = 16 Gbytes, 11100 = 32 Gbytes, 11101= 64 Gbytes, 11110 = 128 Gbytes, 11111 = 256 Gbytes"]
pub type Vfbar0aR = crate::FieldReader;
#[doc = "Field `VFBAR0A` writer - VF BAR 0 Aperture \\[VFBAR0A\\]
Specifies the aperture of the 32-bit VF BAR 0 or 64bit VF BAR0-1. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 = 512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4Kbytes, 00110 = 8 Kbytes, 00111 = 16 Kbytes, 01000 = 32 Kbytes, 01001 = 64 Kbytes, 01010= 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512 Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16 Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes, 10100 = 128 Mbytes, 10101 = 256 Mbytes,10110 = 512 Mbytes, 10111 = 1 Gbyte, 11000 =2 Gbytes, 11001 = 4 Gbytes, 11010 = 8 Gbytes,11011 = 16 Gbytes, 11100 = 32 Gbytes, 11101= 64 Gbytes, 11110 = 128 Gbytes, 11111 = 256 Gbytes"]
pub type Vfbar0aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "VF BAR 0 Control \\[VFBAR0C\\]
Specifies the configuration of VF BAR0. The various encodings are:\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vfbar0c {
    #[doc = "0: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    B000 = 0,
    #[doc = "4: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    B100 = 4,
}
impl From<Vfbar0c> for u8 {
    #[inline(always)]
    fn from(variant: Vfbar0c) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vfbar0c {
    type Ux = u8;
}
#[doc = "Field `VFBAR0C` reader - VF BAR 0 Control \\[VFBAR0C\\]
Specifies the configuration of VF BAR0. The various encodings are:"]
pub type Vfbar0cR = crate::FieldReader<Vfbar0c>;
impl Vfbar0cR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vfbar0c> {
        match self.bits {
            0 => Some(Vfbar0c::B000),
            4 => Some(Vfbar0c::B100),
            _ => None,
        }
    }
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Vfbar0c::B000
    }
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Vfbar0c::B100
    }
}
#[doc = "Field `VFBAR0C` writer - VF BAR 0 Control \\[VFBAR0C\\]
Specifies the configuration of VF BAR0. The various encodings are:"]
pub type Vfbar0cW<'a, REG> = crate::FieldWriter<'a, REG, 3, Vfbar0c>;
impl<'a, REG> Vfbar0cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Vfbar0c::B000)
    }
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110: 64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Vfbar0c::B100)
    }
}
#[doc = "Field `VFBAR1A` reader - VF BAR 1 Aperture \\[VFBAR1A\\]
Specifies the aperture of the VF BAR 1 when it is configured as a 32-bit BAR. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 =512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4 Kbytes, 00110 = 8 Kbytes, 00111 = 16Kbytes, 01000 = 32 Kbytes, 01001 = 64 Kbytes,01010 = 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512 Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes,10100 = 128 Mbytes, 10101 = 256 Mbytes,10110= 512 Mbytes, 10111 = 1 Gbyte, 11000 = 2 Gbytes"]
pub type Vfbar1aR = crate::FieldReader;
#[doc = "Field `VFBAR1A` writer - VF BAR 1 Aperture \\[VFBAR1A\\]
Specifies the aperture of the VF BAR 1 when it is configured as a 32-bit BAR. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 =512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4 Kbytes, 00110 = 8 Kbytes, 00111 = 16Kbytes, 01000 = 32 Kbytes, 01001 = 64 Kbytes,01010 = 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512 Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes,10100 = 128 Mbytes, 10101 = 256 Mbytes,10110= 512 Mbytes, 10111 = 1 Gbyte, 11000 = 2 Gbytes"]
pub type Vfbar1aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "VF BAR 1 Control \\[VFBAR1C\\]
Specifies the configuration of VF BAR1. The various encodings are:\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vfbar1c {
    #[doc = "0: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    B000 = 0,
    #[doc = "4: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    B100 = 4,
}
impl From<Vfbar1c> for u8 {
    #[inline(always)]
    fn from(variant: Vfbar1c) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vfbar1c {
    type Ux = u8;
}
#[doc = "Field `VFBAR1C` reader - VF BAR 1 Control \\[VFBAR1C\\]
Specifies the configuration of VF BAR1. The various encodings are:"]
pub type Vfbar1cR = crate::FieldReader<Vfbar1c>;
impl Vfbar1cR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vfbar1c> {
        match self.bits {
            0 => Some(Vfbar1c::B000),
            4 => Some(Vfbar1c::B100),
            _ => None,
        }
    }
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Vfbar1c::B000
    }
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Vfbar1c::B100
    }
}
#[doc = "Field `VFBAR1C` writer - VF BAR 1 Control \\[VFBAR1C\\]
Specifies the configuration of VF BAR1. The various encodings are:"]
pub type Vfbar1cW<'a, REG> = crate::FieldWriter<'a, REG, 3, Vfbar1c>;
impl<'a, REG> Vfbar1cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Vfbar1c::B000)
    }
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Vfbar1c::B100)
    }
}
#[doc = "Field `VFBAR2A` reader - VF BAR 2 Aperture \\[VFBAR2A\\]
Specifies the aperture of the 32-bit VF BAR 2 or 64bit VF BAR2-3. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 = 512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4Kbytes, 00110 = 8 Kbytes, 00111 = 16 Kbytes,01000 = 32 Kbytes, 01001 = 64 Kbytes, 01010= 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16 Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes, 10100 = 128 Mbytes, 10101 = 256 Mbytes, 10110 = 512 Mbytes, 10111 = 1 Gbyte, 11000 =2 Gbytes, 11001 = 4 Gbytes, 11010 = 8 Gbytes, 11011 = 16 Gbytes, 11100 = 32 Gbytes, 11101= 64 Gbytes, 11110 = 128 Gbytes, 11111 = 256 Gbytes"]
pub type Vfbar2aR = crate::FieldReader;
#[doc = "Field `VFBAR2A` writer - VF BAR 2 Aperture \\[VFBAR2A\\]
Specifies the aperture of the 32-bit VF BAR 2 or 64bit VF BAR2-3. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 = 512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4Kbytes, 00110 = 8 Kbytes, 00111 = 16 Kbytes,01000 = 32 Kbytes, 01001 = 64 Kbytes, 01010= 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16 Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes, 10100 = 128 Mbytes, 10101 = 256 Mbytes, 10110 = 512 Mbytes, 10111 = 1 Gbyte, 11000 =2 Gbytes, 11001 = 4 Gbytes, 11010 = 8 Gbytes, 11011 = 16 Gbytes, 11100 = 32 Gbytes, 11101= 64 Gbytes, 11110 = 128 Gbytes, 11111 = 256 Gbytes"]
pub type Vfbar2aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "VF BAR 2 Control \\[VFBAR2C\\]
Specifies the configuration of VF BAR2. The various encodings are:\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vfbar2c {
    #[doc = "0: 64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    B000 = 0,
    #[doc = "6: 64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    B110 = 6,
}
impl From<Vfbar2c> for u8 {
    #[inline(always)]
    fn from(variant: Vfbar2c) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vfbar2c {
    type Ux = u8;
}
#[doc = "Field `VFBAR2C` reader - VF BAR 2 Control \\[VFBAR2C\\]
Specifies the configuration of VF BAR2. The various encodings are:"]
pub type Vfbar2cR = crate::FieldReader<Vfbar2c>;
impl Vfbar2cR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vfbar2c> {
        match self.bits {
            0 => Some(Vfbar2c::B000),
            6 => Some(Vfbar2c::B110),
            _ => None,
        }
    }
    #[doc = "64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Vfbar2c::B000
    }
    #[doc = "64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Vfbar2c::B110
    }
}
#[doc = "Field `VFBAR2C` writer - VF BAR 2 Control \\[VFBAR2C\\]
Specifies the configuration of VF BAR2. The various encodings are:"]
pub type Vfbar2cW<'a, REG> = crate::FieldWriter<'a, REG, 3, Vfbar2c>;
impl<'a, REG> Vfbar2cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Vfbar2c::B000)
    }
    #[doc = "64bit memory BAR, non prefetchable 111: 64bit memory BAR, prefetchable"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Vfbar2c::B110)
    }
}
#[doc = "Field `VFBAR3A` reader - VF BAR 3 Aperture \\[VFBAR3A\\]
Specifies the aperture of the VF BAR 3 when it is configured as a 32-bit BAR. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 =512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4 Kbytes, 00110 = 8 Kbytes, 00111 = 16Kbytes, 01000 = 32 Kbytes, 01001 = 64 Kbytes, 01010 = 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512 Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16 Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes, 10100 = 128 Mbytes, 10101 = 256 Mbytes, 10110 = 512 Mbytes, 10111 = 1 Gbyte, 11000 = 2 Gbytes"]
pub type Vfbar3aR = crate::FieldReader;
#[doc = "Field `VFBAR3A` writer - VF BAR 3 Aperture \\[VFBAR3A\\]
Specifies the aperture of the VF BAR 3 when it is configured as a 32-bit BAR. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 =512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4 Kbytes, 00110 = 8 Kbytes, 00111 = 16Kbytes, 01000 = 32 Kbytes, 01001 = 64 Kbytes, 01010 = 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512 Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16 Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes, 10100 = 128 Mbytes, 10101 = 256 Mbytes, 10110 = 512 Mbytes, 10111 = 1 Gbyte, 11000 = 2 Gbytes"]
pub type Vfbar3aW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "VF BAR 3 Control \\[VFBAR3C\\]
Specifies the configuration of VF BAR3. The various encodings are:\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vfbar3c {
    #[doc = "0: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    B000 = 0,
    #[doc = "4: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    B100 = 4,
}
impl From<Vfbar3c> for u8 {
    #[inline(always)]
    fn from(variant: Vfbar3c) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vfbar3c {
    type Ux = u8;
}
#[doc = "Field `VFBAR3C` reader - VF BAR 3 Control \\[VFBAR3C\\]
Specifies the configuration of VF BAR3. The various encodings are:"]
pub type Vfbar3cR = crate::FieldReader<Vfbar3c>;
impl Vfbar3cR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vfbar3c> {
        match self.bits {
            0 => Some(Vfbar3c::B000),
            4 => Some(Vfbar3c::B100),
            _ => None,
        }
    }
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Vfbar3c::B000
    }
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Vfbar3c::B100
    }
}
#[doc = "Field `VFBAR3C` writer - VF BAR 3 Control \\[VFBAR3C\\]
Specifies the configuration of VF BAR3. The various encodings are:"]
pub type Vfbar3cW<'a, REG> = crate::FieldWriter<'a, REG, 3, Vfbar3c>;
impl<'a, REG> Vfbar3cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Vfbar3c::B000)
    }
    #[doc = "32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Vfbar3c::B100)
    }
}
impl R {
    #[doc = "Bits 0:4 - VF BAR 0 Aperture \\[VFBAR0A\\]
Specifies the aperture of the 32-bit VF BAR 0 or 64bit VF BAR0-1. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 = 512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4Kbytes, 00110 = 8 Kbytes, 00111 = 16 Kbytes, 01000 = 32 Kbytes, 01001 = 64 Kbytes, 01010= 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512 Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16 Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes, 10100 = 128 Mbytes, 10101 = 256 Mbytes,10110 = 512 Mbytes, 10111 = 1 Gbyte, 11000 =2 Gbytes, 11001 = 4 Gbytes, 11010 = 8 Gbytes,11011 = 16 Gbytes, 11100 = 32 Gbytes, 11101= 64 Gbytes, 11110 = 128 Gbytes, 11111 = 256 Gbytes"]
    #[inline(always)]
    pub fn vfbar0a(&self) -> Vfbar0aR {
        Vfbar0aR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - VF BAR 0 Control \\[VFBAR0C\\]
Specifies the configuration of VF BAR0. The various encodings are:"]
    #[inline(always)]
    pub fn vfbar0c(&self) -> Vfbar0cR {
        Vfbar0cR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - VF BAR 1 Aperture \\[VFBAR1A\\]
Specifies the aperture of the VF BAR 1 when it is configured as a 32-bit BAR. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 =512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4 Kbytes, 00110 = 8 Kbytes, 00111 = 16Kbytes, 01000 = 32 Kbytes, 01001 = 64 Kbytes,01010 = 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512 Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes,10100 = 128 Mbytes, 10101 = 256 Mbytes,10110= 512 Mbytes, 10111 = 1 Gbyte, 11000 = 2 Gbytes"]
    #[inline(always)]
    pub fn vfbar1a(&self) -> Vfbar1aR {
        Vfbar1aR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - VF BAR 1 Control \\[VFBAR1C\\]
Specifies the configuration of VF BAR1. The various encodings are:"]
    #[inline(always)]
    pub fn vfbar1c(&self) -> Vfbar1cR {
        Vfbar1cR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - VF BAR 2 Aperture \\[VFBAR2A\\]
Specifies the aperture of the 32-bit VF BAR 2 or 64bit VF BAR2-3. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 = 512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4Kbytes, 00110 = 8 Kbytes, 00111 = 16 Kbytes,01000 = 32 Kbytes, 01001 = 64 Kbytes, 01010= 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16 Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes, 10100 = 128 Mbytes, 10101 = 256 Mbytes, 10110 = 512 Mbytes, 10111 = 1 Gbyte, 11000 =2 Gbytes, 11001 = 4 Gbytes, 11010 = 8 Gbytes, 11011 = 16 Gbytes, 11100 = 32 Gbytes, 11101= 64 Gbytes, 11110 = 128 Gbytes, 11111 = 256 Gbytes"]
    #[inline(always)]
    pub fn vfbar2a(&self) -> Vfbar2aR {
        Vfbar2aR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - VF BAR 2 Control \\[VFBAR2C\\]
Specifies the configuration of VF BAR2. The various encodings are:"]
    #[inline(always)]
    pub fn vfbar2c(&self) -> Vfbar2cR {
        Vfbar2cR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:28 - VF BAR 3 Aperture \\[VFBAR3A\\]
Specifies the aperture of the VF BAR 3 when it is configured as a 32-bit BAR. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 =512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4 Kbytes, 00110 = 8 Kbytes, 00111 = 16Kbytes, 01000 = 32 Kbytes, 01001 = 64 Kbytes, 01010 = 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512 Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16 Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes, 10100 = 128 Mbytes, 10101 = 256 Mbytes, 10110 = 512 Mbytes, 10111 = 1 Gbyte, 11000 = 2 Gbytes"]
    #[inline(always)]
    pub fn vfbar3a(&self) -> Vfbar3aR {
        Vfbar3aR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - VF BAR 3 Control \\[VFBAR3C\\]
Specifies the configuration of VF BAR3. The various encodings are:"]
    #[inline(always)]
    pub fn vfbar3c(&self) -> Vfbar3cR {
        Vfbar3cR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - VF BAR 0 Aperture \\[VFBAR0A\\]
Specifies the aperture of the 32-bit VF BAR 0 or 64bit VF BAR0-1. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 = 512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4Kbytes, 00110 = 8 Kbytes, 00111 = 16 Kbytes, 01000 = 32 Kbytes, 01001 = 64 Kbytes, 01010= 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512 Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16 Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes, 10100 = 128 Mbytes, 10101 = 256 Mbytes,10110 = 512 Mbytes, 10111 = 1 Gbyte, 11000 =2 Gbytes, 11001 = 4 Gbytes, 11010 = 8 Gbytes,11011 = 16 Gbytes, 11100 = 32 Gbytes, 11101= 64 Gbytes, 11110 = 128 Gbytes, 11111 = 256 Gbytes"]
    #[inline(always)]
    #[must_use]
    pub fn vfbar0a(&mut self) -> Vfbar0aW<VirtualFunctionBarConfiguration0Spec> {
        Vfbar0aW::new(self, 0)
    }
    #[doc = "Bits 5:7 - VF BAR 0 Control \\[VFBAR0C\\]
Specifies the configuration of VF BAR0. The various encodings are:"]
    #[inline(always)]
    #[must_use]
    pub fn vfbar0c(&mut self) -> Vfbar0cW<VirtualFunctionBarConfiguration0Spec> {
        Vfbar0cW::new(self, 5)
    }
    #[doc = "Bits 8:12 - VF BAR 1 Aperture \\[VFBAR1A\\]
Specifies the aperture of the VF BAR 1 when it is configured as a 32-bit BAR. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 =512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4 Kbytes, 00110 = 8 Kbytes, 00111 = 16Kbytes, 01000 = 32 Kbytes, 01001 = 64 Kbytes,01010 = 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512 Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes,10100 = 128 Mbytes, 10101 = 256 Mbytes,10110= 512 Mbytes, 10111 = 1 Gbyte, 11000 = 2 Gbytes"]
    #[inline(always)]
    #[must_use]
    pub fn vfbar1a(&mut self) -> Vfbar1aW<VirtualFunctionBarConfiguration0Spec> {
        Vfbar1aW::new(self, 8)
    }
    #[doc = "Bits 13:15 - VF BAR 1 Control \\[VFBAR1C\\]
Specifies the configuration of VF BAR1. The various encodings are:"]
    #[inline(always)]
    #[must_use]
    pub fn vfbar1c(&mut self) -> Vfbar1cW<VirtualFunctionBarConfiguration0Spec> {
        Vfbar1cW::new(self, 13)
    }
    #[doc = "Bits 16:20 - VF BAR 2 Aperture \\[VFBAR2A\\]
Specifies the aperture of the 32-bit VF BAR 2 or 64bit VF BAR2-3. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 = 512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4Kbytes, 00110 = 8 Kbytes, 00111 = 16 Kbytes,01000 = 32 Kbytes, 01001 = 64 Kbytes, 01010= 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16 Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes, 10100 = 128 Mbytes, 10101 = 256 Mbytes, 10110 = 512 Mbytes, 10111 = 1 Gbyte, 11000 =2 Gbytes, 11001 = 4 Gbytes, 11010 = 8 Gbytes, 11011 = 16 Gbytes, 11100 = 32 Gbytes, 11101= 64 Gbytes, 11110 = 128 Gbytes, 11111 = 256 Gbytes"]
    #[inline(always)]
    #[must_use]
    pub fn vfbar2a(&mut self) -> Vfbar2aW<VirtualFunctionBarConfiguration0Spec> {
        Vfbar2aW::new(self, 16)
    }
    #[doc = "Bits 21:23 - VF BAR 2 Control \\[VFBAR2C\\]
Specifies the configuration of VF BAR2. The various encodings are:"]
    #[inline(always)]
    #[must_use]
    pub fn vfbar2c(&mut self) -> Vfbar2cW<VirtualFunctionBarConfiguration0Spec> {
        Vfbar2cW::new(self, 21)
    }
    #[doc = "Bits 24:28 - VF BAR 3 Aperture \\[VFBAR3A\\]
Specifies the aperture of the VF BAR 3 when it is configured as a 32-bit BAR. The encodings are: 00000 = 128 Bytes, 0001 = 256 Bytes, 0010 =512 Bytes, 0011 = 1 Kbytes, 00100 = 2 Kbytes, 00101 = 4 Kbytes, 00110 = 8 Kbytes, 00111 = 16Kbytes, 01000 = 32 Kbytes, 01001 = 64 Kbytes, 01010 = 128 Kbytes, 01011 = 256 Kbytes, 01100 = 512 Kbytes, 01101 = 1 Mbyte, 01110 = 2 Mbytes, 01111 = 4 Mbytes, 10000 = 8 Mbytes, 10001 = 16 Mbytes, 10010 = 32 Mbytes, 10011 = 64 Mbytes, 10100 = 128 Mbytes, 10101 = 256 Mbytes, 10110 = 512 Mbytes, 10111 = 1 Gbyte, 11000 = 2 Gbytes"]
    #[inline(always)]
    #[must_use]
    pub fn vfbar3a(&mut self) -> Vfbar3aW<VirtualFunctionBarConfiguration0Spec> {
        Vfbar3aW::new(self, 24)
    }
    #[doc = "Bits 29:31 - VF BAR 3 Control \\[VFBAR3C\\]
Specifies the configuration of VF BAR3. The various encodings are:"]
    #[inline(always)]
    #[must_use]
    pub fn vfbar3c(&mut self) -> Vfbar3cW<VirtualFunctionBarConfiguration0Spec> {
        Vfbar3cW::new(self, 29)
    }
}
#[doc = "Virtual Function BAR Configuration Register 0 Specifies the configuration of VF BAR3. The various encodings are: 000: Disabled 001-011: Reserved 100: 32bit memory BAR, non prefetchable 101: 32bit memory BAR, prefetchable 110-111: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`virtual_function_bar_configuration_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`virtual_function_bar_configuration_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VirtualFunctionBarConfiguration0Spec;
impl crate::RegisterSpec for VirtualFunctionBarConfiguration0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`virtual_function_bar_configuration_0::R`](R) reader structure"]
impl crate::Readable for VirtualFunctionBarConfiguration0Spec {}
#[doc = "`write(|w| ..)` method takes [`virtual_function_bar_configuration_0::W`](W) writer structure"]
impl crate::Writable for VirtualFunctionBarConfiguration0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VIRTUAL_FUNCTION_BAR_CONFIGURATION_0 to value 0x8fcf_8fcf"]
impl crate::Resettable for VirtualFunctionBarConfiguration0Spec {
    const RESET_VALUE: u32 = 0x8fcf_8fcf;
}
