#[doc = "Register `EMMCCORE_HOSTCTRL1` reader"]
pub type R = crate::R<EmmccoreHostctrl1Spec>;
#[doc = "Register `EMMCCORE_HOSTCTRL1` writer"]
pub type W = crate::W<EmmccoreHostctrl1Spec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datatranswidth {
    #[doc = "1: 4 bit mode"]
    B1 = 1,
    #[doc = "0: 1 bit mode This bit selects the data width of the HC. The HD shall select it to match the data width of the SD card."]
    B0 = 0,
}
impl From<Datatranswidth> for bool {
    #[inline(always)]
    fn from(variant: Datatranswidth) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATATRANSWIDTH` reader - "]
pub type DatatranswidthR = crate::BitReader<Datatranswidth>;
impl DatatranswidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datatranswidth {
        match self.bits {
            true => Datatranswidth::B1,
            false => Datatranswidth::B0,
        }
    }
    #[doc = "4 bit mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Datatranswidth::B1
    }
    #[doc = "1 bit mode This bit selects the data width of the HC. The HD shall select it to match the data width of the SD card."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Datatranswidth::B0
    }
}
#[doc = "Field `DATATRANSWIDTH` writer - "]
pub type DatatranswidthW<'a, REG> = crate::BitWriter<'a, REG, Datatranswidth>;
impl<'a, REG> DatatranswidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "4 bit mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Datatranswidth::B1)
    }
    #[doc = "1 bit mode This bit selects the data width of the HC. The HD shall select it to match the data width of the SD card."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Datatranswidth::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Highspeedena {
    #[doc = "1: High Speed Mode"]
    B1 = 1,
    #[doc = "0: Normal Speed Mode This bit is optional. Before setting this bit, the HD shall check the High Speed Support in the capabilities register. If this bit is set to 0 (default), the HC outputs CMD line and DAT lines at the falling edge of the SD clock (up to 25 MHz/20MHz for eMMC). If thisbit is set to 1, the HC outputs CMD line and DAT lines at the rising edge of the SD clock (up to 50 MHz for SD/52MHz for eMMC)/ 208Mhz (for SD3.0). If Preset Value Enable in the Host Control 2 register is set to 1, Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitches. After setting this field, the Host Driver sets SD Clock Enable again"]
    B0 = 0,
}
impl From<Highspeedena> for bool {
    #[inline(always)]
    fn from(variant: Highspeedena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIGHSPEEDENA` reader - "]
pub type HighspeedenaR = crate::BitReader<Highspeedena>;
impl HighspeedenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Highspeedena {
        match self.bits {
            true => Highspeedena::B1,
            false => Highspeedena::B0,
        }
    }
    #[doc = "High Speed Mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Highspeedena::B1
    }
    #[doc = "Normal Speed Mode This bit is optional. Before setting this bit, the HD shall check the High Speed Support in the capabilities register. If this bit is set to 0 (default), the HC outputs CMD line and DAT lines at the falling edge of the SD clock (up to 25 MHz/20MHz for eMMC). If thisbit is set to 1, the HC outputs CMD line and DAT lines at the rising edge of the SD clock (up to 50 MHz for SD/52MHz for eMMC)/ 208Mhz (for SD3.0). If Preset Value Enable in the Host Control 2 register is set to 1, Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitches. After setting this field, the Host Driver sets SD Clock Enable again"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Highspeedena::B0
    }
}
#[doc = "Field `HIGHSPEEDENA` writer - "]
pub type HighspeedenaW<'a, REG> = crate::BitWriter<'a, REG, Highspeedena>;
impl<'a, REG> HighspeedenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High Speed Mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Highspeedena::B1)
    }
    #[doc = "Normal Speed Mode This bit is optional. Before setting this bit, the HD shall check the High Speed Support in the capabilities register. If this bit is set to 0 (default), the HC outputs CMD line and DAT lines at the falling edge of the SD clock (up to 25 MHz/20MHz for eMMC). If thisbit is set to 1, the HC outputs CMD line and DAT lines at the rising edge of the SD clock (up to 50 MHz for SD/52MHz for eMMC)/ 208Mhz (for SD3.0). If Preset Value Enable in the Host Control 2 register is set to 1, Host Driver needs to reset SD Clock Enable before changing this field to avoid generating clock glitches. After setting this field, the Host Driver sets SD Clock Enable again"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Highspeedena::B0)
    }
}
#[doc = "One of supported DMA modes can be selected. The host driver\n\nshall check support of DMA modes by referring the Capabilities\n\nregister.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmaselect {
    #[doc = "0: SDMA is selected"]
    D0 = 0,
    #[doc = "1: 32-bit Address ADMA1 is selected"]
    D1 = 1,
    #[doc = "2: 32-bit Address ADMA2 is selected"]
    D2 = 2,
    #[doc = "3: 64-bit Address ADMA2 is selected"]
    D3 = 3,
}
impl From<Dmaselect> for u8 {
    #[inline(always)]
    fn from(variant: Dmaselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmaselect {
    type Ux = u8;
}
#[doc = "Field `DMASELECT` reader - One of supported DMA modes can be selected. The host driver\n\nshall check support of DMA modes by referring the Capabilities\n\nregister."]
pub type DmaselectR = crate::FieldReader<Dmaselect>;
impl DmaselectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaselect {
        match self.bits {
            0 => Dmaselect::D0,
            1 => Dmaselect::D1,
            2 => Dmaselect::D2,
            3 => Dmaselect::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "SDMA is selected"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Dmaselect::D0
    }
    #[doc = "32-bit Address ADMA1 is selected"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Dmaselect::D1
    }
    #[doc = "32-bit Address ADMA2 is selected"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Dmaselect::D2
    }
    #[doc = "64-bit Address ADMA2 is selected"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Dmaselect::D3
    }
}
#[doc = "Field `DMASELECT` writer - One of supported DMA modes can be selected. The host driver\n\nshall check support of DMA modes by referring the Capabilities\n\nregister."]
pub type DmaselectW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Dmaselect>;
impl<'a, REG> DmaselectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDMA is selected"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::D0)
    }
    #[doc = "32-bit Address ADMA1 is selected"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::D1)
    }
    #[doc = "32-bit Address ADMA2 is selected"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::D2)
    }
    #[doc = "64-bit Address ADMA2 is selected"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaselect::D3)
    }
}
#[doc = "This bit controls 8-bit bus width mode for embedded device.\n\nSupport of this function is indicated in 8-bit Support for\n\nEmbedded Device in the Capabilities register. If a device supports\n\n8-bit bus mode, this bit may be set to 1. If this bit is 0, bus width\n\nis controlled by Data Transfer Width in the Host Control 1\n\nregister.This bit is not effective when multiple devices are\n\ninstalled on a bus slot (Slot Type is set to 10b in the Capabilities\n\nregister). In this case, each device bus width is controlled by Bus\n\nWidth Preset field in the Shared Bus register.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extendeddatatranswidth {
    #[doc = "1: 8-bit Bus Width"]
    B1 = 1,
    #[doc = "0: Bus Width is Selected by Data Transfer Width"]
    B0 = 0,
}
impl From<Extendeddatatranswidth> for bool {
    #[inline(always)]
    fn from(variant: Extendeddatatranswidth) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTENDEDDATATRANSWIDTH` reader - This bit controls 8-bit bus width mode for embedded device.\n\nSupport of this function is indicated in 8-bit Support for\n\nEmbedded Device in the Capabilities register. If a device supports\n\n8-bit bus mode, this bit may be set to 1. If this bit is 0, bus width\n\nis controlled by Data Transfer Width in the Host Control 1\n\nregister.This bit is not effective when multiple devices are\n\ninstalled on a bus slot (Slot Type is set to 10b in the Capabilities\n\nregister). In this case, each device bus width is controlled by Bus\n\nWidth Preset field in the Shared Bus register."]
pub type ExtendeddatatranswidthR = crate::BitReader<Extendeddatatranswidth>;
impl ExtendeddatatranswidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extendeddatatranswidth {
        match self.bits {
            true => Extendeddatatranswidth::B1,
            false => Extendeddatatranswidth::B0,
        }
    }
    #[doc = "8-bit Bus Width"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Extendeddatatranswidth::B1
    }
    #[doc = "Bus Width is Selected by Data Transfer Width"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Extendeddatatranswidth::B0
    }
}
#[doc = "Field `EXTENDEDDATATRANSWIDTH` writer - This bit controls 8-bit bus width mode for embedded device.\n\nSupport of this function is indicated in 8-bit Support for\n\nEmbedded Device in the Capabilities register. If a device supports\n\n8-bit bus mode, this bit may be set to 1. If this bit is 0, bus width\n\nis controlled by Data Transfer Width in the Host Control 1\n\nregister.This bit is not effective when multiple devices are\n\ninstalled on a bus slot (Slot Type is set to 10b in the Capabilities\n\nregister). In this case, each device bus width is controlled by Bus\n\nWidth Preset field in the Shared Bus register."]
pub type ExtendeddatatranswidthW<'a, REG> = crate::BitWriter<'a, REG, Extendeddatatranswidth>;
impl<'a, REG> ExtendeddatatranswidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit Bus Width"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Extendeddatatranswidth::B1)
    }
    #[doc = "Bus Width is Selected by Data Transfer Width"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Extendeddatatranswidth::B0)
    }
}
#[doc = "This bit is enabled while the Card Detect Signal Selection is set to\n\n1 and it indicates card inserted or not.\n\nGenerates (card ins or card removal) interrupt when the normal\n\nint sts enable bit is set.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Carddettestlevel {
    #[doc = "1: Card Inserted"]
    B1 = 1,
    #[doc = "0: No Card"]
    B0 = 0,
}
impl From<Carddettestlevel> for bool {
    #[inline(always)]
    fn from(variant: Carddettestlevel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDDETTESTLEVEL` reader - This bit is enabled while the Card Detect Signal Selection is set to\n\n1 and it indicates card inserted or not.\n\nGenerates (card ins or card removal) interrupt when the normal\n\nint sts enable bit is set."]
pub type CarddettestlevelR = crate::BitReader<Carddettestlevel>;
impl CarddettestlevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Carddettestlevel {
        match self.bits {
            true => Carddettestlevel::B1,
            false => Carddettestlevel::B0,
        }
    }
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Carddettestlevel::B1
    }
    #[doc = "No Card"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Carddettestlevel::B0
    }
}
#[doc = "Field `CARDDETTESTLEVEL` writer - This bit is enabled while the Card Detect Signal Selection is set to\n\n1 and it indicates card inserted or not.\n\nGenerates (card ins or card removal) interrupt when the normal\n\nint sts enable bit is set."]
pub type CarddettestlevelW<'a, REG> = crate::BitWriter<'a, REG, Carddettestlevel>;
impl<'a, REG> CarddettestlevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Card Inserted"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Carddettestlevel::B1)
    }
    #[doc = "No Card"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Carddettestlevel::B0)
    }
}
#[doc = "This bit selects source for card detection.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Carddetsginaldet {
    #[doc = "1: The card detect test level is selected"]
    B1 = 1,
    #[doc = "0: SDCD# is selected (for normal use)"]
    B0 = 0,
}
impl From<Carddetsginaldet> for bool {
    #[inline(always)]
    fn from(variant: Carddetsginaldet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CARDDETSGINALDET` reader - This bit selects source for card detection."]
pub type CarddetsginaldetR = crate::BitReader<Carddetsginaldet>;
impl CarddetsginaldetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Carddetsginaldet {
        match self.bits {
            true => Carddetsginaldet::B1,
            false => Carddetsginaldet::B0,
        }
    }
    #[doc = "The card detect test level is selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Carddetsginaldet::B1
    }
    #[doc = "SDCD# is selected (for normal use)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Carddetsginaldet::B0
    }
}
#[doc = "Field `CARDDETSGINALDET` writer - This bit selects source for card detection."]
pub type CarddetsginaldetW<'a, REG> = crate::BitWriter<'a, REG, Carddetsginaldet>;
impl<'a, REG> CarddetsginaldetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The card detect test level is selected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Carddetsginaldet::B1)
    }
    #[doc = "SDCD# is selected (for normal use)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Carddetsginaldet::B0)
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn datatranswidth(&self) -> DatatranswidthR {
        DatatranswidthR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn highspeedena(&self) -> HighspeedenaR {
        HighspeedenaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - One of supported DMA modes can be selected. The host driver\n\nshall check support of DMA modes by referring the Capabilities\n\nregister."]
    #[inline(always)]
    pub fn dmaselect(&self) -> DmaselectR {
        DmaselectR::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 5 - This bit controls 8-bit bus width mode for embedded device.\n\nSupport of this function is indicated in 8-bit Support for\n\nEmbedded Device in the Capabilities register. If a device supports\n\n8-bit bus mode, this bit may be set to 1. If this bit is 0, bus width\n\nis controlled by Data Transfer Width in the Host Control 1\n\nregister.This bit is not effective when multiple devices are\n\ninstalled on a bus slot (Slot Type is set to 10b in the Capabilities\n\nregister). In this case, each device bus width is controlled by Bus\n\nWidth Preset field in the Shared Bus register."]
    #[inline(always)]
    pub fn extendeddatatranswidth(&self) -> ExtendeddatatranswidthR {
        ExtendeddatatranswidthR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is enabled while the Card Detect Signal Selection is set to\n\n1 and it indicates card inserted or not.\n\nGenerates (card ins or card removal) interrupt when the normal\n\nint sts enable bit is set."]
    #[inline(always)]
    pub fn carddettestlevel(&self) -> CarddettestlevelR {
        CarddettestlevelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit selects source for card detection."]
    #[inline(always)]
    pub fn carddetsginaldet(&self) -> CarddetsginaldetR {
        CarddetsginaldetR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn datatranswidth(&mut self) -> DatatranswidthW<EmmccoreHostctrl1Spec> {
        DatatranswidthW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn highspeedena(&mut self) -> HighspeedenaW<EmmccoreHostctrl1Spec> {
        HighspeedenaW::new(self, 2)
    }
    #[doc = "Bits 3:4 - One of supported DMA modes can be selected. The host driver\n\nshall check support of DMA modes by referring the Capabilities\n\nregister."]
    #[inline(always)]
    #[must_use]
    pub fn dmaselect(&mut self) -> DmaselectW<EmmccoreHostctrl1Spec> {
        DmaselectW::new(self, 3)
    }
    #[doc = "Bit 5 - This bit controls 8-bit bus width mode for embedded device.\n\nSupport of this function is indicated in 8-bit Support for\n\nEmbedded Device in the Capabilities register. If a device supports\n\n8-bit bus mode, this bit may be set to 1. If this bit is 0, bus width\n\nis controlled by Data Transfer Width in the Host Control 1\n\nregister.This bit is not effective when multiple devices are\n\ninstalled on a bus slot (Slot Type is set to 10b in the Capabilities\n\nregister). In this case, each device bus width is controlled by Bus\n\nWidth Preset field in the Shared Bus register."]
    #[inline(always)]
    #[must_use]
    pub fn extendeddatatranswidth(&mut self) -> ExtendeddatatranswidthW<EmmccoreHostctrl1Spec> {
        ExtendeddatatranswidthW::new(self, 5)
    }
    #[doc = "Bit 6 - This bit is enabled while the Card Detect Signal Selection is set to\n\n1 and it indicates card inserted or not.\n\nGenerates (card ins or card removal) interrupt when the normal\n\nint sts enable bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn carddettestlevel(&mut self) -> CarddettestlevelW<EmmccoreHostctrl1Spec> {
        CarddettestlevelW::new(self, 6)
    }
    #[doc = "Bit 7 - This bit selects source for card detection."]
    #[inline(always)]
    #[must_use]
    pub fn carddetsginaldet(&mut self) -> CarddetsginaldetW<EmmccoreHostctrl1Spec> {
        CarddetsginaldetW::new(self, 7)
    }
}
#[doc = "Host control 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_hostctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_hostctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreHostctrl1Spec;
impl crate::RegisterSpec for EmmccoreHostctrl1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`emmccore_hostctrl1::R`](R) reader structure"]
impl crate::Readable for EmmccoreHostctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_hostctrl1::W`](W) writer structure"]
impl crate::Writable for EmmccoreHostctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_HOSTCTRL1 to value 0"]
impl crate::Resettable for EmmccoreHostctrl1Spec {
    const RESET_VALUE: u8 = 0;
}
