#[doc = "Register `I2S_TXCR` reader"]
pub type R = crate::R<I2sTxcrSpec>;
#[doc = "Register `I2S_TXCR` writer"]
pub type W = crate::W<I2sTxcrSpec>;
#[doc = "Field `VDW` reader - Valid Data width\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\n0~14:reserved\n\n15:16bit\n\n16:17bit\n\n17:18bit\n\n18:19bit\n\n......\n\nn:(n+1)bit\n\n......\n\n28:29bit\n\n29:30bit\n\n30:31bit\n\n31:32bit"]
pub type VdwR = crate::FieldReader;
#[doc = "Field `VDW` writer - Valid Data width\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\n0~14:reserved\n\n15:16bit\n\n16:17bit\n\n17:18bit\n\n18:19bit\n\n......\n\nn:(n+1)bit\n\n......\n\n28:29bit\n\n29:30bit\n\n30:31bit\n\n31:32bit"]
pub type VdwW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Transfer format select\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfs {
    #[doc = "0: I2S format"]
    B0 = 0,
    #[doc = "1: PCM format"]
    B1 = 1,
}
impl From<Tfs> for bool {
    #[inline(always)]
    fn from(variant: Tfs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFS` reader - Transfer format select\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
pub type TfsR = crate::BitReader<Tfs>;
impl TfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfs {
        match self.bits {
            false => Tfs::B0,
            true => Tfs::B1,
        }
    }
    #[doc = "I2S format"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tfs::B0
    }
    #[doc = "PCM format"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tfs::B1
    }
}
#[doc = "Field `TFS` writer - Transfer format select\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
pub type TfsW<'a, REG> = crate::BitWriter<'a, REG, Tfs>;
impl<'a, REG> TfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2S format"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tfs::B0)
    }
    #[doc = "PCM format"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tfs::B1)
    }
}
#[doc = "PCM bus mode\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pbm {
    #[doc = "0: PCM no delay mode"]
    D0 = 0,
    #[doc = "1: PCM delay 1 mode"]
    D1 = 1,
    #[doc = "2: PCM delay 2 mode"]
    D2 = 2,
    #[doc = "3: PCM delay 3 mode"]
    D3 = 3,
}
impl From<Pbm> for u8 {
    #[inline(always)]
    fn from(variant: Pbm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pbm {
    type Ux = u8;
}
#[doc = "Field `PBM` reader - PCM bus mode\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
pub type PbmR = crate::FieldReader<Pbm>;
impl PbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbm {
        match self.bits {
            0 => Pbm::D0,
            1 => Pbm::D1,
            2 => Pbm::D2,
            3 => Pbm::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "PCM no delay mode"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Pbm::D0
    }
    #[doc = "PCM delay 1 mode"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Pbm::D1
    }
    #[doc = "PCM delay 2 mode"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Pbm::D2
    }
    #[doc = "PCM delay 3 mode"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Pbm::D3
    }
}
#[doc = "Field `PBM` writer - PCM bus mode\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
pub type PbmW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Pbm>;
impl<'a, REG> PbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PCM no delay mode"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(Pbm::D0)
    }
    #[doc = "PCM delay 1 mode"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(Pbm::D1)
    }
    #[doc = "PCM delay 2 mode"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(Pbm::D2)
    }
    #[doc = "PCM delay 3 mode"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(Pbm::D3)
    }
}
#[doc = "I2S bus mode\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ibm {
    #[doc = "0: I2S normal"]
    D0 = 0,
    #[doc = "1: I2S Left justified"]
    D1 = 1,
    #[doc = "2: I2S Right justified"]
    D2 = 2,
    #[doc = "3: reserved"]
    D3 = 3,
}
impl From<Ibm> for u8 {
    #[inline(always)]
    fn from(variant: Ibm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ibm {
    type Ux = u8;
}
#[doc = "Field `IBM` reader - I2S bus mode\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
pub type IbmR = crate::FieldReader<Ibm>;
impl IbmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ibm {
        match self.bits {
            0 => Ibm::D0,
            1 => Ibm::D1,
            2 => Ibm::D2,
            3 => Ibm::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "I2S normal"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Ibm::D0
    }
    #[doc = "I2S Left justified"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Ibm::D1
    }
    #[doc = "I2S Right justified"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Ibm::D2
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Ibm::D3
    }
}
#[doc = "Field `IBM` writer - I2S bus mode\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
pub type IbmW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Ibm>;
impl<'a, REG> IbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "I2S normal"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(Ibm::D0)
    }
    #[doc = "I2S Left justified"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(Ibm::D1)
    }
    #[doc = "I2S Right justified"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(Ibm::D2)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn d3(self) -> &'a mut crate::W<REG> {
        self.variant(Ibm::D3)
    }
}
#[doc = "First Bit Mode\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fbm {
    #[doc = "0: MSB"]
    B0 = 0,
    #[doc = "1: LSB"]
    B1 = 1,
}
impl From<Fbm> for bool {
    #[inline(always)]
    fn from(variant: Fbm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FBM` reader - First Bit Mode\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
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
    #[doc = "MSB"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fbm::B0
    }
    #[doc = "LSB"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fbm::B1
    }
}
#[doc = "Field `FBM` writer - First Bit Mode\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
pub type FbmW<'a, REG> = crate::BitWriter<'a, REG, Fbm>;
impl<'a, REG> FbmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSB"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fbm::B0)
    }
    #[doc = "LSB"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fbm::B1)
    }
}
#[doc = "Store justified mode\n\nSJM\n\nStore justified mode\n\n(Can be written only when XFER\\[1\\]
bit is 0.)\n\n16bit~31bit DATA stored in 32 bits width fifo.\n\nThis bit is invalid if VDW select 16bit data and HWT select 0,\n\nBecause every fifo unit contain two 16bit data and 32 bit space is\n\nfull, it is impossible to choose justified mode.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sjm {
    #[doc = "0: right justified"]
    B0 = 0,
    #[doc = "1: left justified"]
    B1 = 1,
}
impl From<Sjm> for bool {
    #[inline(always)]
    fn from(variant: Sjm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SJM` reader - Store justified mode\n\nSJM\n\nStore justified mode\n\n(Can be written only when XFER\\[1\\]
bit is 0.)\n\n16bit~31bit DATA stored in 32 bits width fifo.\n\nThis bit is invalid if VDW select 16bit data and HWT select 0,\n\nBecause every fifo unit contain two 16bit data and 32 bit space is\n\nfull, it is impossible to choose justified mode."]
pub type SjmR = crate::BitReader<Sjm>;
impl SjmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sjm {
        match self.bits {
            false => Sjm::B0,
            true => Sjm::B1,
        }
    }
    #[doc = "right justified"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sjm::B0
    }
    #[doc = "left justified"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sjm::B1
    }
}
#[doc = "Field `SJM` writer - Store justified mode\n\nSJM\n\nStore justified mode\n\n(Can be written only when XFER\\[1\\]
bit is 0.)\n\n16bit~31bit DATA stored in 32 bits width fifo.\n\nThis bit is invalid if VDW select 16bit data and HWT select 0,\n\nBecause every fifo unit contain two 16bit data and 32 bit space is\n\nfull, it is impossible to choose justified mode."]
pub type SjmW<'a, REG> = crate::BitWriter<'a, REG, Sjm>;
impl<'a, REG> SjmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "right justified"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sjm::B0)
    }
    #[doc = "left justified"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sjm::B1)
    }
}
#[doc = "Halfword word transform\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\nOnly valid when VDW select 16bit data.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hwt {
    #[doc = "0: 32 bit data valid from AHB/APB bus. Low 16 bit for left channel and high 16 bit for right channel."]
    B0 = 0,
    #[doc = "1: low 16bit data valid from AHB/APB bus, high 16 bit data invalid."]
    B1 = 1,
}
impl From<Hwt> for bool {
    #[inline(always)]
    fn from(variant: Hwt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWT` reader - Halfword word transform\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\nOnly valid when VDW select 16bit data."]
pub type HwtR = crate::BitReader<Hwt>;
impl HwtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hwt {
        match self.bits {
            false => Hwt::B0,
            true => Hwt::B1,
        }
    }
    #[doc = "32 bit data valid from AHB/APB bus. Low 16 bit for left channel and high 16 bit for right channel."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hwt::B0
    }
    #[doc = "low 16bit data valid from AHB/APB bus, high 16 bit data invalid."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hwt::B1
    }
}
#[doc = "Field `HWT` writer - Halfword word transform\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\nOnly valid when VDW select 16bit data."]
pub type HwtW<'a, REG> = crate::BitWriter<'a, REG, Hwt>;
impl<'a, REG> HwtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "32 bit data valid from AHB/APB bus. Low 16 bit for left channel and high 16 bit for right channel."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hwt::B0)
    }
    #[doc = "low 16bit data valid from AHB/APB bus, high 16 bit data invalid."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hwt::B1)
    }
}
#[doc = "TX Channel select register\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcsr {
    #[doc = "0: two channel"]
    B00 = 0,
    #[doc = "1: four channel"]
    B01 = 1,
    #[doc = "2: six channel"]
    B10 = 2,
    #[doc = "3: eight channel"]
    B11 = 3,
}
impl From<Tcsr> for u8 {
    #[inline(always)]
    fn from(variant: Tcsr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcsr {
    type Ux = u8;
}
#[doc = "Field `TCSR` reader - TX Channel select register"]
pub type TcsrR = crate::FieldReader<Tcsr>;
impl TcsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcsr {
        match self.bits {
            0 => Tcsr::B00,
            1 => Tcsr::B01,
            2 => Tcsr::B10,
            3 => Tcsr::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "two channel"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Tcsr::B00
    }
    #[doc = "four channel"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Tcsr::B01
    }
    #[doc = "six channel"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Tcsr::B10
    }
    #[doc = "eight channel"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Tcsr::B11
    }
}
#[doc = "Field `TCSR` writer - TX Channel select register"]
pub type TcsrW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Tcsr>;
impl<'a, REG> TcsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "two channel"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Tcsr::B00)
    }
    #[doc = "four channel"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Tcsr::B01)
    }
    #[doc = "six channel"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Tcsr::B10)
    }
    #[doc = "eight channel"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Tcsr::B11)
    }
}
#[doc = "Field `RCNT` reader - right justified counter\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\nOnly valid in I2S Right justified format and slave tx mode is\n\nselected.\n\nStart to transmit data RCNT sclk cycles after left channel valid."]
pub type RcntR = crate::FieldReader;
#[doc = "Field `RCNT` writer - right justified counter\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\nOnly valid in I2S Right justified format and slave tx mode is\n\nselected.\n\nStart to transmit data RCNT sclk cycles after left channel valid."]
pub type RcntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:4 - Valid Data width\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\n0~14:reserved\n\n15:16bit\n\n16:17bit\n\n17:18bit\n\n18:19bit\n\n......\n\nn:(n+1)bit\n\n......\n\n28:29bit\n\n29:30bit\n\n30:31bit\n\n31:32bit"]
    #[inline(always)]
    pub fn vdw(&self) -> VdwR {
        VdwR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Transfer format select\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    pub fn tfs(&self) -> TfsR {
        TfsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 7:8 - PCM bus mode\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    pub fn pbm(&self) -> PbmR {
        PbmR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:10 - I2S bus mode\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    pub fn ibm(&self) -> IbmR {
        IbmR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - First Bit Mode\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    pub fn fbm(&self) -> FbmR {
        FbmR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Store justified mode\n\nSJM\n\nStore justified mode\n\n(Can be written only when XFER\\[1\\]
bit is 0.)\n\n16bit~31bit DATA stored in 32 bits width fifo.\n\nThis bit is invalid if VDW select 16bit data and HWT select 0,\n\nBecause every fifo unit contain two 16bit data and 32 bit space is\n\nfull, it is impossible to choose justified mode."]
    #[inline(always)]
    pub fn sjm(&self) -> SjmR {
        SjmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Halfword word transform\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\nOnly valid when VDW select 16bit data."]
    #[inline(always)]
    pub fn hwt(&self) -> HwtR {
        HwtR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - TX Channel select register"]
    #[inline(always)]
    pub fn tcsr(&self) -> TcsrR {
        TcsrR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:22 - right justified counter\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\nOnly valid in I2S Right justified format and slave tx mode is\n\nselected.\n\nStart to transmit data RCNT sclk cycles after left channel valid."]
    #[inline(always)]
    pub fn rcnt(&self) -> RcntR {
        RcntR::new(((self.bits >> 17) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Valid Data width\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\n0~14:reserved\n\n15:16bit\n\n16:17bit\n\n17:18bit\n\n18:19bit\n\n......\n\nn:(n+1)bit\n\n......\n\n28:29bit\n\n29:30bit\n\n30:31bit\n\n31:32bit"]
    #[inline(always)]
    #[must_use]
    pub fn vdw(&mut self) -> VdwW<I2sTxcrSpec> {
        VdwW::new(self, 0)
    }
    #[doc = "Bit 5 - Transfer format select\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    #[must_use]
    pub fn tfs(&mut self) -> TfsW<I2sTxcrSpec> {
        TfsW::new(self, 5)
    }
    #[doc = "Bits 7:8 - PCM bus mode\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    #[must_use]
    pub fn pbm(&mut self) -> PbmW<I2sTxcrSpec> {
        PbmW::new(self, 7)
    }
    #[doc = "Bits 9:10 - I2S bus mode\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    #[must_use]
    pub fn ibm(&mut self) -> IbmW<I2sTxcrSpec> {
        IbmW::new(self, 9)
    }
    #[doc = "Bit 11 - First Bit Mode\n\n(Can be written only when XFER\\[0\\]
bit is 0.)"]
    #[inline(always)]
    #[must_use]
    pub fn fbm(&mut self) -> FbmW<I2sTxcrSpec> {
        FbmW::new(self, 11)
    }
    #[doc = "Bit 12 - Store justified mode\n\nSJM\n\nStore justified mode\n\n(Can be written only when XFER\\[1\\]
bit is 0.)\n\n16bit~31bit DATA stored in 32 bits width fifo.\n\nThis bit is invalid if VDW select 16bit data and HWT select 0,\n\nBecause every fifo unit contain two 16bit data and 32 bit space is\n\nfull, it is impossible to choose justified mode."]
    #[inline(always)]
    #[must_use]
    pub fn sjm(&mut self) -> SjmW<I2sTxcrSpec> {
        SjmW::new(self, 12)
    }
    #[doc = "Bit 14 - Halfword word transform\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\nOnly valid when VDW select 16bit data."]
    #[inline(always)]
    #[must_use]
    pub fn hwt(&mut self) -> HwtW<I2sTxcrSpec> {
        HwtW::new(self, 14)
    }
    #[doc = "Bits 15:16 - TX Channel select register"]
    #[inline(always)]
    #[must_use]
    pub fn tcsr(&mut self) -> TcsrW<I2sTxcrSpec> {
        TcsrW::new(self, 15)
    }
    #[doc = "Bits 17:22 - right justified counter\n\n(Can be written only when XFER\\[0\\]
bit is 0.)\n\nOnly valid in I2S Right justified format and slave tx mode is\n\nselected.\n\nStart to transmit data RCNT sclk cycles after left channel valid."]
    #[inline(always)]
    #[must_use]
    pub fn rcnt(&mut self) -> RcntW<I2sTxcrSpec> {
        RcntW::new(self, 17)
    }
}
#[doc = "transmit operation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_txcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_txcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sTxcrSpec;
impl crate::RegisterSpec for I2sTxcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_txcr::R`](R) reader structure"]
impl crate::Readable for I2sTxcrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2s_txcr::W`](W) writer structure"]
impl crate::Writable for I2sTxcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2S_TXCR to value 0x0f"]
impl crate::Resettable for I2sTxcrSpec {
    const RESET_VALUE: u32 = 0x0f;
}
