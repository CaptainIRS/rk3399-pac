#[doc = "Register `EMMCCORE_CAP` reader"]
pub type R = crate::R<EmmccoreCapSpec>;
#[doc = "Register `EMMCCORE_CAP` writer"]
pub type W = crate::W<EmmccoreCapSpec>;
#[doc = "Field `TIMEOUTCLOCKFREQUENCY` reader - This bit shows the base clock frequency used to detect Data\n\nTimeout Error.\n\nNot 0: 1Khz to 63Khz or 1Mhz to 63Mhz\n\n0: Get Information viaanother method"]
pub type TimeoutclockfrequencyR = crate::FieldReader;
#[doc = "This bit shows the unit of base clock frequency used to detect\n\nData Timeout Error.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timeoutclockunit {
    #[doc = "0: Khz"]
    B0 = 0,
    #[doc = "1: Mhz"]
    B1 = 1,
}
impl From<Timeoutclockunit> for bool {
    #[inline(always)]
    fn from(variant: Timeoutclockunit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUTCLOCKUNIT` reader - This bit shows the unit of base clock frequency used to detect\n\nData Timeout Error."]
pub type TimeoutclockunitR = crate::BitReader<Timeoutclockunit>;
impl TimeoutclockunitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timeoutclockunit {
        match self.bits {
            false => Timeoutclockunit::B0,
            true => Timeoutclockunit::B1,
        }
    }
    #[doc = "Khz"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Timeoutclockunit::B0
    }
    #[doc = "Mhz"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Timeoutclockunit::B1
    }
}
#[doc = "Field `BASECLOCKFREQSDCLOCK` reader - Base Clock Frequency for SD Clock\n\n(1) 6-bit Base Clock Frequency\n\nThis mode is supported by the Host Controller Version 1.00 and\n\n2.00. Upper 2-bit is not effective and always 0. Unit values are\n\n1MHz. The supported clock range is 10MHz to 63MHz.\n\n8'h00: Get information via another method\n\n8'h01: 1MHz\n\n8'h02: 2MHz\n\n......\n\n8'h3f: 63MHz\n\nothers: not supported\n\n(2) 8-bit Base Clock Frequency\n\nThis mode is supported by the Host Controller Version 3.00. Unit\n\nvalues are 1MHz. The supported clock range is 10MHz to 255MHz.\n\n8'h00: Get information via another method\n\n8'h01: 1MHz\n\n8'h02: 2MHz\n\n......\n\n8'hff: 255MHz\n\nIf the real frequency is 16.5MHz, the lager value shall be set\n\n0001 0001b (17MHz) because the Host Driver use this value to\n\ncalculate the clock divider value (Refer to the SDCLK Frequency\n\nSelect in the Clock Control register.) and it shall not exceed upper\n\nlimit of the SD Clock frequency. If these bits are all 0, the Host\n\nSystem has to get information via another method."]
pub type BaseclockfreqsdclockR = crate::FieldReader;
#[doc = "Max Block Length\n\nThis value indicates the maximum block size that the HD can\n\nread and write to the buffer in the HC.\n\nThe buffer shall transfer this block size without wait cycles. Three\n\nsizes can be defined as indicated below.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maxblocklength {
    #[doc = "0: 512 byte"]
    D0 = 0,
    #[doc = "1: 1024 byte"]
    D1 = 1,
    #[doc = "2: 2048 byte"]
    D2 = 2,
    #[doc = "3: 4096 byte"]
    D3 = 3,
}
impl From<Maxblocklength> for u8 {
    #[inline(always)]
    fn from(variant: Maxblocklength) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maxblocklength {
    type Ux = u8;
}
#[doc = "Field `MAXBLOCKLENGTH` reader - Max Block Length\n\nThis value indicates the maximum block size that the HD can\n\nread and write to the buffer in the HC.\n\nThe buffer shall transfer this block size without wait cycles. Three\n\nsizes can be defined as indicated below."]
pub type MaxblocklengthR = crate::FieldReader<Maxblocklength>;
impl MaxblocklengthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Maxblocklength {
        match self.bits {
            0 => Maxblocklength::D0,
            1 => Maxblocklength::D1,
            2 => Maxblocklength::D2,
            3 => Maxblocklength::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "512 byte"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Maxblocklength::D0
    }
    #[doc = "1024 byte"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Maxblocklength::D1
    }
    #[doc = "2048 byte"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Maxblocklength::D2
    }
    #[doc = "4096 byte"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Maxblocklength::D3
    }
}
#[doc = "Extended Media Bus Support\n\nThis bit indicates whether the Host Controller is capable of using\n\n8-bit bus width mode. This bit is not effective when Slot Type is\n\nset to 10b. In this case, refer to Bus Width Preset in the Shared\n\nBus resister.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extendedmediabussupport {
    #[doc = "1: Extended Media Bus Supported"]
    B1 = 1,
    #[doc = "0: Extended Media Bus not Supported"]
    B0 = 0,
}
impl From<Extendedmediabussupport> for bool {
    #[inline(always)]
    fn from(variant: Extendedmediabussupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTENDEDMEDIABUSSUPPORT` reader - Extended Media Bus Support\n\nThis bit indicates whether the Host Controller is capable of using\n\n8-bit bus width mode. This bit is not effective when Slot Type is\n\nset to 10b. In this case, refer to Bus Width Preset in the Shared\n\nBus resister."]
pub type ExtendedmediabussupportR = crate::BitReader<Extendedmediabussupport>;
impl ExtendedmediabussupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extendedmediabussupport {
        match self.bits {
            true => Extendedmediabussupport::B1,
            false => Extendedmediabussupport::B0,
        }
    }
    #[doc = "Extended Media Bus Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Extendedmediabussupport::B1
    }
    #[doc = "Extended Media Bus not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Extendedmediabussupport::B0
    }
}
#[doc = "ADMA2 Support\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adma2support {
    #[doc = "1: ADMA2 support"]
    B1 = 1,
    #[doc = "0: ADMA2 not support"]
    B0 = 0,
}
impl From<Adma2support> for bool {
    #[inline(always)]
    fn from(variant: Adma2support) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMA2SUPPORT` reader - ADMA2 Support"]
pub type Adma2supportR = crate::BitReader<Adma2support>;
impl Adma2supportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adma2support {
        match self.bits {
            true => Adma2support::B1,
            false => Adma2support::B0,
        }
    }
    #[doc = "ADMA2 support"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Adma2support::B1
    }
    #[doc = "ADMA2 not support"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Adma2support::B0
    }
}
#[doc = "High Speed Support\n\nThis bit indicates whether the HC and the Host System support\n\nHigh Speed mode and they can supply SD Clock frequency from\n\n25Mhz to 50 Mhz (for SD)/ 20MHz to 52MHz (for eMMC).\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Highspeedsupport {
    #[doc = "0: High Speed Not Supported"]
    B0 = 0,
    #[doc = "1: High Speed Supported"]
    B1 = 1,
}
impl From<Highspeedsupport> for bool {
    #[inline(always)]
    fn from(variant: Highspeedsupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIGHSPEEDSUPPORT` reader - High Speed Support\n\nThis bit indicates whether the HC and the Host System support\n\nHigh Speed mode and they can supply SD Clock frequency from\n\n25Mhz to 50 Mhz (for SD)/ 20MHz to 52MHz (for eMMC)."]
pub type HighspeedsupportR = crate::BitReader<Highspeedsupport>;
impl HighspeedsupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Highspeedsupport {
        match self.bits {
            false => Highspeedsupport::B0,
            true => Highspeedsupport::B1,
        }
    }
    #[doc = "High Speed Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Highspeedsupport::B0
    }
    #[doc = "High Speed Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Highspeedsupport::B1
    }
}
#[doc = "This bit indicates whether the HC is capable of using DMA to\n\ntransfer data between system memory and the HC directly.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdmasupport {
    #[doc = "0: SDMA Not Supported"]
    B0 = 0,
    #[doc = "1: SDMA Supported."]
    B1 = 1,
}
impl From<Sdmasupport> for bool {
    #[inline(always)]
    fn from(variant: Sdmasupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDMASUPPORT` reader - This bit indicates whether the HC is capable of using DMA to\n\ntransfer data between system memory and the HC directly."]
pub type SdmasupportR = crate::BitReader<Sdmasupport>;
impl SdmasupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdmasupport {
        match self.bits {
            false => Sdmasupport::B0,
            true => Sdmasupport::B1,
        }
    }
    #[doc = "SDMA Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sdmasupport::B0
    }
    #[doc = "SDMA Supported."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sdmasupport::B1
    }
}
#[doc = "Suspend / Resume Support\n\nThis bit indicates whether the HC supports Suspend / Resume\n\nfunctionality. If this bit is 0, the Suspend and Resume mechanism\n\nare not supported and the HD shall not issue either Suspend /\n\nResume commands.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suspendresumesupport {
    #[doc = "0: Not Supported"]
    B0 = 0,
    #[doc = "1: Supported"]
    B1 = 1,
}
impl From<Suspendresumesupport> for bool {
    #[inline(always)]
    fn from(variant: Suspendresumesupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPENDRESUMESUPPORT` reader - Suspend / Resume Support\n\nThis bit indicates whether the HC supports Suspend / Resume\n\nfunctionality. If this bit is 0, the Suspend and Resume mechanism\n\nare not supported and the HD shall not issue either Suspend /\n\nResume commands."]
pub type SuspendresumesupportR = crate::BitReader<Suspendresumesupport>;
impl SuspendresumesupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suspendresumesupport {
        match self.bits {
            false => Suspendresumesupport::B0,
            true => Suspendresumesupport::B1,
        }
    }
    #[doc = "Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Suspendresumesupport::B0
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Suspendresumesupport::B1
    }
}
#[doc = "Field `SUSPENDRESUMESUPPORT` writer - Suspend / Resume Support\n\nThis bit indicates whether the HC supports Suspend / Resume\n\nfunctionality. If this bit is 0, the Suspend and Resume mechanism\n\nare not supported and the HD shall not issue either Suspend /\n\nResume commands."]
pub type SuspendresumesupportW<'a, REG> = crate::BitWriter<'a, REG, Suspendresumesupport>;
impl<'a, REG> SuspendresumesupportW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Supported"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Suspendresumesupport::B0)
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Suspendresumesupport::B1)
    }
}
#[doc = "Voltage Support 3.3 V\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Voltage33vsupport {
    #[doc = "0: Not Supported"]
    B0 = 0,
    #[doc = "1: Supported"]
    B1 = 1,
}
impl From<Voltage33vsupport> for bool {
    #[inline(always)]
    fn from(variant: Voltage33vsupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOLTAGE33VSUPPORT` reader - Voltage Support 3.3 V"]
pub type Voltage33vsupportR = crate::BitReader<Voltage33vsupport>;
impl Voltage33vsupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Voltage33vsupport {
        match self.bits {
            false => Voltage33vsupport::B0,
            true => Voltage33vsupport::B1,
        }
    }
    #[doc = "Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Voltage33vsupport::B0
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Voltage33vsupport::B1
    }
}
#[doc = "Field `VOLTAGE33VSUPPORT` writer - Voltage Support 3.3 V"]
pub type Voltage33vsupportW<'a, REG> = crate::BitWriter<'a, REG, Voltage33vsupport>;
impl<'a, REG> Voltage33vsupportW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not Supported"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Voltage33vsupport::B0)
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Voltage33vsupport::B1)
    }
}
#[doc = "Voltage Support 3.0 V\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Voltage30vsupport {
    #[doc = "0: Not Supported"]
    B0 = 0,
    #[doc = "1: Supported"]
    B1 = 1,
}
impl From<Voltage30vsupport> for bool {
    #[inline(always)]
    fn from(variant: Voltage30vsupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOLTAGE30VSUPPORT` reader - Voltage Support 3.0 V"]
pub type Voltage30vsupportR = crate::BitReader<Voltage30vsupport>;
impl Voltage30vsupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Voltage30vsupport {
        match self.bits {
            false => Voltage30vsupport::B0,
            true => Voltage30vsupport::B1,
        }
    }
    #[doc = "Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Voltage30vsupport::B0
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Voltage30vsupport::B1
    }
}
#[doc = "Voltage Support 1.8 V\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Voltage18vsupport {
    #[doc = "0: Not Supported"]
    B0 = 0,
    #[doc = "1: Supported"]
    B1 = 1,
}
impl From<Voltage18vsupport> for bool {
    #[inline(always)]
    fn from(variant: Voltage18vsupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VOLTAGE18VSUPPORT` reader - Voltage Support 1.8 V"]
pub type Voltage18vsupportR = crate::BitReader<Voltage18vsupport>;
impl Voltage18vsupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Voltage18vsupport {
        match self.bits {
            false => Voltage18vsupport::B0,
            true => Voltage18vsupport::B1,
        }
    }
    #[doc = "Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Voltage18vsupport::B0
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Voltage18vsupport::B1
    }
}
#[doc = "64-bit System Bus Support\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Systembussupport {
    #[doc = "1: Supports 64 bit system address"]
    B1 = 1,
    #[doc = "0: Does not support 64 bit system address"]
    B0 = 0,
}
impl From<Systembussupport> for bool {
    #[inline(always)]
    fn from(variant: Systembussupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTEMBUSSUPPORT` reader - 64-bit System Bus Support"]
pub type SystembussupportR = crate::BitReader<Systembussupport>;
impl SystembussupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Systembussupport {
        match self.bits {
            true => Systembussupport::B1,
            false => Systembussupport::B0,
        }
    }
    #[doc = "Supports 64 bit system address"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Systembussupport::B1
    }
    #[doc = "Does not support 64 bit system address"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Systembussupport::B0
    }
}
#[doc = "Asynchronous Interrupt Support\n\nRefer to SDIO Specification Version 3.00 about asynchronous\n\ninterrupt.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Asynintsupport {
    #[doc = "1: Asynchronous Interrupt Supported"]
    B1 = 1,
    #[doc = "0: Asynchronous Interrupt Not Supported"]
    B0 = 0,
}
impl From<Asynintsupport> for bool {
    #[inline(always)]
    fn from(variant: Asynintsupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASYNINTSUPPORT` reader - Asynchronous Interrupt Support\n\nRefer to SDIO Specification Version 3.00 about asynchronous\n\ninterrupt."]
pub type AsynintsupportR = crate::BitReader<Asynintsupport>;
impl AsynintsupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Asynintsupport {
        match self.bits {
            true => Asynintsupport::B1,
            false => Asynintsupport::B0,
        }
    }
    #[doc = "Asynchronous Interrupt Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Asynintsupport::B1
    }
    #[doc = "Asynchronous Interrupt Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Asynintsupport::B0
    }
}
#[doc = "This field indicates usage of a slot by a specific Host System. (A\n\nhost controller register set is defined per slot.) Embedded slot for\n\none device (01b) means that only one non-removable device is\n\nconnected to a SD bus slot. Shared Bus Slot (10b) can be set if\n\nHost Controller supports Shared Bus Control register.\n\nThe Standard Host Driver controls only a removable card or one\n\nembedded device is onnected to a SD bus slot. If a slot is\n\nconfigured for shared bus (10b), the Standard Host Driver does\n\nnot control embedded devices connected to a shared bus. Shared\n\nbus slot is controlled by a specific host driver developed by a Host\n\nSystem.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Slottype {
    #[doc = "0: Removable Card Slot"]
    D0 = 0,
    #[doc = "1: Embedded Slot for One Device"]
    D1 = 1,
    #[doc = "2: Shared Bus Slot"]
    D2 = 2,
    #[doc = "3: Reserved"]
    D3 = 3,
}
impl From<Slottype> for u8 {
    #[inline(always)]
    fn from(variant: Slottype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Slottype {
    type Ux = u8;
}
#[doc = "Field `SLOTTYPE` reader - This field indicates usage of a slot by a specific Host System. (A\n\nhost controller register set is defined per slot.) Embedded slot for\n\none device (01b) means that only one non-removable device is\n\nconnected to a SD bus slot. Shared Bus Slot (10b) can be set if\n\nHost Controller supports Shared Bus Control register.\n\nThe Standard Host Driver controls only a removable card or one\n\nembedded device is onnected to a SD bus slot. If a slot is\n\nconfigured for shared bus (10b), the Standard Host Driver does\n\nnot control embedded devices connected to a shared bus. Shared\n\nbus slot is controlled by a specific host driver developed by a Host\n\nSystem."]
pub type SlottypeR = crate::FieldReader<Slottype>;
impl SlottypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slottype {
        match self.bits {
            0 => Slottype::D0,
            1 => Slottype::D1,
            2 => Slottype::D2,
            3 => Slottype::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "Removable Card Slot"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Slottype::D0
    }
    #[doc = "Embedded Slot for One Device"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Slottype::D1
    }
    #[doc = "Shared Bus Slot"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Slottype::D2
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Slottype::D3
    }
}
#[doc = "SDR50 Support\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdr50support {
    #[doc = "1: SDR50 is Supported"]
    B1 = 1,
    #[doc = "0: SDR50 is Not Supported"]
    B0 = 0,
}
impl From<Sdr50support> for bool {
    #[inline(always)]
    fn from(variant: Sdr50support) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDR50SUPPORT` reader - SDR50 Support"]
pub type Sdr50supportR = crate::BitReader<Sdr50support>;
impl Sdr50supportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr50support {
        match self.bits {
            true => Sdr50support::B1,
            false => Sdr50support::B0,
        }
    }
    #[doc = "SDR50 is Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sdr50support::B1
    }
    #[doc = "SDR50 is Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sdr50support::B0
    }
}
#[doc = "SDR104 Support.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sdr104support {
    #[doc = "1: SDR104 is Supported"]
    B1 = 1,
    #[doc = "0: SDR104 is Not Supported"]
    B0 = 0,
}
impl From<Sdr104support> for bool {
    #[inline(always)]
    fn from(variant: Sdr104support) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDR104SUPPORT` reader - SDR104 Support."]
pub type Sdr104supportR = crate::BitReader<Sdr104support>;
impl Sdr104supportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sdr104support {
        match self.bits {
            true => Sdr104support::B1,
            false => Sdr104support::B0,
        }
    }
    #[doc = "SDR104 is Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sdr104support::B1
    }
    #[doc = "SDR104 is Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sdr104support::B0
    }
}
#[doc = "DDR50 Support\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ddr50support {
    #[doc = "1: DDR50 is Supported"]
    B1 = 1,
    #[doc = "0: DDR50 is Not Supported"]
    B0 = 0,
}
impl From<Ddr50support> for bool {
    #[inline(always)]
    fn from(variant: Ddr50support) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDR50SUPPORT` reader - DDR50 Support"]
pub type Ddr50supportR = crate::BitReader<Ddr50support>;
impl Ddr50supportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ddr50support {
        match self.bits {
            true => Ddr50support::B1,
            false => Ddr50support::B0,
        }
    }
    #[doc = "DDR50 is Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ddr50support::B1
    }
    #[doc = "DDR50 is Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ddr50support::B0
    }
}
#[doc = "This bit indicates support of Driver Type A for 1.8 Signaling.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drivertypeasupport {
    #[doc = "1: Driver Type A is Supported"]
    B1 = 1,
    #[doc = "0: Driver Type A is Not Supported"]
    B0 = 0,
}
impl From<Drivertypeasupport> for bool {
    #[inline(always)]
    fn from(variant: Drivertypeasupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRIVERTYPEASUPPORT` reader - This bit indicates support of Driver Type A for 1.8 Signaling."]
pub type DrivertypeasupportR = crate::BitReader<Drivertypeasupport>;
impl DrivertypeasupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drivertypeasupport {
        match self.bits {
            true => Drivertypeasupport::B1,
            false => Drivertypeasupport::B0,
        }
    }
    #[doc = "Driver Type A is Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Drivertypeasupport::B1
    }
    #[doc = "Driver Type A is Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Drivertypeasupport::B0
    }
}
#[doc = "This bit indicates support of Driver Type C for 1.8 Signaling.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drivertypecsupport {
    #[doc = "1: Driver Type C is Supported"]
    B1 = 1,
    #[doc = "0: Driver Type C is Not Supported"]
    B0 = 0,
}
impl From<Drivertypecsupport> for bool {
    #[inline(always)]
    fn from(variant: Drivertypecsupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRIVERTYPECSUPPORT` reader - This bit indicates support of Driver Type C for 1.8 Signaling."]
pub type DrivertypecsupportR = crate::BitReader<Drivertypecsupport>;
impl DrivertypecsupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drivertypecsupport {
        match self.bits {
            true => Drivertypecsupport::B1,
            false => Drivertypecsupport::B0,
        }
    }
    #[doc = "Driver Type C is Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Drivertypecsupport::B1
    }
    #[doc = "Driver Type C is Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Drivertypecsupport::B0
    }
}
#[doc = "Field `DRIVERTYPECSUPPORT` writer - This bit indicates support of Driver Type C for 1.8 Signaling."]
pub type DrivertypecsupportW<'a, REG> = crate::BitWriter<'a, REG, Drivertypecsupport>;
impl<'a, REG> DrivertypecsupportW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Driver Type C is Supported"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Drivertypecsupport::B1)
    }
    #[doc = "Driver Type C is Not Supported"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Drivertypecsupport::B0)
    }
}
#[doc = "This bit indicates support of Driver Type D for 1.8 Signaling.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drivertypedsupport {
    #[doc = "1: Driver Type D is Supported"]
    B1 = 1,
    #[doc = "0: Driver Type D is Not Supported"]
    B0 = 0,
}
impl From<Drivertypedsupport> for bool {
    #[inline(always)]
    fn from(variant: Drivertypedsupport) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRIVERTYPEDSUPPORT` reader - This bit indicates support of Driver Type D for 1.8 Signaling."]
pub type DrivertypedsupportR = crate::BitReader<Drivertypedsupport>;
impl DrivertypedsupportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drivertypedsupport {
        match self.bits {
            true => Drivertypedsupport::B1,
            false => Drivertypedsupport::B0,
        }
    }
    #[doc = "Driver Type D is Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Drivertypedsupport::B1
    }
    #[doc = "Driver Type D is Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Drivertypedsupport::B0
    }
}
#[doc = "Driver Type 4 Support\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drivertype4support {
    #[doc = "1: Driver Type 4 is Supported"]
    B1 = 1,
    #[doc = "0: Driver Type 4 is Not Supported"]
    B0 = 0,
}
impl From<Drivertype4support> for bool {
    #[inline(always)]
    fn from(variant: Drivertype4support) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRIVERTYPE4SUPPORT` reader - Driver Type 4 Support"]
pub type Drivertype4supportR = crate::BitReader<Drivertype4support>;
impl Drivertype4supportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drivertype4support {
        match self.bits {
            true => Drivertype4support::B1,
            false => Drivertype4support::B0,
        }
    }
    #[doc = "Driver Type 4 is Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Drivertype4support::B1
    }
    #[doc = "Driver Type 4 is Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Drivertype4support::B0
    }
}
#[doc = "Field `TIMERCOUNTFORRETUNING` reader - Timer count for ReTuning\n\nThis field indicates an initial value of the Re-Tuning Timer for Re-\n\nTuning Mode 1 to 3.\n\n4'h0 - Get information via other source\n\n4'h1 = 1 seconds\n\n4'h2 = 2 seconds\n\n4'h3 = 4 seconds\n\n4'h4 = 8 seconds\n\n........\n\n4'hB = 1024 seconds\n\n4'hF - Ch = Reserved"]
pub type TimercountforretuningR = crate::FieldReader;
#[doc = "Use Tuning for SDR50\n\nIf this bit is set to 1, this Host Controller requires tuning to\n\noperate SDR50. (Tuning is always required to operate SDR104.)\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usetuningforsdr50 {
    #[doc = "1: SDR50 requires tuning"]
    B1 = 1,
    #[doc = "0: SDR50 does not require tuning"]
    B0 = 0,
}
impl From<Usetuningforsdr50> for bool {
    #[inline(always)]
    fn from(variant: Usetuningforsdr50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USETUNINGFORSDR50` reader - Use Tuning for SDR50\n\nIf this bit is set to 1, this Host Controller requires tuning to\n\noperate SDR50. (Tuning is always required to operate SDR104.)"]
pub type Usetuningforsdr50R = crate::BitReader<Usetuningforsdr50>;
impl Usetuningforsdr50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usetuningforsdr50 {
        match self.bits {
            true => Usetuningforsdr50::B1,
            false => Usetuningforsdr50::B0,
        }
    }
    #[doc = "SDR50 requires tuning"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usetuningforsdr50::B1
    }
    #[doc = "SDR50 does not require tuning"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usetuningforsdr50::B0
    }
}
#[doc = "Re-tuning modes\n\nThis field defines the re-tuning capability of a Host Controller and\n\nhow to manage the data transfer length and a Re-Tuning Timer\n\nby the Host Driver\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Retuningmode {
    #[doc = "0: Mode1"]
    D0 = 0,
    #[doc = "1: Mode2"]
    D1 = 1,
    #[doc = "2: Mode3"]
    D2 = 2,
    #[doc = "3: Reserved There are two re-tuning timings: Re-Tuning Request and expiration of a Re-Tuning Timer. By receiving either timing, the Host Driver executes the re-tuning procedure just before a next command issue"]
    D3 = 3,
}
impl From<Retuningmode> for u8 {
    #[inline(always)]
    fn from(variant: Retuningmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Retuningmode {
    type Ux = u8;
}
#[doc = "Field `RETUNINGMODE` reader - Re-tuning modes\n\nThis field defines the re-tuning capability of a Host Controller and\n\nhow to manage the data transfer length and a Re-Tuning Timer\n\nby the Host Driver"]
pub type RetuningmodeR = crate::FieldReader<Retuningmode>;
impl RetuningmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Retuningmode {
        match self.bits {
            0 => Retuningmode::D0,
            1 => Retuningmode::D1,
            2 => Retuningmode::D2,
            3 => Retuningmode::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "Mode1"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Retuningmode::D0
    }
    #[doc = "Mode2"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Retuningmode::D1
    }
    #[doc = "Mode3"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Retuningmode::D2
    }
    #[doc = "Reserved There are two re-tuning timings: Re-Tuning Request and expiration of a Re-Tuning Timer. By receiving either timing, the Host Driver executes the re-tuning procedure just before a next command issue"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Retuningmode::D3
    }
}
#[doc = "This field indicates clock multiplier value of programmable clock\n\ngenerator. Refer to Clock Control register. Setting 00h means\n\nthat Host Controller does not support programmable clock\n\ngenerator.\n\nValue on reset: 16"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clockmultiplier {
    #[doc = "255: Clock Multiplier M = 256 ...."]
    HFf = 255,
    #[doc = "2: Clock Multiplier M = 3"]
    H02 = 2,
    #[doc = "1: Clock Multiplier M = 2"]
    H01 = 1,
    #[doc = "0: Clock Multiplier is Not Supported"]
    H00 = 0,
}
impl From<Clockmultiplier> for u8 {
    #[inline(always)]
    fn from(variant: Clockmultiplier) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clockmultiplier {
    type Ux = u8;
}
#[doc = "Field `CLOCKMULTIPLIER` reader - This field indicates clock multiplier value of programmable clock\n\ngenerator. Refer to Clock Control register. Setting 00h means\n\nthat Host Controller does not support programmable clock\n\ngenerator."]
pub type ClockmultiplierR = crate::FieldReader<Clockmultiplier>;
impl ClockmultiplierR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clockmultiplier> {
        match self.bits {
            255 => Some(Clockmultiplier::HFf),
            2 => Some(Clockmultiplier::H02),
            1 => Some(Clockmultiplier::H01),
            0 => Some(Clockmultiplier::H00),
            _ => None,
        }
    }
    #[doc = "Clock Multiplier M = 256 ...."]
    #[inline(always)]
    pub fn is_h_ff(&self) -> bool {
        *self == Clockmultiplier::HFf
    }
    #[doc = "Clock Multiplier M = 3"]
    #[inline(always)]
    pub fn is_h02(&self) -> bool {
        *self == Clockmultiplier::H02
    }
    #[doc = "Clock Multiplier M = 2"]
    #[inline(always)]
    pub fn is_h01(&self) -> bool {
        *self == Clockmultiplier::H01
    }
    #[doc = "Clock Multiplier is Not Supported"]
    #[inline(always)]
    pub fn is_h00(&self) -> bool {
        *self == Clockmultiplier::H00
    }
}
#[doc = "SPI block mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spiblockmode {
    #[doc = "0: Not Supported"]
    B0 = 0,
    #[doc = "1: Supported"]
    B1 = 1,
}
impl From<Spiblockmode> for bool {
    #[inline(always)]
    fn from(variant: Spiblockmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIBLOCKMODE` reader - SPI block mode"]
pub type SpiblockmodeR = crate::BitReader<Spiblockmode>;
impl SpiblockmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spiblockmode {
        match self.bits {
            false => Spiblockmode::B0,
            true => Spiblockmode::B1,
        }
    }
    #[doc = "Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Spiblockmode::B0
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Spiblockmode::B1
    }
}
#[doc = "HS400 Support\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hs400support {
    #[doc = "0: Not Supported"]
    B0 = 0,
    #[doc = "1: Supported"]
    B1 = 1,
}
impl From<Hs400support> for bool {
    #[inline(always)]
    fn from(variant: Hs400support) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS400SUPPORT` reader - HS400 Support"]
pub type Hs400supportR = crate::BitReader<Hs400support>;
impl Hs400supportR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hs400support {
        match self.bits {
            false => Hs400support::B0,
            true => Hs400support::B1,
        }
    }
    #[doc = "Not Supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hs400support::B0
    }
    #[doc = "Supported"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hs400support::B1
    }
}
impl R {
    #[doc = "Bits 0:5 - This bit shows the base clock frequency used to detect Data\n\nTimeout Error.\n\nNot 0: 1Khz to 63Khz or 1Mhz to 63Mhz\n\n0: Get Information viaanother method"]
    #[inline(always)]
    pub fn timeoutclockfrequency(&self) -> TimeoutclockfrequencyR {
        TimeoutclockfrequencyR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - This bit shows the unit of base clock frequency used to detect\n\nData Timeout Error."]
    #[inline(always)]
    pub fn timeoutclockunit(&self) -> TimeoutclockunitR {
        TimeoutclockunitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Base Clock Frequency for SD Clock\n\n(1) 6-bit Base Clock Frequency\n\nThis mode is supported by the Host Controller Version 1.00 and\n\n2.00. Upper 2-bit is not effective and always 0. Unit values are\n\n1MHz. The supported clock range is 10MHz to 63MHz.\n\n8'h00: Get information via another method\n\n8'h01: 1MHz\n\n8'h02: 2MHz\n\n......\n\n8'h3f: 63MHz\n\nothers: not supported\n\n(2) 8-bit Base Clock Frequency\n\nThis mode is supported by the Host Controller Version 3.00. Unit\n\nvalues are 1MHz. The supported clock range is 10MHz to 255MHz.\n\n8'h00: Get information via another method\n\n8'h01: 1MHz\n\n8'h02: 2MHz\n\n......\n\n8'hff: 255MHz\n\nIf the real frequency is 16.5MHz, the lager value shall be set\n\n0001 0001b (17MHz) because the Host Driver use this value to\n\ncalculate the clock divider value (Refer to the SDCLK Frequency\n\nSelect in the Clock Control register.) and it shall not exceed upper\n\nlimit of the SD Clock frequency. If these bits are all 0, the Host\n\nSystem has to get information via another method."]
    #[inline(always)]
    pub fn baseclockfreqsdclock(&self) -> BaseclockfreqsdclockR {
        BaseclockfreqsdclockR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Max Block Length\n\nThis value indicates the maximum block size that the HD can\n\nread and write to the buffer in the HC.\n\nThe buffer shall transfer this block size without wait cycles. Three\n\nsizes can be defined as indicated below."]
    #[inline(always)]
    pub fn maxblocklength(&self) -> MaxblocklengthR {
        MaxblocklengthR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Extended Media Bus Support\n\nThis bit indicates whether the Host Controller is capable of using\n\n8-bit bus width mode. This bit is not effective when Slot Type is\n\nset to 10b. In this case, refer to Bus Width Preset in the Shared\n\nBus resister."]
    #[inline(always)]
    pub fn extendedmediabussupport(&self) -> ExtendedmediabussupportR {
        ExtendedmediabussupportR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADMA2 Support"]
    #[inline(always)]
    pub fn adma2support(&self) -> Adma2supportR {
        Adma2supportR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - High Speed Support\n\nThis bit indicates whether the HC and the Host System support\n\nHigh Speed mode and they can supply SD Clock frequency from\n\n25Mhz to 50 Mhz (for SD)/ 20MHz to 52MHz (for eMMC)."]
    #[inline(always)]
    pub fn highspeedsupport(&self) -> HighspeedsupportR {
        HighspeedsupportR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - This bit indicates whether the HC is capable of using DMA to\n\ntransfer data between system memory and the HC directly."]
    #[inline(always)]
    pub fn sdmasupport(&self) -> SdmasupportR {
        SdmasupportR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Suspend / Resume Support\n\nThis bit indicates whether the HC supports Suspend / Resume\n\nfunctionality. If this bit is 0, the Suspend and Resume mechanism\n\nare not supported and the HD shall not issue either Suspend /\n\nResume commands."]
    #[inline(always)]
    pub fn suspendresumesupport(&self) -> SuspendresumesupportR {
        SuspendresumesupportR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3 V"]
    #[inline(always)]
    pub fn voltage33vsupport(&self) -> Voltage33vsupportR {
        Voltage33vsupportR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage Support 3.0 V"]
    #[inline(always)]
    pub fn voltage30vsupport(&self) -> Voltage30vsupportR {
        Voltage30vsupportR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Voltage Support 1.8 V"]
    #[inline(always)]
    pub fn voltage18vsupport(&self) -> Voltage18vsupportR {
        Voltage18vsupportR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - 64-bit System Bus Support"]
    #[inline(always)]
    pub fn systembussupport(&self) -> SystembussupportR {
        SystembussupportR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support\n\nRefer to SDIO Specification Version 3.00 about asynchronous\n\ninterrupt."]
    #[inline(always)]
    pub fn asynintsupport(&self) -> AsynintsupportR {
        AsynintsupportR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - This field indicates usage of a slot by a specific Host System. (A\n\nhost controller register set is defined per slot.) Embedded slot for\n\none device (01b) means that only one non-removable device is\n\nconnected to a SD bus slot. Shared Bus Slot (10b) can be set if\n\nHost Controller supports Shared Bus Control register.\n\nThe Standard Host Driver controls only a removable card or one\n\nembedded device is onnected to a SD bus slot. If a slot is\n\nconfigured for shared bus (10b), the Standard Host Driver does\n\nnot control embedded devices connected to a shared bus. Shared\n\nbus slot is controlled by a specific host driver developed by a Host\n\nSystem."]
    #[inline(always)]
    pub fn slottype(&self) -> SlottypeR {
        SlottypeR::new(((self.bits >> 30) & 3) as u8)
    }
    #[doc = "Bit 32 - SDR50 Support"]
    #[inline(always)]
    pub fn sdr50support(&self) -> Sdr50supportR {
        Sdr50supportR::new(((self.bits >> 32) & 1) != 0)
    }
    #[doc = "Bit 33 - SDR104 Support."]
    #[inline(always)]
    pub fn sdr104support(&self) -> Sdr104supportR {
        Sdr104supportR::new(((self.bits >> 33) & 1) != 0)
    }
    #[doc = "Bit 34 - DDR50 Support"]
    #[inline(always)]
    pub fn ddr50support(&self) -> Ddr50supportR {
        Ddr50supportR::new(((self.bits >> 34) & 1) != 0)
    }
    #[doc = "Bit 36 - This bit indicates support of Driver Type A for 1.8 Signaling."]
    #[inline(always)]
    pub fn drivertypeasupport(&self) -> DrivertypeasupportR {
        DrivertypeasupportR::new(((self.bits >> 36) & 1) != 0)
    }
    #[doc = "Bit 37 - This bit indicates support of Driver Type C for 1.8 Signaling."]
    #[inline(always)]
    pub fn drivertypecsupport(&self) -> DrivertypecsupportR {
        DrivertypecsupportR::new(((self.bits >> 37) & 1) != 0)
    }
    #[doc = "Bit 38 - This bit indicates support of Driver Type D for 1.8 Signaling."]
    #[inline(always)]
    pub fn drivertypedsupport(&self) -> DrivertypedsupportR {
        DrivertypedsupportR::new(((self.bits >> 38) & 1) != 0)
    }
    #[doc = "Bit 39 - Driver Type 4 Support"]
    #[inline(always)]
    pub fn drivertype4support(&self) -> Drivertype4supportR {
        Drivertype4supportR::new(((self.bits >> 39) & 1) != 0)
    }
    #[doc = "Bits 40:43 - Timer count for ReTuning\n\nThis field indicates an initial value of the Re-Tuning Timer for Re-\n\nTuning Mode 1 to 3.\n\n4'h0 - Get information via other source\n\n4'h1 = 1 seconds\n\n4'h2 = 2 seconds\n\n4'h3 = 4 seconds\n\n4'h4 = 8 seconds\n\n........\n\n4'hB = 1024 seconds\n\n4'hF - Ch = Reserved"]
    #[inline(always)]
    pub fn timercountforretuning(&self) -> TimercountforretuningR {
        TimercountforretuningR::new(((self.bits >> 40) & 0x0f) as u8)
    }
    #[doc = "Bit 45 - Use Tuning for SDR50\n\nIf this bit is set to 1, this Host Controller requires tuning to\n\noperate SDR50. (Tuning is always required to operate SDR104.)"]
    #[inline(always)]
    pub fn usetuningforsdr50(&self) -> Usetuningforsdr50R {
        Usetuningforsdr50R::new(((self.bits >> 45) & 1) != 0)
    }
    #[doc = "Bits 46:47 - Re-tuning modes\n\nThis field defines the re-tuning capability of a Host Controller and\n\nhow to manage the data transfer length and a Re-Tuning Timer\n\nby the Host Driver"]
    #[inline(always)]
    pub fn retuningmode(&self) -> RetuningmodeR {
        RetuningmodeR::new(((self.bits >> 46) & 3) as u8)
    }
    #[doc = "Bits 48:55 - This field indicates clock multiplier value of programmable clock\n\ngenerator. Refer to Clock Control register. Setting 00h means\n\nthat Host Controller does not support programmable clock\n\ngenerator."]
    #[inline(always)]
    pub fn clockmultiplier(&self) -> ClockmultiplierR {
        ClockmultiplierR::new(((self.bits >> 48) & 0xff) as u8)
    }
    #[doc = "Bit 57 - SPI block mode"]
    #[inline(always)]
    pub fn spiblockmode(&self) -> SpiblockmodeR {
        SpiblockmodeR::new(((self.bits >> 57) & 1) != 0)
    }
    #[doc = "Bit 63 - HS400 Support"]
    #[inline(always)]
    pub fn hs400support(&self) -> Hs400supportR {
        Hs400supportR::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 23 - Suspend / Resume Support\n\nThis bit indicates whether the HC supports Suspend / Resume\n\nfunctionality. If this bit is 0, the Suspend and Resume mechanism\n\nare not supported and the HD shall not issue either Suspend /\n\nResume commands."]
    #[inline(always)]
    #[must_use]
    pub fn suspendresumesupport(&mut self) -> SuspendresumesupportW<EmmccoreCapSpec> {
        SuspendresumesupportW::new(self, 23)
    }
    #[doc = "Bit 24 - Voltage Support 3.3 V"]
    #[inline(always)]
    #[must_use]
    pub fn voltage33vsupport(&mut self) -> Voltage33vsupportW<EmmccoreCapSpec> {
        Voltage33vsupportW::new(self, 24)
    }
    #[doc = "Bit 37 - This bit indicates support of Driver Type C for 1.8 Signaling."]
    #[inline(always)]
    #[must_use]
    pub fn drivertypecsupport(&mut self) -> DrivertypecsupportW<EmmccoreCapSpec> {
        DrivertypecsupportW::new(self, 37)
    }
}
#[doc = "Capabilities register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_cap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_cap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreCapSpec;
impl crate::RegisterSpec for EmmccoreCapSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`emmccore_cap::R`](R) reader structure"]
impl crate::Readable for EmmccoreCapSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_cap::W`](W) writer structure"]
impl crate::Writable for EmmccoreCapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_CAP to value 0x8010_20f7_44ed_c880"]
impl crate::Resettable for EmmccoreCapSpec {
    const RESET_VALUE: u64 = 0x8010_20f7_44ed_c880;
}
