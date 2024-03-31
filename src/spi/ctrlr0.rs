#[doc = "Register `CTRLR0` reader"]
pub type R = crate::R<Ctrlr0Spec>;
#[doc = "Register `CTRLR0` writer"]
pub type W = crate::W<Ctrlr0Spec>;
#[doc = "Data Frame Size\n\nSelects the data frame length.\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dfs {
    #[doc = "0: 4bit data"]
    B00 = 0,
    #[doc = "1: 8bit data"]
    B01 = 1,
    #[doc = "2: 16bit data"]
    B10 = 2,
}
impl From<Dfs> for u8 {
    #[inline(always)]
    fn from(variant: Dfs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dfs {
    type Ux = u8;
}
#[doc = "Field `DFS` reader - Data Frame Size\n\nSelects the data frame length."]
pub type DfsR = crate::FieldReader<Dfs>;
impl DfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dfs> {
        match self.bits {
            0 => Some(Dfs::B00),
            1 => Some(Dfs::B01),
            2 => Some(Dfs::B10),
            _ => None,
        }
    }
    #[doc = "4bit data"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Dfs::B00
    }
    #[doc = "8bit data"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Dfs::B01
    }
    #[doc = "16bit data"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Dfs::B10
    }
}
#[doc = "Field `DFS` writer - Data Frame Size\n\nSelects the data frame length."]
pub type DfsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dfs>;
impl<'a, REG> DfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4bit data"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Dfs::B00)
    }
    #[doc = "8bit data"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Dfs::B01)
    }
    #[doc = "16bit data"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Dfs::B10)
    }
}
#[doc = "Control Frame Size\n\nSelects the length of the control word for the Microwire frame\n\nformat.\n\n4'b0000~0010:reserved\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfs {
    #[doc = "3: 4-bit serial data transfer"]
    B0011 = 3,
    #[doc = "4: 5-bit serial data transfer"]
    B0100 = 4,
    #[doc = "5: 6-bit serial data transfer"]
    B0101 = 5,
    #[doc = "6: 7-bit serial data transfer"]
    B0110 = 6,
    #[doc = "7: 8-bit serial data transfer"]
    B0111 = 7,
    #[doc = "8: 9-bit serial data transfer"]
    B1000 = 8,
    #[doc = "9: 10-bit serial data transfer"]
    B1001 = 9,
    #[doc = "10: 11-bit serial data transfer"]
    B1010 = 10,
    #[doc = "11: 12-bit serial data transfer"]
    B1011 = 11,
    #[doc = "12: 13-bit serial data transfer"]
    B1100 = 12,
    #[doc = "13: 14-bit serial data transfer"]
    B1101 = 13,
    #[doc = "14: 15-bit serial data transfer"]
    B1110 = 14,
    #[doc = "15: 16-bit serial data transfer"]
    B1111 = 15,
}
impl From<Cfs> for u8 {
    #[inline(always)]
    fn from(variant: Cfs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfs {
    type Ux = u8;
}
#[doc = "Field `CFS` reader - Control Frame Size\n\nSelects the length of the control word for the Microwire frame\n\nformat.\n\n4'b0000~0010:reserved"]
pub type CfsR = crate::FieldReader<Cfs>;
impl CfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfs> {
        match self.bits {
            3 => Some(Cfs::B0011),
            4 => Some(Cfs::B0100),
            5 => Some(Cfs::B0101),
            6 => Some(Cfs::B0110),
            7 => Some(Cfs::B0111),
            8 => Some(Cfs::B1000),
            9 => Some(Cfs::B1001),
            10 => Some(Cfs::B1010),
            11 => Some(Cfs::B1011),
            12 => Some(Cfs::B1100),
            13 => Some(Cfs::B1101),
            14 => Some(Cfs::B1110),
            15 => Some(Cfs::B1111),
            _ => None,
        }
    }
    #[doc = "4-bit serial data transfer"]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == Cfs::B0011
    }
    #[doc = "5-bit serial data transfer"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == Cfs::B0100
    }
    #[doc = "6-bit serial data transfer"]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == Cfs::B0101
    }
    #[doc = "7-bit serial data transfer"]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == Cfs::B0110
    }
    #[doc = "8-bit serial data transfer"]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == Cfs::B0111
    }
    #[doc = "9-bit serial data transfer"]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == Cfs::B1000
    }
    #[doc = "10-bit serial data transfer"]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == Cfs::B1001
    }
    #[doc = "11-bit serial data transfer"]
    #[inline(always)]
    pub fn is_b1010(&self) -> bool {
        *self == Cfs::B1010
    }
    #[doc = "12-bit serial data transfer"]
    #[inline(always)]
    pub fn is_b1011(&self) -> bool {
        *self == Cfs::B1011
    }
    #[doc = "13-bit serial data transfer"]
    #[inline(always)]
    pub fn is_b1100(&self) -> bool {
        *self == Cfs::B1100
    }
    #[doc = "14-bit serial data transfer"]
    #[inline(always)]
    pub fn is_b1101(&self) -> bool {
        *self == Cfs::B1101
    }
    #[doc = "15-bit serial data transfer"]
    #[inline(always)]
    pub fn is_b1110(&self) -> bool {
        *self == Cfs::B1110
    }
    #[doc = "16-bit serial data transfer"]
    #[inline(always)]
    pub fn is_b1111(&self) -> bool {
        *self == Cfs::B1111
    }
}
#[doc = "Field `CFS` writer - Control Frame Size\n\nSelects the length of the control word for the Microwire frame\n\nformat.\n\n4'b0000~0010:reserved"]
pub type CfsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Cfs>;
impl<'a, REG> CfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4-bit serial data transfer"]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(Cfs::B0011)
    }
    #[doc = "5-bit serial data transfer"]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(Cfs::B0100)
    }
    #[doc = "6-bit serial data transfer"]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(Cfs::B0101)
    }
    #[doc = "7-bit serial data transfer"]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(Cfs::B0110)
    }
    #[doc = "8-bit serial data transfer"]
    #[inline(always)]
    pub fn b0111(self) -> &'a mut crate::W<REG> {
        self.variant(Cfs::B0111)
    }
    #[doc = "9-bit serial data transfer"]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(Cfs::B1000)
    }
    #[doc = "10-bit serial data transfer"]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(Cfs::B1001)
    }
    #[doc = "11-bit serial data transfer"]
    #[inline(always)]
    pub fn b1010(self) -> &'a mut crate::W<REG> {
        self.variant(Cfs::B1010)
    }
    #[doc = "12-bit serial data transfer"]
    #[inline(always)]
    pub fn b1011(self) -> &'a mut crate::W<REG> {
        self.variant(Cfs::B1011)
    }
    #[doc = "13-bit serial data transfer"]
    #[inline(always)]
    pub fn b1100(self) -> &'a mut crate::W<REG> {
        self.variant(Cfs::B1100)
    }
    #[doc = "14-bit serial data transfer"]
    #[inline(always)]
    pub fn b1101(self) -> &'a mut crate::W<REG> {
        self.variant(Cfs::B1101)
    }
    #[doc = "15-bit serial data transfer"]
    #[inline(always)]
    pub fn b1110(self) -> &'a mut crate::W<REG> {
        self.variant(Cfs::B1110)
    }
    #[doc = "16-bit serial data transfer"]
    #[inline(always)]
    pub fn b1111(self) -> &'a mut crate::W<REG> {
        self.variant(Cfs::B1111)
    }
}
#[doc = "Serial Clock Phase\n\nValid when the frame format is set to Motorola SPI.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scph {
    #[doc = "0: Serial clock toggles in middle of first data bit"]
    B0 = 0,
    #[doc = "1: Serial clock toggles at start of first data bit"]
    B1 = 1,
}
impl From<Scph> for bool {
    #[inline(always)]
    fn from(variant: Scph) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCPH` reader - Serial Clock Phase\n\nValid when the frame format is set to Motorola SPI."]
pub type ScphR = crate::BitReader<Scph>;
impl ScphR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scph {
        match self.bits {
            false => Scph::B0,
            true => Scph::B1,
        }
    }
    #[doc = "Serial clock toggles in middle of first data bit"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Scph::B0
    }
    #[doc = "Serial clock toggles at start of first data bit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Scph::B1
    }
}
#[doc = "Field `SCPH` writer - Serial Clock Phase\n\nValid when the frame format is set to Motorola SPI."]
pub type ScphW<'a, REG> = crate::BitWriter<'a, REG, Scph>;
impl<'a, REG> ScphW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Serial clock toggles in middle of first data bit"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Scph::B0)
    }
    #[doc = "Serial clock toggles at start of first data bit"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Scph::B1)
    }
}
#[doc = "Serial Clock Polarity\n\nValid when the frame format is set to Motorola SPI.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scpol {
    #[doc = "0: Inactive state of serial clock is low"]
    B0 = 0,
    #[doc = "1: Inactive state of serial clock is high"]
    B1 = 1,
}
impl From<Scpol> for bool {
    #[inline(always)]
    fn from(variant: Scpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCPOL` reader - Serial Clock Polarity\n\nValid when the frame format is set to Motorola SPI."]
pub type ScpolR = crate::BitReader<Scpol>;
impl ScpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scpol {
        match self.bits {
            false => Scpol::B0,
            true => Scpol::B1,
        }
    }
    #[doc = "Inactive state of serial clock is low"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Scpol::B0
    }
    #[doc = "Inactive state of serial clock is high"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Scpol::B1
    }
}
#[doc = "Field `SCPOL` writer - Serial Clock Polarity\n\nValid when the frame format is set to Motorola SPI."]
pub type ScpolW<'a, REG> = crate::BitWriter<'a, REG, Scpol>;
impl<'a, REG> ScpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Inactive state of serial clock is low"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Scpol::B0)
    }
    #[doc = "Inactive state of serial clock is high"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Scpol::B1)
    }
}
#[doc = "Chip Select Mode\n\nValid when the frame format is set to Motorola SPI and SPI used\n\nas a master.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Csm {
    #[doc = "0: ss_n keep low after every frame data is transferred."]
    B00 = 0,
    #[doc = "1: ss_n be high for half sclk_out cycles after every frame data is transferred."]
    B01 = 1,
    #[doc = "2: ss_n be high for one sclk_out cycle after every frame data is transferred."]
    B10 = 2,
}
impl From<Csm> for u8 {
    #[inline(always)]
    fn from(variant: Csm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Csm {
    type Ux = u8;
}
#[doc = "Field `CSM` reader - Chip Select Mode\n\nValid when the frame format is set to Motorola SPI and SPI used\n\nas a master."]
pub type CsmR = crate::FieldReader<Csm>;
impl CsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Csm> {
        match self.bits {
            0 => Some(Csm::B00),
            1 => Some(Csm::B01),
            2 => Some(Csm::B10),
            _ => None,
        }
    }
    #[doc = "ss_n keep low after every frame data is transferred."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Csm::B00
    }
    #[doc = "ss_n be high for half sclk_out cycles after every frame data is transferred."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Csm::B01
    }
    #[doc = "ss_n be high for one sclk_out cycle after every frame data is transferred."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Csm::B10
    }
}
#[doc = "Field `CSM` writer - Chip Select Mode\n\nValid when the frame format is set to Motorola SPI and SPI used\n\nas a master."]
pub type CsmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Csm>;
impl<'a, REG> CsmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ss_n keep low after every frame data is transferred."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Csm::B00)
    }
    #[doc = "ss_n be high for half sclk_out cycles after every frame data is transferred."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Csm::B01)
    }
    #[doc = "ss_n be high for one sclk_out cycle after every frame data is transferred."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Csm::B10)
    }
}
#[doc = "ss_n to sclk_out delay\n\nValid when the frame format is set to Motorola SPI and SPI used\n\nas a master.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssd {
    #[doc = "0: the period between ss_n active and sclk_out active is half sclk_out cycles."]
    B0 = 0,
    #[doc = "1: the period between ss_n active and sclk_out active is one sclk_out cycle."]
    B1 = 1,
}
impl From<Ssd> for bool {
    #[inline(always)]
    fn from(variant: Ssd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSD` reader - ss_n to sclk_out delay\n\nValid when the frame format is set to Motorola SPI and SPI used\n\nas a master."]
pub type SsdR = crate::BitReader<Ssd>;
impl SsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssd {
        match self.bits {
            false => Ssd::B0,
            true => Ssd::B1,
        }
    }
    #[doc = "the period between ss_n active and sclk_out active is half sclk_out cycles."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ssd::B0
    }
    #[doc = "the period between ss_n active and sclk_out active is one sclk_out cycle."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ssd::B1
    }
}
#[doc = "Field `SSD` writer - ss_n to sclk_out delay\n\nValid when the frame format is set to Motorola SPI and SPI used\n\nas a master."]
pub type SsdW<'a, REG> = crate::BitWriter<'a, REG, Ssd>;
impl<'a, REG> SsdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "the period between ss_n active and sclk_out active is half sclk_out cycles."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssd::B0)
    }
    #[doc = "the period between ss_n active and sclk_out active is one sclk_out cycle."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssd::B1)
    }
}
#[doc = "Endian Mode\n\nSerial endian mode can be configured by this bit. Apb endian\n\nmode is always little endian.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em {
    #[doc = "0: little endian"]
    B0 = 0,
    #[doc = "1: big endian"]
    B1 = 1,
}
impl From<Em> for bool {
    #[inline(always)]
    fn from(variant: Em) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM` reader - Endian Mode\n\nSerial endian mode can be configured by this bit. Apb endian\n\nmode is always little endian."]
pub type EmR = crate::BitReader<Em>;
impl EmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em {
        match self.bits {
            false => Em::B0,
            true => Em::B1,
        }
    }
    #[doc = "little endian"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Em::B0
    }
    #[doc = "big endian"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Em::B1
    }
}
#[doc = "Field `EM` writer - Endian Mode\n\nSerial endian mode can be configured by this bit. Apb endian\n\nmode is always little endian."]
pub type EmW<'a, REG> = crate::BitWriter<'a, REG, Em>;
impl<'a, REG> EmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "little endian"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Em::B0)
    }
    #[doc = "big endian"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Em::B1)
    }
}
#[doc = "First Bit Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fbm {
    #[doc = "0: first bit is MSB"]
    B0 = 0,
    #[doc = "1: first bit is LSB"]
    B1 = 1,
}
impl From<Fbm> for bool {
    #[inline(always)]
    fn from(variant: Fbm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FBM` reader - First Bit Mode"]
pub type FbmR = crate::BitReader<Fbm>;
impl FbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fbm {
        match self.bits {
            false => Fbm::B0,
            true => Fbm::B1,
        }
    }
    #[doc = "first bit is MSB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fbm::B0
    }
    #[doc = "first bit is LSB"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fbm::B1
    }
}
#[doc = "Field `FBM` writer - First Bit Mode"]
pub type FbmW<'a, REG> = crate::BitWriter<'a, REG, Fbm>;
impl<'a, REG> FbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "first bit is MSB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fbm::B0)
    }
    #[doc = "first bit is LSB"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fbm::B1)
    }
}
#[doc = "Byte and Halfword Transform\n\nValid when data frame size is 8bit.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bht {
    #[doc = "0: apb 16bit write/read, spi 8bit write/read"]
    B0 = 0,
    #[doc = "1: apb 8bit write/read, spi 8bit write/read"]
    B1 = 1,
}
impl From<Bht> for bool {
    #[inline(always)]
    fn from(variant: Bht) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BHT` reader - Byte and Halfword Transform\n\nValid when data frame size is 8bit."]
pub type BhtR = crate::BitReader<Bht>;
impl BhtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bht {
        match self.bits {
            false => Bht::B0,
            true => Bht::B1,
        }
    }
    #[doc = "apb 16bit write/read, spi 8bit write/read"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bht::B0
    }
    #[doc = "apb 8bit write/read, spi 8bit write/read"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bht::B1
    }
}
#[doc = "Field `BHT` writer - Byte and Halfword Transform\n\nValid when data frame size is 8bit."]
pub type BhtW<'a, REG> = crate::BitWriter<'a, REG, Bht>;
impl<'a, REG> BhtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "apb 16bit write/read, spi 8bit write/read"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bht::B0)
    }
    #[doc = "apb 8bit write/read, spi 8bit write/read"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bht::B1)
    }
}
#[doc = "Rxd Sample Delay\n\nWhen SPI is configured as a master, if the rxd data cannot be\n\nsampled by the sclk_out edge at the right time, this register\n\nshould be configured to define the number of the spi_clk cycles\n\nafter the active sclk_out edge to sample rxd data later when SPI\n\nworks at high frequency.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rsd {
    #[doc = "0: do not delay"]
    B00 = 0,
    #[doc = "1: 1 cycle delay"]
    B01 = 1,
    #[doc = "2: 2 cycles delay"]
    B10 = 2,
    #[doc = "3: 3 cycles delay"]
    B11 = 3,
}
impl From<Rsd> for u8 {
    #[inline(always)]
    fn from(variant: Rsd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rsd {
    type Ux = u8;
}
#[doc = "Field `RSD` reader - Rxd Sample Delay\n\nWhen SPI is configured as a master, if the rxd data cannot be\n\nsampled by the sclk_out edge at the right time, this register\n\nshould be configured to define the number of the spi_clk cycles\n\nafter the active sclk_out edge to sample rxd data later when SPI\n\nworks at high frequency."]
pub type RsdR = crate::FieldReader<Rsd>;
impl RsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rsd {
        match self.bits {
            0 => Rsd::B00,
            1 => Rsd::B01,
            2 => Rsd::B10,
            3 => Rsd::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "do not delay"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Rsd::B00
    }
    #[doc = "1 cycle delay"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Rsd::B01
    }
    #[doc = "2 cycles delay"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Rsd::B10
    }
    #[doc = "3 cycles delay"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Rsd::B11
    }
}
#[doc = "Field `RSD` writer - Rxd Sample Delay\n\nWhen SPI is configured as a master, if the rxd data cannot be\n\nsampled by the sclk_out edge at the right time, this register\n\nshould be configured to define the number of the spi_clk cycles\n\nafter the active sclk_out edge to sample rxd data later when SPI\n\nworks at high frequency."]
pub type RsdW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Rsd>;
impl<'a, REG> RsdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "do not delay"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Rsd::B00)
    }
    #[doc = "1 cycle delay"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Rsd::B01)
    }
    #[doc = "2 cycles delay"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Rsd::B10)
    }
    #[doc = "3 cycles delay"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Rsd::B11)
    }
}
#[doc = "Frame Format\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frf {
    #[doc = "0: Motorola SPI"]
    B00 = 0,
    #[doc = "1: Texas Instruments SSP"]
    B01 = 1,
    #[doc = "2: National Semiconductors Microwire"]
    B10 = 2,
}
impl From<Frf> for u8 {
    #[inline(always)]
    fn from(variant: Frf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frf {
    type Ux = u8;
}
#[doc = "Field `FRF` reader - Frame Format"]
pub type FrfR = crate::FieldReader<Frf>;
impl FrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Frf> {
        match self.bits {
            0 => Some(Frf::B00),
            1 => Some(Frf::B01),
            2 => Some(Frf::B10),
            _ => None,
        }
    }
    #[doc = "Motorola SPI"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Frf::B00
    }
    #[doc = "Texas Instruments SSP"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Frf::B01
    }
    #[doc = "National Semiconductors Microwire"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Frf::B10
    }
}
#[doc = "Field `FRF` writer - Frame Format"]
pub type FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Frf>;
impl<'a, REG> FrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Motorola SPI"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::B00)
    }
    #[doc = "Texas Instruments SSP"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::B01)
    }
    #[doc = "National Semiconductors Microwire"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::B10)
    }
}
#[doc = "Transfer Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Xfm {
    #[doc = "0: Transmit &amp; Receive"]
    B00 = 0,
    #[doc = "1: Transmit Only"]
    B01 = 1,
    #[doc = "2: Receive Only"]
    B10 = 2,
}
impl From<Xfm> for u8 {
    #[inline(always)]
    fn from(variant: Xfm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xfm {
    type Ux = u8;
}
#[doc = "Field `XFM` reader - Transfer Mode"]
pub type XfmR = crate::FieldReader<Xfm>;
impl XfmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Xfm> {
        match self.bits {
            0 => Some(Xfm::B00),
            1 => Some(Xfm::B01),
            2 => Some(Xfm::B10),
            _ => None,
        }
    }
    #[doc = "Transmit &amp; Receive"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Xfm::B00
    }
    #[doc = "Transmit Only"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Xfm::B01
    }
    #[doc = "Receive Only"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Xfm::B10
    }
}
#[doc = "Field `XFM` writer - Transfer Mode"]
pub type XfmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Xfm>;
impl<'a, REG> XfmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transmit &amp; Receive"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Xfm::B00)
    }
    #[doc = "Transmit Only"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Xfm::B01)
    }
    #[doc = "Receive Only"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Xfm::B10)
    }
}
#[doc = "Operation Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opm {
    #[doc = "0: Master Mode"]
    B0 = 0,
    #[doc = "1: Slave Mode"]
    B1 = 1,
}
impl From<Opm> for bool {
    #[inline(always)]
    fn from(variant: Opm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPM` reader - Operation Mode"]
pub type OpmR = crate::BitReader<Opm>;
impl OpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opm {
        match self.bits {
            false => Opm::B0,
            true => Opm::B1,
        }
    }
    #[doc = "Master Mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Opm::B0
    }
    #[doc = "Slave Mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Opm::B1
    }
}
#[doc = "Field `OPM` writer - Operation Mode"]
pub type OpmW<'a, REG> = crate::BitWriter<'a, REG, Opm>;
impl<'a, REG> OpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master Mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Opm::B0)
    }
    #[doc = "Slave Mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Opm::B1)
    }
}
#[doc = "Microwire Transfer Mode\n\nValid when frame format is set to National Semiconductors\n\nMicrowire.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mtm {
    #[doc = "0: non-sequential transfer"]
    B0 = 0,
    #[doc = "1: sequential transfer"]
    B1 = 1,
}
impl From<Mtm> for bool {
    #[inline(always)]
    fn from(variant: Mtm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTM` reader - Microwire Transfer Mode\n\nValid when frame format is set to National Semiconductors\n\nMicrowire."]
pub type MtmR = crate::BitReader<Mtm>;
impl MtmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mtm {
        match self.bits {
            false => Mtm::B0,
            true => Mtm::B1,
        }
    }
    #[doc = "non-sequential transfer"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Mtm::B0
    }
    #[doc = "sequential transfer"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Mtm::B1
    }
}
#[doc = "Field `MTM` writer - Microwire Transfer Mode\n\nValid when frame format is set to National Semiconductors\n\nMicrowire."]
pub type MtmW<'a, REG> = crate::BitWriter<'a, REG, Mtm>;
impl<'a, REG> MtmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "non-sequential transfer"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Mtm::B0)
    }
    #[doc = "sequential transfer"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Mtm::B1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Data Frame Size\n\nSelects the data frame length."]
    #[inline(always)]
    pub fn dfs(&self) -> DfsR {
        DfsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - Control Frame Size\n\nSelects the length of the control word for the Microwire frame\n\nformat.\n\n4'b0000~0010:reserved"]
    #[inline(always)]
    pub fn cfs(&self) -> CfsR {
        CfsR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Serial Clock Phase\n\nValid when the frame format is set to Motorola SPI."]
    #[inline(always)]
    pub fn scph(&self) -> ScphR {
        ScphR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Serial Clock Polarity\n\nValid when the frame format is set to Motorola SPI."]
    #[inline(always)]
    pub fn scpol(&self) -> ScpolR {
        ScpolR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Chip Select Mode\n\nValid when the frame format is set to Motorola SPI and SPI used\n\nas a master."]
    #[inline(always)]
    pub fn csm(&self) -> CsmR {
        CsmR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - ss_n to sclk_out delay\n\nValid when the frame format is set to Motorola SPI and SPI used\n\nas a master."]
    #[inline(always)]
    pub fn ssd(&self) -> SsdR {
        SsdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Endian Mode\n\nSerial endian mode can be configured by this bit. Apb endian\n\nmode is always little endian."]
    #[inline(always)]
    pub fn em(&self) -> EmR {
        EmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - First Bit Mode"]
    #[inline(always)]
    pub fn fbm(&self) -> FbmR {
        FbmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Byte and Halfword Transform\n\nValid when data frame size is 8bit."]
    #[inline(always)]
    pub fn bht(&self) -> BhtR {
        BhtR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Rxd Sample Delay\n\nWhen SPI is configured as a master, if the rxd data cannot be\n\nsampled by the sclk_out edge at the right time, this register\n\nshould be configured to define the number of the spi_clk cycles\n\nafter the active sclk_out edge to sample rxd data later when SPI\n\nworks at high frequency."]
    #[inline(always)]
    pub fn rsd(&self) -> RsdR {
        RsdR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Frame Format"]
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Transfer Mode"]
    #[inline(always)]
    pub fn xfm(&self) -> XfmR {
        XfmR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Operation Mode"]
    #[inline(always)]
    pub fn opm(&self) -> OpmR {
        OpmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Microwire Transfer Mode\n\nValid when frame format is set to National Semiconductors\n\nMicrowire."]
    #[inline(always)]
    pub fn mtm(&self) -> MtmR {
        MtmR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data Frame Size\n\nSelects the data frame length."]
    #[inline(always)]
    #[must_use]
    pub fn dfs(&mut self) -> DfsW<Ctrlr0Spec> {
        DfsW::new(self, 0)
    }
    #[doc = "Bits 2:5 - Control Frame Size\n\nSelects the length of the control word for the Microwire frame\n\nformat.\n\n4'b0000~0010:reserved"]
    #[inline(always)]
    #[must_use]
    pub fn cfs(&mut self) -> CfsW<Ctrlr0Spec> {
        CfsW::new(self, 2)
    }
    #[doc = "Bit 6 - Serial Clock Phase\n\nValid when the frame format is set to Motorola SPI."]
    #[inline(always)]
    #[must_use]
    pub fn scph(&mut self) -> ScphW<Ctrlr0Spec> {
        ScphW::new(self, 6)
    }
    #[doc = "Bit 7 - Serial Clock Polarity\n\nValid when the frame format is set to Motorola SPI."]
    #[inline(always)]
    #[must_use]
    pub fn scpol(&mut self) -> ScpolW<Ctrlr0Spec> {
        ScpolW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Chip Select Mode\n\nValid when the frame format is set to Motorola SPI and SPI used\n\nas a master."]
    #[inline(always)]
    #[must_use]
    pub fn csm(&mut self) -> CsmW<Ctrlr0Spec> {
        CsmW::new(self, 8)
    }
    #[doc = "Bit 10 - ss_n to sclk_out delay\n\nValid when the frame format is set to Motorola SPI and SPI used\n\nas a master."]
    #[inline(always)]
    #[must_use]
    pub fn ssd(&mut self) -> SsdW<Ctrlr0Spec> {
        SsdW::new(self, 10)
    }
    #[doc = "Bit 11 - Endian Mode\n\nSerial endian mode can be configured by this bit. Apb endian\n\nmode is always little endian."]
    #[inline(always)]
    #[must_use]
    pub fn em(&mut self) -> EmW<Ctrlr0Spec> {
        EmW::new(self, 11)
    }
    #[doc = "Bit 12 - First Bit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm(&mut self) -> FbmW<Ctrlr0Spec> {
        FbmW::new(self, 12)
    }
    #[doc = "Bit 13 - Byte and Halfword Transform\n\nValid when data frame size is 8bit."]
    #[inline(always)]
    #[must_use]
    pub fn bht(&mut self) -> BhtW<Ctrlr0Spec> {
        BhtW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Rxd Sample Delay\n\nWhen SPI is configured as a master, if the rxd data cannot be\n\nsampled by the sclk_out edge at the right time, this register\n\nshould be configured to define the number of the spi_clk cycles\n\nafter the active sclk_out edge to sample rxd data later when SPI\n\nworks at high frequency."]
    #[inline(always)]
    #[must_use]
    pub fn rsd(&mut self) -> RsdW<Ctrlr0Spec> {
        RsdW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Frame Format"]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FrfW<Ctrlr0Spec> {
        FrfW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Transfer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn xfm(&mut self) -> XfmW<Ctrlr0Spec> {
        XfmW::new(self, 18)
    }
    #[doc = "Bit 20 - Operation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn opm(&mut self) -> OpmW<Ctrlr0Spec> {
        OpmW::new(self, 20)
    }
    #[doc = "Bit 21 - Microwire Transfer Mode\n\nValid when frame format is set to National Semiconductors\n\nMicrowire."]
    #[inline(always)]
    #[must_use]
    pub fn mtm(&mut self) -> MtmW<Ctrlr0Spec> {
        MtmW::new(self, 21)
    }
}
#[doc = "Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrlr0Spec;
impl crate::RegisterSpec for Ctrlr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlr0::R`](R) reader structure"]
impl crate::Readable for Ctrlr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrlr0::W`](W) writer structure"]
impl crate::Writable for Ctrlr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLR0 to value 0x02"]
impl crate::Resettable for Ctrlr0Spec {
    const RESET_VALUE: u32 = 0x02;
}
