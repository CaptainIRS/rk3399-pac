#[doc = "Register `CONF` reader"]
pub type R = crate::R<ConfSpec>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<ConfSpec>;
#[doc = "Specifies the following\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hashinsel {
    #[doc = "0: Data from independent source;"]
    B00 = 0,
    #[doc = "1: Data from block cipher input;"]
    B01 = 1,
    #[doc = "2: Data from block cipher output;"]
    B10 = 2,
    #[doc = "3: Reserved."]
    B11 = 3,
}
impl From<Hashinsel> for u8 {
    #[inline(always)]
    fn from(variant: Hashinsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hashinsel {
    type Ux = u8;
}
#[doc = "Field `HASHINSEL` reader - Specifies the following"]
pub type HashinselR = crate::FieldReader<Hashinsel>;
impl HashinselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hashinsel {
        match self.bits {
            0 => Hashinsel::B00,
            1 => Hashinsel::B01,
            2 => Hashinsel::B10,
            3 => Hashinsel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Data from independent source;"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Hashinsel::B00
    }
    #[doc = "Data from block cipher input;"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Hashinsel::B01
    }
    #[doc = "Data from block cipher output;"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Hashinsel::B10
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Hashinsel::B11
    }
}
#[doc = "Field `HASHINSEL` writer - Specifies the following"]
pub type HashinselW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Hashinsel>;
impl<'a, REG> HashinselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Data from independent source;"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Hashinsel::B00)
    }
    #[doc = "Data from block cipher input;"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Hashinsel::B01)
    }
    #[doc = "Data from block cipher output;"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Hashinsel::B10)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Hashinsel::B11)
    }
}
#[doc = "Specifies the Destination block cipher of FIFO.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dessel {
    #[doc = "0: AES;"]
    B0 = 0,
    #[doc = "1: DES."]
    B1 = 1,
}
impl From<Dessel> for bool {
    #[inline(always)]
    fn from(variant: Dessel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DESSEL` reader - Specifies the Destination block cipher of FIFO."]
pub type DesselR = crate::BitReader<Dessel>;
impl DesselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dessel {
        match self.bits {
            false => Dessel::B0,
            true => Dessel::B1,
        }
    }
    #[doc = "AES;"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dessel::B0
    }
    #[doc = "DES."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dessel::B1
    }
}
#[doc = "Field `DESSEL` writer - Specifies the Destination block cipher of FIFO."]
pub type DesselW<'a, REG> = crate::BitWriter<'a, REG, Dessel>;
impl<'a, REG> DesselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AES;"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dessel::B0)
    }
    #[doc = "DES."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dessel::B1)
    }
}
#[doc = "Field `BYTESWAP_BRFIFO` reader - If this bit is high, then the data read from the bus is byte-swapped\n\nin a word boundary. If this bit is low (default), then the data is\n\nhanded over to the FIFO without byte-swap. For little endian bus,\n\nthis bit should be 1'b1."]
pub type ByteswapBrfifoR = crate::BitReader;
#[doc = "Field `BYTESWAP_BRFIFO` writer - If this bit is high, then the data read from the bus is byte-swapped\n\nin a word boundary. If this bit is low (default), then the data is\n\nhanded over to the FIFO without byte-swap. For little endian bus,\n\nthis bit should be 1'b1."]
pub type ByteswapBrfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTESWAP_BTFIFO` reader - If this bit is high, then the data read from the bus is byte-swapped\n\nin a word boundary. If this bit is low (default), then the data is\n\nhanded over to the FIFO without byte-swap. For little endian bus,\n\nthis bit should be 1'b1."]
pub type ByteswapBtfifoR = crate::BitReader;
#[doc = "Field `BYTESWAP_BTFIFO` writer - If this bit is high, then the data read from the bus is byte-swapped\n\nin a word boundary. If this bit is low (default), then the data is\n\nhanded over to the FIFO without byte-swap. For little endian bus,\n\nthis bit should be 1'b1."]
pub type ByteswapBtfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTESWAP_HRFIFO` reader - If this bit is high, then the data read from the bus is byte-swapped\n\nin a word boundary. If this bit is low (default), then the data is\n\nhanded over to the FIFO without byte-swap. For little endian bus,\n\nthis bit should be 1'b1."]
pub type ByteswapHrfifoR = crate::BitReader;
#[doc = "Field `BYTESWAP_HRFIFO` writer - If this bit is high, then the data read from the bus is byte-swapped\n\nin a word boundary. If this bit is low (default), then the data is\n\nhanded over to the FIFO without byte-swap. For little endian bus,\n\nthis bit should be 1'b1."]
pub type ByteswapHrfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Block Receive DMA Address Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrAddrMode {
    #[doc = "1: fix"]
    B1 = 1,
    #[doc = "0: increment"]
    B0 = 0,
}
impl From<BrAddrMode> for bool {
    #[inline(always)]
    fn from(variant: BrAddrMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BR_ADDR_MODE` reader - Block Receive DMA Address Mode"]
pub type BrAddrModeR = crate::BitReader<BrAddrMode>;
impl BrAddrModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BrAddrMode {
        match self.bits {
            true => BrAddrMode::B1,
            false => BrAddrMode::B0,
        }
    }
    #[doc = "fix"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BrAddrMode::B1
    }
    #[doc = "increment"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BrAddrMode::B0
    }
}
#[doc = "Field `BR_ADDR_MODE` writer - Block Receive DMA Address Mode"]
pub type BrAddrModeW<'a, REG> = crate::BitWriter<'a, REG, BrAddrMode>;
impl<'a, REG> BrAddrModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fix"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BrAddrMode::B1)
    }
    #[doc = "increment"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BrAddrMode::B0)
    }
}
#[doc = "Block Transmit DMA Address Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BtAddrMode {
    #[doc = "1: fix"]
    B1 = 1,
    #[doc = "0: increment"]
    B0 = 0,
}
impl From<BtAddrMode> for bool {
    #[inline(always)]
    fn from(variant: BtAddrMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BT_ADDR_MODE` reader - Block Transmit DMA Address Mode"]
pub type BtAddrModeR = crate::BitReader<BtAddrMode>;
impl BtAddrModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BtAddrMode {
        match self.bits {
            true => BtAddrMode::B1,
            false => BtAddrMode::B0,
        }
    }
    #[doc = "fix"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BtAddrMode::B1
    }
    #[doc = "increment"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BtAddrMode::B0
    }
}
#[doc = "Field `BT_ADDR_MODE` writer - Block Transmit DMA Address Mode"]
pub type BtAddrModeW<'a, REG> = crate::BitWriter<'a, REG, BtAddrMode>;
impl<'a, REG> BtAddrModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fix"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BtAddrMode::B1)
    }
    #[doc = "increment"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BtAddrMode::B0)
    }
}
#[doc = "Hash Receive DMA Address Mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HrAddrMode {
    #[doc = "1: fix"]
    B1 = 1,
    #[doc = "0: increment"]
    B0 = 0,
}
impl From<HrAddrMode> for bool {
    #[inline(always)]
    fn from(variant: HrAddrMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HR_ADDR_MODE` reader - Hash Receive DMA Address Mode"]
pub type HrAddrModeR = crate::BitReader<HrAddrMode>;
impl HrAddrModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HrAddrMode {
        match self.bits {
            true => HrAddrMode::B1,
            false => HrAddrMode::B0,
        }
    }
    #[doc = "fix"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HrAddrMode::B1
    }
    #[doc = "increment"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HrAddrMode::B0
    }
}
#[doc = "Field `HR_ADDR_MODE` writer - Hash Receive DMA Address Mode"]
pub type HrAddrModeW<'a, REG> = crate::BitWriter<'a, REG, HrAddrMode>;
impl<'a, REG> HrAddrModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "fix"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HrAddrMode::B1)
    }
    #[doc = "increment"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HrAddrMode::B0)
    }
}
impl R {
    #[doc = "Bits 0:1 - Specifies the following"]
    #[inline(always)]
    pub fn hashinsel(&self) -> HashinselR {
        HashinselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Specifies the Destination block cipher of FIFO."]
    #[inline(always)]
    pub fn dessel(&self) -> DesselR {
        DesselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If this bit is high, then the data read from the bus is byte-swapped\n\nin a word boundary. If this bit is low (default), then the data is\n\nhanded over to the FIFO without byte-swap. For little endian bus,\n\nthis bit should be 1'b1."]
    #[inline(always)]
    pub fn byteswap_brfifo(&self) -> ByteswapBrfifoR {
        ByteswapBrfifoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If this bit is high, then the data read from the bus is byte-swapped\n\nin a word boundary. If this bit is low (default), then the data is\n\nhanded over to the FIFO without byte-swap. For little endian bus,\n\nthis bit should be 1'b1."]
    #[inline(always)]
    pub fn byteswap_btfifo(&self) -> ByteswapBtfifoR {
        ByteswapBtfifoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If this bit is high, then the data read from the bus is byte-swapped\n\nin a word boundary. If this bit is low (default), then the data is\n\nhanded over to the FIFO without byte-swap. For little endian bus,\n\nthis bit should be 1'b1."]
    #[inline(always)]
    pub fn byteswap_hrfifo(&self) -> ByteswapHrfifoR {
        ByteswapHrfifoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Block Receive DMA Address Mode"]
    #[inline(always)]
    pub fn br_addr_mode(&self) -> BrAddrModeR {
        BrAddrModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Block Transmit DMA Address Mode"]
    #[inline(always)]
    pub fn bt_addr_mode(&self) -> BtAddrModeR {
        BtAddrModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Hash Receive DMA Address Mode"]
    #[inline(always)]
    pub fn hr_addr_mode(&self) -> HrAddrModeR {
        HrAddrModeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Specifies the following"]
    #[inline(always)]
    #[must_use]
    pub fn hashinsel(&mut self) -> HashinselW<ConfSpec> {
        HashinselW::new(self, 0)
    }
    #[doc = "Bit 2 - Specifies the Destination block cipher of FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn dessel(&mut self) -> DesselW<ConfSpec> {
        DesselW::new(self, 2)
    }
    #[doc = "Bit 3 - If this bit is high, then the data read from the bus is byte-swapped\n\nin a word boundary. If this bit is low (default), then the data is\n\nhanded over to the FIFO without byte-swap. For little endian bus,\n\nthis bit should be 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn byteswap_brfifo(&mut self) -> ByteswapBrfifoW<ConfSpec> {
        ByteswapBrfifoW::new(self, 3)
    }
    #[doc = "Bit 4 - If this bit is high, then the data read from the bus is byte-swapped\n\nin a word boundary. If this bit is low (default), then the data is\n\nhanded over to the FIFO without byte-swap. For little endian bus,\n\nthis bit should be 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn byteswap_btfifo(&mut self) -> ByteswapBtfifoW<ConfSpec> {
        ByteswapBtfifoW::new(self, 4)
    }
    #[doc = "Bit 5 - If this bit is high, then the data read from the bus is byte-swapped\n\nin a word boundary. If this bit is low (default), then the data is\n\nhanded over to the FIFO without byte-swap. For little endian bus,\n\nthis bit should be 1'b1."]
    #[inline(always)]
    #[must_use]
    pub fn byteswap_hrfifo(&mut self) -> ByteswapHrfifoW<ConfSpec> {
        ByteswapHrfifoW::new(self, 5)
    }
    #[doc = "Bit 6 - Block Receive DMA Address Mode"]
    #[inline(always)]
    #[must_use]
    pub fn br_addr_mode(&mut self) -> BrAddrModeW<ConfSpec> {
        BrAddrModeW::new(self, 6)
    }
    #[doc = "Bit 7 - Block Transmit DMA Address Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bt_addr_mode(&mut self) -> BtAddrModeW<ConfSpec> {
        BtAddrModeW::new(self, 7)
    }
    #[doc = "Bit 8 - Hash Receive DMA Address Mode"]
    #[inline(always)]
    #[must_use]
    pub fn hr_addr_mode(&mut self) -> HrAddrModeW<ConfSpec> {
        HrAddrModeW::new(self, 8)
    }
}
#[doc = "Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfSpec;
impl crate::RegisterSpec for ConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for ConfSpec {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for ConfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for ConfSpec {
    const RESET_VALUE: u32 = 0;
}
